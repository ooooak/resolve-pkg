mod syntax;
mod transcompiler;
mod compiler;
use compiler::token::Scanner;
use compiler::deps;

fn die(msg: &'static str){
    println!("{}", msg);
    std::process::exit(0);
}

fn main() {
    match compiler::cli::init() {
        Some(package) => {
            for source in package.source {
                let tokens = Scanner::new(source.source.as_str()).tokens();
                deps::log_deps(&tokens)
            }
        },
        None => {
            die("Enter a dir path")
        },
    }
}