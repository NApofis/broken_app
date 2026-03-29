# broken_app

Тестовый репозиторий для практики поиска багов и оптимизации rust кода.

Репозиторий содержит два проекта:


cargo run --bin demo &> ../checks/step-1/broken-app-after/cargorunbin
cargo check &> ../checks/step-1/broken-app-after/cargocheck
cargo test &> ../checks/step-1/broken-app-after/cargotest
cargo +nightly miri test &> ../checks/step-1/broken-app-after/miri
valgrind --leak-check=full cargo test --tests &> ../checks/step-1/broken-app-after/valgrind
RUSTFLAGS="-Zsanitizer=thread" cargo +nightly test -Zbuild-std --target x86_64-unknown-linux-gnu --lib --tests --bins &> ../checks/step-1/broken-app-after/sanitizers-thread
RUSTFLAGS="-Zsanitizer=address" cargo +nightly test -Zbuild-std --target x86_64-unknown-linux-gnu --lib --tests --bins &> ../checks/step-1/broken-app-after/sanitizers-address 


cargo build --release
perf record -F 300 -g ./demo
perf script | /media/nik/Special/yadisk/Rust/lesson5/FlameGraph/stackcollapse-perf.pl | /media/nik/Special/yadisk/Rust/lesson5/FlameGraph/flamegraph.pl > flame.svg


cargo bench --bench criterion