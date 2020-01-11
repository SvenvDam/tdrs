use std::error::Error;

use structopt::StructOpt;

pub mod actions;
pub mod io;
pub mod model;

fn main() {
    let args = model::Args::from_args();

    run(args).unwrap();
}

fn run(args: model::Args) -> Result<(), Box<dyn Error>> {
    let path_buf = dirs::home_dir()
        .ok_or(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Home directory is not found",
        ))
        .map(|home| home.join(".tdrs").join(&args.list))?;

    let path = path_buf.as_path();

    let mut state = io::read_state(&path)?;

    actions::update(args.cmd, &mut state);

    io::persist_state(path, &state)?;

    Ok(())
}
