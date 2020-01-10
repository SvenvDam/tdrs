use structopt::*;
use serde::{Serialize, Deserialize};

#[derive(StructOpt, Debug)]
pub enum Command {
    List,
    Add {
        #[structopt(short, long)]
        content: String
    },
    Close {
        #[structopt(short, long)]
        index: usize
    },
    Delete {
        #[structopt(short, long)]
        index: usize
    },
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ToDo {
    pub open: bool,
    pub content: String,
}

pub type State = Vec<ToDo>;

