use std::fs;
use std::fs::OpenOptions;
use std::io::Error;
use std::io::prelude::*;
use std::path::Path;

use mktemp::Temp;

use super::model::*;

pub fn read_state(path: &Path) -> Result<State, Error> {
    let mut f = OpenOptions::new().write(true).create(true).read(true).open(&path)?;
    let mut state_str = String::new();
    f.read_to_string(&mut state_str)?;
    let state: State = serde_json::from_str(&state_str).unwrap_or_default();
    Ok(state)
}

pub fn persist_state(path: &Path, state: &[ToDo]) -> Result<(), Error> {
    let tmp_path = Temp::new_file()?;
    let mut tmp = OpenOptions::new().write(true).create(true).open(&tmp_path)?;
    let str = serde_json::to_string(&state)?;
    tmp.write_all(str.as_bytes())?;
    fs::rename(&tmp_path, &path)?;
    tmp_path.release();
    Ok(())
}

pub fn setup_path(str: &str) -> &Path {
    let path = Path::new(str.clone());
    if !path.exists() {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
    }

    path
}
