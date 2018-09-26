use std::str::CharIndices;
use std::iter::Peekable;
use std::collections::{BTreeMap}; // BTreeSet
use std::process;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ScannerError {
    msg: &'static str,
    line: usize,
    column: usize,
}

impl ScannerError{
    pub fn log_and_die(&self) -> ! {
        println!("{}", self.msg);
        process::exit(0);
    }
}

// TODO: each token will have meta data
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Token {
    List(Vec<Token>),
    Vector(Vec<Token>),
    Map(BTreeMap<Token, Token>),
    String(String),
    Symbol(String),
    Keyword(String),
    Comment(String),
    Integer(i64),
    
    // Char(char),
    // Set(BTreeSet<Token>),
    // Tagged(String, Box<Token>),
    // Boolean(bool),
    // Undefined(String),
    // Float(String),
    Nil,
}

type MaybeToken = Option<Result<Token, ScannerError>>;

#[derive(Debug)]
pub struct Scanner<'a> {
    str: &'a str,
    chars: Peekable<CharIndices<'a>>,
    line: usize,
    column: usize,
}

impl<'a> Scanner<'a> {
    pub fn new(str: &'a str) -> Scanner<'a> {
        Scanner {
            str: str,
            chars: str.char_indices().peekable(),
            line: 0,
            column: 0,
        }
    }
    fn read_while<F: FnMut(char) -> bool>(&mut self, mut condition: F) -> String {
        let mut collection = String::new();
        while let Some(item) = self.peek() {
            if condition(item) {
                self.get();
                collection.push(item)
            }else{
                break
            }
        }       

        collection
    }

     fn get(&mut self) -> Option<char> {
        match self.chars.next() {
            Some((_, value)) => {
                if value == '\n'{
                    self.line += 1;
                    self.column = 1;
                }else{
                    self.column += 1;
                }

                Some(value)
            },
            None => None,
        }
    }

    fn peek(&mut self) -> Option<char> {
        match self.chars.peek() {
            Some(&(_, value)) => Some(value),
            None => None,
        }
    }

    fn err(&self, msg: &'static str) -> MaybeToken {
        Some(Err(ScannerError {
            msg: msg,
            line: self.line,
            column: self.column
        }))
    }

    fn lists(&mut self, ch: char) -> MaybeToken {
        let close = match ch {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            _ => unreachable!()
        };

        let _ = self.get(); // skip starting ( 
        let mut collection = vec![];
        let mut map = BTreeMap::new();

        loop {
            match self.peek() {
                Some(tag) => {
                    if tag == close {
                        let _ = self.get(); // skip closing bracket

                        return match close {
                            ')' => Some(Ok(Token::List(collection))),
                            ']' => Some(Ok(Token::Vector(collection))),
                            '}' => Some(Ok(Token::Map(map))),
                            _ => unreachable!(),
                        }
                    }else{
                        if close == '}' {
                            let key = self.token_or_die();
                            let value = self.token_or_die();
                            map.insert(key, value);
                        }else{
                            let item = self.token_or_die();
                            collection.push(item)
                        }
                    }
                },
                None => {
                    return self.err("unclosed tag")
                },
            }
        }
    }

    fn keyword(&mut self) -> MaybeToken {
        let _ = self.get(); // skip :
        let str = self.read_while(|x| match x {
            'a'...'z' | 
            'A'...'Z' => true,
            _ => false,
        });
        
        Some(Ok(Token::Keyword(str)))
    }

    fn comment(&mut self) -> MaybeToken {
        let _ = self.get(); // skip:
        let str = self.read_while(|x| match x {
            '\n' | '\r'  => false,
            _ => true,
        });

        Some(Ok(Token::Comment(str)))
    }

    // fn is_symbol_head(&self, ch: char) -> bool {
    //     match ch {
    //         'a' ... 'z' | 'A' ... 'Z' |
    //         '.' | '*' | '+' | '!' | '-' | '_' |
    //         '?' | '$' | '%' | '&' | '=' | '<' | '>' => true,
    //         _ => false
    //     }
    // }

    // fn is_symbol_tail(&self,  ch: char) -> bool {
    //     self.is_symbol_head(ch) || match ch {
    //         '0' ... '9' | ':' | '#' | '/' => true,
    //         _ => false
    //     }
    // }

    /**
    * symbol
    */
    fn symbol(&mut self) -> MaybeToken {
        let mut collection = String::new();
        while let Some(ch) = self.peek() {
            match ch {
                'a' ... 'z' |
                'A' ... 'Z' |
                '-' | '.' | '*' | '+' | '!' |  '_' | 
                '?' | '$' | '%' | '&' | '=' | '<' | 
                '>' => {
                    collection.push(self.get().unwrap());
                },
                _ => {
                    break
                }
            }
        }
        Some(Ok(Token::Symbol(collection)))
    }


    fn string(&mut self) -> MaybeToken {
        self.get(); // skip starting "
        let mut collection = String::new();
        let mut complete_string = false;
	
    
        while let Some(ch) = self.get() {
            match ch {
                '\\' => {
                    let escape_ch = match self.get() {
                        Some('t') => '\t',
                        Some('r') => '\r',
                        Some('n') => '\n',
                        // TODO: handle strings
                        // Some('\\') => ' ',
                        // Some('"') =>  ' ',
                        // Some('b') =>  '\b',
                        // Some('f') =>  '\f',
                        Some('u') => unimplemented!(),
                        Some(ch2) => {
                            ch2
                        }
                        None => {
                            panic!("unexpected eof2");
                        }
                    };
                    collection.push(escape_ch);
                }
                '"' => {
                    complete_string = true;
                    break;
                },
                
                _ => {
                    collection.push(ch)
                },
            }
        }

        if !complete_string {
            self.err("Unclosed string")
        }else{
            Some(Ok(Token::String(collection)))
        }        
    }

    fn integer(&mut self) -> MaybeToken {
        let item = self.read_while(|x| match x {
            '0'...'9' => true,
            _ => false,
        });

        match item.parse::<i64>() {
            Ok(value) => Some(Ok(Token::Integer(value))),
            Err(_) => {
                self.err("Unable to unpack integer value")
            },
        }        
    }

    fn meta(&mut self) -> MaybeToken {
        None
    }

    fn character(&mut self) -> MaybeToken {
        None
    }

     fn token(&mut self) -> MaybeToken {
        match self.peek() {
            Some(' ') | 
            Some(',') => {
                self.get();
                self.token()
            },
            
            Some('0'...'9') => self.integer(),   
            Some('"') => self.string(),
            Some(';') => self.comment(),
            Some('^') => self.meta(),
            Some(':') => self.keyword(),            
            Some('\\') => self.character(),
            Some('#') => self.symbol(),

            Some(ch @ '(') | 
            Some(ch @ '[') |
            Some(ch @ '{') => self.lists(ch),
            Some(ch) => {
                match ch {
                    '\n' |
                    '\r' => {
                        self.get();
                        self.token()
                    },
                    _ => {
                        self.symbol()
                    },
                }
            },
            None => None,
        }
    }

    fn token_or_die(&mut self) -> Token {
        match self.token() {
            Some(Ok(item)) => item,
            Some(Err(err)) => {
                println!("{:?}", err);
                process::exit(0);
            },
            None => {
                println!("{}", "Unexpected eof1");
                process::exit(0);
            },
        }
    }

    #[allow(dead_code)]
    pub fn tokens(&mut self) -> Vec<Token> {
        let mut ret: Vec<Token> = vec![];
        while let Some(node) = self.token() {
            match node {
                Ok(n) => ret.push(n),
                Err(e) => e.log_and_die(),
            }
        }
        ret
    }
}
