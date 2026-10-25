//! 原绑定名字键、存储反射schema及消费者使用的数据。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BindingNameHash(pub u64);
impl BindingNameHash {
    /// 只识别末尾 ASCII _dims，用原库 limit 参数再加一。
    /// 单独名字 _dims 的 limit=0 在原库表示全串，不能改成空串 Hash。
    pub fn new(name: &str) -> Self {
        let limit = name.strip_suffix("_dims").map(str::len);
        Self(
            const_fnv1a_hash::fnv1a_hash_64(name.as_bytes(), limit)
                .wrapping_add(u64::from(limit.is_some())),
        )
    }
    pub fn with_suffix_dims(self) -> Self {
        Self(self.0.wrapping_add(1))
    }
}

use super::reflection_ty::Type;
use speedy::{Readable, Writable};
/// 值0/1/3/4/5已和发行 SPIR-V 对照；值2由原 descriptor 消费者表确认。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Readable, Writable)]
pub enum BindingKind {
    StorageBuffer,
    UniformBuffer,
    CombinedImageSampler,
    StorageImage,
    SampledImage,
    Sampler,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Readable, Writable)]
pub enum BindingArray {
    Fixed(u32),
    Runtime,
}
#[derive(Clone, Debug, PartialEq, Eq, Readable, Writable)]
pub struct Binding {
    pub kind: BindingKind,
    pub array: BindingArray,
    pub name: String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Readable, Writable)]
pub struct BindingLocation {
    pub set: u32,
    pub binding: u32,
}
/// 原 uniform 块的字节尺寸、字段偏移与 UBO 位置；不是 Vulkan push constant。
#[derive(Clone, Debug, PartialEq, Eq, Readable, Writable)]
pub struct UniformBlock {
    pub byte_size: u32,
    pub name_offsets: Vec<(String, u32)>,
    pub name_hash_offsets: Vec<(u64, u32)>,
    pub binding: BindingLocation,
}
#[derive(Clone, Debug, PartialEq, Eq, Readable, Writable)]
pub struct VertexInput {
    pub location: u32,
    pub ty: Type,
}
/// 以有序键值对保留原 HashMap 的发行顺序，便于逐字节回写。
/// 查询从尾向前找，保留原 map 重复键最后写入覆盖的行为。
#[derive(Clone, Debug, PartialEq, Eq, Readable, Writable)]
pub struct ShaderReflection {
    pub descriptor_sets: Vec<Vec<(u32, Binding)>>,
    pub uniform_block: Option<UniformBlock>,
    pub binding_names: Vec<(String, BindingLocation)>,
    pub binding_name_hashes: Vec<(u64, BindingLocation)>,
    pub vertex_inputs: Vec<(String, VertexInput)>,
    pub local_size: Option<[u32; 3]>,
    /// 原pipeline消费者将该字段作为VkPushConstantRange.size，offset恒0。
    pub push_constant_size: u32,
}
impl ShaderReflection {
    pub fn binding_by_name(&self, name: &str) -> Option<BindingLocation> {
        self.binding_names
            .iter()
            .rev()
            .find(|(key, _)| key == name)
            .map(|(_, value)| *value)
    }
    pub fn binding_by_hash(&self, name: BindingNameHash) -> Option<BindingLocation> {
        self.binding_name_hashes
            .iter()
            .rev()
            .find(|(key, _)| *key == name.0)
            .map(|(_, value)| *value)
    }
}

impl ShaderReflection {
    /// 生产消费者使用此投影，保留原HashMap重复键最后覆盖；原序列顺序仅用于回写。
    pub fn effective_descriptor_sets(&self) -> Vec<std::collections::BTreeMap<u32, &Binding>> {
        self.descriptor_sets
            .iter()
            .map(|set| set.iter().map(|(key, value)| (*key, value)).collect())
            .collect()
    }
    pub fn effective_vertex_inputs(&self) -> std::collections::BTreeMap<&str, &VertexInput> {
        self.vertex_inputs
            .iter()
            .map(|(key, value)| (key.as_str(), value))
            .collect()
    }
}
