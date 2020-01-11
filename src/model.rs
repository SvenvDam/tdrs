use serde::{Deserialize, Serialize};
use structopt::*;

#[derive(StructOpt)]
pub struct Args {
    #[structopt(long, short, default_value = "default")]
    pub list: String,
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    List,
    Add {
        #[structopt(short, long)]
        content: String,
    },
    Close {
        #[structopt(short, long)]
        index: usize,
    },
    Delete {
        #[structopt(short, long)]
        index: usize,
    },
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ToDo {
    pub open: bool,
    pub content: String,
}

pub type State = Vec<ToDo>;
