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
