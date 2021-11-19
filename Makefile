TARGET_BINARY = target/release/xcb-example

all: $(TARGET_BINARY)

clean:
	$(RM) $(TARGET_BINARY)

.PHONY: all clean

$(TARGET_BINARY): src/main.rs
	cargo build --release && \
		strip --strip-all $@ && \
		upx $@
