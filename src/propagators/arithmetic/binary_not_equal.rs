use crate::basic_types::PropagationStatusCP;
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
        _context: &mut PropagatorInitialisationContext,
    ) -> Result<(), PropositionalConjunction> {
        todo!("listen to ASSIGN events on both the 'a' and 'b' variable")
    }

    fn propagate(&self, _context: PropagationContextMut) -> PropagationStatusCP {
        todo!("propagate the domains of 'a' and 'b'")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn non_singleton_domains_are_unaffected() {
        todo!("implement test")
    }

    #[test]
    fn singleton_domain_for_a_prunes_value_from_domain_of_b() {
        todo!("implement test")
    }

    #[test]
    fn singleton_domain_for_b_prunes_value_from_domain_of_a() {
        todo!("implement test")
    }

    #[test]
    fn two_singleton_domains_with_same_value_trigger_conflict() {
        todo!("implement test")
    }
}
