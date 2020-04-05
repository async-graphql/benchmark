mod async_graphql_benchmark;
mod juniper_benchmark;

#[async_std::main]
async fn main() {
    juniper_benchmark::run().await;
    async_graphql_benchmark::run().await;
}
