# Project name
PROJECT_NAME := FakeCrypt

# Output folder
DIST_DIR := binOut

# Windows target
WINDOWS_TARGET := x86_64-pc-windows-gnu
WINDOWS_EXE := $(DIST_DIR)/$(PROJECT_NAME).exe

# Linux binary
LINUX_BIN := $(DIST_DIR)/$(PROJECT_NAME)

# Default target: build both
all: linux windows

# Ensure output directory exists
$(DIST_DIR):
	mkdir -p $(DIST_DIR)

# Native Linux build
linux: $(DIST_DIR)
	cargo build --release
	cp target/release/$(PROJECT_NAME) $(LINUX_BIN)
	strip $(LINUX_BIN)

# Windows build
windows: $(DIST_DIR)
	rustup target add $(WINDOWS_TARGET)
	cargo build --release --target=$(WINDOWS_TARGET)
	cp target/$(WINDOWS_TARGET)/release/$(PROJECT_NAME).exe $(WINDOWS_EXE)
	$(if $(shell which strip),strip $(WINDOWS_EXE),echo "strip not found, skipping...")

# Clean build artifacts
clean:
	cargo clean
	rm -rf $(DIST_DIR)

.PHONY: all linux windows clean
