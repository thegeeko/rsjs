pub mod types {
	#[derive(Clone)]
	pub enum Type {
		String { content: String },
		Number { value: f32 },
		Function { block: Block, params: Vec<Var> },
		Undefined,
	}

	#[derive(Clone)]
	pub enum Operation {
		Assignment {
			first_ele: Var,
			second_ele: Option<Var>,
			value: Option<&'static str>,
		},
		Subtraction {
			first_ele: Var,
			second_ele: Var,
		},
		Addition {
			first_ele: Var,
			second_ele: Var,
		},
		Call {
			funtion: Var,
			params_passed: Option<Vec<Var>>,
		},
	}

	#[derive(Clone)]
	pub struct Var {
		typ: Type,
		pub name: &'static str,
	}
	impl Var {
		pub fn new(typ: Type, name: &'static str) -> Self {
			Self { typ, name }
		}
		pub fn dmy() -> Self {
			Var {
				typ: Type::Undefined,
				name: "dmy",
			}
		}
		fn define(self, block: &mut Block) -> &Block {
			block.vars.push(self);
			block
		}
	}

	#[derive(Clone)]
	pub struct Block {
		pub vars: Vec<Var>,
		sub_blocks: Vec<Option<Box<Block>>>,
		operations: Vec<Operation>,
	}
	impl Block {
		pub fn new(
			vars: Vec<Var>,
			sub_blocks: Vec<Option<Box<Block>>>,
			operations: Vec<Operation>,
		) -> Self {
			return Self {
				vars,
				sub_blocks,
				operations,
			};
		}
	}
}

pub mod funcs {

	use super::types::*;
	use crate::lexer;

	fn check_if_exist(var_name: &str, block: &Block) -> Option<Var> {
		if lexer::types::KEYWORDS_LIST.contains(&var_name) {
			return None;
		}
		for var in &block.vars {
			if var_name == var.name {
				return Some(var.clone());
			}
		}
		None
	}

	fn check_if_not_preserved(var_name: &str) -> bool {
		!lexer::types::KEYWORDS_LIST.contains(&var_name)
	}

	pub fn init() -> Block {
		Block::new(Vec::new(), Vec::new(), Vec::new())
	}

	pub fn keyword_parser(keyword: &str, current_block: Block) {}

	pub fn main_parser(init_block: &mut Block, tokens: Vec<lexer::types::Token>) {
		let mut current_block = init_block;
		let mut token_iter = tokens.iter();
		loop {
			let mut token = token_iter.next();
			if let Some(token) = token {
				match token.token_type {
					Keyword => match &token.value[..] {
						"let" => {
							//assignment <<let>>
							let var = match token_iter.next() {
								//next token => should be var name
								Some(var_name) => {
									if check_if_not_preserved(&var_name.value[..]) {
										if let Some(current_var) = check_if_exist(&var_name.value[..], current_block) {
											//var already exist in current scoop
											current_var
										} else {

											// var not exist declare new one
										}
									} else {
										print!("err can't use that as var name");
										break;
									}
								}
								None => break,
							};
						}
						_ => {
							print!("{} is not implemented yet", token.value)
						}
					},
				}
			}
		}
	}
}
