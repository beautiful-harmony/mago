use crate::definition::PluginDefinition;
use crate::plugin::doctrine::rules::repository_find_one_prefix::RepositoryFindOnePrefixRule;

use crate::plugin::Plugin;
use crate::rule::Rule;

pub mod rules;

#[derive(Debug)]
pub struct DoctrinePlugin;

impl Plugin for DoctrinePlugin {
    fn get_definition(&self) -> PluginDefinition {
        PluginDefinition {
            name: "Doctrine",
            description: "Provides rules that enforce best practices for Doctrine ORM usage.",
            enabled_by_default: false,
        }
    }

    fn get_rules(&self) -> Vec<Box<dyn Rule>> {
        vec![Box::new(RepositoryFindOnePrefixRule)]
    }
}
