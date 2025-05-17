# TreeScanner CLI Verzeichnisscanner

![TreeScanner-Logo](./media/logo-treescanner_512x512.png)  

TreeScanner ist ein leichtgewichtiges CLI-Tool zur Darstellung von Verzeichnisstrukturen als ASCII-Baum. Dieses Tool entstand im Rahmen meines persönlichen Projekts, systemnahe Werkzeuge von C nach Rust zu migrieren.  

Der original treeScanner in Python ist unter <https://github.com/realAscot/treeScannerASCII> zu finden. Dieser ist auch als Python-Modul zu verwenden.

---

## Inhalt

- [TreeScanner CLI Verzeichnisscanner](#treescanner-cli-verzeichnisscanner)
  - [Inhalt](#inhalt)
  - [Struktur](#struktur)
  - [Features](#features)
  - [Verwendung](#verwendung)
  - [Beispielausgabe](#beispielausgabe)
  - [Lizenz](#lizenz)

---

## Struktur

**GEPLANTE STRUKTUR (DEV)**  

```plaintext
src/
├── main.rs                → CLI-Einstieg
├── app/
│   ├── mod.rs
│   └── treebuilder.rs     → Verzeichnisbaum erstellen
├── config/
│   └── args.rs            → Parameterübergabe & Konfig
├── utils/
│   ├── ascii_spinner.rs   → Fortschrittsanzeige
│   └── logger.rs
├── tests/                 → Integrationstests
├── media/                 → Logos / Assets
├── resources/             → .conf-Template, Icons, Versioninfo
```

---

## Features

- 📁 ASCII-Baumstruktur mit Icons (📁, 📄)
- 📂 Max. Tiefe & Datei-Anzahl konfigurierbar (`--max-depth`, `--max-files-per-dir`)
- 🚫 Ignorieren von Verzeichnissen (`--ignore .git,target`)
- 💬 Optional ausrichtbare Kommentarspalte (`--align-comments`)
- ⚙ Konfigurierbar per CLI oder `~/.treescanner.conf`
- 🌀 Fortschrittsanzeige während des Scans
- 🛠 `--quiet`, `--debug`, `--viewonly`, `--output` u. a.
- 🧪 Tests, strukturierter Build, Markdown-fähige Ausgabe

---

## Verwendung

```bash
# Einfacher Scan (aktuelles Verzeichnis)
./treescanner.exe

# Mit Tiefe 3, ohne speichern
./treescanner.exe --max-depth 3 --viewonly

# Mit Kommentar-Ausrichtung
./treescanner.exe --align-comments

# Ergebnis in Datei mit anderem Pfad speichern
./treescanner.exe --output ./struktur/tree.md
```

---

## Beispielausgabe

```plaintext
📁 ./src/
├── 📄 main.rs               #
├── 📁 app/                  #
│   └── 📄 treebuilder.rs    #
└── 📁 utils/                #
    ├── 📄 ascii_spinner.rs  #
    └── 📄 logger.rs         #
```

---

## Lizenz

MIT © [Adam Skotarczak](mailto:adam@skotarczak.net) siehe [LICENSE](./LICENSE)  
