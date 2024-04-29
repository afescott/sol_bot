use std::time::Duration;

pub struct Args {
    pub sol_private_key: String,
    pub interval_time: Duration,
    market_cap_limit: i32,
    full_diluted_limit: i32,
}
