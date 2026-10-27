//! Independent validation of physical placements. This does not call the greedy
//! planner or trust its compatibility/grouping decisions.
use super::transient_memory::PhysicalMemoryPlan;
use std::collections::{BTreeMap, BTreeSet};
pub fn check_memory_plan(plan: &PhysicalMemoryPlan, edges: &[(usize, usize)]) -> crate::Result<()> {
    let invalid = || {
        crate::Error::invalid_usage("transient physical plan violates memory/lifetime constraints")
    };
    let mut facts = BTreeMap::new();
    for fact in &plan.requirements {
        if fact.size == 0
            || fact.alignment == 0
            || !fact.alignment.is_power_of_two()
            || fact.memory_type_bits == 0
            || fact.uses.is_empty()
            || facts.insert(fact.resource, fact).is_some()
        {
            return Err(invalid());
        }
    }
    let mut successors: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for &(from, to) in edges {
        if from >= to {
            return Err(invalid());
        }
        successors.entry(from).or_default().push(to);
    }
    let precedes = |source: usize, target: usize| {
        let mut visited = BTreeSet::new();
        let mut pending = successors.get(&source).cloned().unwrap_or_default();
        while let Some(pass) = pending.pop() {
            if pass == target {
                return true;
            }
            if visited.insert(pass) {
                pending.extend(successors.get(&pass).into_iter().flatten().copied());
            }
        }
        false
    };
    let mut placements = BTreeMap::new();
    for placement in &plan.placements {
        if placements.insert(placement.resource, placement).is_some() {
            return Err(invalid());
        }
    }
    if placements.len() != facts.len() {
        return Err(invalid());
    }
    let mut seen = BTreeSet::new();
    for (index, backing) in plan.backings.iter().enumerate() {
        if backing.size == 0
            || backing.alignment == 0
            || !backing.alignment.is_power_of_two()
            || backing.resources.is_empty()
            || backing.memory_type_bits == 0
        {
            return Err(invalid());
        }
        if let Some(memory_type) = backing.memory_type
            && (memory_type >= 32 || backing.memory_type_bits & (1 << memory_type) == 0)
        {
            return Err(invalid());
        }
        for id in &backing.resources {
            let fact = facts.get(id).ok_or_else(invalid)?;
            let placement = placements.get(id).ok_or_else(invalid)?;
            if !seen.insert(*id)
                || placement.backing != index
                || placement.offset % fact.alignment != 0
                || placement
                    .offset
                    .checked_add(fact.size)
                    .is_none_or(|end| end > backing.size)
                || backing.alignment % fact.alignment != 0
                || backing.memory_type_bits & !fact.memory_type_bits != 0
                || backing.allocation_flags != fact.allocation_flags
                || backing.sharing != fact.sharing
                || (fact.requires_dedicated && backing.resources.len() != 1)
            {
                return Err(invalid());
            }
        }
        for (position, a_id) in backing.resources.iter().enumerate() {
            for b_id in &backing.resources[position + 1..] {
                let a = facts[a_id];
                let b = facts[b_id];
                let a_offset = placements[a_id].offset;
                let b_offset = placements[b_id].offset;
                if a.compatibility != b.compatibility {
                    return Err(invalid());
                }
                if a_offset < b_offset.checked_add(b.size).ok_or_else(|| crate::Error::invalid_usage("transient placement range overflow"))?
                    && b_offset < a_offset.checked_add(a.size).ok_or_else(|| crate::Error::invalid_usage("transient placement range overflow"))? {
                    let a_before_b = a
                        .uses
                        .iter()
                        .all(|&source| b.uses.iter().all(|&target| precedes(source, target)));
                    let b_before_a = b
                        .uses
                        .iter()
                        .all(|&source| a.uses.iter().all(|&target| precedes(source, target)));
                    if !a_before_b && !b_before_a {
                        return Err(invalid());
                    }
                }
            }
        }
    }
    if seen.len() != facts.len() {
        return Err(invalid());
    }
    Ok(())
}
#[cfg(test)]
mod tests;
