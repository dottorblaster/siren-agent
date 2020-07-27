extern crate systemstat;

use std::io::Error;
use systemstat::{saturating_sub_bytes, Platform, System};

fn error_string(error: Error) -> String {
    format!("error: {}", error)
}

pub fn create_instance() -> System {
    return System::new();
}

pub fn memory(system_api: &System) -> String {
    match system_api.memory() {
        Ok(mem) => format!(
            "{} used / {} ({} bytes) total ({:?})",
            saturating_sub_bytes(mem.total, mem.free),
            mem.total,
            mem.total.as_u64(),
            mem.platform_memory
        ),
        Err(error) => error_string(error),
    }
}

pub fn load_avg(system_api: &System) -> String {
    match system_api.load_average() {
        Ok(loadavg) => format!("{} {} {}", loadavg.one, loadavg.five, loadavg.fifteen),
        Err(error) => error_string(error),
    }
}

pub fn uptime(system_api: &System) -> String {
    match system_api.uptime() {
        Ok(uptime) => format!("{:?}", uptime),
        Err(error) => error_string(error),
    }
}

pub fn boot_time(system_api: &System) -> String {
    match system_api.boot_time() {
        Ok(boot_time) => format!("{:?}", boot_time),
        Err(error) => error_string(error),
    }
}
