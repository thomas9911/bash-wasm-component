wac:
    cargo build -q --features python --release --target wasm32-wasip2
    cd test-component && wkg wit fetch && cargo build -q --features python --release --target wasm32-wasip2
    wac plug -o out.wasm --plug ./target/wasm32-wasip2/release/bash_wasm_component.wasm ./test-component/target/wasm32-wasip2/release/test_component.wasm
    wasmtime run --invoke 'run()' ./out.wasm

wac-no-python:
    cargo build -q --release --target wasm32-wasip2
    cd test-component && wkg wit fetch && cargo build -q --release --target wasm32-wasip2
    wac plug -o out.wasm --plug ./target/wasm32-wasip2/release/bash_wasm_component.wasm ./test-component/target/wasm32-wasip2/release/test_component.wasm
    wasmtime run --invoke 'run()' ./out.wasm
