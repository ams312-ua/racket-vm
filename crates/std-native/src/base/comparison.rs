use common::value::{GCValue, Value};
use std_native_macros::native_plugin;
use vm::{
    native::NativePlugins,
    plugin::{MaybeGcValue, NativeError, NativePluginCollection},
    vm::VM,
};

pub(crate) struct BaseComparisonPlugins;

impl NativePluginCollection for BaseComparisonPlugins {
    fn register(self, registry: &mut NativePlugins) {
        registry
            .register_plugin(base_eq_plugin::plugin())
            .register_plugin(base_gt_plugin::plugin())
            .register_plugin(base_lt_plugin::plugin())
            .register_plugin(base_ge_plugin::plugin())
            .register_plugin(base_le_plugin::plugin());
    }
}

#[native_plugin(namespace = "base", name = "=", arity = 2, variadic = true)]
fn eq(_: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    let first = &args[0];

    for arg in &args[1..] {
        if !Value::eq(first.as_ref(), arg.as_ref())? {
            return Ok(Value::Boolean(false).into());
        }
    }

    Ok(Value::Boolean(true).into())
}

#[native_plugin(namespace = "base", name = ">", arity = 2, variadic = true)]
fn gt(_: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    let first = &args[0];

    for arg in &args[1..] {
        if !Value::gt(first.as_ref(), arg.as_ref())? {
            return Ok(Value::Boolean(false).into());
        }
    }

    Ok(Value::Boolean(true).into())
}

#[native_plugin(namespace = "base", name = "<", arity = 2, variadic = true)]
fn lt(_: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    let first = &args[0];

    for arg in &args[1..] {
        if !Value::lt(first.as_ref(), arg.as_ref())? {
            return Ok(Value::Boolean(false).into());
        }
    }

    Ok(Value::Boolean(true).into())
}

#[native_plugin(namespace = "base", name = ">=", arity = 2, variadic = true)]
fn ge(_: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    let first = &args[0];

    for arg in &args[1..] {
        if !Value::ge(first.as_ref(), arg.as_ref())? {
            return Ok(Value::Boolean(false).into());
        }
    }

    Ok(Value::Boolean(true).into())
}

#[native_plugin(namespace = "base", name = "<=", arity = 2, variadic = true)]
fn le(_: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    let first = &args[0];

    for arg in &args[1..] {
        if !Value::le(first.as_ref(), arg.as_ref())? {
            return Ok(Value::Boolean(false).into());
        }
    }

    Ok(Value::Boolean(true).into())
}
