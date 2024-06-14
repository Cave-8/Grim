use std::cell::RefCell;
use std::rc::Rc;
use crate::interpreter::interpreter::{Scope, TypeVal};
use crate::interpreter::interpreter::TypeVal::{Boolean, Float, Int, Str};
use crate::parsing::ast::{BinaryOperator, Expression, UnaryOperator};

/// Function used to evaluate expression.
pub fn evaluate_expression(scope: &&mut Rc<RefCell<Scope>>, expr: &Box<Expression>) -> TypeVal {
    match expr.as_ref() {
        Expression::Int(x) => Int(*x),
        Expression::Float(x) => Float(*x),
        Expression::Bool(x) => Boolean(*x),
        Expression::Str(x) => Str(x.clone()),
        Expression::BinaryOperation { lhs, operator, rhs } => {
            bin_op_evaluator(scope, lhs, operator, rhs)
        }
        Expression::UnaryOperation { operator, rhs } => {
            match operator {
                UnaryOperator::Minus => {
                    let right = evaluate_expression(scope, &rhs);
                    match right {
                        Int(x) => Int(-x),
                        Float(x) => Float(-x),
                        Boolean(_) => panic!("Minus boolean is not supported"),
                        Str(_) => panic!("Minus string is not supported"),
                    }
                }
                UnaryOperator::Not => {
                    let right = evaluate_expression(scope, &rhs);
                    match right {
                        Int(_) => panic!("Not int is not supported"),
                        Float(_) => panic!("Not float is not supported"),
                        Boolean(x) => if x { Boolean(false) } else { Boolean(true) }
                        Str(_) => panic!("Not string is not supported"),
                    }
                }
            }
        }
        Expression::Identifier(variable) => {
            let var = scope.borrow().get_variable_value(variable.as_str());
            var
        }
        _ => panic!("Unknown expression"),
    }
}

/// Evaluator of binary operations
pub fn bin_op_evaluator(scope: &&mut Rc<RefCell<Scope>>, lhs: &Box<Expression>, operator: &BinaryOperator, rhs: &Box<Expression>) -> TypeVal {
    match operator {
        BinaryOperator::Add | BinaryOperator::Sub | BinaryOperator::Mul | BinaryOperator::Div => {
            bin_op_arithmetic_evaluator(scope, lhs, operator, rhs)
        }
        _ => bin_op_logic_evaluator(scope, lhs, operator, rhs),
    }
}

/// Evaluate binary arithmetic expressions.
pub fn bin_op_arithmetic_evaluator(scope: &&mut Rc<RefCell<Scope>>, lhs: &Box<Expression>, operator: &BinaryOperator, rhs: &Box<Expression>) -> TypeVal {
    match operator {
        BinaryOperator::Add => {
            let left = evaluate_expression(scope, &lhs);
            let right = evaluate_expression(scope, &rhs);
            match left {
                Int(x) => {
                    match right {
                        Int(y) => Int(x + y),
                        Float(y) => Float(x as f64 + y),
                        Boolean(_) => panic!("Sum between incompatible types"),
                        Str(_) => panic!("Sum between incompatible types"),
                    }
                }
                Float(x) => {
                    match right {
                        Int(y) => Float(x + y as f64),
                        Float(y) => Float(x + y),
                        Boolean(_) => panic!("Sum between incompatible types"),
                        Str(_) => panic!("Sum between incompatible types"),
                    }
                }
                Boolean(_) => {
                    match right {
                        Int(_) => panic!("Sum between incompatible types"),
                        Float(_) => panic!("Sum between incompatible types"),
                        Boolean(_) => panic!("Sum between boolean types"),
                        Str(_) => panic!("Sum between incompatible types"),
                    }
                }
                Str(_) => {
                    match right {
                        Int(_) => panic!("Sum between incompatible types"),
                        Float(_) => panic!("Sum between incompatible types"),
                        Boolean(_) => panic!("Sum between boolean types"),
                        Str(_) => panic!("Sum between incompatible types"),
                    }
                }
            }
        }
        BinaryOperator::Sub => {
            let left = evaluate_expression(scope, &lhs);
            let right = evaluate_expression(scope, &rhs);
            match left {
                Int(x) => {
                    match right {
                        Int(y) => Int(x - y),
                        Float(y) => Float(x as f64 - y),
                        Boolean(_) => panic!("Difference between incompatible types"),
                        Str(_) => panic!("Difference between incompatible types"),
                    }
                }
                Float(x) => {
                    match right {
                        Int(y) => Float(x - y as f64),
                        Float(y) => Float(x - y),
                        Boolean(_) => panic!("Difference between incompatible types"),
                        Str(_) => panic!("Difference between incompatible types"),
                    }
                }
                Boolean(_) => {
                    match right {
                        Int(_) => panic!("Difference between incompatible types"),
                        Float(_) => panic!("Difference between incompatible types"),
                        Boolean(_) => panic!("Difference between boolean types"),
                        Str(_) => panic!("Difference between incompatible types"),
                    }
                }
                Str(_) => {
                    match right {
                        Int(_) => panic!("Difference between incompatible types"),
                        Float(_) => panic!("Difference between incompatible types"),
                        Boolean(_) => panic!("Difference between boolean types"),
                        Str(_) => panic!("Difference between incompatible types"),
                    }
                }
            }
        }
        BinaryOperator::Mul => {
            let left = evaluate_expression(scope, &lhs);
            let right = evaluate_expression(scope, &rhs);
            match left {
                Int(x) => {
                    match right {
                        Int(y) => Int(x * y),
                        Float(y) => Float(x as f64 * y),
                        Boolean(_y) => panic!("Product between incompatible types"),
                        Str(_) => panic!("Product between incompatible types"),
                    }
                }
                Float(x) => {
                    match right {
                        Int(y) => Float(x * y as f64),
                        Float(y) => Float(x * y),
                        Boolean(_) => panic!("Product between incompatible types"),
                        Str(_) => panic!("Product between incompatible types"),
                    }
                }
                Boolean(_) => {
                    match right {
                        Int(_) => panic!("Product between incompatible types"),
                        Float(_) => panic!("Product between incompatible types"),
                        Boolean(_) => panic!("Product between boolean types"),
                        Str(_) => panic!("Product between incompatible types"),
                    }
                }
                Str(_) => {
                    match right {
                        Int(_) => panic!("Product between incompatible types"),
                        Float(_) => panic!("Product between incompatible types"),
                        Boolean(_) => panic!("Product between boolean types"),
                        Str(_) => panic!("Product between incompatible types"),
                    }
                }
            }
        }
        BinaryOperator::Div => {
            let left = evaluate_expression(scope, &lhs);
            let right = evaluate_expression(scope, &rhs);
            match left {
                Int(x) => {
                    match right {
                        Int(y) => if x % y == 0 { Int(x / y) } else { Float((x as f64) / (y as f64)) },
                        Float(y) => Float(x as f64 / y),
                        Boolean(_) => panic!("Division between incompatible types"),
                        Str(_) => panic!("Division between incompatible types"),
                    }
                }
                Float(x) => {
                    match right {
                        Int(y) => Float(x / y as f64),
                        Float(y) => Float(x / y),
                        Boolean(_) => panic!("Division between incompatible types"),
                        Str(_) => panic!("Division between incompatible types"),
                    }
                }
                Boolean(_) => {
                    match right {
                        Int(_) => panic!("Division between incompatible types"),
                        Float(_) => panic!("Division between incompatible types"),
                        Boolean(_) => panic!("Division between boolean types"),
                        Str(_) => panic!("Division between incompatible types"),
                    }
                }
                Str(_) => {
                    match right {
                        Int(_) => panic!("Division between incompatible types"),
                        Float(_) => panic!("Division between incompatible types"),
                        Boolean(_) => panic!("Division between boolean types"),
                        Str(_) => panic!("Division between incompatible types"),
                    }
                }
            }
        }
        _ => panic!("Unrecognized arithmetic binary expression"),
    }
}

/// Evaluate binary logic expressions.
pub fn bin_op_logic_evaluator(scope: &&mut Rc<RefCell<Scope>>, lhs: &Box<Expression>, operator: &BinaryOperator, rhs: &Box<Expression>) -> TypeVal {
    match operator {
        BinaryOperator::And => {
            let left = evaluate_expression(scope, &lhs);
            let right = evaluate_expression(scope, &rhs);
            match left {
                Int(_) => {
                    match right {
                        Int(_) => panic!("Logical AND between incompatible types"),
                        Float(_) => panic!("Logical AND between incompatible types"),
                        Boolean(_) => panic!("Logical AND between incompatible types"),
                        Str(_) => panic!("Logical AND between incompatible types"),
                    }
                }
                Float(_) => {
                    match right {
                        Int(_) => panic!("Logical AND between incompatible types"),
                        Float(_) => panic!("Logical AND between incompatible types"),
                        Boolean(_) => panic!("Logical AND between incompatible types"),
                        Str(_) => panic!("Logical AND between incompatible types"),
                    }
                }
                Boolean(x) => {
                    match right {
                        Int(_) => panic!("Logical AND between incompatible types"),
                        Float(_) => panic!("Logical AND between incompatible types"),
                        Boolean(y) => Boolean(x && y),
                        Str(_) => panic!("Logical AND between incompatible types"),
                    }
                }
                Str(_) => {
                    match right {
                        Int(_) => panic!("Logical AND between incompatible types"),
                        Float(_) => panic!("Logical AND between incompatible types"),
                        Boolean(_) => panic!("Logical AND between boolean types"),
                        Str(_) => panic!("Logical AND between incompatible types"),
                    }
                }
            }
        }
        BinaryOperator::Or => {
            let left = evaluate_expression(scope, &lhs);
            let right = evaluate_expression(scope, &rhs);
            match left {
                Int(_) => {
                    match right {
                        Int(_) => panic!("Logical OR of incompatible types"),
                        Float(_) => panic!("Logical OR of incompatible types"),
                        Boolean(_) => panic!("Logical OR of incompatible types"),
                        Str(_) => panic!("Logical OR between incompatible types"),
                    }
                }
                Float(_) => {
                    match right {
                        Int(_) => panic!("Logical OR of incompatible types"),
                        Float(_) => panic!("Logical OR of incompatible types"),
                        Boolean(_) => panic!("Logical OR of incompatible types"),
                        Str(_) => panic!("Logical OR between incompatible types"),
                    }
                }
                Boolean(x) => {
                    match right {
                        Int(_) => panic!("Logical OR of incompatible types"),
                        Float(_) => panic!("Logical OR of incompatible types"),
                        Boolean(y) => Boolean(x || y),
                        Str(_) => panic!("Logical OR between incompatible types"),
                    }
                }
                Str(_) => {
                    match right {
                        Int(_) => panic!("Logical OR between incompatible types"),
                        Float(_) => panic!("Logical OR between incompatible types"),
                        Boolean(_) => panic!("Logical OR between boolean types"),
                        Str(_) => panic!("Logical OR between incompatible types"),
                    }
                }
            }
        }
        BinaryOperator::Less => {
            let left = evaluate_expression(scope, &lhs);
            let right = evaluate_expression(scope, &rhs);
            match left {
                Int(x) => {
                    match right {
                        Int(y) => Boolean(x < y),
                        Float(y) => Boolean((x as f64) < y),
                        Boolean(_) => panic!("Logical LE between incompatible types"),
                        Str(_) => panic!("Logical LE between incompatible types"),
                    }
                }
                Float(x) => {
                    match right {
                        Int(y) => Boolean(x < (y as f64)),
                        Float(y) => Boolean(x < y),
                        Boolean(_) => panic!("Logical LE between incompatible types"),
                        Str(_) => panic!("Logical LE between incompatible types"),
                    }
                }
                Boolean(_) => {
                    match right {
                        Int(_) => panic!("Logical LE between incompatible types"),
                        Float(_) => panic!("Logical LE between incompatible types"),
                        Boolean(_) => panic!("Logical LE between incompatible types"),
                        Str(_) => panic!("Logical LE between incompatible types"),
                    }
                }
                Str(_) => {
                    match right {
                        Int(_) => panic!("Logical LE between incompatible types"),
                        Float(_) => panic!("Logical LE between incompatible types"),
                        Boolean(_) => panic!("Logical LE between incompatible types"),
                        Str(_) => panic!("Logical LE between incompatible types"),
                    }
                }
            }
        }
        BinaryOperator::Greater => {
            let left = evaluate_expression(scope, &lhs);
            let right = evaluate_expression(scope, &rhs);
            match left {
                Int(x) => {
                    match right {
                        Int(y) => Boolean(x > y),
                        Float(y) => Boolean((x as f64) > y),
                        Boolean(_) => panic!("Logical GR between incompatible types"),
                        Str(_) => panic!("Logical GR between incompatible types"),
                    }
                }
                Float(x) => {
                    match right {
                        Int(y) => Boolean(x > (y as f64)),
                        Float(y) => Boolean(x > y),
                        Boolean(_) => panic!("Logical GR between incompatible types"),
                        Str(_) => panic!("Logical GR between incompatible types"),
                    }
                }
                Boolean(_) => {
                    match right {
                        Int(_) => panic!("Logical GR between incompatible types"),
                        Float(_) => panic!("Logical GR between incompatible types"),
                        Boolean(_) => panic!("Logical GR between incompatible types"),
                        Str(_) => panic!("Logical GR between incompatible types"),
                    }
                }
                Str(_) => {
                    match right {
                        Int(_) => panic!("Logical GR between incompatible types"),
                        Float(_) => panic!("Logical GR between incompatible types"),
                        Boolean(_) => panic!("Logical GR between incompatible types"),
                        Str(_) => panic!("Logical GR between incompatible types"),
                    }
                }
            }
        }
        BinaryOperator::LessEq => {
            let left = evaluate_expression(scope, &lhs);
            let right = evaluate_expression(scope, &rhs);
            match left {
                Int(x) => {
                    match right {
                        Int(y) => Boolean(x <= y),
                        Float(y) => Boolean(x as f64 <= y),
                        Boolean(_) => panic!("Logical LEQ between incompatible types"),
                        Str(_) => panic!("Logical LEQ between incompatible types"),
                    }
                }
                Float(x) => {
                    match right {
                        Int(y) => Boolean(x <= y as f64),
                        Float(y) => Boolean(x <= y),
                        Boolean(_) => panic!("Logical LEQ between incompatible types"),
                        Str(_) => panic!("Logical LEQ between incompatible types"),
                    }
                }
                Boolean(_) => {
                    match right {
                        Int(_) => panic!("Logical LEQ between incompatible types"),
                        Float(_) => panic!("Logical LEQ between incompatible types"),
                        Boolean(_) => panic!("Logical LEQ between incompatible types"),
                        Str(_) => panic!("Logical LEQ between incompatible types"),
                    }
                }
                Str(_) => {
                    match right {
                        Int(_) => panic!("Logical LEQ between incompatible types"),
                        Float(_) => panic!("Logical LEQ between incompatible types"),
                        Boolean(_) => panic!("Logical LEQ between incompatible types"),
                        Str(_) => panic!("Logical LEQ between incompatible types"),
                    }
                }
            }
        }
        BinaryOperator::GreaterEq => {
            let left = evaluate_expression(scope, &lhs);
            let right = evaluate_expression(scope, &rhs);
            match left {
                Int(x) => {
                    match right {
                        Int(y) => Boolean(x >= y),
                        Float(y) => Boolean(x as f64 >= y),
                        Boolean(_y) => panic!("Logical GEQ between incompatible types"),
                        Str(_) => panic!("Logical GEQ between incompatible types"),
                    }
                }
                Float(x) => {
                    match right {
                        Int(y) => Boolean(x >= y as f64),
                        Float(y) => Boolean(x >= y),
                        Boolean(_) => panic!("Logical GEQ between incompatible types"),
                        Str(_) => panic!("Logical GEQ between incompatible types"),
                    }
                }
                Boolean(_) => {
                    match right {
                        Int(_) => panic!("Logical GEQ between incompatible types"),
                        Float(_) => panic!("Logical GEQ between incompatible types"),
                        Boolean(_) => panic!("Logical GEQ between incompatible types"),
                        Str(_) => panic!("Logical GEQ between incompatible types"),
                    }
                }
                Str(_) => {
                    match right {
                        Int(_) => panic!("Logical GEQ between incompatible types"),
                        Float(_) => panic!("Logical GEQ between incompatible types"),
                        Boolean(_) => panic!("Logical GEQ between incompatible types"),
                        Str(_) => panic!("Logical GEQ between incompatible types"),
                    }
                }
            }
        }
        BinaryOperator::Eq => {
            let left = evaluate_expression(scope, &lhs);
            let right = evaluate_expression(scope, &rhs);
            match left {
                Int(x) => {
                    match right {
                        Int(y) => Boolean(x == y),
                        Float(y) => Boolean(x as f64 == y),
                        Boolean(_) => panic!("Logical EQ between incompatible types"),
                        Str(_) => panic!("Logical EQ between incompatible types"),
                    }
                }
                Float(x) => {
                    match right {
                        Int(y) => Boolean(x == y as f64),
                        Float(y) => Boolean(x == y),
                        Boolean(_) => panic!("Logical EQ between incompatible types"),
                        Str(_) => panic!("Logical EQ between incompatible types"),
                    }
                }
                Boolean(x) => {
                    match right {
                        Int(_) => panic!("Logical EQ between incompatible types"),
                        Float(_) => panic!("Logical EQ between incompatible types"),
                        Boolean(y) => Boolean(x == y),
                        Str(_) => panic!("Logical EQ between incompatible types"),
                    }
                }
                Str(x) => {
                    match right {
                        Int(_) => panic!("Logical EQ between incompatible types"),
                        Float(_) => panic!("Logical EQ between incompatible types"),
                        Boolean(_) => panic!("Logical EQ between incompatible types"),
                        Str(y) => Boolean(x == y),
                    }
                }
            }
        }
        BinaryOperator::NotEq => {
            let left = evaluate_expression(scope, &lhs);
            let right = evaluate_expression(scope, &rhs);
            match left {
                Int(x) => {
                    match right {
                        Int(y) => Boolean(x != y),
                        Float(y) => Boolean(x as f64 != y),
                        Boolean(_) => panic!("Logical NEQ between incompatible types"),
                        Str(_) => panic!("Logical NEQ between incompatible types"),
                    }
                }
                Float(x) => {
                    match right {
                        Int(y) => Boolean(x != y as f64),
                        Float(y) => Boolean(x != y),
                        Boolean(_) => panic!("Logical NEQ between incompatible types"),
                        Str(_) => panic!("Logical NEQ between incompatible types"),
                    }
                }
                Boolean(x) => {
                    match right {
                        Int(_) => panic!("Logical NEQ between incompatible types"),
                        Float(_) => panic!("Logical NEQ between incompatible types"),
                        Boolean(y) => Boolean(x != y),
                        Str(_) => panic!("Logical NEQ between incompatible types"),
                    }
                }
                Str(x) => {
                    match right {
                        Int(_) => panic!("Logical NEQ between incompatible types"),
                        Float(_) => panic!("Logical NEQ between incompatible types"),
                        Boolean(_) => panic!("Logical NEQ between incompatible types"),
                        Str(y) => Boolean(x != y),
                    }
                }
            }
        }
        _ => panic!("Unrecognized logic binary expression"),
    }
}
