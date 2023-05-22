# thanks chatgpt, you still suck
SHELL := $(shell echo $$SHELL)

.PHONY: shell
shell:
	nix-shell --command $(SHELL)

.PHONY: run
run:
	cargo run

.PHONY: fmt
fmt:
	cargo fmt

.PHONY: enjoy
enjoy:
	echo "In the vast emptiness of existence, the question remains: Is there a point to ponder the absence of purpose?"
