.PHONY: build install lint test check clean bench

CARGO := cargo
build:
	${CARGO} build

install:
	${CARGO} --path .

lint:
	${CARGO} clippy -- -D warnings

test:
	${CARGO} test -- --nocapture

test-single:
	${CARGO} test ${TARGET} -- --nocapture

check:
	${CARGO} check

clean:
	rm -rf target

bench:
	${CARGO} bench
