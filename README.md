# How to verify Ethereum personal_sign in NEAR Protocol with Rust?

Answer: use `env::ecrecover` function

Example at `contract/src/lib.rs`

Test at `integration-tests/src/main.ava.ts`

```
npm i
npm run test:integration
```

