use r2r::std_msgs::msg::String;
use r2r::QosProfile;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let context = r2r::Context::create().unwrap();
    let mut node = r2r::Node::create(context, "rust_publisher_node", "").unwrap();

    let qos = QosProfile::default();
    let publisher = node.create_publisher::<String>("/rust_pub", qos).unwrap();

    let mut count = 0;
    loop {
        let msg = String {
            data: format!("Hello from Rust! Count: {}", count),
        };
        publisher.publish(&msg).unwrap();
        println!("Publishing: '{}'", msg.data);
        count += 1;
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
