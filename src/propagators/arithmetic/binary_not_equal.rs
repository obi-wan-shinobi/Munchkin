use crate::basic_types::PropagationStatusCP;
use crate::conjunction;
use crate::engine::cp::domain_events::DomainEvents;
use crate::engine::cp::propagation::propagation_context::ReadDomains;
use crate::engine::cp::propagation::PropagationContextMut;
use crate::engine::cp::propagation::Propagator;
use crate::engine::cp::propagation::PropagatorInitialisationContext;
use crate::predicates::PropositionalConjunction;
use crate::variables::IntegerVariable;

/// Propagator for the constraint `a != b`.
#[derive(Debug)]
pub(crate) struct BinaryNeqPropagator<A, B> {
    a: A,
    b: B,
}

impl<A, B> BinaryNeqPropagator<A, B> {
    pub(crate) fn new(a: A, b: B) -> Self {
        Self { a, b }
    }
}

impl<A, B> Propagator for BinaryNeqPropagator<A, B>
where
    A: IntegerVariable,
    B: IntegerVariable,
{
    fn name(&self) -> &str {
        "BinaryNotEqual"
    }

    fn initialise_at_root(
        &mut self,
        context: &mut PropagatorInitialisationContext,
    ) -> Result<(), PropositionalConjunction> {
        context.register(self.a.clone(), DomainEvents::ASSIGN);
        context.register(self.b.clone(), DomainEvents::ASSIGN);

        Ok(())
    }

    fn propagate(&self, mut context: PropagationContextMut) -> PropagationStatusCP {
        if context.is_fixed(&self.a) {
            let value = context.lower_bound(&self.a);
            context.remove(&self.b, value, conjunction!())?;
        }
        if context.is_fixed(&self.b) {
            let value = context.lower_bound(&self.b);

            context.remove(&self.a, value, conjunction!())?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::engine::test_helper::TestSolver;

    use super::*;

    #[test]
    fn non_singleton_domains_are_unaffected() {
        let mut solver = TestSolver::default();

        let x = solver.new_sparse_variable(&[1, 2, 3, 4]);
        let y = solver.new_sparse_variable(&[1, 2, 3, 4]);

        let mut propagator = solver
            .new_propagator(BinaryNeqPropagator::new(x, y))
            .expect("Something");

        let result = solver.propagate(&mut propagator);
        assert!(result.is_ok());

        solver.assert_bounds(x, 1, 4);
        solver.assert_bounds(y, 1, 4);
    }

    #[test]
    fn singleton_domain_for_a_prunes_value_from_domain_of_b() {
        let mut solver = TestSolver::default();

        let x = solver.new_variable(10, 10);
        let y = solver.new_variable(10, 15);

        let mut propagator = solver
            .new_propagator(BinaryNeqPropagator::new(x, y))
            .expect("Something");

        let result = solver.propagate(&mut propagator);
        assert!(result.is_ok());

        solver.assert_bounds(x, 10, 10);
        solver.assert_bounds(y, 11, 15);
    }

    #[test]
    fn singleton_domain_for_b_prunes_value_from_domain_of_a() {
        let mut solver = TestSolver::default();

        let x = solver.new_variable(1, 10);
        let y = solver.new_variable(10, 10);

        let mut propagator = solver
            .new_propagator(BinaryNeqPropagator::new(x, y))
            .expect("Something");

        let result = solver.propagate(&mut propagator);
        assert!(result.is_ok());

        solver.assert_bounds(x, 1, 9);
        solver.assert_bounds(y, 10, 10);
    }

    #[test]
    fn two_singleton_domains_with_same_value_trigger_conflict() {
        let mut solver = TestSolver::default();

        let x = solver.new_variable(10, 10);
        let y = solver.new_variable(10, 10);

        let _ = solver
            .new_propagator(BinaryNeqPropagator::new(x, y))
            .expect_err("Expected conflict at the root level");
    }
}
