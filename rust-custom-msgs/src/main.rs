use futures::{future, StreamExt};
use r2r;
use r2r::tutorial_interfaces::msg::Num;
use r2r::tutorial_interfaces::srv::AddThreeInts;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = r2r::Context::create()?;
    let mut node = r2r::Node::create(ctx, "rust_node", "")?;

    println!("node name: {}", node.name()?);
    println!(
        "node fully qualified name: {}",
        node.fully_qualified_name()?
    );

    // Num 메시지 생성
    let num_msg = Num { num: 42 };
    println!("Num message: {:?}", num_msg);

    // 서비스 생성
    let service = node
        .create_service::<AddThreeInts::Service>("/add_three_ints", r2r::QosProfile::default())?;
    tokio::task::spawn(async move {
        service
            .for_each(|req| {
                println!(
                    "Request received: a={}, b={}, c={}",
                    req.message.a, req.message.b, req.message.c
                );
                let resp = AddThreeInts::Response {
                    sum: req.message.a + req.message.b + req.message.c,
                };
                req.respond(resp).expect("Could not send response");
                future::ready(())
            })
            .await
    });

    loop {
        node.spin_once(std::time::Duration::from_millis(100));
    }
}
