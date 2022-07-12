# Cycan

## 1. Introduction

Cycan Network is an DeFi infrastructures  based the Substrate framework,it supports both wasm smart contracts and solidity smart contracts.

Benefit from the framework,we can easily add widgets (called pallet) to add new features.We have added some pallet to this framework to make the network more advantageous in decentralized finance.

These pallet include:

GVM-Bridge : Through it WASM contracts and EVM contracts can call each other.

RGrandpa : Random GHOST-based Recursive ANcestor Deriving Prefix Agreement,based the Grandpa.By randomly selecting some validator nodes, the number of nodes participating in the consensus is reduced, and the effect of improving the efficiency of the network consensus is achieved.

Frontier: Substrate's Ethereum compatibility layer. It allows you to run unmodified Ethereum dapps.

ESBind : Binding a Substrate address and a Ethereum address,share balance and transaction between them.

As the project develops, more pallet will be added.

## 2. Building

### Build from Source

If you'd like to build from source, first install Rust. You may need to add Cargo's bin directory to your PATH environment variable. Restarting your computer will do this for you automatically.

```
curl https://sh.rustup.rs -sSf | sh
```

If you already have Rust installed, make sure you're using the latest version by running:

```
rustup update
```

Once done, finish installing the support software:

|        OS        |                    Installation commands                     |
| :--------------: | :----------------------------------------------------------: |
| Ubuntu or Debian | sudo apt update && sudo apt install -y git clang curl libssl-dev llvm libudev-dev |
|    Arch Linux    |       pacman -Syu --needed --noconfirm curl git clang        |
|      Fedora      | sudo dnf update sudo dnf install clang curl git openssl-devel |
|     OpenSUSE     | sudo zypper install clang curl git openssl-devel llvm-devel libudev-devel |
|      macOS       |        brew update && brew install openssl cmake llvm        |
|      CentOS      | yum install -y cmake pkg-config libssl-dev git build-essential clang libclang-dev curl gcc yum install gcc-c++ |

Get the source code:

```
git clone https://github.com/CycanTech/Cycan.git
```

Build the client by cloning this repository and running the following commands from the root directory of the repo:

```
./scripts/init.sh
cargo build --release
```

#### Build from Source with Docker

you can running the following commands from the root directory of the repo:

```
./script/docker/build.sh
```

then,run test node as:

```
docker run -d -p 9933:9933 -p 9944:9944 -p 30333:30333 -p 9615:9615 cycan/cycan:v0.1.0  --tmp --dev  --rpc-cors all --unsafe-rpc-external --unsafe-ws-external --prometheus-external
```

# 3. Test

you can start a test with: 

```
cargo test
```

or run:

```
./scripts/test.sh
```

# 4. Run

### Local development Testnet:

```
./target/release/cycan --tmp --dev
```

### Local Two-node Testnet

If you want to see the multi-node consensus algorithm in action locally, then you can create a local testnet. You'll need two terminals open. In one, run:

```
./target/release/cycan   --base-path /tmp/alice   --chain=local   --alice   --node-key 0000000000000000000000000000000000000000000000000000000000000001   --validator --rpc-cors=all --no-mdns
```

And in the other, run:

```
./target/release/cycan   --base-path /tmp/bob   --chain=local   --bob   --port 30334 --ws-port 9946 --rpc-port 9934 --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp   --validator --rpc-cors=all --no-mdns
```

### Local Four-node Beta Livenet

Terminal 1 run:

```
./target/release/cycan   --base-path /tmp/alice   --chain=beta   --bootnodes "/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWMwqgHC8bsZ7nixSagXUhZ6wG4qURLXCm4Q9YzzGr6Cee" "/ip4/127.0.0.1/tcp/30334/p2p/12D3KooWLfM4y2FHZxMrNmyD6Xon4tC55T1qCEtiB2mJq8WcpfYH" "/ip4/127.0.0.1/tcp/30335/p2p/12D3KooWG6n3bXcogzzmvstDMRBaYwSadHHnVCMzkF6NSt2pbAWm" "/ip4/127.0.0.1/tcp/30336/p2p/12D3KooWBpV8LsrpxdCLeEfWwVDaEtVijdB8hMtJ6mC1TaXUQFAF" --node-key 758426877c0b54bf656b5ec804b1a514da6573341948c2f4e2a5251ac17af4c2   --validator --rpc-cors=all --no-mdns --keystore-path ./keystore/1
```

Terminal 2 run:

```
./target/release/cycan   --base-path /tmp/bob   --chain=beta   --port 30334 --ws-port 9946 --rpc-port 9934 --bootnodes "/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWMwqgHC8bsZ7nixSagXUhZ6wG4qURLXCm4Q9YzzGr6Cee" "/ip4/127.0.0.1/tcp/30334/p2p/12D3KooWLfM4y2FHZxMrNmyD6Xon4tC55T1qCEtiB2mJq8WcpfYH" "/ip4/127.0.0.1/tcp/30335/p2p/12D3KooWG6n3bXcogzzmvstDMRBaYwSadHHnVCMzkF6NSt2pbAWm" "/ip4/127.0.0.1/tcp/30336/p2p/12D3KooWBpV8LsrpxdCLeEfWwVDaEtVijdB8hMtJ6mC1TaXUQFAF" --node-key 7ed8339f7523ac2709deec4753ec4a7cb1f7a09069ed58659c85c82e6641c9c5  --validator --rpc-cors=all --no-mdns --keystore-path ./keystore/2
```

Terminal 3 run:

```
./target/release/cycan   --base-path /tmp/charlie   --chain=beta   --port 30335 --ws-port 9947 --rpc-port 9935 --bootnodes "/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWMwqgHC8bsZ7nixSagXUhZ6wG4qURLXCm4Q9YzzGr6Cee" "/ip4/127.0.0.1/tcp/30334/p2p/12D3KooWLfM4y2FHZxMrNmyD6Xon4tC55T1qCEtiB2mJq8WcpfYH" "/ip4/127.0.0.1/tcp/30335/p2p/12D3KooWG6n3bXcogzzmvstDMRBaYwSadHHnVCMzkF6NSt2pbAWm" "/ip4/127.0.0.1/tcp/30336/p2p/12D3KooWBpV8LsrpxdCLeEfWwVDaEtVijdB8hMtJ6mC1TaXUQFAF" --node-key 7311e3909937398c074a20a82470f9b3658dfe49cb2e9d2a7acfa6ca8ed3dc78   --validator --rpc-cors=all --no-mdns --keystore-path ./keystore/3
```

Terminal 4 run:

```
./target/release/cycan   --base-path /tmp/dave   --chain=beta   --port 30336 --ws-port 9948 --rpc-port 9936 --bootnodes "/ip4/127.0.0.1/tcp/30333/p2p/12D3KooWMwqgHC8bsZ7nixSagXUhZ6wG4qURLXCm4Q9YzzGr6Cee" "/ip4/127.0.0.1/tcp/30334/p2p/12D3KooWLfM4y2FHZxMrNmyD6Xon4tC55T1qCEtiB2mJq8WcpfYH" "/ip4/127.0.0.1/tcp/30335/p2p/12D3KooWG6n3bXcogzzmvstDMRBaYwSadHHnVCMzkF6NSt2pbAWm" "/ip4/127.0.0.1/tcp/30336/p2p/12D3KooWBpV8LsrpxdCLeEfWwVDaEtVijdB8hMtJ6mC1TaXUQFAF" --node-key ea8a1d99f96b7680419eaccaa40c73d61ac1c19801392f064255d49d93bc02e7   --validator --rpc-cors=all --no-mdns --keystore-path ./keystore/4
```

all [node key](https://docs.substrate.io/tutorials/get-started/simulate-network/) above is generate by [Subkey](https://docs.substrate.io/reference/command-line-tools/subkey/).

The command and output are as follows:

```
subkey generate-node-key
======================================================================
12D3KooWMwqgHC8bsZ7nixSagXUhZ6wG4qURLXCm4Q9YzzGr6Cee
758426877c0b54bf656b5ec804b1a514da6573341948c2f4e2a5251ac17af4c2

12D3KooWLfM4y2FHZxMrNmyD6Xon4tC55T1qCEtiB2mJq8WcpfYH
7ed8339f7523ac2709deec4753ec4a7cb1f7a09069ed58659c85c82e6641c9c5

12D3KooWG6n3bXcogzzmvstDMRBaYwSadHHnVCMzkF6NSt2pbAWm
7311e3909937398c074a20a82470f9b3658dfe49cb2e9d2a7acfa6ca8ed3dc78

12D3KooWBpV8LsrpxdCLeEfWwVDaEtVijdB8hMtJ6mC1TaXUQFAF
ea8a1d99f96b7680419eaccaa40c73d61ac1c19801392f064255d49d93bc02e7
```

## License

Cycan is [GPL 3.0 ](https://github.com/CycanTech/Cycan/blob/master/LICENSE-GPL3) and [Apache 2.0](https://github.com/CycanTech/Cycan/blob/master/LICENSE-APACHE2) licensed.



