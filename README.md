In this function, I expect d to be very close to 100ms

```rust
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
```

But on my PC, the results are always between 110 ~ 120ms:

```
cargo run --release
    Finished release [optimized] target(s) in 0.03s
     Running `target\release\tokio-timer-delay.exe`
d = 120.1023ms
```
