# DeadManDAO presents New2Web3
A simple example of using a ReactJS frontend to make calls to WebAssembly Rust services decentralized on the Fluence servers.
Our demo video can be found at:

https://youtu.be/dLQdivDMJJE

## Getting started

#### Building the Rust WebAssembly service

```bash
cd nft-service
mkdir artifacts
marine build --release
cp target/wasm32-wasi/release/deadman-nft-service.wasm ./artifacts/deadmandao-nft-service-0_5_1.wasm
marine aqua artifacts/deadman-nft-service-0_5_1.wasm
```

#### Building the ReactJS user interface

Change directory to the web subproject:

```bash
cd web
```

Run npm install to download all the dependencies:

```bash
npm install
```

Run npm run compile-aqua to generate the Aqua container

```bash
npm run compile-aqua
```

Run npm run build to compile/consolidate the code

```bash
npm run build
```

Start the application

```bash
npm start
```

A browser window with `localhost:3000` should open automatically.

## About DeadManDAO and New2Web3
This project was created to meet the requirements of a GitCoin Schelling Point Hackathon challenge:
https://gitcoin.co/issue/fluencelabs/gitcoin-schelling-point-hackathon/1/100027508

####A big thanks for the Schelling Point folks and GitCoin for making this possible!
