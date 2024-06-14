use std::cell::RefCell;
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use crate::interpreter::expression_evaluator::evaluate_expression;
use crate::interpreter::interpreter::TypeVal::{Boolean, Float, Int, Str};
use crate::parsing::ast::{Statement};
use crate::parsing::ast::Statement::{AssignmentStatement, IfElseStatement, IfStatement, PrintStatement, VariableDeclarationStatement, WhileStatement};

#[derive(Debug, Clone, PartialEq)]
pub enum TypeVal {
    Int(i64),
    Float(f64),
    Boolean(bool),
    Str(String),
}

/// A local scope is composed by two fields:
///
/// parent: It contains the reference (counted using Reference Counter) to an eventual father.
///
/// local_variables: it contains all the local variables bound with their value.
///
/// reachable_variables: it contains all the variables seen by the scope.
#[derive(Debug, Default, Clone)]
pub struct Scope {
    parent: Option<Rc<RefCell<Scope>>>,
    local_variables: HashMap<String, TypeVal>,
    reachable_variables: HashSet<String>,
}

impl Scope {
    /// Create new scope.
    pub fn new(parent: Option<Rc<RefCell<Scope>>>, local_variables: HashMap<String, TypeVal>, reachable_variables: HashSet<String>) -> Self {
        Self {
            parent,
            local_variables,
            reachable_variables,
        }
    }

    /// Insert value for the first time in the scope.
    pub fn insert_value(&mut self, variable_name: &str, value: &TypeVal) {
        if let Some(&ref value) = self.local_variables.get(variable_name) {
            panic!("A variable with this name ({}) already exists and it is in scope", variable_name);
        } else {
            match value {
                Int(x) => {
                    if self.reachable_variables.contains(&variable_name.to_string()) {
                        panic!("You are overshadowing ({})", variable_name)
                    }
                    self.local_variables.insert(variable_name.to_string(), Int(x.clone()));
                    self.reachable_variables.insert(variable_name.to_string());
                }
                Float(x) => {
                    if self.reachable_variables.contains(&variable_name.to_string()) {
                        panic!("You are overshadowing ({})", variable_name)
                    }
                    self.local_variables.insert(variable_name.to_string(), Float(x.clone()));
                    self.reachable_variables.insert(variable_name.to_string());
                }
                Boolean(x) => {
                    if self.reachable_variables.contains(&variable_name.to_string()) {
                        panic!("You are overshadowing ({})", variable_name)
                    }
                    self.local_variables.insert(variable_name.to_string(), Boolean(x.clone()));
                    self.reachable_variables.insert(variable_name.to_string());
                }
                Str(x) => {
                    if self.reachable_variables.contains(&variable_name.to_string()) {
                        panic!("You are overshadowing ({})", variable_name)
                    }
                    self.local_variables.insert(variable_name.to_string(), Str(x.clone()));
                    self.reachable_variables.insert(variable_name.to_string());
                }
            }
        }
    }

    /// Get value of a variable.
    ///
    /// If the variable is found then it is returned, if not a mutable reference to the parent is borrowed and the search recursively goes up.
    pub fn get_variable_value(&self, variable_name: &str) -> TypeVal {
        if let Some(&ref value) = self.local_variables.get(variable_name) {
            value.clone()
        } else if let Some(parent) = self.parent.as_ref() {
            parent.borrow_mut().get_variable_value(variable_name)
        } else {
            panic!("{} does not exist", variable_name);
        }
    }

    /// Update value of a variable in the scope
    ///
    /// If the variable is found then it is updated, if not a mutable reference to the parent is borrowed and the search recursively goes up.
    pub fn update_value(&mut self, variable_name: &str, value: &TypeVal, already_found: bool) {
        if let Some(&ref some) = self.local_variables.get(variable_name) {
            match value {
                Int(value) => {
                    self.local_variables.insert(variable_name.to_string(), Int(value.clone()));
                }
                Float(value) => {
                    self.local_variables.insert(variable_name.to_string(), Float(value.clone()));
                }
                Boolean(value) => {
                    self.local_variables.insert(variable_name.to_string(), Boolean(value.clone()));
                }
                Str(value) => {
                    self.local_variables.insert(variable_name.to_string(), Str(value.clone()));
                }
            }
        } else if let Some(parent) = self.parent.as_mut() {
            parent.borrow_mut().update_value(variable_name, &value, already_found);
        } else {
            if !already_found {
                panic!("{} does not exist", variable_name);
            }
        }
    }

    /// Set parent of the given scope
    pub fn set_parent(&mut self, parent: Rc<RefCell<Scope>>) {
        self.parent = Some(parent);
    }

    /// Set variable reachable from self scope
    pub fn set_reachable_variables(&mut self, reachable_variables: HashSet<String>) {
        self.reachable_variables = reachable_variables;
    }
}

/// Start the interpreter
pub fn boot_interpreter(tree: &Vec<Statement>) -> Rc<RefCell<Scope>> {
    let mut main_scope = Rc::new(RefCell::new(Scope::new(Default::default(), Default::default(), Default::default())));
    evaluate_ast(&tree, &mut main_scope)
}

/// AST evaluation
pub fn evaluate_ast(tree: &Vec<Statement>, scope: &mut Rc<RefCell<Scope>>) -> Rc<RefCell<Scope>> {
    for stmt in tree {
        match stmt {
            VariableDeclarationStatement { name, value } => {
                let evaluated_expr = evaluate_expression(&scope, value);
                scope.borrow_mut().insert_value(&name, &evaluated_expr);
            }
            AssignmentStatement { name, value } => {
                let evaluated_expr = evaluate_expression(&scope, value);
                scope.borrow_mut().update_value(&name, &evaluated_expr, false);
            }
            IfStatement { cond, then_part } => {
                let evaluated_expr = evaluate_expression(&scope, cond);
                match evaluated_expr {
                    Boolean(true) => {

                        // Create new local scope
                        let mut new_scope = Rc::new(RefCell::new(Scope::new(Default::default(), Default::default(), Default::default())));
                        // Set parent for local scope
                        new_scope.borrow_mut().set_parent(Rc::clone(&scope));
                        // Update reachable variables
                        new_scope.borrow_mut().set_reachable_variables(scope.borrow().reachable_variables.clone());

                        evaluate_ast(then_part, &mut new_scope);
                        println!("{:#?}", new_scope);
                    }
                    Int(_) => panic!("Int cannot be used as if condition"),
                    Float(_) => panic!("Float cannot be used as if condition"),
                    _ => ()
                }
            }
            IfElseStatement { cond, then_part, else_part } => {
                let evaluated_expr = evaluate_expression(&scope, cond);
                match evaluated_expr {
                    Boolean(true) => {

                        // Create new local scope
                        let mut new_scope = Rc::new(RefCell::new(Scope::new(Default::default(), Default::default(), Default::default())));
                        // Set parent for local scope
                        new_scope.borrow_mut().set_parent(Rc::clone(&scope));
                        // Update reachable variables
                        new_scope.borrow_mut().set_reachable_variables(scope.borrow().reachable_variables.clone());

                        // Execute then_part
                        evaluate_ast(then_part, &mut new_scope);
                        println!("{:#?}", new_scope);
                    }
                    Boolean(false) => {

                        // Create new local scope
                        let mut new_scope = Rc::new(RefCell::new(Scope::new(Default::default(), Default::default(), Default::default())));
                        // Set parent for local scope
                        new_scope.borrow_mut().set_parent(Rc::clone(&scope));
                        // Update reachable variables
                        new_scope.borrow_mut().set_reachable_variables(scope.borrow().reachable_variables.clone());

                        // Execute else_part
                        evaluate_ast(else_part, &mut new_scope);
                        println!("{:#?}", new_scope);
                    }
                    Int(_) => panic!("Int cannot be used as if condition"),
                    Float(_) => panic!("Float cannot be used as if condition"),
                    Str(_) => panic!("A string cannot be used as if condition"),
                }
            }
            WhileStatement { cond, body } => {

                // Create new local scope
                let mut new_scope = Rc::new(RefCell::new(Scope::new(Default::default(), Default::default(), Default::default())));
                // Set parent for local scope
                new_scope.borrow_mut().set_parent(Rc::clone(&scope));
                // Update reachable variables
                new_scope.borrow_mut().set_reachable_variables(scope.borrow().reachable_variables.clone());

                loop {
                    let evaluated_expr = evaluate_expression(&scope, cond);
                    match evaluated_expr {
                        Boolean(true) => {
                            evaluate_ast(&body, &mut new_scope);
                        }
                        Boolean(false) => {
                            break;
                        }
                        Int(_) => panic!("Int cannot be used as if condition"),
                        Float(_) => panic!("Float cannot be used as if condition"),
                        Str(_) => panic!("A string cannot be used as if condition"),
                    }
                }
            }
            PrintStatement{content} => {
                println!("OUTPUT > {:?}", evaluate_expression(&scope, content))
            }
            _ => { println!("{:#?}", stmt) }
        }
    }

    return scope.to_owned();
}
