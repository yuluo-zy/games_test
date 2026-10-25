//! pass_resources.rs — PassResources（`PassBuilder::render<UserData = PassResources>` 的用户数据）。
//!
//! 机器码锚点：`PassResources::new_from` 0x140909310（893 B）。
//!
//! 观测（O）：
//!  * 对 7 个 128 位 TypeId 依次调用 `Components::get_resource_id` → 查 world 资源表，取资源数据指针；
//!    查找顺序 A,B,C,D,E,F,G；其中 **B 缺失时不报错，记 0**（`je 0x1409093f2` 跳过），其余缺失 →
//!    `core::option::unwrap_failed`（0x140909644..0x140909680 六处）。
//!  * 结果按内存布局写入 56 B（`[rsi+0x00..0x30]`）：`+0x00=A, +0x08=C, +0x10=D, +0x18=E, +0x20=G, +0x28=F, +0x30=B`
//!    （编译器重排，与查找顺序不同）。
//!  * 下游调用点确认的角色（O）：`+0x00` = `AssetShaderLibrary`（`get_compute_by_name`/`add_compute` 的 self）、
//!    `+0x08` = `AssetTextureLibrary`（`get_rg_texture` 的 self）、`+0x10` = `AssetSsboLibrary`。
//!    2026-10-18 原 RNE 的完整 TypeId 对与具名 `App::insert_resource<T>` 交叉确认：
//!    E = `AssetVAOLibrary`（0x14012153a/544），G = `AssetPlacementDataLibrary`（0x14012383a/844），
//!    F = `DynamicConstantsResource`（0x14011f88a/894），可选 B = `ShaderWatch`（0x140115fe6/ff0）。
//!    原始字节证据在 TG-727475d65610eb845f/main-chain-frame-20261018/map_resource_ids.json。
//!
//! 偏离：原版按**真实 TypeId** 查 bevy `Components`；TypeId 的 128 位值是原 EXE 里的常量，Rust 重建里没有同一
//! 类型的同一 TypeId，故查找被抽成 [`ResourceLookup`] 接缝：测试/集成可提供按 TypeId 对查询的实现。
//! `impl ResourceLookup for World` 因 bevy 内部表不可按外来 TypeId 查询而为 typed gap（恒 `None`，
//! 因此对 `&World` 调用 `new_from` 会照原版 `unwrap_failed` 路径 panic）。

use bevy_ecs::world::World;

use crate::asset_libraries::shader_library::AssetShaderLibrary;
use crate::asset_libraries::ssbo_library::AssetSsboLibrary;
use crate::asset_libraries::texture_library::AssetTextureLibrary;

/// 128 位 TypeId（`get_resource_id` 的两个 u64 实参，O）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TypeId128 {
    pub lo: u64,
    pub hi: u64,
}

/// 7 个帧级资源的 TypeId（O，取自 `new_from` 的 `movabs` 立即数；字母 = 查找顺序）。
pub mod type_ids {
    use super::TypeId128;
    /// A → `+0x00`：`AssetShaderLibrary`（O 角色）。
    pub const SHADER_LIBRARY: TypeId128 = TypeId128 {
        lo: 0x8597ee0727b461b3,
        hi: 0x8b297ec5ae15f15c,
    };
    /// B → `+0x30`：可选 `render::shaderwatch::inner::ShaderWatch`。
    pub const OPTIONAL_B: TypeId128 = TypeId128 {
        lo: 0xf0b963d163fe760f,
        hi: 0x7cb1ed0b9f8556a8,
    };
    /// C → `+0x08`：`AssetTextureLibrary`（O 角色）。
    pub const TEXTURE_LIBRARY: TypeId128 = TypeId128 {
        lo: 0xa68561b396784eaa,
        hi: 0xf5fd74e6faf4ed14,
    };
    /// D → `+0x10`：`AssetSsboLibrary`（O 角色；与 `bind_ray_tracing` 里 `get_resource_id` 立即数相同）。
    pub const SSBO_LIBRARY: TypeId128 = TypeId128 {
        lo: 0x8226cc786e679355,
        hi: 0x65cfda6a55a841ac,
    };
    /// E → `+0x18`：`AssetVAOLibrary`。
    pub const RESOURCE_E: TypeId128 = TypeId128 {
        lo: 0x874b1541877dacfd,
        hi: 0x2394c835ed9c6f89,
    };
    /// F → `+0x28`：`render::DynamicConstantsResource`。
    pub const DYNAMIC_CONSTANTS: TypeId128 = TypeId128 {
        lo: 0x47c94cb78283a72e,
        hi: 0x8c173ded97323e9d,
    };
    /// G → `+0x20`：`AssetPlacementDataLibrary`。
    pub const RESOURCE_G: TypeId128 = TypeId128 {
        lo: 0xb65bf3141f93e938,
        hi: 0xffcf78a6ae974587,
    };
}

/// 7 个帧级核心资源指针（56 B，内存顺序 = 机器码布局）。
///
/// 指针指向 world 内的资源；其生命周期受 world 约束（原版同为裸指针，`PassBuilder::render` 回调期间有效）。
#[derive(Clone, Copy, Debug)]
pub struct PassResources {
    /// `+0x00`：`AssetShaderLibrary`。
    pub shader_library: *const AssetShaderLibrary,
    /// `+0x08`：`AssetTextureLibrary`。
    pub texture_library: *const AssetTextureLibrary,
    /// `+0x10`：`AssetSsboLibrary`。
    pub ssbo_library: *const AssetSsboLibrary,
    /// `+0x18`：`AssetVAOLibrary`（TypeId E）。
    pub resource_e: *const (),
    /// `+0x20`：`AssetPlacementDataLibrary`（TypeId G）。
    pub resource_g: *const (),
    /// `+0x28`：`Mutex<DynamicConstants>`（I）。
    pub dynamic_constants: *const (),
    /// `+0x30`：可选 `ShaderWatch`（TypeId B），缺失为空指针。
    pub optional_b: *const (),
}

impl Default for PassResources {
    fn default() -> Self {
        PassResources {
            shader_library: core::ptr::null(),
            texture_library: core::ptr::null(),
            ssbo_library: core::ptr::null(),
            resource_e: core::ptr::null(),
            resource_g: core::ptr::null(),
            dynamic_constants: core::ptr::null(),
            optional_b: core::ptr::null(),
        }
    }
}

/// `Components::get_resource_id` + 资源表读取的接缝：按 TypeId 取资源数据指针（缺失 `None`）。
pub trait ResourceLookup {
    fn get_resource_ptr(&self, id: TypeId128) -> Option<*const ()>;
}

impl ResourceLookup for World {
    /// typed gap：bevy 的 `Components` 只接受 Rust `TypeId`，无法用原 EXE 的 128 位常量查询。
    fn get_resource_ptr(&self, _id: TypeId128) -> Option<*const ()> {
        None
    }
}

impl PassResources {
    /// `new_from`（0x140909310）：按 A,B,C,D,E,F,G 顺序查找；B 缺失记空，其余缺失 `unwrap_failed`。
    pub fn new_from(lookup: &(impl ResourceLookup + ?Sized)) -> PassResources {
        let a = required(lookup, type_ids::SHADER_LIBRARY);
        let b = lookup
            .get_resource_ptr(type_ids::OPTIONAL_B)
            .unwrap_or(core::ptr::null());
        let c = required(lookup, type_ids::TEXTURE_LIBRARY);
        let d = required(lookup, type_ids::SSBO_LIBRARY);
        let e = required(lookup, type_ids::RESOURCE_E);
        let f = required(lookup, type_ids::DYNAMIC_CONSTANTS);
        let g = required(lookup, type_ids::RESOURCE_G);
        PassResources {
            shader_library: a as *const AssetShaderLibrary,
            texture_library: c as *const AssetTextureLibrary,
            ssbo_library: d as *const AssetSsboLibrary,
            resource_e: e,
            resource_g: g,
            dynamic_constants: f,
            optional_b: b,
        }
    }

    /// `+0x00` 资源（调用方保证 world 仍存活，同原版）。
    pub fn shader_library(&self) -> &AssetShaderLibrary {
        // SAFETY: 由 `new_from` 从存活的 world 取得；非空由其必选语义保证（`default()` 构造的实例不得解引用）。
        unsafe {
            self.shader_library
                .as_ref()
                .expect("PassResources.shader_library is null")
        }
    }

    /// `+0x08` 资源。
    pub fn texture_library(&self) -> &AssetTextureLibrary {
        // SAFETY: 同 `shader_library`。
        unsafe {
            self.texture_library
                .as_ref()
                .expect("PassResources.texture_library is null")
        }
    }

    /// `+0x10` 资源。
    pub fn ssbo_library(&self) -> &AssetSsboLibrary {
        // SAFETY: 同 `shader_library`。
        unsafe {
            self.ssbo_library
                .as_ref()
                .expect("PassResources.ssbo_library is null")
        }
    }
}

/// 必选资源缺失 → `core::option::unwrap_failed`（"called `Option::unwrap()` on a `None` value"）。
fn required(lookup: &(impl ResourceLookup + ?Sized), id: TypeId128) -> *const () {
    lookup.get_resource_ptr(id).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[derive(Default)]
    struct Fake {
        map: HashMap<TypeId128, usize>,
        asked: std::cell::RefCell<Vec<TypeId128>>,
    }
    impl ResourceLookup for Fake {
        fn get_resource_ptr(&self, id: TypeId128) -> Option<*const ()> {
            self.asked.borrow_mut().push(id);
            self.map.get(&id).map(|a| *a as *const ())
        }
    }

    fn full() -> Fake {
        let mut f = Fake::default();
        use type_ids::*;
        for (i, id) in [
            SHADER_LIBRARY,
            TEXTURE_LIBRARY,
            SSBO_LIBRARY,
            RESOURCE_E,
            RESOURCE_G,
            DYNAMIC_CONSTANTS,
            OPTIONAL_B,
        ]
        .into_iter()
        .enumerate()
        {
            f.map.insert(id, 0x1000 + i * 0x10);
        }
        f
    }

    #[test]
    fn layout_is_56_bytes_like_the_original() {
        assert_eq!(core::mem::size_of::<PassResources>(), 56);
        assert_eq!(core::mem::offset_of!(PassResources, shader_library), 0x00);
        assert_eq!(core::mem::offset_of!(PassResources, texture_library), 0x08);
        assert_eq!(core::mem::offset_of!(PassResources, ssbo_library), 0x10);
        assert_eq!(core::mem::offset_of!(PassResources, resource_e), 0x18);
        assert_eq!(core::mem::offset_of!(PassResources, resource_g), 0x20);
        assert_eq!(
            core::mem::offset_of!(PassResources, dynamic_constants),
            0x28
        );
        assert_eq!(core::mem::offset_of!(PassResources, optional_b), 0x30);
    }

    #[test]
    fn lookup_order_is_a_b_c_d_e_f_g() {
        let f = full();
        let r = PassResources::new_from(&f);
        use type_ids::*;
        assert_eq!(
            *f.asked.borrow(),
            [
                SHADER_LIBRARY,
                OPTIONAL_B,
                TEXTURE_LIBRARY,
                SSBO_LIBRARY,
                RESOURCE_E,
                DYNAMIC_CONSTANTS,
                RESOURCE_G
            ]
        );
        assert_eq!(r.shader_library as usize, 0x1000);
        assert_eq!(r.optional_b as usize, 0x1060);
        assert_eq!(r.dynamic_constants as usize, 0x1050);
        assert_eq!(r.resource_g as usize, 0x1040);
    }

    #[test]
    fn optional_b_missing_is_null_not_an_error() {
        let mut f = full();
        f.map.remove(&type_ids::OPTIONAL_B);
        assert!(PassResources::new_from(&f).optional_b.is_null());
    }

    #[test]
    #[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
    fn mandatory_missing_panics_with_unwrap_failed() {
        let mut f = full();
        f.map.remove(&type_ids::SSBO_LIBRARY);
        PassResources::new_from(&f);
    }

    #[test]
    fn typeid_constants_match_exe_immediates() {
        // `new_from` 里的 movabs（有符号立即数取补）。
        assert_eq!(
            type_ids::SHADER_LIBRARY.lo,
            (-0x7a6811f8d84b9e4d_i64) as u64
        );
        assert_eq!(
            type_ids::DYNAMIC_CONSTANTS.hi,
            (-0x73e8c21268cdc163_i64) as u64
        );
        assert_eq!(type_ids::RESOURCE_G.hi, (-0x3087595168ba79_i64) as u64);
    }
}
