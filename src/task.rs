use crate::{cli::Cli, item::Item};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::{fs::OpenOptions, io::Write};
use structopt::StructOpt;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Task {
    pub items: Vec<Item>,
}

impl Task {
    pub fn parse(content: String) -> Task {
        return serde_json::from_str(&content).unwrap();
    }
    pub fn add(&mut self, new_item: Item) {
        self.items.push(new_item);
    }
    pub fn save(&self) {
        let args = Cli::from_args();
        let mut f = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(args.path)
            .unwrap();
        let result = f.write(serde_json::to_string(self).unwrap().as_bytes());
        f.flush().unwrap();
        match result {
            Ok(_) => {}
            Err(err) => {
                panic!("{}", err)
            }
        }
    }
}

pub fn from_storage(path: &std::path::PathBuf) -> Task {
    if !path.exists() {
        let result = File::create(path);
        match result {
            Ok(_) => {
                println!("create new file at: {}", &path.to_str().unwrap());
                // write empty tasks
                let task: Task = Default::default();
                task.save()
            }
            Err(err) => {
                panic!("{}", err)
            }
        }
    }
    let content = std::fs::read_to_string(path);
    match content {
        Ok(content) => return Task::parse(content),
        Err(err) => {
            panic!("{}", err)
        }
    }
}
