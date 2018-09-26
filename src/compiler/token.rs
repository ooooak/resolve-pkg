use std::str::CharIndices;
use std::iter::Peekable;
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
pub enum Identifiers {
    Package,
    Import,
    Keyword(String),
    String(String),
    // Undefined,
    WhiteSpace,
    Unknown
}



impl Identifiers{
    pub fn is_white_space(&self) -> bool {
        match self {
            Identifiers::WhiteSpace => true,
            _ => false,
        }
    }
}

type MaybeToken = Option<Result<Identifiers, ScannerError>>;

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




    fn ok(&self, id: Identifiers) -> MaybeToken{
        Some(Ok(id))
    }

    fn keyword(&mut self) -> MaybeToken {
        let mut word = String::new();
        while let Some(item) = self.peek() {
            if item == ' ' || item == '\n' || item == '\r'  {
                break
            }

            self.get();
            word.push(item)
        }

        match word.as_ref() {
            "package" => self.ok(Identifiers::Package),
            "import" => self.ok(Identifiers::Import),
            _ => {
                self.ok(Identifiers::Keyword(word))
            },
        }
        
    }

    fn string(&mut self) -> MaybeToken {
        self.get();
        let mut content = String::new();
        while let Some(item) = self.peek() {
            if item == '"'{
                self.get();
                break
            }

            self.get();
            content.push(item)
        }

        self.ok(Identifiers::String(content))
    }


    fn item(&mut self) -> MaybeToken {
        match self.peek() {
            Some(' ')   |
            Some('\r')  |
            Some('\n')  => {
                self.get();
                self.ok(Identifiers::WhiteSpace)
            },
            Some('"') => self.string(),
            Some('a'...'z') |
            Some( 'A'...'Z') => self.keyword(),
            Some( _ ) => {
                self.get();
                self.ok(Identifiers::Unknown)

            },
            None => None,
        }
    }

    
    pub fn tokens(&mut self) -> Vec<Identifiers> {
        let mut ret: Vec<Identifiers> = vec![];
        while let Some(node) = self.item() {
            match node {
                Ok(n) => ret.push(n),
                Err(e) => e.log_and_die(),
            }
        }
        ret
    }
}
