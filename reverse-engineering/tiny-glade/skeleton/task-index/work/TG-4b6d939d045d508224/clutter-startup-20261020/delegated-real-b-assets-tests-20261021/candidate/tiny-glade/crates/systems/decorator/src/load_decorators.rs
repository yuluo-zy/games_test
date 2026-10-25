//! 原 B 装饰物启动链：先订阅并注册 JSON 网格，再建立全部 subtype/rank 四表。
//!
//! 原完整入口1407b66e0、三个真实回调和bbox helper的身份与逐分支合同见
//! TG-4b6d939d045d508224。root/path/demo由现代启动宿主捕获，资源仍是原三权威owner。

use std::path::Path;
use std::sync::Arc;

use country_core::asset_libraries::mesh_library::{AssetMeshLibrary, MeshAsset};
use country_core::asset_libraries::meshwatch::MeshWatch;
use country_core::geometry::sphere::Sphere;
use country_core::render::mesh::{ATTR_NORMAL, ATTR_UV, Mesh};
use country_core::resources::walls::decorator::DecoratorType;
use country_core::resources::walls::decorator_subtype::{
    DecoratorCollision, DecoratorCollisionOutline, DecoratorLibKey, DecoratorMeshLibrary,
    DecoratorOutline,
};
use country_core::systems::collision::world::RaycastMeshData;
use country_core::systems::wall::shapes::rectangle::ui_edit_rectangle_orientation::box_trigger_volume;
use country_core::utils::components::transform::Transform;
use country_core::utils::load_json::load_json_as_mesh;
use glam::{Vec2, Vec3};
use utils::geometry::Rectangle3d;

/// 原目录142a84020的24字节；相对启动资源根解析。
pub const DECORATOR_MESH_DIR: &str = "assets/meshes/decorators";

/// 原PreStartup三资源系统的现代安全调用入口。
/// 宿主必须传实际资产根和现有启动demo开关；任何必需资产失败都中止启动。
pub fn load_decorators(
    root: &Path,
    library: &mut DecoratorMeshLibrary,
    assets: &AssetMeshLibrary,
    watch: &MeshWatch,
    demo_mode: bool,
) {
    register_json_meshes(root, assets, watch);
    for subtype in 0_u8..26 {
        // 原demo全局只屏蔽gothic与arrow-slit族，保留其它真实分支。
        if demo_mode && matches!(subtype, 6..=9 | 22 | 23) {
            continue;
        }
        let max_rank = match subtype {
            0..=5 | 10 => DecoratorType(0x38).max_rank(),
            6..=9 => DecoratorType(0x39).max_rank(),
            22 | 23 => DecoratorType(0x3a).max_rank(),
            _ => 1,
        };
        for rank in 1..=max_rank {
            let key = DecoratorLibKey::new(subtype, rank);
            match subtype {
                0 | 1 => {
                    let base = format!("decorators/window_cottage_{rank}x1");
                    let collision_name = if subtype == 1 {
                        format!("{base}_halfdormer_collision")
                    } else {
                        format!("{base}_collision")
                    };
                    let collision = load_collision_as_bbx(assets, &collision_name);
                    let interaction = load_collision_as_bbx(assets, &format!("{base}_interaction"));
                    let outline =
                        DecoratorOutline::Mesh(format!("decorators/outline_cottage_window_{rank}"));
                    let collision_outline = DecoratorCollisionOutline::Lines(load_lines(
                        root,
                        &format!("{base}_collision_outline"),
                    ));
                    library.insert(key, collision, interaction, outline, collision_outline);
                }
                2 => {
                    let base = format!("decorators/window_cottage_{rank}x1_full_dormer");
                    let collision = load_collision_as_bbx(assets, &base);
                    let outline = DecoratorOutline::Mesh(format!(
                        "decorators/outline_window_cottage_{rank}x1_full_dormer"
                    ));
                    let collision_outline = DecoratorCollisionOutline::Lines(load_lines(
                        root,
                        &format!("{base}_collision_outline"),
                    ));
                    library.insert_collision_and_interaction_as_same(
                        key,
                        collision,
                        outline,
                        collision_outline,
                    );
                }
                3 => {
                    let base = format!("decorators/window_cottage_corner_{rank}x1");
                    let outline_name = format!("decorators/outline_window_cottage_corner_{rank}x1");
                    let collision =
                        indexed_collision(required_mesh(assets, &format!("{base}_collision")));
                    let interaction = load_collision_as_bbx(assets, &format!("{base}_interaction"));
                    let outline = DecoratorOutline::Mesh(outline_name.clone());
                    let collision_outline =
                        DecoratorCollisionOutline::Lines(load_lines(root, &outline_name));
                    library.insert(key, collision, interaction, outline, collision_outline);
                }
                4 => {
                    let base = format!("decorators/balcony_door_rank{rank}");
                    let outline_name = format!("decorators/outline_balcony_door_rank{rank}");
                    let collision = load_collision_as_bbx(assets, &format!("{base}_collision"));
                    let interaction = load_collision_as_bbx(assets, &format!("{base}_interaction"));
                    // 原程序独立加载两份线点，不共用可变Vec或省略第二次文件失败。
                    let outline = DecoratorOutline::Lines(load_lines(root, &outline_name));
                    let collision_outline =
                        DecoratorCollisionOutline::Lines(load_lines(root, &outline_name));
                    library.insert(key, collision, interaction, outline, collision_outline);
                }
                5 | 10 => {
                    let collision =
                        load_collision_as_bbx(assets, "decorators/trap_door_cottage_collision");
                    let outline =
                        DecoratorOutline::Mesh("decorators/trap_door_cottage_outline".to_owned());
                    let collision_outline = DecoratorCollisionOutline::Lines(load_lines(
                        root,
                        "decorators/trap_door_cottage_collision_outline",
                    ));
                    library.insert_collision_and_interaction_as_same(
                        key,
                        collision,
                        outline,
                        collision_outline,
                    );
                }
                6..=9 => {
                    let base = match subtype {
                        6 | 7 => format!("decorators/window_gothic_{rank}x1"),
                        8 => format!("decorators/window_gothic_{rank}x1_dormer"),
                        9 => format!("decorators/balcony_door_gothic_rank{rank}"),
                        _ => unreachable!(),
                    };
                    let collision = convex_collision(assets, &format!("{base}_collision"));
                    let interaction = convex_collision(assets, &format!("{base}_interaction"));
                    let outline =
                        DecoratorOutline::Lines(load_lines(root, &format!("{base}_outline")));
                    let collision_outline = DecoratorCollisionOutline::Lines(load_lines(
                        root,
                        &format!("{base}_collision_outline"),
                    ));
                    library.insert(key, collision, interaction, outline, collision_outline);
                }
                11 => {
                    let collision = load_collision_as_bbx(assets, "decorators/flag_wall_collision");
                    let interaction =
                        load_collision_as_bbx(assets, "decorators/flag_wall_interaction");
                    let collision_outline = DecoratorCollisionOutline::Lines(load_lines(
                        root,
                        "decorators/flag_wall_collision_outline",
                    ));
                    library.insert(
                        key,
                        collision,
                        interaction,
                        DecoratorOutline::Sphere(0.3),
                        collision_outline,
                    );
                }
                12 => {
                    let collision = indexed_collision(required_mesh(
                        assets,
                        "decorators/flag_corner_collision",
                    ));
                    let interaction =
                        load_collision_as_bbx(assets, "decorators/flag_wall_interaction");
                    let collision_outline = DecoratorCollisionOutline::Lines(load_lines(
                        root,
                        "decorators/flag_corner_collision_outline",
                    ));
                    library.insert(
                        key,
                        collision,
                        interaction,
                        DecoratorOutline::Sphere(0.3),
                        collision_outline,
                    );
                }
                13 | 14 => {
                    let collision = load_collision_as_bbx(assets, "decorators/flag_roof_collision");
                    let interaction =
                        load_collision_as_bbx(assets, "decorators/flag_roof_interaction");
                    let collision_outline = DecoratorCollisionOutline::Lines(load_lines(
                        root,
                        "decorators/flag_roof_collision_outline",
                    ));
                    library.insert(
                        key,
                        collision,
                        interaction,
                        DecoratorOutline::Sphere(0.3),
                        collision_outline,
                    );
                }
                15 => {
                    let collision =
                        load_collision_as_bbx(assets, "decorators/flag_terrain_collision");
                    let interaction = indexed_collision(required_mesh(
                        assets,
                        "decorators/flag_terrain_interaction",
                    ));
                    let collision_outline = DecoratorCollisionOutline::Lines(load_lines(
                        root,
                        "decorators/flag_terrain_collision",
                    ));
                    library.insert(
                        key,
                        collision,
                        interaction,
                        DecoratorOutline::Sphere(0.3),
                        collision_outline,
                    );
                }
                16 | 17 => {
                    let base = if subtype == 16 {
                        "decorators/lantern_wall"
                    } else {
                        "decorators/lantern_terrain"
                    };
                    let collision =
                        indexed_collision(required_mesh(assets, &format!("{base}_collision")));
                    let interaction =
                        indexed_collision(required_mesh(assets, &format!("{base}_interaction")));
                    let outline = DecoratorOutline::Sphere(if subtype == 16 { 0.3 } else { 0.4 });
                    let collision_outline_name = if subtype == 16 {
                        "decorators/lantern_wall_collision_outline"
                    } else {
                        "decorators/lantern_terrain_collision"
                    };
                    let collision_outline =
                        DecoratorCollisionOutline::Lines(load_lines(root, collision_outline_name));
                    library.insert(key, collision, interaction, outline, collision_outline);
                }
                18 => {
                    let collision =
                        load_collision_as_bbx(assets, "decorators/lantern_flat_roof_collision");
                    let collision_outline = DecoratorCollisionOutline::Lines(load_lines(
                        root,
                        "decorators/lantern_flat_roof_collision_outline",
                    ));
                    library.insert_collision_and_interaction_as_same(
                        key,
                        collision,
                        DecoratorOutline::Sphere(0.4),
                        collision_outline,
                    );
                }
                19..=21 | 25 => {
                    let sphere = Mesh::try_from(Sphere::new(0, 0.5)).unwrap();
                    let collision = indexed_collision(&sphere);
                    drop(sphere);
                    let outline_name = match subtype {
                        19 => "decorators/chimney_collision_outline",
                        20 => "decorators/chimney_collision_outline_vent_pipe",
                        21 | 25 => "decorators/chimney_collision_outline_roof",
                        _ => unreachable!(),
                    };
                    let collision_outline =
                        DecoratorCollisionOutline::Lines(load_lines(root, outline_name));
                    library.insert_collision_and_interaction_as_same(
                        key,
                        collision,
                        DecoratorOutline::Sphere(0.5),
                        collision_outline,
                    );
                }
                22 | 23 => {
                    let base = format!("decorators/arrow_slit_1x{rank}");
                    let collision = if rank == 3 {
                        convex_collision(assets, &format!("{base}_collision"))
                    } else {
                        load_collision_as_bbx(assets, &format!("{base}_collision"))
                    };
                    let interaction = if rank == 3 {
                        convex_collision(assets, &format!("{base}_interaction"))
                    } else {
                        load_collision_as_bbx(assets, &format!("{base}_interaction"))
                    };
                    let outline_name = format!("{base}_outline");
                    let outline = DecoratorOutline::Lines(load_lines(root, &outline_name));
                    let collision_outline =
                        DecoratorCollisionOutline::Lines(load_lines(root, &outline_name));
                    library.insert(key, collision, interaction, outline, collision_outline);
                }
                24 => {
                    let collision = DecoratorCollision::empty();
                    let interaction = DecoratorCollision::empty();
                    library.insert(
                        key,
                        collision,
                        interaction,
                        DecoratorOutline::Sphere(0.2),
                        DecoratorCollisionOutline::None,
                    );
                }
                _ => unreachable!("原装饰subtype枚举止于26"),
            }
        }
    }
}

fn register_json_meshes(root: &Path, assets: &AssetMeshLibrary, watch: &MeshWatch) {
    let directory = root.join(DECORATOR_MESH_DIR);
    let entries = std::fs::read_dir(&directory).unwrap_or_else(|error| {
        panic!(
            "Reading assets/meshes/decorators: {}: {error}",
            directory.display()
        )
    });
    for entry in entries {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        let name = file_name.to_str().unwrap();
        let Some(stem) = name.strip_suffix(".json") else {
            continue;
        };
        let path = entry.path();
        let mut mesh = load_json_as_mesh(path.to_str().unwrap());
        // 原stem[..7]的UTF8边界失败也是可观测分支，不能换成无条件starts_with。
        if stem.len() >= 7 && &stem[..7] == "outline" {
            mesh.add_color(Vec3::ONE);
        }
        let name = format!("decorators/{stem}");
        // 原140b1aca0先注册监听，再把CPU资产发布到唯一网格库。
        watch.watch(&mesh);
        assets.add(MeshAsset::new(mesh, Some(name)));
    }
}

fn required_mesh<'a>(assets: &'a AssetMeshLibrary, name: &str) -> &'a Mesh {
    assets.get_by_name(name).unwrap()
}

fn mesh_bounds(mesh: &Mesh) -> Rectangle3d {
    let bounds = mesh.collision_bbx();
    Rectangle3d {
        footprint: bounds.rect,
        bottom: bounds.min_y,
        top: bounds.max_y,
    }
}

/// 原closure0；当前new_indexed切片API内部拥有复制，避免在producer再克隆一轮。
fn indexed_collision(mesh: &Mesh) -> DecoratorCollision {
    let indices = mesh.arrayed_indices();
    let positions = mesh.positions_without_indices();
    let normals = mesh
        .has_attribute(ATTR_NORMAL)
        .then(|| mesh.normals_without_indices());
    let uvs = mesh
        .has_attribute(ATTR_UV)
        .then(|| mesh.uvs_without_indices());
    let raycast = RaycastMeshData::new_indexed(&indices, positions, normals, uvs);
    let triangles = mesh.as_triangles();
    let bounds = mesh_bounds(mesh);
    DecoratorCollision {
        triangles,
        mesh: Arc::new(raycast),
        bounds,
    }
}

/// 原closure3；退化输入必须保留convex构造失败，不能换成bbox或空shape。
fn convex_collision(assets: &AssetMeshLibrary, name: &str) -> DecoratorCollision {
    let mesh = required_mesh(assets, name);
    let positions = mesh.positions_without_indices().to_vec();
    let raycast = RaycastMeshData::new_convex(&positions).expect("Convex mesh");
    drop(positions);
    let triangles = mesh.as_triangles();
    let bounds = mesh_bounds(mesh);
    DecoratorCollision {
        triangles,
        mesh: Arc::new(raycast),
        bounds,
    }
}

/// 原closure1直接重读JSON的未索引顶点，追加首点封口，不去重、不注册资产。
fn load_lines(root: &Path, name: &str) -> Vec<Vec3> {
    let path = root.join(format!("assets/meshes/{name}.json"));
    let mesh = load_json_as_mesh(path.to_str().unwrap());
    let mut points = mesh.positions_without_indices().to_vec();
    drop(mesh);
    let first = *points.first().unwrap();
    points.push(first);
    points
}

/// 原1407bb4b0：真实bbox三角形+完整尺寸cuboid+原资产矩形，失败不回退。
pub fn load_collision_as_bbx(assets: &AssetMeshLibrary, name: &str) -> DecoratorCollision {
    let bounds = mesh_bounds(required_mesh(assets, name));
    assert_eq!(bounds.footprint.axis, Vec2::X);
    let dimensions = Vec3::new(
        bounds.footprint.size.x,
        bounds.top - bounds.bottom,
        bounds.footprint.size.y,
    );
    let center = Vec3::new(
        bounds.footprint.center.x,
        (bounds.top + bounds.bottom) * 0.5,
        bounds.footprint.center.y,
    );
    let axis = bounds.footprint.axis;
    let transform = Transform {
        rotation: utils::xz_rotation_bad(Vec2::new(-axis.y, axis.x)),
        translation: center,
        scale: Vec3::ONE,
    };
    let triangles = box_trigger_volume(dimensions.x, dimensions.y, dimensions.z, &transform);
    let mesh = Arc::new(RaycastMeshData::new_cuboid_at(dimensions, center));
    // 原程序构造shape后再次required lookup读取bounds；保留这次失败/读取顺序。
    let bounds = mesh_bounds(required_mesh(assets, name));
    DecoratorCollision {
        triangles,
        mesh,
        bounds,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use country_core::render::mesh::{ATTR_COLOR, ATTR_POSITION};
    use std::sync::atomic::{AtomicU64, Ordering};

    // 此矩阵从原B跳表、各call参数与七张rank相对指针表冻结，不由被测分支生成。
    // 测试几何带每个原资产独有的x中心，能识别同类型但错误的资产绑定。
    #[derive(Clone, Copy)]
    enum Source {
        Bbox(&'static str),
        Indexed(&'static str),
        Convex(&'static str),
        Sphere,
        Empty,
        Same,
    }
    enum Outline {
        Mesh(&'static str),
        Lines(&'static str),
        Sphere(f32),
        None,
    }
    struct Expected {
        subtype: u8,
        rank: u8,
        collision: Source,
        interaction: Source,
        outline: Outline,
        collision_outline: Outline,
    }
    const EXPECTED: &[Expected] = &[
        Expected {
            subtype: 0,
            rank: 1,
            collision: Source::Bbox("decorators/window_cottage_1x1_collision"),
            interaction: Source::Bbox("decorators/window_cottage_1x1_interaction"),
            outline: Outline::Mesh("decorators/outline_cottage_window_1"),
            collision_outline: Outline::Lines("decorators/window_cottage_1x1_collision_outline"),
        },
        Expected {
            subtype: 0,
            rank: 2,
            collision: Source::Bbox("decorators/window_cottage_2x1_collision"),
            interaction: Source::Bbox("decorators/window_cottage_2x1_interaction"),
            outline: Outline::Mesh("decorators/outline_cottage_window_2"),
            collision_outline: Outline::Lines("decorators/window_cottage_2x1_collision_outline"),
        },
        Expected {
            subtype: 0,
            rank: 3,
            collision: Source::Bbox("decorators/window_cottage_3x1_collision"),
            interaction: Source::Bbox("decorators/window_cottage_3x1_interaction"),
            outline: Outline::Mesh("decorators/outline_cottage_window_3"),
            collision_outline: Outline::Lines("decorators/window_cottage_3x1_collision_outline"),
        },
        Expected {
            subtype: 1,
            rank: 1,
            collision: Source::Bbox("decorators/window_cottage_1x1_halfdormer_collision"),
            interaction: Source::Bbox("decorators/window_cottage_1x1_interaction"),
            outline: Outline::Mesh("decorators/outline_cottage_window_1"),
            collision_outline: Outline::Lines("decorators/window_cottage_1x1_collision_outline"),
        },
        Expected {
            subtype: 1,
            rank: 2,
            collision: Source::Bbox("decorators/window_cottage_2x1_halfdormer_collision"),
            interaction: Source::Bbox("decorators/window_cottage_2x1_interaction"),
            outline: Outline::Mesh("decorators/outline_cottage_window_2"),
            collision_outline: Outline::Lines("decorators/window_cottage_2x1_collision_outline"),
        },
        Expected {
            subtype: 1,
            rank: 3,
            collision: Source::Bbox("decorators/window_cottage_3x1_halfdormer_collision"),
            interaction: Source::Bbox("decorators/window_cottage_3x1_interaction"),
            outline: Outline::Mesh("decorators/outline_cottage_window_3"),
            collision_outline: Outline::Lines("decorators/window_cottage_3x1_collision_outline"),
        },
        Expected {
            subtype: 2,
            rank: 1,
            collision: Source::Bbox("decorators/window_cottage_1x1_full_dormer"),
            interaction: Source::Same,
            outline: Outline::Mesh("decorators/outline_window_cottage_1x1_full_dormer"),
            collision_outline: Outline::Lines(
                "decorators/window_cottage_1x1_full_dormer_collision_outline",
            ),
        },
        Expected {
            subtype: 2,
            rank: 2,
            collision: Source::Bbox("decorators/window_cottage_2x1_full_dormer"),
            interaction: Source::Same,
            outline: Outline::Mesh("decorators/outline_window_cottage_2x1_full_dormer"),
            collision_outline: Outline::Lines(
                "decorators/window_cottage_2x1_full_dormer_collision_outline",
            ),
        },
        Expected {
            subtype: 2,
            rank: 3,
            collision: Source::Bbox("decorators/window_cottage_3x1_full_dormer"),
            interaction: Source::Same,
            outline: Outline::Mesh("decorators/outline_window_cottage_3x1_full_dormer"),
            collision_outline: Outline::Lines(
                "decorators/window_cottage_3x1_full_dormer_collision_outline",
            ),
        },
        Expected {
            subtype: 3,
            rank: 1,
            collision: Source::Indexed("decorators/window_cottage_corner_1x1_collision"),
            interaction: Source::Bbox("decorators/window_cottage_corner_1x1_interaction"),
            outline: Outline::Mesh("decorators/outline_window_cottage_corner_1x1"),
            collision_outline: Outline::Lines("decorators/outline_window_cottage_corner_1x1"),
        },
        Expected {
            subtype: 3,
            rank: 2,
            collision: Source::Indexed("decorators/window_cottage_corner_2x1_collision"),
            interaction: Source::Bbox("decorators/window_cottage_corner_2x1_interaction"),
            outline: Outline::Mesh("decorators/outline_window_cottage_corner_2x1"),
            collision_outline: Outline::Lines("decorators/outline_window_cottage_corner_2x1"),
        },
        Expected {
            subtype: 3,
            rank: 3,
            collision: Source::Indexed("decorators/window_cottage_corner_3x1_collision"),
            interaction: Source::Bbox("decorators/window_cottage_corner_3x1_interaction"),
            outline: Outline::Mesh("decorators/outline_window_cottage_corner_3x1"),
            collision_outline: Outline::Lines("decorators/outline_window_cottage_corner_3x1"),
        },
        Expected {
            subtype: 4,
            rank: 1,
            collision: Source::Bbox("decorators/balcony_door_rank1_collision"),
            interaction: Source::Bbox("decorators/balcony_door_rank1_interaction"),
            outline: Outline::Lines("decorators/outline_balcony_door_rank1"),
            collision_outline: Outline::Lines("decorators/outline_balcony_door_rank1"),
        },
        Expected {
            subtype: 4,
            rank: 2,
            collision: Source::Bbox("decorators/balcony_door_rank2_collision"),
            interaction: Source::Bbox("decorators/balcony_door_rank2_interaction"),
            outline: Outline::Lines("decorators/outline_balcony_door_rank2"),
            collision_outline: Outline::Lines("decorators/outline_balcony_door_rank2"),
        },
        Expected {
            subtype: 4,
            rank: 3,
            collision: Source::Bbox("decorators/balcony_door_rank3_collision"),
            interaction: Source::Bbox("decorators/balcony_door_rank3_interaction"),
            outline: Outline::Lines("decorators/outline_balcony_door_rank3"),
            collision_outline: Outline::Lines("decorators/outline_balcony_door_rank3"),
        },
        Expected {
            subtype: 5,
            rank: 1,
            collision: Source::Bbox("decorators/trap_door_cottage_collision"),
            interaction: Source::Same,
            outline: Outline::Mesh("decorators/trap_door_cottage_outline"),
            collision_outline: Outline::Lines("decorators/trap_door_cottage_collision_outline"),
        },
        Expected {
            subtype: 5,
            rank: 2,
            collision: Source::Bbox("decorators/trap_door_cottage_collision"),
            interaction: Source::Same,
            outline: Outline::Mesh("decorators/trap_door_cottage_outline"),
            collision_outline: Outline::Lines("decorators/trap_door_cottage_collision_outline"),
        },
        Expected {
            subtype: 5,
            rank: 3,
            collision: Source::Bbox("decorators/trap_door_cottage_collision"),
            interaction: Source::Same,
            outline: Outline::Mesh("decorators/trap_door_cottage_outline"),
            collision_outline: Outline::Lines("decorators/trap_door_cottage_collision_outline"),
        },
        Expected {
            subtype: 6,
            rank: 1,
            collision: Source::Convex("decorators/window_gothic_1x1_collision"),
            interaction: Source::Convex("decorators/window_gothic_1x1_interaction"),
            outline: Outline::Lines("decorators/window_gothic_1x1_outline"),
            collision_outline: Outline::Lines("decorators/window_gothic_1x1_collision_outline"),
        },
        Expected {
            subtype: 6,
            rank: 2,
            collision: Source::Convex("decorators/window_gothic_2x1_collision"),
            interaction: Source::Convex("decorators/window_gothic_2x1_interaction"),
            outline: Outline::Lines("decorators/window_gothic_2x1_outline"),
            collision_outline: Outline::Lines("decorators/window_gothic_2x1_collision_outline"),
        },
        Expected {
            subtype: 6,
            rank: 3,
            collision: Source::Convex("decorators/window_gothic_3x1_collision"),
            interaction: Source::Convex("decorators/window_gothic_3x1_interaction"),
            outline: Outline::Lines("decorators/window_gothic_3x1_outline"),
            collision_outline: Outline::Lines("decorators/window_gothic_3x1_collision_outline"),
        },
        Expected {
            subtype: 7,
            rank: 1,
            collision: Source::Convex("decorators/window_gothic_1x1_collision"),
            interaction: Source::Convex("decorators/window_gothic_1x1_interaction"),
            outline: Outline::Lines("decorators/window_gothic_1x1_outline"),
            collision_outline: Outline::Lines("decorators/window_gothic_1x1_collision_outline"),
        },
        Expected {
            subtype: 7,
            rank: 2,
            collision: Source::Convex("decorators/window_gothic_2x1_collision"),
            interaction: Source::Convex("decorators/window_gothic_2x1_interaction"),
            outline: Outline::Lines("decorators/window_gothic_2x1_outline"),
            collision_outline: Outline::Lines("decorators/window_gothic_2x1_collision_outline"),
        },
        Expected {
            subtype: 7,
            rank: 3,
            collision: Source::Convex("decorators/window_gothic_3x1_collision"),
            interaction: Source::Convex("decorators/window_gothic_3x1_interaction"),
            outline: Outline::Lines("decorators/window_gothic_3x1_outline"),
            collision_outline: Outline::Lines("decorators/window_gothic_3x1_collision_outline"),
        },
        Expected {
            subtype: 8,
            rank: 1,
            collision: Source::Convex("decorators/window_gothic_1x1_dormer_collision"),
            interaction: Source::Convex("decorators/window_gothic_1x1_dormer_interaction"),
            outline: Outline::Lines("decorators/window_gothic_1x1_dormer_outline"),
            collision_outline: Outline::Lines(
                "decorators/window_gothic_1x1_dormer_collision_outline",
            ),
        },
        Expected {
            subtype: 8,
            rank: 2,
            collision: Source::Convex("decorators/window_gothic_2x1_dormer_collision"),
            interaction: Source::Convex("decorators/window_gothic_2x1_dormer_interaction"),
            outline: Outline::Lines("decorators/window_gothic_2x1_dormer_outline"),
            collision_outline: Outline::Lines(
                "decorators/window_gothic_2x1_dormer_collision_outline",
            ),
        },
        Expected {
            subtype: 8,
            rank: 3,
            collision: Source::Convex("decorators/window_gothic_3x1_dormer_collision"),
            interaction: Source::Convex("decorators/window_gothic_3x1_dormer_interaction"),
            outline: Outline::Lines("decorators/window_gothic_3x1_dormer_outline"),
            collision_outline: Outline::Lines(
                "decorators/window_gothic_3x1_dormer_collision_outline",
            ),
        },
        Expected {
            subtype: 9,
            rank: 1,
            collision: Source::Convex("decorators/balcony_door_gothic_rank1_collision"),
            interaction: Source::Convex("decorators/balcony_door_gothic_rank1_interaction"),
            outline: Outline::Lines("decorators/balcony_door_gothic_rank1_outline"),
            collision_outline: Outline::Lines(
                "decorators/balcony_door_gothic_rank1_collision_outline",
            ),
        },
        Expected {
            subtype: 9,
            rank: 2,
            collision: Source::Convex("decorators/balcony_door_gothic_rank2_collision"),
            interaction: Source::Convex("decorators/balcony_door_gothic_rank2_interaction"),
            outline: Outline::Lines("decorators/balcony_door_gothic_rank2_outline"),
            collision_outline: Outline::Lines(
                "decorators/balcony_door_gothic_rank2_collision_outline",
            ),
        },
        Expected {
            subtype: 9,
            rank: 3,
            collision: Source::Convex("decorators/balcony_door_gothic_rank3_collision"),
            interaction: Source::Convex("decorators/balcony_door_gothic_rank3_interaction"),
            outline: Outline::Lines("decorators/balcony_door_gothic_rank3_outline"),
            collision_outline: Outline::Lines(
                "decorators/balcony_door_gothic_rank3_collision_outline",
            ),
        },
        Expected {
            subtype: 10,
            rank: 1,
            collision: Source::Bbox("decorators/trap_door_cottage_collision"),
            interaction: Source::Same,
            outline: Outline::Mesh("decorators/trap_door_cottage_outline"),
            collision_outline: Outline::Lines("decorators/trap_door_cottage_collision_outline"),
        },
        Expected {
            subtype: 10,
            rank: 2,
            collision: Source::Bbox("decorators/trap_door_cottage_collision"),
            interaction: Source::Same,
            outline: Outline::Mesh("decorators/trap_door_cottage_outline"),
            collision_outline: Outline::Lines("decorators/trap_door_cottage_collision_outline"),
        },
        Expected {
            subtype: 10,
            rank: 3,
            collision: Source::Bbox("decorators/trap_door_cottage_collision"),
            interaction: Source::Same,
            outline: Outline::Mesh("decorators/trap_door_cottage_outline"),
            collision_outline: Outline::Lines("decorators/trap_door_cottage_collision_outline"),
        },
        Expected {
            subtype: 11,
            rank: 1,
            collision: Source::Bbox("decorators/flag_wall_collision"),
            interaction: Source::Bbox("decorators/flag_wall_interaction"),
            outline: Outline::Sphere(0.3),
            collision_outline: Outline::Lines("decorators/flag_wall_collision_outline"),
        },
        Expected {
            subtype: 12,
            rank: 1,
            collision: Source::Indexed("decorators/flag_corner_collision"),
            interaction: Source::Bbox("decorators/flag_wall_interaction"),
            outline: Outline::Sphere(0.3),
            collision_outline: Outline::Lines("decorators/flag_corner_collision_outline"),
        },
        Expected {
            subtype: 13,
            rank: 1,
            collision: Source::Bbox("decorators/flag_roof_collision"),
            interaction: Source::Bbox("decorators/flag_roof_interaction"),
            outline: Outline::Sphere(0.3),
            collision_outline: Outline::Lines("decorators/flag_roof_collision_outline"),
        },
        Expected {
            subtype: 14,
            rank: 1,
            collision: Source::Bbox("decorators/flag_roof_collision"),
            interaction: Source::Bbox("decorators/flag_roof_interaction"),
            outline: Outline::Sphere(0.3),
            collision_outline: Outline::Lines("decorators/flag_roof_collision_outline"),
        },
        Expected {
            subtype: 15,
            rank: 1,
            collision: Source::Bbox("decorators/flag_terrain_collision"),
            interaction: Source::Indexed("decorators/flag_terrain_interaction"),
            outline: Outline::Sphere(0.3),
            collision_outline: Outline::Lines("decorators/flag_terrain_collision"),
        },
        Expected {
            subtype: 16,
            rank: 1,
            collision: Source::Indexed("decorators/lantern_wall_collision"),
            interaction: Source::Indexed("decorators/lantern_wall_interaction"),
            outline: Outline::Sphere(0.3),
            collision_outline: Outline::Lines("decorators/lantern_wall_collision_outline"),
        },
        Expected {
            subtype: 17,
            rank: 1,
            collision: Source::Indexed("decorators/lantern_terrain_collision"),
            interaction: Source::Indexed("decorators/lantern_terrain_interaction"),
            outline: Outline::Sphere(0.4),
            collision_outline: Outline::Lines("decorators/lantern_terrain_collision"),
        },
        Expected {
            subtype: 18,
            rank: 1,
            collision: Source::Bbox("decorators/lantern_flat_roof_collision"),
            interaction: Source::Same,
            outline: Outline::Sphere(0.4),
            collision_outline: Outline::Lines("decorators/lantern_flat_roof_collision_outline"),
        },
        Expected {
            subtype: 19,
            rank: 1,
            collision: Source::Sphere,
            interaction: Source::Same,
            outline: Outline::Sphere(0.5),
            collision_outline: Outline::Lines("decorators/chimney_collision_outline"),
        },
        Expected {
            subtype: 20,
            rank: 1,
            collision: Source::Sphere,
            interaction: Source::Same,
            outline: Outline::Sphere(0.5),
            collision_outline: Outline::Lines("decorators/chimney_collision_outline_vent_pipe"),
        },
        Expected {
            subtype: 21,
            rank: 1,
            collision: Source::Sphere,
            interaction: Source::Same,
            outline: Outline::Sphere(0.5),
            collision_outline: Outline::Lines("decorators/chimney_collision_outline_roof"),
        },
        Expected {
            subtype: 22,
            rank: 1,
            collision: Source::Bbox("decorators/arrow_slit_1x1_collision"),
            interaction: Source::Bbox("decorators/arrow_slit_1x1_interaction"),
            outline: Outline::Lines("decorators/arrow_slit_1x1_outline"),
            collision_outline: Outline::Lines("decorators/arrow_slit_1x1_outline"),
        },
        Expected {
            subtype: 22,
            rank: 2,
            collision: Source::Bbox("decorators/arrow_slit_1x2_collision"),
            interaction: Source::Bbox("decorators/arrow_slit_1x2_interaction"),
            outline: Outline::Lines("decorators/arrow_slit_1x2_outline"),
            collision_outline: Outline::Lines("decorators/arrow_slit_1x2_outline"),
        },
        Expected {
            subtype: 22,
            rank: 3,
            collision: Source::Convex("decorators/arrow_slit_1x3_collision"),
            interaction: Source::Convex("decorators/arrow_slit_1x3_interaction"),
            outline: Outline::Lines("decorators/arrow_slit_1x3_outline"),
            collision_outline: Outline::Lines("decorators/arrow_slit_1x3_outline"),
        },
        Expected {
            subtype: 23,
            rank: 1,
            collision: Source::Bbox("decorators/arrow_slit_1x1_collision"),
            interaction: Source::Bbox("decorators/arrow_slit_1x1_interaction"),
            outline: Outline::Lines("decorators/arrow_slit_1x1_outline"),
            collision_outline: Outline::Lines("decorators/arrow_slit_1x1_outline"),
        },
        Expected {
            subtype: 23,
            rank: 2,
            collision: Source::Bbox("decorators/arrow_slit_1x2_collision"),
            interaction: Source::Bbox("decorators/arrow_slit_1x2_interaction"),
            outline: Outline::Lines("decorators/arrow_slit_1x2_outline"),
            collision_outline: Outline::Lines("decorators/arrow_slit_1x2_outline"),
        },
        Expected {
            subtype: 23,
            rank: 3,
            collision: Source::Convex("decorators/arrow_slit_1x3_collision"),
            interaction: Source::Convex("decorators/arrow_slit_1x3_interaction"),
            outline: Outline::Lines("decorators/arrow_slit_1x3_outline"),
            collision_outline: Outline::Lines("decorators/arrow_slit_1x3_outline"),
        },
        Expected {
            subtype: 24,
            rank: 1,
            collision: Source::Empty,
            interaction: Source::Empty,
            outline: Outline::Sphere(0.2),
            collision_outline: Outline::None,
        },
        Expected {
            subtype: 25,
            rank: 1,
            collision: Source::Sphere,
            interaction: Source::Same,
            outline: Outline::Sphere(0.5),
            collision_outline: Outline::Lines("decorators/chimney_collision_outline_roof"),
        },
    ];
    const ASSET_NAMES: &[&str] = &[
        "decorators/arrow_slit_1x1_collision",
        "decorators/arrow_slit_1x1_interaction",
        "decorators/arrow_slit_1x1_outline",
        "decorators/arrow_slit_1x2_collision",
        "decorators/arrow_slit_1x2_interaction",
        "decorators/arrow_slit_1x2_outline",
        "decorators/arrow_slit_1x3_collision",
        "decorators/arrow_slit_1x3_interaction",
        "decorators/arrow_slit_1x3_outline",
        "decorators/balcony_door_gothic_rank1_collision",
        "decorators/balcony_door_gothic_rank1_collision_outline",
        "decorators/balcony_door_gothic_rank1_interaction",
        "decorators/balcony_door_gothic_rank1_outline",
        "decorators/balcony_door_gothic_rank2_collision",
        "decorators/balcony_door_gothic_rank2_collision_outline",
        "decorators/balcony_door_gothic_rank2_interaction",
        "decorators/balcony_door_gothic_rank2_outline",
        "decorators/balcony_door_gothic_rank3_collision",
        "decorators/balcony_door_gothic_rank3_collision_outline",
        "decorators/balcony_door_gothic_rank3_interaction",
        "decorators/balcony_door_gothic_rank3_outline",
        "decorators/balcony_door_rank1_collision",
        "decorators/balcony_door_rank1_interaction",
        "decorators/balcony_door_rank2_collision",
        "decorators/balcony_door_rank2_interaction",
        "decorators/balcony_door_rank3_collision",
        "decorators/balcony_door_rank3_interaction",
        "decorators/chimney_collision_outline",
        "decorators/chimney_collision_outline_roof",
        "decorators/chimney_collision_outline_vent_pipe",
        "decorators/flag_corner_collision",
        "decorators/flag_corner_collision_outline",
        "decorators/flag_roof_collision",
        "decorators/flag_roof_collision_outline",
        "decorators/flag_roof_interaction",
        "decorators/flag_terrain_collision",
        "decorators/flag_terrain_interaction",
        "decorators/flag_wall_collision",
        "decorators/flag_wall_collision_outline",
        "decorators/flag_wall_interaction",
        "decorators/lantern_flat_roof_collision",
        "decorators/lantern_flat_roof_collision_outline",
        "decorators/lantern_terrain_collision",
        "decorators/lantern_terrain_interaction",
        "decorators/lantern_wall_collision",
        "decorators/lantern_wall_collision_outline",
        "decorators/lantern_wall_interaction",
        "decorators/outline_balcony_door_rank1",
        "decorators/outline_balcony_door_rank2",
        "decorators/outline_balcony_door_rank3",
        "decorators/outline_cottage_window_1",
        "decorators/outline_cottage_window_2",
        "decorators/outline_cottage_window_3",
        "decorators/outline_window_cottage_1x1_full_dormer",
        "decorators/outline_window_cottage_2x1_full_dormer",
        "decorators/outline_window_cottage_3x1_full_dormer",
        "decorators/outline_window_cottage_corner_1x1",
        "decorators/outline_window_cottage_corner_2x1",
        "decorators/outline_window_cottage_corner_3x1",
        "decorators/trap_door_cottage_collision",
        "decorators/trap_door_cottage_collision_outline",
        "decorators/trap_door_cottage_outline",
        "decorators/window_cottage_1x1_collision",
        "decorators/window_cottage_1x1_collision_outline",
        "decorators/window_cottage_1x1_full_dormer",
        "decorators/window_cottage_1x1_full_dormer_collision_outline",
        "decorators/window_cottage_1x1_halfdormer_collision",
        "decorators/window_cottage_1x1_interaction",
        "decorators/window_cottage_2x1_collision",
        "decorators/window_cottage_2x1_collision_outline",
        "decorators/window_cottage_2x1_full_dormer",
        "decorators/window_cottage_2x1_full_dormer_collision_outline",
        "decorators/window_cottage_2x1_halfdormer_collision",
        "decorators/window_cottage_2x1_interaction",
        "decorators/window_cottage_3x1_collision",
        "decorators/window_cottage_3x1_collision_outline",
        "decorators/window_cottage_3x1_full_dormer",
        "decorators/window_cottage_3x1_full_dormer_collision_outline",
        "decorators/window_cottage_3x1_halfdormer_collision",
        "decorators/window_cottage_3x1_interaction",
        "decorators/window_cottage_corner_1x1_collision",
        "decorators/window_cottage_corner_1x1_interaction",
        "decorators/window_cottage_corner_2x1_collision",
        "decorators/window_cottage_corner_2x1_interaction",
        "decorators/window_cottage_corner_3x1_collision",
        "decorators/window_cottage_corner_3x1_interaction",
        "decorators/window_gothic_1x1_collision",
        "decorators/window_gothic_1x1_collision_outline",
        "decorators/window_gothic_1x1_dormer_collision",
        "decorators/window_gothic_1x1_dormer_collision_outline",
        "decorators/window_gothic_1x1_dormer_interaction",
        "decorators/window_gothic_1x1_dormer_outline",
        "decorators/window_gothic_1x1_interaction",
        "decorators/window_gothic_1x1_outline",
        "decorators/window_gothic_2x1_collision",
        "decorators/window_gothic_2x1_collision_outline",
        "decorators/window_gothic_2x1_dormer_collision",
        "decorators/window_gothic_2x1_dormer_collision_outline",
        "decorators/window_gothic_2x1_dormer_interaction",
        "decorators/window_gothic_2x1_dormer_outline",
        "decorators/window_gothic_2x1_interaction",
        "decorators/window_gothic_2x1_outline",
        "decorators/window_gothic_3x1_collision",
        "decorators/window_gothic_3x1_collision_outline",
        "decorators/window_gothic_3x1_dormer_collision",
        "decorators/window_gothic_3x1_dormer_collision_outline",
        "decorators/window_gothic_3x1_dormer_interaction",
        "decorators/window_gothic_3x1_dormer_outline",
        "decorators/window_gothic_3x1_interaction",
        "decorators/window_gothic_3x1_outline",
    ];
    const BOX_JSON: &str = r#"{{"indices":{{"buffer":[0,1,2,2,3,0,4,5,6,6,7,4,8,9,10,10,11,8,12,13,14,14,15,12,16,17,18,18,19,16,20,21,22,22,23,20]}},"attributes":["Vertex_Position"],"Vertex_Position":{{"type":["float",3],"buffer":[{positions}]}}}}"#;
    static NEXT_FIXTURE: AtomicU64 = AtomicU64::new(0);

    struct Fixture(std::path::PathBuf);
    impl Fixture {
        fn new() -> Self {
            let root = std::env::temp_dir().join(format!(
                "tg-native-loader26-{}-{}",
                std::process::id(),
                NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed)
            ));
            std::fs::create_dir_all(root.join(DECORATOR_MESH_DIR)).unwrap();
            let fixture = Self(root);
            for &name in ASSET_NAMES {
                fixture.write_box(name);
            }
            fixture
        }
        fn path(&self, name: &str) -> std::path::PathBuf {
            self.0.join(format!("assets/meshes/{name}.json"))
        }
        fn center(name: &str) -> f32 {
            (ASSET_NAMES.iter().position(|&p| p == name).unwrap() as f32) * 16.0
        }
        fn write_box(&self, name: &str) {
            let x = Self::center(name);
            // 每面四顶点是完整有效CPU网格，不用mock shape或bbox占位。
            let corners = [
                [-1., -2., 3.],
                [1., -2., 3.],
                [1., 2., 3.],
                [-1., 2., 3.],
                [-1., 2., -3.],
                [1., 2., -3.],
                [1., -2., -3.],
                [-1., -2., -3.],
                [1., -2., -3.],
                [1., 2., -3.],
                [1., 2., 3.],
                [1., -2., 3.],
                [-1., -2., 3.],
                [-1., 2., 3.],
                [-1., 2., -3.],
                [-1., -2., -3.],
                [1., 2., -3.],
                [-1., 2., -3.],
                [-1., 2., 3.],
                [1., 2., 3.],
                [1., -2., 3.],
                [-1., -2., 3.],
                [-1., -2., -3.],
                [1., -2., -3.],
            ];
            let positions = corners
                .into_iter()
                .map(|[a, b, c]| format!("[{},{b},{c}]", a + x))
                .collect::<Vec<_>>()
                .join(",");
            let json = BOX_JSON
                .replace("{positions}", &positions)
                .replace("{{", "{")
                .replace("}}", "}");
            std::fs::write(self.path(name), json).unwrap();
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.0).unwrap();
        }
    }

    fn check_collision(record: &DecoratorCollision, source: Source) {
        let shape = record.mesh.shape_ref().expect("必须有真实物理shape");
        match source {
            Source::Bbox(name) | Source::Indexed(name) | Source::Convex(name) => {
                assert_eq!(record.bounds.footprint.axis, Vec2::X);
                assert_eq!(
                    record.bounds.footprint.center,
                    Vec2::new(Fixture::center(name), 0.)
                );
                assert_eq!(record.bounds.footprint.size, Vec2::new(2., 6.));
                assert_eq!((record.bounds.bottom, record.bounds.top), (-2., 2.));
                assert_eq!(record.triangles.len(), 12);
                match source {
                    Source::Bbox(_) => {
                        assert_eq!(
                            shape.as_cuboid().unwrap().half_extents.as_slice(),
                            &[1., 2., 3.]
                        );
                        assert_eq!(
                            record.mesh.local_transform().translation,
                            Vec3::new(Fixture::center(name), 0., 0.)
                        );
                    }
                    Source::Indexed(_) => {
                        assert!(shape.as_trimesh().is_some());
                        assert_eq!(record.mesh.vertices().len(), 24);
                        assert_eq!(record.mesh.indices().len(), 12);
                    }
                    Source::Convex(_) => assert!(shape.as_convex_polyhedron().is_some()),
                    _ => unreachable!(),
                }
            }
            Source::Sphere => {
                assert!(shape.as_trimesh().is_some());
                assert_eq!(record.triangles.len(), 20);
                assert_eq!(record.mesh.vertices().len(), 12);
            }
            Source::Empty => {
                assert!(record.triangles.is_empty());
                assert_eq!(
                    shape.as_cuboid().unwrap().half_extents.as_slice(),
                    &[0., 0., 0.]
                );
                assert_eq!(
                    record.bounds,
                    Rectangle3d::new(Vec3::ZERO, Vec3::ONE, Vec2::X)
                );
            }
            Source::Same => unreachable!(),
        }
    }
    fn check_lines(points: &[Vec3], name: &str) {
        assert_eq!(points.len(), 25);
        assert_eq!(points[0], Vec3::new(Fixture::center(name) - 1., -2., 3.));
        assert_eq!(points.last(), points.first());
    }

    #[test]
    fn native_loader_populates_every_real_key_shape_and_outline_in_full_and_demo_modes() {
        let fixture = Fixture::new();
        for demo in [false, true] {
            let assets = AssetMeshLibrary::new();
            let watch = MeshWatch::new(false);
            let mut library = DecoratorMeshLibrary::default();
            load_decorators(&fixture.0, &mut library, &assets, &watch, demo);
            let count = if demo { 34 } else { 52 };
            assert_eq!(library.collision.len(), count);
            assert_eq!(library.interaction.len(), count);
            assert_eq!(library.outline.len(), count);
            assert_eq!(library.collision_outline.len(), count);
            for expected in EXPECTED {
                let key = DecoratorLibKey::new(expected.subtype, expected.rank);
                if demo && matches!(expected.subtype, 6..=9 | 22 | 23) {
                    assert!(!library.collision.contains_key(&key));
                    continue;
                }
                let collision = library.collision_record(key);
                let interaction = library.interaction_record(key);
                check_collision(collision, expected.collision);
                if matches!(expected.interaction, Source::Same) {
                    check_collision(interaction, expected.collision);
                    assert!(Arc::ptr_eq(&collision.mesh, &interaction.mesh));
                    assert_eq!(collision.triangles, interaction.triangles);
                    assert_ne!(collision.triangles.as_ptr(), interaction.triangles.as_ptr());
                } else {
                    check_collision(interaction, expected.interaction);
                    assert!(!Arc::ptr_eq(&collision.mesh, &interaction.mesh));
                }
                match (&expected.outline, library.get_outline(key)) {
                    (Outline::Mesh(name), DecoratorOutline::Mesh(actual)) => {
                        assert_eq!(actual.as_str(), *name)
                    }
                    (Outline::Lines(name), DecoratorOutline::Lines(points)) => {
                        check_lines(points, name)
                    }
                    (Outline::Sphere(radius), DecoratorOutline::Sphere(actual)) => {
                        assert_eq!(actual, radius)
                    }
                    _ => panic!("主描边类型或载荷错误: {key:?}"),
                }
                match (
                    &expected.collision_outline,
                    library.get_collision_outline(key),
                ) {
                    (Outline::Lines(name), DecoratorCollisionOutline::Lines(points)) => {
                        check_lines(points, name)
                    }
                    (Outline::None, DecoratorCollisionOutline::None) => {}
                    _ => panic!("碰撞描边类型或载荷错误: {key:?}"),
                }
            }
            let outline = required_mesh(&assets, "decorators/outline_cottage_window_1");
            assert!(outline.has_attribute(ATTR_COLOR));
            assert!(
                !required_mesh(&assets, "decorators/window_cottage_1x1_collision")
                    .has_attribute(ATTR_COLOR)
            );
            assert!(
                outline
                    .source_file
                    .as_ref()
                    .unwrap()
                    .path
                    .ends_with("outline_cottage_window_1.json")
            );
        }
    }

    #[test]
    fn native_loader_fails_before_first_key_when_required_outline_is_missing() {
        let fixture = Fixture::new();
        std::fs::remove_file(fixture.path("decorators/window_cottage_1x1_collision_outline"))
            .unwrap();
        let assets = AssetMeshLibrary::new();
        let mut library = DecoratorMeshLibrary::default();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            load_decorators(
                &fixture.0,
                &mut library,
                &assets,
                &MeshWatch::new(false),
                false,
            )
        }));
        assert!(result.is_err());
        assert!(library.collision.is_empty());
        assert!(library.interaction.is_empty());
        // 文件注册阶段已完成，失败发生在真实四表发布前。
        assert!(
            assets
                .try_get_handle_by_name("decorators/window_cottage_1x1_collision")
                .is_some()
        );
    }

    #[test]
    fn native_loader_keeps_completed_keys_but_does_not_replace_failed_convex_with_a_box() {
        let fixture = Fixture::new();
        // 合法JSON/索引而非三维凸包：原convex回调必须失败。
        let json = r#"{"indices":{"buffer":[0,1,2]},"attributes":["Vertex_Position"],"Vertex_Position":{"type":["float",3],"buffer":[[0,0,0],[1,0,0],[0,1,0]]}}"#;
        std::fs::write(fixture.path("decorators/window_gothic_1x1_collision"), json).unwrap();
        let assets = AssetMeshLibrary::new();
        let mut library = DecoratorMeshLibrary::default();
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            load_decorators(
                &fixture.0,
                &mut library,
                &assets,
                &MeshWatch::new(false),
                false,
            )
        }));
        assert!(result.is_err());
        assert_eq!(library.collision.len(), 18);
        assert!(!library.collision.contains_key(&DecoratorLibKey::new(6, 1)));
        assert!(library.collision.contains_key(&DecoratorLibKey::new(5, 3)));
    }

    #[test]
    fn native_outline_loader_closes_unindexed_points_without_deduplicating() {
        let fixture = Fixture::new();
        let name = "decorators/chimney_collision_outline";
        let json = r#"{"indices":{"buffer":[0,0,0]},"attributes":["Vertex_Position"],"Vertex_Position":{"type":["float",3],"buffer":[[7,8,9],[1,2,3],[7,8,9]]}}"#;
        std::fs::write(fixture.path(name), json).unwrap();
        assert_eq!(
            load_lines(&fixture.0, name),
            vec![
                Vec3::new(7., 8., 9.),
                Vec3::new(1., 2., 3.),
                Vec3::new(7., 8., 9.),
                Vec3::new(7., 8., 9.)
            ]
        );
        let single = r#"{"indices":{"buffer":[0,0,0]},"attributes":["Vertex_Position"],"Vertex_Position":{"type":["float",3],"buffer":[[7,8,9]]}}"#;
        std::fs::write(fixture.path(name), single).unwrap();
        assert_eq!(load_lines(&fixture.0, name), vec![Vec3::new(7., 8., 9.); 2]);
    }

    #[test]
    fn native_registration_preserves_utf8_slice_boundary_failure_and_json_filter() {
        let fixture = Fixture::new();
        // 第7字节位于多字节字符内部；原切片失败不能用starts_with掩盖。
        let invalid_stem = "abcdefé";
        std::fs::copy(
            fixture.path("decorators/chimney_collision_outline"),
            fixture
                .0
                .join(DECORATOR_MESH_DIR)
                .join(format!("{invalid_stem}.json")),
        )
        .unwrap();
        let assets = AssetMeshLibrary::new();
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| register_json_meshes(
                &fixture.0,
                &assets,
                &MeshWatch::new(false)
            )))
            .is_err()
        );
        assert!(
            assets
                .try_get_handle_by_name(&format!("decorators/{invalid_stem}"))
                .is_none()
        );
        std::fs::remove_file(
            fixture
                .0
                .join(DECORATOR_MESH_DIR)
                .join(format!("{invalid_stem}.json")),
        )
        .unwrap();
        std::fs::write(
            fixture.0.join(DECORATOR_MESH_DIR).join("invalid.JSON"),
            "bad",
        )
        .unwrap();
        std::fs::write(
            fixture.0.join(DECORATOR_MESH_DIR).join("invalid.txt"),
            "bad",
        )
        .unwrap();
        let assets = AssetMeshLibrary::new();
        register_json_meshes(&fixture.0, &assets, &MeshWatch::new(false));
        assert!(
            assets
                .try_get_handle_by_name("decorators/invalid")
                .is_none()
        );
        assert!(
            required_mesh(&assets, "decorators/outline_cottage_window_1").has_attribute(ATTR_COLOR)
        );
    }

    #[test]
    fn indexed_collision_preserves_real_optional_attributes_and_is_not_a_bbox() {
        let mut mesh = Mesh::new("有法线UV的三角网格");
        mesh.set_indices(vec![0, 1, 2]);
        mesh.set_attribute(ATTR_POSITION, vec![Vec3::ZERO, Vec3::X, Vec3::Y]);
        mesh.set_attribute(ATTR_NORMAL, vec![Vec3::Z; 3]);
        mesh.set_attribute(ATTR_UV, vec![Vec2::ZERO, Vec2::X, Vec2::Y]);
        let record = indexed_collision(&mesh);
        assert!(record.mesh.shape_ref().unwrap().as_trimesh().is_some());
        assert_eq!(record.mesh.vertices(), mesh.positions_without_indices());
        assert_eq!(record.mesh.normals(), mesh.normals_without_indices());
        assert_eq!(record.mesh.uvs(), mesh.uvs_without_indices());
        assert_eq!(record.triangles, mesh.as_triangles());
    }

    fn original_b_installation() -> std::path::PathBuf {
        let root = std::path::PathBuf::from(
            std::env::var_os("TG_INSTALL_ROOT")
                .expect("必须设置 TG_INSTALL_ROOT 指向原版 B 安装目录"),
        )
        .canonicalize()
        .expect("解析真实原版安装目录");
        assert!(
            root.join(DECORATOR_MESH_DIR).is_dir(),
            "缺少真实装饰网格目录：{}",
            root.display()
        );
        assert!(
            root.join("default_settings.ron").is_file(),
            "必须使用完整原版安装根"
        );
        assert!(
            root.join("compiled-shaders").is_dir(),
            "必须使用完整原版安装根"
        );
        root
    }

    fn assert_original_asset_collision(
        root: &Path,
        assets: &AssetMeshLibrary,
        record: &DecoratorCollision,
        source: Source,
    ) {
        let name = match source {
            Source::Bbox(name) | Source::Indexed(name) | Source::Convex(name) => name,
            Source::Sphere | Source::Empty => {
                check_collision(record, source);
                return;
            }
            Source::Same => panic!("Same 必须由真实两表之间的 Arc 身份断言检查"),
        };
        let asset = required_mesh(assets, name);
        let source_path = root.join(format!("assets/meshes/{name}.json"));
        assert_eq!(
            std::path::Path::new(&asset.source_file.as_ref().expect("真实源文件").path)
                .canonicalize()
                .unwrap(),
            source_path.canonicalize().unwrap(),
            "原 subtype/rank 必须引用指定安装树里的真实资产：{name}"
        );
        let points = asset.positions_without_indices();
        assert!(!points.is_empty(), "原碰撞资产不能替换为空几何：{name}");
        let mut minimum = Vec3::splat(f32::INFINITY);
        let mut maximum = Vec3::splat(f32::NEG_INFINITY);
        for point in points {
            assert!(point.is_finite(), "原碰撞位置必须有限：{name}");
            minimum = minimum.min(*point);
            maximum = maximum.max(*point);
        }
        let dimensions = maximum - minimum;
        let center = (maximum + minimum) * 0.5;
        assert_eq!(record.bounds.footprint.axis, Vec2::X, "{name}");
        assert_eq!(
            record.bounds.footprint.center,
            Vec2::new(center.x, center.z),
            "{name}"
        );
        assert_eq!(
            record.bounds.footprint.size,
            Vec2::new(dimensions.x, dimensions.z),
            "{name}"
        );
        assert_eq!(
            (record.bounds.bottom, record.bounds.top),
            (minimum.y, maximum.y),
            "{name}"
        );
        let shape = record.mesh.shape_ref().expect("真实物理 shape 必须存在");
        match source {
            Source::Bbox(_) => {
                assert_eq!(record.triangles.len(), 12, "原 box 六面 topology：{name}");
                let shape = shape.as_cuboid().expect("原 bbox 分支必须是真 cuboid");
                assert_eq!(
                    shape.half_extents.as_slice(),
                    &(dimensions * 0.5).to_array(),
                    "{name}"
                );
                assert_eq!(record.mesh.local_transform().translation, center, "{name}");
            }
            Source::Indexed(_) => {
                assert!(
                    shape.as_trimesh().is_some(),
                    "不可把原 indexed 分支降级 bbox：{name}"
                );
                assert_eq!(record.mesh.vertices(), points, "原顶点序列：{name}");
                assert_eq!(
                    record.mesh.indices(),
                    asset.arrayed_indices(),
                    "原三角索引：{name}"
                );
                assert_eq!(
                    record.triangles,
                    asset.as_triangles(),
                    "真实 source triangles：{name}"
                );
                if asset.has_attribute(ATTR_NORMAL) {
                    assert_eq!(
                        record.mesh.normals(),
                        asset.normals_without_indices(),
                        "{name}"
                    );
                } else {
                    assert!(record.mesh.normals().is_empty(), "{name}");
                }
                if asset.has_attribute(ATTR_UV) {
                    assert_eq!(record.mesh.uvs(), asset.uvs_without_indices(), "{name}");
                } else {
                    assert!(record.mesh.uvs().is_empty(), "{name}");
                }
            }
            Source::Convex(_) => {
                assert!(
                    shape.as_convex_polyhedron().is_some(),
                    "不可把原 convex 分支降级 bbox：{name}"
                );
                assert_eq!(record.triangles, asset.as_triangles(), "{name}");
            }
            _ => unreachable!(),
        }
    }

    fn assert_original_asset_lines(root: &Path, name: &str, actual: &[Vec3]) {
        let path = root.join(format!("assets/meshes/{name}.json"));
        let mesh = load_json_as_mesh(path.to_str().unwrap());
        let source = mesh.positions_without_indices();
        assert!(!source.is_empty(), "真实描边不能是占位：{name}");
        assert_eq!(
            actual.len(),
            source.len() + 1,
            "保留全部原点，只追加封口：{name}"
        );
        assert_eq!(
            &actual[..source.len()],
            source,
            "不能重采样或去重原描边：{name}"
        );
        assert_eq!(actual.last(), source.first(), "追加原首点：{name}");
    }

    fn assert_original_b_catalog(demo: bool) {
        let root = original_b_installation();
        let assets = AssetMeshLibrary::new();
        let watch = MeshWatch::new(false);
        let mut library = DecoratorMeshLibrary::default();
        // 完整执行生产入口，不生成 JSON，也不预填四表或替换资产库。
        load_decorators(&root, &mut library, &assets, &watch, demo);
        let count = if demo { 34 } else { 52 };
        assert_eq!(EXPECTED.len(), 52);
        let mut expected_keys: Vec<_> = EXPECTED
            .iter()
            .filter(|e| !(demo && matches!(e.subtype, 6..=9 | 22 | 23)))
            .map(|e| (e.subtype, e.rank))
            .collect();
        expected_keys.sort_unstable();
        assert_eq!(expected_keys.len(), count);
        for (table, keys) in [
            (
                "collision",
                library
                    .collision
                    .keys()
                    .map(|k| (k.subtype, k.variant))
                    .collect::<Vec<_>>(),
            ),
            (
                "interaction",
                library
                    .interaction
                    .keys()
                    .map(|k| (k.subtype, k.variant))
                    .collect::<Vec<_>>(),
            ),
            (
                "outline",
                library
                    .outline
                    .keys()
                    .map(|k| (k.subtype, k.variant))
                    .collect::<Vec<_>>(),
            ),
            (
                "collision_outline",
                library
                    .collision_outline
                    .keys()
                    .map(|k| (k.subtype, k.variant))
                    .collect::<Vec<_>>(),
            ),
        ] {
            let mut keys = keys;
            keys.sort_unstable();
            assert_eq!(keys, expected_keys, "真实 B 四表 {table}, demo={demo}");
        }
        for expected in EXPECTED {
            let key = DecoratorLibKey::new(expected.subtype, expected.rank);
            if demo && matches!(expected.subtype, 6..=9 | 22 | 23) {
                continue;
            }
            let collision = library.collision_record(key);
            let interaction = library.interaction_record(key);
            assert_original_asset_collision(&root, &assets, collision, expected.collision);
            if matches!(expected.interaction, Source::Same) {
                assert!(
                    Arc::ptr_eq(&collision.mesh, &interaction.mesh),
                    "原 same 分支必须共享 mesh Arc：{key:?}"
                );
                assert_eq!(collision.triangles, interaction.triangles, "{key:?}");
                if !collision.triangles.is_empty() {
                    assert_ne!(
                        collision.triangles.as_ptr(),
                        interaction.triangles.as_ptr(),
                        "三角 Vec 应真实 clone：{key:?}"
                    );
                }
            } else {
                assert_original_asset_collision(&root, &assets, interaction, expected.interaction);
                assert!(
                    !Arc::ptr_eq(&collision.mesh, &interaction.mesh),
                    "原不同 shape 不可伪共享：{key:?}"
                );
            }
            match (&expected.outline, library.get_outline(key)) {
                (Outline::Mesh(name), DecoratorOutline::Mesh(actual)) => {
                    assert_eq!(actual.as_str(), *name, "{key:?}");
                    let mesh = required_mesh(&assets, name);
                    assert!(!mesh.positions_without_indices().is_empty(), "{key:?}");
                    assert!(
                        std::path::Path::new(&mesh.source_file.as_ref().unwrap().path)
                            .canonicalize()
                            .unwrap()
                            .starts_with(&root)
                    );
                }
                (Outline::Lines(name), DecoratorOutline::Lines(points)) => {
                    assert_original_asset_lines(&root, name, points)
                }
                (Outline::Sphere(radius), DecoratorOutline::Sphere(actual)) => {
                    assert_eq!(actual, radius, "{key:?}")
                }
                _ => panic!("真实 B 主描边类型不符：{key:?}"),
            }
            match (
                &expected.collision_outline,
                library.get_collision_outline(key),
            ) {
                (Outline::Lines(name), DecoratorCollisionOutline::Lines(points)) => {
                    assert_original_asset_lines(&root, name, points)
                }
                (Outline::None, DecoratorCollisionOutline::None) => {}
                _ => panic!("真实 B 碰撞描边类型不符：{key:?}"),
            }
        }
        eprintln!(
            "真实 B 装饰目录加载完成：root={}, demo={demo}, 四表各{count}键",
            root.display()
        );
    }

    #[test]
    #[ignore = "必须显式提供 TG_INSTALL_ROOT 原版 B 完整安装树"]
    fn original_b_asset_tree_full_populates_52_keys_in_all_four_tables() {
        assert_original_b_catalog(false);
    }

    #[test]
    #[ignore = "必须显式提供 TG_INSTALL_ROOT 原版 B 完整安装树"]
    fn original_b_asset_tree_demo_populates_34_keys_in_all_four_tables() {
        assert_original_b_catalog(true);
    }
}
