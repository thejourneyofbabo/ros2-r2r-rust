use futures_util::stream::StreamExt;
use r2r::std_msgs::msg::String;
use r2r::QosProfile;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = r2r::Context::create()?;
    let mut node = r2r::Node::create(ctx, "rust_subscriber_node", "")?;
    let qos = QosProfile::default();

    let mut subscriber = node.subscribe::<String>("/rust_pub", qos)?;

    tokio::task::spawn(async move {
        while let Some(msg) = subscriber.next().await {
            println!("Received: '{}'", msg.data);
        }
    });

    loop {
        node.spin_once(std::time::Duration::from_millis(100));
    }
}
