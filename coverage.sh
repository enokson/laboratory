rm -r target/debug/examples

export CARGO_INCREMENTAL=0
# when
# when -Zpanic_abort_tests -Cpanic=abort are enabled, the should_panic macro fails to catch the panic
export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Copt-level=0 -Clink-dead-code -Coverflow-checks=off"
export RUSTDOCFLAGS="-Cpanic=abort"

cargo build

cargo test
cargo test --examples

grcov ./target/debug/ -s . -t html --llvm --branch --ignore-not-existing -o ./target/debug/coverage/