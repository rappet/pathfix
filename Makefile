CARGO=cargo

.PHONY: all test build doc package clean

all: test build doc man

test:
	$(CARGO) test

build:
	$(CARGO) build --release

doc:
	$(CARGO) doc

man:
	$(MAKE) -C doc;

package: test man
	$(CARGO) deb

cov: export CARGO_INCREMENTAL=0
cov: export RUSTFLAGS=-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off -Zpanic_abort_tests -Cpanic=abort
cov: export RUSTDOCFLAGS=-Cpanic=abort
cov:
	env
	$(CARGO) build
	$(CARGO) test
	grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/

clean:
	cargo clean
	$(MAKE) -C doc clean;
