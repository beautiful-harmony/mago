use mago_linter::plugin::doctrine_strict::rules::begin_transaction_outside_try::BeginTransactionOutsideTryRule;
use mago_linter::plugin::doctrine_strict::rules::repository_find_one_prefix::RepositoryFindOnePrefixRule;

use crate::rule_test;

rule_test!(test_begin_transaction_outside_try, BeginTransactionOutsideTryRule);
rule_test!(test_repository_find_one_prefix, RepositoryFindOnePrefixRule);
