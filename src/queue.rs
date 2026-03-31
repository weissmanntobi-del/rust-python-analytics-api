use crate::{repository::event_repository, state::QueuedEvent};
use sqlx::PgPool;
use tokio::{sync::mpsc::Receiver, task::JoinHandle};
use tracing::{error, info};

pub fn spawn_event_worker(mut rx: Receiver<QueuedEvent>, db: PgPool) -> JoinHandle<()> {
    tokio::spawn(async move {
        info!("event worker started");

        while let Some(event) = rx.recv().await {
            if let Err(err) = event_repository::insert_event(&db, &event).await {
                error!(
                    error = %err,
                    event_id = %event.id,
                    user_id = %event.user_id,
                    "failed to persist queued event"
                );
            }
        }

        info!("event worker stopped");
    })
}
