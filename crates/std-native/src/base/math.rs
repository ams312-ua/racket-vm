use common::value::{GCValue, GCValueExt, Value};
use std_native_macros::native_plugin;
use vm::{
    native::NativePlugins,
    plugin::{MaybeGcValue, NativeError, NativePluginCollection},
    vm::VM,
};

pub(crate) struct BaseMathPlugin;

impl NativePluginCollection for BaseMathPlugin {
    fn register(self, registry: &mut NativePlugins) {
        registry
            .register_plugin(base_add_plugin::plugin())
            .register_plugin(base_sub_plugin::plugin())
            .register_plugin(base_mul_plugin::plugin())
            .register_plugin(base_div_plugin::plugin())
            .register_plugin(base_expt_plugin::plugin());
    }
}

#[native_plugin(namespace = "base", name = "+", arity = 2, variadic = true)]
fn add(_: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    let mut result = args[0].into_value();

    for arg in &args[1..] {
        result = result.add(arg)?;
    }

    Ok(result.into())
}

#[native_plugin(namespace = "base", name = "-", arity = 2, variadic = true)]
fn sub(_: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    let mut result = args[0].into_value();

    for arg in &args[1..] {
        result = result.sub(arg)?;
    }

    Ok(result.into())
}

#[native_plugin(namespace = "base", name = "*", arity = 2, variadic = true)]
fn mul(_: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    let mut result = args[0].into_value();

    for arg in &args[1..] {
        result = result.mul(arg)?;
    }

    Ok(result.into())
}

#[native_plugin(namespace = "base", name = "/", arity = 2, variadic = true)]
fn div(_: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    let mut result = args[0].into_value();

    for arg in &args[1..] {
        result = result.div(arg)?;
    }

    Ok(result.into())
}

#[native_plugin(namespace = "base", name = "expt", arity = 2, variadic = false)]
fn expt(_: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    let base = args[0].into_value();
    let exponent = args[1].into_value();

    Ok(base.expt(&exponent)?.into())
}
