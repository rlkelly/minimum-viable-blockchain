# minimum-viable-blockchain
a minimal blockchain in RUST


### QuickStart

You'll need to run a miner on port 3000 so other miner's can find the blockchain.

    cargo run --bin main 3000

And perhaps run a few peers in other terminals:

    cargo run --bin main 3001
    cargo run --bin main 3002

They should all sync and begin to mine blocks.

A wallet can be run by:

    cargo run --bin wallet

This will create a wallet and it can send transactions.
