# Error handler

## test all features

```cmd
cargo test --all-features
```

## test without features

```cmd
cargo test
```


## test code coverage

```cmd
cargo tarpaulin --all-features
```

## test code coverage with html report

```cmd
cargo tarpaulin --all-features --workspace --all-targets --out Html
```