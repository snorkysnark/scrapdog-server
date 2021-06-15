use crate::storage::Storage;
use anyhow::Result;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::{reject::Reject, Filter, Rejection};

struct AnyhowRejecion(anyhow::Error);
impl Reject for AnyhowRejecion {}
impl std::fmt::Debug for AnyhowRejecion {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(fmt)
    }
}

trait IntoRejection {
    fn into_rejection(self) -> Rejection;
}
impl IntoRejection for anyhow::Error {
    fn into_rejection(self) -> Rejection {
        warp::reject::custom(AnyhowRejecion(self))
    }
}

async fn list_scrapbooks(storage: &Arc<Mutex<Storage>>) -> Result<String> {
    let storage = storage.lock().await;
    let scrapbooks = storage.list_scrapbooks()?;
    let json = serde_json::to_string(&scrapbooks)?;
    Ok(json)
}

async fn get_scrapbook_tree(storage: &Arc<Mutex<Storage>>, scrapbook_id: i32) -> Result<String> {
    let storage = storage.lock().await;
    let tree = storage.get_scrapbook_node_tree(scrapbook_id)?;
    let json = serde_json::to_string_pretty(&tree)?;
    Ok(json)
}

fn with_db(
    storage: Arc<Mutex<Storage>>,
) -> impl Filter<Extract = (Arc<Mutex<Storage>>,), Error = Infallible> + Clone {
    warp::any().map(move || storage.clone())
}

#[tokio::main]
pub async fn serve(storage: Storage) {
    let storage = Arc::new(Mutex::new(storage));

    let scrapbook_list = warp::path("scrapbooks")
        .and(with_db(storage.clone()))
        .and_then(|storage: Arc<Mutex<Storage>>| async move {
            list_scrapbooks(&storage)
                .await
                .map_err(IntoRejection::into_rejection)
        });

    let scrapbook_tree = warp::path!("scrapbook" / i32)
        .and(with_db(storage.clone()))
        .and_then(|id, storage| async move {
            get_scrapbook_tree(&storage, id)
                .await
                .map_err(IntoRejection::into_rejection)
        });

    warp::serve(scrapbook_list.or(scrapbook_tree))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
