mod utils;

use active_win_pos_rs::{get_active_window, ActiveWindow};
#[allow(unused)]
use std::fs::File;
#[allow(unused)]
use std::io::Write;
use std::{collections::HashMap, default};
use tokio::time::{self, Duration};
use utils::Second;

impl Second for u64 {
    fn seconds_to_hms(&self) -> (u64, u64, u64) {
        let hours = self / 3600;
        let minutes = (self % 3600) / 60;
        let seconds = self % 60;
        (hours, minutes, seconds)
    }
}

pub async fn start_deamon() -> () {
    // Initialisiere den Daemon
    println!("Daemon gestartet...");

    // Intervall für die Prozessüberwachung (z.B. jede Sekunde)
    let mut interval = time::interval(Duration::from_secs(1));

    // Prozessdaten speichern
    let mut app_usage: HashMap<String, u64> = HashMap::new();
    let mut sec_past: u32 = 0;
    loop {
        sec_past += 1;
        println!("Seconds past: {sec_past}");

        interval.tick().await;
        let active_window = get_active_window();
        if active_window == Err(()) {
            continue;
        }

        let current_window_name = active_window.unwrap().app_name;
        *app_usage.entry(current_window_name).or_insert(0) += 1;

        println!("{:?}", app_usage)
    }
}
fn log_data(app_usage: HashMap<String, u64>) {
    let app_usage = app_usage
        .values()
        .map(|sec| {
            let secs = sec.seconds_to_hms();
            format!("{}.{}.{}", secs.0, secs.1, secs.2)
        })
        .collect::<HashMap<String, String>>();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_deamon() {
        start_deamon();
    }
}
