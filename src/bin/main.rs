use chrono::Local;
use circadianlight::{gamma_function, timelike_to_hours, Config};

fn main() {
    let now = Local::now();
    let hours = timelike_to_hours(&now);
    let config = Config::default();
    let gamma = gamma_function(config)(hours);
    println!("{:.3}:{:.3}:{:.3}", gamma[0], gamma[1], gamma[2]);
}
