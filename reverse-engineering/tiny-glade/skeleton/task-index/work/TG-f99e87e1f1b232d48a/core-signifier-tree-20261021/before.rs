//! `signifier_factory.rs` — 指示符记录工厂与实体构建器（O，2026-10-17 coregap-1 fresh 反编译）。
//!
//! 证据：
//! - `SignifierFactory::new` 0x14089d8b0：分配并整块拷贝载荷字节 → 40B 记录
//!   `{name_len@+0, name_ptr@+8, name_len@+0x10, &inner@+0x18, marker@+0x20}`（O 机器码
//!   `memcpy` + 5×qword 写）。原生 40 B 记录 = `{name: String, inner: Arc<dyn Fn(
//!   &AssetMeshLibrary, &AssetShaderLibrary, &mut Commands) -> Vec<Entity> + Send + Sync>}`：
//!   前 24 B 是 `String {len,ptr,len}`，后 16 B 是 Arc 胖指针 `{data,vtable}`。glade-border
//!   perimeter（0x141be0e73）与 `DebugLine::show`（0x140b04f70）都以此形状入
//!   `UiSignifierStream`；前者 env = `{1,1, label Vec, perimeter f32}`（0x30 B，
//!   vtable 0x142efa150），后者 env = `{points, colors, length, width, ui_pass, dashed}`
//!   （ArcInner 数据 0x40 B，vtable 0x142ae70d0）。
//! - `SignifierEntityBuilder`（0x90B）：`new` 0x14089d9b0（拷 48B 载荷 + 置默认）、
//!   `set_ui_pass` 0x14089dad0（`+0x8a = pass ^ 1`，整块拷 0x90）、`animation_duration`
//!   0x14089dc90（写 `+0x78`）、`custom_shader` 0x14089db00（替换 shader 名
//!   `{len@+0x60, ptr@+0x68, cap@+0x70}`）、`build` 0x14089dd50（spawn + 组件入队）。
//!
//! `build` 完整入队需要 `DrawableMeshBundle`/`UiPass`/`WireframePass`/`DontCastShadow`/
//! `SignifierMinimumLifespan`/`SignifierParentToVolumeOrigin`/`TransientMesh`（render 边界，
//! 未落库）——本实现 spawn 实体并插入已落库的 `SignifierAnimation`，其余登记 GAP（不冒充）。

use std::sync::Arc;

use bevy_ecs::entity::Entity;
use bevy_ecs::prelude::*;

use crate::asset_libraries::mesh_library::AssetMeshLibrary;
use crate::asset_libraries::shader_library::AssetShaderLibrary;

use super::animation::SignifierAnimation;
use super::signifier_create::SignifierId;

/// 原生唤醒签名（O：vtable 0x142ae70d0 / 0x142efa150 的 call 槽）。
///
/// 记录 `{name: String, inner: Arc<dyn Fn(..) -> Vec<Entity> + Send + Sync>}` 的
/// 第 4/5 qword 就是该胖指针的 `{data, vtable}`；消费端 `signifier_create`
/// （0x140b2cfa0）在去重后调用 vtable 槽唤醒它。对应 `Arc::drop_slow` 单态
/// 0x14064f9d0（`dyn Fn(tuple<ref AssetMeshLibrary, ref AssetShaderLibrary,
/// ref_mut Commands>, Output = Vec<Entity>) + Send + Sync`）。
pub type SignifierClosure = dyn Fn(
        &AssetMeshLibrary,
        &AssetShaderLibrary,
        &mut Commands<'_, '_>,
    ) -> Vec<Entity>
    + Send
    + Sync;

/// `SignifierFactory` — 40 B 指示符记录容器（O 0x14089d8b0）。
///
/// 布局（O，fresh）：`{name_len@+0, name_ptr@+8, name_len@+0x10, &inner@+0x18,
/// marker@+0x20}`。前 24 B 是 `String`（`{len, ptr, len}`；`new` 把 `&str` 拷贝成
/// 自有分配），后 16 B 是 `Arc<dyn Fn(..) -> Vec<Entity>>` 的 `{data, vtable}`。
///
/// crate 侧把闭包所有权保留在 [`Self::closure`]（不在 40 B 记录内），使
/// `write_factory`/`wake` 无需从裸 vtable 指针重建类型；裸 `inner`/`marker` 字段
/// 仍与原生 5-qword 视图逐位对应（`new` 的旧调用点使用）。
#[derive(Default)]
pub struct SignifierFactory {
    /// @+0：载荷/名字字节长度。
    pub name_len: usize,
    /// @+8：拷贝后的载荷指针。
    pub name_ptr: *mut u8,
    /// @+0x10：长度副本。
    pub name_len_dup: usize,
    /// @+0x18：内部记录/消费端指针。
    pub inner: *const u8,
    /// @+0x20：closure 标记/vtable。
    pub marker: u64,
    /// crate 侧闭包槽（非 40 B 记录）：`from_closure` 时持有真实唤醒闭包。
    pub closure: Option<Arc<SignifierClosure>>,
}

impl Clone for SignifierFactory {
    /// 深拷贝载荷（避免派生 `Clone` 对裸指针 + `Drop` 的双重释放），并共享闭包
    /// （原生记录经 `Arc` 克隆，引用计数 +1）。
    fn clone(&self) -> Self {
        let payload = self.payload();
        let mut cloned = Self::new(payload, self.inner, self.marker);
        cloned.closure = self.closure.clone();
        cloned
    }
}

impl std::fmt::Debug for SignifierFactory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SignifierFactory")
            .field("name_len", &self.name_len)
            .field("name_ptr", &self.name_ptr)
            .field("name_len_dup", &self.name_len_dup)
            .field("inner", &self.inner)
            .field("marker", &self.marker)
            .field("has_closure", &self.has_closure())
            .finish()
    }
}

// SAFETY: 原始类型把载荷指针置于共享记录；crate 内以 `Box<[u8]>` 持有实际字节，指针字段
// 仅用于布局对齐（本类型不直接 deref 载荷）。供 `SignifierEntityBuilder` 与
// `UiSignifierStream` 消费面使用。
unsafe impl Send for SignifierFactory {}
unsafe impl Sync for SignifierFactory {}

impl SignifierFactory {
    /// `SignifierFactory::new`（0x14089d8b0，O）：`memcpy` 载荷字节到新建分配，
    /// 写 40B `{len, ptr, len, &inner, marker}`。载荷以 `Box<[u8]>` 持有（分配由本类型
    /// 接管，与原版 `rust_alloc` + 拷贝等价）；`name_ptr` 指向该盒。
    pub fn new(payload: &[u8], inner: *const u8, marker: u64) -> Self {
        let boxed: Box<[u8]> = payload.into();
        let ptr = Box::into_raw(boxed) as *mut u8;
        let len = payload.len();
        Self {
            name_len: len,
            name_ptr: ptr,
            name_len_dup: len,
            inner,
            marker,
            closure: None,
        }
    }

    /// 从真实闭包构造原生记录（`{name String, inner Arc<dyn Fn..>}`，40 B）。
    ///
    /// 名字仍按 `new` 的语义拷贝为自有字节串；闭包所有权进入 crate 侧
    /// [`Self::closure`] 槽，供 [`Self::wake`] 唤醒。原生两处记录构造
    /// （`DebugLine::show` 0x140b04f70 与 glade-border 0x141be0e73）都经
    /// `SignifierFactory::new` 的等价内联序列写同样的 5 qword。
    pub fn from_closure(name: &str, closure: Arc<SignifierClosure>) -> Self {
        let mut factory = Self::new(name.as_bytes(), std::ptr::null(), 0);
        factory.closure = Some(closure);
        factory
    }

    /// 是否携带可唤醒闭包（原生闭包记录 vs crate 旧 raw 5-qword 视图）。
    pub fn has_closure(&self) -> bool {
        self.closure.is_some()
    }

    /// 名字载荷（`from_closure`/`new` 均存字节串；非 UTF-8 返回 None）。
    pub fn name(&self) -> Option<&str> {
        std::str::from_utf8(self.payload()).ok()
    }

    /// 唤醒闭包（O：`signifier_create` 0x140b2cfa0 的 vtable 调用槽；
    /// 参数 `(&AssetMeshLibrary, &AssetShaderLibrary, &mut Commands)`，返回
    /// `Vec<Entity>`）。无闭包的 raw 记录返回空列表——旧 crate 调用点不参与唤醒。
    pub fn wake(
        &self,
        mesh_library: &AssetMeshLibrary,
        shader_library: &AssetShaderLibrary,
        commands: &mut Commands<'_, '_>,
    ) -> Vec<Entity> {
        match &self.closure {
            Some(closure) => closure(mesh_library, shader_library, commands),
            None => Vec::new(),
        }
    }

    /// 载荷长度（O：@+0）。
    pub fn len(&self) -> usize {
        self.name_len
    }

    /// 载荷字节（I：`{len,ptr,len}` 记录的安全读面）。
    pub fn payload(&self) -> &[u8] {
        if self.name_ptr.is_null() {
            return &[];
        }
        // SAFETY: 指针来自 `new` 的 `Box<[u8]>`，长度记录在 `name_len`。
        unsafe { std::slice::from_raw_parts(self.name_ptr, self.name_len) }
    }

    /// 载荷是否为空（O：`len == 0`）。
    pub fn is_empty(&self) -> bool {
        self.name_len == 0
    }
}

impl Drop for SignifierFactory {
    /// 释放 `new` 里 `Box::into_raw` 的载荷盒（与原版 `rust_dealloc` 对应）。
    fn drop(&mut self) {
        if !self.name_ptr.is_null() {
            // SAFETY: 指针来自 `Box::into_raw`，类型与长度以 `name_len` 记录。
            unsafe {
                drop(Box::from_raw(std::slice::from_raw_parts_mut(
                    self.name_ptr,
                    self.name_len,
                )));
            }
            self.name_ptr = std::ptr::null_mut();
        }
    }
}

/// `SignifierEntityBuilder` — 指示符实体构建器（O：0x90 B，`new` 0x14089d9b0）。
///
/// 布局（O，fresh `new`/`set_ui_pass`/`animation_duration`/`custom_shader`/`build`）：
/// ```text
/// +0x00  u64 tag                      +0x40  u16 = 0x100
/// +0x08  u8  color_flag, f32 1.0@+0xc +0x48  u32 extra_a
/// +0x10  u64 mesh_kind (build switch) +0x4c  u32 extra_b
/// +0x18  [u64;5] mesh_payload (40B)   +0x50  u32 flag50（byte@+0x50）
///                                       +0x54  f32 f54
///                                       +0x58  u32 flag58（byte@+0x58 bit0）
///                                       +0x5c  f32 f5c
/// +0x60  u64 shader_len               +0x78  f32 duration
/// +0x68  u64 shader_ptr               +0x7c  u64 u7c
/// +0x70  u64 shader_cap               +0x84  u32 u84
///                                       +0x88  u8 flag88
///                                       +0x89  u8 flag89
///                                       +0x8a  u8 ui_pass（= pass ^ 1）
/// ```
/// 未取证的中间字段保持 O 位模式默认，不发明语义。
#[derive(Debug)]
pub struct SignifierEntityBuilder {
    /// @+0x00。
    pub tag: u64,
    /// @+0x08（O：默认 0；`build` 以之为 shader 是否有色的标志）。
    pub color_flag: u8,
    /// @+0x09..+0x0b。
    pub pad_09: [u8; 3],
    /// @+0x0c（O：位模式 `0x3f800000` = 1.0）。
    pub one: f32,
    /// @+0x10（O：`build` 的 switch 键 0/1/2/3/4——`new` 载荷首 qword）。
    pub mesh_kind: u64,
    /// @+0x18..+0x3f（O：`new` 从输入载荷 `[1..6]` 拷入 5 qword）。
    pub mesh_payload: [u64; 5],
    /// @+0x40（O：默认 0x100）。
    pub flags16: u16,
    /// @+0x42..+0x47。
    pub pad_42: [u8; 6],
    /// @+0x48（O：`build` 读面）。
    pub extra_a: u32,
    /// @+0x4c（O：`build` 读面）。
    pub extra_b: u32,
    /// @+0x50（O：整 u32；`build` 读 byte@+0x50）。
    pub flag50: u32,
    /// @+0x54（O：`flag50` 置位时 `build` 读面）。
    pub f54: f32,
    /// @+0x58（O：整 u32；`build` 读 byte@+0x58 bit0）。
    pub flag58: u32,
    /// @+0x5c（O：`flag58` 置位时 `build` 读面）。
    pub f5c: f32,
    /// @+0x60（shader 名 `{len,ptr,len}` 首部）。
    pub shader_len: u64,
    /// @+0x68。
    pub shader_ptr: *mut u8,
    /// @+0x70。
    pub shader_cap: u64,
    /// @+0x78（O：`animation_duration` 写面；`build` 落到 `SignifierAnimation.duration`）。
    pub duration: f32,
    /// @+0x7c（O：`build` 读面）。
    pub u7c: u64,
    /// @+0x84（O：`build` 读面）。
    pub u84: u32,
    /// @+0x88（O：`build` 读面——SignifierParentToVolumeOrigin 门控）。
    pub flag88: u8,
    /// @+0x89（O：`build` 读面）。
    pub flag89: u8,
    /// @+0x8a（O：`set_ui_pass` 写 `pass ^ 1`；`build` 决定 UiPass 槽）。
    pub ui_pass: u8,
}

// SAFETY: 布局与 `SignifierFactory` 相同理由——`shader_ptr` 指向 `Box<[u8]>` 持有的字节；
// 仅作布局对齐，deref 由 `shader_name()` 安全封装。
unsafe impl Send for SignifierEntityBuilder {}

impl Clone for SignifierEntityBuilder {
    /// 深拷贝（避免派生 `Clone` 对裸指针 + `Drop` 的双重释放）。
    fn clone(&self) -> Self {
        let shader = self.shader_name().map(str::to_owned);
        let mut b = Self {
            tag: self.tag,
            color_flag: self.color_flag,
            pad_09: self.pad_09,
            one: self.one,
            mesh_kind: self.mesh_kind,
            mesh_payload: self.mesh_payload,
            flags16: self.flags16,
            pad_42: self.pad_42,
            extra_a: self.extra_a,
            extra_b: self.extra_b,
            flag50: self.flag50,
            f54: self.f54,
            flag58: self.flag58,
            f5c: self.f5c,
            shader_len: 0,
            shader_ptr: std::ptr::null_mut(),
            shader_cap: 0,
            duration: self.duration,
            u7c: self.u7c,
            u84: self.u84,
            flag88: self.flag88,
            flag89: self.flag89,
            ui_pass: self.ui_pass,
        };
        if let Some(name) = shader {
            b.shader_name_replace(&name);
        }
        b
    }
}

impl Default for SignifierEntityBuilder {
    /// `new` 的默认面（O：0x14089d9b0 置默认值序列 + 装默认 shader
    /// `"vertex_color_shader"`（len 0x13 = 19））。
    fn default() -> Self {
        let mut b = Self {
            tag: 0,
            color_flag: 0,
            pad_09: [0; 3],
            one: 1.0,
            mesh_kind: 0,
            mesh_payload: [0; 5],
            flags16: 0x100,
            pad_42: [0; 6],
            extra_a: 0,
            extra_b: 0,
            flag50: 0,
            f54: 0.0,
            flag58: 0,
            f5c: 0.0,
            shader_len: 0,
            shader_ptr: std::ptr::null_mut(),
            shader_cap: 0,
            duration: 0.2,
            u7c: 0,
            u84: 0,
            flag88: 0,
            flag89: 0,
            ui_pass: 0,
        };
        b.shader_name_replace("vertex_color_shader");
        b
    }
}

impl SignifierEntityBuilder {
    /// `SignifierEntityBuilder::new`（0x14089d9b0，O）：拷贝 48B 网格载荷 + 置默认。
    /// `payload` 以 `[u64;6]` 承接（O：`new` 拷 6 qword）——**首 qword = `mesh_kind`
    /// （`build` switch 键）**，余 5 qword 入 `mesh_payload`（+0x18..+0x3f）。
    ///
    /// 注意：不能写成 `Self { .., ..Self::default() }`——本类型实现 `Drop`，
    /// 结构体更新语法会让默认临时值的 `Drop` 释放掉 `shader_name_replace` 分配的
    /// 默认 shader 串，留下悬垂 `shader_ptr`（后续 `custom_shader` 释放时 trap）。
    pub fn new(payload: &[u64; 6]) -> Self {
        let mut builder = Self::default();
        builder.mesh_kind = payload[0];
        builder.mesh_payload = [
            payload[1],
            payload[2],
            payload[3],
            payload[4],
            payload[5],
        ];
        builder
    }

    /// `set_ui_pass`（0x14089dad0，O）：写 `+0x8a = pass ^ 1` 后整块返回 0x90B 构建器。
    pub fn set_ui_pass(mut self, pass: u8) -> Self {
        self.ui_pass = pass ^ 1;
        self
    }

    /// `animation_duration`（0x14089dc90，O）：写 `+0x78`。
    pub fn animation_duration(mut self, duration: f32) -> Self {
        self.duration = duration;
        self
    }

    /// `custom_shader`（0x14089db00，O）：把 `{len@+0x60, ptr@+0x68, cap@+0x70}` 换成
    /// `name` 的新拷贝（旧串先 dealloc）。
    pub fn custom_shader(mut self, name: &str) -> Self {
        self.shader_name_replace(name);
        self
    }

    /// 替换 shader 名（O：0x14089db00 **先释放旧串**（`+0x60 != 0` → `rust_dealloc`），
    /// 再分配拷贝 + 写 `{len@+0x60, ptr@+0x68, cap@+0x70}`）。
    fn shader_name_replace(&mut self, name: &str) {
        if !self.shader_ptr.is_null() {
            // SAFETY: 指针来自此前 `shader_name_replace` 的 `Box<[u8]>`，长度记录在 `shader_len`。
            unsafe {
                drop(Box::from_raw(std::slice::from_raw_parts_mut(
                    self.shader_ptr,
                    self.shader_len as usize,
                )));
            }
        }
        let bytes: Box<[u8]> = name.as_bytes().into();
        let ptr = Box::into_raw(bytes) as *mut u8;
        self.shader_len = name.len() as u64;
        self.shader_ptr = ptr;
        self.shader_cap = name.len() as u64;
    }

    /// shader 名字面（I：`{len,ptr,len}` 记录的安全读面）。
    pub fn shader_name(&self) -> Option<&str> {
        if self.shader_ptr.is_null() {
            return None;
        }
        // SAFETY: 指针来自 `shader_name_replace` 的 `Box<[u8]>`，长度记录在 `shader_len`。
        unsafe {
            let bytes = std::slice::from_raw_parts(self.shader_ptr, self.shader_len as usize);
            Some(std::str::from_utf8(bytes).unwrap_or(""))
        }
    }

    /// `build`（0x14089dd50，O 消费面）：spawn 指示符实体 + 插入已落库组件。
    ///
    /// **已实现**：`Commands::spawn_empty` + 插入 `SignifierAnimation`（`duration` 取自
    /// `+0x78`）。
    /// **GAP（登记）**：原版随后把 `DrawableMeshBundle`/`SignifierAnimation`、
    /// `UiPass`/`WireframePass`（按 +0x89/+0x8a）、`DontCastShadow`、
    /// `SignifierMinimumLifespan`（按 +0x58 bit0）、`SignifierParentToVolumeOrigin`
    /// （按 +0x88）、`TransientMesh`（按 mesh_kind 载荷）压入 Commands——这些组件在
    /// country-core render 边界未落库，不冒充。shader 网格经 `AssetMeshLibrary`/
    /// `AssetShaderLibrary` 的解析面同样未接。
    pub fn build(self, commands: &mut Commands) -> Entity {
        let entity = commands.spawn_empty().id();
        commands.entity(entity).insert(SignifierAnimation {
            duration: self.duration,
            ..SignifierAnimation::default()
        });
        commands.entity(entity).insert(SignifierId(0));
        // GAP：DrawableMeshBundle / UiPass / WireframePass / DontCastShadow /
        // SignifierMinimumLifespan / SignifierParentToVolumeOrigin / TransientMesh 未落库。
        entity
    }
}

impl Drop for SignifierEntityBuilder {
    fn drop(&mut self) {
        if !self.shader_ptr.is_null() {
            // SAFETY: 指针来自 `shader_name_replace` 的 `Box::into_raw`。
            unsafe {
                drop(Box::from_raw(std::slice::from_raw_parts_mut(
                    self.shader_ptr,
                    self.shader_len as usize,
                )));
            }
            self.shader_ptr = std::ptr::null_mut();
        }
    }
}

