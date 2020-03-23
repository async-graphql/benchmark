use juniper::RootNode;
use std::time::Instant;

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    async fn value_i32(&self) -> i32 {
        999
    }

    async fn obj(&self) -> MyObj {
        MyObj
    }
}

pub struct MyObj;

#[juniper::graphql_object]
impl MyObj {
    async fn value_i32(&self) -> i32 {
        999
    }

    async fn value_list(&self) -> Vec<i32> {
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9]
    }
}

pub async fn run() {
    let s = Instant::now();
    let schema = RootNode::new(
        QueryRoot,
        juniper::EmptyMutation::new(),
        juniper::EmptySubscription::new(),
    );
    for _ in 0..100000i32 {
        juniper::execute(
            "{ valueI32 obj { valueI32 valueList } }",
            None,
            &schema,
            &Default::default(),
            &(),
        )
        .await
        .unwrap();
    }
    println!("juniper: {} ms", s.elapsed().as_millis());
}
