use common::value::{GCValue, Value, ValueExt};
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
            .register_plugin(base_or_plugin::plugin())
            .register_plugin(base_not_plugin::plugin())
            .register_plugin(base_nand_plugin::plugin())
            .register_plugin(base_nor_plugin::plugin());
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

#[native_plugin(namespace = "base", name = "not", arity = 1, variadic = false)]
fn not(_: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    let arg = &args[0];
    Ok(Value::Boolean(!arg.is_truthy()).into())
}

#[native_plugin(namespace = "base", name = "nand", arity = 1, variadic = true)]
fn nand(vm: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    let and_res = and(vm, args)?.into_gc_value();
    not(vm, &[and_res])
}

#[native_plugin(namespace = "base", name = "nor", arity = 1, variadic = true)]
fn nor(vm: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    let or_res = or(vm, args)?.into_gc_value();
    not(vm, &[or_res])
}
