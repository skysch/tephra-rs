////////////////////////////////////////////////////////////////////////////////
// Tephra parser library
////////////////////////////////////////////////////////////////////////////////
// Copyright 2020 Skylor R. Schermer
// This code is dual licenced using the MIT or Apache 2 license.
// See licence-mit.md and licence-apache.md for details.
////////////////////////////////////////////////////////////////////////////////
//! Atma parser & tests.
////////////////////////////////////////////////////////////////////////////////

// Internal modules.
mod ast;
mod ast_match;
mod color;
mod color_expr;
mod common;
mod expr;
mod function;
mod scanner;
mod selection;

// Exports.
pub use ast::*;
pub use ast_match::*;
pub use color_expr::*;
pub use common::*;
pub use expr::*;
pub use function::*;
pub use scanner::*;
pub use selection::*;
pub use self::color::*;

