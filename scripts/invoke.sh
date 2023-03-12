#soroban contract invoke \
#    --wasm ../target/wasm32-unknown-unknown/release/tips_4_goals.wasm \
#    --id 1 \
#    --fn hello \
#    -- \
#    --to friend

soroban contract invoke \
    --wasm ../target/wasm32-unknown-unknown/release/tips_4_goals.wasm \
    --id 1 \
    --fn increment \
