ifeq ($(shell uname),Darwin)
    EXT := dylib
else
    EXT := so
endif

all: target/debug/libdouble_input.$(EXT)
	php src/ffi.php

target/debug/libdouble_input.$(EXT): src/lib.rs Cargo.toml
	cargo build

clean:
	rm -rf target
