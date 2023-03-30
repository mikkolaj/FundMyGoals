# Deploys contract deployer
# Returns contract deployer's contract id

soroban contract deploy \
    --wasm ../target/wasm32-unknown-unknown/release/contract_deployer.wasm \
    --secret-key <Creator private key> \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022'
