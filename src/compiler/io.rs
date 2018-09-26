use std::process;


pub fn error(msg: &'static str){
  println!("Error: {:?}", msg);
  process::exit(0);
}