use std::env;
use std::path::Path;
use std::fs;
use std::io;
use std::fs::File;
use std::io::prelude::*;


#[derive(Debug)]
pub struct Package{
  pub dir: String,
  pub source: Vec<Source>
}

#[derive(Debug)]
pub struct Source{
    pub source: String
}

fn build_source(dir: &Path) -> io::Result<Vec<Source>> {
    let mut file_list = vec![];

     if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                continue
            }

             let mut file = File::open(path)?;
             let mut content = String::new();
             file.read_to_string(&mut content)?;

            file_list.push(Source{
                source: content
            });
        }
    }

    Ok(file_list)
}


pub fn init() -> Option<Package> {    

    match env::args().nth(1) {
        Some(dir_name) => {
            let path = Path::new(&dir_name);
            match build_source(&path) {
                Ok(source) => {
                    Some(Package{
                        dir:dir_name.clone(),
                        source:source,
                    })
                    
                },
                Err(e) => {
                    println!("{:?}", e);
                    None
                },
            }
        },
        None => None,
    }
}