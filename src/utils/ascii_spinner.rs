use std::sync::mpsc::{self, Sender};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use std::io::Write;

/// Startet einen minimalistischen ASCII-Spinner in einem Hintergrundthread.
///
/// Gibt alle X Millisekunden einen neuen Frame auf stdout aus (mit `\r`).
/// Die Frequenz wird über `ticks_per_second` gesteuert.
///
/// # Beispiel
/// ```no_run
/// # use treescanner::utils::ascii_spinner;
/// let (stop, handle) = ascii_spinner::start_spinner(8); // 8 Ticks pro Sekunde
/// let _ = stop.send(());
/// let _ = handle.join();
/// ```
pub fn start_spinner(ticks_per_second: u64) -> (Sender<()>, JoinHandle<()>) {
    let (tx, rx) = mpsc::channel();
    let frames = vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let clamped = ticks_per_second.clamp(1, 20);
    let interval = Duration::from_millis(1000 / clamped);

    let handle = thread::spawn(move || {
        let mut idx = 0;
        while rx.try_recv().is_err() {
            print!("\r[{}] läuft ...", frames[idx % frames.len()]);
            let _ = std::io::stdout().flush();
            idx += 1;
            thread::sleep(interval);
        }
        // Spinnerzeile zuverlässig löschen
        print!("\rr\x1B[2K\r"); // ANSI: ganze Zeile löschen
        let _ = std::io::stdout().flush();
    });

    (tx, handle)
}
