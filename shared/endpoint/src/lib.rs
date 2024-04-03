pub mod user;
pub mod post;

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub trait Endpoint {
    const URL: &'static str;
    fn url(&self) -> &'static str {
        Self::URL
    }
}

//создаем тип ответа на ошибку
#[derive(Error, Debug, Deserialize, Serialize)]
#[error("{msg}")]
pub struct RequestFailed {
    pub msg: String,
}