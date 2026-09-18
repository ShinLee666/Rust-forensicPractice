#[derive(Debug)]
struct BruteForceItem {
    item: Vec<u8>,
    score: f32,
    key: u8
}
impl BruteForceItem {
    fn new(item: Vec<u8>, key: u8) -> BruteForceItem {
        BruteForceItem {
            item,
            score: 0.0,
            key
        }
    }
    fn calc_score(&mut self) {
        let mut count = 0u32;
        let total = self.item.len() as u32;
        for b in self.item.iter() {
            if *b >= 32 && *b <= 126 {
                count += 1;
            }
        }
        self.score = count as f32 / total as f32;
    }

}
fn xor_byte_crypt(plaintext: &[u8], key: u8) -> Vec<u8> {
    let mut cipher = Vec::new();
    for byte in plaintext {
        cipher.push(*byte ^ key);
    }
    cipher
}
fn xor_brute_force(cipher: &Vec<u8>) -> Vec<BruteForceItem> {
    let mut brute_force = Vec::new();
    for k in 0..=255u8 {
        let mut brute_force_item = BruteForceItem::new(xor_byte_crypt(&cipher, k), k);
        brute_force_item.calc_score();
        brute_force.push(brute_force_item);
    }
    brute_force
}
fn main() {
    let plaintext = b"mypassword";
    let cipher = xor_byte_crypt(plaintext, 0x47);
    println!("plaintext: {}, cipher: {:?}", std::str::from_utf8(plaintext).unwrap(), cipher);
    let mut brute_force_result = xor_brute_force(&cipher);
    brute_force_result.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    for brute_force_item in brute_force_result {
        println!("{:?}, plaintext:{}", brute_force_item, std::str::from_utf8(&brute_force_item.item).unwrap_or(""));
    }
}
