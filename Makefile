.PHONY: build install lint test check clean bench

# FIXME: nightly because of bench. Should create benches/
CARGO := cargo +nightly
build:
	${CARGO} build

install:
	${CARGO} --path .

lint:
	${CARGO} clippy

test:
	${CARGO} test -- --nocapture

test-single:
	${CARGO} test ${TARGET} -- --nocapture	

check:
	${CARGO} check

clean:
	rm -rf target

bench:
	# NOTE: all benches annotated with ignored
	${CARGO} bench -- --ignored