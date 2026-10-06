use std::io;
pub fn run() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let n: i32 = input.trim().parse().unwrap();

    let mut valid: Vec<String> = Vec::new();
    let mut s = String::new();

    generate(&mut s, n, n, &mut valid);

    for ele in valid {
        println!("{}", ele);
    }
}

fn generate(s: &mut String, open: i32, close: i32, valid: &mut Vec<String>) {
    if open == 0 && close == 0 {
        valid.push(s.clone());
        return;
    }

    if open > 0 {
        s.push('(');
        generate(s, open - 1, close, valid);
        s.pop();
    }

    if close > 0 {
        if open < close {
            s.push(')');
            generate(s, open, close - 1, valid);
            s.pop();
        }
    }
}

