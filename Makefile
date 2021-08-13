.PHONY: build install lint test check clean bench

# NOTE: nightly due to feature(map_first_last)
CARGO := cargo +nightly
build:
	${CARGO} build

install:
	${CARGO} --path .

# NOTE: allow needless-borrow due to clippy version inconsistency
lint:
	${CARGO} clippy -- -D warnings -A clippy::needless-borrow

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
