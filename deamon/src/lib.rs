use active_win_pos_rs::{get_active_window, ActiveWindow};
use std::collections::HashMap;
#[allow(unused)]
use std::fs::File;
#[allow(unused)]
use std::io::Write;
use tokio::time::{self, Duration};
pub async fn start_deamon() -> () {
    // Initialisiere den Daemon
    println!("Daemon gestartet...");

    // Intervall für die Prozessüberwachung (z.B. jede Sekunde)
    let mut interval = time::interval(Duration::from_secs(1));

    // Prozessdaten speichern
    let mut app_usage: HashMap<String, u64> = HashMap::new();
    loop {
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
fn log_data() {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_deamon() {
        start_deamon();
    }
}
