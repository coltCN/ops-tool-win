use anyhow::Result;
use std::{
  env,
  fs::File,
  io::{BufRead, BufReader},
};

use flate2::read::GzDecoder;
fn main() -> Result<()> {
  let current_dir = env::current_dir()?;
  println!("当前目录: {:?}", &current_dir);
  let file = File::open(r"../test-file/db.sql.gz").expect("Ops. file not found!");
  let mut reader = BufReader::new(GzDecoder::new(file));
  let mut line = 0;
  loop {
    let mut buf = String::new();
    let mun_bytes = reader.read_line(&mut buf).expect("Error reading");
    if mun_bytes == 0 || line > 10 {
      break;
    }
    println!("{}", buf);
    buf.clear();
    line += 1;
  }

  Ok(())
}
