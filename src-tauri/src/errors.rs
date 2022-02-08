use tauri::InvokeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
  #[error(transparent)]
  Other(#[from] anyhow::Error),
}

impl From<InvokeError> for AppError {
  fn from(_: InvokeError) -> Self {
    todo!()
  }
}
