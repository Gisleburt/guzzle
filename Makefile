.PHONY: test

test:
	EXIT_STATUS=0
	cargo test || EXIT_STATUS=$?
	(cd guzzle-derive && cargo test) || EXIT_STATUS=$?
	@exit ${EXIT_STATUS}
