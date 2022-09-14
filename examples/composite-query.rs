use chrono::{DateTime, Utc};
use firestore::*;
use futures_util::stream::BoxStream;
use serde::{Deserialize, Serialize};
use tokio_stream::StreamExt;

pub fn config_env_var(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|e| format!("{}: {}", name, e))
}

// Example structure to play with
#[derive(Debug, Clone, Deserialize, Serialize)]
struct MyTestStructure {
    some_id: String,
    some_string: String,
    one_more_string: String,
    some_num: u64,
    created_at: DateTime<Utc>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Logging with debug enabled
    let subscriber = tracing_subscriber::fmt()
        .with_env_filter("firestore=debug")
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    // Create an instance
    let db = FirestoreDb::new(&config_env_var("PROJECT_ID")?).await?;

    const TEST_COLLECTION_NAME: &'static str = "test";

    println!("Querying a test collection as a stream");
    // Query as a stream our data
    let mut object_stream: BoxStream<MyTestStructure> = db
        .stream_query_obj(
            FirestoreQueryParams::new(TEST_COLLECTION_NAME.into()).with_filter(
                FirestoreQueryFilter::Composite(FirestoreQueryFilterComposite::new(vec![
                    FirestoreQueryFilter::Compare(Some(FirestoreQueryFilterCompare::Equal(
                        path!(MyTestStructure::some_num),
                        42.into(),
                    ))),
                    FirestoreQueryFilter::Compare(Some(
                        FirestoreQueryFilterCompare::LessThanOrEqual(
                            path!(MyTestStructure::created_at),
                            Utc::now().into(),
                        ),
                    )),
                ])),
            ),
        )
        .await?;

    while let Some(object) = object_stream.next().await {
        println!("Object in stream: {:?}", object);
    }

    Ok(())
}
