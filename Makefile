b:
	cargo build
t:
	cargo test

p1w:
	cargo test -- --ignored p1_wins

p0w:
	cargo test -- --ignored --show-output p0_wins

psc:
	cargo test -- --ignored --show-output post_start_connection
