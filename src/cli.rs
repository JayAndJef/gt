use argh::FromArgs;
use std::path::PathBuf;

#[derive(FromArgs, PartialEq, Debug)]
/// Bookmark and access directories
pub struct Goto {
    #[argh(subcommand)]
    pub nested: CommandEnum,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum CommandEnum {
    Add(AddCommand),
    Get(GetCommand),
    Delete(DeleteCommand),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Add a bookmark to a directory
#[argh(subcommand, name = "add")]
pub struct AddCommand {
    #[argh(option)]
    /// optional description of bookmark
    pub desc: Option<String>,

    #[argh(positional)]
    /// name of bookmark
    pub name: String,

    #[argh(positional)]
    /// relative directory to link to
    pub dir: PathBuf,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Retrieve the path of a bookmark
#[argh(subcommand, name = "get")]
pub struct GetCommand {
    #[argh(positional)]
    /// name of bookmark
    pub name: String,
}

#[derive(FromArgs, PartialEq, Debug)]
/// Delete a bookmark
#[argh(subcommand, name = "delete")]
pub struct DeleteCommand{
    #[argh(positional)]
    /// name of bookmark
    pub name: String,
}

