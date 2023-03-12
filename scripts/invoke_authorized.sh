soroban config identity generate acc1

ACCOUNT=$(soroban config identity address acc1)

echo "Invoking auth_incr for: $ACCOUNT"

soroban contract invoke \
  --account $ACCOUNT \
  --wasm ../target/wasm32-unknown-unknown/release/tips_4_goals.wasm \
  --id 1 \
  --fn auth_incr \
  -- \
  --user $ACCOUNT
  --value 2


