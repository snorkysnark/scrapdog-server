use crate::storage::Storage;
use anyhow::Result;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::Mutex;
use warp::Filter;

mod rejections {
    use warp::{reject::Reject, Rejection};

    struct AnyhowRejecion(anyhow::Error);
    impl Reject for AnyhowRejecion {}
    impl std::fmt::Debug for AnyhowRejecion {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.fmt(fmt)
        }
    }

    pub trait IntoRejection {
        fn into_rejection(self) -> Rejection;
    }
    impl IntoRejection for anyhow::Error {
        fn into_rejection(self) -> Rejection {
            warp::reject::custom(AnyhowRejecion(self))
        }
    }
}
use rejections::IntoRejection;

async fn list_scrapbooks(storage: &Arc<Mutex<Storage>>) -> Result<String> {
    let storage = storage.lock().await;
    let scrapbooks = storage.list_scrapbooks()?;
    let json = serde_json::to_string_pretty(&scrapbooks)?;
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
    let buckets_dir = storage.get_bucket_path().to_owned();
    let storage = Arc::new(Mutex::new(storage));

    let scrapbook_list = warp::path("scrapbooklist")
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

    let files = warp::path("files").and(warp::fs::dir(buckets_dir));

    warp::serve(scrapbook_list.or(scrapbook_tree).or(files))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
