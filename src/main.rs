use structopt::StructOpt;

pub mod io;
pub mod actions;
pub mod model;

fn main() {
    let args = model::Args::from_args();
    let path_buf = dirs::home_dir()
        .map(|home| {
            home.join(".tdrs").join(&args.list)
        })
        .unwrap();

    let path = path_buf.as_path();

    let mut state = io::read_state(&path).unwrap();

    actions::update(args.cmd, &mut state);

    io::persist_state(path, &state).unwrap()
}
