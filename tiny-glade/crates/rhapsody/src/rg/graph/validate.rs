//! Structural validation of a graph declaration.
use crate::rg::resource_info::NO_ACCESS;
use super::declaration::GraphDeclaration;
use super::declaration::INLINE_NAME_LIMIT;
use super::declaration::MAX_PASSES;
use super::declaration::OriginSource;
use super::error::BuildError;

impl GraphDeclaration {
    /// 在资源分配和索引打包之前检查调用方可直接构造的声明。
    pub fn validate(&self) -> Result<(), BuildError> {
        if self.resources.len() > usize::from(u16::MAX) {
            return Err(BuildError::ResourceLimit);
        }
        if self.passes.len() > MAX_PASSES {
            return Err(BuildError::PassLimit);
        }
        // Common frame graphs have only a few external resources. Keep their
        // identity check on the stack; larger declarations retain the ordered
        // set bound rather than turning validation into an unbounded quadratic scan.
        let mut inline_imports = [0u64; 32];
        let mut inline_count = 0usize;
        let mut imports = std::collections::BTreeSet::new();
        // Interned names compare by pointer, so a short list is checked with plain
        // scans and no per-frame allocation; only a large declaration builds a set.
        let mut names = (self.resources.len() > INLINE_NAME_LIMIT)
            .then(|| std::collections::HashSet::with_capacity(self.resources.len()));
        for (index, resource) in self.resources.iter().enumerate() {
            let duplicate = match &mut names {
                Some(names) => !names.insert(resource.name),
                None => self.resources[..index].iter().any(|earlier| earlier.name == resource.name),
            };
            if duplicate { return Err(BuildError::DuplicateResourceName(resource.name.to_string())); }
            if let OriginSource::ImportOwned { token, .. }
            | OriginSource::ImportUnowned { token, .. } = resource.source
            {
                if inline_count < inline_imports.len() {
                    if inline_imports[..inline_count].contains(&token) {
                        return Err(BuildError::AliasedImport(token));
                    }
                    inline_imports[inline_count] = token;
                    inline_count += 1;
                } else {
                    if imports.is_empty() {
                        imports.extend(inline_imports);
                    }
                    if !imports.insert(token) {
                        return Err(BuildError::AliasedImport(token));
                    }
                }
            }
        }
        let mut count = 0usize;
        for pass in &self.passes {
            count = count
                .checked_add(pass.accesses.len())
                .ok_or(BuildError::AccessLimit)?;
            if count >= NO_ACCESS as usize {
                return Err(BuildError::AccessLimit);
            }
            for access in &pass.accesses {
                if access.resource_index as usize >= self.resources.len() {
                    return Err(BuildError::InvalidHandle(access.resource_index));
                }
            }
        }
        for export in &self.exports {
            let resource = self
                .resources
                .get(export.resource_index as usize)
                .ok_or(BuildError::InvalidHandle(export.resource_index))?;
            if export.tag != resource.descriptor.kind_code() {
                return Err(BuildError::ResourceKindMismatch(export.resource_index));
            }
            export
                .range
                .validate(resource.descriptor)
                .map_err(|error| {
                    super::super::resource_contracts::range_error(export.resource_index, error)
                })?;
        }
        Ok(())
    }
}
