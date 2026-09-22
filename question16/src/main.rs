use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use::clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short = 'i', long = "input", value_name = "FILE")]
    input: String,
}

struct Field<T> {
    value: T,
    size: usize,
    offset: usize,
}
impl<T: Display> Display for Field<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "offset: 0x{:08x}, size: {}, value: {}", self.offset, self.size, self.value)
    }
}

struct LocalFileHeader {
    offset: usize,
    fr_version: Field<u16>,
    fr_flags: Field<u16>,
    fr_compression: Field<u16>,
    fr_file_time: Field<u16>,
    fr_file_date: Field<u16>,
    fr_crc: Field<u32>,
    fr_compressed_size: Field<u32>,
    fr_uncompressed_size: Field<u32>,
    fr_file_name_length: Field<u16>,
    fr_extra_field_length: Field<u16>,
    fr_file_name: Field<String>,
}
impl LocalFileHeader {
    const SIGNATURE: [u8; 4] = [0x50, 0x4b, 0x03, 0x04];
}
impl Display for LocalFileHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "***LocalFileHeader***")?;
        writeln!(f, "signature: {:?}", Self::SIGNATURE)?;
        writeln!(f, "offset: 0x{:08x}", self.offset)?;
        write!(f, "fr_version: {}", self.fr_version)?;
        write!(f, "fr_flags: {}", self.fr_flags)?;
        write!(f, "fr_compression: {}", self.fr_compression)?;
        write!(f, "fr_file_time: {}", self.fr_file_time)?;
        write!(f, "fr_file_date: {}", self.fr_file_date)?;
        write!(f, "fr_crc: {}", self.fr_crc)?;
        write!(f, "fr_compressed_size: {}", self.fr_compressed_size)?;
        write!(f, "fr_uncompressed_size: {}", self.fr_uncompressed_size)?;
        write!(f, "fr_file_name_length: {}", self.fr_file_name_length)?;
        write!(f, "fr_extra_field_length: {}", self.fr_extra_field_length)?;
        write!(f, "fr_file_name: {}", self.fr_file_name)?;
        Ok(())
    }
}

struct CentralDirectoryEntry {
    offset: usize,
    de_version_made_by: Field<u16>,
    de_version_to_extract: Field<u16>,
    de_flags: Field<u16>,
    de_compression: Field<u16>,
    de_file_time: Field<u16>,
    de_file_date: Field<u16>,
    de_crc: Field<u32>,
    de_compressed_size: Field<u32>,
    de_uncompressed_size: Field<u32>,
    de_file_name_length: Field<u16>,
    de_extra_field_length: Field<u16>,
    de_file_comment_length: Field<u16>,
    de_disk_number_start: Field<u16>,
    de_internal_attributes: Field<u16>,
    de_external_attributes: Field<u32>,
    de_header_offset: Field<u32>,
    de_file_name: Field<String>,
}
impl CentralDirectoryEntry {
    const SIGNATURE: [u8; 4] = [0x50, 0x4b, 0x01, 0x02];
}
impl Display for CentralDirectoryEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "***CentralDirectoryEntry***")?;
        writeln!(f, "signature: {:?}", Self::SIGNATURE)?;
        writeln!(f, "offset: 0x{:08x}", self.offset)?;
        write!(f, "de_version_made_by: {}", self.de_version_made_by)?;
        write!(f, "de_version_to_extract: {}", self.de_version_to_extract)?;
        write!(f, "de_flags: {}", self.de_flags)?;
        write!(f, "de_compression: {}", self.de_compression)?;
        write!(f, "de_file_time: {}", self.de_file_time)?;
        write!(f, "de_file_date: {}", self.de_file_date)?;
        write!(f, "de_crc: {}", self.de_crc)?;
        write!(f, "de_compressed_size: {}", self.de_compressed_size)?;
        write!(f, "de_uncompressed_size: {}", self.de_uncompressed_size)?;
        write!(f, "de_file_name_length: {}", self.de_file_name_length)?;
        write!(f, "de_extra_field_length: {}", self.de_extra_field_length)?;
        write!(f, "de_file_comment_length: {}", self.de_file_comment_length)?;
        write!(f, "de_disk_number_start: {}", self.de_disk_number_start)?;
        write!(f, "de_internal_attributes: {}", self.de_internal_attributes)?;
        write!(f, "de_external_attributes: {}", self.de_external_attributes)?;
        write!(f, "de_header_offset: {}", self.de_header_offset)?;
        write!(f, "de_file_name: {}", self.de_file_name)?;
        Ok(())
    }
}

struct EndOfCentralDirectory {
    offset: usize,
    el_disk_number: Field<u16>,
    el_start_disk_number: Field<u16>,
    el_entries_on_disk: Field<u16>,
    el_entries_in_directory: Field<u16>,
    el_directory_size: Field<u32>,
    el_directory_offset: Field<u32>,
    el_comment_length: Field<u16>,
}
impl EndOfCentralDirectory {
    const SIGNATURE: [u8; 4] = [0x50, 0x4b, 0x05, 0x06];
}
impl Display for EndOfCentralDirectory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "***EndOfCentralDirectory***")?;
        writeln!(f, "signature: {:?}", Self::SIGNATURE)?;
        writeln!(f, "offset: 0x{:08x}", self.offset)?;
        write!(f, "el_disk_number: {}", self.el_disk_number)?;
        write!(f, "el_start_disk_number: {}", self.el_start_disk_number)?;
        write!(f, "el_entries_on_disk: {}", self.el_entries_on_disk)?;
        write!(f, "el_entries_in_directory: {}", self.el_entries_in_directory)?;
        write!(f, "el_directory_size: {}", self.el_directory_size)?;
        write!(f, "el_directory_offset: {}", self.el_directory_offset)?;
        write!(f, "el_comment_length: {}", self.el_comment_length)?;
        Ok(())
    }
}

struct DataDescriptor {
    offset: usize,
    dd_crc: Field<u32>,
    dd_compressed_size: Field<u32>,
    dd_uncompressed_size: Field<u32>,
}
impl DataDescriptor {
    const SIGNATURE: [u8; 4] = [0x50, 0x4b, 0x07, 0x08];
}
impl Display for DataDescriptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "***DataDescriptor***")?;
        writeln!(f, "signature: {:?}", Self::SIGNATURE)?;
        writeln!(f, "offset: 0x{:08x}", self.offset)?;
        write!(f, "dd_crc: {}", self.dd_crc)?;
        write!(f, "dd_compressed_size: {}", self.dd_compressed_size)?;
        write!(f, "dd_uncompressed_size: {}", self.dd_uncompressed_size)?;
        Ok(())
    }
}

struct ZipDetector{
    local_file_headers: Vec<LocalFileHeader>,
    central_directory_entries: Vec<CentralDirectoryEntry>,
    data_descriptor: Option<DataDescriptor>,
    end_of_central_directory: EndOfCentralDirectory,
}
impl ZipDetector {
    fn new(file_path: &str) -> ZipDetector {
        let mut file = File::open(file_path).unwrap();
        let mut buffer = vec![0u8;file.metadata().unwrap().len() as usize];
        file.read(&mut buffer).unwrap();
        let mut local_file_headers:Vec<LocalFileHeader> = Vec::new();
        let mut next_offset: usize = 0;
        loop {
            let Some(mut pos) = buffer[next_offset..].windows(4).position(|sig| sig == LocalFileHeader::SIGNATURE) else { break };
            pos = pos + next_offset;
            let local_file_header:LocalFileHeader = LocalFileHeader {
                offset: pos,
                fr_version: Field{value: u16::from_le_bytes(buffer[pos+4..pos+6].try_into().unwrap()), size: 2, offset: pos + 4},
                fr_flags: Field{value: u16::from_le_bytes(buffer[pos+6..pos+8].try_into().unwrap()), size: 2, offset: pos + 6},
                fr_compression: Field{value: u16::from_le_bytes(buffer[pos+8..pos+10].try_into().unwrap()), size: 2, offset: pos + 8},
                fr_file_time: Field{value: u16::from_le_bytes(buffer[pos+10..pos+12].try_into().unwrap()), size: 2, offset: pos + 10},
                fr_file_date: Field{value: u16::from_le_bytes(buffer[pos+12..pos+14].try_into().unwrap()), size: 2, offset: pos + 12},
                fr_crc: Field{value: u32::from_le_bytes(buffer[pos+14..pos+18].try_into().unwrap()), size: 4, offset: pos + 14},
                fr_compressed_size: Field{value: u32::from_le_bytes(buffer[pos+18..pos+22].try_into().unwrap()), size: 4, offset: pos + 18},
                fr_uncompressed_size: Field{value: u32::from_le_bytes(buffer[pos+22..pos+26].try_into().unwrap()), size: 4, offset: pos + 22},
                fr_file_name_length: Field{value: u16::from_le_bytes(buffer[pos+26..pos+28].try_into().unwrap()), size: 2, offset: pos + 26},
                fr_extra_field_length: Field{value: u16::from_le_bytes(buffer[pos+28..pos+30].try_into().unwrap()), size: 2, offset: pos + 28},
                fr_file_name: Field{value: String::from_utf8(buffer[pos+30..pos+30+u16::from_le_bytes(buffer[pos+26..pos+28].try_into().unwrap()) as usize].try_into().unwrap()).unwrap(), size: u16::from_le_bytes(buffer[pos+26..pos+28].try_into().unwrap()) as usize, offset: pos + 30},
            };
            next_offset = pos + 30 + local_file_header.fr_file_name_length.value as usize + local_file_header.fr_compressed_size.value as usize;
            local_file_headers.push(local_file_header);
        }
        let mut central_directory_entries:Vec<CentralDirectoryEntry> = Vec::new();
        next_offset = 0;
        loop {
            let Some(mut pos) = buffer[next_offset..].windows(4).position(|sig| sig == CentralDirectoryEntry::SIGNATURE) else { break};
            pos = pos + next_offset;
            let central_directory_entry: CentralDirectoryEntry = CentralDirectoryEntry {
                offset: pos,
                de_version_made_by: Field{value: u16::from_le_bytes(buffer[pos+4..pos+6].try_into().unwrap()), size: 2, offset: pos + 4},
                de_version_to_extract: Field{value: u16::from_le_bytes(buffer[pos+6..pos+8].try_into().unwrap()), size: 2, offset: pos + 6},
                de_flags: Field{value: u16::from_le_bytes(buffer[pos+8..pos+10].try_into().unwrap()), size: 2, offset: pos + 8},
                de_compression: Field{value: u16::from_le_bytes(buffer[pos+10..pos+12].try_into().unwrap()), size: 2, offset: pos + 10},
                de_file_time: Field{value: u16::from_le_bytes(buffer[pos+12..pos+14].try_into().unwrap()), size: 2, offset: pos + 12},
                de_file_date: Field{value: u16::from_le_bytes(buffer[pos+14..pos+16].try_into().unwrap()), size: 2, offset: pos + 14},
                de_crc: Field{value: u32::from_le_bytes(buffer[pos+16..pos+20].try_into().unwrap()), size: 4, offset: pos + 16},
                de_compressed_size: Field{value: u32::from_le_bytes(buffer[pos+20..pos+24].try_into().unwrap()), size: 4, offset: pos + 20},
                de_uncompressed_size: Field{value: u32::from_le_bytes(buffer[pos+24..pos+28].try_into().unwrap()), size: 4, offset: pos + 24},
                de_file_name_length: Field{value: u16::from_le_bytes(buffer[pos+28..pos+30].try_into().unwrap()), size: 2, offset: pos + 28},
                de_extra_field_length: Field{value: u16::from_le_bytes(buffer[pos+30..pos+32].try_into().unwrap()), size: 2, offset: pos + 30},
                de_file_comment_length: Field{value: u16::from_le_bytes(buffer[pos+32..pos+34].try_into().unwrap()), size: 2, offset: pos + 32},
                de_disk_number_start: Field{value: u16::from_le_bytes(buffer[pos+34..pos+36].try_into().unwrap()), size: 2, offset: pos + 34},
                de_internal_attributes: Field{value: u16::from_le_bytes(buffer[pos+36..pos+38].try_into().unwrap()), size: 2, offset: pos + 36},
                de_external_attributes: Field{value: u32::from_le_bytes(buffer[pos+38..pos+42].try_into().unwrap()), size: 4, offset: pos + 38},
                de_header_offset: Field{value: u32::from_le_bytes(buffer[pos+42..pos+46].try_into().unwrap()), size: 4, offset: pos + 42},
                de_file_name: Field{value: String::from_utf8(buffer[pos+46..pos+46+u16::from_le_bytes(buffer[pos+28..pos+30].try_into().unwrap()) as usize].try_into().unwrap()).unwrap(), size: u16::from_le_bytes(buffer[pos+28..pos+30].try_into().unwrap()) as usize, offset: pos + 46},
            };
            next_offset = pos + 46 + central_directory_entry.de_file_name_length.value as usize;
            central_directory_entries.push(central_directory_entry);
        }
        let data_descriptor: Option<DataDescriptor> = if let Some(pos) = buffer.windows(4).position(|sig| sig == DataDescriptor::SIGNATURE) {
            Some(DataDescriptor{
                offset: pos,
                dd_crc: Field{value: u32::from_le_bytes(buffer[pos+4..pos+8].try_into().unwrap()), size: 4, offset: pos + 4},
                dd_compressed_size: Field{value: u32::from_le_bytes(buffer[pos+12..pos+16].try_into().unwrap()), size: 4, offset: pos + 12},
                dd_uncompressed_size: Field{value: u32::from_le_bytes(buffer[pos+20..pos+24].try_into().unwrap()), size: 4, offset: pos + 20},
            })
        } else {
            None
        };
        let pos = buffer.windows(4).position(|sig| sig == EndOfCentralDirectory::SIGNATURE).unwrap_or(0);
        let end_of_central_directory = EndOfCentralDirectory {
            offset: pos,
            el_disk_number: Field{value: u16::from_le_bytes(buffer[pos+4..pos+6].try_into().unwrap()), size: 2, offset: pos + 4},
            el_start_disk_number: Field{value: u16::from_le_bytes(buffer[pos+6..pos+8].try_into().unwrap()), size: 2, offset: pos + 6},
            el_entries_on_disk: Field{value: u16::from_le_bytes(buffer[pos+8..pos+10].try_into().unwrap()), size: 2, offset: pos + 8},
            el_entries_in_directory: Field{value: u16::from_le_bytes(buffer[pos+10..pos+12].try_into().unwrap()), size: 2, offset: pos + 10},
            el_directory_size: Field{value: u32::from_le_bytes(buffer[pos+12..pos+16].try_into().unwrap()), size: 4, offset: pos + 12},
            el_directory_offset: Field{value: u32::from_le_bytes(buffer[pos+16..pos+20].try_into().unwrap()), size: 4, offset: pos + 16},
            el_comment_length: Field{value: u16::from_le_bytes(buffer[pos+20..pos+22].try_into().unwrap()), size: 2, offset: pos + 20},
        };
        ZipDetector {
            local_file_headers,
            central_directory_entries,
            data_descriptor,
            end_of_central_directory,
        }
    }
}
impl Display for ZipDetector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "***ZipDetector Result***")?;
        for local_file_header in &self.local_file_headers {
            writeln!(f, "{}", local_file_header)?;
        }
        for central_directory_entry in &self.central_directory_entries {
            writeln!(f, "{}", central_directory_entry)?;
        }
        match &self.data_descriptor {
            Some(data_descriptor) => {
                writeln!(f, "{}", data_descriptor)?;
            }
            None => {
                writeln!(f, "No data descriptor found")?;
            }
        }
        writeln!(f, "{}", &self.end_of_central_directory)?;
        Ok(())
    }
}

fn main() {
    let args = Args::parse();
    let zip = ZipDetector::new(&args.input);
    println!("{}", zip);
}
