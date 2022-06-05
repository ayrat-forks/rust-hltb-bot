use std::str::FromStr;
use log::LevelFilter;
use simple_logger::SimpleLogger;

mod model;
mod formatting;
mod page_parsing;
mod telegram;

mod tests;
mod lambda;

const STACK_SIZE: usize = 4 * 1024 * 1024;

#[tokio::main]
async fn main() {
    configure_logging();
    telegram::run_polling().await.unwrap();

    // running actual logic in different thread so stack size could be set
    // otherwise frankenstein can fail with STACK_OVERFLOW on larger json responses
    // std::thread::Builder::new()
    //     .stack_size(STACK_SIZE)
    //     .spawn(run)
    //     .unwrap()
    //     .join()
    //     .unwrap();
}

fn run() {
    let runtime = tokio::runtime::Runtime::new().unwrap();

    let future =
        // telegram::run_polling();
        lambda::run();

    runtime.block_on(future).unwrap();
}

fn configure_logging() {
    let log_level = LevelFilter::from_str(
        &std::env::var("LOG_LEVEL").unwrap_or("Info".to_string())
    ).unwrap_or(LevelFilter::Info);
    SimpleLogger::new().with_level(log_level).init().unwrap();
}
