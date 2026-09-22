use clap::Parser;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

#[derive(Parser)]
struct Args {
    #[arg(short = 'i', long = "input", value_name = "FILE")]
    input: String,
}

fn get_str(start_addr:usize, data: &[u8]) -> (String, usize) {
    let mut result = String::from("");
    let mut i = start_addr;
    loop {
        if i >= data.len() {
            break;
        }
        if data[i] >=32 && data[i] < 127 {
            result += (data[i] as char).to_string().as_str();
            i += 1;
        } else {
            break;
        }
    }
    let size = i - start_addr;
    (result, size)
}

fn main() {
    let args = Args::parse();
    let input = args.input;
    let mut file = File::open(input).unwrap();
    let page_size = 4096u32;
    let file_size = file.metadata().unwrap().len();
    for i in (0..file_size).step_by(page_size as usize) {
        let mut page = vec![0; page_size as usize];
        file.seek(SeekFrom::Start(i)).unwrap();
        file.read(&mut page).unwrap();
        let mut index = 0usize;
        loop {
            let (str, str_len) = get_str(index, &page);
            if str_len >= 4 {
                println!("0x{:08x}:  {}",i as usize + index , str);
            }
            index += if str_len == 0 { 1 } else { str_len };
            if index >= page_size as usize {
                break;
            }
        }
    }
}
