//! crypto-lab：经典密码算法手写实现学习项目
//!
//! 用法：
//!   cargo run -- test                          # 运行全部标准测试向量
//!   cargo run -- enc  <算法> <十六进制密钥> <十六进制数据>
//!   cargo run -- dec  <算法> <十六进制密钥> <十六进制数据>
//!
//! 算法名：tea | xtea | xxtea | rc4 | des | des3 | aes128
//!
//! 长度要求：
//!   tea/xtea  : 密钥 16 字节，数据 8 字节（单分组）
//!   xxtea     : 密钥 16 字节，数据 8 的倍数且 >= 8 字节
//!   rc4       : 密钥 1..=256 字节，数据任意长度
//!   des       : 密钥 8 字节，数据 8 的非零倍数
//!   des3      : 密钥 24 字节，数据 8 的非零倍数
//!   aes128    : 密钥 16 字节，数据 16 的非零倍数

mod aes;
mod des;
mod rc4;
mod tea;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "crypto-lab", version, about = "手写经典密码算法学习工具")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 运行全部标准测试向量（KAT）
    Test,
    /// 加密：crypto-lab enc <算法> <密钥hex> <数据hex>
    Enc {
        #[arg(value_enum)]
        algo: Algo,
        /// 十六进制密钥
        key: String,
        /// 十六进制数据
        data: String,
    },
    /// 解密：crypto-lab dec <算法> <密钥hex> <数据hex>
    Dec {
        #[arg(value_enum)]
        algo: Algo,
        /// 十六进制密钥
        key: String,
        /// 十六进制数据
        data: String,
    },
}

#[derive(Clone, ValueEnum)]
enum Algo {
    Tea,
    Xtea,
    Xxtea,
    Rc4,
    Des,
    Des3,
    Aes128,
}

// ---------------------------------------------------------------------------
// hex 工具
// ---------------------------------------------------------------------------

fn hex_to_bytes(s: &str) -> Result<Vec<u8>, String> {
    let s: String = s.chars().filter(|c| !c.is_whitespace()).collect();
    if s.len() % 2 != 0 {
        return Err(format!("hex 字符串长度必须为偶数：{}", s));
    }
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).map_err(|e| format!("hex 解析失败 {}: {}", s, e)))
        .collect()
}

fn to_hex(data: &[u8]) -> String {
    data.iter().map(|b| format!("{:02x}", b)).collect()
}

// ---------------------------------------------------------------------------
// 加解密调度
// ---------------------------------------------------------------------------

fn encrypt(algo: &Algo, key: &[u8], data: &[u8]) -> Result<Vec<u8>, String> {
    match algo {
        Algo::Tea => {
            let k = tea::bytes_to_key(expect_len(key, 16, "TEA 密钥")?);
            Ok(tea::block_to_bytes(&tea::tea_encrypt(
                &tea::bytes_to_block(expect_len(data, 8, "TEA 数据")?),
                &k,
            ))
                .to_vec())
        }
        Algo::Xtea => {
            let k = tea::bytes_to_key(expect_len(key, 16, "XTEA 密钥")?);
            Ok(tea::block_to_bytes(&tea::xtea_encrypt(
                &tea::bytes_to_block(expect_len(data, 8, "XTEA 数据")?),
                &k,
            ))
                .to_vec())
        }
        Algo::Xxtea => {
            let k = tea::bytes_to_key(expect_len(key, 16, "XXTEA 密钥")?);
            if data.len() % 4 != 0 || data.len() < 8 {
                return Err("XXTEA 数据长度必须是 4 的倍数且 >= 8 字节".into());
            }
            let mut words: Vec<u32> = data
                .chunks(4)
                .map(|c| u32::from_be_bytes(c.try_into().unwrap()))
                .collect();
            tea::xxtea_encrypt(&mut words, &k);
            Ok(words.iter().flat_map(|w| w.to_be_bytes()).collect())
        }
        Algo::Rc4 => Ok(rc4::rc4(key, data)),
        Algo::Des => {
            let k: [u8; 8] = expect_len(key, 8, "DES 密钥")?.try_into().unwrap();
            Ok(des::des_encrypt_ecb(data, &k))
        }
        Algo::Des3 => {
            let k: [u8; 24] = expect_len(key, 24, "3DES 密钥")?.try_into().unwrap();
            Ok(des::tdes_encrypt_ecb(data, &k))
        }
        Algo::Aes128 => {
            let k: [u8; 16] = expect_len(key, 16, "AES-128 密钥")?.try_into().unwrap();
            Ok(aes::aes128_encrypt_ecb(data, &k))
        }
    }
}

fn decrypt(algo: &Algo, key: &[u8], data: &[u8]) -> Result<Vec<u8>, String> {
    match algo {
        Algo::Tea => {
            let k = tea::bytes_to_key(expect_len(key, 16, "TEA 密钥")?);
            Ok(tea::block_to_bytes(&tea::tea_decrypt(
                &tea::bytes_to_block(expect_len(data, 8, "TEA 数据")?),
                &k,
            ))
                .to_vec())
        }
        Algo::Xtea => {
            let k = tea::bytes_to_key(expect_len(key, 16, "XTEA 密钥")?);
            Ok(tea::block_to_bytes(&tea::xtea_decrypt(
                &tea::bytes_to_block(expect_len(data, 8, "XTEA 数据")?),
                &k,
            ))
                .to_vec())
        }
        Algo::Xxtea => {
            let k = tea::bytes_to_key(expect_len(key, 16, "XXTEA 密钥")?);
            if data.len() % 4 != 0 || data.len() < 8 {
                return Err("XXTEA 数据长度必须是 4 的倍数且 >= 8 字节".into());
            }
            let mut words: Vec<u32> = data
                .chunks(4)
                .map(|c| u32::from_be_bytes(c.try_into().unwrap()))
                .collect();
            tea::xxtea_decrypt(&mut words, &k);
            Ok(words.iter().flat_map(|w| w.to_be_bytes()).collect())
        }
        Algo::Rc4 => Ok(rc4::rc4(key, data)),
        Algo::Des => {
            let k: [u8; 8] = expect_len(key, 8, "DES 密钥")?.try_into().unwrap();
            Ok(des::des_decrypt_ecb(data, &k))
        }
        Algo::Des3 => {
            let k: [u8; 24] = expect_len(key, 24, "3DES 密钥")?.try_into().unwrap();
            Ok(des::tdes_decrypt_ecb(data, &k))
        }
        Algo::Aes128 => {
            let k: [u8; 16] = expect_len(key, 16, "AES-128 密钥")?.try_into().unwrap();
            Ok(aes::aes128_decrypt_ecb(data, &k))
        }
    }
}

fn expect_len<'a>(b: &'a [u8], len: usize, what: &str) -> Result<&'a [u8], String> {
    if b.len() != len {
        Err(format!("{}长度必须为 {} 字节，实际 {} 字节", what, len, b.len()))
    } else {
        Ok(b)
    }
}

// ---------------------------------------------------------------------------
// 标准测试向量（与单元测试一致，便于在 main 里统一调用观察）
// ---------------------------------------------------------------------------

fn run_all_kats() -> bool {
    let mut pass = true;
    let mut check = |name: &str, ok: bool| {
        println!("[{}] {}", if ok { "PASS" } else { "FAIL" }, name);
        if !ok {
            pass = false;
        }
    };

    // ---- TEA ----
    let ct = tea::tea_encrypt(&[0, 0], &[0; 4]);
    check("TEA     key=0,pt=0 -> 41ea3a0a94baa940", ct == [0x41ea3a0a, 0x94baa940]);
    let k = tea::bytes_to_key(&hex_b("000102030405060708090a0b0c0d0e0f"));
    let pt = tea::bytes_to_block(&hex_b("0123456789abcdef"));
    let ct2 = tea::tea_encrypt(&pt, &k);
    check(
        "TEA     key=000102..0e0f,pt=012345..cdef -> 14f0c75d2bebd98d",
        tea::block_to_bytes(&ct2).to_vec() == hex_b("14f0c75d2bebd98d"),
    );

    // ---- XTEA（Bouncy Castle 向量）----
    let xvecs: [([u32; 2], [u32; 4], [u32; 2]); 4] = [
        ([0x00000000, 0x00000000], [0; 4], [0xDEE9D4D8, 0xF7131ED9]),
        ([0x01020304, 0x05060708], [0; 4], [0x065C1B89, 0x75C6A816]),
        ([0x00000000, 0x00000000], [0x01234567, 0x12345678, 0x23456789, 0x3456789A], [0x1FF9A026, 0x1AC64264]),
        ([0x01020304, 0x05060708], [0x01234567, 0x12345678, 0x23456789, 0x3456789A], [0x8C67155B, 0x2EF91EAD]),
    ];
    for (i, (p, k, c)) in xvecs.iter().enumerate() {
        check(
            &format!("XTEA    BouncyCastle 向量 #{}", i + 1),
            tea::xtea_encrypt(p, k) == *c && tea::xtea_decrypt(c, k) == *p,
        );
    }

    // ---- XXTEA ----
    let xk = [0x00010203, 0x04050607, 0x08090a0b, 0x0c0d0e0f];
    let mut w = vec![0x00112233u32, 0x44556677, 0x8899aabb, 0xccddeeff];
    let saved = w.clone();
    tea::xxtea_encrypt(&mut w, &xk);
    check(
        "XXTEA   字数组 KAT（与官方参考实现比对） -> 04aa55a8ce3989ba2572970b8dee764f",
        w == vec![0x04aa55a8, 0xce3989ba, 0x2572970b, 0x8dee764f],
    );
    tea::xxtea_decrypt(&mut w, &xk);
    check("XXTEA   解密往返", w == saved);

    // ---- RC4 ----
    check(
        "RC4     key=\"Key\", pt=\"Plaintext\" -> BBF316E8D940AF0AD3",
        rc4::rc4(b"Key", b"Plaintext") == hex_b("BBF316E8D940AF0AD3"),
    );
    check(
        "RC4     key=\"Wiki\", pt=\"pedia\" -> 1021BF0420",
        rc4::rc4(b"Wiki", b"pedia") == hex_b("1021BF0420"),
    );
    check(
        "RC4     key=\"Secret\", pt=\"Attack at dawn\" -> 45A01F645FC35B383552544B9BF5",
        rc4::rc4(b"Secret", b"Attack at dawn") == hex_b("45A01F645FC35B383552544B9BF5"),
    );

    // ---- DES ----
    let dk: [u8; 8] = hex_b("0123456789ABCDEF").try_into().unwrap();
    let ct = des::des_encrypt_ecb(&hex_b("4E6F772069732074"), &dk);
    check(
        "DES     key=0123456789ABCDEF,pt=4E6F772069732074 -> 3FA40E8A984D4815",
        ct == hex_b("3FA40E8A984D4815"),
    );
    check("DES     解密往返", des::des_decrypt_ecb(&ct, &dk) == hex_b("4E6F772069732074"));

    // ---- 3DES ----
    let tdk: [u8; 24] = hex_b("0123456789ABCDEFFEDCBA987654321089ABCDEF01234567")
        .try_into()
        .unwrap();
    let tct = des::tdes_encrypt_ecb(b"Now is the time for all ", &tdk);
    check(
        "3DES    EDE-ECB 3 分组 KAT",
        tct == hex_b("FBE62B683922941E0E05E3677C31FC264259965404D683DF"),
    );
    check("3DES    解密往返", des::tdes_decrypt_ecb(&tct, &tdk) == b"Now is the time for all ".to_vec());

    // ---- AES-128 ----
    let ak: [u8; 16] = hex_b("000102030405060708090a0b0c0d0e0f").try_into().unwrap();
    let rks = aes::key_expansion(&ak);
    let act = aes::encrypt_block(&hex_b("00112233445566778899aabbccddeeff").try_into().unwrap(), &rks);
    check(
        "AES-128 FIPS-197 KAT -> 69c4e0d86a7b0430d8cdb78070b4c55a",
        act.to_vec() == hex_b("69c4e0d86a7b0430d8cdb78070b4c55a"),
    );
    // NIST SP 800-38A ECB 向量
    let nk: [u8; 16] = hex_b("2b7e151628aed2a6abf7158809cf4f3c").try_into().unwrap();
    let npt = hex_b("6bc1bee22e409f96e93d7e117393172aae2d8a571e03ac9c9eb76fac45af8e5130c81c46a35ce411e5fbc1191a0a52eff69f2445df4f9b17ad2b417be66c3710");
    let nct = aes::aes128_encrypt_ecb(&npt, &nk);
    check(
        "AES-128 NIST SP800-38A ECB 向量",
        nct == hex_b("3ad77bb40d7a3660a89ecaf32466ef97f5d3d58503b9699de785895a96fdbaaf43b1cd7f598ece23881b00e3ed0306887b0c785e27e8ad3f8223207104725dd4"),
    );
    check("AES-128 解密往返", aes::aes128_decrypt_ecb(&nct, &nk) == npt);

    println!();
    if pass {
        println!("全部测试向量通过 ✔");
    } else {
        println!("存在失败的测试向量 ✘");
    }
    pass
}

fn hex_b(s: &str) -> Vec<u8> {
    hex_to_bytes(s).unwrap()
}

// ---------------------------------------------------------------------------
// main
// ---------------------------------------------------------------------------

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Test => {
            std::process::exit(if run_all_kats() { 0 } else { 1 });
        }
        Commands::Enc { algo, key, data } => match (hex_to_bytes(&key), hex_to_bytes(&data)) {
            (Ok(k), Ok(d)) => match encrypt(&algo, &k, &d) {
                Ok(ct) => println!("{}", to_hex(&ct)),
                Err(e) => {
                    eprintln!("错误：{}", e);
                    std::process::exit(1);
                }
            },
            (Err(e), _) | (_, Err(e)) => {
                eprintln!("错误：{}", e);
                std::process::exit(1);
            }
        },
        Commands::Dec { algo, key, data } => match (hex_to_bytes(&key), hex_to_bytes(&data)) {
            (Ok(k), Ok(d)) => match decrypt(&algo, &k, &d) {
                Ok(pt) => {
                    // 尝试按 UTF-8 打印，失败则打印 hex
                    match String::from_utf8(pt.clone()) {
                        Ok(s) => println!("{}  (hex: {})", s, to_hex(&pt)),
                        Err(_) => println!("{}", to_hex(&pt)),
                    }
                }
                Err(e) => {
                    eprintln!("错误：{}", e);
                    std::process::exit(1);
                }
            },
            (Err(e), _) | (_, Err(e)) => {
                eprintln!("错误：{}", e);
                std::process::exit(1);
            }
        },
    }
}
