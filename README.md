# p5-rs-starter
Starter template for p5-sys

1. [Install Rust](https://www.rust-lang.org/tools/install)
2. [Install wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
3. `git clone https://github.com/Maan2003/p5-rs-starter.git`
4. `cd p5-rs-starter`
5. Open Cargo.toml and replace `{{ project }}` with the project name and `{{ author }}` with your name
6. `wasm-pack build --target web --out-name sketch --dev` (--dev is for developing mode will compile faster)
7. `cd pkg`
8. Start some http server here like `python -m http.server`

[open this example in browser](https://maan2003.github.io/p5-rs-starter/pkg)