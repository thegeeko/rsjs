use std::env;
use std::fs::File;
use std::io::prelude::*;
mod lexer;
mod parser;
fn main() {
	let args_list: Vec<String> = env::args().collect();
	assert_eq!(args_list.len() as i32, 2, "try : cargo run \"file name\"");
	let file_name = &args_list[1];
	let mut file = File::open(file_name).expect("can't open the file");
	let mut code: String = String::new();
	file.read_to_string(&mut code).expect("can't read the file");

	let code_special_parsed = lexer::funcs::replace_special(&mut code);
	let code_lines = lexer::funcs::split_by_lines(&code_special_parsed);
	let code_spaces_parsed = lexer::funcs::split_by_white_space(code_lines);
	let code_tokens = lexer::funcs::gen_tokens(code_spaces_parsed);
	print!("{:#?}", code_tokens);
}
