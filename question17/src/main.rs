use clap::Parser;
use std::fs::File;
use std::io::{self, Read};

#[derive(Parser)]
struct Args {
    #[arg(short = 'i', long = "input", value_name = "FILE")]
    input_file: String,
}

const MAGIC_US: u32 = 0xa1b2c3d4; // 微秒级时间戳
const MAGIC_NS: u32 = 0xa1b23c4d; // 纳秒级时间戳

#[derive(Copy, Clone)]
enum Endian {
    Little,
    Big,
}
impl Endian {
    fn read_u16(&self, buf: &[u8]) -> u16 {
        match self {
            Endian::Little => u16::from_le_bytes([buf[0], buf[1]]),
            Endian::Big => u16::from_be_bytes([buf[0], buf[1]]),
        }
    }
    fn read_u32(&self, buf: &[u8]) -> u32 {
        match self {
            Endian::Little => u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]),
            Endian::Big => u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]),
        }
    }
}

struct GlobalHeader {
    endian: Endian,
    nanosecond: bool,
    version_major: u16,
    version_minor: u16,
    thiszone: u32,
    sigfigs: u32,
    snaplen: u32,
    network: u32,
}

fn read_global_header(f: &mut File) -> io::Result<GlobalHeader> {
    let mut buf = [0u8; 24];
    f.read_exact(&mut buf)?;
    let be_magic = u32::from_be_bytes([buf[0], buf[1], buf[2], buf[3]]);
    let (endian, nanosecond) = match be_magic {
        MAGIC_US => (Endian::Big, false),
        MAGIC_NS => (Endian::Big, true),
        _ => {
            let le_magic = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]);
            match le_magic {
                MAGIC_US => (Endian::Little, false),
                MAGIC_NS => (Endian::Little, true),
                other => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!("不是有效的 pcap 文件（魔数 0x{other:08x}）")
                    ))
                }
            }
        }
    };
    Ok(GlobalHeader {
        endian,
        nanosecond,
        version_major: endian.read_u16(&buf[4..6]),
        version_minor: endian.read_u16(&buf[6..8]),
        thiszone: endian.read_u32(&buf[8..12]),
        sigfigs: endian.read_u32(&buf[12..16]),
        snaplen: endian.read_u32(&buf[16..20]),
        network: endian.read_u32(&buf[20..24]),
    })
}

fn format_ts(sec: u32, frac: u32, nanosecond: bool) -> String {
    let days = sec / 86400;
    let secs_of_day = sec % 86400;
    let (hour, minute, second) = (secs_of_day / 3600, (secs_of_day % 3600) / 60, secs_of_day % 60);
    let mut year = 1970i64;
    let mut day = days as i64;
    loop {
        let y1 = if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {366} else { 365 };
        if day < y1 {
            break;
        }
        day -= y1;
        year += 1;
    }
    const MONTH_DAYS: [i64; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let mut month = 0usize;
    while month < 12 {
        let md = if month == 1 && leap { 29 } else { MONTH_DAYS[month] };
        if day < md {
            break;
        }
        day -= md;
        month += 1;
    }
    let frac_str = if nanosecond {
        format!("{frac:09}")
    } else {
        format!("{frac:06}")
    };
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{} UTC",
        year,
        month + 1,
        day + 1,
        hour,
        minute,
        second,
        frac_str
    )
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let mut file = File::open(&args.input_file).map_err(|e| io::Error::new(e.kind(), format!("无法打开 {}: {e}", args.input_file)))?;
    let gh = read_global_header(&mut file)?;
    println!("===== PCAP 全局头 =====");
    println!("字节序      : {}", match gh.endian {Endian::Little => "小端 (Little-endian)", Endian::Big => "大端 (Big-endian)"});
    println!("时间精度    : {}", if gh.nanosecond {"纳秒"} else {"微秒"});
    println!("版本        : {}.{}", gh.version_major, gh.version_minor);
    println!("时区修正    : {}", gh.thiszone);
    println!("时间戳标志  : {}", gh.sigfigs);
    println!("最大捕获长度: {}", gh.snaplen);
    println!("链路层类型  : {} (1 = Ethernet)", gh.network);
    println!();
    println!("===== 数据包列表 =====");
    println!("{:<6} {:<30} {:<12} {:<12}", "序号", "时间戳", "捕获长度", "原始长度");
    let mut seq = 1usize;
    loop {
        let mut hdr = [0u8; 16];
        match file.read_exact(&mut hdr) {
            Ok(()) => {},
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(e) => return Err(e),
        }
        let ts_sec = gh.endian.read_u32(&hdr[0..4]);
        let ts_frac = gh.endian.read_u32(&hdr[4..8]);
        let incl_len = gh.endian.read_u32(&hdr[8..12]);
        let orig_len = gh.endian.read_u32(&hdr[12..16]);
        println!(
            "{:<6} {:<30} {:<12} {:<12}",
            seq,
            format_ts(ts_sec, ts_frac, gh.nanosecond),
            incl_len,
            orig_len
        );
        io::copy(&mut file.by_ref().take(incl_len as u64), &mut io::sink())?;
        seq += 1;
    }
    println!();
    println!("共 {} 个数据包", seq - 1);
    Ok(())
}
