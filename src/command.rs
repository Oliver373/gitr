use std::env::current_dir;
use std::error::Error;
use std::fs::create_dir;

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// create git warehouse
    #[arg(short, long)]
    init: bool,
}

pub fn control(args: Args) -> MyResult<()> {
    if args.init {
        // dbg!("创建文件夹");
        let dir = current_dir()?;
        println!("{:?}", dir);
        match dir.join(".git").exists() {
            false => create_dir(dir.join(".git"))?,
            true => println!("该文件已由git管理"),
        }
    }
    return Ok(());
}
