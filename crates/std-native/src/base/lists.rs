use common::value::{GCValue, GCValueExt, Value, ValueExt};
use std_native_macros::native_plugin;
use vm::{
    native::NativePlugins,
    plugin::{MaybeGcValue, NativeError, NativePluginCollection},
    vm::VM,
};

pub(crate) struct BaseListPlugin;
impl NativePluginCollection for BaseListPlugin {
    fn register(self, registry: &mut NativePlugins) {
        registry
            .register_plugin(base_list_plugin::plugin())
            .register_plugin(base_len_plugin::plugin())
            .register_plugin(base_append_plugin::plugin())
            .register_plugin(base_reverse_plugin::plugin())
            .register_plugin(base_is_null_plugin::plugin())
            .register_plugin(base_build_list_plugin::plugin())
            .register_plugin(base_first_plugin::plugin())
            .register_plugin(base_second_plugin::plugin())
            .register_plugin(base_third_plugin::plugin())
            .register_plugin(base_forth_plugin::plugin())
            .register_plugin(base_rest_plugin::plugin())
            .register_plugin(base_is_list_plugin::plugin());
    }
}

#[native_plugin(namespace = "base", name = "list", arity = 0, variadic = true)]
fn list(_: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    if args.is_empty() {
        return Ok(Value::Null.into());
    }

    let mut items = Vec::with_capacity(args.len());

    for arg in args {
        items.push(arg.clone());
    }

    Ok(Value::list(items).into())
}

#[native_plugin(namespace = "base", name = "length", arity = 0, variadic = false)]
fn len(_: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    let item = &args[0];

    if !item.is_list() {
        return Err(NativeError::InvalidType {
            expected: "List",
            got: item.data_type_name(),
        });
    }

    let mut len = 1;
    let mut current = item.as_ref();

    while let Value::Pair {
        cdr, is_list: true, ..
    } = current
    {
        len += 1;
        current = cdr.as_ref();
    }

    Ok(Value::Integer(len).into())
}

#[native_plugin(namespace = "base", name = "append", arity = 2, variadic = false)]
fn append(_: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    let head = &args[0];
    let tail = &args[1];

    if !head.is_list() {
        return Err(NativeError::InvalidType {
            expected: "List",
            got: head.data_type_name(),
        });
    }

    let mut list_items = head
        .iter_list()?
        .map(|item| item.into_value())
        .collect::<Vec<_>>();

    // If tail is list, we just need to clone the head and link the last cdr
    // to the head of the tail.
    if tail.is_list() {
        // We build the new list in reverse order, starting from the tail.
        let mut current = tail.clone();

        // reverse it cuz we need to link from last to first
        list_items.reverse();
        for item in list_items {
            current = Value::pair(item, current).into_gc_value();
        }

        Ok(current.into_value().into())
    } else {
        // if not, we just push it to the end of the list and return it as a new list.
        list_items.push(tail.into_value());
        Ok(Value::list(list_items).into())
    }
}

#[native_plugin(namespace = "base", name = "reverse", arity = 1, variadic = false)]
fn reverse(_: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    let item = &args[0];

    if !item.is_list() {
        return Err(NativeError::InvalidType {
            expected: "List",
            got: item.data_type_name(),
        });
    }

    let mut list_items = item
        .iter_list()?
        .map(|item| item.into_value())
        .collect::<Vec<_>>();

    list_items.reverse();
    Ok(Value::list(list_items).into())
}

#[native_plugin(namespace = "base", name = "null?", arity = 1, variadic = false)]
fn is_null(_: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    let item = &args[0];
    Ok(Value::Boolean(item.is_null()).into())
}

#[native_plugin(namespace = "base", name = "build-list", arity = 2, variadic = false)]
fn build_list(vm: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    let Value::Integer(n) = args[0].as_ref() else {
        return Err(NativeError::InvalidType {
            expected: "Integer",
            got: args[0].data_type_name(),
        });
    };

    let Value::Function(ident) = args[1].as_ref() else {
        return Err(NativeError::InvalidType {
            expected: "Function",
            got: args[1].data_type_name(),
        });
    };

    let items = (0..*n)
        .map(|i| {
            let arg = Value::Integer(i).into();
            vm.call(ident.clone(), vec![arg]).map_err(Box::new)
        })
        .collect::<Result<Vec<GCValue>, _>>()?;

    Ok(Value::list(items).into())
}

#[native_plugin(namespace = "base", name = "first", arity = 1, variadic = false)]
fn first(_: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    let item = &args[0];

    if !item.is_list() {
        return Err(NativeError::InvalidType {
            expected: "List",
            got: item.data_type_name(),
        });
    }

    if let Value::Pair { car, .. } = item.as_ref() {
        Ok(car.clone().into())
    } else {
        Ok(Value::Null.into())
    }
}

#[native_plugin(namespace = "base", name = "second", arity = 1, variadic = false)]
fn second(_: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    let item = &args[0];

    item.iter_list()?.nth(1).map(|item| item.into_value().into()).ok_or_else(|| NativeError::InvalidType {
        expected: "List with at least 2 items",
        got: item.data_type_name(),
    })
}

#[native_plugin(namespace = "base", name = "third", arity = 1, variadic = false)]
fn third(_: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    let item = &args[0];

    item.iter_list()?.nth(2).map(|item| item.into_value().into()).ok_or_else(|| NativeError::InvalidType {
        expected: "List with at least 3 items",
        got: item.data_type_name(),
    })
}

#[native_plugin(namespace = "base", name = "forth", arity = 1, variadic = false)]
fn forth(_: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    let item = &args[0];

    item.iter_list()?.nth(3).map(|item| item.into_value().into()).ok_or_else(|| NativeError::InvalidType {
        expected: "List with at least 4 items",
        got: item.data_type_name(),
    })
}

#[native_plugin(namespace = "base", name = "rest", arity = 1, variadic = false)]
fn rest(_: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    let item = &args[0];

    if !item.is_list() {
        return Err(NativeError::InvalidType {
            expected: "List",
            got: item.data_type_name(),
        });
    }

    if let Value::Pair { cdr, .. } = item.as_ref() {
        Ok(cdr.clone().into())
    } else {
        Ok(Value::Null.into())
    }
}

#[native_plugin(namespace = "base", name = "list?", arity = 1, variadic = false)]
fn is_list(_: &mut VM, args: &[GCValue]) -> Result<MaybeGcValue, NativeError> {
    let item = &args[0];
    Ok(Value::Boolean(item.is_list()).into())
}
