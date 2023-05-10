use ::std::env::{args, Args};

pub fn calculate() {
    let mut args: Args = args();
    let first_number = args.nth(1).unwrap();
    let first_number = first_number.parse::<f32>().unwrap();
    let operator = args.nth(0).unwrap();
    let operator = operator.parse::<char>().unwrap();
    let second_number = args.nth(0).unwrap();
    let second_number = second_number.parse::<f32>().unwrap();

    let result = math_operation(operator, first_number, second_number);
    println!(
        "{} {} {} = {}",
        first_number, operator, second_number, result
    );
}

fn math_operation(operator: char, first_number: f32, second_number: f32) -> f32 {
    match operator {
        '+' => first_number + second_number,
        '-' => first_number - second_number,
        '*' | 'x' | 'X' => first_number * second_number,
        '/' => first_number / second_number,
        _ => panic!("Invalid Operator!"),
    }
}
