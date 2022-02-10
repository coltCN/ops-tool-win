use flate2::read::GzDecoder;
use lazy_static::lazy_static;
use std::{
  fs::{self, File},
  io::{BufRead, BufReader, BufWriter, Write},
  path::Path,
  str,
};

use anyhow::{anyhow, Result};
use regex::Regex;

lazy_static! {
  static ref RE: Regex = Regex::new(r"^-- Current Database: `(.*)`").unwrap();
  static ref TMP_PATH: &'static Path = Path::new("./dump_tmp/");
}

pub struct Dumpfile {
  reader: Box<dyn BufRead>,
}

impl Dumpfile {
  pub fn new(file: &str) -> Result<Self> {
    let path = Path::new(file);
    // println!("当前文件扩展名: {:?}", path.extension());
    let file = fs::File::open(path).expect("file not found!");
    match path.extension() {
      Some(ext) if ext == "sql" => {
        let reader = BufReader::new(file);
        Ok(Self {
          reader: Box::new(reader),
        })
      }
      Some(ext) if ext == "gz" => {
        let reader = BufReader::new(GzDecoder::new(file));
        Ok(Self {
          reader: Box::new(reader),
        })
      }
      _ => Err(anyhow!("不支持的文件!")),
    }
  }

  /// 从备份文件中提取指定数据库内容
  pub fn extract(&mut self, save_path: &str, db_list: Vec<String>) -> Result<()> {
    // let reader = BufReader::new(file);

    let mut writer_warp: Option<Box<BufWriter<File>>> = None;
    let mut buf = vec![];
    loop {
      buf.clear();
      if let Ok(mun) = self.reader.as_mut().read_until(0xA, &mut buf) {
        if mun == 0 {
          break;
        }
        if buf.starts_with(b"-") {
          if let Ok(line) = str::from_utf8(buf.as_slice()) {
            if let Some(cap) = RE.captures(&line) {
              // 检查当前数据库是不是 指定数据库，如果是，新建文件
              if db_list.contains(&cap[1].to_string()) {
                let out_file = fs::File::create(format!("{}/{}.sql", save_path, &cap[1]))?;
                let writer = BufWriter::new(out_file);
                writer_warp = Some(Box::new(writer));
              } else {
                writer_warp = None;
              }
            }
          }
        }
        if let Some(ref mut writer) = writer_warp {
          writer.write(buf.as_slice())?;
          writer.write("\r\n".as_bytes())?;
        }
      }
    }
    Ok(())
  }

  /// 列出所有的数据库名称
  pub fn list_db(&mut self) -> Result<Vec<String>> {
    let mut dbs = Vec::new();

    let mut buf = vec![];
    loop {
      buf.clear();
      if let Ok(mun) = self.reader.as_mut().read_until(0xA, &mut buf) {
        if mun == 0 {
          break;
        }
        if buf.starts_with(b"-") {
          if let Ok(line) = str::from_utf8(buf.as_slice()) {
            if let Some(cap) = RE.captures(&line) {
              println!("找到数据库: {}", cap[1].to_string());
              dbs.push(cap[1].to_string());
            }
          } else {
            println!("not uf-8");
          }
        }
      }
    }
    Ok(dbs)
  }

  /// 解析dump file 并安数据库提取 dumpfile
  pub fn list_extract_dbs(&mut self) -> Result<Vec<String>> {
    let mut dbs = Vec::new();

    // 创建临时目录
    if TMP_PATH.exists() {
      fs::remove_dir_all(*TMP_PATH)?;
    }
    // println!("{:?}", &*TMP_PATH);
    fs::create_dir(*TMP_PATH)?;

    let mut writer_warp: Option<Box<BufWriter<File>>> = None;
    let mut buf = vec![];
    loop {
      buf.clear();
      if let Ok(mun) = self.reader.as_mut().read_until(0xA, &mut buf) {
        if mun == 0 {
          break;
        }
        if buf.starts_with(b"-") {
          if let Ok(line) = str::from_utf8(buf.as_slice()) {
            if let Some(cap) = RE.captures(&line) {
              println!("找到数据库: {}", cap[1].to_string());
              dbs.push(cap[1].to_string());
              let out_file = fs::File::create(TMP_PATH.join(format!("{}.sql", &cap[1])))?;
              let writer = BufWriter::new(out_file);
              writer_warp = Some(Box::new(writer));
            }
          }
        }
        if let Some(ref mut writer) = writer_warp {
          writer.write(buf.as_slice())?;
          writer.write("\r\n".as_bytes())?;
        }
      }
    }
    Ok(dbs)
  }
}
/// 另存为，对已提取到临时目录的文件另存为
pub fn save_as(save_path: &str, db_list: Vec<String>) -> Result<()> {
  let save_path = Path::new(save_path);
  // println!("另存到: {:?}", save_path,);
  for db in db_list {
    let file_path = TMP_PATH.join(format!("{}.sql", &db));
    if file_path.exists() {
      fs::copy(file_path, save_path.join(format!("{}.sql", &db)))?;
    }
  }

  Ok(())
}

#[cfg(test)]
mod test {
  use super::*;
  use anyhow::Result;
}
