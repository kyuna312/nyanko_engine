use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("I/O error")]
  TestError,
}