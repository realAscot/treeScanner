# Changelog treeScanner (Rust)

- **2025-05-17 - v1.0.0**
  - **Geändert:**  
    - [x] Komplette Neuumsetzung des TreeScanner-Tools in Rust  
    - [x] Aufbau einer modularen Projektstruktur (`treebuilder.rs`, `args.rs`, `loader.rs`, `ascii_spinner.rs`, `logger.rs`)  
    - [x] CLI-Parsing mit `clap` umgesetzt, inkl. Aliase und Hilfetexte  
    - [x] Fortschrittsanzeige mit ASCII-Spinner als wiederverwendbares Modul integriert  
    - [x] Unterstützung für `.treescanner.conf` im Home-Verzeichnis eingebaut (inkl. Auto-Erzeugung und TOML-Validierung)  
    - [x] Optionale Parameter wie `--max-depth`, `--viewonly`, `--ignore`, `--align-comments` implementiert  
    - [x] Fallback-Logik: CLI-Eingaben überschreiben Konfiguration bei Kollision  
    - [x] Ausgabe wahlweise in Konsole oder Datei (Standard: `tree.txt`)  
    - [x] Unit-Tests für Konfiguration und Kommentar-Ausrichtung hinzugefügt (`tests/config_tests.rs`)  
    - [x] Fehlerausgaben für ungültige Konfiguration oder fehlende Verzeichnisse eingebaut  
    - [x] Makefile mit Windows-Kompatibilität für Build + Copy-Ziel erstellt  
    - [x] Icon und Metadaten in .exe eingebettet via `build.rs` und `rc.exe`  

  - **Hinzugefügt:**  
    - [x] CLI-Argument `--ignore` unterstützt sowohl Kommaseparierung (CLI) als auch Listen (TOML)  
    - [x] Debug- und Silent-Modus zur Steuerung der Ausgaben  
    - [x] Unterstützung für Konfigurationsparameter `output`, `viewonly`, `language`, `align_comments`  
    - [x] Automatische Erstellung einer gültigen Standardkonfiguration, wenn keine `.treescanner.conf` vorhanden ist  
    - [x] Fehler-Feedback mit TOML-Zeilenangaben bei ungültiger Konfiguration  

  - **Entfernt:**  
    - [x] Python-Version vollständig ersetzt  
    - [x] Temporäre Hilfsfunktionen aus der Portierungsphase entfernt  
