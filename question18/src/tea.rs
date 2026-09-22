const DELTA: u32 = 0x9E37_79B9;

pub fn tea_encrypt(v: &[u32; 2], key: &[u32; 4]) -> [u32; 2] {
    let [k0, k1, k2, k3] = *key;
    let [mut v0, mut v1] = *v;
    let mut sum: u32 = 0;
    for _ in 0..32 {
        sum = sum.wrapping_add(DELTA);
        v0 = v0.wrapping_add(((v1 << 4).wrapping_add(k0)) ^ (v1.wrapping_add(sum)) ^ ((v1 >> 5).wrapping_add(k1)));
        v1 = v1.wrapping_add(((v0 << 4).wrapping_add(k2)) ^ (v0.wrapping_add(sum)) ^ ((v0 >> 5).wrapping_add(k3)));
    }
    [v0, v1]
}

pub fn tea_decrypt(v: &[u32; 2], key: &[u32; 4]) -> [u32; 2] {
    let [k0, k1, k2, k3] = *key;
    let [mut v0, mut v1] = *v;
    let mut sum: u32 = DELTA.wrapping_mul(32);
    for _ in 0..32 {
        v1 = v1.wrapping_sub(((v0 << 4).wrapping_add(k2)) ^ (v0.wrapping_add(sum)) ^ ((v0 >> 5).wrapping_add(k3)));
        v0 = v0.wrapping_sub(((v1 << 4).wrapping_add(k0)) ^ (v1.wrapping_add(sum)) ^ ((v1 >> 5).wrapping_add(k1)));
        sum = sum.wrapping_sub(DELTA);
    }
    [v0, v1]
}

pub fn xtea_encrypt(v: &[u32; 2], key: &[u32; 4]) -> [u32; 2] {
    let [mut v0, mut v1] = *v;
    let mut sum: u32 = 0;
    for _ in 0..32 {
        v0 = v0.wrapping_add((((v1 << 4) ^ (v1 >> 5)).wrapping_add(v1)) ^ (sum.wrapping_add(key[(sum & 3) as usize])));
        sum = sum.wrapping_add(DELTA);
        v1 = v1.wrapping_add((((v0 << 4) ^ (v0 >> 5)). wrapping_add(v0)) ^ (sum.wrapping_add(key[((sum >> 11) & 3) as usize])));
    }
    [v0, v1]
}

pub fn xtea_decrypt(v: &[u32; 2], key: &[u32; 4]) -> [u32; 2] {
    let [mut v0, mut v1] = *v;
    let mut sum: u32 = DELTA.wrapping_mul(32);
    for _ in 0..32 {
        v1 = v1.wrapping_sub((((v0 << 4) ^ (v0 >> 5)).wrapping_add(v0)) ^ (sum.wrapping_add(key[((sum >> 11) & 3) as usize])));
        sum = sum.wrapping_sub(DELTA);
        v0 = v0.wrapping_sub((((v1 << 4) ^ (v1 >> 5)).wrapping_add(v1)) ^ (sum.wrapping_add(key[(sum & 3) as usize])));
    }
    [v0, v1]
}

#[inline]
fn mx(z: u32, y: u32, sum: u32, key: &[u32; 4], p: usize, e: u32) -> u32 {
    ((z >> 5) ^ (y << 2)).wrapping_add((y >> 3) ^ (z << 4)) ^ (sum ^ y).wrapping_add(key[(p & 3) ^ e as usize] ^ e)
}

/// XXTEA 加密：就地修改 v（n 个 32 位字，n >= 2）
pub fn xxtea_encrypt(v: &mut [u32], key: &[u32; 4]) {
    let n = v.len();
    assert!(n >= 2, "XXTEA 需要至少 2 个 32 位字（8 字节）的数据");
    let rounds = 6 + 52 / n;
    let mut sum: u32 = 0;
    let mut z = v[n - 1];
    for _ in 0..rounds {
        sum = sum.wrapping_add(DELTA);
        let e = (sum >> 2) & 3;
        for p in 0..n - 1 {
            let y = v[p + 1];
            v[p] = v[p].wrapping_add(mx(z, y, sum, key, p, e));
            z = v[p];
        }
        let y = v[0];
        v[n - 1] = v[n - 1].wrapping_add(mx(z, y, sum, key, n - 1, e));
        z = v[n - 1];
    }
}

/// XXTEA 解密：就地修改 v
pub fn xxtea_decrypt(v: &mut [u32], key: &[u32; 4]) {
    let n = v.len();
    assert!(n >= 2, "XXTEA 需要至少 2 个 32 位字（8 字节）的数据");
    let rounds = 6 + 52 / n;
    let mut sum: u32 = DELTA.wrapping_mul(rounds as u32);
    let mut y = v[0];
    while sum != 0 {
        let e = (sum >> 2) & 3;
        for p in (1..n).rev() {
            let z = v[p - 1];
            v[p] = v[p].wrapping_sub(mx(z, y, sum, key, p, e));
            y = v[p];
        }
        let z = v[n - 1];
        v[0] = v[0].wrapping_sub(mx(z, y, sum, key, 0, e));
        y = v[0];
        sum = sum.wrapping_sub(DELTA);
    }
}

/// 8 字节 <-> [u32; 2]（大端）
pub fn bytes_to_block(b: &[u8]) -> [u32; 2] {
    assert_eq!(b.len(), 8, "需要 8 字节");
    [
        u32::from_be_bytes([b[0], b[1], b[2], b[3]]),
        u32::from_be_bytes([b[4], b[5], b[6], b[7]]),
    ]
}

pub fn block_to_bytes(v: &[u32; 2]) -> [u8; 8] {
    let mut out = [0u8; 8];
    out[0..4].copy_from_slice(&v[0].to_be_bytes());
    out[4..8].copy_from_slice(&v[1].to_be_bytes());
    out
}

/// 16 字节密钥 <-> [u32; 4]（大端）
pub fn bytes_to_key(b: &[u8]) -> [u32; 4] {
    assert_eq!(b.len(), 16, "需要 16 字节密钥");
    let mut k = [0u32; 4];
    for i in 0..4 {
        k[i] = u32::from_be_bytes([b[4 * i], b[4 * i + 1], b[4 * i + 2], b[4 * i + 3]]);
    }
    k
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tea_known_answer() {
        let key = [0u32; 4];
        let pt = [0u32; 2];
        let ct = tea_encrypt(&pt, &key);
        assert_eq!(ct, [0x41ea3a0a, 0x94baa940]);
        assert_eq!(tea_decrypt(&ct, &key), pt);
    }
    /// 另一组常用 TEA KAT：key=000102...0e0f, pt=0123456789abcdef
    #[test]
    fn tea_kat2() {
        let key = bytes_to_key(&hex("000102030405060708090a0b0c0d0e0f"));
        let pt = bytes_to_block(&hex("0123456789abcdef"));
        let ct = tea_encrypt(&pt, &key);
        assert_eq!(ct, bytes_to_block(&hex("14f0c75d2bebd98d")));
        assert_eq!(tea_decrypt(&ct, &key), pt);
    }
    /// XTEA KAT（Bouncy Castle 测试向量，32 轮）
    #[test]
    fn xtea_known_answers() {
        let vectors: [([u32; 2], [u32; 4], [u32; 2]); 4] = [
            ([0x00000000, 0x00000000], [0; 4], [0xDEE9D4D8, 0xF7131ED9]),
            ([0x01020304, 0x05060708], [0; 4], [0x065C1B89, 0x75C6A816]),
            (
                [0x00000000, 0x00000000],
                [0x01234567, 0x12345678, 0x23456789, 0x3456789A],
                [0x1FF9A026, 0x1AC64264],
            ),
            (
                [0x01020304, 0x05060708],
                [0x01234567, 0x12345678, 0x23456789, 0x3456789A],
                [0x8C67155B, 0x2EF91EAD],
            ),
        ];
        for (pt, key, ct) in vectors {
            assert_eq!(xtea_encrypt(&pt, &key), ct, "XTEA 加密不匹配");
            assert_eq!(xtea_decrypt(&ct, &key), pt, "XTEA 解密不匹配");
        }
    }
    
    /// XXTEA 对不同长度分组做往返测试（2 字 ~ 26 字）
    #[test]
    fn xxtea_roundtrip_various_sizes() {
        let key = [0x01234567, 0x89abcdef, 0xfedcba98, 0x76543210];
        for n in [2usize, 3, 4, 5, 8, 13, 26] {
            let mut v: Vec<u32> = (0..n).map(|i| (i as u32).wrapping_mul(0x9e3779b9)).collect();
            let original = v.clone();
            xxtea_encrypt(&mut v, &key);
            xxtea_decrypt(&mut v, &key);
            assert_eq!(v, original, "XXTEA 往返失败，n = {}", n);
        }
    }
    /// 测试辅助：解析 hex 字符串
    fn hex(s: &str) -> Vec<u8> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
            .collect()
    }
}