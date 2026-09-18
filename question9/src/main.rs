use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg[short = 'i', long = "input"]]
    input: String,
}

fn main() {
    let cli = Args::parse();
    let input = cli.input;
    println!("input: {}", input);
    let input_hex:u32 = u32::from_str_radix(input.to_lowercase().trim_start_matches("0x"), 16).unwrap();
    println!("input hex: {}", input_hex);
    println!("bigendian: {:?}", input_hex.to_be_bytes());
    println!("littleendian: {:?}", input_hex.to_le_bytes());
}
