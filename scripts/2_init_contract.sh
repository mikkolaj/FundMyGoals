# Init (if necessary replace token id with one returned by get_stellar_asset_id.py script
# Goals are specified in stroops (1XLM = 10.000.000 Stroops)

soroban contract invoke \
    --id 97f4e7012b3436915914db8f018c8678cb4a72d4696cd48e390bc53e7506c4df \
    --secret-key SDR2AST6XYDYWFIEHBN64CSM3QY5W3SO3JNKHY4DKKTEEDL2MJ7A6YHI \
    --rpc-url https://rpc-futurenet.stellar.org:443 \
    --network-passphrase 'Test SDF Future Network ; October 2022' \
    --fn init \
    -- \
    --owner GDAOKBBNVAPV66V2OMQA6EJWF7EETE3JVCHJKZBOH4XXAMUXBKOS5ZNC \
    --token d93f5c7bb0ebc4a9c8f727c5cebc4e41194d38257e1d0d910356b43bfc528813 \
    --goals '["10000000","100000000"]'