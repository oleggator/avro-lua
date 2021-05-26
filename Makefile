build:
	mkdir -p .rocks/lib/tarantool
	cargo build --release

install:
	cp target/release/libavro.so .rocks/lib/tarantool/avro.so || cp target/release/libavro.dylib .rocks/lib/tarantool/avro.dylib
