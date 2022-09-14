use chrono::{DateTime, Utc};
use firestore::*;
use serde::{Deserialize, Serialize};

pub fn config_env_var(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|e| format!("{}: {}", name, e))
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Test1(pub u8);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Test1i(pub Test1);

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Test2 {
    some_id: String,
    some_bool: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestEnum {
    TestChoice,
    TestWithParam(String),
    TestWithMultipleParams(String, String),
    TestWithStruct(Test2),
}

// Example structure to play with
#[derive(Debug, Clone, Deserialize, Serialize)]
struct MyTestStructure {
    some_id: String,
    some_string: String,
    some_num: u64,
    #[serde(with = "firestore::serialize_as_timestamp")]
    created_at: DateTime<Utc>,
    test1: Test1,
    test1i: Test1i,
    test11: Option<Test1>,
    test2: Option<Test2>,
    test3: Vec<Test2>,
    test4: TestEnum,
    test5: (TestEnum, TestEnum),
    test6: TestEnum,
    test7: TestEnum,
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

    const TEST_COLLECTION_NAME: &'static str = "test-ts1";

    let my_struct = MyTestStructure {
        some_id: "test-1".to_string(),
        some_string: "Test".to_string(),
        some_num: 41,
        created_at: Utc::now(),
        test1: Test1(1),
        test1i: Test1i(Test1(1)),
        test11: Some(Test1(1)),
        test2: Some(Test2 {
            some_id: "test-1".to_string(),
            some_bool: Some(true),
        }),
        test3: vec![
            Test2 {
                some_id: "test-2".to_string(),
                some_bool: Some(false),
            },
            Test2 {
                some_id: "test-2".to_string(),
                some_bool: Some(true),
            },
        ],
        test4: TestEnum::TestChoice,
        test5: (TestEnum::TestChoice, TestEnum::TestChoice),
        test6: TestEnum::TestWithMultipleParams("ss".to_string(), "ss".to_string()),
        test7: TestEnum::TestWithStruct(Test2 {
            some_id: "test-2".to_string(),
            some_bool: Some(true),
        }),
    };

    // Remove if it already exist
    db.delete_by_id(TEST_COLLECTION_NAME, &my_struct.some_id)
        .await?;

    // Let's insert some data
    db.create_obj(TEST_COLLECTION_NAME, &my_struct.some_id, &my_struct)
        .await?;

    // Update some field in it
    let updated_obj = db
        .update_obj(
            TEST_COLLECTION_NAME,
            &my_struct.some_id,
            &MyTestStructure {
                some_num: my_struct.some_num + 1,
                some_string: "updated-value".to_string(),
                ..my_struct.clone()
            },
            Some(paths!(MyTestStructure::{
                some_num,
                some_string
            })),
        )
        .await?;

    println!("Updated object: {:?}", updated_obj);

    // Get object by id
    let find_it_again: MyTestStructure =
        db.get_obj(TEST_COLLECTION_NAME, &my_struct.some_id).await?;

    println!("Should be the same: {:?}", find_it_again);

    // Query our data
    let objects: Vec<MyTestStructure> = db
        .query_obj(
            FirestoreQueryParams::new(TEST_COLLECTION_NAME.into()).with_filter(
                FirestoreQueryFilter::Compare(Some(FirestoreQueryFilterCompare::Equal(
                    path!(MyTestStructure::some_num),
                    find_it_again.some_num.into(),
                ))),
            ),
        )
        .await?;

    println!("Now in the list: {:?}", objects);

    Ok(())
}
