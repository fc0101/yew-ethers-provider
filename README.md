# Rust Yew Ethers Provider

This example shows how to connect wallet in Yew using ethers.

The main purpose of using [Rust](https://www.rust-lang.org/learn) is enhanced safety, speed, and concurrency, or the ability to run multiple computations parallelly. In simple words, Rust is used for three essential purposes in programming: performance, safety, and memory management.

```bash
    # install Rust
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    rustup toolchain install nightly    
    rustup target add wasm32-unknown-unknown    
    cargo install --locked wasm-bindgen-cli
    
    cargo install trunk

    
    git clone https://github.com/fc0101/yew-ethers-provider.git
    
    cd yew-ethers-provider/sample/web3-ether-connect-wallet-example

    trunk serve
```

It will be helpfull if you are a new Rust Developer
