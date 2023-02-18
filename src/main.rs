mod kafka;

use kafka::sub_to_topic;
use tokio;

#[tokio::main]
async fn main() {
    let _ = tokio::spawn(async {
        sub_to_topic("localhost:29092", "default", &["test"]).await;
    })
    .await;
}
