// use jemallocator::Jemalloc;
//
// #[cfg(not(target_env = "msvc"))]
// #[global_allocator]
// static GLOBAL: Jemalloc = Jemalloc;
// use mimallocator::Mimalloc;

mod async_graphql_benchmark;
mod juniper_benchmark;

#[tokio::main]
async fn main() {
    juniper_benchmark::run().await;
    async_graphql_benchmark::run().await;
}
