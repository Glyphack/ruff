use ruff_python_ast as ast;
use ruff_python_ast::visitor::Visitor;
use ruff_python_ast::{Arguments, Expr, Stmt, visitor};
use ruff_python_semantic::analyze::function_type;
use ruff_python_semantic::{ScopeKind, SemanticModel};
use ruff_text_size::TextRange;

use crate::settings::LinterSettings;

/// Returns the value of the `name` parameter to, e.g., a `TypeVar` constructor.
pub(super) fn type_param_name(arguments: &Arguments) -> Option<&str> {
    // Handle both `TypeVar("T")` and `TypeVar(name="T")`.
    let name_param = arguments.find_argument_value("name", 0)?;
    if let Expr::StringLiteral(ast::ExprStringLiteral { value, .. }) = &name_param {
        Some(value.to_str())
    } else {
        None
    }
}

pub(super) fn in_dunder_method(
    dunder_name: &str,
    semantic: &SemanticModel,
    settings: &LinterSettings,
) -> bool {
    let scope = semantic.current_scope();
    let ScopeKind::Function(ast::StmtFunctionDef {
        name,
        decorator_list,
        ..
    }) = scope.kind
    else {
        return false;
    };
    if name != dunder_name {
        return false;
    }
    let Some(parent) = semantic.first_non_type_parent_scope(scope) else {
        return false;
    };

    if !matches!(
        function_type::classify(
            name,
            decorator_list,
            parent,
            semantic,
            &settings.pep8_naming.classmethod_decorators,
            &settings.pep8_naming.staticmethod_decorators,
        ),
        function_type::FunctionType::Method
    ) {
        return false;
    }
    true
}

/// Visitor to track reads from an iterable in a loop.
#[derive(Debug)]
pub(crate) struct SequenceIndexVisitor<'a> {
    /// `letters`, given `for index, letter in enumerate(letters)`.
    sequence_name: &'a str,
    /// `index`, given `for index, letter in enumerate(letters)`.
    index_name: &'a str,
    /// `letter`, given `for index, letter in enumerate(letters)`.
    value_name: &'a str,
    /// The ranges of any `letters[index]` accesses.
    accesses: Vec<TextRange>,
    /// Whether any of the variables have been modified.
    modified: bool,
}

impl<'a> SequenceIndexVisitor<'a> {
    pub(crate) fn new(sequence_name: &'a str, index_name: &'a str, value_name: &'a str) -> Self {
        Self {
            sequence_name,
            index_name,
            value_name,
            accesses: Vec::new(),
            modified: false,
        }
    }

    pub(crate) fn into_accesses(self) -> Vec<TextRange> {
        self.accesses
    }
}

impl SequenceIndexVisitor<'_> {
    fn is_assignment(&self, expr: &Expr) -> bool {
        // If we see the sequence, a subscript, or the index being modified, we'll stop emitting
        // diagnostics.
        match expr {
            Expr::Name(ast::ExprName { id, .. }) => {
                id == self.sequence_name || id == self.index_name || id == self.value_name
            }
            Expr::Subscript(ast::ExprSubscript { value, slice, .. }) => {
                let Expr::Name(ast::ExprName { id, .. }) = value.as_ref() else {
                    return false;
                };
                if id == self.sequence_name {
                    let Expr::Name(ast::ExprName { id, .. }) = slice.as_ref() else {
                        return false;
                    };
                    if id == self.index_name {
                        return true;
                    }
                }
                false
            }
            _ => false,
        }
    }
}

impl Visitor<'_> for SequenceIndexVisitor<'_> {
    fn visit_stmt(&mut self, stmt: &Stmt) {
        if self.modified {
            return;
        }
        match stmt {
            Stmt::Assign(ast::StmtAssign { targets, value, .. }) => {
                self.modified = targets.iter().any(|target| self.is_assignment(target));
                self.visit_expr(value);
            }
            Stmt::AnnAssign(ast::StmtAnnAssign { target, value, .. }) => {
                if let Some(value) = value {
                    self.modified = self.is_assignment(target);
                    self.visit_expr(value);
                }
            }
            Stmt::AugAssign(ast::StmtAugAssign { target, value, .. }) => {
                self.modified = self.is_assignment(target);
                self.visit_expr(value);
            }
            Stmt::Delete(ast::StmtDelete { targets, .. }) => {
                self.modified = targets.iter().any(|target| self.is_assignment(target));
            }
            _ => visitor::walk_stmt(self, stmt),
        }
    }

    fn visit_expr(&mut self, expr: &Expr) {
        if self.modified {
            return;
        }
        if let Expr::Subscript(ast::ExprSubscript {
            value,
            slice,
            range,
            ..
        }) = expr
        {
            if let Expr::Name(ast::ExprName { id, .. }) = &**value {
                if id == self.sequence_name {
                    if let Expr::Name(ast::ExprName { id, .. }) = &**slice {
                        if id == self.index_name {
                            self.accesses.push(*range);
                        }
                    }
                }
            }
        }

        visitor::walk_expr(self, expr);
    }
}

pub(crate) fn is_dunder_operator_method(method: &str) -> bool {
    matches!(
        method,
        "__lt__"
            | "__le__"
            | "__eq__"
            | "__ne__"
            | "__gt__"
            | "__ge__"
            | "__add__"
            | "__sub__"
            | "__mul__"
            | "__matmul__"
            | "__truediv__"
            | "__floordiv__"
            | "__mod__"
            | "__divmod__"
            | "__pow__"
            | "__lshift__"
            | "__rshift__"
            | "__and__"
            | "__xor__"
            | "__or__"
            | "__radd__"
            | "__rsub__"
            | "__rmul__"
            | "__rmatmul__"
            | "__rtruediv__"
            | "__rfloordiv__"
            | "__rmod__"
            | "__rdivmod__"
            | "__rpow__"
            | "__rlshift__"
            | "__rrshift__"
            | "__rand__"
            | "__rxor__"
            | "__ror__"
            | "__iadd__"
            | "__isub__"
            | "__imul__"
            | "__imatmul__"
            | "__itruediv__"
            | "__ifloordiv__"
            | "__imod__"
            | "__ipow__"
            | "__ilshift__"
            | "__irshift__"
            | "__iand__"
            | "__ixor__"
            | "__ior__"
    )
}

/// Returns `true` if a method is a known dunder method.
pub(super) fn is_known_dunder_method(method: &str) -> bool {
    is_dunder_operator_method(method)
        || matches!(
            method,
            "__abs__"
            | "__aenter__"
            | "__aexit__"
            | "__aiter__"
            | "__anext__"
            | "__attrs_init__"
            | "__attrs_post_init__"
            | "__attrs_pre_init__"
            | "__await__"
            | "__bool__"
            | "__buffer__"
            | "__bytes__"
            | "__call__"
            | "__ceil__"
            | "__class__"
            | "__class_getitem__"
            | "__complex__"
            | "__contains__"
            | "__copy__"
            | "__deepcopy__"
            | "__del__"
            | "__delattr__"
            | "__delete__"
            | "__delitem__"
            | "__dict__"
            | "__dir__"
            | "__doc__"
            | "__enter__"
            | "__exit__"
            | "__float__"
            | "__floor__"
            | "__format__"
            | "__fspath__"
            | "__get__"
            | "__getattr__"
            | "__getattribute__"
            | "__getitem__"
            | "__getnewargs__"
            | "__getnewargs_ex__"
            | "__getstate__"
            | "__hash__"
            | "__html__"
            | "__index__"
            | "__init__"
            | "__init_subclass__"
            | "__instancecheck__"
            | "__int__"
            | "__invert__"
            | "__iter__"
            | "__len__"
            | "__length_hint__"
            | "__missing__"
            | "__module__"
            | "__mro_entries__"
            | "__neg__"
            | "__new__"
            | "__next__"
            | "__pos__"
            | "__post_init__"
            | "__prepare__"
            | "__reduce__"
            | "__reduce_ex__"
            | "__release_buffer__"
            | "__replace__"
            | "__repr__"
            | "__reversed__"
            | "__round__"
            | "__set__"
            | "__set_name__"
            | "__setattr__"
            | "__setitem__"
            | "__setstate__"
            | "__sizeof__"
            | "__str__"
            | "__subclasscheck__"
            | "__subclasses__"
            | "__subclasshook__"
            | "__trunc__"
            | "__weakref__"
            // Overridable sunder names from the `Enum` class.
            // See: https://docs.python.org/3/library/enum.html#supported-sunder-names
            | "_add_alias_"
            | "_add_value_alias_"
            | "_name_"
            | "_value_"
            | "_missing_"
            | "_ignore_"
            | "_order_"
            | "_generate_next_value_"
        )
}
