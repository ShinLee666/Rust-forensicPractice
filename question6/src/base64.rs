const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
pub fn encode(input: &[u8]) -> String {
    let mut out = String::new();
    for chunk in input.chunks(3) {
        let n = (chunk[0] as u32) << 16 |
            (*chunk.get(1).unwrap_or(&0) as u32) << 8 |
            (*chunk.get(2).unwrap_or(&0) as u32);
        out.push(ALPHABET[(n >> 18) as usize & 0x3f] as char);
        out.push(ALPHABET[(n >> 12) as usize & 0x3f] as char);
        out.push(
            if chunk.len() > 1 {
                ALPHABET[(n >> 6) as usize & 0x3f] as char
            } else {
                '='
            }
        );
        out.push(
            if chunk.len() > 2 {
                ALPHABET[n as usize & 0x3f] as char
            } else {
                '='
            }
        );
    }
    out
}

pub fn decode(input: &[u8]) -> String {
    let mut out = String::new();
    for chunk in input.chunks(4) {
        let n = ((ALPHABET.iter().position(|&b| b == chunk[0] ).unwrap() as u32) << 18) |
            ((ALPHABET.iter().position(|&b| b == chunk[1] ).unwrap() as u32) << 12) |
            ((ALPHABET.iter().position(|&b| b == chunk[2]).unwrap_or(0) as u32) << 6) |
            ALPHABET.iter().position(|&b| b == chunk[3]).unwrap_or(0) as u32;
        out.push(((n >> 16) & 0xff) as u8 as char);
        if (n >> 8) & 0xff != 0 {
            out.push(((n >> 8) & 0xff) as u8 as char);
        }
        if n & 0xff != 0 {
            out.push((n & 0xff) as u8 as char);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_encode() {
        assert_eq!(encode(b"123456"), "MTIzNDU2");
        assert_eq!(encode(b"12345"), "MTIzNDU=");
        assert_eq!(encode(b"1234"), "MTIzNA==");
    }
    #[test]
    fn test_decode() {
        assert_eq!(decode(b"MTIzNDU2"), "123456");
        assert_eq!(decode(b"MTIzNDU="), "12345");
        assert_eq!(decode(b"MTIzNA=="), "1234");
    }
}