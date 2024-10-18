use futures_util::stream::StreamExt;
use r2r::geometry_msgs::msg::Twist;
use r2r::QosProfile;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = r2r::Context::create()?;
    let mut node = r2r::Node::create(ctx, "turtlesim_twist_subscriber", "")?;
    let qos = QosProfile::default();

    let mut subscriber = node.subscribe::<Twist>("/turtle1/cmd_vel", qos)?;

    println!("Listening for Twist messages on /turtle1/cmd_vel...");

    tokio::task::spawn(async move {
        while let Some(msg) = subscriber.next().await {
            println!("Received Twist message:");
            println!(
                "  Linear: x={}, y={}, z={}",
                msg.linear.x, msg.linear.y, msg.linear.z
            );
            println!(
                "  Angular: x={}, y={}, z={}",
                msg.angular.x, msg.angular.y, msg.angular.z
            );
        }
    });

    loop {
        node.spin_once(std::time::Duration::from_millis(100));
    }
}
