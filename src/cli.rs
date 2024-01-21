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
#[argh(subcommand, name = "add")]
pub struct GetCommand {
    #[argh(positional)]
    /// name of bookmark
    pub name: String,
}

