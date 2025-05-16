use std::sync::mpsc::{self, Sender};
use std::thread;
use std::time::Duration;

/// Startet einen minimalistischen ASCII-Spinner in einem Hintergrundthread.
///
/// Gibt alle X Millisekunden einen neuen Frame auf stdout aus (mit `\r`).
/// Die Frequenz wird über `ticks_per_second` gesteuert.
///
/// # Beispiel
/// ```
/// let stop = ascii_spinner::start_spinner(8); // 8 Ticks pro Sekunde
/// // ... lange Operation ...
/// let _ = stop.send(());
/// ```
pub fn start_spinner(ticks_per_second: u64) -> Sender<()> {
    let (tx, rx) = mpsc::channel();
    let frames = vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let clamped = ticks_per_second.clamp(1, 20);
    let interval = Duration::from_millis(1000 / clamped);

    thread::spawn(move || {
        let mut idx = 0;
        while rx.try_recv().is_err() {
            print!("\r[{}] läuft ...", frames[idx % frames.len()]);
            let _ = std::io::Write::flush(&mut std::io::stdout());
            idx += 1;
            thread::sleep(interval);
        }
        print!("\r                \r"); // Spinner löschen
        let _ = std::io::Write::flush(&mut std::io::stdout());
    });

    tx
} 
