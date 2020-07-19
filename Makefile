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

clean:
	cargo clean
	$(MAKE) -C doc clean;
