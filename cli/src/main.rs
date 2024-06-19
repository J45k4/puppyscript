use std::fs;
use std::path::Path;

use args::Args;
use args::Command;
use clap::Parser;
use puppy_script::debug::pretty_print_bytecode;
use puppy_script::PuppyScriptVM;
mod args;

fn get_code(path: &str) -> String {
	let path2 = Path::new(path);

	if path2.exists() {
		fs::read_to_string(path2).unwrap()
	} else {
		path.to_string()
	}
}

fn main() {
	let args = Args::parse();
	let mut vm = PuppyScriptVM::new();
	vm.log = args.log;

	match args.command {
		Command::Run { path } => {
			let code = get_code(&path);
			vm.run_code(&code);
		}
		Command::Ast { path, pretty } => {
			let code = get_code(&path);
			let ast = puppy_script::parse(&code);

			if pretty {
				println!("{:#?}", ast);
				return;
			} else {
				println!("{:?}", ast);
			}
		}
		Command::Bytecode { path } => {
			let code = get_code(&path);
			let blk = vm.compile_code(&code);
			pretty_print_bytecode(&vm, blk);
		}
	}
}
