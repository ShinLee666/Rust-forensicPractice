use clap::Parser;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

/// 一个简易文件hexdump工具
#[derive(Parser)]
#[command(
name = "simple-file-hexdump",
version,
about = "A simple tool for hexdump files."
)]
struct Args {
    /// 输入文件的路径
    #[arg(short = 'f', long = "file", help = "Read from a file")]
    file: String,
    /// 起始偏移（10进制数或0x开头的16进制数）
    #[arg(short = 'o', long = "offset", help = "Target offset")]
    offset: String,
    /// 读取字节数（10进制数或0x开头的16进制数）
    #[arg(short = 's', long = "size", help = "Target size")]
    size: String,
}

fn main() {
    let cli = Args::parse();
    let file_path = cli.file;
    let offset = match cli.offset.parse::<u64>() {
        Ok(offset) => offset,
        Err(_e) => {
            match u64::from_str_radix(cli.offset.to_lowercase().trim_start_matches("0x"), 16) {
                Ok(offset) => offset,
                Err(e) => {
                    eprintln!("[ERROR] {e}");
                    std::process::exit(1);
                }
            }
        }
    };
    let size = match cli.size.parse::<usize>() {
        Ok(size) => size,
        Err(_e) => {
            match usize::from_str_radix(cli.size.to_lowercase().trim_start_matches("0x"), 16) {
                Ok(size) => size,
                Err(e) => {
                    eprintln!("[ERROR] {e}");
                    std::process::exit(1);
                }
            }
        }
    };
    let mut file = File::open(file_path).unwrap();
    file.seek(SeekFrom::Start(offset)).unwrap();
    let mut buffer = vec![0u8; size];
    let bytes_read = file.read(&mut buffer).unwrap();
    buffer.truncate(bytes_read);
    for (index, chunk) in buffer.chunks(16).enumerate() {
        let addr = offset + index as u64 * 16;
        print!("0x{:08X}  ", addr);
        for byte in chunk {
            print!("{:02X} ", byte);
        }
        println!();
    }
}
