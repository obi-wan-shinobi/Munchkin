#![allow(unused, reason = "this file is a skeleton for the assignment")]

use crate::basic_types::PropagationStatusCP;
use crate::engine::cp::domain_events::DomainEvents;
use crate::engine::cp::propagation::propagation_context::ReadDomains;
use crate::engine::cp::propagation::PropagationContextMut;
use crate::engine::cp::propagation::Propagator;
use crate::engine::cp::propagation::PropagatorInitialisationContext;
use crate::predicates::PropositionalConjunction;
use crate::variables::IntegerVariable;

/// Propagator for constraint `element([x_1, \ldots, x_n], i, e)`, where `x_j` are
///  variables, `i` is an integer variable, and `e` is a variable, which holds iff `x_i = e`
///
/// Note that this propagator is 0-indexed
pub(crate) struct ElementPropagator<IndexVar, ArrayVar, RhsVar> {
    index: IndexVar,
    array: Box<[ArrayVar]>,
    rhs: RhsVar,
    // TODO: you can add more fields here!
}

impl<IndexVar, ArrayVar, RhsVar> ElementPropagator<IndexVar, ArrayVar, RhsVar> {
    pub(crate) fn new(index: IndexVar, array: Box<[ArrayVar]>, rhs: RhsVar) -> Self {
        ElementPropagator { index, array, rhs }
    }
}

impl<IndexVar, ArrayVar, RhsVar> Propagator for ElementPropagator<IndexVar, ArrayVar, RhsVar>
where
    IndexVar: IntegerVariable + 'static,
    ArrayVar: IntegerVariable + 'static,
    RhsVar: IntegerVariable + 'static,
{
    fn name(&self) -> &str {
        "Element"
    }

    fn propagate(&self, context: PropagationContextMut) -> PropagationStatusCP {
        if context.is_fixed(&self.rhs) {
            let value = context.lower_bound(&self.rhs);

            // Iterate over the array and reduce domain of index var to keep
            // those variables which contain value
            for i in 0..self.array.len() {
                // Check if current variable domain allows value
                if self.array[i].contains(value) {}
            }
        }

        Ok(())
    }

    fn initialise_at_root(
        &mut self,
        context: &mut PropagatorInitialisationContext,
    ) -> Result<(), PropositionalConjunction> {
        context.register(self.rhs.clone(), DomainEvents::ASSIGN);
        context.register(self.index.clone(), DomainEvents::ASSIGN);

        Ok(())
    }
}
