cargo build --release
strip ./target/release/eclipsis
mv ./target/release/eclipsis ./solstice
zip -q ./solstice.zip ./solstice ./assets/models/*/*.obj