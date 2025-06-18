use mago_linter::plugin::doctrine::rules::repository_find_one_prefix::RepositoryFindOnePrefixRule;

use crate::rule_test;

rule_test!(test_repository_find_one_prefix, RepositoryFindOnePrefixRule);
