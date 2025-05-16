
// Importiert das Modul `utils`, das vermutlich Unterfunktionen wie Fenster oder Logging enthält
mod utils;
mod app;

fn main() {
    // Initialisiert das Logger-System (für Debug- oder Datei-Logging - nicht implementiert)
    utils::logger::init();
    app::run();
}
