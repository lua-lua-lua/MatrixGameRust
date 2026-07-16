# MatrixGame (Rust/WASM)

A Rust port of **MatrixGame** — the Planetary Battles engine from
*Space Rangers 2* (originally a DirectX 9 C++ engine, GPL). Renders with
wgpu and runs both natively and in the browser via WebAssembly.

Original game data (`Data/robots.pkg`, `Data/Lang.dat`, …) is required and
not included in this repository.

## Build & Run

### Native

```bash
cargo build
cargo run   # needs a display and Data/robots.pkg
```

### Web (WASM)

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack

wasm-pack build --target web --out-dir pkg

# Pack per-map asset bundles from robots.pkg
for m in $(cargo run --example pkg_ls -- Data/robots.pkg 2>/dev/null \
  | grep -i '^MATRIX/MAP/.*CMAP$'); do cargo run --example pack_bundle -- "$m"; done

# Main-menu art and map texts
cargo run --example menu_assets
python3 tools/lang_dat.py Data/Lang.dat --maps assets/menu/maps.txt

python3 -m http.server 8081   # then open http://localhost:8081
```

## Layout

- `src/` — engine code, mirroring the original C++ tree
  (`matrix_lib/`, `matrix_game/`); `CROSSREF.md` maps each Rust file to
  its C++ source
- `shaders/` — WGSL shaders
- `examples/` — headless tools and probes (`game_sim.rs` is the autonomous
  battle harness, see `SIM.md`)
- `tests/` — integration tests against real `Data/` files
- `index.html` — browser front end (main menu + game canvas)

## Testing

```bash
cargo test --lib   # unit tests
cargo test         # integration tests (need Data/)
```

## License

GPLv3 — see [LICENSE](LICENSE).
