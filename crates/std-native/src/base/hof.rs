use common::value::{GCValue, GCValueExt, Value};
use std_native_macros::native_plugin;
use vm::{
    native::NativePlugins,
    plugin::{MaybeGcValue, NativeError, NativePluginCollection},
    vm::VM,
};

pub(crate) struct BaseHOFPlugins;

impl NativePluginCollection for BaseHOFPlugins {
    fn register(self, registry: &mut NativePlugins) {
        registry
            .register_plugin(base_map_plugin::plugin())
            .register_plugin(base_filter_plugin::plugin())
            .register_plugin(base_foldl_plugin::plugin());
    }
}

#[native_plugin(namespace = "base", name = "map", arity = 2, variadic = false)]
fn map(vm: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    let f = &args[0];
    let list = &args[1];

    let Value::Function(f) = f.as_ref() else {
        return Err(NativeError::InvalidType {
            expected: "function",
            got: f.data_type_name(),
        });
    };

    let mut result = Vec::new();
    for item in list.iter_list()? {
        result.push(vm.call(f.clone(), vec![item]).map_err(Box::new)?);
    }

    Ok(Value::list(result).into())
}

#[native_plugin(namespace = "base", name = "filter", arity = 2, variadic = false)]
fn filter(vm: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    let f = &args[0];
    let list = &args[1];

    let Value::Function(f) = f.as_ref() else {
        return Err(NativeError::InvalidType {
            expected: "function",
            got: f.data_type_name(),
        });
    };

    let mut result = Vec::new();
    for item in list.iter_list()? {
        let res = vm.call(f.clone(), vec![item.clone()]).map_err(Box::new)?;

        if res.is_truthy() {
            result.push(item);
        }
    }

    Ok(Value::list(result).into())
}

#[native_plugin(namespace = "base", name = "foldl", arity = 3, variadic = true)]
fn foldl(vm: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    let f = &args[0];
    let list = &args[1];
    let init = &args[2];
    let first_pair = &args[3];

    let Value::Function(f) = f.as_ref() else {
        return Err(NativeError::InvalidType {
            expected: "function",
            got: f.data_type_name(),
        });
    };

    println!("{:?}", args);

    todo!()
}
