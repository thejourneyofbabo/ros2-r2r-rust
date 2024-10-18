use r2r::std_msgs::msg::String;
use r2r::QosProfile;
use std::time::Duration;

pub async fn run_publisher() -> Result<(), Box<dyn std::error::Error>> {
    // ROS 2 컨텍스트와 노드 초기화
    let ctx = r2r::Context::create()?;
    let mut node = r2r::Node::create(ctx, "rust_publisher_node", "")?;

    let qos = QosProfile::default();
    let publisher = node.create_publisher::<String>("/rust_pub", qos)?;

    let mut count = 0;
    loop {
        let msg = String {
            data: format!("Hello from Rust! Count: {}", count),
        };
        publisher.publish(&msg)?;
        println!("Publishing: '{}'", msg.data);
        count += 1;
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
