pub mod handler;
pub mod cli;

use cli::{Goto, CommandEnum};

fn main() {
    let goto: Goto = argh::from_env();

    match goto.nested {
        CommandEnum::Add(addargs) => todo!(),
        CommandEnum::Get(getargs) => todo!(),
    }
}
