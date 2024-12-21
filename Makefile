CC ?= cc
CXX ?= cxx
RUSTC ?= rustc
PYTHON ?= python3
TIME ?= time
EXE_SUFFIX :=
ifeq ($(OS),Windows_NT)
	EXE_SUFFIX := .exe
else
	SHELL := /bin/bash
endif

.PHONY: all
all: time-c time-cpp py py-numba time-rust time-rust-no-core

build_dir:
	mkdir -p build

.PHONY: c
c: build_dir
	$(CC) c/speedometer.c -g0 -o build/bl_c$(EXE_SUFFIX)

.PHONY: time-c
time-c: c
	$(TIME) build/bl_c$(EXE_SUFFIX)

.PHONY: cpp
cpp: build_dir
	$(CXX) cpp/speedometer.cpp -g0 -o build/bl_cpp$(EXE_SUFFIX)

.PHONY: time-cpp
time-cpp: cpp
	$(TIME) build/bl_cpp$(EXE_SUFFIX)

.PHONY: py
py:
	time $(PYTHON) py/speedometer.py

.PHONY: py-numba
py-numba:
	time $(PYTHON) py-numba/numbed.py || true

.PHONY: rust
rust: build_dir
	$(RUSTC) -Cpanic=abort -Cdebuginfo=0 -Coverflow-checks=false -Cdebug-assertions=false \
	rust/src/main.rs -o build/bl_rs$(EXE_SUFFIX)

.PHONY: time-rust
time-rust: rust
	time build/bl_rs$(EXE_SUFFIX)

.PHONY: rust-no-core
rust-no-core: build_dir
	$(RUSTC) -Cpanic=abort -Cdebuginfo=0 -Coverflow-checks=false -Cdebug-assertions=false \
	rust-no-core/src/main.rs -o build/bl_rs-no-core$(EXE_SUFFIX)

.PHONY: time-rust-no-core
time-rust-no-core: rust-no-core
	time build/bl_rs-no-core$(EXE_SUFFIX)

# TODO: better Windows support
