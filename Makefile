CARGO=cargo

.PHONY: all test build doc package

all: test build doc

test:
	$(CARGO) test

build:
	$(CARGO) build --release

doc:
	$(CARGO) doc

package: test
	$(CARGO) deb
