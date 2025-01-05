use log::{LevelFilter, info};
use env_logger::Builder;
use std::io::Write;
use chrono::Local;

pub fn init_logger() {
    Builder::new()
        .format(|buf, record| {
            writeln!(buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();
    
    info!("Logger initialized");
}