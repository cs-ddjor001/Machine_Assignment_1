use std::env;

const MAX_DIGITS: u32 = 8;

fn main() {
    let args: Vec<String> = env::args().collect();

    let f64_numbers: Vec<f64> = args
        .iter()
        .skip(1)
        .flat_map(|arg| arg.parse::<f64>())
        .collect();

    let mut binary_numbers: Vec<String> = Vec::new();
    for num in f64_numbers.iter() {
        binary_numbers.push(convert_from_decimal_to_binary(*num, 2));
    }

    println!(
        "| {:^10} | {:^10} |",
        "Base 10", "Base 2"
    );

    println!(
        "|{:-<12}|{:-<12}|",
        ":", ":"
    );

    for i in 0..binary_numbers.len() {
        println!(
            "| {:<7} | {:<10} |",
            format!("{:.1$}", f64_numbers[i], MAX_DIGITS as usize), 
            binary_numbers[i]
        );
    }
}

fn convert_from_decimal_to_binary(decimal: f64, target_base: u32) -> String {
    let mut result = String::from("0.");
    let mut fraction = decimal;

    for _ in 0..MAX_DIGITS {
        fraction *= target_base as f64;
        let digit = fraction.floor() as u32;
        result.push_str(&digit.to_string());
        fraction -= digit as f64;

        if fraction == 0.0 {
            break;
        }
    }

    result
}
