## Using Cargo and npm

Requires `cargo`, `npm` and `wasm-pack` (0.11.0):

```bash
# 1: Clone the repository
$ git clone https://github.com/evan3v4n/Evolution-Simulation
$ cd Evolution-Simulation

# 2: Compile Rust into WebAssembly
$ cd libs/simulation-wasm
$ wasm-pack build --release

# 3/3: Start the frontend application
$ cd ../../www
$ npm install
$ npm run start

# ^ After launching this, open `http://localhost:8080` in your web browser
```

Credits to Patryk for writing the guide on this project
https://pwy.io/posts/learning-to-fly-pt1/
