use std::io;


static TITLE: &'static str = "Python.rs\n[Rust] on OSX\nType statements to explore.\n";
static PROMPT: &'static str = ">>> ";
static SEPERATOR: char = '\n';


#[deriving(Show)]
enum PythonType<'a> {
	None,
    Bool(bool),
    Int(i32),
    Float(f64),
    Str(&'a str),
}

// struct expression;

#[deriving(Show)]
struct Statement {
    token: String,
}

impl Statement {
	fn new(mut line: String) -> Statement {
		let hopefully_newline = line.pop_char().unwrap();
		assert!(hopefully_newline == SEPERATOR);
		
		Statement{token: line}
	}

	fn evaluate(&self) -> Result<PythonType, ()> {
		match self.token.as_slice() {
			"None" => { Ok(None) },
			"True" => { Ok(Bool(true)) },
			"False" => { Ok(Bool(false)) },
			// "False" => { Ok(Bool(false)) },
			// "False" => { Ok(Bool(false)) },
			_ => Err(()),
		}
	}
}

fn main() {
    print!("{}{}", TITLE, PROMPT);
    loop {
    	for line in io::stdin().lines() {
	    	
    		let statement = match line {
				Err(_) => {
			    	print!("{}", PROMPT);
					continue
				},
				Ok(string) => { Statement::new(string) },
			};

			let value = match statement.evaluate() {
				Err(_) => {
			    	print!("{}", PROMPT);
			    	continue
			    },
				Ok(v) => v,
			};

	    	print!("{}\n", value);
	    	print!("{}", PROMPT);
    	}
    }
}
