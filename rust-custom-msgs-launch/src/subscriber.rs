use futures::stream::StreamExt;
use r2r;
use r2r::tutorial_interfaces::msg::Num;
use r2r::QosProfile;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = r2r::Context::create()?;
    let mut node = r2r::Node::create(ctx, "rust_subscriber_node", "")?;
    let qos = QosProfile::default();
    let mut subscriber = node.subscribe::<Num>("topic", qos)?; // 토픽 이름을 "/num"으로 변경

    loop {
        tokio::select! {
            msg = subscriber.next() => {
                if let Some(msg) = msg {
                    println!("Received: {}", msg.num);
                }
            }
            _ = tokio::time::sleep(std::time::Duration::from_millis(100)) => {
                node.spin_once(std::time::Duration::from_millis(0));
            }
        }
    }
}
