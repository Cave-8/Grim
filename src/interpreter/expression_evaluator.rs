use std::cell::RefCell;
use std::rc::Rc;
use crate::interpreter::interpreter::{evaluate_ast, Scope, TypeVal};
use crate::interpreter::interpreter::TypeVal::{Boolean, Float, Int, Str};
use crate::parsing::ast::{BinaryOperator, Expression, UnaryOperator};
use crate::interpreter::error_reporting::{error_reporting_binary_operator, error_reporting_generic, error_reporting_unary_operator};

/// Function used to evaluate expression.
pub fn evaluate_expression(scope: &&mut Rc<RefCell<Scope>>, expr: &Box<Expression>) -> Result<TypeVal, String> {
    match expr.as_ref() {
        Expression::Int(x) => Ok(Int(*x)),
        Expression::Float(x) => Ok(Float(*x)),
        Expression::Bool(x) => Ok(Boolean(*x)),
        Expression::Str(x) => Ok(Str(x.clone())),
        Expression::BinaryOperation { lhs, operator, rhs } => {
            bin_op_evaluator(scope, lhs, operator, rhs)
        }
        Expression::UnaryOperation { operator, rhs } => {
            match operator {
                UnaryOperator::Minus => {
                    let right = evaluate_expression(scope, &rhs);
                    match right {
                        Ok(Int(x)) => Ok(Int(-x)),
                        Ok(Float(x)) => Ok(Float(-x)),
                        Ok(Boolean(x)) => error_reporting_unary_operator("Minus boolean is not supported".to_string(), &Boolean(x)),
                        Ok(Str(x)) => error_reporting_unary_operator("Minus boolean is not supported".to_string(), &Str(x)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                UnaryOperator::Not => {
                    let right = evaluate_expression(scope, &rhs);
                    match right {
                        Ok(Int(x)) => error_reporting_unary_operator("Not int is not supported".to_string(), &Int(x)),
                        Ok(Float(x)) => error_reporting_unary_operator("Not float is not supported".to_string(), &Float(x)),
                        Ok(Boolean(x)) => if x { Ok(Boolean(false)) } else { Ok(Boolean(true)) }
                        Ok(Str(x)) => error_reporting_unary_operator("Not string is not supported".to_string(), &Str(x)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
            }
        }
        Expression::Identifier(variable) => {
            let var = scope.borrow().get_variable_value(variable.as_str());
            Ok(var)
        }
        Expression::FunctionCall { name, arguments } => {
            let (fun_args, fun_body) = scope.borrow().get_function_info(name);
            let mut fun_scope = Rc::new(RefCell::new(Scope::default()));
            let mut errors = false;

            // Bind each argument with its value
            fun_args.iter()
                .zip(arguments.iter())
                .for_each(|(x, y)| {
                    match evaluate_expression(scope, y) {
                        Ok(eval_exp) => { fun_scope.borrow_mut().local_variables.insert(x.clone(), eval_exp); }
                        Err(_) => errors = true,
                    }
                });

            // Return after errors
            if errors {
                return Err("Error during function call\n".to_string());
            }

            // Evaluate function scope
            let evaluated_function = evaluate_ast(&fun_body, &mut fun_scope);
            // Get result
            let res = evaluated_function.unwrap();
            let borrow_scope = res.borrow();
            let result = borrow_scope.local_variables.get("return");
            Ok(result.unwrap().clone())
        }
        _ => error_reporting_generic("Unrecognized expression".to_string()),
    }
}

/// Evaluator of binary operations
pub fn bin_op_evaluator(scope: &&mut Rc<RefCell<Scope>>, lhs: &Box<Expression>, operator: &BinaryOperator, rhs: &Box<Expression>) -> Result<TypeVal, String> {
    match operator {
        BinaryOperator::Add | BinaryOperator::Sub | BinaryOperator::Mul | BinaryOperator::Div | BinaryOperator::Mod => {
            match bin_op_arithmetic_evaluator(scope, lhs, operator, rhs) {
                Ok(result) => Ok(result),
                Err(err) => Err(format! {"Error during binary arithmetic expression evaluation\n{}\n", err})
            }
        }
        _ => {
            match bin_op_logic_evaluator(scope, lhs, operator, rhs) {
                Ok(result) => Ok(result),
                Err(err) => Err(format! {"Error during binary logic expression evaluation\n{}", err})
            }
        }
    }
}

/// Evaluate binary arithmetic expressions.
pub fn bin_op_arithmetic_evaluator(scope: &&mut Rc<RefCell<Scope>>, lhs: &Box<Expression>, operator: &BinaryOperator, rhs: &Box<Expression>) -> Result<TypeVal, String> {
    match operator {
        BinaryOperator::Add => {
            let left = evaluate_expression(scope, &lhs);
            let right = evaluate_expression(scope, &rhs);
            match left {
                Ok(Int(x)) => {
                    match right {
                        Ok(Int(y)) => Ok(Int(x + y)),
                        Ok(Float(y)) => Ok(Float(x as f64 + y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Sum between incompatible types".to_string(), &Int(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Sum between incompatible types".to_string(), &Int(x), &Str(y)),
                        Err(err) => Err(format! {"Error during binary arithmetic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Float(x)) => {
                    match right {
                        Ok(Int(y)) => Ok(Float(x + y as f64)),
                        Ok(Float(y)) => Ok(Float(x + y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Sum between incompatible types".to_string(), &Float(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Sum between incompatible types".to_string(), &Float(x), &Str(y)),
                        Err(err) => Err(format! {"Error during binary arithmetic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Boolean(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Sum between incompatible types".to_string(), &Boolean(x), &Int(y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Sum between incompatible types".to_string(), &Boolean(x), &Float(y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Sum between incompatible types".to_string(), &Boolean(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Sum between incompatible types".to_string(), &Boolean(x), &Str(y)),
                        Err(err) => Err(format! {"Error during binary arithmetic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Str(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Sum between incompatible types".to_string(), &Str(x), &Int(y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Sum between incompatible types".to_string(), &Str(x), &Float(y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Sum between incompatible types".to_string(), &Str(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Sum between incompatible types".to_string(), &Str(x), &Str(y)),
                        Err(err) => Err(format! {"Error during binary arithmetic expression evaluation\n{}\n", err})
                    }
                }
                Err(err) => Err(format! {"Error during arithmetic expression evaluation\n{}\n", err})
            }
        }
        BinaryOperator::Sub => {
            let left = evaluate_expression(scope, &lhs);
            let right = evaluate_expression(scope, &rhs);
            match left {
                Ok(Int(x)) => {
                    match right {
                        Ok(Int(y)) => Ok(Int(x - y)),
                        Ok(Float(y)) => Ok(Float(x as f64 - y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Difference between incompatible types".to_string(), &Int(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Difference between incompatible types".to_string(), &Int(x), &Str(y)),
                        Err(err) => Err(format! {"Error during binary arithmetic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Float(x)) => {
                    match right {
                        Ok(Int(y)) => Ok(Float(x - y as f64)),
                        Ok(Float(y)) => Ok(Float(x - y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Difference between incompatible types".to_string(), &Float(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Difference between incompatible types".to_string(), &Float(x), &Str(y)),
                        Err(err) => Err(format! {"Error during binary arithmetic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Boolean(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Difference between incompatible types".to_string(), &Boolean(x), &Int(y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Difference between incompatible types".to_string(), &Boolean(x), &Float(y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Difference between incompatible types".to_string(), &Boolean(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Difference between incompatible types".to_string(), &Boolean(x), &Str(y)),
                        Err(err) => Err(format! {"Error during binary arithmetic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Str(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Difference between incompatible types".to_string(), &Str(x), &Int(y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Difference between incompatible types".to_string(), &Str(x), &Float(y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Difference between incompatible types".to_string(), &Str(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Difference between incompatible types".to_string(), &Str(x), &Str(y)),
                        Err(err) => Err(format! {"Error during binary arithmetic expression evaluation\n{}\n", err})
                    }
                }
                Err(err) => Err(format! {"Error during arithmetic expression evaluation\n{}\n", err})
            }
        }
        BinaryOperator::Mul => {
            let left = evaluate_expression(scope, &lhs);
            let right = evaluate_expression(scope, &rhs);
            match left {
                Ok(Int(x)) => {
                    match right {
                        Ok(Int(y)) => Ok(Int(x * y)),
                        Ok(Float(y)) => Ok(Float(x as f64 * y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Product between incompatible types".to_string(), &Int(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Product between incompatible types".to_string(), &Int(x), &Str(y)),
                        Err(err) => Err(format! {"Error during binary arithmetic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Float(x)) => {
                    match right {
                        Ok(Int(y)) => Ok(Float(x * y as f64)),
                        Ok(Float(y)) => Ok(Float(x * y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Product between incompatible types".to_string(), &Float(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Product between incompatible types".to_string(), &Float(x), &Str(y)),
                        Err(err) => Err(format! {"Error during binary arithmetic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Boolean(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Product between incompatible types".to_string(), &Boolean(x), &Int(y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Product between incompatible types".to_string(), &Boolean(x), &Float(y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Product between incompatible types".to_string(), &Boolean(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Product between incompatible types".to_string(), &Boolean(x), &Str(y)),
                        Err(err) => Err(format! {"Error during binary arithmetic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Str(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Product between incompatible types".to_string(), &Str(x), &Int(y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Product between incompatible types".to_string(), &Str(x), &Float(y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Product between incompatible types".to_string(), &Str(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Product between incompatible types".to_string(), &Str(x), &Str(y)),
                        Err(err) => Err(format! {"Error during binary arithmetic expression evaluation\n{}\n", err})
                    }
                }
                Err(err) => Err(format! {"Error during arithmetic expression evaluation\n{}\n", err})
            }
        }
        BinaryOperator::Div => {
            let left = evaluate_expression(scope, &lhs);
            let right = evaluate_expression(scope, &rhs);
            match left {
                Ok(Int(x)) => {
                    match right {
                        Ok(Int(y)) => if x % y == 0 { Ok(Int(x / y)) } else { Ok(Float((x as f64) / (y as f64))) },
                        Ok(Float(y)) => Ok(Float(x as f64 / y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Division between incompatible types".to_string(), &Int(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Division between incompatible types".to_string(), &Int(x), &Str(y)),
                        Err(err) => Err(format! {"Error during binary arithmetic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Float(x)) => {
                    match right {
                        Ok(Int(y)) => Ok(Float(x / y as f64)),
                        Ok(Float(y)) => Ok(Float(x / y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Division between incompatible types".to_string(), &Float(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Division between incompatible types".to_string(), &Float(x), &Str(y)),
                        Err(err) => Err(format! {"Error during binary arithmetic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Boolean(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Division between incompatible types".to_string(), &Boolean(x), &Int(y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Division between incompatible types".to_string(), &Boolean(x), &Float(y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Division between incompatible types".to_string(), &Boolean(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Division between incompatible types".to_string(), &Boolean(x), &Str(y)),
                        Err(err) => Err(format! {"Error during binary arithmetic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Str(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Division between incompatible types".to_string(), &Str(x), &Int(y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Division between incompatible types".to_string(), &Str(x), &Float(y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Division between incompatible types".to_string(), &Str(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Division between incompatible types".to_string(), &Str(x), &Str(y)),
                        Err(err) => Err(format! {"Error during binary arithmetic expression evaluation\n{}\n", err})
                    }
                }
                Err(err) => Err(format! {"Error during arithmetic expression evaluation\n{}\n", err})
            }
        }
        BinaryOperator::Mod => {
            let left = evaluate_expression(scope, &lhs);
            let right = evaluate_expression(scope, &rhs);
            match left {
                Ok(Int(x)) => {
                    match right {
                        Ok(Int(y)) => Ok(Int(x % y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Modulo between incompatible types".to_string(), &Int(x), &Float(y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Modulo between incompatible types".to_string(), &Int(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Modulo between incompatible types".to_string(), &Int(x), &Str(y)),
                        Err(err) => Err(format! {"Error during binary arithmetic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Float(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Modulo between incompatible types".to_string(), &Float(x), &Int(y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Modulo between incompatible types".to_string(), &Float(x), &Float(y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Modulo between incompatible types".to_string(), &Float(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Modulo between incompatible types".to_string(), &Float(x), &Str(y)),
                        Err(err) => Err(format! {"Error during binary arithmetic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Boolean(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Modulo between incompatible types".to_string(), &Boolean(x), &Int(y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Modulo between incompatible types".to_string(), &Boolean(x), &Float(y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Modulo between incompatible types".to_string(), &Boolean(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Modulo between incompatible types".to_string(), &Boolean(x), &Str(y)),
                        Err(err) => Err(format! {"Error during binary arithmetic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Str(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Modulo between incompatible types".to_string(), &Str(x), &Int(y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Modulo between incompatible types".to_string(), &Str(x), &Float(y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Modulo between incompatible types".to_string(), &Str(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Modulo between incompatible types".to_string(), &Str(x), &Str(y)),
                        Err(err) => Err(format! {"Error during binary arithmetic expression evaluation\n{}\n", err})
                    }
                }
                Err(err) => Err(format! {"Error during arithmetic expression evaluation\n{}\n", err})
            }
        }
        _ => error_reporting_generic("Unrecognized binary arithmetic operation".to_string()),
    }
}

/// Evaluate binary logic expressions.
pub fn bin_op_logic_evaluator(scope: &&mut Rc<RefCell<Scope>>, lhs: &Box<Expression>, operator: &BinaryOperator, rhs: &Box<Expression>) -> Result<TypeVal, String> {
    match operator {
        BinaryOperator::And => {
            let left = evaluate_expression(scope, &lhs);
            let right = evaluate_expression(scope, &rhs);
            match left {
                Ok(Int(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Logical AND between incompatible types".to_string(), &Int(x), &Int(y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Logical AND between incompatible types".to_string(), &Int(x), &Float(y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Logical AND between incompatible types".to_string(), &Int(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Logical AND between incompatible types".to_string(), &Int(x), &Str(y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Float(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Logical AND between incompatible types".to_string(), &Float(x), &Int(y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Logical AND between incompatible types".to_string(), &Float(x), &Float(y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Logical AND between incompatible types".to_string(), &Float(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Logical AND between incompatible types".to_string(), &Float(x), &Str(y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Boolean(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Logical AND between incompatible types".to_string(), &Boolean(x), &Int(y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Logical AND between incompatible types".to_string(), &Boolean(x), &Float(y)),
                        Ok(Boolean(y)) => Ok(Boolean(x && y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Logical AND between incompatible types".to_string(), &Boolean(x), &Str(y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Str(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Logical AND between incompatible types".to_string(), &Str(x), &Int(y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Logical AND between incompatible types".to_string(), &Str(x), &Float(y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Logical AND between incompatible types".to_string(), &Str(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Logical AND between incompatible types".to_string(), &Str(x), &Str(y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
            }
        }
        BinaryOperator::Or => {
            let left = evaluate_expression(scope, &lhs);
            let right = evaluate_expression(scope, &rhs);
            match left {
                Ok(Int(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Logical OR between incompatible types".to_string(), &Int(x), &Int(y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Logical OR between incompatible types".to_string(), &Int(x), &Float(y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Logical OR between incompatible types".to_string(), &Int(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Logical OR between incompatible types".to_string(), &Int(x), &Str(y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Float(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Logical OR between incompatible types".to_string(), &Float(x), &Int(y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Logical OR between incompatible types".to_string(), &Float(x), &Float(y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Logical OR between incompatible types".to_string(), &Float(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Logical OR between incompatible types".to_string(), &Float(x), &Str(y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Boolean(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Logical OR between incompatible types".to_string(), &Boolean(x), &Int(y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Logical OR between incompatible types".to_string(), &Boolean(x), &Float(y)),
                        Ok(Boolean(y)) => Ok(Boolean(x || y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Logical OR between incompatible types".to_string(), &Boolean(x), &Str(y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Str(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Logical OR between incompatible types".to_string(), &Str(x), &Int(y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Logical OR between incompatible types".to_string(), &Str(x), &Float(y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Logical OR between incompatible types".to_string(), &Str(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Logical OR between incompatible types".to_string(), &Str(x), &Str(y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
            }
        }
        BinaryOperator::Less => {
            let left = evaluate_expression(scope, &lhs);
            let right = evaluate_expression(scope, &rhs);
            match left {
                Ok(Int(x)) => {
                    match right {
                        Ok(Int(y)) => Ok(Boolean(x < y)),
                        Ok(Float(y)) => Ok(Boolean(x < y as i64)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Logical LESS between incompatible types".to_string(), &Int(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Logical LESS between incompatible types".to_string(), &Int(x), &Str(y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Float(x)) => {
                    match right {
                        Ok(Int(y)) => Ok(Boolean(x < y as f64)),
                        Ok(Float(y)) => Ok(Boolean(x < y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Logical LESS between incompatible types".to_string(), &Float(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Logical LESS between incompatible types".to_string(), &Float(x), &Str(y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Boolean(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Logical LESS between incompatible types".to_string(), &Boolean(x), &Int(y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Logical LESS between incompatible types".to_string(), &Boolean(x), &Float(y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Logical LESS between incompatible types".to_string(), &Boolean(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Logical LESS between incompatible types".to_string(), &Boolean(x), &Str(y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Str(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Logical LESS between incompatible types".to_string(), &Str(x), &Int(y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Logical LESS between incompatible types".to_string(), &Str(x), &Float(y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Logical LESS between incompatible types".to_string(), &Str(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Logical LESS between incompatible types".to_string(), &Str(x), &Str(y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})

            }
        }
        BinaryOperator::Greater => {
            let left = evaluate_expression(scope, &lhs);
            let right = evaluate_expression(scope, &rhs);
            match left {
                Ok(Int(x)) => {
                    match right {
                        Ok(Int(y) )=> Ok(Boolean(x > y)),
                        Ok( Float(y)) => Ok(Boolean(x > y as i64)),
                        Ok(Boolean(y) )=> error_reporting_binary_operator("Logical GREATER between incompatible types".to_string(), &Int(x), &Boolean(y)),
                        Ok(  Str(y)) => error_reporting_binary_operator("Logical GREATER between incompatible types".to_string(), &Int(x), &Str(y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Float(x)) => {
                    match right {
                        Ok(Int(y)) => Ok(Boolean(x > y as f64)),
                        Ok(Float(y)) => Ok(Boolean(x > y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Logical GREATER between incompatible types".to_string(), &Float(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Logical GREATER between incompatible types".to_string(), &Float(x), &Str(y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Boolean(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Logical GREATER between incompatible types".to_string(), &Boolean(x), &Int(y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Logical GREATER between incompatible types".to_string(), &Boolean(x), &Float(y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Logical GREATER between incompatible types".to_string(), &Boolean(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Logical GREATER between incompatible types".to_string(), &Boolean(x), &Str(y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Str(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Logical GREATER between incompatible types".to_string(), &Str(x), &Int(y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Logical GREATER between incompatible types".to_string(), &Str(x), &Float(y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Logical GREATER between incompatible types".to_string(), &Str(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Logical GREATER between incompatible types".to_string(), &Str(x), &Str(y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})

            }
        }
        BinaryOperator::LessEq => {
            let left = evaluate_expression(scope, &lhs);
            let right = evaluate_expression(scope, &rhs);
            match left {
                Ok(Int(x)) => {
                    match right {
                        Ok(Int(y)) => Ok(Boolean(x <= y)),
                        Ok(Float(y)) => Ok(Boolean(x <= y as i64)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Logical LEQ between incompatible types".to_string(), &Int(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Logical LEQ between incompatible types".to_string(), &Int(x), &Str(y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Float(x)) => {
                    match right {
                        Ok(Int(y)) => Ok(Boolean(x <= y as f64)),
                        Ok(Float(y)) => Ok(Boolean(x <= y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Logical LEQ between incompatible types".to_string(), &Float(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Logical LEQ between incompatible types".to_string(), &Float(x), &Str(y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Boolean(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Logical LEQ between incompatible types".to_string(), &Boolean(x), &Int(y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Logical LEQ between incompatible types".to_string(), &Boolean(x), &Float(y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Logical LEQ between incompatible types".to_string(), &Boolean(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Logical LEQ between incompatible types".to_string(), &Boolean(x), &Str(y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Str(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Logical LEQ between incompatible types".to_string(), &Str(x), &Int(y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Logical LEQ between incompatible types".to_string(), &Str(x), &Float(y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Logical LEQ between incompatible types".to_string(), &Str(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Logical LEQ between incompatible types".to_string(), &Str(x), &Str(y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})

            }
        }
        BinaryOperator::GreaterEq => {
            let left = evaluate_expression(scope, &lhs);
            let right = evaluate_expression(scope, &rhs);
            match left {
                Ok(Int(x)) => {
                    match right {
                        Ok(Int(y)) => Ok(Boolean(x >= y)),
                        Ok(Float(y)) => Ok(Boolean(x >= y as i64)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Logical GEQ between incompatible types".to_string(), &Int(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Logical GEQ between incompatible types".to_string(), &Int(x), &Str(y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Float(x)) => {
                    match right {
                        Ok(Int(y)) => Ok(Boolean(x >= y as f64)),
                        Ok(Float(y)) => Ok(Boolean(x >= y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Logical GEQ between incompatible types".to_string(), &Float(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Logical GEQ between incompatible types".to_string(), &Float(x), &Str(y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Boolean(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Logical GEQ between incompatible types".to_string(), &Boolean(x), &Int(y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Logical GEQ between incompatible types".to_string(), &Boolean(x), &Float(y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Logical GEQ between incompatible types".to_string(), &Boolean(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Logical GEQ between incompatible types".to_string(), &Boolean(x), &Str(y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Str(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Logical GEQ between incompatible types".to_string(), &Str(x), &Int(y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Logical GEQ between incompatible types".to_string(), &Str(x), &Float(y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Logical GEQ between incompatible types".to_string(), &Str(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Logical GEQ between incompatible types".to_string(), &Str(x), &Str(y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})

            }
        }
        BinaryOperator::CompareEq => {
            let left = evaluate_expression(scope, &lhs);
            let right = evaluate_expression(scope, &rhs);
            match left {
                Ok(Int(x)) => {
                    match right {
                        Ok(Int(y)) => Ok(Boolean(x == y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Logical EQ between incompatible types".to_string(), &Int(x), &Float(y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Logical EQ between incompatible types".to_string(), &Int(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Logical EQ between incompatible types".to_string(), &Int(x), &Str(y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Float(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Logical EQ between incompatible types".to_string(), &Float(x), &Int(y)),
                        Ok(Float(y)) => Ok(Boolean(x == y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Logical EQ between incompatible types".to_string(), &Float(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Logical EQ between incompatible types".to_string(), &Float(x), &Str(y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Boolean(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Logical EQ between incompatible types".to_string(), &Boolean(x), &Int(y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Logical EQ between incompatible types".to_string(), &Boolean(x), &Float(y)),
                        Ok(Boolean(y)) => Ok(Boolean(x == y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Logical EQ between incompatible types".to_string(), &Boolean(x), &Str(y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Str(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Logical EQ between incompatible types".to_string(), &Str(x), &Int(y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Logical EQ between incompatible types".to_string(), &Str(x), &Float(y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Logical EQ between incompatible types".to_string(), &Str(x), &Boolean(y)),
                        Ok(Str(y)) => Ok(Boolean(x == y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})

            }
        }
        BinaryOperator::CompareNeq => {
            let left = evaluate_expression(scope, &lhs);
            let right = evaluate_expression(scope, &rhs);
            match left {
                Ok(Int(x)) => {
                    match right {
                        Ok(Int(y)) => Ok(Boolean(x != y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Logical NEQ between incompatible types".to_string(), &Int(x), &Float(y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Logical NEQ between incompatible types".to_string(), &Int(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Logical NEQ between incompatible types".to_string(), &Int(x), &Str(y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Float(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Logical NEQ between incompatible types".to_string(), &Float(x), &Int(y)),
                        Ok(Float(y)) => Ok(Boolean(x != y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Logical NEQ between incompatible types".to_string(), &Float(x), &Boolean(y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Logical NEQ between incompatible types".to_string(), &Float(x), &Str(y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Boolean(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Logical NEQ between incompatible types".to_string(), &Boolean(x), &Int(y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Logical NEQ between incompatible types".to_string(), &Boolean(x), &Float(y)),
                        Ok(Boolean(y)) => Ok(Boolean(x != y)),
                        Ok(Str(y)) => error_reporting_binary_operator("Logical NEQ between incompatible types".to_string(), &Boolean(x), &Str(y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Ok(Str(x)) => {
                    match right {
                        Ok(Int(y)) => error_reporting_binary_operator("Logical NEQ between incompatible types".to_string(), &Str(x), &Int(y)),
                        Ok(Float(y)) => error_reporting_binary_operator("Logical NEQ between incompatible types".to_string(), &Str(x), &Float(y)),
                        Ok(Boolean(y)) => error_reporting_binary_operator("Logical NEQ between incompatible types".to_string(), &Str(x), &Boolean(y)),
                        Ok(Str(y)) => Ok(Boolean(x != y)),
                        Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
                    }
                }
                Err(err) => Err(format! {"Error during logic expression evaluation\n{}\n", err})
            }
        }
        _ => error_reporting_generic("Unrecognized binary logic operation".to_string()),
    }
}
