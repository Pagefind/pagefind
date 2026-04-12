#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

use pagefind::runner::run_indexer;

#[tokio::main]
async fn main() {
    match run_indexer().await {
        Ok(_) => { /* success */ }
        Err(msg) => {
            eprintln!("{msg}");
            std::process::exit(1);
        }
    }
}
