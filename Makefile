CARGO = cargo
JQ = jq

all: Cargo.toml
	$(CARGO) build --release

check: Cargo.toml tests/fixtures/x_profile.jsonld
	$(CARGO) test

clean: Cargo.toml
	@rm -rf *~ target
	$(CARGO) clean

tests/fixtures/x_profile.jsonld: tests/fixtures/x_profile.json src/jq/x_profile.jq
	$(JQ) -f src/jq/x_profile.jq < $< > $@

.PHONY: all check clean
.SECONDARY:
.SUFFIXES:
