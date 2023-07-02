use ruff_text_size::TextRange;
use rustpython_parser::ast::{self, Ranged};

use ruff_diagnostics::{Diagnostic, Violation};
use ruff_macros::{derive_message_formats, violation};

use crate::checkers::ast::Checker;

use crate::rules::wemake_python_styleguide::helpers::is_too_short_name;

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

pub(crate) struct Checkable(String, TextRange);
pub(crate) struct NotCheckable;

impl From<&ast::Identifier> for Checkable {
    fn from(value: &ast::Identifier) -> Self {
        Self(value.to_string(), value.range())
    }
}

impl From<&ast::Arg> for Checkable {
    fn from(value: &ast::Arg) -> Self {
        (&value.arg).into()
    }
}

impl TryFrom<&ast::Alias> for Checkable {
    type Error = NotCheckable;

    fn try_from(value: &ast::Alias) -> Result<Self, Self::Error> {
        match value {
            ast::Alias {
                asname: Some(identifier),
                ..
            } => Ok(identifier.into()),
            _ => Err(NotCheckable),
        }
    }
}

impl TryFrom<&ast::Expr> for Checkable {
    type Error = NotCheckable;

    fn try_from(value: &ast::Expr) -> Result<Self, Self::Error> {
        match value {
            ast::Expr::Name(ast::ExprName { id, range, .. }) => Ok(Self(id.to_string(), *range)),
            _ => Err(NotCheckable),
        }
    }
}

impl TryFrom<&Box<ast::Expr>> for Checkable {
    type Error = NotCheckable;

    fn try_from(value: &Box<ast::Expr>) -> Result<Self, Self::Error> {
        (&(**value)).try_into()
    }
}

/// WPS111
pub(crate) fn too_short_name(checker: &mut Checker, node: impl TryInto<Checkable>, trim: bool) {
    if let Ok(Checkable(name, range)) = node.try_into() {
        if is_too_short_name(
            &name,
            checker.settings.wemake_python_styleguide.min_name_length,
            trim,
        ) {
            checker
                .diagnostics
                .push(Diagnostic::new(TooShortName { name }, range));
        }
    };
}
