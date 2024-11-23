use std::io::{stdout, Write};

use deamon::start_deamon;
use tokio::task::JoinHandle;
use tokio::time::{self, Duration};
#[tokio::main]
async fn main() {
    let handle: JoinHandle<()> = tokio::spawn(start_deamon());

    handle.await.unwrap();
}
