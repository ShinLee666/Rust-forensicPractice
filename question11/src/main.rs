use clap::Parser;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
#[derive(Parser)]
struct Args {
    #[arg(short = 'i', long = "input", value_name = "FILE")]
    input: String,
    #[arg(short = 'o', long = "offset", value_name = "OFFSET", default_value = "0")]
    offset: Option<String>,
    #[arg(short = 's', long = "size", value_name = "SIZE", default_value = "256")]
    size: Option<String>,
}
struct HexDump {
    file_path: String,
    offset: u64,
    size: u64,
    data: Option<Vec<u8>>,
}
impl HexDump {
    const DEFAULT_ROW_SIZE:u32 = 16;
    const DEFAULT_TITLE: &str = "OFFSET     0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F  ASCII";
    fn new(file_path: &str, offset: u64, size: u64) -> HexDump {
        HexDump {
            file_path: String::from(file_path),
            offset,
            size,
            data: None,
        }
    }
    fn read_data(&mut self) {
        let mut file = File::open(&self.file_path).unwrap();
        file.seek(SeekFrom::Start(self.offset)).unwrap();
        let mut buffer = vec![0u8; self.size as usize];
        let read_size = file.read(&mut buffer).unwrap();
        buffer.truncate(read_size);
        self.data = Some(buffer);
    }
    fn display(&self) {
        println!("{}",Self::DEFAULT_TITLE);
        for (index, chunk) in self.data.as_ref().unwrap().chunks(Self::DEFAULT_ROW_SIZE as usize).enumerate() {
            print!("0x{:08X} ", self.offset + index as u64 * Self::DEFAULT_ROW_SIZE as u64);
            for byte in chunk {
                print!("{:02X} ", byte);
            }
            for byte in chunk {
                if *byte >= 32 && *byte <= 126 {
                    print!("{}", *byte as char);
                }else {
                    print!(".");
                }
            }
            println!();
        }
    }
}
fn string_to_int(input: &str) -> u64 {
    match input.parse::<u64>() {
        Ok(offset) => offset,
        Err(_) => {
            match u64::from_str_radix(input.to_lowercase().trim_start_matches("0x"), 16) {
                Ok(offset) => offset,
                Err(e) => {
                    eprintln!("[ERROR]{e}");
                    std::process::exit(1);
                }
            }
        }
    }
}
fn main() {
    let args = Args::parse();
    let input = args.input;
    let offset_str = args.offset.unwrap_or("0".to_string());
    let size_str = args.size.unwrap_or("256".to_string());
    let offset = string_to_int(&offset_str);
    let size = string_to_int(&size_str);
    let mut hex_dump = HexDump::new(&input, offset, size);
    hex_dump.read_data();
    hex_dump.display();
}
