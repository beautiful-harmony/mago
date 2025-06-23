use indoc::indoc;
use mago_reporting::*;
use mago_span::*;
use mago_syntax::ast::*;

use crate::context::LintContext;
use crate::definition::RuleDefinition;
use crate::definition::RuleUsageExample;
use crate::directive::LintDirective;
use crate::rule::Rule;

#[derive(Clone, Debug)]
pub struct BeginTransactionOutsideTryRule;

impl Rule for BeginTransactionOutsideTryRule {
    fn get_definition(&self) -> RuleDefinition {
        RuleDefinition::enabled(
            "begin-transaction-outside-try",
            Level::Warning,
        )
        .with_description(
            "Doctrine EntityManager beginTransaction() must be called outside try block to prevent potential rollback exceptions.",
        )
        .with_example(RuleUsageExample::valid(
            "beginTransaction() called before try block",
            indoc! {r#"
                <?php
                $entityManager->beginTransaction();
                try {
                    $entityManager->persist($entity);
                    $entityManager->flush();
                    $entityManager->commit();
                } catch (Exception $e) {
                    $entityManager->rollback();
                    throw $e;
                }
            "#}
        ))
        .with_example(RuleUsageExample::valid(
            "EntityManager obtained and transaction started outside try",
            indoc! {r#"
                <?php
                $em = $this->getDoctrine()->getManager();
                $em->beginTransaction();
                try {
                    // Database operations
                    $em->commit();
                } catch (Exception $e) {
                    $em->rollback();
                }
            "#}
        ))
        .with_example(RuleUsageExample::invalid(
            "beginTransaction() called inside try block",
            indoc! {r#"
                <?php
                try {
                    $entityManager->beginTransaction();
                    $entityManager->persist($entity);
                    $entityManager->flush();
                    $entityManager->commit();
                } catch (Exception $e) {
                    $entityManager->rollback();
                    throw $e;
                }
            "#}
        ))
        .with_example(RuleUsageExample::invalid(
            "EntityManager transaction started inside try block",
            indoc! {r#"
                <?php
                try {
                    $em = $this->getDoctrine()->getManager();
                    $em->beginTransaction();
                    // Database operations
                    $em->commit();
                } catch (Exception $e) {
                    $em->rollback();
                }
            "#}
        ))
    }

    fn lint_node(&self, node: Node<'_>, context: &mut LintContext<'_>) -> LintDirective {
        let Node::Statement(Statement::Try(try_statement)) = node else {
            return LintDirective::default();
        };

        let violations = find_begin_transaction_calls_in_try_block(&try_statement.block, context);

        if violations.is_empty() {
            return LintDirective::default();
        }

        for violation in violations {
            let issue = Issue::new(
                context.level(),
                "beginTransaction() should be called outside the try block to prevent potential rollback exceptions",
            )
            .with_annotation(
                Annotation::primary(violation.span())
                    .with_message("beginTransaction() called inside try block")
            )
            .with_annotation(
                Annotation::secondary(try_statement.r#try.span())
                    .with_message("try block begins here")
            )
            .with_help("Move the beginTransaction() call before the try block to avoid potential issues where rollback in catch block could throw another exception");

            context.report(issue);
        }

        LintDirective::default()
    }
}

fn find_begin_transaction_calls_in_try_block<'a>(block: &'a Block, context: &LintContext) -> Vec<&'a MethodCall> {
    let mut violations = Vec::new();

    for statement in block.statements.iter() {
        find_begin_transaction_calls_in_statement(statement, context, &mut violations);
    }

    violations
}

fn find_begin_transaction_calls_in_statement<'a>(
    statement: &'a Statement,
    context: &LintContext,
    violations: &mut Vec<&'a MethodCall>,
) {
    match statement {
        Statement::Expression(expr_stmt) => {
            find_begin_transaction_calls_in_expression(&expr_stmt.expression, context, violations);
        }
        Statement::Block(block_stmt) => {
            for stmt in block_stmt.statements.iter() {
                find_begin_transaction_calls_in_statement(stmt, context, violations);
            }
        }
        Statement::If(if_stmt) => {
            find_begin_transaction_calls_in_expression(&if_stmt.condition, context, violations);

            match &if_stmt.body {
                IfBody::Statement(body) => {
                    find_begin_transaction_calls_in_statement(&body.statement, context, violations);

                    for else_if_clause in body.else_if_clauses.iter() {
                        find_begin_transaction_calls_in_expression(&else_if_clause.condition, context, violations);
                        find_begin_transaction_calls_in_statement(&else_if_clause.statement, context, violations);
                    }

                    if let Some(else_clause) = &body.else_clause {
                        find_begin_transaction_calls_in_statement(&else_clause.statement, context, violations);
                    }
                }
                IfBody::ColonDelimited(body) => {
                    for stmt in body.statements.iter() {
                        find_begin_transaction_calls_in_statement(stmt, context, violations);
                    }

                    for else_if_clause in body.else_if_clauses.iter() {
                        find_begin_transaction_calls_in_expression(&else_if_clause.condition, context, violations);
                        for stmt in else_if_clause.statements.iter() {
                            find_begin_transaction_calls_in_statement(stmt, context, violations);
                        }
                    }

                    if let Some(else_clause) = &body.else_clause {
                        for stmt in else_clause.statements.iter() {
                            find_begin_transaction_calls_in_statement(stmt, context, violations);
                        }
                    }
                }
            }
        }
        Statement::While(while_stmt) => {
            find_begin_transaction_calls_in_expression(&while_stmt.condition, context, violations);

            match &while_stmt.body {
                WhileBody::Statement(stmt) => {
                    find_begin_transaction_calls_in_statement(stmt, context, violations);
                }
                WhileBody::ColonDelimited(body) => {
                    for stmt in body.statements.iter() {
                        find_begin_transaction_calls_in_statement(stmt, context, violations);
                    }
                }
            }
        }
        Statement::For(for_stmt) => {
            for expr in for_stmt.initializations.iter() {
                find_begin_transaction_calls_in_expression(expr, context, violations);
            }
            for expr in for_stmt.conditions.iter() {
                find_begin_transaction_calls_in_expression(expr, context, violations);
            }
            for expr in for_stmt.increments.iter() {
                find_begin_transaction_calls_in_expression(expr, context, violations);
            }

            match &for_stmt.body {
                ForBody::Statement(stmt) => {
                    find_begin_transaction_calls_in_statement(stmt, context, violations);
                }
                ForBody::ColonDelimited(body) => {
                    for stmt in body.statements.iter() {
                        find_begin_transaction_calls_in_statement(stmt, context, violations);
                    }
                }
            }
        }
        Statement::Foreach(foreach_stmt) => {
            find_begin_transaction_calls_in_expression(&foreach_stmt.expression, context, violations);

            match &foreach_stmt.body {
                ForeachBody::Statement(stmt) => {
                    find_begin_transaction_calls_in_statement(stmt, context, violations);
                }
                ForeachBody::ColonDelimited(body) => {
                    for stmt in body.statements.iter() {
                        find_begin_transaction_calls_in_statement(stmt, context, violations);
                    }
                }
            }
        }
        Statement::Try(try_stmt) => {
            for stmt in try_stmt.block.statements.iter() {
                find_begin_transaction_calls_in_statement(stmt, context, violations);
            }

            for catch_clause in try_stmt.catch_clauses.iter() {
                for stmt in catch_clause.block.statements.iter() {
                    find_begin_transaction_calls_in_statement(stmt, context, violations);
                }
            }

            if let Some(finally_clause) = &try_stmt.finally_clause {
                for stmt in finally_clause.block.statements.iter() {
                    find_begin_transaction_calls_in_statement(stmt, context, violations);
                }
            }
        }
        _ => {}
    }
}

fn find_begin_transaction_calls_in_expression<'a>(
    expression: &'a Expression,
    context: &LintContext,
    violations: &mut Vec<&'a MethodCall>,
) {
    match expression {
        Expression::Call(Call::Method(method_call)) => {
            if is_begin_transaction_call(method_call, context) {
                violations.push(method_call);
            }
            // Continue walking the object expression
            find_begin_transaction_calls_in_expression(&method_call.object, context, violations);
            for arg in method_call.argument_list.arguments.iter() {
                match arg {
                    Argument::Positional(pos_arg) => {
                        find_begin_transaction_calls_in_expression(&pos_arg.value, context, violations);
                    }
                    Argument::Named(named_arg) => {
                        find_begin_transaction_calls_in_expression(&named_arg.value, context, violations);
                    }
                }
            }
        }
        Expression::Call(Call::NullSafeMethod(method_call)) => {
            if is_begin_transaction_null_safe_call(method_call, context) {
                // Note: We can't easily convert NullSafeMethodCall to MethodCall here
                // For now, skip null-safe method calls
            }
            find_begin_transaction_calls_in_expression(&method_call.object, context, violations);
            for arg in method_call.argument_list.arguments.iter() {
                match arg {
                    Argument::Positional(pos_arg) => {
                        find_begin_transaction_calls_in_expression(&pos_arg.value, context, violations);
                    }
                    Argument::Named(named_arg) => {
                        find_begin_transaction_calls_in_expression(&named_arg.value, context, violations);
                    }
                }
            }
        }
        Expression::Access(Access::Property(prop_access)) => {
            find_begin_transaction_calls_in_expression(&prop_access.object, context, violations);
        }
        Expression::Access(Access::StaticProperty(static_prop)) => {
            find_begin_transaction_calls_in_expression(&static_prop.class, context, violations);
        }
        Expression::Call(Call::StaticMethod(static_call)) => {
            find_begin_transaction_calls_in_expression(&static_call.class, context, violations);
            for arg in static_call.argument_list.arguments.iter() {
                match arg {
                    Argument::Positional(pos_arg) => {
                        find_begin_transaction_calls_in_expression(&pos_arg.value, context, violations);
                    }
                    Argument::Named(named_arg) => {
                        find_begin_transaction_calls_in_expression(&named_arg.value, context, violations);
                    }
                }
            }
        }
        Expression::Call(Call::Function(func_call)) => {
            find_begin_transaction_calls_in_expression(&func_call.function, context, violations);
            for arg in func_call.argument_list.arguments.iter() {
                match arg {
                    Argument::Positional(pos_arg) => {
                        find_begin_transaction_calls_in_expression(&pos_arg.value, context, violations);
                    }
                    Argument::Named(named_arg) => {
                        find_begin_transaction_calls_in_expression(&named_arg.value, context, violations);
                    }
                }
            }
        }
        Expression::Assignment(assignment) => {
            find_begin_transaction_calls_in_expression(&assignment.lhs, context, violations);
            find_begin_transaction_calls_in_expression(&assignment.rhs, context, violations);
        }
        Expression::Binary(binary) => {
            find_begin_transaction_calls_in_expression(&binary.lhs, context, violations);
            find_begin_transaction_calls_in_expression(&binary.rhs, context, violations);
        }
        // Skip unary expressions as they don't contain method calls
        Expression::Conditional(conditional) => {
            find_begin_transaction_calls_in_expression(&conditional.condition, context, violations);
            if let Some(then_expr) = &conditional.then {
                find_begin_transaction_calls_in_expression(then_expr, context, violations);
            }
            find_begin_transaction_calls_in_expression(&conditional.r#else, context, violations);
        }
        Expression::Array(array) => {
            for element in array.elements.iter() {
                match element {
                    ArrayElement::KeyValue(kv) => {
                        find_begin_transaction_calls_in_expression(&kv.key, context, violations);
                        find_begin_transaction_calls_in_expression(&kv.value, context, violations);
                    }
                    ArrayElement::Value(value) => {
                        find_begin_transaction_calls_in_expression(&value.value, context, violations);
                    }
                    ArrayElement::Variadic(variadic) => {
                        find_begin_transaction_calls_in_expression(&variadic.value, context, violations);
                    }
                    ArrayElement::Missing(_) => {}
                }
            }
        }
        _ => {}
    }
}

fn is_begin_transaction_call(method_call: &MethodCall, context: &LintContext) -> bool {
    let method_name_str = match &method_call.method {
        ClassLikeMemberSelector::Identifier(local) => context.lookup(&local.value),
        ClassLikeMemberSelector::Variable(_) => return false, // Dynamic method names not supported
        ClassLikeMemberSelector::Expression(_) => return false, // Dynamic method names not supported
    };

    if !method_name_str.eq_ignore_ascii_case("beginTransaction") {
        return false;
    }

    // For now, use simple heuristics to identify EntityManager instances
    // This can be enhanced later with proper type checking
    is_likely_entity_manager_instance(&method_call.object, context)
}

fn is_begin_transaction_null_safe_call(method_call: &NullSafeMethodCall, context: &LintContext) -> bool {
    let method_name_str = match &method_call.method {
        ClassLikeMemberSelector::Identifier(local) => context.lookup(&local.value),
        ClassLikeMemberSelector::Variable(_) => return false, // Dynamic method names not supported
        ClassLikeMemberSelector::Expression(_) => return false, // Dynamic method names not supported
    };

    if !method_name_str.eq_ignore_ascii_case("beginTransaction") {
        return false;
    }

    is_likely_entity_manager_instance(&method_call.object, context)
}

fn is_likely_entity_manager_instance(expression: &Expression, context: &LintContext) -> bool {
    // Check if the expression type implements EntityManagerInterface or is an EntityManager instance
    // This follows the interface-first approach specified in the requirements
    
    // Try to resolve the type of the expression through reflection
    if let Some(expression_type) = resolve_expression_type(expression, context) {
        return is_entity_manager_type(&expression_type, context);
    }
    
    // Fallback to heuristic-based detection only when type information is unavailable
    fallback_entity_manager_detection(expression, context)
}

fn resolve_expression_type(expression: &Expression, context: &LintContext) -> Option<String> {
    match expression {
        Expression::Variable(var) => {
            // Try to resolve variable type from current scope
            if let Variable::Direct(_direct) = var {
                // Look up variable type in the current scope
                // This would need to be implemented with proper type resolution
                // For now, return None to indicate type resolution is not available
                None
            } else {
                None
            }
        }
        Expression::Access(Access::Property(prop_access)) => {
            // Try to resolve property type from class reflection
            if let ClassLikeMemberSelector::Identifier(_prop_name) = &prop_access.property {
                // This would need proper implementation to look up property type
                // from class reflection data
                None
            } else {
                None
            }
        }
        Expression::Call(Call::Method(method_call)) => {
            // Try to resolve return type of method call
            if let ClassLikeMemberSelector::Identifier(method_name) = &method_call.method {
                let method_name_str = context.lookup(&method_name.value);
                // Check if this is a known EntityManager factory method
                if is_known_entity_manager_factory_method(method_name_str) {
                    Some("Doctrine\\ORM\\EntityManager".to_string())
                } else {
                    None
                }
            } else {
                None
            }
        }
        _ => None,
    }
}

fn is_entity_manager_type(type_name: &str, _context: &LintContext) -> bool {
    // Check if the type implements EntityManagerInterface or is EntityManager
    type_name.contains("EntityManagerInterface") || 
    type_name.contains("EntityManager") ||
    type_name.ends_with("\\EntityManager")
}

fn is_known_entity_manager_factory_method(method_name: &str) -> bool {
    // These are well-known methods that return EntityManager instances
    matches!(method_name, 
        "getManager" | "getEntityManager" | "getDoctrine" | "getDoctrineManager"
    )
}

fn fallback_entity_manager_detection(expression: &Expression, context: &LintContext) -> bool {
    // Fallback heuristic-based detection when type information is unavailable
    // This is less reliable but necessary when reflection data is insufficient
    
    match expression {
        Expression::Call(Call::Method(method_call)) => {
            // Check for known EntityManager factory methods
            if let ClassLikeMemberSelector::Identifier(method_name) = &method_call.method {
                let method_name_str = context.lookup(&method_name.value);
                is_known_entity_manager_factory_method(method_name_str)
            } else {
                false
            }
        }
        Expression::Variable(var) => {
            // Fallback: Check for variables with clear EntityManager naming
            if let Variable::Direct(direct) = var {
                let var_name = context.lookup(&direct.name);
                var_name.eq_ignore_ascii_case("entityManager") || var_name.eq_ignore_ascii_case("em")
            } else {
                false
            }
        }
        Expression::Access(Access::Property(prop_access)) => {
            // Fallback: Check for properties with clear EntityManager naming
            if let ClassLikeMemberSelector::Identifier(prop_name) = &prop_access.property {
                let property_name_str = context.lookup(&prop_name.value);
                property_name_str.eq_ignore_ascii_case("entityManager")
            } else {
                false
            }
        }
        _ => false,
    }
}
