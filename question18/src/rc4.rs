pub struct Rc4 {
    s: [u8; 256],
    i: u8,
    j: u8,
}

impl Rc4 {
    pub fn new(key: &[u8]) -> Self {
        assert!(!key.is_empty() && key.len() <= 256, "RC4 密钥长度需在 1..=256 字节");
        let mut s = [0u8; 256];
        for i in 0..256 {
            s[i] = i as u8;
        }
        let mut j: u8 = 0;
        for i in 0..256 {
            j = j.wrapping_add(s[i].wrapping_add(key[i % key.len()]));
            s.swap(i, j as usize);
        }
        Rc4 {
            s,
            i: 0,
            j: 0,
        }
    }
    pub fn keystream(&mut self, len: usize) -> Vec<u8> {
        let mut out = Vec::with_capacity(len);
        for _ in 0..len {
            self.i = self.i.wrapping_add(1);
            self.j = self.j.wrapping_add(self.s[self.i as usize]);
            self.s.swap(self.i as usize, self.j as usize);
            let k = self.s[self.s[self.i as usize].wrapping_add(self.s[self.j as usize]) as usize];
            out.push(k);
        }
        out
    }
    pub fn apply(&mut self, data: &[u8]) -> Vec<u8> {
        let ks = self.keystream(data.len());
        data.iter().zip(ks.iter()).map(|(d, k)| d ^ k).collect()
    }
}

pub fn rc4(key: &[u8], data: &[u8]) -> Vec<u8> {
    Rc4::new(key).apply(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn hex(s: &str) -> Vec<u8> {
        (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i+2], 16).unwrap()).collect()
    }

    #[test]
    fn rc4_wikipedia_vectors() {
        let cases: [(&[u8], &[u8], &str); 3] = [
            (b"Key", b"Plaintext", "BBF316E8D940AF0AD3"),
            (b"Wiki", b"pedia", "1021BF0420"),
            (b"Secret", b"Attack at dawn", "45A01F645FC35B383552544B9BF5"),
        ];
        for (key, pt, ct_hex) in cases {
            let ct = rc4(key, pt);
            assert_eq!(ct, hex(ct_hex), "RC4 加密不匹配（key = {:?}）", key);
            assert_eq!(rc4(key, &ct), pt, "RC4 解密不匹配（key = {:?}）", key);
        }
    }
}