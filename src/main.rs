use std::time::Instant;

#[tokio::main]
async fn main() {
    let t = Instant::now();
    let delay1 = tokio::time::delay_for(std::time::Duration::from_millis(50));
    delay1.await;
    let delay2 = tokio::time::delay_for(std::time::Duration::from_millis(50));
    delay2.await;
    println!("d = {:?}", Instant::now() - t);
}