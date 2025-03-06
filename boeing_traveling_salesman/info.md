### Instructions:

To build the project `wasm-pack build --target web`
It will build in the target/web directory.

To run the project on a local server run `python3 -m http.server 8000`
Then visit this link [click here](http://localhost:8000/target/web)

#### Wasm-pack Info:

[wasm-pack docs](https://rustwasm.github.io/docs)
[wasm-pack book](https://rustwasm.github.io/docs/book)

- After building, wasm-pack generates files in `/pkg/` directory. The files are the wasm binary, a JS wrapper file, our README, and a package.json file.

#### Rust Crates:

serde [docs](https://docs.rs/serde/latest/serde/)
serde-wasm-bindgen [docs](https://docs.rs/serde-wasm-bindgen/latest/serde_wasm_bindgen/)
serde-json [docs](https://docs.rs/serde_json/latest/serde_json/)
- These crates are used to serialze and deserialize data structures.

wasm-bindgen [docs](https://docs.rs/wasm-bindgen/)
- Exposes Rust code to Javascript via building marked sections to web assembly.


petgraph [docs](https://docs.rs/petgraph/latest/petgraph/)
- A graph data structure library that seems to be very compatible with our stack.

csv [docs](https://docs.rs/csv/latest/csv/)
- CSV read and write support that is compatible with serde.

bitvec [docs](https://docs.rs/bitvec/latest/bitvec/) 
- Allows rust to interact with memory as bit addresses instead of byte addressed. Brayden seems to need it for Algorithm stuff.
