pub mod handler;
pub mod cli;

use cli::{Goto, CommandEnum};
use handler::{add_entry, delete_entry, get_entry, list};
use rusqlite::Connection;

const DB_PATH: &str = "./db.db3";

fn main() {
    let goto: Goto = argh::from_env();

    let db = Connection::open(DB_PATH).expect("failed to initialize database");
    db.execute(
        "CREATE TABLE IF NOT EXISTS bookmarks (
            name TEXT NOT NULL UNIQUE,
            path TEXT NOT NULL,
            description TEXT
        )",
        (),
    ).expect("failed to create database table");

    match goto.nested {
        CommandEnum::Add(addargs) => add_entry(addargs, db),
        CommandEnum::Get(getargs) => get_entry(getargs, db),
        CommandEnum::Delete(deleteargs) => delete_entry(deleteargs, db),
        CommandEnum::List(_) => list(db),
    }
}
