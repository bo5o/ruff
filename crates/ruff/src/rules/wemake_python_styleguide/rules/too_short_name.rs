use rustpython_parser::ast::Stmt;

use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::identifier::Identifier;

/// ## What it does
/// Checks for short variable or module names.
///
/// ## Why is this bad?
/// It is hard to understand what the variable means and why it is used, if its name is too short.
///
/// ## Options
/// - `wemake-python-styleguide.min-name-length`
///
/// ## Example
/// ```python
/// x = 1
/// y = 2
/// ```
///
/// Use instead:
/// ```python
/// x_coordinate = 1
/// abscissa = 2
/// ```
#[violation]
pub struct TooShortName {
    pub name: String,
}

impl Violation for TooShortName {
    #[derive_message_formats]
    fn message(&self) -> String {
        let TooShortName { name } = self;
        format!("Found too short name: `{name}`")
    }
}

/// WPS111
pub(crate) fn too_short_name(
    stmt: &Stmt,
    name: &str,
    min_name_length: usize,
) -> Option<Diagnostic> {
    if name.len() >= min_name_length {
        return None;
    }

    Some(Diagnostic::new(
        TooShortName {
            name: name.to_string(),
        },
        stmt.identifier(),
    ))
}
