use lazy_static::lazy_static;
use std::{
  cell::RefCell,
  fs::{self, File},
  io::{BufRead, BufReader, BufWriter, Write},
  path::Path,
  rc::Rc,
};

use anyhow::Result;
use regex::Regex;

pub struct Dumpfile<'a> {
  file: &'a str,
}

impl<'a> Dumpfile<'a> {
  pub fn new(file: &'a str) -> Self {
    Self { file }
  }

  /// 从备份文件中提取指定数据库内容
  pub fn extract(&self, save_path: &str, db_list: Vec<String>) -> Result<()> {
    let path = Path::new(&self.file);
    let file = fs::File::open(&path)?;

    let reader = BufReader::new(file);

    lazy_static! {
      static ref RE: Regex = Regex::new(r"^-- Current Database: `(.*)`").unwrap();
    }
    let mut writer_warp: Option<Rc<RefCell<BufWriter<File>>>> = None;
    for line in reader.lines() {
      if let Ok(text) = line {
        if let Some(cap) = RE.captures(&text) {
          if db_list.contains(&cap[1].to_string()) {
            let out_file = fs::File::create(format!("{}/{}.sql", save_path, &cap[1]))?;
            let writer = BufWriter::new(out_file);
            writer_warp = Some(Rc::new(RefCell::new(writer)));
          } else {
            writer_warp = None;
          }
        }

        if let Some(ref v) = writer_warp {
          let mut writer = v.borrow_mut();
          writer.write(text.as_bytes())?;
          writer.write("\r\n".as_bytes())?;
        }
      }
    }
    Ok(())
  }

  /// 列出所有的数据库名称
  pub fn list_db(&self) -> Result<Vec<String>> {
    let mut dbs = Vec::new();
    let path = Path::new(&self.file);
    let file = fs::File::open(&path)?;
    let reader = BufReader::new(file);

    lazy_static! {
      static ref RE: Regex = Regex::new(r"^-- Current Database: `(.*)`").unwrap();
    }
    for line in reader.lines() {
      if let Ok(line) = line {
        if let Some(cap) = RE.captures(&line) {
          dbs.push(cap[1].to_string());
        }
      }
    }

    Ok(dbs)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use anyhow::Result;
}
