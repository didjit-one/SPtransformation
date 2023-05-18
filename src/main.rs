use std::collections::HashMap;

// Incoming data for encryption
const INPUT_DATA: &str = "10010111";

fn main() {
    s_box();
    p_box();
}

fn s_box() {
    // Generate the constant table for the forward transformation
    let mut s_box_forward_table: HashMap<String, String> = HashMap::new();
    s_box_forward_table.insert(String::from("1001"), String::from("1101"));
    s_box_forward_table.insert(String::from("0111"), String::from("1000"));
    s_box_forward_table.insert(String::from("1010"), String::from("1111"));
    s_box_forward_table.insert(String::from("0010"), String::from("1010"));
    s_box_forward_table.insert(String::from("1101"), String::from("0101"));
    s_box_forward_table.insert(String::from("1000"), String::from("1011"));

    // Generate the constant table for the back transformation
    let mut s_box_back_table: HashMap<String, String> = HashMap::new();
    s_box_back_table.insert(String::from("1101"), String::from("1001"));
    s_box_back_table.insert(String::from("1000"), String::from("0111"));
    s_box_back_table.insert(String::from("1010"), String::from("1111"));
    s_box_back_table.insert(String::from("0010"), String::from("1010"));
    s_box_back_table.insert(String::from("0101"), String::from("0001"));
    s_box_back_table.insert(String::from("1100"), String::from("1011"));

    // Perform the forward S-box transformation
    let output_data = s_box_forward(&INPUT_DATA, &s_box_forward_table);

    println!("Forward S-box Transformation:");
    println!("Input data: {:?}", INPUT_DATA);
    println!("Output data: {:?}", output_data);

    // Perform the backward S-box transformation
    let input_data_backward = s_box_backward(&output_data, &s_box_back_table);

    println!("Backward S-box Transformation:");
    println!("Output data: {:?}", output_data);
    println!("Input data: {:?}", input_data_backward);
}

fn s_box_backward(output_data: &str, s_box_back_table: &HashMap<String, String>) -> String {
    let len = output_data.len();
    let half_len = len / 2;
    let first_half = &output_data[..half_len];
    let second_half = &output_data[half_len..];

    let first_result = s_box_back_table.get(first_half);
    let second_result = s_box_back_table.get(second_half);

    match (first_result, second_result) {
        (Some(first_value), Some(second_value)) => {
            // Return the result of S-block transformation
            format!("{}{}", first_value, second_value)
        }
        _ => {
            println!("No matching value was found for one or both halves.");
            String::new() // Returns an empty string if no value is found
        }
    }
}

fn s_box_forward(input_data: &str, s_box_table: &HashMap<String, String>) -> String {
    let len = input_data.len();
    let half_len = len / 2;
    let first_half = &input_data[..half_len];
    let second_half = &input_data[half_len..];

    let first_result = s_box_table.get(first_half);
    let second_result = s_box_table.get(second_half);

    match (first_result, second_result) {
        (Some(first_value), Some(second_value)) => {
            // Return the result of S-block transformation
            format!("{}{}", first_value, second_value)
        }
        _ => {
            println!("No matching value was found for one or both halves.");
            String::new() // Returns an empty string if no value is found
        }
    }
}

fn p_box() {
    let input_data: Vec<u8> = INPUT_DATA
        .chars()
        .map(|c| c.to_digit(2).unwrap() as u8)
        .collect();
    // Perform the forward P-box transformation
    let output_data = p_box_forward(&input_data);

    println!("Forward P-box Transformation:");
    println!("Input data: {:?}", input_data);
    println!("Output data: {:?}", output_data);

    // Perform the backward P-box transformation
    let input_data_backward = p_box_backward(&output_data);

    println!("Backward P-box Transformation:");
    println!("Output data: {:?}", output_data);
    println!("Input data: {:?}", input_data_backward);
}

fn p_box_forward(input_data: &[u8]) -> Vec<u8> {
    let p_box_permutation = [1, 0, 2, 3, 4, 5, 7, 6];
    let mut output_data = vec![0u8; input_data.len()];
    for i in 0..input_data.len() {
        output_data[i] = input_data[p_box_permutation[i]];
    }
    output_data
}

fn p_box_backward(output_data: &[u8]) -> Vec<u8> {
    let p_box_permutation = [1, 0, 2, 3, 4, 5, 7, 6];
    let mut input_data = vec![0u8; output_data.len()];
    for i in 0..output_data.len() {
        input_data[i] = output_data[p_box_permutation[i]];
    }
    input_data
}
