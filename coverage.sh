#find target/debug | grep .gc | rm

export CARGO_INCREMENTAL=0
# when
# when -Zpanic_abort_tests -Cpanic=abort are enabled, the should_panic macro fails to catch the panic
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off"
export RUSTDOCFLAGS="-Cpanic=abort"

cargo build --lib

cargo test --lib

grcov ./target/debug/ -s . -t html --ignore ./examples/**/* --llvm --branch --ignore-not-existing -o ./target/debug/coverage/
