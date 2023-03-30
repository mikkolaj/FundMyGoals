# Installs tip contract's code on the network
# Returns Wasm hash

soroban contract install \
    --wasm ../target/wasm32-unknown-unknown/release/tip_contract.wasm \
    --secret-key <Creator private key> \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022'