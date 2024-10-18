use futures_util::stream::StreamExt;
use r2r::std_msgs::msg::String;
use r2r::QosProfile;

pub async fn run_subscriber() -> Result<(), Box<dyn std::error::Error>> {
    // ROS 2 컨텍스트와 노드 초기화
    let context = r2r::Context::create()?;
    let mut node = r2r::Node::create(context, "rust_subscriber_node", "")?;

    // 서브스크라이버 초기화
    let mut subscriber = node.subscribe::<String>("/rust_pub", QosProfile::default())?;

    println!("Subscriber is running. Waiting for messages...");

    // 메시지 처리를 위한 비동기 태스크 생성
    tokio::task::spawn(async move {
        while let Some(msg) = subscriber.next().await {
            println!("Received: '{}'", msg.data);
        }
    });

    // 노드 스핀
    loop {
        node.spin_once(std::time::Duration::from_millis(100));
    }
}
