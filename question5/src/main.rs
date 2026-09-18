use std::fs;

fn hex_str_to_binary(hex_str: &str) -> Vec<u8> {
    let input_str = hex_str.trim();
    let mut result: Vec<u8> = Vec::new();
    for i in (0..input_str.len()).step_by(2) {
        match u8::from_str_radix(&input_str[i..i + 2], 16) {
            Ok(byte) => result.push(byte),
            Err(e) => panic!("Error parsing hex string: {}", e),
        }
    }
    result

}
fn main() {
    let input_str: String = String::from("FFD8FFE0");
    let input_binary = hex_str_to_binary(&input_str);
    println!("{:?}", input_binary);
    fs::write("output.bin", input_binary).unwrap();
}
