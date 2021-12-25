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
(venv) % time python ./sum_rust.py
4999999950000000
python ./sum_rust.py  0.07s user 0.05s system 0% cpu 3:29.75 total

(venv) % time python ./sum_py.py
4999999950000000
python ./sum_py.py  4266.34s user 1881.99s system 66% cpu 2:34:12.24 total

(venv) % time redis-cli --eval sum_mod.lua
(integer) 4999999950000000
redis-cli --eval sum_mod.lua  0.00s user 0.01s system 0% cpu 3:35.99 total

(venv) % time redis-cli --eval sum_lua.lua
(integer) 4999999950000000
redis-cli --eval sum_lua.lua  0.00s user 0.01s system 0% cpu 3:45.67 total
```
