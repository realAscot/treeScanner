# =======================================
# 🦀 Windows-kompatibles Makefile für TreeScanner
# =======================================

PROJECT_NAME := treescanner
BUILD_DIR := target/release
OUT_DIR := bin

# Standardziel
.PHONY: all
all: build copy

# Release-Build
.PHONY: build
build:
	cargo build --release

# Kopiere EXE in bin\
.PHONY: copy
copy:
	if not exist "$(OUT_DIR)" mkdir "$(OUT_DIR)"
	copy /Y "$(BUILD_DIR)\$(PROJECT_NAME).exe" "$(OUT_DIR)\$(PROJECT_NAME).exe"

# Lösche alles außer bin\
.PHONY: clean
clean:
	cargo clean

# Tests
.PHONY: test
test:
	cargo test

# Lokale Installation
.PHONY: install
install:
	cargo install --path . --root install-local

# Vollständiger Reset
.PHONY: reset
reset: clean
	if exist "$(OUT_DIR)" rmdir /S /Q "$(OUT_DIR)"
	if exist "install-local" rmdir /S /Q "install-local"
