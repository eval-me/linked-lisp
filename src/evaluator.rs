use crate::types::{self, Environment};
use std::fmt::Debug;
use std::thread::sleep;
use std::{collections::HashSet, time::Duration};
use types::Expression;

struct Evaluator {
    index: usize,
    env: Environment,
}

impl Evaluator {
    fn new() -> Evaluator {
        Evaluator {
            index: 0,
            env: Environment::new(),
        }
    }
}

fn reduce_addition(vexp: Vec<Expression>, eval: &mut Evaluator) -> Expression {
    let mut vexp: Vec<Expression> = vexp.iter().map(|e| evaluate_expression(e, eval)).collect();
    let (numbers, mut symbols): (Vec<_>, Vec<_>) = vexp
        .drain(1..)
        .partition(|e| matches!(e, Expression::Number(_)));

    let res: f64 = numbers.iter().fold(0.0, |mut acc, e| {
        if let Expression::Number(n) = e {
            acc += n;
        }
        acc
    });

    if symbols.is_empty() {
        Expression::Number(res)
    } else {
        let mut v = vec![Expression::Symbol("+".to_string())];
        if res != 0.0 {
            v.push(Expression::Number(res))
        }
        v.append(&mut symbols);
        Expression::List(v)
    }
}

fn reduce_subtraction(vexp: Vec<Expression>, eval: &mut Evaluator) -> Expression {
    let mut vexp: Vec<Expression> = vexp.iter().map(|e| evaluate_expression(e, eval)).collect();
    if vexp.len() <= 2 {
        if let Expression::Number(n) = vexp[1] {
            Expression::Number(-n)
        } else {
            Expression::List(vexp)
        }
    } else {
        let (numbers, mut symbols): (Vec<_>, Vec<_>) = vexp
            .drain(2..)
            .partition(|e| matches!(e, Expression::Number(_)));

        let res: f64 = numbers.iter().fold(0.0, |mut acc, e| {
            if let Expression::Number(n) = e {
                acc += n;
            }
            acc
        });

        if symbols.is_empty() {
            if let Expression::Number(n) = vexp[1] {
                Expression::Number(n - res)
            } else {
                Expression::List(vec![
                    Expression::Symbol("-".to_string()),
                    vexp[1].clone(),
                    Expression::Number(res),
                ])
            }
        } else {
            if let Expression::Number(n) = vexp[1] {
                let res = n - res;
                let mut v = vec![Expression::Symbol("-".to_string())];
                if res != 0.0 {
                    v.push(Expression::Number(res))
                }
                v.append(&mut symbols);
                Expression::List(v)
            } else {
                let mut v = vec![Expression::Symbol("-".to_string()), vexp[1].clone()];
                v.append(&mut symbols);
                if res != 0.0 {
                    v.push(Expression::Number(res))
                }
                Expression::List(v)
            }
        }
    }
}

fn reduce_multiplication(vexp: Vec<Expression>, eval: &mut Evaluator) -> Expression {
    let mut vexp: Vec<Expression> = vexp.iter().map(|e| evaluate_expression(e, eval)).collect();
    let (numbers, mut symbols): (Vec<_>, Vec<_>) = vexp
        .drain(1..)
        .partition(|e| matches!(e, Expression::Number(_)));

    let res: f64 = numbers.iter().fold(1.0, |mut acc, e| {
        if let Expression::Number(n) = e {
            acc *= n;
        }
        acc
    });

    if symbols.is_empty() {
        Expression::Number(res)
    } else {
        let mut v = vec![Expression::Symbol("*".to_string())];
        if res != 1.0 {
            v.push(Expression::Number(res))
        }
        v.append(&mut symbols);
        Expression::List(v)
    }
}

fn reduce_division(vexp: Vec<Expression>, eval: &mut Evaluator) -> Expression {
    let mut vexp: Vec<Expression> = vexp.iter().map(|e| evaluate_expression(e, eval)).collect();
    if vexp.len() <= 2 {
        if let Expression::Number(n) = vexp[1] {
            Expression::Number(1.0 / n)
        } else {
            Expression::List(vexp)
        }
    } else {
        let (numbers, mut symbols): (Vec<_>, Vec<_>) = vexp
            .drain(2..)
            .partition(|e| matches!(e, Expression::Number(_)));

        let res: f64 = numbers.iter().fold(1.0, |mut acc, e| {
            if let Expression::Number(n) = e {
                acc *= n;
            }
            acc
        });

        if symbols.is_empty() {
            if let Expression::Number(n) = vexp[1] {
                Expression::Number(n / res)
            } else {
                Expression::List(vec![
                    Expression::Symbol("/".to_string()),
                    vexp[1].clone(),
                    Expression::Number(res),
                ])
            }
        } else {
            if let Expression::Number(n) = vexp[1] {
                let res = n / res;
                let mut v = vec![Expression::Symbol("/".to_string())];
                if res != 1.0 {
                    v.push(Expression::Number(res));
                }
                v.append(&mut symbols);
                Expression::List(v)
            } else {
                let mut v = vec![Expression::Symbol("/".to_string()), vexp[1].clone()];
                v.append(&mut symbols);
                if res != 1.0 {
                    v.push(Expression::Number(res));
                }
                Expression::List(v)
            }
        }
    }
}

fn reduce_equality(vexp: Vec<Expression>, eval: &mut Evaluator) -> Expression {
    let mut vexp: Vec<Expression> = vexp.iter().map(|e| evaluate_expression(e, eval)).collect();
    if vexp.len() <= 1 {
        return Expression::List(vexp);
    }
    if vexp.len() == 2 {
        return Expression::Boolean(true);
    }
    let (symbols, literals): (Vec<_>, Vec<_>) = vexp
        .drain(1..)
        .partition(|e| matches!(e, Expression::Symbol(_)));

    let is_match = literals.is_empty() || literals.iter().all(|e| e == &literals[0]);
    if symbols.is_empty() || !is_match {
        Expression::Boolean(is_match)
    } else {
        let mut ret = vec![Expression::Symbol("=".to_string())];
        if !literals.is_empty() {
            ret.push(literals[0].clone());
        }
        let mut seen = HashSet::new();
        let mut count = 0;
        for e in symbols.iter() {
            if let Expression::Symbol(s) = e {
                if !seen.contains(s) {
                    seen.insert(s);
                    count += 1;
                    ret.push(Expression::Symbol(s.clone()))
                }
            }
        }
        // If there's only one symbol it's obviously equal to itself!
        if count == 1 && literals.len() == 0 {
            return Expression::Boolean(true);
        }
        Expression::List(ret)
    }
}

fn evaluate_define(vexp: Vec<Expression>, eval: &mut Evaluator) -> Expression {
    if vexp.len() <= 2 {
        return Expression::List(vexp);
    } else {
        if let Expression::Symbol(s) = vexp[1].clone() {
            let result = evaluate_expression(&vexp[2], eval);
            eval.env.global_push(s, Vec::new(), result.clone());
            return result;
        } else if let Expression::List(v) = vexp[1].clone() {
            let params = v.iter().try_fold(
                Vec::with_capacity(v.len()),
                |mut list: Vec<String>, e| match e {
                    Expression::Symbol(s) => {
                        list.push(s.clone());
                        Ok(list)
                    }
                    _ => Err(()),
                },
            );

            match params {
                Ok(list) => {
                    let result = evaluate_expression(&vexp[2], eval);
                    eval.env
                        .global_push(list[0].clone(), list[1..].to_vec(), result.clone());
                    return result;
                }
                Err(_) => Expression::List(vexp),
            }
        } else {
            Expression::List(vexp)
        }
    }
}

fn apply(vexp: Vec<Expression>, eval: &mut Evaluator) -> Expression {
    if let Expression::Symbol(s) = &vexp[0] {
        eval.env.add_scope(); // Add a scope for the current evaluation!
        if eval.env.is_defined(&s) {
            let (params, e) = eval.env.get(&s);
            let mappings = params.iter().zip(&vexp[1..]);
            for (param_name, arg_val) in mappings {
                let arg_val = evaluate_expression(arg_val, eval);
                eval.env.local_push(param_name.clone(), Vec::new(), arg_val);
            }
            let ret = evaluate_expression(&e, eval);
            eval.env.pop_scope();
            ret
        } else {
            eval.env.pop_scope();
            Expression::List(vexp)
        }
    } else {
        Expression::List(vexp)
    }
}

fn evaluate_cond(vexp: Vec<Expression>, eval: &mut Evaluator) -> Expression {
    if vexp.len() < 2 {
        return Expression::List(vexp);
    } else {
        let mut unevaluated = Vec::new();
        for condition in vexp[1..].iter() {
            match condition {
                Expression::List(l) => {
                    if l.len() == 2 {
                        let check = evaluate_expression(&l[0], eval);
                        if check == Expression::Boolean(true) {
                            if unevaluated.len() == 0 {
                                return evaluate_expression(&l[1], eval);
                            } else {
                                unevaluated.push(condition.clone());
                            }
                        } else if check != Expression::Boolean(false) {
                            unevaluated.push(condition.clone());
                        }
                    } else {
                        unevaluated.push(condition.clone());
                    }
                }
                _ => unevaluated.push(condition.clone()),
            }
        }
        if unevaluated.len() != 0 {
            let mut v = vec![Expression::Symbol("cond".to_string())];
            v.push(Expression::List(unevaluated));
            Expression::List(v)
        } else {
            Expression::Nil
        }
    }
}

// fn evaluate_lambda(vexp: Vec<Expression>, eval: &mut Evaluator) -> Expression {

// }

fn evaluate_list(vexp: Vec<Expression>, eval: &mut Evaluator) -> Expression {
    if let Some(Expression::Symbol(s)) = vexp.get(0) {
        match s.as_str() {
            "+" => reduce_addition(vexp, eval),
            "-" => reduce_subtraction(vexp, eval),
            "*" => reduce_multiplication(vexp, eval),
            "/" => reduce_division(vexp, eval),
            "=" => reduce_equality(vexp, eval),
            "define" => evaluate_define(vexp, eval),
            "cond" => evaluate_cond(vexp, eval),
            // "lambda" => evaluate_lambda(vexp, eval),
            _ => apply(vexp, eval),
        }
    } else {
        let new_vexp: Vec<Expression> = vexp.iter().map(|e| evaluate_expression(e, eval)).collect();
        Expression::List(new_vexp)
    }
}

fn evaluate_expression(expression: &Expression, eval: &mut Evaluator) -> Expression {
    let ret: Expression;
    match expression {
        Expression::Symbol(s) => ret = eval.env.get(&s).1,
        Expression::Number(n) => ret = Expression::Number(*n),
        Expression::Boolean(b) => ret = Expression::Boolean(*b),
        Expression::List(v) => ret = evaluate_list(v.clone(), eval),
        Expression::Nil => ret = Expression::Nil,
        _ => ret = expression.clone(),
    }
    ret
}

pub fn evaluate(expressions: Vec<Expression>) -> Expression {
    let mut eval = Evaluator::new();

    let mut e = Expression::Nil;
    while eval.index < expressions.len() {
        if let Some(expression) = expressions.get(eval.index) {
            e = evaluate_expression(&expression, &mut eval);
            eval.index += 1;
        }
    }
    e
}
