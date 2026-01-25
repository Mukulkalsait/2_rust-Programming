<!-- # Y: ----------------------------------- NIX ------------------------------------------------------- -->

 # This is the only kind of settup we needed in nixos
```nix 
home.packages = with pkgs; [
  # Minimal non-conflicting tools
  gcc  # For Rust linking

  # rustup for all Rust tools (cargo, rustc, rustfmt, clippy, rust-analyzer)
  rustup

  # No standalones: cargo, rustc, rustfmt, rust-analyzer—rustup covers them
];
```

<!-- # Y: ------------------------------------------------------------------------------------------ -->

### Finalizing Your Rust Setup on NixOS

You've nailed the rebuild—now the rustup initialization is the missing piece to get everything firing. Yes, run those exact commands in your shell (post-rebuild, in a fresh session like `exec zsh`). They'll populate `~/.rustup` with the stable toolchain and components, making `cargo`, `rustc`, `rustfmt`, `rust-analyzer`, and sources available without Nix duplicates. Here's the full sequence, including Clippy:

1. **Install Stable Toolchain** (core for builds and LSP):
   ```bash
   rustup toolchain install stable
   rustup default stable  # Sets stable as global default for Cargo/rustc
   ```

2. **Add Essential Components** (sources for std lib hovers, formatter, linter, LSP):
   ```bash
   rustup component add rust-src rustfmt clippy rust-analyzer
   ```
   - `rust-src`: Enables full docs/hovers for `std::fs::File`, etc.
   - `rustfmt`: For `:RustFmt` in Neovim.
   - `clippy`: Linter for diagnostics (works on stable, but fuller on nightly—see below).
   - `rust-analyzer`: LSP binary (matches your project's toolchain).

3. **For Clippy on Nightly** (Yes, full Clippy needs nightly for advanced lints like `cargo-clippy --fix` or unstable flags):
   - Install nightly alongside stable (no conflicts—rustup manages per-toolchain):
     ```bash
     rustup toolchain install nightly
     rustup component add clippy --toolchain nightly  # Nightly-specific Clippy
     ```
   - To use nightly per-project (e.g., for your guessing game repo): Add `rust-toolchain.toml` to the project root:
     ```toml
     [toolchain]
     channel = "nightly"
     components = ["rust-src", "clippy", "rustfmt"]
     ```
     - Run `cargo build` in the dir—it auto-switches to nightly. For global nightly: `rustup default nightly`.
   - In Neovim's rustaceanvim, set `check.command = "clippy"` in settings—it'll use the project toolchain.

After running these, verify:
- `which rust-analyzer` → `~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rust-analyzer`.
- `rust-analyzer --version` → Recent build (e.g., 2025-12-15+).
- `cargo clippy --version` → Stable or nightly, depending on dir.
- In project: `cargo check` → No sysroot errors.

#### Maintaining Versions and Updates
Rustup is self-managing and lightweight—updates are simple and don't require Nix rebuilds:
- **Daily/Weekly Update**: Run `rustup update` (updates all toolchains/components to latest stable/nightly). Add to a cron job if automated:
  ```bash
  crontab -e
  # Add: 0 2 * * 1 /home/mukuldk/.cargo/bin/rustup update  # Weekly Mondays at 2AM
  ```
- **Per-Project Overrides**: `rust-toolchain.toml` (as above) pins versions—`rustup show` checks active one.
- **Component Refresh**: `rustup component add --toolchain stable rust-src` (re-adds sources if outdated).
- **Cleanup**: `rustup toolchain list` (view installed), `rustup toolchain uninstall <name>` (remove old ones), `rustup self update` (update rustup itself).
- **Nix Integration**: In `home.nix`, keep the activation hook from earlier to init on rebuild:
  ```nix
  home.activation = {
    initRustup = lib.hm.dag.entryAfter ["writeBoundary"] ''
      if [ ! -d $HOME/.rustup ]; then
        ${pkgs.rustup}/bin/rustup toolchain install stable
        ${pkgs.rustup}/bin/rustup default stable
        ${pkgs.rustup}/bin/rustup component add rust-src rustfmt clippy rust-analyzer
        ${pkgs.rustup}/bin/rustup toolchain install nightly
        ${pkgs.rustup}/bin/rustup component add clippy --toolchain nightly
      fi
    '';
  };
  ```
  - This runs only if `~/.rustup` is missing—avoids re-init on every switch. Rebuild once to apply.

For info: Check `rustup help`, `rustup doc` (opens local docs), or `cargo --help`. Track releases at rust-lang.org (stable ~every 6 weeks).

#### Neovim Config: Separate `rust_config.lua`
Smart call—modularize to avoid bloating `webdev.lua`. Create `~/.config/nvim/lua/rust_config.lua` (or in your plugin dir) and load it in `init.lua` with `require("rust_config")`.

Contents for `rust_config.lua` (integrates with rustaceanvim, points to rustup):
```lua
return {
  {
    "mrcjkb/rustaceanvim",
    version = "^6",
    ft = { "rust" },
    opts = {
      server = {
        cmd = function()
          -- Use rustup's rust-analyzer (stable or project-specific)
          local toolchain = "stable-x86_64-unknown-linux-gnu"  -- Adjust if using nightly
          local ra_path = vim.fn.expand("~/.rustup/toolchains/" .. toolchain .. "/bin/rust-analyzer")
          if vim.fn.filereadable(ra_path) == 1 then
            return { ra_path }
          end
          -- Fallback to PATH (Nix if rustup fails)
          return { "rust-analyzer" }
        end,
        settings = {
          ["rust-analyzer"] = {
            cargo = {
              allFeatures = true,
              buildScripts = { enable = true },
              loadOutDirsFromCheck = true,
            },
            check = {
              command = "clippy",  -- Uses nightly if project sets it
              extraArgs = { "--no-deps" },
            },
            checkOnSave = true,
            diagnostics = { enable = true },
            files = {
              excludeDirs = { ".direnv", ".git", ".github", ".gitlab", "bin", "node_modules", "target", "venv", ".venv" },
            },
            procMacro = { enable = true },
            rust = {
              analyzerTargetDir = true,
              linkedProjects = { "Cargo.toml" },  -- Auto-detects workspace
              sysrootSrc = vim.env.RUST_SRC_PATH,  -- Ties to your env for std lib
            },
          },
        },
      },
      tools = {
        inlay_hints = { auto = true },  -- Optional: Rust hints
      },
    },
  },
}
```
- **Load It**: In `init.lua`: `require("rust_config")` (after lspconfig setup).
- **Why Separate?** Keeps webdev clean; easy to tweak Rust-only (e.g., add `dap` for debugging later).
- **Disable lspconfig rust_analyzer**: Confirm it's removed from `servers` in `webdev.lua` to avoid duplicates.

Restart Neovim, open your `.rs` file, `:LspInfo` (one client), `:w` (Clippy runs), hover/complete—should all work now. If not, check `~/.rustup` exists and share `:LspInfo` or logs. This gets your guessing game project humming with full LSP power.
