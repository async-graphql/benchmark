use async_graphql::*;
use async_std::task;

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

const Q: &str = r#"{
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
}"#;

lazy_static::lazy_static! {
    static ref S: Schema<QueryRoot, EmptyMutation, EmptySubscription> = Schema::new(QueryRoot, EmptyMutation, EmptySubscription);
}

pub fn run() {
    task::block_on(async {
        S.execute(Q).await.unwrap();
    });
}
