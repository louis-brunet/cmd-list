## Run

```bash
cargo run -- --help
```

```bash
cargo run -- --output-format=header run ls -- -l -a -al '-l -a'
```

## Install

```bash
cargo build --release
# if $HOME/bin is in $PATH
cp ./target/release/cmd-list "$HOME/bin"
cmd-list gen completion --shell zsh --bin-name cmd-list >"$HOME/bin/_cmd-list"
exec zsh
```
