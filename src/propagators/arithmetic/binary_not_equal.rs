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
