# TreeScanner CLI Verzeichnisscanner

![TreeScanner-Logo](./media/logo-treescanner_512x512.png)  

**TreeScanner** ist ein leichtgewichtiges, portables CLI-Tool zur rekursiven Analyse von Verzeichnisstrukturen.
Es erzeugt eine klar strukturierte ASCII-Ausgabe und eignet sich hervorragend für Dokumentation, Debugging oder Buildsysteme.

> 🔧 Diese Version ist eine komplette Neuentwicklung in **Rust** und ersetzt das ursprüngliche Python-Projekt:  
> ➜ [treeScannerASCII (Python)](https://github.com/realAscot/treeScannerASCII)

Der original treeScanner in Python ist unter <https://github.com/realAscot/treeScannerASCII> zu finden. Dieser ist auch als Python-Modul zu verwenden.

---

## Inhalt

- [TreeScanner CLI Verzeichnisscanner](#treescanner-cli-verzeichnisscanner)
  - [Inhalt](#inhalt)
  - [Beschreibung](#beschreibung)
  - [Installation](#installation)
    - [Über .zip Archiv](#über-zip-archiv)
    - [Installer](#installer)
  - [Struktur](#struktur)
  - [✨ Features](#-features)
  - [▶️ Verwendung](#️-verwendung)
  - [🖼 Beispielausgabe](#-beispielausgabe)
  - [⚙️ Konfiguration `.treescanner.conf`](#️-konfiguration-treescannerconf)
    - [🔍 Ort](#-ort)
    - [📘 Format](#-format)
    - [📝 Format (.toml)](#-format-toml)
  - [Lizenz](#lizenz)
    - [Eingesetzte Libraries (MIT-kompatibel):](#eingesetzte-libraries-mit-kompatibel)
  - [💬 Kontakt](#-kontakt)

---

## Beschreibung

Der treeScanner.exe ist ursprünglich als ein Tool entwickelt worden mit dem man Verzeichnisstrukturen für Dokumentationen erzeugen konnte.  
TreeScanner durchsucht Verzeichnisse rekursiv, filtert optional bestimmte Ordner aus und gibt eine **strukturierte ASCII-Baumdarstellung** mit Icons und optional ausgerichteten Kommentaren aus.  
Er eignet sich für technische Dokumentationen, Versionskontrollen, Release-Skripte und CI/CD-Prozesse.

---

## Installation

### Über .zip Archiv

### Installer

Der Installer `treeScanner-Setup.exe` der jeweils die aktuelle Version auf GitHub enthällt, bietet Dir an das Programm im Windows-Verzeichnis zu installieren.
Dies ist zwar unüblich aber

---

## Struktur

**GEPLANTE STRUKTUR (DEV)**  

```plaintext

📁 treeScanner                           #
├── 📁 .cargo                            #
│   └── 📄 config.toml                   #
├── 📁 .vscode                           #
│   └── 📄 tasks.json                    #
├── 📁 media                             #
│   ├── 📄 logo-treescanner.png          #
│   └── 📄 logo-treescanner_512x512.png  #
├── 📁 resources                         #
│   ├── 📄 icon.ico                      #
│   └── 📄 version.rc                    #
├── 📁 src                               #
│   ├── 📁 app                           #
│   │   ├── 📄 mod.rs                    #
│   │   └── 📄 treebuilder.rs            #
│   ├── 📁 config                        #
│   │   ├── 📄 args.rs                   #
│   │   ├── 📄 loader.rs                 #
│   │   └── 📄 mod.rs                    #
│   ├── 📁 utils                         #
│   │   ├── 📄 ascii_spinner.rs          #
│   │   ├── 📄 logger.rs                 #
│   │   └── 📄 mod.rs                    #
│   ├── 📄 lib.rs                        #
│   └── 📄 main.rs                       #
├── 📁 tests                             #
│   ├── 📄 config_tests.rs               #
│   └── 📄 treebuilder_tests.rs          #
├── 📄 .gitignore                        #
├── 📄 build.rs                          #
├── 📄 Cargo.lock                        #
├── 📄 Cargo.toml                        #
├── 📄 CHANGELOG.md                      #
├── 📄 LICENSE                           #
├── 📄 Makefile                          #
├── 📄 README.md                         #
└── 📄 VERSION                           #

```

---

## ✨ Features

- 📁 ASCII-Baumdarstellung mit Icons (📁, 📄)
- 🚫 Ignorierliste per CLI oder Konfig-Datei
- ⏫ Limit für Tiefe (`--max-depth`) und Dateianzahl pro Verzeichnis
- 📄 Ausgabe in Datei oder Konsole
- ⚙ Konfigurierbar via `~/.treescanner.conf`
- 🌀 Fortschrittsanzeige beim Scannen (Spinner)
- 💬 Optionale Kommentarspalte (`--align-comments`)
- 🧪 Getestete Komponenten (unit-tested)
- 🔕 Silent-Modus (`--quiet`)
- 🛠 Portable Binary (`.exe`) ohne externe Abhängigkeiten

---

## ▶️ Verwendung

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

## 🖼 Beispielausgabe

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

## ⚙️ Konfiguration `.treescanner.conf`

### 🔍 Ort

Standardmäßig gesucht im **Benutzerverzeichnis**:

```plaintext
Windows:  C:\Users\<Benutzername>\.treescanner.conf
Linux:    /home/<user>/.treescanner.conf
```

### 📘 Format

### 📝 Format (.toml)

```toml
max_depth = 3
max_files_per_dir = 100
ignore = [".git", "target", ".vscode"]
output = "tree.txt"
viewonly = false
align_comments = false
```

- CLI-Einstellungen überschreiben Konfigurationswerte bei Kollision  
- Die Datei wird beim ersten Start automatisch erzeugt, falls sie fehlt  
- Der Pfad ist **nicht fest kodiert**, sondern dynamisch via `dirs::home_dir()` ermittelt  

---

## Lizenz

Dieses Projekt steht unter der [MIT-Lizenz](./LICENSE).

### Eingesetzte Libraries (MIT-kompatibel):

| Crate           | Lizenz     |
|-----------------|------------|
| `clap`          | MIT/Apache |
| `dirs`          | MIT/Apache |
| `serde`         | MIT/Apache |
| `serde_derive`  | MIT/Apache |
| `toml`          | MIT/Apache |
| `tempfile`      | MIT/Apache (nur für Tests) |
| `console`       | MIT        |

⚠️ Alle eingebundenen Libraries sind **MIT- oder Apache-2.0-kompatibel** und dürfen ohne Einschränkungen in proprietären oder Open-Source-Projekten verwendet werden.

siehe [LICENSE](./LICENSE)  

---

## 💬 Kontakt

**Adam Skotarczak**  
✉ [adam@skotarczak.net](mailto:adam@skotarczak.net)  
🔗 [realAscot auf GitHub](https://github.com/realAscot)
