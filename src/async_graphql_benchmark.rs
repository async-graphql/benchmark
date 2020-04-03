use async_graphql::*;
use std::sync::Arc;
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
    async fn value_list(&self) -> &[i32] {
        &[1, 2, 3, 4, 5, 6, 7, 8, 9]
    }

    #[field]
    async fn obj(&self) -> MyObj {
        MyObj
    }
}

pub async fn run() {
    let s = Instant::now();
    let schema = Arc::new(Schema::new(QueryRoot, EmptyMutation, EmptySubscription));
    let mut jobs = Vec::new();

    for _ in 0..4 {
        let schema = schema.clone();
        let handle = tokio::spawn(async move {
            for _ in 0..10000i32 {
                schema
                    .execute(
                        r#"
            {
                valueI32 obj {
                    valueI32 valueList obj {
                        valueI32 valueList obj {
                            valueI32 valueList obj {
                                valueI32 valueList obj {
                                    valueI32 valueList obj {
                                        valueI32 valueList obj {
                                            valueI32 valueList obj {
                                                valueI32 valueList obj {
                                                    valueI32 valueList obj {
                                                        valueI32 valueList obj {
                                                            valueI32 valueList obj {
                                                                valueI32 valueList obj {
                                                                    valueI32 valueList obj {
                                                                        valueI32 valueList obj {
                                                                            valueI32 valueList
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }"#,
                    )
                    .await
                    .unwrap();
            }
        });
        jobs.push(handle);
    }
    for i in 0..4 {
        jobs.get_mut(i).unwrap().await;
    }

    println!("async-graphql: {} ms", s.elapsed().as_millis());
}
