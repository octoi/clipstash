use crate::data::DatabasePool;
use crate::ShortCode;
use crate::service::{self, ServiceError};
use crossbeam_channel::TryRecvError;
use crossbeam_channel::{unbounded, Sender};
use parking_lot::Mutex;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Handle;

enum HitCountMsg {
    Commit,
    Hit(ShortCode, u32)
}

pub struct HitCounter {
    tx: Sender<HitCountMsg>
}

impl HitCounter {
    pub fn new(pool: DatabasePool, handle: Handle) -> Self {
        todo!()
    }
}