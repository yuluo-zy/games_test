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
                if a_offset < b_offset + b.size && b_offset < a_offset + a.size {
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
mod tests {
    use super::*;
    use crate::rg::transient_memory::{MemoryRequirement, plan_memory_requirements};
    fn plan() -> PhysicalMemoryPlan {
        plan_memory_requirements(
            &(0..2)
                .map(|resource| MemoryRequirement {
                    sharing: crate::resource::ResourceSharing::Exclusive,
                    resource,
                    compatibility: 0,
                    size: 4096,
                    alignment: 256,
                    memory_type_bits: 3,
                    allocation_flags: 0,
                    requires_dedicated: false,
                    prefers_dedicated: false,
                    uses: vec![resource as usize],
                })
                .collect::<Vec<_>>(),
            &[(0, 1)],
        )
        .unwrap()
    }
    #[test]
    fn independently_rejects_bad_offsets_missing_order_types_and_duplicate_ownership() {
        let original = plan();
        assert!(check_memory_plan(&original, &[(0, 1)]).is_ok());
        assert!(check_memory_plan(&original, &[]).is_err());
        let mut broken = original.clone();
        broken.placements[0].offset = 1;
        assert!(check_memory_plan(&broken, &[(0, 1)]).is_err());
        let mut broken = original.clone();
        broken.requirements[0].requires_dedicated = true;
        assert!(check_memory_plan(&broken, &[(0, 1)]).is_err());
        let mut broken = original.clone();
        broken.backings[0].memory_type_bits = 4;
        assert!(check_memory_plan(&broken, &[(0, 1)]).is_err());
        let mut broken = original.clone();
        broken.backings.push(broken.backings[0].clone());
        assert!(check_memory_plan(&broken, &[(0, 1)]).is_err());
        let mut broken = original.clone();
        broken.requirements[0].compatibility = 1;
        assert!(check_memory_plan(&broken, &[(0, 1)]).is_err());
        let mut broken = original;
        broken.backings[0].memory_type = Some(32);
        assert!(check_memory_plan(&broken, &[(0, 1)]).is_err());
    }
}
