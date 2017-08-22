all: target/fastbrackets

run:
	gdb -ex run --args ~/repos/zsh/Src/zsh test.sh

install: target/fastbrackets
	cp target/fastbrackets.so /usr/lib/zsh/5.4.1/aaron/

target/fastbrackets: target/release/libfastbrackets.so
	cp target/release/libfastbrackets.so target/fastbrackets.so

target/release/libfastbrackets.so: src/lib.rs Cargo.toml
	cargo build --release

clean:
	rm -rf target
