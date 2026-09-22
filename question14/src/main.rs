use std::fmt;
use std::fmt::{Display, Formatter};
use clap::Parser;
use std::fs::File;
use std::io::{Read};
use std::str::FromStr;


#[derive(Parser)]
struct Args {
    #[arg(short = 'i', long = "input", value_name = "FILE")]
    input: String,
}
#[derive(Default)]
enum ChunkType {
    IHDR,
    PLTE,
    IDAT,
    IEND,
    tEXt,
    zTXt,
    iTXt,
    tIME,
    pHYs,
    gAMA,
    cHRM,
    sRGB,
    iCCP,
    sBIT,
    bKGD,
    hIST,
    tRNS,
    sPLT,
    eXIf,
    #[default]
    Unknown
}
impl FromStr for ChunkType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "IHDR" => Ok(ChunkType::IHDR),
            "PLTE" => Ok(ChunkType::PLTE),
            "IDAT" => Ok(ChunkType::IDAT),
            "IEND" => Ok(ChunkType::IEND),
            "tEXt" => Ok(ChunkType::tEXt),
            "zTXt" => Ok(ChunkType::zTXt),
            "iTXt" => Ok(ChunkType::iTXt),
            "tIME" => Ok(ChunkType::tIME),
            "pHYs" => Ok(ChunkType::pHYs),
            "gAMA" => Ok(ChunkType::gAMA),
            "cHRM" => Ok(ChunkType::cHRM),
            "sRGB" => Ok(ChunkType::sRGB),
            "iCCP" => Ok(ChunkType::iCCP),
            "sBIT" => Ok(ChunkType::sBIT),
            "bKGD" => Ok(ChunkType::bKGD),
            "hIST" => Ok(ChunkType::hIST),
            "tRNS" => Ok(ChunkType::tRNS),
            "sPLT" => Ok(ChunkType::sPLT),
            "eXIF" => Ok(ChunkType::eXIf),
            _ => Ok(ChunkType::Unknown)
        }
    }
}
impl Display for ChunkType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkType::IHDR => write!(f, "IHDR"),
            ChunkType::PLTE => write!(f, "PLTE"),
            ChunkType::IDAT => write!(f, "IDAT"),
            ChunkType::IEND => write!(f, "IEND"),
            ChunkType::tEXt => write!(f, "tEXt"),
            ChunkType::zTXt => write!(f, "zTXt"),
            ChunkType::iTXt => write!(f, "iTXt"),
            ChunkType::tIME => write!(f, "tIME"),
            ChunkType::pHYs => write!(f, "pHYs"),
            ChunkType::gAMA => write!(f, "gAMA"),
            ChunkType::cHRM => write!(f, "cHRM"),
            ChunkType::sRGB => write!(f, "sRGB"),
            ChunkType::iCCP => write!(f, "iCCP"),
            ChunkType::sBIT => write!(f, "sBIT"),
            ChunkType::bKGD => write!(f, "bKGD"),
            ChunkType::hIST => write!(f, "hIST"),
            ChunkType::tRNS => write!(f, "tRNS"),
            ChunkType::sPLT => write!(f, "sPLT"),
            ChunkType::eXIf => write!(f, "eXIf"),
            ChunkType::Unknown => write!(f, "Unknown"),
        }
    }
}
struct Chunk {
    chunk_length: usize,
    chunk_type: ChunkType,
    chunk_data: Vec<u8>,
    chunk_crc: u32,
    chunk_offset: usize,
}
impl Chunk {
    fn new(chunk_length: usize, chunk_type: ChunkType, chunk_data: Vec<u8>, chunk_crc: u32, chunk_offset: usize) -> Chunk {
        Chunk {
            chunk_length,
            chunk_type,
            chunk_data,
            chunk_crc,
            chunk_offset,
        }
    }
    fn check_crc(&self) -> bool {
        if self.chunk_crc == self.get_crc() {true} else {false}
    }
    fn get_crc(&self) -> u32 {
        let mut crc: u32 = 0xffffffff;
        let chunk_type_string = format!("{}", self.chunk_type);
        let chunk_type = chunk_type_string.as_bytes();
        let mut crc_data: Vec<u8> = Vec::with_capacity(chunk_type.len() + self.chunk_data.len());
        crc_data.extend_from_slice(chunk_type);
        crc_data.extend_from_slice(&self.chunk_data);
        for b in crc_data {
            crc ^= b as u32;
            for _ in 0..8 {
                crc = (crc >> 1) ^ (if crc & 1 == 1 { 0xEDB88320 } else { 0 });
            }
        }
        crc ^ 0xffffffffu32
    }
}
impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let crc_check: bool = self.check_crc();
        writeln!(f, "chunk_type: {}, chunk_length: {}, chunk_offset: 0x{:08x}, chunk_crc: 0x{:08x}, real_crc: 0x{:08x}, crc_check: {}", self.chunk_type, self.chunk_length, self.chunk_offset, self.chunk_crc, self.get_crc(), crc_check)
    }
}
struct Png {
    chunks: Vec<Chunk>,
}
impl Png {
    fn new(file_path: &str) -> Png {
        let mut file = File::open(file_path).unwrap();
        let mut buffer = vec![0u8; file.metadata().unwrap().len() as usize];
        file.read(&mut buffer).unwrap();
        let mut offset = 8usize;
        let mut chunks: Vec<Chunk> = Vec::new();
        while offset < file.metadata().unwrap().len() as usize {
            let chunk_length = u32::from_be_bytes(buffer[offset..offset + 4].try_into().unwrap()) as usize;
            offset += 4;
            let chunk_type = String::from_utf8(buffer[offset..offset + 4].try_into().unwrap()).unwrap();
            offset += 4;
            let chunk_data = buffer[offset..offset + chunk_length].to_vec();
            offset += chunk_length;
            let chunk_crc = u32::from_be_bytes(buffer[offset..offset + 4].try_into().unwrap());
            offset += 4;
            let chunk_offset = offset;
            chunks.push(Chunk::new(chunk_length, ChunkType::from_str(&chunk_type).unwrap(), chunk_data, chunk_crc, chunk_offset));
        }
        Png {
            chunks
        }
    }
}
impl Display for Png {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for chunk in self.chunks.iter() {
            writeln!(f, "{}", chunk)?;
        }
        Ok(())
    }
}

fn main() {
    let args = Args::parse();
    let png = Png::new(&args.input);
    println!("{}", png);
}
