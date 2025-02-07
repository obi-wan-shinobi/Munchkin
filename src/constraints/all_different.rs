use super::Constraint;
use crate::propagators::all_different::AllDifferentPropagator;
use crate::todo_constraint;
use crate::variables::IntegerVariable;

/// Creates the [`Constraint`] that enforces that all the given `variables` are distinct using a
/// decomposition.
pub fn all_different_decomposition<Var: IntegerVariable + 'static>(
    _variables: impl IntoIterator<Item = Var>,
) -> impl Constraint {
    todo_constraint!("decompose all_different using binary not equals")
}

/// Creates the [`Constraint`] that enforces that all the given `variables` are distinct.
pub fn all_different<Var: IntegerVariable + 'static>(
    variables: impl Into<Box<[Var]>>,
) -> impl Constraint {
    AllDifferentPropagator::new(variables.into())
}
