use std::fs::{read_to_string, metadata};
use std::path::Path;
use std::time::{Duration, SystemTime};

use anyhow::Result;

use crate::display::models::DisplayMessage;
use crate::display::DisplayEventTx;

pub async fn watch_todo(display_tx: DisplayEventTx, path: impl AsRef<Path>) -> Result<()> {
    let mut current_ts = SystemTime::now();
    loop {
        let path = path.as_ref();
        let meta = metadata(path)?;

        if meta.modified()? != current_ts {
            current_ts = meta.modified()?;
            let data = match read_to_string(path) {
                Ok(d) => d,
                Err(e) => {
                    log::error!("Failed to read file: {}", e);
                    break;
                }
            };
            display_tx.send(DisplayMessage::TodoUpdate(data))?;
        }

        tokio::time::sleep(Duration::from_secs(2)).await;
    }
    Ok(())
}
