# Bitcoin Mining using WebAssembly in Rust

This is a simple example of how to mine Bitcoin using WebAssembly in Rust.

<div align="center" style="display: flex; justify-content: center; align-items: flex-start;">

<img src="https://github.com/user-attachments/assets/75a8423d-3cd2-4fe5-a157-d2ac16260c96" alt="logo" width="256" />

</div>

## Prerequisites

- [`wasm-pack`](https://github.com/rustwasm/wasm-pack)
  - ref: https://rustwasm.github.io/wasm-pack/installer/
- [`yarn`](https://github.com/yarnpkg/berry)
  - ref: https://yarnpkg.com/getting-started/install

## How to run

1. Build the Rust code to WebAssembly

  ```console
  $ wasm-pack build --target web --out-dir frontend/public/pkg
  ```

2. Install the dependencies for frontend

  ```console
  $ cd frontend
  $ yarn install
  ```

3. Run the frontend

  ```console
  $ yarn start
  ```
