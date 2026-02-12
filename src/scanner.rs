// All the possible scan states
enum ScanState {
    NORMAL,
    STRING,
    BACKSLASH,
    COMMENT,
}

// Scanner structure
struct Scanner {
    state : ScanState,
    tokens : Vec<String>,
    current : String,
}

impl Scanner {
    fn new() -> Scanner {
	Scanner {
	    state: ScanState::NORMAL,
	    tokens: Vec::new(),
            current: String::new(),
	}
    }
    
    fn flush(&mut self) {
	if !self.current.is_empty() {
	    self.tokens.push(std::mem::take(&mut self.current));
	}
    }
    
    fn push_char(&mut self, c: char) {
        self.current.push(c);
    }

    fn push_token(&mut self, s: String) {
        self.tokens.push(s);
    }
}

/* Checks if a given char should be treated as a single character lexeme. */
fn is_single(c: char) -> bool {
    match c {
        '(' | ')' | '/' | '*' | '%' | '\'' | ',' | '=' |'\\' => true,
        _ => false,
    }
}

fn is_prefix(c: char) -> bool {
    match c {
        '+' | '-' | '.' => true,
        _ => false,
    }
}

/* Checks if a given char is a whitespace character. */
fn is_whitespace(c: char) -> bool {
    match c {
        ' ' | '\n' | '\t' | '\r' => true,
        _ => false,
    }
}

/*
 * Scans the input and returns an easier to parse version of the input.
 * Uses a finite state machine scannner.
 */
pub fn scan(input : &str) -> Vec<String> {
    let mut scanner = Scanner::new();
    
    for c in input.chars() {
	match scanner.state {
	    /* When in NORMAL, add words as tokens and switch to STRING or COMMENT if necessary */
	    ScanState::NORMAL => {
		if is_single(c) {
		    scanner.flush();
		    scanner.push_token(c.to_string());
		}
		else if is_prefix(c) && scanner.current.is_empty() {
		    scanner.push_char(c);
		}
		else if c == '\"' {
		    scanner.flush();
		    scanner.push_char(c);
		    scanner.state = ScanState::STRING;
		}
		else if c == ';' {
		    scanner.state = ScanState::COMMENT;
		}
		else if is_whitespace(c) {
		    scanner.flush();
		}
		else {
		    scanner.push_char(c);
		}
	    },
	    /* When in STRING, add sentence as a token and switch to BACKSLASH or NORMAL if necessary */
	    ScanState::STRING => {
		if c == '\"' {
		    scanner.push_char(c);
		    scanner.flush();
		    scanner.state = ScanState::NORMAL;
		}
		else if c == '\\' {
		    scanner.state = ScanState::BACKSLASH;
		}
		else {
		    scanner.push_char(c);
		}
	    },
	    /* When in BACKSLASH, process the next char as a special and switch back to STRING */
	    ScanState::BACKSLASH => {
		match c {
		    'n' => scanner.push_char('\n'),
		    't' => scanner.push_char('\t'),
		    'r' => scanner.push_char('\r'),
		    '\\' => scanner.push_char('\\'),
		    '\"' => scanner.push_char('\"'),
		    _ => {
			scanner.push_char('\\');
			scanner.push_char(c);
		    },
		};
		scanner.state = ScanState::STRING;
	    },
	    /* When in COMMENT, ignore input until we reach the newline then switch to NORMAL */
	    ScanState::COMMENT => {
		if c == '\n' {
		    scanner.state = ScanState::NORMAL;
		}
	    }	    
	}
    }
    /* Safety flush */
    scanner.flush();

    scanner.tokens
}
