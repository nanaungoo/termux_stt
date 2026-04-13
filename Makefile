# Makefile for termux_stt
# Includes logic for setting LD_LIBRARY_PATH based on OS/Arch

SHELL := /bin/bash
OS := $(shell uname -s)
ARCH := $(shell uname -m)

# Select library path
ifeq ($(ARCH),x86_64)
	LIB_DIR := $(PWD)/libs/x86_64
else ifeq ($(ARCH),aarch64)
	LIB_DIR := $(PWD)/libs/aarch64
else ifeq ($(ARCH),armv7l)
	LIB_DIR := $(PWD)/libs/armv7
else
	LIB_DIR := $(PWD)/libs/x86_64
endif

export LD_LIBRARY_PATH := $(LD_LIBRARY_PATH):$(LIB_DIR)

.PHONY: build run test clean help

build:
	cargo build

run:
	cargo run

test:
	cargo test -- --nocapture

clean:
	cargo clean

help:
	@echo "Usage:"
	@echo "  make build  - Build the project"
	@echo "  make run    - Run the application"
	@echo "  make test   - Run all unit tests"
	@echo "  make clean  - Remove build artifacts"
