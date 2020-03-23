use async_graphql::*;
use std::time::Instant;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    #[field]
    async fn value_i32(&self) -> i32 {
        999
    }

    #[field]
    async fn obj(&self) -> MyObj {
        MyObj
    }
}

pub struct MyObj;

#[Object]
impl MyObj {
    #[field]
    async fn value_i32(&self) -> i32 {
        999
    }

    #[field]
    async fn value_list(&self) -> Vec<i32> {
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
    }
}

pub async fn run() {
    let s = Instant::now();
    let schema = Schema::new(QueryRoot, EmptyMutation, EmptySubscription);
    for _ in 0..100000i32 {
        schema
            .query("{ valueI32 obj { valueI32 valueList } }")
            .execute()
            .await
            .unwrap();
    }
    println!("async-graphql: {} ms", s.elapsed().as_millis());
}
