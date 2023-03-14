soroban config identity generate acc1

ACCOUNT=$(soroban config identity address acc1)

echo "Invoking auth_incr for: $ACCOUNT"

for i in {1..4}; do
  soroban contract invoke \
    --account $ACCOUNT \
    --wasm ../target/wasm32-unknown-unknown/release/tip_contract.wasm \
    --id 1 \
    --fn auth_incr \
    -- \
    --user $ACCOUNT \
    --value 2
done

echo "Result with auth payload payload: "

soroban contract invoke \
  --account $ACCOUNT \
  --auth \
  --wasm ../target/wasm32-unknown-unknown/release/tip_contract.wasm \
  --id 1 \
  --fn auth_incr \
  -- \
  --user $ACCOUNT \
  --value 2

echo "Current contract data:"
soroban contract read --id 1
