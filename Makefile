TARGET = core
DESTDIR = /usr/local/lib
CARGO = cargo

.PHONY: all clean install uninstall

all:
	$(CARGO) build --release --manifest-path=$(TARGET)/Cargo.toml

clean:
	$(CARGO) clean --manifest-path=$(TARGET)/Cargo.toml

install:
	install -d $(DESTDIR)
	install -m 0755 $(TARGET)/target/release/libamble.so $(DESTDIR)

uninstall:
	rm -f $(DESTDIR)/libamble.so
