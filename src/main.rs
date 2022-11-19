/**
 * 以实现git init add commit 3个命令完成本地版本管理
 *
 * 第一步 init
 * 要点：读取当前操作目录
 * 创建.git文件夹
 * 第二部 add
 * 创建 index文件 完成blob压缩文件的创建
 * 第三步 commit
 * 创建tree commit 文件，
 *
 */
use clap::Parser;
use gitr::command::{control, Args};

fn main() {
    let args = Args::parse();
    control(args).unwrap();
}
