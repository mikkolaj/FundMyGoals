# First Tip

soroban contract invoke \
    --id <Tip contract contract id> \
    --secret-key <Tipper1 private key> \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --fn tip \
    -- \
    --tipper '{"nickname":"4A6F686E20446F65", "address": "<Tipper1 public key>"}' \
    --max_transfer 10000000

# Second Tip

soroban contract invoke \
    --id <Tip contract contract id> \
    --secret-key <Tipper2 private key> \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --fn tip \
    -- \
    --tipper '{"nickname":"4A6F686E20446F65", "address": "<Tipper2 public key>"}' \
    --max_transfer 100000000
