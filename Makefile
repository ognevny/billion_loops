CC ?= cc
CXX ?= cxx
RUSTC ?= rustc
TIME ?= time
EXE_SUFFIX :=
ifeq($(OS),Windows_NT)
	EXE_SUFFIX := .exe
endif

.PHONY: all
all: time-c time-cpp py py-numba time-rust time-rust-no-core

.PHONY: c
c:
	$(CC) c/speedometer.c -g0 -o build/bl_c$(EXE_SUFFIX)

.PHONY: time-c
time-c: c
	$(TIME) build/bl_c$(EXE_SUFFIX)

.PHONY: cpp
cpp:
	$(CXX) cpp/speedometer.cpp -g0 -o build/bl_cpp$(EXE_SUFFIX)

.PHONY: time-cpp
time-cpp: cpp
	$(TIME) build/bl_cpp$(EXE_SUFFIX)

# TODO: others...
