.PHONY: build install lint test check clean bench

# NOTE: nightly due to feature(map_first_last)
CARGO := cargo +nightly
build:
	${CARGO} build

install:
	${CARGO} --path .

# NOTE: allow needless-borrow due to clippy version inconsistency
CLIPPY_OPTION := -D warnings -A clippy::needless-borrow
ADDITIONAL_CLIPPY_OPTION := # -D missing-docs
lint:
	${CARGO} clippy -- ${CLIPPY_OPTION} ${ADDITIONAL_CLIPPY_OPTION}

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
