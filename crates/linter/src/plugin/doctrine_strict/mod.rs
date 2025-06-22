use crate::definition::PluginDefinition;
use crate::plugin::doctrine_strict::rules::repository_find_one_prefix::RepositoryFindOnePrefixRule;

use crate::plugin::Plugin;
use crate::rule::Rule;

pub mod rules;

#[derive(Debug)]
pub struct DoctrineStrictPlugin;

impl Plugin for DoctrineStrictPlugin {
    fn get_definition(&self) -> PluginDefinition {
        PluginDefinition {
            name: "DoctrineStrict",
            description: "Provides strict rules for Doctrine ORM usage and best practices.",
            enabled_by_default: false,
        }
    }

    fn get_rules(&self) -> Vec<Box<dyn Rule>> {
        vec![Box::new(RepositoryFindOnePrefixRule)]
    }
}
