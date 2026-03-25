# broken_app

RUSTFLAGS="-Zsanitizer=thread" \
cargo +nightly test \
-Zbuild-std \
--target x86_64-unknown-linux-gnu \
--lib --tests --bins

RUSTFLAGS="-Zsanitizer=address" \
cargo +nightly test \
-Zbuild-std \
--target x86_64-unknown-linux-gnu \
--lib --tests --bins