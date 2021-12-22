# redis-mod-sample

This is a sample Redis module which adds a "redsum" command.
The command sums keys by keys' prefix.

```redis-cli
> redsum test_
```

## Building and running the module

Build the module using `cargo` command.
```sh
cargo build --release
```

Once the module is built, run redis-server and load the module.

```sh
redis-server --loadmodule ./target/debug/libredsum.dylib
```

## Testing

You can add some test data by running the `setup_data.py` script.
Two other Python scripts are for summing the keys using pure Python and the native module.