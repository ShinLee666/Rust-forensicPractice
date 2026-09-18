use question6::base64;

fn main() {
    let s = base64::encode(b"1234567");
    println!("123456的base64编码：{}", s);
    let decode = base64::decode(b"eGN6aGtqZGFzaXVlcndidA==");
    println!("eGN6aGtqZGFzaXVlcndidA==的base64接码：{}", decode);
    
}
