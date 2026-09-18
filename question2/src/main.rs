use std::path::Path;
use std::env;
use chrono:: {DateTime, Local};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let file_path = Path::new(&args[1]);
        if file_path.exists() {
            let metadata = file_path.metadata()?;
            println!("file name: {:?}", file_path.file_name());
            let file_size = if metadata.len() >= 1024 && metadata.len() < 1024*1024 {
                format!("{} KB", metadata.len()/1024)
            } else if metadata.len() > 1024 * 1024 {
                format!("{} MB", metadata.len()/(1024*1024))
            } else {
                format!("{} B", metadata.len())
            };
            println!("file size: {}", file_size);
            println!("is file? {}", metadata.is_file());
            println!("is dir? {}", metadata.is_dir());
            let datetime: DateTime<Local> = metadata.modified()?.into();
            println!("modified time: {}", datetime.format("%Y-%m-%d %H:%M:%S"));

        }else {
            println!("File does not exist: {}", file_path.display());
        }
    } else {
        println!("参数错误");
    }
    Ok(())
}
