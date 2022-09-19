//! # g_calc
//! 
//! `g_calc` is a small utility to make performing calculations more convenient.


/// Checks the priority of the operator and returns appropriate value
///
/// # Examples
///
/// ```
/// let operator = '+';
/// let output = g_calc::priority(&operator);
///
/// assert_eq!(1,output);
/// ```
pub fn priority(ch: &char) -> u8 {
    if *ch == '^' {
        return 3;
    } else if *ch == '*' || *ch == '/' {
        return 2;
    } else if *ch == '+' || *ch == '-' {
        return 1;
    } else {
        return 0;
    }
}

/// Converts the given string slice input to post-fix expression and 
/// returns a Result type of String and static error message
///
/// # Examples
///
/// ```
/// let expr = "1+2*(3^4-5)^(6+7*8)-9"; 
/// let postfix_expr = g_calc::convert(expr).unwrap();
///
/// assert_eq!(postfix_expr,"1 2  3 4 ^ 5 -   6 7 8 * +  ^ * + 9 -");
/// ```
pub fn convert(expr: &str) -> Result<String, &'static str> {
    let mut is_bracket_closed:bool = true;
    let mut stack: Vec<char> = Vec::new();
    let mut postfix_expr = String::new();
    let mut iter = expr.chars();
    while let Some(mut ch) = iter.next() {                   
            let mut c = ch;
            while c.is_ascii_digit() || c == ' ' || c == '.' {
                if !c.is_whitespace(){
                    postfix_expr.push(c);
                }                
                c = match iter.next(){
                    Some(v) => v,
                    None => break
                };            
            }
            ch = c;
            postfix_expr.push(' ');            
        if ch.is_ascii_whitespace() {
            continue;
        } else if ch == '+' || ch == '-' || ch == '*' || ch == '/' || ch == '^' {
            if stack.is_empty() {
                stack.push(ch);
            } else if !stack.is_empty() && priority(&ch) > priority(&stack.last().unwrap()) {
                stack.push(ch);
            } else {
                while !stack.is_empty() && priority(&ch) <= priority(&stack.last().unwrap()) {
                    postfix_expr.push(stack.pop().unwrap());
                    postfix_expr.push(' ');
                }
                stack.push(ch);
            }
        } else {
            if ch == '(' {
                stack.push(ch);
                is_bracket_closed = false;
            } else if ch == ')' {
                while match stack.last(){
                    Some(v)=> v,
                    None => {
                        return Err("Syntax error : open bracket missing");                        
                    }
                } != &'(' {
                    postfix_expr.push(stack.pop().unwrap());
                    postfix_expr.push(' ');
                }
                stack.pop();
                is_bracket_closed = true;
            }
            else{
                break;
            }
        }
    }
    while !stack.is_empty() {
        postfix_expr.push(stack.pop().unwrap());
        postfix_expr.push(' ');
    }
    postfix_expr =  postfix_expr.trim().to_string();
    if is_bracket_closed && !postfix_expr.is_empty(){
        Ok(postfix_expr)
    }else if !is_bracket_closed{
        Err("Syntax error : close bracket missing")
        
    }else{
        Err("Syntax error : wrong input provided")        
    }
}

/// solves the postfix expression and returns the result
/// 
/// # Examples
/// 
/// ```
/// let post_fix_expr = "1 2  3 4 ^ 5 -   6 7 8 * +  ^ * + 9 -";
/// let output = g_calc::solve(post_fix_expr).unwrap();
/// 
/// assert_eq!(output,8.155915490338936e116);
/// ```
pub fn solve(postfix_expr: &str) -> Result<f64,&'static str> {
    // dbg!(&postfix_expr);
    let mut stack: Vec<f64> = Vec::new();
    for ch in postfix_expr.split_whitespace() {
        if ch.chars().all(|x| x.is_ascii_digit() || x == '.') {
            stack.push(match ch.trim().parse(){
                Ok(v) => v,
                Err(_) => {
                    return Err("Parse error : invalid number");
                }
            });
        } else {
            let num2 = match stack.pop(){
                Some(v) => v,
                None => {
                    return Err("Syntax error : wrong value entered");
                }
            };
            let num1 = match stack.pop(){
                Some(v) => v,
                None => {
                    return Err("Syntax error : wrong value entered");
                }
            };
            match ch.chars().next().unwrap() {
                '+' => stack.push(num1 + num2),
                '-' => stack.push(num1 - num2),
                '*' => stack.push(num1 * num2),
                '/' => {
                    if num2 == 0.0{
                        return Err("Divide by zero error");
                    }
                    stack.push(num1 / num2)
                },
                '^' => stack.push(f64::powf(num1, num2)),
                _ => {}
            }
        }
    }
    Ok(stack.pop().unwrap())
}

#[cfg(test)]
mod tests {
    use super::{priority,convert,solve};
    #[test]
    fn test_priority() {
        assert_eq!(3,priority(&'^'));
        assert_eq!(2,priority(&'*'));
        assert_eq!(2,priority(&'/'));
        assert_eq!(1,priority(&'-'));
        assert_eq!(1,priority(&'+'));
        assert_eq!(0,priority(&'!'));        
    }

    #[test]
    fn test_convert(){
        assert_eq!("1 2 +",convert("1 + 2").unwrap());
        assert_eq!("2 3 4 * +",convert("2 + 3 * 4").unwrap());
        assert_eq!("0.156 2  1 2 /  ^ *  14  3 5 *  *  2 / +",convert("0.156 * 2 ^ (1 / 2) + (14 * ( 3* 5))/2").unwrap());
    }
    #[test]
    fn test_solve(){
        assert_eq!(3.00,solve("1 2 +").unwrap());
        assert_eq!(14.00,solve("2 3 4 * +").unwrap());
        assert_eq!(105.22061731573021,solve("0.156 2 1 2 / ^ * 14 3 5 * * 2 / +").unwrap());
    }
}

