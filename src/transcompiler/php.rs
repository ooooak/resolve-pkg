// use syntax::scanner::Scanner;
use syntax::ast::Ast;
use std::fs::File;
use std::io::prelude::*;
use std::process;


#[derive(Debug)]
pub struct Transcompiler {
  ast:Vec<Ast>
}

impl<'a> Transcompiler {
  pub fn new(ast: Vec<Ast>) -> Transcompiler {
    Transcompiler {
      ast: ast,
    }
  }

  fn is_scalar(&self, node: &Ast) -> bool {
    match node {
        &Ast::String(_)  |
        &Ast::Boolean(_) |
        &Ast::Integer(_) |
        &Ast::Float(_)   |         
        _ => true,
        _ => false,
    }
  }

  pub fn decode_scaler(&self, node: &Ast) -> String {
    // let mut ret = String::new();
    match node {
        &Ast::String(ref value) => {
           format!("'{value}'", value=value)
        },
        _ => {
          panic!("{:?}", node);
        },
    }
  }

  pub fn compile(&mut self){
    let mut buffer = File::create("main.php").unwrap();
    let _ = buffer.write_all(b"<?php \n");
    for node in &self.ast {
      match node {
        &Ast::FnCall{ref name, ref args} => {
          let mut scalar_args: Vec<String> = vec![];
          for arg in args {
            if self.is_scalar(&arg) {
              scalar_args.push(self.decode_scaler(&arg));
            }else {
              unimplemented!();
            }
          }
          
          let text = format!("{name}({args});", name=name, args=scalar_args.join(","));
          let _ = buffer.write_all(text.as_bytes());
        },
        _ => {
          break
        }
      }
    }
  }
}