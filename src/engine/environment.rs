use std::collections::HashMap;

use super::AST;

pub struct Environment<'a> {
    bindings: HashMap<String, AST>,
    parent_environment: Option<&'a Environment<'a>>
}

impl<'a> Environment<'a> {
    pub fn new(parent_environment: Option<&'a Environment>) -> Environment<'a> {
        Environment {
            bindings: HashMap::new(),
            parent_environment
        }
    }

    pub fn dump(&self) {
        println!("{:#?}", self.bindings);
        if let Some(parent) = self.parent_environment {
            parent.dump();
        }
    }

    fn _has(&self, binding: &String) -> bool {
        self.bindings.contains_key(binding)
    }

    pub fn get(&self, binding: &String) -> AST {
        if let Some(ast) = self.bindings.get(binding) {
            ast.clone()
        }
        else if let Some(ref env) = self.parent_environment {
            env.get(binding)
        }
        else {
            self.dump();
            panic!(format!("Undefined variable '{}'", binding));
        }
    }

    /*pub fn set(&mut self, name: &String, value: AST) {
        if self.has(name) {
            self.bindings.insert(name.clone(), value);
        }
        else if let Some(ref mut env) = self.parent_environment {
            env.set(name, value);
        }
        else {
            panic!(format!("Undeclared variable '{}'", name));
        }
    }*/

    pub fn def(&mut self, name: &String, value: AST) {
        self.bindings.insert(name.clone(), value);
    }
}
