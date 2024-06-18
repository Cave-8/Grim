use std::cell::RefCell;
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use colored::{Colorize};
use crate::interpreter::expression_evaluator::evaluate_expression;
use crate::interpreter::interpreter::TypeVal::{Boolean, Float, Int, Str};
use crate::parsing::ast::{Statement};
use crate::parsing::ast::Statement::{AssignmentStatement, FunctionDeclaration, IfElseStatement, IfStatement, PrintStatement, ReturnStatement, VariableDeclarationStatement, WhileStatement};

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
    pub parent: Option<Rc<RefCell<Scope>>>,
    pub local_variables: HashMap<String, TypeVal>,
    pub local_functions: HashMap<String, (Vec<String>, Vec<Statement>)>,
    pub reachable_variables: HashSet<String>,
    pub reachable_functions: HashSet<String>,
}

impl Scope {
    /// Insert value for the first time in the scope.
    pub fn insert_value(&mut self, variable_name: &str, value: &TypeVal) {
        if let Some(&ref _value) = self.local_variables.get(variable_name) {
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

    /// Insert function for the first time in the scope.
    pub fn insert_function(&mut self, function_name: &str, arguments: &Vec<String>, body: &Vec<Statement>) {
        if let Some(&ref _value) = self.local_functions.get(function_name) {
            panic!("A function with this name ({}) already exists and it is in scope", function_name);
        } else {
            self.local_functions.insert(function_name.to_string(), (arguments.clone(), body.clone()));
            self.reachable_functions.insert(function_name.to_string());
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

    /// Get argument list and body of a function.
    ///
    /// If the function is found then it is returned, if not a mutable reference to the parent is borrowed and the search recursively goes up.
    pub fn get_function_info(&self, variable_name: &str) -> (Vec<String>, Vec<Statement>) {
        if let Some(&ref value) = self.local_functions.get(variable_name) {
            value.clone()
        } else if let Some(parent) = self.parent.as_ref() {
            parent.borrow_mut().get_function_info(variable_name)
        } else {
            panic!("{} does not exist", variable_name);
        }
    }

    /// Update value of a variable in the scope
    ///
    /// If the variable is found then it is updated, if not a mutable reference to the parent is borrowed and the search recursively goes up.
    pub fn update_value(&mut self, variable_name: &str, value: &TypeVal) -> Result<String, String>{
        if let Some(&ref _some) = self.local_variables.get(variable_name) {
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
            parent.borrow_mut().update_value(variable_name, &value);
        } else {
            return Err(format!("{} does not exist", variable_name));
        }
        Ok("Correct assignment".to_string())
    }

    /// Set parent of the given scope
    pub fn set_parent(&mut self, parent: Rc<RefCell<Scope>>) {
        self.parent = Some(parent);
    }

    /// Set variable reachable from self scope
    pub fn set_reachable_variables(&mut self, reachable_variables: HashSet<String>) {
        self.reachable_variables = reachable_variables;
    }

    /// Set functions reachable from self scope
    pub fn set_reachable_functions(&mut self, reachable_functions: HashSet<String>) {
        self.reachable_functions = reachable_functions;
    }
}

/// Start the interpreter
pub fn boot_interpreter(tree: &Vec<Statement>) -> Result<Rc<RefCell<Scope>>, String> {
    let mut main_scope = Rc::new(RefCell::new(Scope::default()));
    evaluate_ast(&tree, &mut main_scope)
}

/// AST evaluation
pub fn evaluate_ast(tree: &Vec<Statement>, scope: &mut Rc<RefCell<Scope>>) -> Result<Rc<RefCell<Scope>>, String> {
    for stmt in tree {
        match stmt {
            VariableDeclarationStatement { name, value } => {
                match evaluate_expression(&scope, value) {
                    Ok(evaluated_expr) => scope.borrow_mut().insert_value(&name, &evaluated_expr),
                    Err(err) => return Err(format! {"Error during variable declaration\n{}\n", err}),
                }
            }
            AssignmentStatement { name, value } => {
                match evaluate_expression(&scope, value) {
                    Ok(evaluated_expr) => {
                        match scope.borrow_mut().update_value(&name, &evaluated_expr) {
                            Ok(_) => (),
                            Err(err) => return Err(format! {"Error during variable assignment\n{}\n", err}),
                        }
                    }
                    Err(err) => return Err(format! {"Error during variable assignment\n{}\n", err}),
                }
            }
            IfStatement { cond, then_part } => {
                let evaluated_expr = evaluate_expression(&scope, cond);
                match evaluated_expr {
                    Ok(Boolean(true)) => {

                        // Create new local scope
                        let mut new_scope = Rc::new(RefCell::new(Scope::default()));
                        // Set parent for local scope
                        new_scope.borrow_mut().set_parent(Rc::clone(&scope));
                        // Update reachable variables
                        new_scope.borrow_mut().set_reachable_variables(scope.borrow().reachable_variables.clone());
                        // Update reachable functions
                        new_scope.borrow_mut().set_reachable_functions(scope.borrow().reachable_functions.clone());

                        // Execute then_part
                        match evaluate_ast(then_part, &mut new_scope) {
                            Ok(_) => (),
                            Err(err) => return Err(format! {"Error during if evaluation\n{}\n", err}),
                        }
                    }
                    Ok(Int(_)) => return Err("Int cannot be used as if condition".red().to_string()),
                    Ok(Float(_)) => return Err("Float cannot be used as if condition".red().to_string()),
                    Ok(Str(_)) => return Err("Str cannot be used as if condition".red().to_string()),
                    Err(err) => return Err(format! {"Error during if evaluation\n{}\n", err}),
                    _ => {}
                }
            }
            IfElseStatement { cond, then_part, else_part } => {
                let evaluated_expr = evaluate_expression(&scope, cond);
                match evaluated_expr {
                    Ok(Boolean(true)) => {

                        // Create new local scope
                        let mut new_scope = Rc::new(RefCell::new(Scope::default()));
                        // Set parent for local scope
                        new_scope.borrow_mut().set_parent(Rc::clone(&scope));
                        // Update reachable variables
                        new_scope.borrow_mut().set_reachable_variables(scope.borrow().reachable_variables.clone());
                        // Update reachable functions
                        new_scope.borrow_mut().set_reachable_functions(scope.borrow().reachable_functions.clone());

                        // Execute then_part
                        match evaluate_ast(then_part, &mut new_scope) {
                            Ok(_) => (),
                            Err(err) => return Err(format! {"Error during if-else evaluation\n{}\n", err}),
                        }
                    }
                    Ok(Boolean(false)) => {

                        // Create new local scope
                        let mut new_scope = Rc::new(RefCell::new(Scope::default()));
                        // Set parent for local scope
                        new_scope.borrow_mut().set_parent(Rc::clone(&scope));
                        // Update reachable variables
                        new_scope.borrow_mut().set_reachable_variables(scope.borrow().reachable_variables.clone());
                        // Update reachable functions
                        new_scope.borrow_mut().set_reachable_functions(scope.borrow().reachable_functions.clone());

                        // Execute else_part
                        match evaluate_ast(else_part, &mut new_scope) {
                            Ok(_) => (),
                            Err(err) => return Err(format! {"Error during if-else evaluation\n{}\n", err}),
                        }
                    }
                    Ok(Int(_)) => return Err("Int cannot be used as if condition".red().to_string()),
                    Ok(Float(_)) => return Err("Float cannot be used as if condition".red().to_string()),
                    Ok(Str(_)) => return Err("Str cannot be used as if condition".red().to_string()),
                    Err(err) => return Err(format! {"Error during if-else evaluation\n{}\n", err}),
                }
            }
            WhileStatement { cond, body } => {

                // Create new local scope
                let mut new_scope = Rc::new(RefCell::new(Scope::default()));
                // Set parent for local scope
                new_scope.borrow_mut().set_parent(Rc::clone(&scope));
                // Update reachable variables
                new_scope.borrow_mut().set_reachable_variables(scope.borrow().reachable_variables.clone());
                // Update reachable functions
                new_scope.borrow_mut().set_reachable_functions(scope.borrow().reachable_functions.clone());

                loop {
                    let evaluated_expr = evaluate_expression(&scope, cond);
                    match evaluated_expr {
                        Ok(Boolean(true)) => {
                            match evaluate_ast(body, &mut new_scope) {
                                Ok(_) => (),
                                Err(err) => return Err(format! {"Error during while evaluation\n{}\n", err}),
                            }
                        }
                        Ok(Boolean(false)) => {
                            break;
                        }
                        Ok(Int(_)) => return Err("Int cannot be used as if condition".red().to_string()),
                        Ok(Float(_)) => return Err("Float cannot be used as if condition".red().to_string()),
                        Ok(Str(_)) => return Err("Str cannot be used as if condition".red().to_string()),
                        Err(err) => return Err(format! {"Error during while evaluation\n{}\n", err}),
                    }
                }
            }
            FunctionDeclaration { name, parameters, body } => {
                scope.borrow_mut().insert_function(name, parameters, body);
            }

            ReturnStatement { value } => {
                match evaluate_expression(&scope, value) {
                    Ok(res) => scope.borrow_mut().local_variables.insert("return".to_string(), res),
                    Err(err) => return Err(format! {"Error during return statement\n{}\n", err}),
                };
                return Ok(scope.to_owned());
            }

            PrintStatement{content} => {
                match evaluate_expression(&scope, content) {
                    Ok(x) => println!("OUTPUT > {:?}", x),
                    Err(x) => return Err(x)
                }
            }
            _ => { println!("{:#?}", stmt) }
        }
    }
    return Ok(scope.to_owned());
}
