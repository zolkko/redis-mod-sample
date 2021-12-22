use redis_module::redisraw::bindings::{RedisModuleCtx, RedisModuleKey, RedisModuleString, RedisModule_Scan, RedisModule_ScanCursorCreate, RedisModule_ScanCursorDestroy};
use redis_module::{redis_command, redis_module, Context, NextArg, RedisError, RedisResult, RedisString};
use std::ffi::c_void;


fn callback(ctx: &Context, keyname: &RedisString, data: &mut Data) {
    let name = match keyname.try_as_str() {
        Ok(s) => s,
        Err(err) => {
            ctx.log_warning(&format!("key \"{}\"; {}", keyname, err));
            (*data).is_error = true;
            return;
        }
    };

    if name.starts_with(&(*data).prefix) {
        match ctx.open_key(&keyname).read() {
            Ok(Some(value)) => {
                match value.parse::<usize>() {
                    Ok(integer) =>  {
                        (*data).counter += integer;
                    },
                    Err(err) => {
                        ctx.log_warning(&format!("key \"{}\"; {}", &name, err));
                        (*data).is_error = true;
                    },
                }
            },
            Ok(None) => ctx.log_debug("the key has no value"),
            Err(err) => {
                ctx.log_warning(&format!("\"{}\"; {}", &name, err));
                (*data).is_error = true;
            }
        }
    }
}

fn redsum(ctx: &Context, args: Vec<RedisString>) -> RedisResult {
    if args.len() < 2 {
        return Err(RedisError::WrongArity);
    }
    let prefix_str = args.into_iter().skip(1).next_str()?;
    let mut data = Data {
        prefix: prefix_str.to_string(),
        counter: 0,
        is_error: false,
    };

    unsafe {
        let cursor_create = RedisModule_ScanCursorCreate.ok_or(RedisError::Str("ScanCursorCreate"))?;
        let cursor_scan = RedisModule_Scan.ok_or(RedisError::Str("Scan"))?;
        let cursor_destroy = RedisModule_ScanCursorDestroy.ok_or(RedisError::Str("ScanCursorDestroy"))?;
        let cursor = cursor_create();
        while cursor_scan(ctx.get_raw(), cursor, Some(scan_callback), &mut data as *mut Data as *mut c_void) > 0 {
            if data.is_error {
                break;
            }
        }
        cursor_destroy(cursor);
    }

    if data.is_error {
        Err(RedisError::Str("failed to compute the redsum"))
    } else {
        Ok(data.counter.into())
    }
}

struct Data {
    prefix: String,
    counter: usize,
    is_error: bool,
}

unsafe extern "C" fn scan_callback(ctx: *mut RedisModuleCtx, keyname: *mut RedisModuleString, _key: *mut RedisModuleKey, privdata: *mut c_void) {
    let data = &mut *(privdata as *mut Data);
    let ctx = Context::new(ctx);
    let keyname = RedisString::new(ctx.get_raw(), keyname);
    callback(&ctx, &keyname, data);
}

/// To load the module start Redis as: `redis-server --loadmodule ./target/debug/libredsum.dylib`
redis_module! {
    name: "redsum",
    version: 1,
    data_types: [],
    commands: [
        ["redsum", redsum, "", 0, 0, 0],
    ],
}

