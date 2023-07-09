use rustpython_parser::ast::Stmt;

use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::identifier::Identifier;

use crate::rules::wemake_python_styleguide::helpers::does_contain_underscored_number;

/// ## What it does
/// Forbid names with underscored numbers pattern.
///
/// ## Why is this bad?
/// Do not put an underscore between text and numbers,
/// that is confusing. Rename your variable or modules
/// do not include underscored numbers.
///
/// ## Example
/// ```python
/// star_wars_episode_2 = 'not so awesome'
/// iso_123_456 = 'some data' = 2
/// ```
///
/// Use instead:
/// ```python
/// star_wars_episode2 = 'awesome!'
/// iso123_456 = 'some data'
/// ```
#[violation]
pub struct UnderscoredNumberName {
    pub name: String,
}

impl Violation for UnderscoredNumberName {
    #[derive_message_formats]
    fn message(&self) -> String {
        let UnderscoredNumberName { name } = self;
        format!("Found underscored number name pattern: `{name}`")
    }
}

/// WPS114
pub(crate) fn underscored_number_name(stmt: &Stmt, name: &str) -> Option<Diagnostic> {
    if does_contain_underscored_number(name) {
        return Some(Diagnostic::new(
            UnderscoredNumberName {
                name: name.to_string(),
            },
            stmt.identifier(),
        ));
    }
    None
}
