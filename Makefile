build: 
		@cargo rustc --target aarch64-unknown-linux-musl -- -C target-cpu=cortex-a72 -C linker=aarch64-linux-gnu-gcc
		@cp ./target/aarch64-unknown-linux-musl/debug/driver ./
