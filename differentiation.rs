use std::fmt;
use std::io;

/// Enum to represent mathematical expressions
#[derive(Clone, Debug)]
enum Expr {
    Const(f64),         // A constant value
    Var(String),        // A variable (e.g., "x")
    Add(Box<Expr>, Box<Expr>), // Addition
    Sub(Box<Expr>, Box<Expr>), // Subtraction
    Mul(Box<Expr>, Box<Expr>), // Multiplication
    Div(Box<Expr>, Box<Expr>), // Division
    Pow(Box<Expr>, f64),       // Power (e.g., x^n)
}

impl Expr {
    /// Compute the derivative of the expression with respect to the given variable
    fn differentiate(&self, var: &str) -> Expr {
        match self {
            Expr::Const(_) => Expr::Const(0.0), // Derivative of a constant is 0
            Expr::Var(v) => {
                if v == var {
                    Expr::Const(1.0) // Derivative of the variable w.r.t itself is 1
                } else {
                    Expr::Const(0.0) // Derivative of an unrelated variable is 0
                }
            }
            Expr::Add(lhs, rhs) => {
                // (f + g)' = f' + g'
                Expr::Add(Box::new(lhs.differentiate(var)), Box::new(rhs.differentiate(var)))
            }
            Expr::Sub(lhs, rhs) => {
                // (f - g)' = f' - g'
                Expr::Sub(Box::new(lhs.differentiate(var)), Box::new(rhs.differentiate(var)))
            }
            Expr::Mul(lhs, rhs) => {
                // (f * g)' = f' * g + f * g'
                Expr::Add(
                    Box::new(Expr::Mul(Box::new(lhs.differentiate(var)), rhs.clone())),
                    Box::new(Expr::Mul(lhs.clone(), Box::new(rhs.differentiate(var)))),
                )
            }
            Expr::Div(lhs, rhs) => {
                // (f / g)' = (f' * g - f * g') / g^2
                Expr::Div(
                    Box::new(Expr::Sub(
                        Box::new(Expr::Mul(Box::new(lhs.differentiate(var)), rhs.clone())),
                        Box::new(Expr::Mul(lhs.clone(), Box::new(rhs.differentiate(var)))),
                    )),
                    Box::new(Expr::Pow(rhs.clone(), 2.0)),
                )
            }
            Expr::Pow(base, exp) => {
                // (x^n)' = n * x^(n-1)
                Expr::Mul(
                    Box::new(Expr::Const(*exp)),
                    Box::new(Expr::Pow(base.clone(), exp - 1.0)),
                )
            }
        }
    }
}

/// Implement display formatting for expressions
impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Const(c) => write!(f, "{}", c),
            Expr::Var(v) => write!(f, "{}", v),
            Expr::Add(lhs, rhs) => write!(f, "({} + {})", lhs, rhs),
            Expr::Sub(lhs, rhs) => write!(f, "({} - {})", lhs, rhs),
            Expr::Mul(lhs, rhs) => write!(f, "({} * {})", lhs, rhs),
            Expr::Div(lhs, rhs) => write!(f, "({} / {})", lhs, rhs),
            Expr::Pow(base, exp) => write!(f, "({}^{})", base, exp),
        }
    }
}

/// Parse an expression recursively, including nested parentheses
fn parse_expression(input: &str) -> Result<Expr, String> {
    let input = input.trim();
    if input.starts_with('(') && input.ends_with(')') {
        return parse_expression(&input[1..input.len() - 1]); // Strip outer parentheses
    }

    // Try to parse a simple constant or variable
    if let Ok(value) = input.parse::<f64>() {
        return Ok(Expr::Const(value));
    }
    if input.chars().all(|c| c.is_alphabetic()) {
        return Ok(Expr::Var(input.to_string()));
    }

    // Parse binary operations
    let mut balance = 0;
    let mut operator_index = None;

    for (i, ch) in input.chars().enumerate() {
        match ch {
            '(' => balance += 1,
            ')' => balance -= 1,
            '+' | '-' | '*' | '/' | '^' if balance == 0 => {
                operator_index = Some(i);
                break; // Stop at the first operator outside parentheses
            }
            _ => {}
        }
    }

    if let Some(index) = operator_index {
        let op = input.chars().nth(index).unwrap();
        let lhs = &input[..index];
        let rhs = &input[index + 1..];

        let left_expr = parse_expression(lhs)?;
        let right_expr = parse_expression(rhs)?;

        return match op {
            '+' => Ok(Expr::Add(Box::new(left_expr), Box::new(right_expr))),
            '-' => Ok(Expr::Sub(Box::new(left_expr), Box::new(right_expr))),
            '*' => Ok(Expr::Mul(Box::new(left_expr), Box::new(right_expr))),
            '/' => Ok(Expr::Div(Box::new(left_expr), Box::new(right_expr))),
            '^' => {
                if let Expr::Const(exp) = right_expr {
                    Ok(Expr::Pow(Box::new(left_expr), exp))
                } else {
                    Err("Exponent must be a constant".to_string())
                }
            }
            _ => Err("Unsupported operator".to_string()),
        };
    }

    Err("Invalid expression format".to_string())
}

fn main() {
    let mut input = String::new();
    println!("Enter a mathematical expression (e.g., ((x^3) + (2 * x))):");
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    println!("Enter the variable for differentiation (e.g., x):");
    let mut var = String::new();
    io::stdin().read_line(&mut var).unwrap();
    let var = var.trim();

    match parse_expression(input) {
        Ok(expr) => {
            println!("Expression: {}", expr);
            let derivative = expr.differentiate(var);
            println!("Derivative: {}", derivative);
        }
        Err(err) => println!("Error parsing expression: {}", err),
    }
}
