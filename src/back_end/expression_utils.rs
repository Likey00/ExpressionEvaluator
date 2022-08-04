use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Open,
    Close,
}

impl Operator {
    fn get_priority(&self) -> i32 {
        match self {
            Open | Close => 0,
            Add | Sub => 1,
            Mul | Div => 2,
        }
    }
}

impl Ord for Operator {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_priority().cmp(&other.get_priority())
    }
}

#[derive(Debug)]
enum Token {
    Num(f64),
    Op(Operator),
}

use Operator::{Add, Sub, Mul, Div, Open, Close};
use Token::{Num, Op};

fn tokenize(s: &str) -> Result<Vec<Token>, &str> {
    let mut tokens = Vec::new();
    let mut current_num = "".to_owned();
    let mut open_count = 0;

    for c in s.chars() {
        match c {
            ' ' | '\n' => continue,
            '0'..='9' | '.' => current_num.push(c),
            '(' => {
                if !current_num.is_empty() {
                    tokens.push(Num(current_num.parse().unwrap()));
                    current_num.clear();
                    tokens.push(Op(Mul));
                }
                else if let Some(Op(Close)) = tokens.last() {
                    tokens.push(Op(Mul));
                }

                open_count += 1;
                tokens.push(Op(Open));
            },
            '+' | '-' | '*' | '/' | ')' => {
                if !current_num.is_empty() {
                    tokens.push(Num(current_num.parse().unwrap()));
                    current_num.clear();
                }
                else if !matches!(tokens.last(), Some(Op(Close))) {
                    if c == '-' { 
                        current_num.push(c);
                        continue;
                    }
                    return Err("Operator before number");
                }
                
                let operator = match c {
                    '+' => Add,
                    '-' => Sub,
                    '*' => Mul,
                    '/' => Div,
                    _ => {
                        if open_count < 1 { return Err("Closing paren before open"); }
                        open_count -= 1;
                        Close
                    },
                };

                tokens.push(Op(operator));
            },
            _ => return Err("Invalid character in input"),
        }
    }
    
    if !current_num.is_empty() {
        tokens.push(Num(current_num.parse().unwrap()));
    }
    else if !matches!(tokens.last(), Some(Op(Close))) {
        return Err("Need number at the end");
    }

    if open_count != 0 { return Err("Unbalanced parenthesis"); }

    Ok(tokens)
}

fn to_postorder(inorder: Vec<Token>) -> Vec<Token> {
    let mut stack = Vec::new();
    let mut postorder = Vec::new();

    for t in inorder {
        match t {
            Num(num) => postorder.push(Num(num)),
            Op(Open) => stack.push(Open),
            Op(Close) => {
                while let Some(stack_op) = stack.pop() {
                    if stack_op == Open { break; }
                    postorder.push(Op(stack_op));
                }
            },
            Op(op) => {
                while let Some(stack_op) = stack.pop() {
                    if matches!(op.cmp(&stack_op), Ordering::Greater) { 
                        stack.push(stack_op);
                        break;
                    }
                    
                    postorder.push(Op(stack_op));
                }

                stack.push(op);
            }
        }
    }

    while let Some(op) = stack.pop() {
        postorder.push(Op(op));
    }

    postorder
}

fn evaluate_postorder(postorder: Vec<Token>) -> Option<f64> {
    let mut stack = Vec::new();

    for t in postorder {
        match t {
            Num(num) => stack.push(num),
            Op(op) => {
                if let (Some(num2), Some(num1)) = (stack.pop(), stack.pop()) {
                    stack.push(match op {
                        Add => num1 + num2,
                        Sub => num1 - num2,
                        Mul => num1 * num2,
                        Div => num1 / num2,
                        _ => return None,
                    });
                }
                else { return None; }
            }
        }
    }
    
    stack.pop()
}

pub fn evaluate_expression(s: &str) -> Result<f64, &str> {
    match tokenize(s) {
        Err(e) => Err(e),
        Ok(tokens) => {
            match evaluate_postorder(to_postorder(tokens)) {
                Some(result) => Ok(result),
                None => Err("Something went wrong during evaluation")
            }
        }
    }
}