use std::collections::HashMap;

use rspack_core::{ConstDependency, Plugin, SpanExt};
use swc_core::common::Spanned;

use crate::parser_plugin::JavascriptParserPlugin;

type DefineValue = HashMap<String, String>;

#[derive(Debug)]
pub struct DefinePlugin;

impl Plugin for DefinePlugin {
  fn name(&self) -> &'static str {
    "rspack.DefinePlugin"
  }
}

fn dep(
  parser: &mut crate::visitors::JavascriptParser,
  for_name: &str,
  definitions: &DefineValue,
  start: u32,
  end: u32,
) -> Option<ConstDependency> {
  if let Some(value) = definitions.get(for_name) {
    if parser.in_short_hand {
      return Some(ConstDependency::new(
        start,
        end,
        format!("{for_name}: {value}").into(),
        None,
      ));
    } else {
      return Some(ConstDependency::new(
        start,
        end,
        value.to_string().into(),
        None,
      ));
    }
  }
  None
}

impl JavascriptParserPlugin for DefinePlugin {
  fn can_rename(&self, parser: &mut crate::visitors::JavascriptParser, str: &str) -> Option<bool> {
    // if parser.compiler_options.builtins.define.get(str).is_some() {
    //   return Some(true);
    // }
    None
  }

  fn call(
    &self,
    parser: &mut crate::visitors::JavascriptParser,
    expr: &swc_core::ecma::ast::CallExpr,
    for_name: &str,
  ) -> Option<bool> {
    dep(
      parser,
      for_name,
      &parser.compiler_options.builtins.define,
      expr.callee.span().real_lo(),
      expr.callee.span().real_hi(),
    )
    .map(|dep| {
      parser.presentational_dependencies.push(Box::new(dep));
      // FIXME: webpack use `walk_expression` here
      parser.walk_expr_or_spread(&expr.args);
      true
    })
  }

  fn member(
    &self,
    parser: &mut crate::visitors::JavascriptParser,
    expr: &swc_core::ecma::ast::MemberExpr,
    for_name: &str,
  ) -> Option<bool> {
    dep(
      parser,
      for_name,
      &parser.compiler_options.builtins.define,
      expr.span().real_lo(),
      expr.span().real_hi(),
    )
    .map(|dep| {
      parser.presentational_dependencies.push(Box::new(dep));
      true
    })
  }

  fn identifier(
    &self,
    parser: &mut crate::visitors::JavascriptParser,
    ident: &swc_core::ecma::ast::Ident,
    for_name: &str,
  ) -> Option<bool> {
    dep(
      parser,
      for_name,
      &parser.compiler_options.builtins.define,
      ident.span.real_lo(),
      ident.span.real_hi(),
    )
    .map(|dep| {
      parser.presentational_dependencies.push(Box::new(dep));
      true
    })
  }
}
