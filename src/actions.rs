use std::io::{stdout, Write, BufWriter};

use super::model::*;

fn write_state(state: &State, writer: impl Write) -> Result<(), std::io::Error> {
    let mut buf = BufWriter::new(writer);
    writeln!(buf, "TODO")?;
    writeln!(buf)?;
    state
        .iter()
        .enumerate()
        .map(|(i, td)| {
            writeln!(
                buf,
                "{}\t{}\t{}",
                i,
                if td.open { "open" } else { "done" },
                td.content
            )
        })
        .fold(Ok(()), |acc, res| acc.and(res))
        .and(buf.flush())
}

pub fn update(cmd: Command, state: &mut State) {
    match cmd {
        Command::List => write_state(&state, &mut stdout()).unwrap(),
        Command::Add { content } => {
            state.push(ToDo {
                open: true,
                content,
            });
        }
        Command::Delete { index } => {
            state.remove(index);
        }
        Command::Close { index } => {
            state[index].open = false;
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cmd_add() {
        let mut state: State = Vec::new();
        update(
            Command::Add {
                content: "test".to_owned(),
            },
            &mut state,
        );
        assert_eq!(
            state,
            vec![ToDo {
                open: true,
                content: "test".to_owned(),
            }]
        )
    }

    #[test]
    fn cmd_close() {
        let mut state: State = vec![ToDo {
            open: true,
            content: "test".to_owned(),
        }];
        update(Command::Close { index: 0 }, &mut state);
        assert_eq!(
            state,
            vec![ToDo {
                open: false,
                content: "test".to_owned(),
            }]
        )
    }

    #[test]
    fn cmd_delete() {
        let mut state: State = vec![ToDo {
            open: true,
            content: "test".to_owned(),
        }];
        update(Command::Delete { index: 0 }, &mut state);
        assert_eq!(state, Vec::new())
    }

    #[test]
    fn write_state() {
        let mut console = Vec::new();
        let state: State = vec![ToDo {
            open: true,
            content: "test".to_owned(),
        }];
        let res = super::write_state(&state, &mut console);
        assert_eq!(console, b"TODO\n\n0\topen\ttest\n");
        assert_eq!(res.ok(), Some(()));
    }
}
