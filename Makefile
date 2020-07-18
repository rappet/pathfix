CARGO=cargo

.PHONY: all test build doc

all: test build doc

test:
	$(CARGO) test

build:
	$(CARGO) build --release

doc:
	$(CARGO) doc
