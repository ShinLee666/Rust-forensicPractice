use std::fmt;
use std::fmt::{Formatter};
use clap::Parser;
use std::fs::File;
use std::io::{Read};

#[derive(Parser)]
struct Args {
    #[arg(short = 'i', long = "input", value_name = "FILE")]
    input: String,
}
struct BitMapFileHeader {
    bf_type: u16,
    bf_size: u32,
    bf_reserved1: u16,
    bf_reserved2: u16,
    bf_off_bits: u32,
}
struct BitMapInfoHeader {
    bi_size: u32,
    bi_width: i32,
    bi_height: i32,
    bi_planes: u16,
    bi_bit_count: u16,
    bi_compression: u32,
    bi_size_image: u32,
    bi_x_pels_per_meter: i32,
    bi_y_pels_per_meter: i32,
    bi_clr_used: u32,
    bi_clr_important: u32,
}
struct RgbQuad {
    b: u8,
    g: u8,
    r: u8,
    a: u8,
}
struct BMP {
    file_header: BitMapFileHeader,
    info_header: BitMapInfoHeader,
    rgb_quads: Vec<RgbQuad>,
}
impl BMP {
    fn new(file_path: &str) -> BMP {
        let mut file = File::open(file_path).unwrap();
        let mut buffer = vec![0u8;file.metadata().unwrap().len() as usize];
        file.read(&mut buffer).unwrap();
        let (file_header, info_header, rgb_quads) = Self::get_structure(&buffer);
        BMP{
            file_header,
            info_header,
            rgb_quads,
        }
    }
    fn get_structure(buffer: &[u8]) -> (BitMapFileHeader, BitMapInfoHeader, Vec<RgbQuad>) {
        let bf_type = u16::from_le_bytes(buffer[0..2].try_into().unwrap());
        let bf_size = u32::from_le_bytes(buffer[2..6].try_into().unwrap());
        let bf_reserved1 = u16::from_le_bytes(buffer[6..8].try_into().unwrap());
        let bf_reserved2 = u16::from_le_bytes(buffer[8..10].try_into().unwrap());
        let bf_off_bits = u32::from_le_bytes(buffer[10..14].try_into().unwrap());
        let file_header = BitMapFileHeader {
            bf_type,
            bf_size,
            bf_reserved1,
            bf_reserved2,
            bf_off_bits,
        };
        let bi_size = u32::from_le_bytes(buffer[14..18].try_into().unwrap());
        let bi_width = i32::from_le_bytes(buffer[18..22].try_into().unwrap());
        let bi_height = i32::from_le_bytes(buffer[22..26].try_into().unwrap());
        let bi_planes = u16::from_le_bytes(buffer[26..28].try_into().unwrap());
        let bi_bit_count = u16::from_le_bytes(buffer[28..30].try_into().unwrap());
        let bi_compression = u32::from_le_bytes(buffer[30..34].try_into().unwrap());
        let bi_size_image = u32::from_le_bytes(buffer[34..38].try_into().unwrap());
        let bi_x_pels_per_meter = i32::from_le_bytes(buffer[38..42].try_into().unwrap());
        let bi_y_pels_per_meter = i32::from_le_bytes(buffer[42..46].try_into().unwrap());
        let bi_clr_used = u32::from_le_bytes(buffer[46..50].try_into().unwrap());
        let bi_clr_important = u32::from_le_bytes(buffer[50..54].try_into().unwrap());
        let info_header = BitMapInfoHeader {
            bi_size,
            bi_width,
            bi_height,
            bi_planes,
            bi_bit_count,
            bi_compression,
            bi_size_image,
            bi_x_pels_per_meter,
            bi_y_pels_per_meter,
            bi_clr_used,
            bi_clr_important,
        };
        let clr_data = &buffer[54..bf_off_bits as usize];
        let mut rgb_quads = Vec::new();
        for rgb_data in clr_data.chunks(4) {
            let rgb_quad = RgbQuad {
                r: rgb_data[2],
                g: rgb_data[1],
                b: rgb_data[0],
                a: rgb_data[3],
            };
            rgb_quads.push(rgb_quad);
        }
        (file_header, info_header, rgb_quads)
    }
}
impl fmt::Display for BMP {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "******file_header:******\n\
        bf_type:{:x}\n\
        bf_size:{}\n\
        bf_reserved1:{}\n\
        bf_reserved2:{}\n\
        bf_off_bits:{}\n\
        ******info_header:******\n\
        bi_size:{}\n\
        bi_width:{}\n\
        bi_height:{}\n\
        bi_planes:{}\n\
        bi_bit_count:{}\n\
        bi_compression:{}\n\
        bi_size_image:{}\n\
        bi_x_pels_per_meter:{}\n\
        bi_y_pels_per_meter:{}\n\
        bi_clr_used:{}\n\
        bi_clr_important:{}\n",
               self.file_header.bf_type,
               self.file_header.bf_size,
               self.file_header.bf_reserved1,
               self.file_header.bf_reserved2,
               self.file_header.bf_off_bits,
               self.info_header.bi_size,
               self.info_header.bi_width,
               self.info_header.bi_height,
               self.info_header.bi_planes,
               self.info_header.bi_bit_count,
               self.info_header.bi_compression,
               self.info_header.bi_size_image,
               self.info_header.bi_x_pels_per_meter,
               self.info_header.bi_y_pels_per_meter,
               self.info_header.bi_clr_used,
               self.info_header.bi_clr_important)?;
        write!(f, "******rgb_quads:******\n")?;
        for quad in &self.rgb_quads {
            writeln!(f, "clr_data: (r:{}, g:{}, b:{}, a:{})", quad.r, quad.g, quad.b, quad.a)?;
        }
        Ok(())

    }
}

fn main() {
    let args = Args::parse();
    let bmp = BMP::new(&args.input);
    println!("{}", bmp);
}
