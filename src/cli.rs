use std::path::PathBuf;

use crate::item::{Category, Item};
use crate::task::Task;
use std::io::{stdin, stdout, Write};
use structopt::StructOpt;
use termion::{clear, cursor, event::Key, input::TermRead, raw::IntoRawMode};

#[derive(StructOpt)]
pub struct Cli {
    #[structopt(short, long, parse(from_os_str), default_value = ".task.db")]
    pub path: PathBuf,
    #[structopt(subcommand)]
    pub command: Option<Command>,
}

#[derive(StructOpt)]
pub enum Command {
    Add { args: Vec<String> },
}

pub fn add(mut task: Task, args: Vec<String>) {
    let mut categories = Vec::<Category>::new();
    for mut arg in args.clone() {
        if arg.starts_with("@") {
            arg.remove(0);
            categories.push(Category {
                name: arg.to_string(),
            })
        }
    }
    let new_item = Item {
        title: args.join(" "),
        categories,
        ..Default::default()
    };
    task.add(new_item.clone());
    println!("Task added: {}", new_item.title);
    task.save()
}

pub fn list(mut task: Task) {
    let mut cur = 0;
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut will_deleted_index = task.items.len();

    loop {
        let stdin = stdin();
        for (i, task) in task.items.iter().enumerate() {
            write!(stdout, "\n\r{}", clear::CurrentLine).unwrap();

            let mut prefix = String::from("");
            if i == cur {
                prefix.push_str("> ");
            } else {
                prefix.push_str("  ");
            }
            if task.done {
                prefix.push_str("[x]");
            } else {
                prefix.push_str("[ ]");
            }
            if i == will_deleted_index {
                write!(stdout, "{} delete?", prefix).unwrap();
            } else {
                write!(
                    stdout,
                    "{} {} ({})",
                    prefix,
                    task.title,
                    task.created_at.format("%Y-%m-%d %H:%M")
                )
                .unwrap();
            }
        }
        stdout.lock().flush().unwrap();
        let next = stdin.keys().next().unwrap();
        let task_items_amount = task.items.len();
        match next.unwrap() {
            Key::Char('\n') => {
                task.save();
                break;
            }
            Key::Up | Key::Char('k') if cur != 0 => {
                cur -= 1;
            }
            Key::Down | Key::Char('j') if cur != task_items_amount - 1 => {
                cur += 1;
            }
            Key::Char(' ') => {
                task.items[cur].toggle();
            }
            Key::Char('d') => {
                if will_deleted_index == cur {
                    task.items.remove(cur);
                    will_deleted_index = task_items_amount + 1;
                    if task.items.len() - 1 < cur {
                        cur -= 1
                    }
                } else {
                    will_deleted_index = cur;
                }
            }
            Key::Esc => {
                if will_deleted_index < task_items_amount {
                    will_deleted_index = task_items_amount
                }
            }
            _ => {
                // pass
            }
        }
        write!(stdout, "{}", cursor::Up(task_items_amount as u16)).unwrap();
        write!(stdout, "{}", clear::AfterCursor).unwrap();
    }
}
