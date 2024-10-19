use std::process::Command;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 퍼블리셔 실행
    let publisher = tokio::spawn(async {
        Command::new("cargo")
            .args(&["run", "--bin", "publisher"])
            .status()
            .expect("Failed to start publisher");
    });

    // 서브스크라이버 실행
    let subscriber = tokio::spawn(async {
        Command::new("cargo")
            .args(&["run", "--bin", "subscriber"])
            .status()
            .expect("Failed to start subscriber");
    });

    // 두 태스크가 모두 완료될 때까지 기다림
    tokio::try_join!(publisher, subscriber)?;

    Ok(())
}
