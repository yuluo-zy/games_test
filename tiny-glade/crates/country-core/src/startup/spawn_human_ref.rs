//! 人类尺寸参考实体（B `spawn_human_ref` @0x14089d5a0）。
//!
//! 证据（R10 契约 §2.5，O）：
//! * 名式网格 `human_ref_cube`（0x142a9947e，14 字符）——由 mesh atlas/prefab
//!   通道按名查找，**不是** `assets/meshes/human_ref_cube.json`（该文件不存在）。
//! * shader 查表 `vertex_color_shader`（19 字符）。
//! * `geometry::cube::Box::new` 常量 `[-0.25, 0.25, -0.9, 0.9]`（0x142a98e30）与
//!   `[0.5, 1.8, 0.0, 0.0]`（0x142a98e50）；spawn bundle 0x40，`+0x30 = 0.5f`。
//!
//! U：两个 Box 常量的语义（半宽/高）、bundle 其余字段。按名查不到网格时只保留
//! 常量与告警，不注册伪造资产。
use super::StartupAssetHost;

pub const HUMAN_REF_MESH_NAME: &str = "human_ref_cube";
pub const HUMAN_REF_SHADER: (&str, &str) = ("/vertex_color.raster.hlsl", "vertex_color_shader");

/// O：`geometry::cube::Box::new` 常量（0x142a98e30）。
pub const HUMAN_REF_BOX_0: [f32; 4] = [-0.25, 0.25, -0.9, 0.9];
/// O：0x142a98e50。
pub const HUMAN_REF_BOX_1: [f32; 4] = [0.5, 1.8, 0.0, 0.0];
/// O：spawn bundle +0x30 = 0.5（0x3f000000）；缩放/半格语义 U。
pub const HUMAN_REF_SCALE: f32 = 0.5;

pub fn spawn_human_ref<H: StartupAssetHost>(host: &mut H) {
    let Some(mesh) = host.lookup_mesh(HUMAN_REF_MESH_NAME) else {
        log::warn!(
            target: "country_core::startup::spawn_human_ref",
            "U: mesh {HUMAN_REF_MESH_NAME} is not present in the mesh library; bundle constants retained"
        );
        return;
    };
    let shader = host.load_shader(HUMAN_REF_SHADER.0, HUMAN_REF_SHADER.1);
    host.spawn_drawable(mesh, shader);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct Rec {
        shader: Option<(&'static str, &'static str)>,
        spawned: usize,
        lookup_hit: bool,
    }
    impl StartupAssetHost for Rec {
        type Mesh = u32;
        type Shader = u32;
        fn load_mesh(&mut self, _: &'static str, _: Option<&'static str>) -> u32 {
            0
        }
        fn load_shader(&mut self, path: &'static str, name: &'static str) -> u32 {
            self.shader = Some((path, name));
            2
        }
        fn lookup_mesh(&mut self, name: &str) -> Option<u32> {
            self.lookup_hit = name == HUMAN_REF_MESH_NAME;
            self.lookup_hit.then_some(1)
        }
        fn spawn_drawable(&mut self, _: u32, _: u32) {
            self.spawned += 1;
        }
    }

    #[test]
    fn human_reference_looks_up_the_named_mesh_and_uses_vertex_color_shader() {
        let mut host = Rec::default();
        spawn_human_ref(&mut host);
        assert!(host.lookup_hit);
        assert_eq!(host.shader, Some(HUMAN_REF_SHADER));
        assert_eq!(host.spawned, 1);
        assert_eq!(HUMAN_REF_BOX_0, [-0.25, 0.25, -0.9, 0.9]);
        assert_eq!(HUMAN_REF_BOX_1, [0.5, 1.8, 0.0, 0.0]);
        assert_eq!(HUMAN_REF_SCALE.to_bits(), 0x3f00_0000);
    }

    #[test]
    fn missing_named_mesh_does_not_fabricate_an_asset() {
        struct Missing;
        impl StartupAssetHost for Missing {
            type Mesh = u32;
            type Shader = u32;
            fn load_mesh(&mut self, _: &'static str, _: Option<&'static str>) -> u32 {
                panic!("no file load may happen")
            }
            fn load_shader(&mut self, _: &'static str, _: &'static str) -> u32 {
                panic!("no shader load may happen")
            }
            fn spawn_drawable(&mut self, _: u32, _: u32) {
                panic!("no spawn may happen")
            }
        }
        spawn_human_ref(&mut Missing);
    }
}
