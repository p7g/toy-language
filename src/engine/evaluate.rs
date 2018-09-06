use super::{ AST, Environment };

pub fn evaluate(ast: AST, mut env: Environment) -> Option<AST> {
    match ast {
        AST::Number(_) | AST::String_(_) | AST::Boolean(_) | AST::Variable(_) => Some(ast),
        AST::Program(exprs) => {
            for expr in exprs.iter() {
                evaluate(expr.clone(), Environment::new(Some(&env)));
            }
            None
        },
        AST::Assign { left, right, .. } => {
            if let AST::Variable(name) = (*left).clone() {
                if let Some(result) = evaluate(*right.clone(), Environment::new(Some(&env))) {
                    env.def(&name, result);
                }
                None
            }
            else {
                env.dump();
                panic!(format!("Can only assign to variable: {:?} = {:?}", left, right));
            }
        },
        func @ AST::Function { .. } => {
            Some(func)
        },
        AST::Call { function, arguments } => {
            let mut fnenv = Environment::new(Some(&env));
            if let AST::Variable(name) = *function.clone() {
                if let function @ AST::Function { .. } = env.get(&name) {
                    if let AST::Function { parameters, body, native } = function {
                        if let Some(f) = native {
                            Some(f(arguments))
                        }
                        else {
                            for i in 0..parameters.len() {
                                let name = &parameters[i];
                                let value = if let Some(value) = arguments.get(i) {
                                    value.clone()
                                }
                                else {
                                    AST::Boolean(false)
                                };
                                fnenv.def(name, value);
                            }
                            evaluate((*body).clone(), fnenv)
                        }
                    }
                    else {
                        env.dump();
                        panic!(format!("Cannot call non-function '{:?}'", function));
                    }
                }
                else {
                    env.dump();
                    panic!(format!("Cannot call non-function '{:?}'", function));
                }
            }
            else {
                panic!(format!("Unrecognized token: {:?}", function));
            }
        },
        AST::If { condition, then, otherwise } => {
            let cond = evaluate(*condition, Environment::new(Some(&env)));
            match cond {
                Some(AST::Boolean(b)) => {
                    if b {
                        evaluate(*then, Environment::new(Some(&env)))
                    }
                    else if let Some(exp) = otherwise {
                        evaluate(*exp, Environment::new(Some(&env)))
                    }
                    else {
                        Some(AST::Boolean(false))
                    }
                },
                _ => {
                    env.dump();
                    panic!("Condition must evaluate to boolean");
                }
            }
        },
        AST::Binary { operator, left, right } => {
            if let Some(left) = evaluate(*left, Environment::new(Some(&env))) {
                if let Some(right) = evaluate(*right, Environment::new(Some(&env))) {
                    match operator.as_ref() {
                        "+" => Some(add(left, right, &env)),
                        "-" => Some(subtract(left, right, &env)),
                        "*" => Some(multiply(left, right, &env)),
                        "/" => Some(divide(left, right, &env)),
                        "%" => Some(modulus(left, right, &env)),
                        "||" => Some(or(left, right, &env)),
                        "&&" => Some(and(left, right, &env)),
                        "==" => Some(equals(left, right, &env)),
                        "<" => Some(less_than(left, right, &env)),
                        ">" => Some(greater_than(left,right, &env)),
                        "<=" => Some(less_than_or_equals(left, right, &env)),
                        ">=" => Some(greater_than_or_equals(left, right, &env)),
                        _ => {
                            env.dump();
                            panic!(format!("Unkown operator '{:?}'", operator));
                        }
                    }
                }
                else {
                    env.dump();
                    panic!("Could not evaluate right operand");
                }
            }
            else {
                env.dump();
                panic!("Unable to evaluate left operand");
            }
        }
    }
}

fn lookup_or_self(ast: &AST, env: &Environment) -> AST {
    if let AST::Variable(name) = ast {
        env.get(&name)
    }
    else {
        ast.clone()
    }
}

fn add(left: AST, right: AST, env: &Environment) -> AST {
    let left = lookup_or_self(&left, env);
    let right = lookup_or_self(&right, env);
    match (&left, &right) {
        (AST::Number(l), AST::Number(r)) => AST::Number(l + r),
        (AST::String_(l), AST::String_(r)) => AST::String_([l.as_ref(), r.as_ref()].join("")),
        _ => panic!(format!("Cannot add operands: {:?} + {:?}", left, right))
    }
}

fn subtract(left: AST, right: AST, env: &Environment) -> AST {
    let left = lookup_or_self(&left, env);
    let right = lookup_or_self(&right, env);
    match (&left, &right) {
        (AST::Number(l), AST::Number(r)) => AST::Number(l - r),
        _ => panic!(format!("Cannot subtract operands: {:?} - {:?}", left, right))
    }
}

fn multiply(left: AST, right: AST, env: &Environment) -> AST {
    let left = lookup_or_self(&left, env);
    let right = lookup_or_self(&right, env);
    match (&left, &right) {
        (AST::Number(l), AST::Number(r)) => AST::Number(l * r),
        (AST::String_(l), AST::Number(r)) => AST::String_(l.repeat(*r as usize)),
        _ => panic!(format!("Cannot multiply operands: {:?} * {:?}", left, right))
    }
}

fn divide(left: AST, right: AST, env: &Environment) -> AST {
    let left = lookup_or_self(&left, env);
    let right = lookup_or_self(&right, env);
    match (&left, &right) {
        (AST::Number(l), AST::Number(r)) => AST::Number(l / r),
        _ => panic!(format!("Cannot divide operands: {:?} / {:?}", left, right))
    }
}

fn modulus(left: AST, right: AST, env: &Environment) -> AST {
    let left = lookup_or_self(&left, env);
    let right = lookup_or_self(&right, env);
    match (&left, &right) {
        (AST::Number(l), AST::Number(r)) => AST::Number(l % r),
        _ => panic!(format!("Cannot modulus operands: {:?} % {:?}", left, right))
    }
}

fn or(left: AST, right: AST, env: &Environment) -> AST {
    if let AST::Boolean(b) = lookup_or_self(&left, env) {
        if !b {
            if let AST::Boolean(b2) = lookup_or_self(&right, env) {
                AST::Boolean(b || b2)
            }
            else {
                panic!(format!("Cannot OR operands: {:?} || {:?}", left, right));
            }
        }
        else {
            AST::Boolean(true)
        }
    }
    else {
        panic!(format!("Cannot OR operands: {:?} || {:?}", left, right));
    }
}

fn and(left: AST, right: AST, env: &Environment) -> AST {
    if let AST::Boolean(b) = lookup_or_self(&left, env) {
        if b {
            if let AST::Boolean(b2) = lookup_or_self(&right, env) {
                AST::Boolean(b || b2)
            }
            else {
                panic!(format!("Cannot AND operands: {:?} && {:?}", left, right));
            }
        }
        else {
            AST::Boolean(false)
        }
    }
    else {
        panic!(format!("Cannot AND operands: {:?} && {:?}", left, right));
    }
}

fn equals(left: AST, right: AST, env: &Environment) -> AST {
    let left = lookup_or_self(&left, env);
    let right = lookup_or_self(&right, env);
    AST::Boolean(left == right)
}

fn less_than(left: AST, right: AST, env: &Environment) -> AST {
    let left = lookup_or_self(&left, env);
    let right = lookup_or_self(&right, env);
    match (&left, &right) {
        (AST::Number(l), AST::Number(r)) => AST::Boolean(l < r),
        _ => panic!(format!("Cannot compare operands: {:?} < {:?}", left, right))
    }
}

fn less_than_or_equals(left: AST, right: AST, env: &Environment) -> AST {
    let left = lookup_or_self(&left, env);
    let right = lookup_or_self(&right, env);
    match (&left, &right) {
        (AST::Number(l), AST::Number(r)) => AST::Boolean(l <= r),
        _ => panic!(format!("Cannot compare operands: {:?} <= {:?}", left, right))
    }
}

fn greater_than(left: AST, right: AST, env: &Environment) -> AST {
    let left = lookup_or_self(&left, env);
    let right = lookup_or_self(&right, env);
    match (&left, &right) {
        (AST::Number(l), AST::Number(r)) => AST::Boolean(l > r),
        _ => panic!(format!("Cannot compare operands: {:?} > {:?}", left, right))
    }
}

fn greater_than_or_equals(left: AST, right: AST, env: &Environment) -> AST {
    let left = lookup_or_self(&left, env);
    let right = lookup_or_self(&right, env);
    match (&left, &right) {
        (AST::Number(l), AST::Number(r)) => AST::Boolean(l >= r),
        _ => panic!(format!("Cannot compare operands: {:?} >= {:?}", left, right))
    }
}
