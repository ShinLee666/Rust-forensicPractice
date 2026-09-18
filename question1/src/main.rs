use std::env;
use std::path::Path;
fn main() {
    let args: Vec<String> = env::args().collect();
    let count = args.len();
    match count{
        2 => {
            let path = Path::new(&args[1]);
            if path.exists() {
                println!("{} exists", path.display());
            } else {
                println!("{} does not exists", path.display());
            }
        },
        _ => {
            println!("只能提供1个参数");
        }
    }
}
