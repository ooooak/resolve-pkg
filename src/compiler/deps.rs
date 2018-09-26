use compiler::token;
use compiler::io;


pub fn log_deps(tokens: &Vec<token::Identifiers>){
  use compiler::token::Identifiers;

  let mut iter = tokens.iter().filter(|i| !i.is_white_space() );
  while let Some(item) = &iter.next() {
    match item {
      Identifiers::Import => {
        let package_name = iter.next();
        match &package_name {
          Some(Identifiers::String(value)) => {
            println!("{:?}", value);
          },
          _ => {
            io::error("Unexpected file path")
          }
        }
      },
      _ => {

      }
    }
  }
}