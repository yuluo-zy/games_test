//! The shared bindless shader ABI a pipeline is compiled against: the descriptor layout
//! description every bindless pipeline agrees on. Plain data; the heap that binds it lives in
//! [`crate::bindless`], above the pipeline layer.
use shader_compiler::bindless::BindlessAbi;

pub struct BindlessShaderAbi {
    pub(crate) definition: BindlessAbi,
    pub(crate) parameter_count: usize,
    pub(crate) parameter_cpu_bytes: usize,
    pub(crate) _memory: crate::accounting::CpuMemoryReservation,
}
impl std::ops::Deref for BindlessShaderAbi {
    type Target = BindlessAbi;
    fn deref(&self) -> &Self::Target {
        &self.definition
    }
}
impl PartialEq for BindlessShaderAbi {
    fn eq(&self, other: &Self) -> bool {
        self.definition == other.definition
    }
}
impl Eq for BindlessShaderAbi {}
impl std::fmt::Debug for BindlessShaderAbi {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.definition.fmt(f)
    }
}
