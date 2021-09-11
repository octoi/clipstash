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

type Hitstore = Arc<Mutex<HashMap<ShortCode, u32>>>;

#[derive(Debug, thiserror::Error)]
pub enum HitCountError {
    #[error("service error: {0}")]
    Service(#[from] ServiceError),
    #[error("communication error: {0}")]
    Channel(#[from] crossbeam_channel::SendError<HitCountMsg>),
}

pub enum HitCountMsg {
    Commit,
    Hit(ShortCode, u32),
}

pub struct HitCounter {
    tx: Sender<HitCountMsg>,
}

impl HitCounter {
    pub fn commit_hits(
        hits: Hitstore,
        handle: Handle,
        pool: DatabasePool,
    ) -> Result<(), HitCountError> {
        let hits: Hitstore = Arc::clone(&hits);
        let hits: Vec<(ShortCode, u32)> = {
            let mut hits = hits.lock();
            let hits_vec = hits.iter().map(|(k, v)| (k.clone(), *v)).collect();
            hits.clear();
            hits_vec
        };
        handle.block_on(async move {
            let transaction = service::action::begin_transaction(&pool).await?;
            for (shortcode, hit) in hits {
                if let Err(e) = service::action::increase_hit_count(&shortcode, hit, &pool).await {
                    eprintln!("error increasing hit count: {:?}", e);
                }
            }
            Ok(service::action::end_transaction(transaction).await?)
        })
    }

    pub fn process_msg(
        msg: HitCountMsg,
        hits: Hitstore,
        handle: Handle,
        pool: DatabasePool,
    ) -> Result<(), HitCountError> {
        match msg {
            HitCountMsg::Commit => Self::commit_hits(hits.clone(), handle.clone(), pool)?,
            HitCountMsg::Hit(shortcode, count) => {
                let mut hitcount = hits.lock();
                let hitcount = hitcount.entry(shortcode).or_insert(0);
                *hitcount += count;
            }
        }
        Ok(())
    }

    pub fn new(pool: DatabasePool, handle: Handle) -> Self {
        let (tx, rx) = unbounded();
        let tx_clone = tx.clone();
        let rx_clone = rx.clone();

        let _ = std::thread::spawn(move || {
            println!("HitCounter thread spawned");
            let store: Hitstore = Arc::new(Mutex::new(HashMap::new()));

            loop {
                match rx_clone.try_recv() {
                    Ok(msg) => if let Err(e) = Self::process_msg(msg, store.clone(), handle.clone(), pool.clone()) {
                        eprintln!("message processing error: {:?}", e);
                    },
                    Err(e) => todo!()
                }
            }
        });

        Self { tx }
    }

    pub fn hit(&self, shortcode: ShortCode, count: u32) {
        if let Err(e) = self.tx.send(HitCountMsg::Hit(shortcode, count)) {
            eprintln!("hit count error: {}", e);
        }
    }
}