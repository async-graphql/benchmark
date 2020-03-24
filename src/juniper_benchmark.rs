use async_std::task;
use juniper::RootNode;
use std::sync::Arc;
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

    async fn obj(&self) -> MyObj {
        MyObj
    }
}

pub async fn run() {
    let s = Instant::now();
    let schema = Arc::new(RootNode::new(
        QueryRoot,
        juniper::EmptyMutation::new(),
        juniper::EmptySubscription::new(),
    ));
    let mut jobs = Vec::new();

    for _ in 0..4 {
        let schema = schema.clone();
        let handle = task::spawn(async move {
            for _ in 0..100000i32 {
                juniper::execute(
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
                    None,
                    &schema,
                    &Default::default(),
                    &(),
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
    println!("juniper: {} ms", s.elapsed().as_millis());
}
