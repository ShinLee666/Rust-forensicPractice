use clap::Parser;
use std::fs::File;
use std::io::Read;
use std::fmt;
use std::fmt::Formatter;

#[derive(Parser)]
#[command(
name = "file_type_detect v1",
version = "0.1.0",
about = "简易文件类型检测工具,读取文件头 8 字节，识别 PNG/JPG/GIF/PDF/ZIP/ELF 并输出类型"
)]
struct Args {
    /// 指定的文件
    #[arg(short = 'f', long = "file", value_name = "FILE", help = "输入的文件路径")]
    file_path: String,
}
enum MagicType {
    PNG,
    JPG,
    GIF,
    PDF,
    ZIP,
    ELF,
    Unknown,
}
impl MagicType {
    const PNG_MAGIC: [u8; 8] = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];
    const JPG_MAGIC: [u8; 3] = [0xff, 0xd8, 0xff];
    const GIF_MAGIC: [u8; 3] = [0x47, 0x49, 0x46];
    const PDF_MAGIC: [u8; 5] = [0x25, 0x50, 0x44, 0x46, 0x2D];
    const ZIP_MAGIC: [u8; 2] = [0x50, 0x4b];
    const ELF_MAGIC: [u8; 4] = [0x7f, 0x45, 0x4c, 0x46];
    fn matches(header: &[u8]) -> Self {
        if header.starts_with(&Self::PNG_MAGIC) {
            Self::PNG
        } else if header.starts_with(&Self::JPG_MAGIC) {
            Self::JPG
        } else if header.starts_with(&Self::GIF_MAGIC) {
            Self::GIF
        } else if header.starts_with(&Self::PDF_MAGIC) {
            Self::PDF
        } else if header.starts_with(&Self::ZIP_MAGIC) {
            Self::ZIP
        } else if header.starts_with(&Self::ELF_MAGIC) {
            Self::ELF
        } else {
            Self::Unknown
        }
    }
}
impl fmt::Display for MagicType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PNG => "PNG",
            Self::JPG => "JPG",
            Self::GIF => "GIF",
            Self::PDF => "PDF",
            Self::ZIP => "ZIP",
            Self::ELF => "ELF",
            Self::Unknown => "Unknown",
        };
        write!(f, "{}", s)
    }
}
fn main() {
    let args = Args::parse();
    let file_path = args.file_path;
    let mut file = File::open(&file_path).expect("File not found");
    let mut buffer = vec![0u8; 8];
    let read_count = file.read(&mut buffer).expect("Failed to read");
    buffer.truncate(read_count);
    println!("file: {}", file_path);
    println!("header: {:02x?}", buffer);
    println!("type: {}", MagicType::matches(&buffer));
}
