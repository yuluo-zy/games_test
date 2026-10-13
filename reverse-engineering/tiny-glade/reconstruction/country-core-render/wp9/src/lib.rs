//! lib.rs — WP9 屋顶内容域证据束 crate 根（镜像 `reconstruction/country-core` 装配布局）。
//!
//! 本束只含 WP9 交付的屋顶资源/系统 + 模块胶水；完整可编译装配在
//! `reconstruction/country-core/src/lib.rs`（集成 owner 组装，wp1–wp6 + wp9）。
//! 骨架来源：`skeleton/B/crates/country-core/src/resources/roofs/`、
//! `skeleton/B/crates/country-core/src/systems/{render_loop,audio}/`、
//! `skeleton/B/crates/systems/roof/`（system_roof crate）。
//! 方法：业务逻辑（反编译 + 调用链 + PDB 骨架）→ 合同（contracts/*）→ 骨架实现；
//! 未知字段 → `typed_gap::Gap`。

extern crate alloc;

pub mod resources;
pub mod startup;
pub mod systems;

/// 复用装配类型占位（本束不交付 wp1/wp3 渲染域；集成时指向真实类型）。
pub mod consume {
    pub mod pass_builder {
        use crate::render::pass_builder::shader_uri::ShaderUri;
        use crate::typed_gap::Gap;

        pub struct ComputePass(pub Gap);
        pub struct ComputeShaderSource<'a> {
            pub uri: ShaderUri,
            pub defs: ShaderDefValue,
            pub _pd: core::marker::PhantomData<&'a ()>,
        }
        #[derive(Default)]
        pub struct ShaderDefValue;
        pub trait AsComputeShaderSource {
            fn source_hash(&self) -> u64;
            fn as_compute_pipeline(&self) -> ComputeShaderSource<'_>;
        }
        impl ComputePass {
            pub fn new<T: AsComputeShaderSource>(_src: T) -> Self {
                Self(Gap)
            }
            pub fn read_storage_buffer<T2>(self) -> Self {
                self
            }
            pub fn dispatch_with<F>(self, _args: DispatchArgs, _closure: F) -> Self {
                self
            }
        }
        pub struct DispatchArgs(pub Gap);
    }
    pub mod core {
        use crate::typed_gap::Gap;
        pub struct DofAutofocusSsbo(pub Gap);
    }
}

/// wp1/wp3 渲染域装配占位（本束不交付；集成时指向真实类型）。
pub mod render {
    pub mod sparse_instance_buffer {
        use crate::typed_gap::Gap;
        #[derive(Clone, Debug)]
        pub struct SparseInstanceBufferAllocation<T>(pub Gap, pub core::marker::PhantomData<T>);
    }
    pub mod pass_builder {
        pub mod shader_uri {
            pub struct ShaderUri(pub &'static str);
            impl ShaderUri {
                pub fn parse(s: &'static str) -> Self {
                    Self(s)
                }
            }
            pub struct ComputeShaderSourceHash;
            impl ComputeShaderSourceHash {
                pub fn compute(_s: &str) -> u64 {
                    0
                }
            }
        }
    }
}

/// Typed-gap 标记：未从原机器码恢复布局/语义的字段。不臆造。
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, Hash)]
pub struct Gap;

pub mod typed_gap {
    pub use crate::{gap_f32, gap_f32_pair, Gap};
}

impl Gap {
    /// 占位 f32（U 值）。
    pub fn f32() -> f32 {
        0.0
    }
    pub fn pair() -> [f32; 2] {
        [0.0; 2]
    }
}

/// 便捷占位（U）。
pub fn gap_f32() -> f32 {
    0.0
}
pub fn gap_f32_pair() -> [f32; 2] {
    [0.0; 2]
}
