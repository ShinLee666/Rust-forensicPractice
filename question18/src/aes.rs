//! AES-128-ECB 手写实现（FIPS-197）
//!
//! 128 位分组、128 位密钥、10 轮。
//! 状态矩阵按列优先存放为一维数组：state[col * 4 + row]。
//! 仅实现 ECB 模式，不做填充，数据长度必须是 16 的倍数。

// ---------------------------------------------------------------------------
// S 盒与其逆（FIPS-197 表）
// ---------------------------------------------------------------------------

const SBOX: [u8; 256] = [
    0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab,
    0x76, 0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4,
    0x72, 0xc0, 0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71,
    0xd8, 0x31, 0x15, 0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2,
    0xeb, 0x27, 0xb2, 0x75, 0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6,
    0xb3, 0x29, 0xe3, 0x2f, 0x84, 0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb,
    0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf, 0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45,
    0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8, 0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5,
    0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2, 0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44,
    0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73, 0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a,
    0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb, 0xe0, 0x32, 0x3a, 0x0a, 0x49,
    0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79, 0xe7, 0xc8, 0x37, 0x6d,
    0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08, 0xba, 0x78, 0x25,
    0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a, 0x70, 0x3e,
    0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e, 0xe1,
    0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
    0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb,
    0x16,
];

const INV_SBOX: [u8; 256] = [
    0x52, 0x09, 0x6a, 0xd5, 0x30, 0x36, 0xa5, 0x38, 0xbf, 0x40, 0xa3, 0x9e, 0x81, 0xf3, 0xd7,
    0xfb, 0x7c, 0xe3, 0x39, 0x82, 0x9b, 0x2f, 0xff, 0x87, 0x34, 0x8e, 0x43, 0x44, 0xc4, 0xde,
    0xe9, 0xcb, 0x54, 0x7b, 0x94, 0x32, 0xa6, 0xc2, 0x23, 0x3d, 0xee, 0x4c, 0x95, 0x0b, 0x42,
    0xfa, 0xc3, 0x4e, 0x08, 0x2e, 0xa1, 0x66, 0x28, 0xd9, 0x24, 0xb2, 0x76, 0x5b, 0xa2, 0x49,
    0x6d, 0x8b, 0xd1, 0x25, 0x72, 0xf8, 0xf6, 0x64, 0x86, 0x68, 0x98, 0x16, 0xd4, 0xa4, 0x5c,
    0xcc, 0x5d, 0x65, 0xb6, 0x92, 0x6c, 0x70, 0x48, 0x50, 0xfd, 0xed, 0xb9, 0xda, 0x5e, 0x15,
    0x46, 0x57, 0xa7, 0x8d, 0x9d, 0x84, 0x90, 0xd8, 0xab, 0x00, 0x8c, 0xbc, 0xd3, 0x0a, 0xf7,
    0xe4, 0x58, 0x05, 0xb8, 0xb3, 0x45, 0x06, 0xd0, 0x2c, 0x1e, 0x8f, 0xca, 0x3f, 0x0f, 0x02,
    0xc1, 0xaf, 0xbd, 0x03, 0x01, 0x13, 0x8a, 0x6b, 0x3a, 0x91, 0x11, 0x41, 0x4f, 0x67, 0xdc,
    0xea, 0x97, 0xf2, 0xcf, 0xce, 0xf0, 0xb4, 0xe6, 0x73, 0x96, 0xac, 0x74, 0x22, 0xe7, 0xad,
    0x35, 0x85, 0xe2, 0xf9, 0x37, 0xe8, 0x1c, 0x75, 0xdf, 0x6e, 0x47, 0xf1, 0x1a, 0x71, 0x1d,
    0x29, 0xc5, 0x89, 0x6f, 0xb7, 0x62, 0x0e, 0xaa, 0x18, 0xbe, 0x1b, 0xfc, 0x56, 0x3e, 0x4b,
    0xc6, 0xd2, 0x79, 0x20, 0x9a, 0xdb, 0xc0, 0xfe, 0x78, 0xcd, 0x5a, 0xf4, 0x1f, 0xdd, 0xa8,
    0x33, 0x88, 0x07, 0xc7, 0x31, 0xb1, 0x12, 0x10, 0x59, 0x27, 0x80, 0xec, 0x5f, 0x60, 0x51,
    0x7f, 0xa9, 0x19, 0xb5, 0x4a, 0x0d, 0x2d, 0xe5, 0x7a, 0x9f, 0x93, 0xc9, 0x9c, 0xef, 0xa0,
    0xe0, 0x3b, 0x4d, 0xae, 0x2a, 0xf5, 0xb0, 0xc8, 0xeb, 0xbb, 0x3c, 0x83, 0x53, 0x99, 0x61,
    0x17, 0x2b, 0x04, 0x7e, 0xba, 0x77, 0xd6, 0x26, 0xe1, 0x69, 0x14, 0x63, 0x55, 0x21, 0x0c,
    0x7d,
];

/// 轮常数
const RCON: [u8; 10] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36];

// ---------------------------------------------------------------------------
// 密钥扩展：128 位密钥 -> 11 组轮密钥（每组 16 字节）
// ---------------------------------------------------------------------------

/// 密钥扩展：返回 11 个轮密钥（rk[0] 为初始轮密钥加，rk[10] 用于最后一轮）
pub fn key_expansion(key: &[u8; 16]) -> [[u8; 16]; 11] {
    // w[i] 为 4 字节字，共 44 个字
    let mut w = [[0u8; 4]; 44];
    for i in 0..4 {
        w[i] = [key[4 * i], key[4 * i + 1], key[4 * i + 2], key[4 * i + 3]];
    }
    for i in 4..44 {
        let mut t = w[i - 1];
        if i % 4 == 0 {
            // RotWord + SubWord + Rcon
            t = [SBOX[t[1] as usize], SBOX[t[2] as usize], SBOX[t[3] as usize], SBOX[t[0] as usize]];
            t[0] ^= RCON[i / 4 - 1];
        }
        for j in 0..4 {
            w[i][j] = w[i - 4][j] ^ t[j];
        }
    }
    let mut rks = [[0u8; 16]; 11];
    for r in 0..11 {
        for c in 0..4 {
            rks[r][4 * c..4 * c + 4].copy_from_slice(&w[4 * r + c]);
        }
    }
    rks
}

// ---------------------------------------------------------------------------
// 轮函数组件（状态为 16 字节，按列优先：state[col * 4 + row]）
// ---------------------------------------------------------------------------

/// xtime：GF(2^8) 上乘 2（模多项式 x^8+x^4+x^3+x+1 = 0x11B）
#[inline]
fn xtime(a: u8) -> u8 {
    let hi = a & 0x80;
    let shifted = a << 1;
    if hi != 0 {
        shifted ^ 0x1b
    } else {
        shifted
    }
}

/// GF(2^8) 乘法（俄罗斯农夫算法）
fn gf_mul(mut a: u8, mut b: u8) -> u8 {
    let mut r = 0u8;
    while b != 0 {
        if b & 1 != 0 {
            r ^= a;
        }
        a = xtime(a);
        b >>= 1;
    }
    r
}

fn add_round_key(state: &mut [u8; 16], rk: &[u8; 16]) {
    for i in 0..16 {
        state[i] ^= rk[i];
    }
}

fn sub_bytes(state: &mut [u8; 16]) {
    for b in state.iter_mut() {
        *b = SBOX[*b as usize];
    }
}

fn inv_sub_bytes(state: &mut [u8; 16]) {
    for b in state.iter_mut() {
        *b = INV_SBOX[*b as usize];
    }
}

fn shift_rows(state: &mut [u8; 16]) {
    for r in 1..4 {
        // 取第 r 行（4 个字节，跨 4 列），循环左移 r 格
        let row = [state[r], state[4 + r], state[8 + r], state[12 + r]];
        for c in 0..4 {
            state[c * 4 + r] = row[(c + r) % 4];
        }
    }
}

fn inv_shift_rows(state: &mut [u8; 16]) {
    for r in 1..4 {
        let row = [state[r], state[4 + r], state[8 + r], state[12 + r]];
        for c in 0..4 {
            state[c * 4 + r] = row[(c + 4 - r) % 4];
        }
    }
}

fn mix_columns(state: &mut [u8; 16]) {
    for c in 0..4 {
        let a0 = state[c * 4];
        let a1 = state[c * 4 + 1];
        let a2 = state[c * 4 + 2];
        let a3 = state[c * 4 + 3];
        state[c * 4] = xtime(a0) ^ (xtime(a1) ^ a1) ^ a2 ^ a3; // 2*a0 + 3*a1 + a2 + a3
        state[c * 4 + 1] = a0 ^ xtime(a1) ^ (xtime(a2) ^ a2) ^ a3;
        state[c * 4 + 2] = a0 ^ a1 ^ xtime(a2) ^ (xtime(a3) ^ a3);
        state[c * 4 + 3] = (xtime(a0) ^ a0) ^ a1 ^ a2 ^ xtime(a3);
    }
}

fn inv_mix_columns(state: &mut [u8; 16]) {
    for c in 0..4 {
        let a0 = state[c * 4];
        let a1 = state[c * 4 + 1];
        let a2 = state[c * 4 + 2];
        let a3 = state[c * 4 + 3];
        state[c * 4] = gf_mul(a0, 14) ^ gf_mul(a1, 11) ^ gf_mul(a2, 13) ^ gf_mul(a3, 9);
        state[c * 4 + 1] = gf_mul(a0, 9) ^ gf_mul(a1, 14) ^ gf_mul(a2, 11) ^ gf_mul(a3, 13);
        state[c * 4 + 2] = gf_mul(a0, 13) ^ gf_mul(a1, 9) ^ gf_mul(a2, 14) ^ gf_mul(a3, 11);
        state[c * 4 + 3] = gf_mul(a0, 11) ^ gf_mul(a1, 13) ^ gf_mul(a2, 9) ^ gf_mul(a3, 14);
    }
}

// ---------------------------------------------------------------------------
// 单分组加解密
// ---------------------------------------------------------------------------

/// 加密一个 16 字节分组
pub fn encrypt_block(block: &[u8; 16], rks: &[[u8; 16]; 11]) -> [u8; 16] {
    let mut state = *block;
    add_round_key(&mut state, &rks[0]);
    for rnd in 1..10 {
        sub_bytes(&mut state);
        shift_rows(&mut state);
        mix_columns(&mut state);
        add_round_key(&mut state, &rks[rnd]);
    }
    // 最后一轮没有 MixColumns
    sub_bytes(&mut state);
    shift_rows(&mut state);
    add_round_key(&mut state, &rks[10]);
    state
}

/// 解密一个 16 字节分组
pub fn decrypt_block(block: &[u8; 16], rks: &[[u8; 16]; 11]) -> [u8; 16] {
    let mut state = *block;
    add_round_key(&mut state, &rks[10]);
    for rnd in (1..10).rev() {
        inv_shift_rows(&mut state);
        inv_sub_bytes(&mut state);
        add_round_key(&mut state, &rks[rnd]);
        inv_mix_columns(&mut state);
    }
    inv_shift_rows(&mut state);
    inv_sub_bytes(&mut state);
    add_round_key(&mut state, &rks[0]);
    state
}

// ---------------------------------------------------------------------------
// ECB 模式（无填充，数据长度必须是 16 的倍数）
// ---------------------------------------------------------------------------

/// AES-128-ECB 加密
pub fn aes128_encrypt_ecb(data: &[u8], key: &[u8; 16]) -> Vec<u8> {
    assert!(data.len() % 16 == 0 && !data.is_empty(), "AES-ECB 数据长度必须是 16 的非零倍数");
    let rks = key_expansion(key);
    data.chunks(16)
        .flat_map(|c| encrypt_block(c.try_into().unwrap(), &rks))
        .collect()
}

/// AES-128-ECB 解密
pub fn aes128_decrypt_ecb(data: &[u8], key: &[u8; 16]) -> Vec<u8> {
    assert!(data.len() % 16 == 0 && !data.is_empty(), "AES-ECB 数据长度必须是 16 的非零倍数");
    let rks = key_expansion(key);
    data.chunks(16)
        .flat_map(|c| decrypt_block(c.try_into().unwrap(), &rks))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hex(s: &str) -> Vec<u8> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
            .collect()
    }

    /// FIPS-197 附录 B 的 AES-128 KAT：
    /// key = 000102030405060708090a0b0c0d0e0f
    /// pt  = 00112233445566778899aabbccddeeff
    /// ct  = 69c4e0d86a7b0430d8cdb78070b4c55a
    #[test]
    fn aes_fips197_known_answer() {
        let key: [u8; 16] = hex("000102030405060708090a0b0c0d0e0f").try_into().unwrap();
        let pt: [u8; 16] = hex("00112233445566778899aabbccddeeff").try_into().unwrap();
        let rks = key_expansion(&key);
        let ct = encrypt_block(&pt, &rks);
        assert_eq!(ct.to_vec(), hex("69c4e0d86a7b0430d8cdb78070b4c55a"), "AES 加密不匹配");
        assert_eq!(decrypt_block(&ct, &rks), pt, "AES 解密不匹配");
    }

    /// 经典 AES-128 KAT：key=00000000000000000000000000000000，pt=00000000000000000000000000000000
    /// ct=66e94bd4ef8a2c3b884cfa59ca342b2e
    #[test]
    fn aes_all_zero() {
        let key = [0u8; 16];
        let pt = [0u8; 16];
        let rks = key_expansion(&key);
        let ct = encrypt_block(&pt, &rks);
        assert_eq!(ct.to_vec(), hex("66E94BD4EF8A2C3B884CFA59CA342B2E"));
        assert_eq!(decrypt_block(&ct, &rks), pt);
    }

    /// ECB 多分组往返
    #[test]
    fn aes_ecb_multiblock_roundtrip() {
        let key: [u8; 16] = hex("2b7e151628aed2a6abf7158809cf4f3c").try_into().unwrap();
        let data = hex("6bc1bee22e409f96e93d7e117393172aae2d8a571e03ac9c9eb76fac45af8e5130c81c46a35ce411e5fbc1191a0a52eff69f2445df4f9b17ad2b417be66c3710");
        let ct = aes128_encrypt_ecb(&data, &key);
        // NIST SP 800-38A 的 AES-128-ECB 标准密文
        assert_eq!(ct, hex("3ad77bb40d7a3660a89ecaf32466ef97f5d3d58503b9699de785895a96fdbaaf43b1cd7f598ece23881b00e3ed0306887b0c785e27e8ad3f8223207104725dd4"));
        assert_eq!(aes128_decrypt_ecb(&ct, &key), data);
    }
}
