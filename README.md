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

## Results

On my machine the results are the following:

```sh
(venv)  % time python sum_py.py
4999950000
python sum_py.py  4.11s user 1.73s system 67% cpu 8.648 total

(venv) % time python sum_rust.py
4999950000
python sum_rust.py  0.05s user 0.02s system 48% cpu 0.157 total
```
