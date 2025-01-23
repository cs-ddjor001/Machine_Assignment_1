use std::env;

const MAX_DIGITS: u32 = 8;

/// The entry point of the program that reads command-line arguments,
/// converts the arguments from decimal to binary, and prints the results.
///
/// This function expects the command-line arguments to be a list of
/// floating-point numbers in base 10, which will be converted to binary
/// fractional representations. The program prints a table displaying the
/// original decimal numbers and their binary equivalents.
fn main() {
    let f64_numbers = parse_input();

    let binary_numbers: Vec<String> = f64_numbers
        .iter()
        .map(|&num| convert_from_decimal_to_binary(num, 2))
        .collect();

    display(f64_numbers, binary_numbers);
}

/// Reads fractional numbers in base 10 from the command-line arguments
/// and parses them into a vector of `f64` values.
///
/// # Returns
///
/// A vector containing the parsed `f64` decimal numbers.
///
/// # Panics
///
/// This function assumes all arguments after the program name are valid
/// floating-point numbers. If invalid arguments are provided, they will
/// be skipped.
///
/// # Example
/// ```
/// // Assuming the program is run as follows:
/// // cargo run 0.1 0.25 0.5
/// let parsed = parse_input();
/// assert_eq!(parsed, vec![0.1, 0.25, 0.5]);
/// ```
fn parse_input() -> Vec<f64> {
    let args: Vec<String> = env::args().collect();

    let f64_numbers: Vec<f64> = args
        .iter()
        .skip(1)
        .flat_map(|arg| arg.parse::<f64>())
        .collect();
    f64_numbers
}

/// Converts a decimal number (f64) to its binary fractional representation as a string.
///
/// # Arguments
///
/// * `decimal` - The fractional number to convert.
/// * `target_base` - The base to convert to. For binary, this must be 2.
///
/// # Returns
///
/// A `String` containing the binary fractional representation of the input `decimal`.
/// The binary representation is truncated to `MAX_DIGITS` precision.
///
/// # Example
/// ```
/// let binary = convert_from_decimal_to_binary(0.75, 2);
/// assert_eq!(binary, "0.11");
/// ```
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

/// Outputs the decimal numbers and their binary fractional representations in a table format.
///
/// # Arguments
///
/// * `f64_numbers` - A vector of decimal numbers in base 10.
/// * `binary_numbers` - A vector of binary fractional strings corresponding to the decimal numbers.
///
/// # Example
/// ```
/// display(vec![0.5, 0.25], vec!["0.1".to_string(), "0.01".to_string()]);
/// ```
/// Output:
/// |   Base 10   |   Base 2   |
/// |:------------|:-----------|
/// | 0.5         | 0.1        |
/// | 0.25        | 0.01       |
fn display(f64_numbers: Vec<f64>, binary_numbers: Vec<String>) {
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
