generate:
	cd errno-gen && \
		cargo build --release && \
		cargo run --release -- ..

clean:
	rm -rf src/errno.rs
