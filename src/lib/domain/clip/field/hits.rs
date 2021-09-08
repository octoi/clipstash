use derive_more::Constructor;
use serde::{ Deserialize, Serialize };
use crate::domain::clip::ClipError;

#[derive(Clone, Constructor, Debug, Deserialize, Serialize)]
pub struct Hits(u64);

impl Hits {
    pub fn into_inner(self) -> u64 {
        self.0
    }
}