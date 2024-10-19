use r2r;
use r2r::tutorial_interfaces::msg::Num;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = r2r::Context::create()?;
    let mut node = r2r::Node::create(ctx, "num_publisher", "")?;

    // 퍼블리셔 생성
    let publisher = node.create_publisher::<Num>("topic", r2r::QosProfile::default())?;

    let mut count = 0;

    loop {
        // 메시지 생성
        let msg = Num { num: count };
        println!("Publishing: {:?}", msg);
        publisher.publish(&msg)?; // &msg로 참조를 전달

        // 1초 대기
        tokio::time::sleep(Duration::from_millis(500)).await;
        count += 1;
    }
}
