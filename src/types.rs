use std::collections::HashMap;
use std::collections::LinkedList;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Symbol(String), // Variables
    Number(f64),    // Numbers
    String(String),
    Boolean(bool), //
    #[allow(dead_code)]
    Quote(Box<Expression>), // This is always a quasiquote! //
    #[allow(dead_code)]
    Unquote(Box<Expression>), //
    List(Vec<Expression>), // Expression -> Expression //
    Nil,
}

pub struct Environment {
    scopes: LinkedList<HashMap<String, (Vec<String>, Expression)>>,
}

impl Environment {
    pub fn new() -> Environment {
        let mut env = Environment {
            scopes: LinkedList::new(),
        };
        env.add_scope();
        env
    }

    pub fn is_defined(&self, s: &str) -> bool {
        for scope in self.scopes.iter().rev() {
            if scope.contains_key(s) {
                return true;
            }
        }
        false
    }

    pub fn get(&self, s: &str) -> (Vec<String>, Expression) {
        for scope in self.scopes.iter().rev() {
            if let Some(result) = scope.get(s) {
                return result.clone();
            }
        }
        (Vec::new(), Expression::Symbol(s.to_string()))
    }

    pub fn local_push(&mut self, s: String, params: Vec<String>, e: Expression) {
        if !self.scopes.is_empty() {
            if let Some(scope) = self.scopes.back_mut() {
                scope.insert(s, (params, e));
            }
        }
    }

    pub fn global_push(&mut self, s: String, params: Vec<String>, e: Expression) {
        if !self.scopes.is_empty() {
            if let Some(scope) = self.scopes.front_mut() {
                scope.insert(s, (params, e));
            }
        }
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop_back();
    }

    pub fn add_scope(&mut self) {
        let scope = HashMap::new();
        self.scopes.push_back(scope);
    }
}
