use crate::types::Expression;

pub fn print(e: Expression) {
    match e {
	Expression::Symbol(s) => print!("{}", s),
	Expression::Number(n) => print!("{}", n),
	Expression::String(s) => print!("\"{}\"", s),
	Expression::Boolean(b) => print!("{}", b),
	Expression::List(v) => {
	    print!("{}", '(');
	    for (i, el) in v.iter().enumerate() {
                print(el.clone());
                // Only print a space if it's not the last element
                if i < v.len() - 1 {
                    print!(" ");
                }
            }
	    print!("{}", ')');
	}
	Expression::Nil => print!("nil"),
	_ => println!("Well that was weird...")
    }
}
