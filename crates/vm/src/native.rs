use rapidhash::{HashMapExt, RapidHashMap};
use crate::plugin::{NativePlugin, NativePluginCollection};

pub struct NativePlugins {
    /// Namespace storage, used to store the namespaces of the plugins and point to the actual plugin instances in the `plugins` vector.
    namespaces: RapidHashMap<Box<str>, Vec<NativePlugin>>,
    /// Function lookup for plugins whose namespaces have been activated.
    active: RapidHashMap<Box<str>, NativePlugin>,
}

impl NativePlugins {
    pub(crate) fn new() -> Self {
        Self {
            namespaces: RapidHashMap::new(),
            active: RapidHashMap::new(),
        }
    }

    /// Registers a new native plugin, adding it to the `plugins` vector and updating the `namespaces` map accordingly.
    pub fn register_plugin(&mut self, plugin: NativePlugin) -> &mut Self {
        let namespace = plugin.namespace();
        
        self.namespaces.entry(namespace.into()).or_default().push(plugin);

        self
    }

    pub fn register_collection(&mut self, collection: impl NativePluginCollection) {
        collection.register(self);
    }

    /// Activates a namespace, adding all its plugins to the `active` map for function lookup.
    pub fn activate_namespace(&mut self, namespace: &str) {
        if let Some(plugins) = self.namespaces.get(namespace) {
            for plugin in plugins {
                self.active.insert(plugin.name().into(), *plugin);
            }
        }
    }

    /// Deactivates a namespace, removing all its plugins from the `active` map.
    pub fn clear_active_namespaces(&mut self) {
        self.active.clear();
    }

    /// Gets a plugin by its name from the `active` map, returning a reference to the plugin if found.
    pub fn get_plugin<'a>(&'a self, name: &str) -> Option<NativePlugin> {
        self.active.get(name).copied()
    }
}
