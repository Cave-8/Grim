use crate::interpreter::expression_evaluator::evaluate_expression;
use crate::interpreter::interpreter::TypeVal::{Boolean, Float, Int, Str};
use crate::parsing::ast::Statement::{
    AssignmentStatement, FunctionDeclaration, IfElseStatement, IfStatement, InputStatement,
    PrintStatement, ReturnStatement, VariableDeclarationStatement, WhileStatement,
};
use crate::parsing::ast::{Expression, Statement};
use colored::Colorize;
use std::cell::RefCell;
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::io;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum TypeVal {
    Int(i64),
    Float(f64),
    Boolean(bool),
    Str(String),
}

impl Default for TypeVal {
    fn default() -> Self {
        Int(0)
    }
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
    pub return_value: TypeVal,
}

impl Scope {
    /// Insert value for the first time in the scope.
    pub fn insert_value(&mut self, variable_name: &str, value: &TypeVal) -> Result<String, String> {
        if let Some(&ref _value) = self.local_variables.get(variable_name) {
            Err(format!(
                "A variable with this name ({}) already exists and it is in scope",
                variable_name
            ))
        } else {
            match value {
                Int(x) => {
                    if self
                        .reachable_variables
                        .contains(&variable_name.to_string())
                    {
                        return Err(format!("You are overshadowing ({})", variable_name));
                    }
                    self.local_variables
                        .insert(variable_name.to_string(), Int(x.clone()));
                    self.reachable_variables.insert(variable_name.to_string());
                }
                Float(x) => {
                    if self
                        .reachable_variables
                        .contains(&variable_name.to_string())
                    {
                        return Err(format!("You are overshadowing ({})", variable_name));
                    }
                    self.local_variables
                        .insert(variable_name.to_string(), Float(x.clone()));
                    self.reachable_variables.insert(variable_name.to_string());
                }
                Boolean(x) => {
                    if self
                        .reachable_variables
                        .contains(&variable_name.to_string())
                    {
                        return Err(format!("You are overshadowing ({})", variable_name));
                    }
                    self.local_variables
                        .insert(variable_name.to_string(), Boolean(x.clone()));
                    self.reachable_variables.insert(variable_name.to_string());
                }
                Str(x) => {
                    if self
                        .reachable_variables
                        .contains(&variable_name.to_string())
                    {
                        return Err(format!("You are overshadowing ({})", variable_name));
                    }
                    self.local_variables
                        .insert(variable_name.to_string(), Str(x.clone()));
                    self.reachable_variables.insert(variable_name.to_string());
                }
            }
            Ok("Correct insertion".to_string())
        }
    }

    /// Insert function for the first time in the scope.
    pub fn insert_function(
        &mut self,
        function_name: &str,
        arguments: &Vec<String>,
        body: &Vec<Statement>,
    ) -> Result<String, String> {
        if let Some(&ref _value) = self.local_functions.get(function_name) {
            Err(format!(
                "A function with this name ({}) already exists and it is in scope",
                function_name
            ))
        } else {
            self.local_functions
                .insert(function_name.to_string(), (arguments.clone(), body.clone()));
            self.reachable_functions.insert(function_name.to_string());
            Ok("Correct insertion".to_string())
        }
    }

    /// Get value of a variable.
    ///
    /// If the variable is found then it is returned, if not a mutable reference to the parent is borrowed and the search recursively goes up.
    pub fn get_variable_value(&self, variable_name: &str) -> Result<TypeVal, String> {
        if let Some(&ref value) = self.local_variables.get(variable_name) {
            Ok(value.clone())
        } else if let Some(parent) = self.parent.as_ref() {
            parent.borrow_mut().get_variable_value(variable_name)
        } else {
            return Err(format!("Variable {} does not exist", variable_name));
        }
    }

    /// Get argument list and body of a function.
    ///
    /// If the function is found then it is returned, if not a mutable reference to the parent is borrowed and the search recursively goes up.
    pub fn get_function_info(
        &self,
        function_name: &str,
    ) -> Result<(Vec<String>, Vec<Statement>), String> {
        if let Some(&ref value) = self.local_functions.get(function_name) {
            Ok(value.clone())
        } else if let Some(parent) = self.parent.as_ref() {
            parent.borrow_mut().get_function_info(function_name)
        } else {
            return Err(format! {"Function {} does not exist", function_name});
        }
    }

    /// Update value of a variable in the scope
    ///
    /// If the variable is found then it is updated, if not a mutable reference to the parent is borrowed and the search recursively goes up.
    pub fn update_value(&mut self, variable_name: &str, value: &TypeVal) -> Result<String, String> {
        if let Some(&ref _some) = self.local_variables.get(variable_name) {
            match value {
                Int(value) => {
                    self.local_variables
                        .insert(variable_name.to_string(), Int(value.clone()));
                }
                Float(value) => {
                    self.local_variables
                        .insert(variable_name.to_string(), Float(value.clone()));
                }
                Boolean(value) => {
                    self.local_variables
                        .insert(variable_name.to_string(), Boolean(value.clone()));
                }
                Str(value) => {
                    self.local_variables
                        .insert(variable_name.to_string(), Str(value.clone()));
                }
            }
        } else if let Some(parent) = self.parent.as_mut() {
            parent.borrow_mut().update_value(variable_name, &value)?;
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

    /// Set return value of current scope
    pub fn set_return_value(&mut self, return_value: &TypeVal) {
        self.return_value = return_value.clone();
        if let Some(parent) = self.parent.as_mut() {
            parent.borrow_mut().set_return_value(&return_value);
        }
    }
}

/// Start the interpreter
pub fn boot_interpreter(tree: &Vec<Statement>) -> Result<Rc<RefCell<Scope>>, String> {
    let mut main_scope = Rc::new(RefCell::new(Scope::default()));
    evaluate_ast(&tree, &mut main_scope)
}

impl PartialEq<TypeVal> for &TypeVal {
    fn eq(&self, other: &TypeVal) -> bool {
        self == other
    }
}

/// AST evaluation
pub fn evaluate_ast(
    tree: &Vec<Statement>,
    scope: &mut Rc<RefCell<Scope>>,
) -> Result<Rc<RefCell<Scope>>, String> {
    for stmt in tree {
        match stmt {
            VariableDeclarationStatement { name, value } => {
                match evaluate_expression(&scope, value) {
                    Ok(evaluated_expr) => {
                        match scope.borrow_mut().insert_value(&name, &evaluated_expr) {
                            Ok(_) => (),
                            Err(err) => {
                                return Err(
                                    format! {"Error during variable declaration\n{}\n", err},
                                )
                            }
                        }
                    }
                    Err(err) => {
                        return Err(format! {"Error during variable declaration\n{}\n", err})
                    }
                }
            }
            AssignmentStatement { name, value } => match evaluate_expression(&scope, value) {
                Ok(evaluated_expr) => {
                    match scope.borrow_mut().update_value(&name, &evaluated_expr) {
                        Ok(_) => (),
                        Err(err) => {
                            return Err(format! {"Error during variable assignment\n{}\n", err})
                        }
                    }
                }
                Err(err) => return Err(format! {"Error during variable assignment\n{}\n", err}),
            },
            IfStatement { cond, then_part } => {
                let evaluated_expr = evaluate_expression(&scope, cond);
                match evaluated_expr {
                    Ok(Boolean(true)) => {
                        // Create new local scope
                        let mut new_scope = Rc::new(RefCell::new(Scope::default()));
                        // Set parent for local scope
                        new_scope.borrow_mut().set_parent(Rc::clone(&scope));
                        // Update reachable variables
                        new_scope
                            .borrow_mut()
                            .set_reachable_variables(scope.borrow().reachable_variables.clone());
                        // Update reachable functions
                        new_scope
                            .borrow_mut()
                            .set_reachable_functions(scope.borrow().reachable_functions.clone());

                        // Execute then_part
                        match evaluate_ast(then_part, &mut new_scope) {
                            Ok(_) => (),
                            Err(err) => {
                                return Err(format! {"Error during if-else evaluation\n{}\n", err})
                            }
                        }
                    }
                    Ok(Int(_)) => {
                        return Err("Int cannot be used as if condition".red().to_string())
                    }
                    Ok(Float(_)) => {
                        return Err("Float cannot be used as if condition".red().to_string())
                    }
                    Ok(Str(_)) => {
                        return Err("Str cannot be used as if condition".red().to_string())
                    }
                    Err(err) => return Err(format! {"Error during if evaluation\n{}\n", err}),
                    _ => {}
                }
            }
            IfElseStatement {
                cond,
                then_part,
                else_part,
            } => {
                let evaluated_expr = evaluate_expression(&scope, cond);
                match evaluated_expr {
                    Ok(Boolean(true)) => {
                        // Create new local scope
                        let mut new_scope = Rc::new(RefCell::new(Scope::default()));
                        // Set parent for local scope
                        new_scope.borrow_mut().set_parent(Rc::clone(&scope));
                        // Update reachable variables
                        new_scope
                            .borrow_mut()
                            .set_reachable_variables(scope.borrow().reachable_variables.clone());
                        // Update reachable functions
                        new_scope
                            .borrow_mut()
                            .set_reachable_functions(scope.borrow().reachable_functions.clone());

                        // Execute then_part
                        match evaluate_ast(then_part, &mut new_scope) {
                            Ok(_) => (),
                            Err(err) => {
                                return Err(format! {"Error during if-else evaluation\n{}\n", err})
                            }
                        }
                    }
                    Ok(Boolean(false)) => {
                        // Create new local scope
                        let mut new_scope = Rc::new(RefCell::new(Scope::default()));
                        // Set parent for local scope
                        new_scope.borrow_mut().set_parent(Rc::clone(&scope));
                        // Update reachable variables
                        new_scope
                            .borrow_mut()
                            .set_reachable_variables(scope.borrow().reachable_variables.clone());
                        // Update reachable functions
                        new_scope
                            .borrow_mut()
                            .set_reachable_functions(scope.borrow().reachable_functions.clone());

                        // Execute else_part
                        match evaluate_ast(else_part, &mut new_scope) {
                            Ok(_) => (),
                            Err(err) => {
                                return Err(format! {"Error during if-else evaluation\n{}\n", err})
                            }
                        }
                    }
                    Ok(Int(_)) => {
                        return Err("Int cannot be used as if condition".red().to_string())
                    }
                    Ok(Float(_)) => {
                        return Err("Float cannot be used as if condition".red().to_string())
                    }
                    Ok(Str(_)) => {
                        return Err("Str cannot be used as if condition".red().to_string())
                    }
                    Err(err) => return Err(format! {"Error during if-else evaluation\n{}\n", err}),
                }
            }
            WhileStatement { cond, body } => {
                // Create new local scope
                let mut new_scope = Rc::new(RefCell::new(Scope::default()));
                // Set parent for local scope
                new_scope.borrow_mut().set_parent(Rc::clone(&scope));
                // Update reachable variables
                new_scope
                    .borrow_mut()
                    .set_reachable_variables(scope.borrow().reachable_variables.clone());
                // Update reachable functions
                new_scope
                    .borrow_mut()
                    .set_reachable_functions(scope.borrow().reachable_functions.clone());

                loop {
                    let evaluated_expr = evaluate_expression(&scope, cond);
                    match evaluated_expr {
                        Ok(Boolean(true)) => match evaluate_ast(body, &mut new_scope) {
                            Ok(_) => (),
                            Err(err) => {
                                return Err(format! {"Error during while evaluation\n{}\n", err})
                            }
                        },
                        Ok(Boolean(false)) => {
                            break;
                        }
                        Ok(Int(_)) => {
                            return Err("Int cannot be used as if condition".red().to_string())
                        }
                        Ok(Float(_)) => {
                            return Err("Float cannot be used as if condition".red().to_string())
                        }
                        Ok(Str(_)) => {
                            return Err("Str cannot be used as if condition".red().to_string())
                        }
                        Err(err) => {
                            return Err(format! {"Error during while evaluation\n{}\n", err})
                        }
                    }
                }
            }

            FunctionDeclaration {
                name,
                parameters,
                body,
            } => match scope.borrow_mut().insert_function(name, parameters, body) {
                Ok(_) => (),
                Err(err) => return Err(format! {"Error during function declaration\n{}\n", err}),
            },

            ReturnStatement { value } => {
                match evaluate_expression(&scope, value) {
                    Ok(res) => scope.borrow_mut().set_return_value(&res),
                    Err(err) => return Err(format! {"Error during return statement\n{}\n", err}),
                };
                break;
            }

            PrintStatement { content } => match evaluate_expression(&scope, content) {
                Ok(x) => match x {
                    Int(x) => println!("{}", x),
                    Float(x) => println!("{}", x),
                    Str(x) => println!("{}", x),
                    Boolean(x) => println!("{}", x),
                },
                Err(x) => return Err(x),
            },

            InputStatement { name } => {
                let mut input = String::new();
                let mut recognized = false;
                match io::stdin().read_line(&mut input) {
                    Ok(_) => (),
                    Err(x) => return Err(format! {"Error during input statement {}", x}),
                };
                let mut parsed_input = Box::from(Expression::Int(0));
                // Try to parse as i64
                match input.trim().parse::<i64>() {
                    Ok(x) => {
                        parsed_input = Box::from(Expression::Int(x));
                        match scope.borrow().local_variables.get(name) {
                            Some(Int(_)) => recognized = true,
                            Some(Float(_)) => {
                                return Err(format!(
                                    "Error of type incoherence, \"{name}\" is a float"
                                ))
                            }
                            Some(Boolean(_)) => {
                                return Err(format!(
                                    "Error of type incoherence, \"{name}\" is a boolean"
                                ))
                            }
                            Some(Str(_)) => {
                                return Err(format!(
                                    "Error of type incoherence, \"{name}\" is a string"
                                ))
                            }
                            _ => return Err(format!("Input variable {name} does not exist")),
                        };
                    }
                    Err(_) => (),
                };
                // Try to parse as f64
                match input.trim().parse::<f64>() {
                    Ok(x) => {
                        if !recognized {
                            parsed_input = Box::from(Expression::Float(x));
                            match scope.borrow().local_variables.get(name) {
                                Some(Float(_)) => recognized = true,
                                Some(Int(_)) => {
                                    return Err(format!(
                                        "Error of type incoherence, \"{name}\" is a integer"
                                    ))
                                }
                                Some(Boolean(_)) => {
                                    return Err(format!(
                                        "Error of type incoherence, \"{name}\" is a boolean"
                                    ))
                                }
                                Some(Str(_)) => {
                                    return Err(format!(
                                        "Error of type incoherence, \"{name}\" is a string"
                                    ))
                                }
                                _ => return Err(format!("Input variable {name} does not exist")),
                            }
                        }
                    }
                    Err(_) => (),
                }
                // Try to parse as boolean
                match input.trim().parse::<bool>() {
                    Ok(x) => {
                        if !recognized {
                            parsed_input = Box::from(Expression::Bool(x));
                            match scope.borrow().local_variables.get(name) {
                                Some(Boolean(_)) => recognized = true,
                                Some(Int(_)) => {
                                    return Err(format!(
                                        "Error of type incoherence, \"{name}\" is a integer"
                                    ))
                                }
                                Some(Float(_)) => {
                                    return Err(format!(
                                        "Error of type incoherence, \"{name}\" is a float"
                                    ))
                                }
                                Some(Str(_)) => {
                                    return Err(format!(
                                        "Error of type incoherence, \"{name}\" is a string"
                                    ))
                                }
                                _ => return Err(format!("Input variable {name} does not exist")),
                            };
                        }
                    }
                    Err(_) => (),
                };
                // Otherwise parse as string
                match input.trim().parse::<String>() {
                    Ok(x) => {
                        if !recognized {
                            parsed_input = Box::from(Expression::Str(x));
                            match scope.borrow().local_variables.get(name) {
                                Some(Str(_)) => recognized = true,
                                Some(Int(_)) => {
                                    return Err(format!(
                                        "Error of type incoherence, \"{name}\" is a integer"
                                    ))
                                }
                                Some(Float(_)) => {
                                    return Err(format!(
                                        "Error of type incoherence, \"{name}\" is a float"
                                    ))
                                }
                                Some(Boolean(_)) => {
                                    return Err(format!(
                                        "Error of type incoherence, \"{name}\" is a boolean"
                                    ))
                                }
                                _ => return Err(format!("Input variable {name} does not exist")),
                            };
                        }
                    }
                    Err(_) => return Err("Cannot parse given value".to_string()),
                };
                let evaluated_expr = match evaluate_expression(&scope, &parsed_input) {
                    Ok(x) => x,
                    Err(err) => return Err(format! {"Error during input statement {}", err}),
                };
                match scope.borrow_mut().update_value(&name, &evaluated_expr) {
                    Ok(_) => (),
                    Err(err) => {
                        return Err(format! {"Error during variable assignment\n{}\n", err})
                    }
                }
            }
        }
    }
    Ok(scope.to_owned())
}
