//! 原作 B 的组件证据清单，供恢复源码和核对接口使用。
//!
//! 记录 57 个类型的名称、register_required_components 地址及 clone_behavior。
//! 表中全部 required 集合为空；这不是运行时注册器，也不表示 57 种载荷均已实现。
//! 注册/克隆胶水由锁定版本的 Bevy 依赖提供，不计独立业务恢复。
//!
//! Default 是 Bevy 的默认克隆策略，不能理解为调用组件的 Default::default；
//! Custom 表示原入口选择 component_clone_via_clone::<T>。
//! 证据入口：country-core-deep/components-geometry/ALGORITHMS.md §1–§2。

/// `ComponentCloneBehavior`（bevy_ecs 0.16）里原版实际出现的两种。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CloneBehavior {
    /// 原入口返回 Bevy Default（本清单中为未实现 Clone 的类型）。
    Default,
    /// 原入口选择 Clone 专用处理；clone_behavior_va 记录该选择入口的地址。
    Custom { clone_behavior_va: u64 },
}

/// 一个组件的已确认事实。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ComponentEntry {
    pub name: &'static str,
    /// `register_required_components` 的地址。
    pub register_va: u64,
    pub clone: CloneBehavior,
}

/// 57 个组件（O）。按原文件分组，各组内按注册函数地址升序：`components/mod.rs` 44 个、`components/drawable.rs` 12 个、
/// `components/transient_mesh.rs` 1 个。
pub const COMPONENTS: [ComponentEntry; 57] = [
    ComponentEntry {
        name: "FollowMouse",
        register_va: 0x140844f20,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "TestCube",
        register_va: 0x140845610,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "TreeComponent",
        register_va: 0x140845d00,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "ConvertedPreplacedTree",
        register_va: 0x1408463f0,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "TreeAnimation",
        register_va: 0x140846ae0,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "TreeMoveBend",
        register_va: 0x1408471d0,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "TreeMarkedForDeletion",
        register_va: 0x1408478c0,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "StairsStep",
        register_va: 0x140847fb0,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "ArrowGizmo",
        register_va: 0x1408486a0,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "SnappingToEndPreview",
        register_va: 0x140848d90,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "SnappingToSegmentPreview",
        register_va: 0x140849480,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "DashedLineLength",
        register_va: 0x140849b70,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "DontReceiveContactShadows",
        register_va: 0x14084a260,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "SemiTransparent",
        register_va: 0x14084a950,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "FlatLighting",
        register_va: 0x14084b040,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "NormalMap",
        register_va: 0x14084b730,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "OverwriteRoughness",
        register_va: 0x14084be20,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "ClothlineCloth",
        register_va: 0x14084c510,
        clone: CloneBehavior::Custom {
            clone_behavior_va: 0x14084cc00,
        },
    },
    ComponentEntry {
        name: "GothicWindowBrickDeformation",
        register_va: 0x14084cc10,
        clone: CloneBehavior::Custom {
            clone_behavior_va: 0x14084d300,
        },
    },
    ComponentEntry {
        name: "BrickColor",
        register_va: 0x14084d310,
        clone: CloneBehavior::Custom {
            clone_behavior_va: 0x14084da00,
        },
    },
    ComponentEntry {
        name: "Ui2DFollowMouse",
        register_va: 0x14084da10,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "UiInactive",
        register_va: 0x14084e100,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "Ui2dExtent",
        register_va: 0x14084e7f0,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "Ui2dWsExtent",
        register_va: 0x14084eee0,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "Ui2dImage",
        register_va: 0x14084f5d0,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "UiInteractionState",
        register_va: 0x14084fcc0,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "UiMouseInteractable",
        register_va: 0x1408503b0,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "UiDrawOrder",
        register_va: 0x140850aa0,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "UiNav",
        register_va: 0x140851190,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "HistoryButton",
        register_va: 0x140851880,
        clone: CloneBehavior::Custom {
            clone_behavior_va: 0x140851f70,
        },
    },
    ComponentEntry {
        name: "MeshFollowTerrain",
        register_va: 0x140851f80,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "KeepOnLevelUnload",
        register_va: 0x140852670,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "BackgroundTerrain",
        register_va: 0x140852d60,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "WoodenGate",
        register_va: 0x140853450,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "BalconyDoor",
        register_va: 0x140853b40,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "Smoke",
        register_va: 0x140854230,
        clone: CloneBehavior::Custom {
            clone_behavior_va: 0x140854920,
        },
    },
    ComponentEntry {
        name: "RoofFlagCloth",
        register_va: 0x140854930,
        clone: CloneBehavior::Custom {
            clone_behavior_va: 0x140855020,
        },
    },
    ComponentEntry {
        name: "WallFlagCloth",
        register_va: 0x140855030,
        clone: CloneBehavior::Custom {
            clone_behavior_va: 0x140855720,
        },
    },
    ComponentEntry {
        name: "SplashParticle",
        register_va: 0x140855730,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "TreeAmbience",
        register_va: 0x140855e20,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "LanternLightParams",
        register_va: 0x140856510,
        clone: CloneBehavior::Custom {
            clone_behavior_va: 0x140856c00,
        },
    },
    ComponentEntry {
        name: "AutoClutterConvertableToDecoClutter",
        register_va: 0x140856c10,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "AutoClutter",
        register_va: 0x140857300,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "SittableForBirds",
        register_va: 0x1408579f0,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "DrawTopology",
        register_va: 0x14082c3d0,
        clone: CloneBehavior::Custom {
            clone_behavior_va: 0x14082cac0,
        },
    },
    ComponentEntry {
        name: "NaniInstanceDesc",
        register_va: 0x14082cad0,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "NaniActiveInstance",
        register_va: 0x14082d1c0,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "DesignatedRenderPass",
        register_va: 0x14082d8b0,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "UiPass",
        register_va: 0x14082dfa0,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "WireframePass",
        register_va: 0x14082e690,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "DontCastShadow",
        register_va: 0x14082ed80,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "OcclusionCulled",
        register_va: 0x14082f470,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "AlbedoMap",
        register_va: 0x14082fb60,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "AlphaMap",
        register_va: 0x140830250,
        clone: CloneBehavior::Default,
    },
    ComponentEntry {
        name: "NaniSolidTexture",
        register_va: 0x140830940,
        clone: CloneBehavior::Custom {
            clone_behavior_va: 0x140831030,
        },
    },
    ComponentEntry {
        name: "PrevTransform",
        register_va: 0x140831040,
        clone: CloneBehavior::Custom {
            clone_behavior_va: 0x140831730,
        },
    },
    ComponentEntry {
        name: "TransientMesh",
        register_va: 0x140aac640,
        clone: CloneBehavior::Default,
    },
];

/// 每个组件的 required 组件：全部为空（见模块说明）。
pub fn required_components(_component: &ComponentEntry) -> &'static [&'static str] {
    &[]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn there_are_57_components_with_unique_names_and_addresses() {
        let names: HashSet<_> = COMPONENTS.iter().map(|c| c.name).collect();
        let addrs: HashSet<_> = COMPONENTS.iter().map(|c| c.register_va).collect();
        assert_eq!(names.len(), 57);
        assert_eq!(addrs.len(), 57);
    }

    #[test]
    fn exactly_eleven_components_derive_clone() {
        let custom: Vec<_> = COMPONENTS
            .iter()
            .filter(|c| matches!(c.clone, CloneBehavior::Custom { .. }))
            .map(|c| c.name)
            .collect();
        assert_eq!(
            custom,
            [
                "ClothlineCloth",
                "GothicWindowBrickDeformation",
                "BrickColor",
                "HistoryButton",
                "Smoke",
                "RoofFlagCloth",
                "WallFlagCloth",
                "LanternLightParams",
                "DrawTopology",
                "NaniSolidTexture",
                "PrevTransform",
            ]
        );
    }

    #[test]
    fn clone_behavior_entries_sit_next_to_their_register_function() {
        for c in &COMPONENTS {
            if let CloneBehavior::Custom { clone_behavior_va } = c.clone {
                assert_ne!(clone_behavior_va, c.register_va, "{}", c.name);
                assert!(
                    clone_behavior_va.abs_diff(c.register_va) < 0x2000,
                    "{}",
                    c.name
                );
            }
        }
    }

    #[test]
    fn no_component_requires_another() {
        assert!(COMPONENTS.iter().all(|c| required_components(c).is_empty()));
    }

    #[test]
    fn already_defined_marker_components_are_in_the_table() {
        for name in [
            "SemiTransparent",
            "FlatLighting",
            "BackgroundTerrain",
            "DontReceiveContactShadows",
            "OverwriteRoughness",
            "TransientMesh",
        ] {
            let entry = COMPONENTS.iter().find(|c| c.name == name).unwrap();
            assert_eq!(
                entry.clone,
                CloneBehavior::Default,
                "{name} has no Clone in B"
            );
        }
    }
}
