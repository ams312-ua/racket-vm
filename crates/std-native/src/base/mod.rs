mod comparison;
mod cons;
mod hof;
mod lists;
mod logic;
mod math;

use vm::{native::NativePlugins, plugin::NativePluginCollection};

pub struct DefaultPlugins;

impl NativePluginCollection for DefaultPlugins {
    fn register(self, registry: &mut NativePlugins) {
        comparison::BaseComparisonPlugins.register(registry);
        cons::BaseConsPlugin.register(registry);
        hof::BaseHOFPlugins.register(registry);
        lists::BaseListPlugin.register(registry);
        logic::BaseLogicPlugin.register(registry);
        math::BaseMathPlugin.register(registry);
    }
}
