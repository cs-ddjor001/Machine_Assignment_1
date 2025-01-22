use std::env;

const MAX_DIGITS: u32 = 8;

fn main() {
    let args: Vec<String> = env::args().collect();

    let f64_numbers: Vec<f64> = args
        .iter()
        .skip(1)
        .flat_map(|arg| arg.parse::<f64>())
        .collect();

    let binary_numbers: Vec<String> = f64_numbers
        .iter()
        .map(|&num| convert_from_decimal_to_binary(num, 2))
        .collect();

    println!("| {:^10} | {:^10} |", "Base 10", "Base 2");

    println!("|{:-<12}|{:-<12}|", ":", ":");

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

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest2::prelude::*;

    #[test]
    fn test_conversion() {
        assert_that!(convert_from_decimal_to_binary(0.5, 2), equal_to("0.1"));
        assert_that!(convert_from_decimal_to_binary(0.25, 2), equal_to("0.01"));
        assert_that!(convert_from_decimal_to_binary(0.75, 2), equal_to("0.11"));
        assert_that!(convert_from_decimal_to_binary(0.125, 2), equal_to("0.001"));
        assert_that!(
            convert_from_decimal_to_binary(0.6875, 2),
            equal_to("0.1011")
        );
        assert_that!(
            convert_from_decimal_to_binary(0.7, 2),
            equal_to("0.10110011")
        );
        assert_that!(
            convert_from_decimal_to_binary(0.8, 2),
            equal_to("0.11001100")
        );
        assert_that!(
            convert_from_decimal_to_binary(0.9, 2),
            equal_to("0.11100110")
        );
        assert_that!(
            convert_from_decimal_to_binary(0.6, 2),
            equal_to("0.10011001")
        );
        assert_that!(
            convert_from_decimal_to_binary(0.3, 2),
            equal_to("0.01001100")
        );
        assert_that!(
            convert_from_decimal_to_binary(0.1, 2),
            equal_to("0.00011001")
        );
    }
}
