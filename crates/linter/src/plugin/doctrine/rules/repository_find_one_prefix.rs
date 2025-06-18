use indoc::indoc;

use mago_reporting::*;
use mago_span::*;
use mago_syntax::ast::*;

use crate::context::LintContext;
use crate::definition::RuleDefinition;
use crate::definition::RuleUsageExample;
use crate::directive::LintDirective;
use crate::rule::Rule;

#[derive(Clone, Copy, Debug)]
pub struct RepositoryFindOnePrefixRule;

impl Rule for RepositoryFindOnePrefixRule {
    fn get_definition(&self) -> RuleDefinition {
        RuleDefinition::enabled("RepositoryFindOnePrefix", Level::Warning)
            .with_description(indoc! {"
                Detects repository methods that return single entities but do not follow the 'findOne' prefix naming convention.
                Methods in Doctrine Repository classes that return a single entity should be prefixed with 'findOne'.
            "})
            .with_example(RuleUsageExample::valid(
                "Repository method with correct findOne prefix",
                indoc! {r#"
                    <?php

                    class UserRepository extends EntityRepository
                    {
                        public function findOneByEmail(string $email): ?User
                        {
                            return $this->findOneBy(['email' => $email]);
                        }

                        public function findOneActiveUser(): ?User
                        {
                            return $this->findOneBy(['active' => true]);
                        }
                    }
                "#},
            ))
            .with_example(RuleUsageExample::invalid(
                "Repository method without findOne prefix for single entity return",
                indoc! {r#"
                    <?php

                    class UserRepository extends EntityRepository
                    {
                        public function getByEmail(string $email): ?User
                        {
                            return $this->findOneBy(['email' => $email]);
                        }

                        public function getUserById(int $id): User
                        {
                            return $this->find($id);
                        }
                    }
                "#},
            ))
    }

    fn lint_node(&self, node: Node<'_>, context: &mut LintContext<'_>) -> LintDirective {
        let Node::Method(method) = node else { return LintDirective::default() };

        // Check if we're in a repository class by looking at the current scope
        if !self.is_in_repository_class(context) {
            return LintDirective::default();
        }

        // Only check public methods
        let is_public = method.modifiers.iter().any(|modifier| matches!(modifier, Modifier::Public(_)))
            || !method.modifiers.iter().any(|modifier| modifier.is_visibility());

        if !is_public {
            return LintDirective::default();
        }

        let method_name = context.lookup(&method.name.value);

        // Skip if already follows the convention or is a standard Doctrine method
        if method_name.starts_with("findOne") || self.is_standard_doctrine_method(method_name) {
            return LintDirective::default();
        }

        // Check if the method returns a single entity (nullable or non-nullable)
        if self.returns_single_entity(method, context) {
            context.report(
                Issue::new(
                    context.level(),
                    format!("Repository method `{method_name}` should be prefixed with 'findOne' as it returns a single entity."),
                )
                .with_annotation(
                    Annotation::primary(method.name.span())
                        .with_message(format!("Method `{method_name}` is declared here.")),
                )
                .with_annotation(
                    Annotation::secondary(method.span())
                        .with_message("Method definition")
                )
                .with_note(
                    "Methods that return single entities should follow the 'findOne' prefix convention for clarity and consistency."
                )
                .with_help(format!(
                    "Consider renaming to `findOne{}` to adhere to the naming convention.",
                    self.suggest_method_name(method_name)
                )),
            );
        }

        LintDirective::default()
    }
}

impl RepositoryFindOnePrefixRule {
    fn is_in_repository_class(&self, context: &LintContext<'_>) -> bool {
        // Check if we're currently in a class-like scope
        if let Some(class_scope) = context.scope.get_class_like_scope() {
            match class_scope {
                crate::scope::ClassLikeScope::Class(class_id)
                | crate::scope::ClassLikeScope::Interface(class_id)
                | crate::scope::ClassLikeScope::Trait(class_id)
                | crate::scope::ClassLikeScope::Enum(class_id) => {
                    let class_name = context.lookup(class_id);

                    // Check if class name contains "Repository"
                    if class_name.contains("Repository") {
                        return true;
                    }
                }
                crate::scope::ClassLikeScope::AnonymousClass(_) => {
                    // Anonymous classes cannot be repositories
                    return false;
                }
            }

            // For more sophisticated checking, we would need access to the extends clause
            // This could be enhanced by checking the reflection information
        }

        false
    }

    fn is_standard_doctrine_method(&self, method_name: &str) -> bool {
        matches!(
            method_name,
            "find"
                | "findBy"
                | "findOneBy"
                | "findAll"
                | "count"
                | "countBy"
                | "exists"
                | "existsBy"
                | "save"
                | "remove"
                | "flush"
                | "clear"
                | "getEntityName"
                | "getClassName"
                | "getEntityManager"
                | "createQueryBuilder"
                | "createQuery"
                | "createNativeQuery"
        )
    }

    fn returns_single_entity(&self, method: &Method, context: &LintContext<'_>) -> bool {
        let Some(return_type) = &method.return_type_hint else {
            return false;
        };

        self.is_single_entity_hint(&return_type.hint, context)
    }

    fn is_single_entity_hint(&self, hint: &Hint, context: &LintContext<'_>) -> bool {
        match hint {
            // Nullable types like ?User
            Hint::Nullable(nullable) => self.is_single_entity_hint(&nullable.hint, context),
            // Union types like User|null
            Hint::Union(union) => {
                // Check if one is null and the other is an entity
                let left_is_null = matches!(&*union.left, Hint::Null(_));
                let right_is_null = matches!(&*union.right, Hint::Null(_));
                let left_is_entity = self.is_entity_hint(&union.left, context);
                let right_is_entity = self.is_entity_hint(&union.right, context);

                (left_is_null && right_is_entity) || (right_is_null && left_is_entity)
            }
            // Regular types
            _ => self.is_entity_hint(hint, context),
        }
    }

    fn is_entity_hint(&self, hint: &Hint, context: &LintContext<'_>) -> bool {
        match hint {
            Hint::Identifier(identifier) => {
                let type_name = match identifier {
                    Identifier::Local(local) => context.lookup(&local.value),
                    Identifier::Qualified(qualified) => context.lookup(&qualified.value),
                    Identifier::FullyQualified(fq) => context.lookup(&fq.value),
                };

                // Skip primitive types and common non-entity types
                !matches!(type_name.to_lowercase().as_str(),
                    "int" | "integer" | "string" | "float" | "double" | "bool" | "boolean" |
                    "array" | "object" | "mixed" | "void" | "null" | "callable" | "resource" |
                    "iterable" | "static" | "self" | "parent" | "true" | "false" |
                    "datetime" | "datetimeinterface" | "dateinterval" | "datetimezone"
                ) &&
                // Assume uppercase-starting types are likely entities
                type_name.chars().next().is_some_and(|c| c.is_uppercase())
            }
            _ => false,
        }
    }

    fn suggest_method_name(&self, current_name: &str) -> String {
        // Remove common prefixes that don't follow the convention
        let cleaned = current_name
            .strip_prefix("get")
            .or_else(|| current_name.strip_prefix("fetch"))
            .or_else(|| current_name.strip_prefix("retrieve"))
            .or_else(|| current_name.strip_prefix("load"))
            .unwrap_or(current_name);

        // Capitalize first letter if needed
        if let Some(first_char) = cleaned.chars().next() {
            if first_char.is_lowercase() {
                format!("{}{}", first_char.to_uppercase(), &cleaned[1..])
            } else {
                cleaned.to_string()
            }
        } else {
            cleaned.to_string()
        }
    }
}
