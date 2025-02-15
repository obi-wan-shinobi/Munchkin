#![cfg(any(test, doc))]
//! This module exposes helpers that aid testing of CP propagators. The [`TestSolver`] allows
//! setting up specific scenarios under which to test the various operations of a propagator.
use std::fmt::Debug;
use std::fmt::Formatter;

use super::cp::WatchListPropositional;
use crate::basic_types::Inconsistency;
use crate::basic_types::PropagationStatusCP;
use crate::basic_types::PropositionalConjunction;
use crate::engine::cp::propagation::PropagationContext;
use crate::engine::cp::propagation::PropagationContextMut;
use crate::engine::cp::propagation::Propagator;
use crate::engine::cp::propagation::PropagatorId;
use crate::engine::cp::propagation::PropagatorInitialisationContext;
use crate::engine::cp::reason::ReasonStore;
use crate::engine::cp::AssignmentsInteger;
use crate::engine::cp::EmptyDomain;
use crate::engine::cp::WatchListCP;
use crate::engine::predicates::integer_predicate::IntegerPredicate;
use crate::engine::sat::AssignmentsPropositional;
use crate::engine::variables::DomainId;
use crate::engine::variables::IntegerVariable;
use crate::engine::variables::Literal;
use crate::engine::variables::PropositionalVariable;

/// A container for CP variables, which can be used to test propagators.
#[derive(Default, Debug)]
pub(crate) struct TestSolver {
    assignments_integer: AssignmentsInteger,
    reason_store: ReasonStore,
    assignments_propositional: AssignmentsPropositional,
    watch_list: WatchListCP,
    watch_list_propositional: WatchListPropositional,
    next_id: u32,
}

type BoxedPropagator = Box<dyn Propagator>;

impl Debug for BoxedPropagator {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "test_helper::Propagator(<boxed value>)")
    }
}

#[allow(unused, reason = "can be used in an assignment")]
impl TestSolver {
    pub(crate) fn new_variable(&mut self, lb: i32, ub: i32) -> DomainId {
        self.watch_list.grow();
        self.assignments_integer.grow(lb, ub)
    }

    pub(crate) fn new_sparse_variable(&mut self, values: &[i32]) -> DomainId {
        let min_value = *values.iter().min().unwrap();
        let max_value = *values.iter().max().unwrap();

        self.watch_list.grow();
        let domain_id = self.assignments_integer.grow(min_value, max_value);

        for value in min_value..=max_value {
            if values.contains(&value) {
                continue;
            }
            self.assignments_integer
                .remove_value_from_domain(domain_id, value, None);
        }

        domain_id
    }

    pub(crate) fn new_literal(&mut self) -> Literal {
        let new_variable_index = self.assignments_propositional.num_propositional_variables();
        self.watch_list_propositional.grow();
        self.assignments_propositional.grow();

        Literal::new(PropositionalVariable::new(new_variable_index), true)
    }

    pub(crate) fn new_propagator(
        &mut self,
        propagator: impl Propagator + 'static,
    ) -> Result<BoxedPropagator, Inconsistency> {
        let id = PropagatorId(self.next_id);
        self.next_id += 1;

        let mut propagator: Box<dyn Propagator> = Box::new(propagator);

        propagator.initialise_at_root(&mut PropagatorInitialisationContext::new(
            &mut self.watch_list,
            &mut self.watch_list_propositional,
            id,
            &self.assignments_integer,
            &self.assignments_propositional,
        ))?;

        self.propagate(&mut propagator)?;

        Ok(propagator)
    }

    pub(crate) fn assert_domain<Var: IntegerVariable>(&self, var: Var, domain: Vec<i32>) {
        if domain.is_empty() {
            panic!("Domain provided to test solver is empty");
        }
        let min_domain = *domain.iter().min().unwrap();
        let max_domain = *domain.iter().max().unwrap();

        self.assert_bounds(var.clone(), min_domain, max_domain);
        for value in min_domain..=max_domain {
            if !domain.contains(&value) {
                assert!(
                    !self.contains(var.clone(), value),
                    "{value} was in the domain while it should not be (provided domain {domain:?})"
                )
            } else {
                assert!(
                    self.contains(var.clone(), value),
                    "{value} was not in the domain while it should be (provided domain {domain:?})"
                )
            }
        }
    }

    pub(crate) fn contains<Var: IntegerVariable>(&self, var: Var, value: i32) -> bool {
        var.contains(&self.assignments_integer, value)
    }

    pub(crate) fn lower_bound(&self, var: DomainId) -> i32 {
        self.assignments_integer.get_lower_bound(var)
    }

    pub(crate) fn increase_lower_bound(&mut self, var: DomainId, value: i32) {
        let result = self
            .assignments_integer
            .tighten_lower_bound(var, value, None);
        assert!(result.is_ok(), "The provided value to `increase_lower_bound` caused an empty domain, generally the propagator should not be notified of this change!");
    }

    pub(crate) fn set_literal(&mut self, var: Literal, val: bool) {
        self.assignments_propositional
            .enqueue_decision_literal(if val { var } else { !var });
    }

    pub(crate) fn is_literal_false(&self, var: Literal) -> bool {
        self.assignments_propositional
            .is_literal_assigned_false(var)
    }

    pub(crate) fn upper_bound(&self, var: DomainId) -> i32 {
        self.assignments_integer.get_upper_bound(var)
    }

    pub(crate) fn remove(&mut self, var: DomainId, value: i32) -> Result<(), EmptyDomain> {
        self.assignments_integer
            .remove_value_from_domain(var, value, None)
    }

    pub(crate) fn propagate(&mut self, propagator: &mut BoxedPropagator) -> PropagationStatusCP {
        let context = PropagationContextMut::new(
            &mut self.assignments_integer,
            &mut self.reason_store,
            &mut self.assignments_propositional,
            PropagatorId(0),
        );
        propagator.propagate(context)
    }

    pub(crate) fn propagate_until_fixed_point(
        &mut self,
        propagator: &mut BoxedPropagator,
    ) -> PropagationStatusCP {
        let mut num_trail_entries = self.assignments_integer.num_trail_entries()
            + self.assignments_propositional.num_trail_entries();
        loop {
            {
                // Specify the life-times to be able to retrieve the trail entries
                let context = PropagationContextMut::new(
                    &mut self.assignments_integer,
                    &mut self.reason_store,
                    &mut self.assignments_propositional,
                    PropagatorId(0),
                );
                propagator.propagate(context)?;
            }
            if self.assignments_integer.num_trail_entries()
                + self.assignments_propositional.num_trail_entries()
                == num_trail_entries
            {
                break;
            }
            num_trail_entries = self.assignments_integer.num_trail_entries()
                + self.assignments_propositional.num_trail_entries();
        }
        Ok(())
    }

    pub(crate) fn get_reason_int(
        &mut self,
        predicate: IntegerPredicate,
    ) -> &PropositionalConjunction {
        let reason_ref = self.assignments_integer.get_reason_for_predicate(predicate);
        let context =
            PropagationContext::new(&self.assignments_integer, &self.assignments_propositional);
        self.reason_store
            .get_or_compute(reason_ref, &context)
            .expect("reason_ref should not be stale")
    }

    pub(crate) fn get_reason_bool(
        &mut self,
        literal: Literal,
        assignment: bool,
    ) -> &PropositionalConjunction {
        let reason_ref = self
            .assignments_propositional
            .get_reason_for_assignment(literal, assignment);
        let context =
            PropagationContext::new(&self.assignments_integer, &self.assignments_propositional);
        self.reason_store
            .get_or_compute(reason_ref, &context)
            .expect("reason_ref should not be stale")
    }

    pub(crate) fn assert_bounds<Var: IntegerVariable>(&self, var: Var, lb: i32, ub: i32) {
        let actual_lb = var.lower_bound(&self.assignments_integer);
        let actual_ub = var.upper_bound(&self.assignments_integer);

        assert_eq!(
            (lb, ub), (actual_lb, actual_ub),
            "The expected bounds [{lb}..{ub}] did not match the actual bounds [{actual_lb}..{actual_ub}]"
        );
    }
}
