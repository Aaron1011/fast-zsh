all: target/fastbrackets

run:
	gdb -ex run --args ~/repos/zsh/Src/zsh test.sh

install: target/fastbrackets
	cp target/fastbrackets.so /usr/lib/zsh/5.3.1/aaron/

target/fastbrackets: target/debug/libfastbrackets.so
	cp target/debug/libfastbrackets.so target/fastbrackets.so

target/debug/libfastbrackets.so: src/lib.rs Cargo.toml
	cargo build

clean:
	rm -rf target
