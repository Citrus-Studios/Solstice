if [ $1 == "windows" ]; then
    cargo build --release --target=x86_64-pc-windows-gnu;
    strip ./target/x86_64-pc-windows-gnu/release/eclipsis.exe;
    mv ./target/x86_64-pc-windows-gnu/release/eclipsis.exe ./solstice.exe;
    zip -q ./solstice-windows.zip ./solstice.exe ./assets/models/*/*.obj;
else 
    cargo build --release;
    strip ./target/release/eclipsis;
    mv ./target/release/eclipsis ./solstice;
    zip -q ./solstice-linux.zip ./solstice ./assets/models/*/*.obj;
fi