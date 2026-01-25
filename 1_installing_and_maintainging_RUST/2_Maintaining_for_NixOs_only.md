# if neovim is not showing use this cmds 1 by one 

```bash 
rustup component add rust-analyzer rust-src clippy rustfmt
rustup update 
```

## New ALIAS

```nix 
rust-maintain= "rustup update && rustup component add rust-analyzer rust-src clippy rustfmt && rustup component add clippy --toolchain nightly";
```

## for maintaining : 

    - rustup toolchain list

