use common::value::{GCValue, Value};
use std_native_macros::native_plugin;
use vm::{
    plugin::{MaybeGcValue, NativeError, NativePluginCollection},
    vm::VM,
};

pub(crate) struct BaseLogicPlugin;

impl NativePluginCollection for BaseLogicPlugin {
    fn register(self, registry: &mut vm::native::NativePlugins) {
        registry
            .register_plugin(base_and_plugin::plugin())
            .register_plugin(base_or_plugin::plugin());
    }
}

#[native_plugin(namespace = "base", name = "and", arity = 0, variadic = true)]
fn and(_: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    if args.is_empty() {
        return Ok(MaybeGcValue::Value(Value::Boolean(true)));
    }

    for arg in args {
        if !arg.is_truthy() {
            return Ok(MaybeGcValue::Value(Value::Boolean(false)));
        }
    }

    // return tail
    Ok(args.last().cloned().unwrap().into())
}

#[native_plugin(namespace = "base", name = "or", arity = 0, variadic = true)]
fn or(_: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    if args.is_empty() {
        return Ok(MaybeGcValue::Value(Value::Boolean(false)));
    }

    for arg in args {
        if arg.is_truthy() {
            return Ok(arg.clone().into());
        }
    }

    Ok(MaybeGcValue::Value(Value::Boolean(false)))
}
