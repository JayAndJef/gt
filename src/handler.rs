use rusqlite::{Connection, params};
use std::{io, path::PathBuf};

use crate::cli::{AddCommand, DeleteCommand, GetCommand};

fn convert_path(path: PathBuf) -> io::Result<String> {
    let temp = path.canonicalize()?;
    Ok(temp.to_string_lossy().to_string())
}

/// Add an entry into the database given a name, path, and optional desc. Aborts if path could not
/// be found.
pub fn add_entry(args: AddCommand, conn: Connection) {
    let absolute_path = convert_path(args.dir).expect("Could not find path - canonicalization failed");
    conn.execute(
        "INSERT INTO bookmarks VALUES (?1, ?2, ?3)",
        params!(&args.name, absolute_path, &args.desc),
    ).expect("Error inserting into database - perhaps you reused a name?");
}

/// Delete a row from the database given a name
pub fn delete_entry(args: DeleteCommand, conn: Connection) {
    conn.execute(
        "DELETE FROM bookmarks WHERE name = ?1",
        (&args.name,)
    ).expect("Error deleting from database");
}

pub fn get_entry(args: GetCommand, conn: Connection) {
    let mut stmt = conn.prepare(
        "SELECT * FROM bookmarks WHERE name = ?1",
    ).expect("Sqlite call to select bookmarks failed");

    let mut binding = stmt.query((&args.name,))
        .unwrap();
    let row = binding
        .next()
        .unwrap()
        .expect("Could not find bookmark with given name");
    println!("{}", row.get::<usize, String>(1).unwrap(),);
}

pub fn list(conn: Connection) {
    let mut stmt = conn.prepare(
        "SELECT * FROM bookmarks",
    ).expect("Sqlite call to select bookmarks failed");
    let mut rows = stmt.query(()).unwrap();

    while let Some(row) = rows.next().unwrap() {
        println!("{} {} {}",
            row.get::<usize, String>(0).unwrap(),
            row.get::<usize, String>(1).unwrap(),
            row.get::<usize, String>(2).unwrap_or("N/A".to_string()),
        );
    }
}

