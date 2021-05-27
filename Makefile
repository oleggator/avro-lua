build:
	mkdir -p .rocks/lib/tarantool
	cargo build --release

install:
	cp target/release/libavro.so $(TARANTOOL_INSTALL_LIBDIR)/avro.so || cp target/release/libavro.dylib $(TARANTOOL_INSTALL_LIBDIR)/avro.dylib
