use clap::Parser;
use clap::Subcommand;


#[derive(Debug, Parser)]
#[clap(name = "ps")]
pub struct Args {
    #[clap(subcommand)]
    pub command: Command,
	#[clap(long, default_value = "0")]
	pub log: usize
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Run {
        path: String,
    },
	Ast {
		path: String,
		#[clap(default_value = "false", long)]
		pretty: bool
	},
	Bytecode {
		path: String,
	}
}