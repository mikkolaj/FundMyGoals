# Deploys tip contract
# Each deployed contract must have a unique salt
# Goals are specified in stroops (1XLM = 10.000.000 Stroops)

soroban contract invoke \
    --id <Contract deployers contract id> \
    --secret-key <Creator private key> \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --fn deploy \
    -- \
    --salt 0000000000000000000000000000000000000000000000000000000000000000 \
    --wasm_hash <Wasm hash> \
    --creator <Creator public key> \
    --token d93f5c7bb0ebc4a9c8f727c5cebc4e41194d38257e1d0d910356b43bfc528813 \
    --goals '["10000000","100000000"]'