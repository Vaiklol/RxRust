

use std::time::{Duration, Instant};
use std::num::Int;
use reactive::Publisher;
use reactor::StreamBuf;
use std::thread::sleep;

/// This spins in a loop at a specific frame rate
/// it sleeps between tries
/// rate is the max number of loops per second that
/// it makes There is no minimum number of loops
pub fn fixed_loop<F>(rate: u64, mut f: F) where F : FnMut() -> bool {
    debug!("Starting loop");
    let loop_time_ns = 1_000_000_000u128 / rate;
    let mut run = true;

    loop {
        let start = Instant::now();
        if !f() {
            break;
        }
        let span = start.elapsed();
        if let Some(diff) = loop_time_ns.checked_sub(span.as_nanos()) {
            let ts = Duration::from_nanos(diff as u64);
            sleep(ts);
        }
    }

    debug!("Done with loop");
}

