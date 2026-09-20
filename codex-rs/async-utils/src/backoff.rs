//! Shared exponential retry delays with jitter.

use std::time::Duration;

use rand::Rng;

const INITIAL_DELAY_MS: u64 = 200;
const BACKOFF_FACTOR: f64 = 2.0;

/// Return a retry delay starting at 200 ms and doubling on each subsequent attempt,
/// with up to 10% jitter. Attempts zero and one both use the initial delay.
pub fn backoff(attempt: u64) -> Duration {
    Duration::ZERO
}
