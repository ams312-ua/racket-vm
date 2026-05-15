use common::value::{GCValue, Value};
use std_native_macros::native_plugin;
use vm::{
    native::NativePlugins, plugin::{MaybeGcValue, NativeError, NativePluginCollection}, vm::VM
};

pub(crate) struct BaseConsPlugin;

impl NativePluginCollection for BaseConsPlugin {
    fn register(self, registry: &mut NativePlugins) {
        registry
            .register_plugin(base_cons_plugin::plugin())
            .register_plugin(base_car_plugin::plugin())
            .register_plugin(base_cdr_plugin::plugin());
    }
}

#[native_plugin(namespace = "base", name = "cons", arity = 2, variadic = false)]
fn cons(_: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    let car = args[0].clone();
    let cdr = args[1].clone();

    Ok(Value::pair(car, cdr).into())
}

#[native_plugin(namespace = "base", name = "car", arity = 1, variadic = false)]
fn car(_: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    if !args[0].is_cons_like() {
        return Err(NativeError::InvalidType {
            expected: "cons-like value (pair or list)",
            got: args[0].data_type_name(),
        });
    }

    match args[0].as_ref() {
        Value::Pair { car, .. } => Ok(car.clone().into()),
        _ => unreachable!(), // We already checked it's cons-like
    }
}

#[native_plugin(namespace = "base", name = "cdr", arity = 1, variadic = false)]
fn cdr(_: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    if !args[0].is_cons_like() {
        return Err(NativeError::InvalidType {
            expected: "cons-like value (pair or list)",
            got: args[0].data_type_name(),
        });
    }

    match args[0].as_ref() {
        Value::Pair { cdr, .. } => Ok(cdr.clone().into()),
        _ => unreachable!(), // We already checked it's cons-like
    }
}
