use std::io::{BufRead, BufReader};
use std::fs::File;
use std::result::Result;
use std::error::Error;


/// 使用一个open函数来包裹不同的情况，最后都返回一个BufRead类型的，
/// 这个需要对读取器这个trait有了解才可以写出来，这个写法有点 rust-style
/// 要知道 stdin 和 File 都实现了 read？trait？TODO 回去翻一下书
#[allow(dead_code)]
pub fn open(filename: &str) -> Result<Box<dyn BufRead>, Box<dyn Error>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}