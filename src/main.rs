use std::time::{Duration};
use tokio::time::{delay_until, Instant};
use tokio::runtime::Builder;

fn main() {
    unsafe {
        winapi::um::processthreadsapi::SetPriorityClass(
            winapi::um::processthreadsapi::GetCurrentProcess(),
            winapi::um::winbase::REALTIME_PRIORITY_CLASS,
        );
    }

    let mut rt = Builder::new().enable_all().build().unwrap();

    rt.block_on(async {
        let mut values = vec![];
        for _ in 0..10 {
            let real_now = Instant::now();
            delay_until(real_now + Duration::from_millis(500)).await;
            values.push((Instant::now() - real_now).as_millis());
        }
        dbg!(values);
    });
}