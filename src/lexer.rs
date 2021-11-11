pub mod types {
	pub const KEYWORDS_LIST: [&str; 91] = [
		"await",
		"break",
		"case",
		"catch",
		"class",
		"const",
		"continue",
		"debugger",
		"default",
		"delete",
		"do",
		"else",
		"enum",
		"export",
		"extends",
		"false",
		"finally",
		"for",
		"function",
		"if",
		"implements",
		"import",
		"in",
		"instanceof",
		"interface",
		"let",
		"new",
		"null",
		"package",
		"private",
		"protected",
		"public",
		"return",
		"super",
		"switch",
		"static",
		"this",
		"throw",
		"try",
		"True",
		"typeof",
		"var",
		"void",
		"while",
		"with",
		"yieldawait",
		"break",
		"case",
		"catch",
		"class",
		"const",
		"continue",
		"debugger",
		"default",
		"delete",
		"do",
		"else",
		"enum",
		"export",
		"extends",
		"false",
		"finally",
		"for",
		"function",
		"if",
		"implements",
		"import",
		"in",
		"instanceof",
		"interface",
		"let",
		"new",
		"null",
		"package",
		"private",
		"protected",
		"public",
		"return",
		"super",
		"switch",
		"static",
		"this",
		"throw",
		"try",
		"True",
		"typeof",
		"var",
		"void",
		"while",
		"with",
		"yield",
	];

	const OPERATORS_LIST: [&str; 10] = ["||", "&&", ";", ">", "=", "==", "===", "<", "<=", ">="];

	#[derive(Debug, Clone, Copy)]
	pub enum TokenTypes {
		Name,
		Keyword,
		Symbol,
		Operator,
	}
	#[derive(Debug)]
	pub struct Token {
		pub value: String,
		pub token_type: TokenTypes,
	}
	impl Token {
		pub fn new(word: &str) -> Self {
			let mut token = Token {
				value: word.to_string(),
				token_type: TokenTypes::Name,
			};
			if KEYWORDS_LIST.contains(&word) {
				token.token_type = TokenTypes::Keyword;
			} else if OPERATORS_LIST.contains(&word) {
				token.token_type = TokenTypes::Operator;
			}
			token
		}
	}
}

pub mod funcs {

	use super::types::*;

	pub fn replace_special(code: &mut String) -> String {
		let mut parsed_string = String::new();
		parsed_string = code.replace(";", " ");
		parsed_string = parsed_string.replace("\"", " \" ");
		parsed_string
	}

	pub fn split_by_lines(code: &String) -> Vec<&str> {
		let parsed_string: Vec<&str> = code.lines().collect();
		parsed_string
	}

	pub fn split_by_white_space(code_lines: Vec<&str>) -> Vec<&str> {
		let mut parsed_string: Vec<&str> = Vec::new();
		for line in code_lines {
			let mut splited: Vec<&str> = line.split_whitespace().collect();
			parsed_string.append(&mut splited);
		}
		parsed_string
	}

	pub fn gen_tokens(code_words: Vec<&str>) -> Vec<Token> {
		let mut tokens: Vec<Token> = Vec::new();
		for word in code_words {
			tokens.push(Token::new(word));
		}
		tokens
	}
}
