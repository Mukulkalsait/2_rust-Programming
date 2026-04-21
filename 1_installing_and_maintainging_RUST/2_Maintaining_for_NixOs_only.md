# Maintainance: 
### For even better use this technique:

  - while running and all we sometime get "cc" linker problem to solve immideately
  >  keep Nightly as your global default (for Neovim/rust-analyzer to stay happy and fast)
  ```bash 
    rustup default nightly
 ```

# Then, For compiling project, always build/run with stable when you don't need nightly features:
``` bash 
    cargo +stable run
    cargo +stable build
    cargo +stable check
    cargo +stable test
```


#### IMP: THE COMPONENT ADD LINE IS THE ONE SOVLES THE PROBLEM 🔻

```bash 
rustup component add rust-analyzer rust-src clippy rustfmt
rustup update 
```

## New ALIAS

```nix 
rust-maintain= "rustup update && rustup component add rust-analyzer rust-src clippy rustfmt && rustup component add clippy --toolchain nightly";
```

## for maintaining : 
```bash 

   rustup toolchain list => list 
   rustup toolchain uninstall '<name>' 

   rustup toolchain install nightly/stable
   rustup toolchain default nightly/stable

   rustup component remove <comp_name> 
   rustup self update {then use}=> 
   rustup prune

```


# importatn NVIM 

we need  
: Lazy or <space>l
    ● rustaceanvim 0.06ms  rust 
installed to work 

