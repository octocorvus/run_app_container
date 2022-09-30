use crate::wide_string::WideString;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None, trailing_var_arg = true)]
pub struct Cli {
    /// The name of the app container.
    #[clap(short, long)]
    pub container_name: WideString,

    /// The name of the module to be executed.
    #[clap(short, long)]
    pub application_name: String,

    /// The command line to be executed (make sure this is the last argument).
    #[clap(short = 'l', long, multiple_values = true, allow_hyphen_values = true)]
    pub command_line: Vec<String>,

    /// Turn debugging information on.
    #[clap(long, action = clap::ArgAction::Count)]
    pub debug: u8,
}
