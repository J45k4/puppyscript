mod parsing;
mod parsing_tests;
mod types;
mod scope;
mod callstack;
mod vm;
mod vm_types;
mod vm_tests;
pub mod debug;

use parsing::Parser;
use types::ASTNode;
pub use vm::PuppyScriptVM;

pub fn parse(code: &str) -> Vec<ASTNode> {
	let mut parser = Parser::new(code);
	parser.parse()
}