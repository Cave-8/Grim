use std::cmp::PartialEq;
use std::collections::HashMap;
use crate::interpreter::interpreter::TypeVal::{Boolean, Float, Int};
use crate::parsing::ast::{BinaryOperator, Expression, Statement, UnaryOperator};
use crate::parsing::ast::Statement::{AssignmentStatement, IfStatement, VariableDeclarationStatement};

#[derive(Debug, Clone, PartialEq)]
pub enum TypeVal {
    Int(i64),
    Float(f64),
    Boolean(bool),
}

#[derive(Debug, Default, Clone)]
pub struct Frame {
    parent: Option<Box<Frame>>,
    local_variables: HashMap<String, TypeVal>,
}

impl Frame {
    /// Insert value for the first time in the frame
    pub fn insert_value(&mut self, variable_name: &str, value: &TypeVal) {
        if let Some(&ref value) = self.local_variables.get(variable_name) {
            panic!("{} already defined", variable_name);
        } else {
            match value {
                Int(x) => {
                    self.local_variables.insert(variable_name.to_string(), Int(x.clone()));
                }
                Float(x) => {
                    self.local_variables.insert(variable_name.to_string(), Float(x.clone()));
                }
                Boolean(x) => {
                    self.local_variables.insert(variable_name.to_string(), Boolean(x.clone()));
                }
            }
        }
    }

    /// Get value of a variable
    pub fn variable_value(&self, variable_name: &str) -> TypeVal {
        if let Some(&ref value) = self.local_variables.get(variable_name) {
            value.clone()
        } else if let Some(parent) = self.parent.as_ref() {
            parent.variable_value(variable_name)
        } else {
            panic!("{} does not exist", variable_name);
        }
    }

    /// Update value of a variable in the frame
    pub fn update_value(&mut self, variable_name: &str, value: &TypeVal) {
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
            }
        } else if let Some(parent) = self.parent.as_mut() {
            parent.update_value(variable_name, &value);
        } else {
            panic!("{} does not exist", variable_name);
        }
    }
}


/// AST evaluation
pub fn evaluate_ast(tree: &Vec<Statement>, frame: &mut Box<Frame>) -> Box<Frame> {
    for stmt in tree {
        match stmt {
            VariableDeclarationStatement { name, value } => {
                let evaluated_expr = evaluate_expression(&frame, value);
                frame.insert_value(&name, &evaluated_expr);
            }
            AssignmentStatement { name, value } => {
                let evaluated_expr = evaluate_expression(&frame, value);
                frame.update_value(&name, &evaluated_expr);
            }
            IfStatement { cond, then_part } => {
                let evaluated_expr = evaluate_expression(&frame, cond);
                match evaluated_expr {
                    Boolean(true) => {
                        let mut local_frame = Box::new(Frame::default());
                        local_frame.parent = Some(frame.to_owned());
                        evaluate_ast(then_part, &mut local_frame);
                        println!("{:#?}", local_frame);
                        // todo() need to update parents
                    }
                    Int(_) => panic!("Int cannot be used as if condition"),
                    Float(_) => panic!("Float cannot be used as if condition"),
                    _ => ()
                }
            }
            _ => { println!("{:#?}", stmt) }
        }
    }

    return frame.to_owned();
}

fn evaluate_expression(frame: &Frame, expr: &Box<Expression>) -> TypeVal {
    match expr.as_ref() {
        Expression::Int(x) => Int(*x),
        Expression::Float(x) => Float(*x),
        Expression::Bool(x) => Boolean(*x),
        Expression::BinaryOperation { lhs, operator, rhs } => {
            match operator {
                BinaryOperator::Add => {
                    let left = evaluate_expression(frame, &lhs);
                    let right = evaluate_expression(frame, &rhs);
                    match left {
                        Int(x) => {
                            match right {
                                Int(y) => Int(x + y),
                                Float(y) => Float(x as f64 + y),
                                Boolean(_) => panic!("Sum between incompatible types"),
                            }
                        }
                        Float(x) => {
                            match right {
                                Int(y) => Float(x + y as f64),
                                Float(y) => Float(x + y),
                                Boolean(_) => panic!("Sum between incompatible types"),
                            }
                        }
                        Boolean(_) => {
                            match right {
                                Int(_) => panic!("Sum between incompatible types"),
                                Float(_) => panic!("Sum between incompatible types"),
                                Boolean(_) => panic!("Sum between boolean types"),
                            }
                        }
                    }
                }
                BinaryOperator::Sub => {
                    let left = evaluate_expression(frame, &lhs);
                    let right = evaluate_expression(frame, &rhs);
                    match left {
                        Int(x) => {
                            match right {
                                Int(y) => Int(x - y),
                                Float(y) => Float(x as f64 - y),
                                Boolean(_) => panic!("Difference between incompatible types"),
                            }
                        }
                        Float(x) => {
                            match right {
                                Int(y) => Float(x - y as f64),
                                Float(y) => Float(x - y),
                                Boolean(_) => panic!("Difference between incompatible types"),
                            }
                        }
                        Boolean(_) => {
                            match right {
                                Int(_) => panic!("Difference between incompatible types"),
                                Float(_) => panic!("Difference between incompatible types"),
                                Boolean(_) => panic!("Difference between boolean types"),
                            }
                        }
                    }
                }
                BinaryOperator::Mul => {
                    let left = evaluate_expression(frame, &lhs);
                    let right = evaluate_expression(frame, &rhs);
                    match left {
                        Int(x) => {
                            match right {
                                Int(y) => Int(x * y),
                                Float(y) => Float(x as f64 * y),
                                Boolean(_y) => panic!("Product between incompatible types"),
                            }
                        }
                        Float(x) => {
                            match right {
                                Int(y) => Float(x * y as f64),
                                Float(y) => Float(x * y),
                                Boolean(_) => panic!("Product between incompatible types"),
                            }
                        }
                        Boolean(_) => {
                            match right {
                                Int(_) => panic!("Product between incompatible types"),
                                Float(_) => panic!("Product between incompatible types"),
                                Boolean(_) => panic!("Product between boolean types"),
                            }
                        }
                    }
                }
                BinaryOperator::Div => {
                    let left = evaluate_expression(frame, &lhs);
                    let right = evaluate_expression(frame, &rhs);
                    match left {
                        Int(x) => {
                            match right {
                                Int(y) => if x % y == 0 { Int(x / y) } else { Float((x as f64) / (y as f64)) },
                                Float(y) => Float(x as f64 / y),
                                Boolean(_) => panic!("Division between incompatible types"),
                            }
                        }
                        Float(x) => {
                            match right {
                                Int(y) => Float(x / y as f64),
                                Float(y) => Float(x / y),
                                Boolean(_) => panic!("Division between incompatible types"),
                            }
                        }
                        Boolean(_) => {
                            match right {
                                Int(_) => panic!("Division between incompatible types"),
                                Float(_) => panic!("Division between incompatible types"),
                                Boolean(_) => panic!("Division between boolean types"),
                            }
                        }
                    }
                }
                BinaryOperator::And => {
                    let left = evaluate_expression(frame, &lhs);
                    let right = evaluate_expression(frame, &rhs);
                    match left {
                        Int(_) => {
                            match right {
                                Int(_) => panic!("Logical AND between incompatible types"),
                                Float(_) => panic!("Logical AND between incompatible types"),
                                Boolean(_) => panic!("Logical AND between incompatible types"),
                            }
                        }
                        Float(_) => {
                            match right {
                                Int(_) => panic!("Logical AND between incompatible types"),
                                Float(_) => panic!("Logical AND between incompatible types"),
                                Boolean(_) => panic!("Logical AND between incompatible types"),
                            }
                        }
                        Boolean(x) => {
                            match right {
                                Int(_) => panic!("Logical AND between incompatible types"),
                                Float(_) => panic!("Logical AND between incompatible types"),
                                Boolean(y) => Boolean(x && y)
                            }
                        }
                    }
                }
                BinaryOperator::Or => {
                    let left = evaluate_expression(frame, &lhs);
                    let right = evaluate_expression(frame, &rhs);
                    match left {
                        Int(_) => {
                            match right {
                                Int(_) => panic!("Logical OR of incompatible types"),
                                Float(_) => panic!("Logical OR of incompatible types"),
                                Boolean(_) => panic!("Logical OR of incompatible types"),
                            }
                        }
                        Float(_) => {
                            match right {
                                Int(_) => panic!("Logical OR of incompatible types"),
                                Float(_) => panic!("Logical OR of incompatible types"),
                                Boolean(_) => panic!("Logical OR of incompatible types"),
                            }
                        }
                        Boolean(x) => {
                            match right {
                                Int(_) => panic!("Logical OR of incompatible types"),
                                Float(_) => panic!("Logical OR of incompatible types"),
                                Boolean(y) => Boolean(x || y)
                            }
                        }
                    }
                }
                BinaryOperator::Less => {
                    let left = evaluate_expression(frame, &lhs);
                    let right = evaluate_expression(frame, &rhs);
                    match left {
                        Int(x) => {
                            match right {
                                Int(y) => Boolean(x < y),
                                Float(y) => Boolean((x as f64) < y),
                                Boolean(_) => panic!("Logical LE between incompatible types"),
                            }
                        }
                        Float(x) => {
                            match right {
                                Int(y) => Boolean(x < (y as f64)),
                                Float(y) => Boolean(x < y),
                                Boolean(_) => panic!("Logical LE between incompatible types"),
                            }
                        }
                        Boolean(_) => {
                            match right {
                                Int(_) => panic!("Logical LE between incompatible types"),
                                Float(_) => panic!("Logical LE between incompatible types"),
                                Boolean(_) => panic!("Logical LE between incompatible types"),
                            }
                        }
                    }
                }
                BinaryOperator::Greater => {
                    let left = evaluate_expression(frame, &lhs);
                    let right = evaluate_expression(frame, &rhs);
                    match left {
                        Int(x) => {
                            match right {
                                Int(y) => Boolean(x > y),
                                Float(y) => Boolean((x as f64) > y),
                                Boolean(_) => panic!("Logical GR between incompatible types"),
                            }
                        }
                        Float(x) => {
                            match right {
                                Int(y) => Boolean(x > (y as f64)),
                                Float(y) => Boolean(x > y),
                                Boolean(_) => panic!("Logical GR between incompatible types"),
                            }
                        }
                        Boolean(_) => {
                            match right {
                                Int(_) => panic!("Logical GR between incompatible types"),
                                Float(_) => panic!("Logical GR between incompatible types"),
                                Boolean(_) => panic!("Logical GR between incompatible types"),
                            }
                        }
                    }
                }
                BinaryOperator::LessEq => {
                    let left = evaluate_expression(frame, &lhs);
                    let right = evaluate_expression(frame, &rhs);
                    match left {
                        Int(x) => {
                            match right {
                                Int(y) => Boolean(x <= y),
                                Float(y) => Boolean(x as f64 <= y),
                                Boolean(_) => panic!("Logical LEQ between incompatible types"),
                            }
                        }
                        Float(x) => {
                            match right {
                                Int(y) => Boolean(x <= y as f64),
                                Float(y) => Boolean(x <= y),
                                Boolean(_) => panic!("Logical LEQ between incompatible types"),
                            }
                        }
                        Boolean(_) => {
                            match right {
                                Int(_) => panic!("Logical LEQ between incompatible types"),
                                Float(_) => panic!("Logical LEQ between incompatible types"),
                                Boolean(_) => panic!("Logical LEQ between incompatible types"),
                            }
                        }
                    }
                }
                BinaryOperator::GreaterEq => {
                    let left = evaluate_expression(frame, &lhs);
                    let right = evaluate_expression(frame, &rhs);
                    match left {
                        Int(x) => {
                            match right {
                                Int(y) => Boolean(x >= y),
                                Float(y) => Boolean(x as f64 >= y),
                                Boolean(_y) => panic!("Logical GEQ between incompatible types"),
                            }
                        }
                        Float(x) => {
                            match right {
                                Int(y) => Boolean(x >= y as f64),
                                Float(y) => Boolean(x >= y),
                                Boolean(_) => panic!("Logical GEQ between incompatible types"),
                            }
                        }
                        Boolean(_) => {
                            match right {
                                Int(_) => panic!("Logical GEQ between incompatible types"),
                                Float(_) => panic!("Logical GEQ between incompatible types"),
                                Boolean(_) => panic!("Logical GEQ between incompatible types"),
                            }
                        }
                    }
                }
                BinaryOperator::Eq => {
                    let left = evaluate_expression(frame, &lhs);
                    let right = evaluate_expression(frame, &rhs);
                    match left {
                        Int(x) => {
                            match right {
                                Int(y) => Boolean(x == y),
                                Float(y) => Boolean(x as f64 == y),
                                Boolean(_) => panic!("Logical EQ between incompatible types"),
                            }
                        }
                        Float(x) => {
                            match right {
                                Int(y) => Boolean(x == y as f64),
                                Float(y) => Boolean(x == y),
                                Boolean(_) => panic!("Logical EQ between incompatible types"),
                            }
                        }
                        Boolean(x) => {
                            match right {
                                Int(_) => panic!("Logical EQ between incompatible types"),
                                Float(_) => panic!("Logical EQ between incompatible types"),
                                Boolean(y) => Boolean(x == y),
                            }
                        }
                    }
                }
                BinaryOperator::NotEq => {
                    let left = evaluate_expression(frame, &lhs);
                    let right = evaluate_expression(frame, &rhs);
                    match left {
                        Int(x) => {
                            match right {
                                Int(y) => Boolean(x != y),
                                Float(y) => Boolean(x as f64 != y),
                                Boolean(_) => panic!("Logical NEQ between incompatible types"),
                            }
                        }
                        Float(x) => {
                            match right {
                                Int(y) => Boolean(x != y as f64),
                                Float(y) => Boolean(x != y),
                                Boolean(_) => panic!("Logical NEQ between incompatible types"),
                            }
                        }
                        Boolean(x) => {
                            match right {
                                Int(_) => panic!("Logical NEQ between incompatible types"),
                                Float(_) => panic!("Logical NEQ between incompatible types"),
                                Boolean(y) => Boolean(x != y)
                            }
                        }
                    }
                }
            }
        }
        Expression::UnaryOperation { operator, rhs } => {
            match operator {
                UnaryOperator::Minus => {
                    let right = evaluate_expression(frame, &rhs);
                    match right {
                        Int(x) => Int(-x),
                        Float(x) => Float(-x),
                        Boolean(_) => panic!("Minus boolean is not supported"),
                    }
                }
                UnaryOperator::Not => {
                    let right = evaluate_expression(frame, &rhs);
                    match right {
                        Int(_) => panic!("Not int is not supported"),
                        Float(_) => panic!("Not float is not supported"),
                        Boolean(x) => if x { Boolean(false) } else { Boolean(true) }
                    }
                }
            }
        }
        Expression::String(variable) => {
            let var = frame.variable_value(variable.as_str());
            var
        }
        _ => panic!("Unknown expression"),
    }
}
