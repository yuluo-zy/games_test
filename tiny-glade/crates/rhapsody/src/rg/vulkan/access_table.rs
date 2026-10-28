//! Access-kind lookup table used for dependency derivation.
use ash::vk;
use crate::native::access_table::AccessSpec;
use crate::rg::graph;

/// Fixed u8 access-code space. Cloning the policy never allocates.
#[derive(Clone)]
pub struct AccessTable(pub AccessEntries);

#[derive(Clone)]
pub struct AccessEntries([Option<AccessSpec>; 256]);

impl Default for AccessEntries {
    fn default() -> Self {
        Self([None; 256])
    }
}

impl AccessEntries {
    pub fn get(&self, code: &u8) -> Option<&AccessSpec> {
        self.0[*code as usize].as_ref()
    }
    pub fn get_mut(&mut self, code: &u8) -> Option<&mut AccessSpec> {
        self.0[*code as usize].as_mut()
    }
    pub fn contains_key(&self, code: &u8) -> bool {
        self.get(code).is_some()
    }
    pub fn insert(&mut self, code: u8, spec: AccessSpec) -> Option<AccessSpec> {
        self.0[code as usize].replace(spec)
    }
    pub fn remove(&mut self, code: &u8) -> Option<AccessSpec> {
        self.0[*code as usize].take()
    }
}

impl std::ops::Index<&u8> for AccessEntries {
    type Output = AccessSpec;
    fn index(&self, code: &u8) -> &AccessSpec {
        self.get(code).expect("validated access code")
    }
}

impl Default for AccessTable {
    fn default() -> Self {
        let mut entries = AccessEntries::default();
        entries.insert(
            0,
            AccessSpec {
                scope: crate::native::AccessScope {
                    stages: vk::PipelineStageFlags::TOP_OF_PIPE,
                    access: vk::AccessFlags::empty(),
                    write: false,
                },
                layout: vk::ImageLayout::UNDEFINED,
            },
        );
        Self(entries)
    }
}

impl AccessTable {
    /// Resolve broad or custom native stage policies against VkDevice's actual
    /// enabled features. Access masks and access codes stay intact.
    pub(super) fn normalize_enabled_stages(&mut self, enabled: crate::device::DeviceCapabilities) {
        let mut disabled = vk::PipelineStageFlags::empty();
        if !enabled.geometry_shader {
            disabled |= vk::PipelineStageFlags::GEOMETRY_SHADER;
        }
        if !enabled.tessellation_shader {
            disabled |= vk::PipelineStageFlags::TESSELLATION_CONTROL_SHADER
                | vk::PipelineStageFlags::TESSELLATION_EVALUATION_SHADER;
        }
        for entry in self.0.0.iter_mut().flatten() {
            let scope = &mut entry.scope;
            let original = scope.stages;
            scope.stages &= !disabled;
            // Empty stages must not carry memory access. A custom policy aimed
            // only at a disabled stage still has conservative ordering semantics.
            if scope.stages.is_empty() && (!original.is_empty() || !scope.access.is_empty()) {
                scope.stages = vk::PipelineStageFlags::ALL_COMMANDS;
            }
        }
    }
    /// The complete built-in 64-code policy (see `raw::access_table`).
    pub fn original() -> Self {
        let mut entries = AccessEntries::default();
        for (code, spec) in crate::native::access_table::ACCESS_TABLE.iter().enumerate() {
            entries.insert(code as u8, *spec);
        }
        Self(entries)
    }
}

impl graph::AccessDependency for AccessTable {
    fn access_mask(&self, code: u8) -> vk::AccessFlags {
        self.0[&code].scope.access
    }
    fn is_write(&self, code: u8) -> bool {
        self.0[&code].scope.write
    }
    fn native_semantics(&self, code: u8) -> (crate::native::AccessScope, vk::ImageLayout) {
        let spec = self.0[&code];
        let mut scope = spec.scope;
        // The shared shader heap lowers uniform blocks to storage-buffer reads.
        // Keep both read classes in the native scope so either pipeline ABI is
        // valid under this same policy. Descriptor usage remains independently
        // validated when the resource is bound.
        if scope.access.contains(vk::AccessFlags::UNIFORM_READ) {
            scope.access |= vk::AccessFlags::SHADER_READ;
        }
        (scope, spec.layout)
    }
}
