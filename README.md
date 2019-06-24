# cargo workspace && mod learning

## for workspace (with build )

* build

```code
cargo build --all
```

* running binary file

```code
root:
target/debug/mod-app

lib/usersmain:
target/debug/usersmain
```

## test

* root running

```code
cargo run
```

* running usersmain (workspace)

```code
cargo run  -p usersmain
```