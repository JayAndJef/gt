pub mod handler;
pub mod cli;

use cli::{Goto, CommandEnum};
use rusqlite::Connection;

const DB_PATH: &str = "~/.config/gt/db.db3";

fn main() {
    let goto: Goto = argh::from_env();

    let db = Connection::open(DB_PATH).expect("failed to initialize database");
    db.execute(
        "CREATE TABLE IF NOT EXISTS bookmarks (
            name TEXT NOT NULL,
            path TEXT NOT NULL,
            description TEXT
        )",
        (),
    ).expect("failed to create database table");

    match goto.nested {
        CommandEnum::Add(addargs) => todo!(),
        CommandEnum::Get(getargs) => todo!(),
        CommandEnum::Delete(deleteargs) => todo!(),
    }
}
