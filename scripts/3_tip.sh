# Tip

soroban contract invoke \
    --id 97f4e7012b3436915914db8f018c8678cb4a72d4696cd48e390bc53e7506c4df \
    --secret-key SA55ABAEDRSVDBIJHXUKSU2VXVUTM2PWWZIZZZMYVVOI5B4EVRODP4AD \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --fn tip \
    -- \
    --tipper '{"nickname":"4A6F686E20446F65", "address": "GDE6KCKLTB7TTAT3KGQBS2P6N7D7S4CUNQ3HPBNHLZPKQYLJYZBIYPJM"}' \
    --max_transfer 1000000000
