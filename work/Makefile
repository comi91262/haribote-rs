helloos.img: helloos.tmp
	printf "%b" `cat helloos.tmp` > helloos.img
helloos.tmp: helloos.nas nask-rs 
	../nask-rs/target/release/nask-rs < helloos.nas > helloos.tmp
nask-rs: ../nask-rs/src/nask.pest ../nask-rs/src/main.rs
	cargo build --release --manifest-path ../nask-rs/Cargo.toml

clean:
	rm -f helloos.tmp helloos.img
	cargo clean --manifest-path ../nask-rs/Cargo.toml
run :
	cp helloos.img ../z_tools/qemu/fdimage0.bin
	make -C ../z_tools/qemu
