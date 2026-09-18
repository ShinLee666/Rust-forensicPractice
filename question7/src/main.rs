fn crc32_bit(buf: &[u8], poly: Option<u32>) -> u32 {
    let p = poly.unwrap_or(0xEDB88320);
    let mut crc: u32 = 0xFFFFFFFF;
    for b in buf {
        crc ^= *b as u32;
        for _ in 0..8 {
            if crc & 1 == 1 {
                crc = (crc >> 1) ^ p;
            } else {
                crc >>= 1;
            }
        }
    }
    crc ^ 0xFFFFFFFF
}
fn main() {
    let test_cases = b"123456789";
    let crc = crc32_bit(test_cases, None);
    println!("{:?} crc32 bits = {:x}", test_cases, crc);
}
