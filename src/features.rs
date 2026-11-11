//! Source-level lowering for language features that expand into ordinary AST.

use std::collections::HashSet;

use crate::{
    ast::{Block, Expr, Item, Program, Span, Stmt},
    error::AlacoError,
    language::BinaryOp,
};

#[derive(Clone)]
struct DerivedValue {
    name: String,
    value: Expr,
    dependencies: HashSet<String>,
    span: Span,
}

/// Lower `derive` declarations to mutable storage plus dependency-triggered
/// assignments. Derived values are recalculated in declaration order, which
/// also gives chained derivations a deterministic topological order.
pub fn expand_derived(program: &mut Program) -> Result<(), AlacoError> {
    for item in &mut program.items {
        if let Item::Function(function) = item {
            expand_derived_block(&mut function.body, &[])?;
        }
    }
    Ok(())
}

fn expand_derived_block(block: &mut Block, inherited: &[DerivedValue]) -> Result<(), AlacoError> {
    let declarations: Vec<(String, Expr, Span)> = block
        .statements
        .iter()
        .filter_map(|statement| {
            if let Stmt::Derive { name, value, span } = statement {
                Some((name.clone(), value.clone(), *span))
            } else {
                None
            }
        })
        .collect();
    let derived_names: HashSet<String> = declarations
        .iter()
        .map(|(name, _, _)| name.clone())
        .collect();
    let mut graph = Vec::new();
    for (name, value, span) in &declarations {
        let mut dependencies = HashSet::new();
        collect_expression_names(value, &mut dependencies);
        dependencies.retain(|dependency| derived_names.contains(dependency));
        graph.push((name.clone(), dependencies, *span));
    }
    let mut visiting = HashSet::new();
    let mut visited = HashSet::new();
    for (name, _, span) in &graph {
        if has_cycle(name, &graph, &mut visiting, &mut visited) {
            return Err(AlacoError::Analysis {
                message: format!("circular derived-value dependency involving '{}'", name),
                span: Some((*span).into()),
            });
        }
    }
    let source = std::mem::take(&mut block.statements);
    let mut output = Vec::new();
    let mut derived: Vec<DerivedValue> = Vec::new();
    for mut statement in source {
        if let Stmt::Derive { name, value, span } = statement {
            let mut dependencies = HashSet::new();
            collect_expression_names(&value, &mut dependencies);
            output.push(Stmt::Let {
                name: name.clone(),
                mutable: true,
                ty: None,
                lifetime_ms: None,
                value: value.clone(),
                span,
            });
            derived.push(DerivedValue {
                name,
                value,
                dependencies,
                span,
            });
            continue;
        }

        let mut active = inherited.to_vec();
        active.extend(derived.iter().cloned());
        match &mut statement {
            Stmt::If {
                then_block,
                else_block,
                ..
            } => {
                expand_derived_block(then_block, &active)?;
                if let Some(else_block) = else_block {
                    expand_derived_block(else_block, &active)?;
                }
            }
            Stmt::Loop { body, .. } => expand_derived_block(body, &active)?,
            Stmt::Match { arms, .. } => {
                for arm in arms {
                    expand_derived_block(&mut arm.body, &active)?;
                }
            }
            Stmt::When { body, .. } => expand_derived_block(body, &active)?,
            Stmt::Watch { body, .. } => expand_derived_block(body, &active)?,
            Stmt::TryCatch {
                try_block,
                catch_block,
                ..
            } => {
                expand_derived_block(try_block, &active)?;
                expand_derived_block(catch_block, &active)?;
            }
            _ => {}
        }

        let mut writes = HashSet::new();
        collect_statement_writes(&statement, &mut writes);
        if let Some(derived_target) = inherited
            .iter()
            .chain(derived.iter())
            .find(|item| writes.contains(&item.name))
        {
            return Err(AlacoError::Analysis {
                message: format!(
                    "cannot assign directly to derived value '{}'",
                    derived_target.name
                ),
                span: Some(statement.span().into()),
            });
        }
        output.push(statement);

        if writes.is_empty() || (derived.is_empty() && inherited.is_empty()) {
            continue;
        }
        let mut changed = writes;
        let mut affected = Vec::new();
        for item in inherited.iter().chain(derived.iter()) {
            if item
                .dependencies
                .iter()
                .any(|dependency| changed.contains(dependency))
            {
                changed.insert(item.name.clone());
                affected.push(item);
            }
        }
        for item in affected {
            let target = Expr::Identifier(item.name.clone(), item.span);
            let assignment = Expr::Binary {
                left: Box::new(target),
                operator: BinaryOp::Assign,
                right: Box::new(item.value.clone()),
                span: item.span,
            };
            output.push(Stmt::Expr(assignment, item.span));
        }
    }
    block.statements = output;
    Ok(())
}

fn has_cycle(
    name: &str,
    graph: &[(String, HashSet<String>, Span)],
    visiting: &mut HashSet<String>,
    visited: &mut HashSet<String>,
) -> bool {
    if visiting.contains(name) {
        return true;
    }
    if visited.contains(name) {
        return false;
    }
    visiting.insert(name.to_owned());
    if let Some((_, dependencies, _)) = graph.iter().find(|(candidate, _, _)| candidate == name) {
        for dependency in dependencies {
            if has_cycle(dependency, graph, visiting, visited) {
                return true;
            }
        }
    }
    visiting.remove(name);
    visited.insert(name.to_owned());
    false
}

fn collect_block_names(block: &Block, names: &mut HashSet<String>) {
    for statement in &block.statements {
        collect_statement_names(statement, names);
    }
}

fn collect_statement_names(statement: &Stmt, names: &mut HashSet<String>) {
    match statement {
        Stmt::Let { value, .. } | Stmt::Destructure { value, .. } => {
            collect_expression_names(value, names)
        }
        Stmt::Return(Some(value), _) | Stmt::Throw(value, _) | Stmt::Stop(Some(value), _) => {
            collect_expression_names(value, names)
        }
        Stmt::If { condition, then_block, else_block, .. } => {
            collect_expression_names(condition, names);
            collect_block_names(then_block, names);
            if let Some(else_block) = else_block {
                collect_block_names(else_block, names);
            }
        }
        Stmt::Loop { body, .. } => collect_block_names(body, names),
        Stmt::Expr(expression, _) => collect_expression_names(expression, names),
        Stmt::Match { expression, arms, .. } => {
            collect_expression_names(expression, names);
            for arm in arms {
                collect_block_names(&arm.body, names);
            }
        }
        Stmt::When { condition, body, .. } => {
            collect_expression_names(condition, names);
            collect_block_names(body, names);
        }
        Stmt::Watch { body, .. } => collect_block_names(body, names),
        Stmt::TryCatch { try_block, catch_block, .. } => {
            collect_block_names(try_block, names);
            collect_block_names(catch_block, names);
        }
        Stmt::Return(None, _) | Stmt::Stop(None, _) | Stmt::Skip(_) | Stmt::Derive { .. } => {}
    }
}


fn generate_watch_condition(body: &Block, span: Span) -> Expr {
    let mut names = HashSet::new();
    collect_block_names(body, &mut names);

    let mut condition: Option<Expr> = None;
    for name in names {
        let left = Expr::Identifier(name.clone(), span);
        let right = Expr::Previous(name, span);
        let expr = Expr::Binary {
            left: Box::new(left),
            operator: BinaryOp::NotEqual,
            right: Box::new(right),
            span,
        };

        if let Some(c) = condition {
            condition = Some(Expr::Binary {
                left: Box::new(c),
                operator: BinaryOp::Or,
                right: Box::new(expr),
                span,
            });
        } else {
            condition = Some(expr);
        }
    }
    condition.unwrap_or(Expr::Bool(false, span))
}

fn collect_expression_names(expression: &Expr, names: &mut HashSet<String>) {
    match expression {
        Expr::Identifier(name, _) => {
            names.insert(name.clone());
        }
        Expr::Interpolation(parts, _) => {
            for name in parts.iter().skip(1).step_by(2) {
                names.insert(name.clone());
            }
        }
        Expr::StructLiteral { fields, .. } => {
            for (_, value) in fields {
                collect_expression_names(value, names);
            }
        }
        Expr::Binary { left, right, .. }
        | Expr::Pipe {
            input: left,
            target: right,
            ..
        } => {
            collect_expression_names(left, names);
            collect_expression_names(right, names);
        }
        Expr::Unary { operand, .. }
        | Expr::Member {
            object: operand, ..
        } => collect_expression_names(operand, names),
        Expr::Call {
            callee, arguments, ..
        } => {
            if !matches!(callee.as_ref(), Expr::Identifier(..)) {
                collect_expression_names(callee, names);
            }
            for argument in arguments {
                collect_expression_names(argument, names);
            }
        }
        Expr::Intrinsic { arguments, .. } => {
            for argument in arguments {
                collect_expression_names(argument, names);
            }
        }
        Expr::Number(..)
        | Expr::String(..)
        | Expr::Bool(..)
        | Expr::None(..)
        | Expr::Previous(..)
        | Expr::Shell { .. } => {}
    }
}

fn collect_statement_writes(statement: &Stmt, writes: &mut HashSet<String>) {
    match statement {
        Stmt::Expr(expression, _) => collect_expression_writes(expression, writes),
        Stmt::Let { value, .. } | Stmt::Destructure { value, .. } => {
            collect_expression_writes(value, writes)
        }
        Stmt::Return(Some(value), _) | Stmt::Stop(Some(value), _) => {
            collect_expression_writes(value, writes)
        }
        Stmt::Throw(value, _) => collect_expression_writes(value, writes),
        Stmt::If { condition, .. } => {
            collect_expression_writes(condition, writes);
        }
        Stmt::Loop { kind, .. } => match kind {
            crate::ast::LoopKind::Repeat(value) | crate::ast::LoopKind::While(value) => {
                collect_expression_writes(value, writes)
            }
            crate::ast::LoopKind::For { iterable, .. } => {
                collect_expression_writes(iterable, writes)
            }
            crate::ast::LoopKind::Infinite => {}
        },
        Stmt::Match { expression, .. } => {
            collect_expression_writes(expression, writes);
        }
        Stmt::When { condition, .. } => {
            collect_expression_writes(condition, writes);
        }
        Stmt::Return(None, _) | Stmt::Stop(None, _) | Stmt::Skip(_) | Stmt::Derive { .. } => {}
        Stmt::Watch { .. } | Stmt::TryCatch { .. } => {}
    }
}

fn collect_expression_writes(expression: &Expr, writes: &mut HashSet<String>) {
    match expression {
        Expr::Binary {
            left,
            operator,
            right,
            ..
        } => {
            if matches!(
                operator,
                BinaryOp::Assign
                    | BinaryOp::AddAssign
                    | BinaryOp::SubtractAssign
                    | BinaryOp::MultiplyAssign
                    | BinaryOp::DivideAssign
            ) {
                if let Expr::Identifier(name, _) = left.as_ref() {
                    writes.insert(name.clone());
                }
            }
            collect_expression_writes(left, writes);
            collect_expression_writes(right, writes);
        }
        Expr::StructLiteral { fields, .. } => {
            for (_, value) in fields {
                collect_expression_writes(value, writes);
            }
        }
        Expr::Pipe { input, target, .. } => {
            collect_expression_writes(input, writes);
            collect_expression_writes(target, writes);
        }
        Expr::Unary { operand, .. }
        | Expr::Member {
            object: operand, ..
        } => collect_expression_writes(operand, writes),
        Expr::Call {
            callee, arguments, ..
        } => {
            collect_expression_writes(callee, writes);
            for argument in arguments {
                collect_expression_writes(argument, writes);
            }
        }
        Expr::Intrinsic { arguments, .. } => {
            for argument in arguments {
                collect_expression_writes(argument, writes);
            }
        }
        Expr::Number(..)
        | Expr::String(..)
        | Expr::Interpolation(..)
        | Expr::Bool(..)
        | Expr::None(..)
        | Expr::Identifier(..)
        | Expr::Previous(..)
        | Expr::Shell { .. } => {}
    }
}

/// Expand `when` into condition checks after each statement in its lexical
/// block. The generated AST is then analyzed and lowered like ordinary code.
pub fn expand_when(program: &mut Program) {
    for item in &mut program.items {
        if let Item::Function(function) = item {
            expand_block(&mut function.body);
        }
    }
}

fn expand_block(block: &mut Block) {
    for statement in &mut block.statements {
        match statement {
            Stmt::If {
                then_block,
                else_block,
                ..
            } => {
                expand_block(then_block);
                if let Some(else_block) = else_block {
                    expand_block(else_block);
                }
            }
            Stmt::Loop { body, .. } => expand_block(body),
            Stmt::Match { arms, .. } => {
                for arm in arms {
                    expand_block(&mut arm.body);
                }
            }
            Stmt::When { body, .. } => expand_block(body),
            Stmt::Watch { body, .. } => expand_block(body),
            Stmt::TryCatch {
                try_block,
                catch_block,
                ..
            } => {
                expand_block(try_block);
                expand_block(catch_block);
            }
            _ => {}
        }
    }

    let mut source = std::mem::take(&mut block.statements);
    let mut output = Vec::new();
    let mut watchers: Vec<(Expr, Block, Span)> = Vec::new();

    // Re-process for Watch
    let mut new_source = Vec::new();
    for statement in source {
        if let Stmt::Watch { body, span } = statement {
            let condition = generate_watch_condition(&body, span);
            new_source.push(Stmt::When { condition, body, span });
        } else {
            new_source.push(statement);
        }
    }
    source = new_source;

    for statement in source {
        if let Stmt::When {
            condition,
            body,
            span,
        } = statement
        {
            watchers.push((condition, body, span));
            append_checks(&mut output, &watchers);
        } else {
            output.push(statement);
            append_checks(&mut output, &watchers);
        }
    }
    block.statements = output;
}

fn append_checks(output: &mut Vec<Stmt>, watchers: &[(Expr, Block, Span)]) {
    for (condition, body, span) in watchers {
        output.push(Stmt::If {
            condition: condition.clone(),
            then_block: body.clone(),
            else_block: None,
            span: *span,
        });
    }
}
