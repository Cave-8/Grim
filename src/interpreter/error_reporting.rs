use crate::interpreter::interpreter::TypeVal;
use colored::Colorize;

/// Build a generic error message
pub fn error_reporting_generic(err_message: String) -> Result<TypeVal, String> {
    let err_mess = err_message.red();
    Err(format!("{}", err_mess))
}

/// Build and return an error message for unary operator
pub fn error_reporting_unary_operator(
    err_message: String,
    val1: &TypeVal,
) -> Result<TypeVal, String> {
    let err_mess = err_message.red();
    let var1 = format!("{:?}", val1);
    Err(format!("{} -> {}", err_mess, var1))
}

/// Build and return an error message for binary operator
pub fn error_reporting_binary_operator(
    err_message: String,
    val1: &TypeVal,
    val2: &TypeVal,
) -> Result<TypeVal, String> {
    let err_mess = err_message.red();
    let var1 = format!("{:?}", val1);
    let var2 = format!("{:?}", val2);
    Err(format!("{} -> {} and {}", err_mess, var1, var2))
}
