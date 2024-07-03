use colored::Colorize;
use std::io::Write;

pub fn prompt_string(message: &str) -> String {
    print!("{}", format!("{:>22}", message).yellow());
    print!(" > ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();

    let result = std::io::stdin().read_line(&mut input);

    if result.is_ok() {
        return input.trim().to_uppercase();
    }

    println!("{:?}", result.as_ref().err());
    return prompt_string(message);
}

pub fn promt_i32(message: &str) -> i32 {
    let input = prompt_string(message);
    let result = input.parse::<i32>();

    return match result {
        Ok(result) => result,
        Err(error) => {
            red_line(format!("{error:?}"));
            return promt_i32(message);
        }
    };
}

pub fn promt_u32(message: &str) -> u32 {
    let input = prompt_string(message);
    let result = input.parse::<u32>();

    return match result {
        Ok(result) => result,
        Err(error) => {
            red_line(format!("{error:?}"));
            return promt_u32(message);
        }
    };
}

pub fn red_line(message: String) {
    println!("{}", message.red());
}

pub fn green_line(message: String) {
    println!("{}", message.green());
}
