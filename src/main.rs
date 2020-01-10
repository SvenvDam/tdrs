use structopt::StructOpt;

pub mod io;
pub mod actions;
pub mod model;

fn main() {
    let cmd = model::Command::from_args();
    let path = io::setup_path("~/.tdrs/default".as_ref());

    let mut state = io::read_state(&path).unwrap();

    actions::update(cmd, &mut state);

    io::persist_state(path, &state).unwrap()
}
