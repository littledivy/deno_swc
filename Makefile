ifeq ($(OS),Windows_NT)
	WASM_SOURCE := swc_wasm\lib.rs
	WASM_BUILD := swc_wasm\pkg\deno_swc_bg.wasm
else
	WASM_SOURCE := swc_wasm/lib.rs
	WASM_BUILD := swc_wasm/pkg/deno_swc_bg.wasm
endif

WASM_BUILD := deno run -A --unstable build.js

$(WASM_BUILD): $(WASM_SOURCE)
	cd swc_wasm \
	&& $(WASM_BUILD)

fmt:
	deno fmt --ignore=swc_wasm --unstable

lint: fmt
	deno lint --ignore=swc_wasm --unstable

test: fmt lint
	deno test tests/

clean:
	rm -rf swc_wasm/pkg

.PHONY: fmt lint test
