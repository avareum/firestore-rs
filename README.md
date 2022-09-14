[![Cargo](https://img.shields.io/crates/v/firestore.svg)](https://crates.io/crates/firestore)
![tests and formatting](https://github.com/abdolence/firestore-rs/workflows/tests%20&amp;%20formatting/badge.svg)
![security audit](https://github.com/abdolence/firestore-rs/workflows/security%20audit/badge.svg)

# Firestore for Rust

Library provides a simple API for Google Firestore:
- Create or update documents using Rust structures and Serde; 
- Support for:
  - Querying/streaming docs/objects;
  - Listing documents/objects (and auto pages scrolling support);
  - Listening changes from Firestore;
  - Transactions;
- Full async based on Tokio runtime;
- Macro that helps you use JSON paths as references to your structure fields;
- Implements own Serde serializer to Firestore values;
- Supports for Firestore timestamp with `#[serde(with)]`
- Google client based on [gcloud-sdk library](https://github.com/abdolence/gcloud-sdk-rs) 
  that automatically detects GKE environment or application default accounts for local development;

## Quick start

Cargo.toml:
```toml
[dependencies]
firestore = "0.10"
```

Example code:
```rust

    // Create an instance
    let db = FirestoreDb::new(&config_env_var("PROJECT_ID")?).await?;

    const TEST_COLLECTION_NAME: &'static str = "test";

    let my_struct = MyTestStructure {
        some_id: "test-1".to_string(),
        some_string: "Test".to_string(),
        one_more_string: "Test2".to_string(),
        some_num: 42,
    };

    // Remove if it already exist
    db.delete_by_id(
        TEST_COLLECTION_NAME,
        &my_struct.some_id,
    ).await?;

    // Let's insert some data
    db.create_obj(
        TEST_COLLECTION_NAME,
        &my_struct.some_id,
        &my_struct,
    ).await?;

    // Update some field in it
    let updated_obj = db.update_obj(
        TEST_COLLECTION_NAME,
        &my_struct.some_id,
        &MyTestStructure {
            some_num: my_struct.some_num + 1,
            some_string: "updated-value".to_string(),
            ..my_struct.clone()
        },
        Some(
            paths!(MyTestStructure::{
                some_num,
                some_string
            })
        ),
    ).await?;

    println!("Updated object: {:?}", updated_obj);

    // Get object by id
    let find_it_again: MyTestStructure = db.get_obj(
        TEST_COLLECTION_NAME,
        &my_struct.some_id,
    ).await?;

    println!("Should be the same: {:?}", find_it_again);

    // Query our data
    let objects: Vec<MyTestStructure> = db.query_obj(
        FirestoreQueryParams::new(
            TEST_COLLECTION_NAME.into()
        ).with_filter(
            FirestoreQueryFilter::Compare(Some(
                FirestoreQueryFilterCompare::Equal(
                    path!(MyTestStructure::some_num),
                    find_it_again.some_num.into(),
                ),
            ))
        )
    ).await?;

    println!("Now in the list: {:?}", objects);
```

All examples available at [examples](examples) directory.

To run example use it with environment variables:
```
# PROJECT_ID=<your-google-project-id> cargo run --example simple-crud
```

## Timestamps support
By default, the types such as DateTime<Utc> serializes as a string
to Firestore (while deserialization works from Timestamps and Strings).
To change it to support Timestamp natively use `#[serde(with)]`:

```
#[derive(Debug, Clone, Deserialize, Serialize)]
struct MyTestStructure {
    #[serde(with = "firestore::serialize_as_timestamp")]
    created_at: DateTime<Utc>,
}
```
This will change it only for firestore serialization and it still serializes as string
to JSON (so you can reuse the same model for JSON and Firestore).

## Licence
Apache Software License (ASL)

## Author
Abdulla Abdurakhmanov
