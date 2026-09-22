use clap::Parser;
use std::fs::File;
use std::io::{Read, Write};

#[derive(Parser)]
struct Args {
    #[arg(short = 'i', long = "input", value_name = "FILE")]
    input: String,
}
fn png_detect(data: &[u8]) {
    let mut is_additional_data = false;
    for (offset, _) in data.iter().enumerate() {
        if data[offset..offset + 4].iter().eq(b"IEND") && (offset + 8) < data.len() {
            println!("***存在附加数据***");
            is_additional_data = true;
            let addition_data_offset = offset + 8;
            let addition_data = &data[addition_data_offset..];
            let mut addition_data_file = File::create("extract_data").unwrap();
            addition_data_file.write_all(addition_data).unwrap();
            break;
        }
    }
    if !is_additional_data {
        println!("***不存在附加数据***");
    }
}
fn main() {
    let args = Args::parse();
    let mut file = File::open(&args.input).unwrap();
    let mut data = vec![0u8; file.metadata().unwrap().len() as usize];
    file.read(&mut data).unwrap();
    png_detect(&data);
}
