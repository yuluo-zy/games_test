//! 结构规则与解析结果：领域意图在这里转成引擎无关的数据，预览和正式生成共同消费。
//! 不生成门窗或网格；不同规则类型共享精确输入比较与单快照缓存。
//! 设计说明见 docs/上传与结构规则重构.md。
use crate::mesh::GeometryProfile;
use garden_domain::{BlockId, Building, Facade, Roof};
use garden_geometry::Rect;

/// 规则类型决定自己的输入语义和解析过程，公共缓存不判断业务类型。
pub trait StructureRule {
    /// 输入必须携带影响解析结果的全部数据与规则配置。
    type Input: Clone;
    /// 引擎无关的确定性结果，不保存运行时资源或世界放置。
    type Output;
    /// 返回 true 必须保证解析结果等价，禁止忽略会改变结果的依赖。
    fn equivalent(&self, previous: &Self::Input, next: &Self::Input) -> bool;
    /// 纯解析入口；实现不得读取未纳入输入的可变外部状态。
    fn resolve(&self, input: &Self::Input) -> Self::Output;
}

/// 每个消费者只缓存一个最新快照，不积累拖动期间的历史参数。
/// 比较命中后返回原解析结果，避免重新分配；规则实例固定在缓存内部，不能悄悄热换。
pub struct RuleCache<R: StructureRule> {
    rule: R,
    cached: Option<(R::Input, R::Output)>,
}
impl<R: StructureRule + Default> Default for RuleCache<R> {
    fn default() -> Self {
        Self::new(R::default())
    }
}
impl<R: StructureRule> RuleCache<R> {
    /// 为一个消费者创建固定规则实例和空缓存。
    pub fn new(rule: R) -> Self {
        Self { rule, cached: None }
    }
    /// 复用等价输入的结果，失效时替换唯一快照；返回值只借用本缓存。
    pub fn resolve(&mut self, input: &R::Input) -> &R::Output {
        if !self
            .cached
            .as_ref()
            .is_some_and(|(old, _)| self.rule.equivalent(old, input))
        {
            self.cached = Some((input.clone(), self.rule.resolve(input)));
        }
        &self.cached.as_ref().unwrap().1
    }
}

/// 已解析体块结构不含世界放置，平移和旋转可以复用同一局部结构。
#[derive(Debug, Clone, PartialEq)]
pub struct BuildingStructure {
    pub blocks: Vec<BlockStructure>,
}
/// 结构关系与尺寸的唯一解析结果；有效屋顶不覆盖玩家保存的屋顶意图。
#[derive(Debug, Clone, PartialEq)]
pub struct BlockStructure {
    pub block: BlockId,
    pub footprint: Rect,
    pub base_elevation: f64,
    pub height: f64,
    pub stories: u8,
    pub facade: Facade,
    pub effective_roof: Roof,
    pub terraces: Vec<Rect>,
}
/// 解析已经通过领域校验的承托树；输出按体块身份排序。
#[derive(Default)]
pub struct BuildingRule;
impl StructureRule for BuildingRule {
    type Input = Building;
    type Output = BuildingStructure;
    fn equivalent(&self, previous: &Building, next: &Building) -> bool {
        // 身份和放置不改变局部关系；体块身份、尺寸、样式和承托均参与精确比较。
        previous.blocks() == next.blocks()
    }
    fn resolve(&self, building: &Building) -> BuildingStructure {
        let mut blocks = building.blocks().iter().collect::<Vec<_>>();
        blocks.sort_by_key(|b| b.id);
        let blocks = blocks
            .into_iter()
            .map(|block| {
                let mut base = 0.;
                let mut parent = block.parent;
                while let Some(id) = parent {
                    let support = building.block(id).expect("领域已经校验承托关系");
                    base += support.height;
                    parent = support.parent;
                }
                let mut children = building
                    .blocks()
                    .iter()
                    .filter(|b| b.parent == Some(block.id))
                    .collect::<Vec<_>>();
                children.sort_by_key(|b| b.id);
                let effective_roof = if children.is_empty() {
                    block.roof_intent
                } else {
                    Roof::Flat
                };
                let terraces = if children.is_empty() {
                    Vec::new()
                } else {
                    let mut pieces = vec![block.footprint];
                    for child in children {
                        pieces = pieces
                            .into_iter()
                            .flat_map(|r| r.subtract(child.footprint))
                            .collect();
                    }
                    pieces
                };
                BlockStructure {
                    block: block.id,
                    footprint: block.footprint,
                    base_elevation: base,
                    height: block.height,
                    stories: block.stories.count(),
                    facade: block.facade,
                    effective_roof,
                    terraces,
                }
            })
            .collect();
        BuildingStructure { blocks }
    }
}

/// 屋顶规则的完整局部输入，含几何配置；配置变化自动使公共缓存失效。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RoofInput {
    pub footprint: Rect,
    pub kind: Roof,
    pub profile: GeometryProfile,
}
/// u 为屋顶横向、v 为纵向，长度单位米；不含体块高度及世界放置。
/// 平顶、双坡、四坡使用相同结果合同，预览可以简化拓扑但不能另算参数。
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RoofStructure {
    pub kind: Roof,
    pub along_z: bool,
    pub u: [f64; 2],
    pub v: [f64; 2],
    pub rise: f64,
    pub ridge: [f64; 2],
}
impl RoofStructure {
    /// 屋顶横向跨度，单位米，斜屋顶包含出檐。
    pub fn width(&self) -> f64 {
        self.u[1] - self.u[0]
    }
    /// 屋顶纵向跨度，单位米，斜屋顶包含出檐。
    pub fn depth(&self) -> f64 {
        self.v[1] - self.v[0]
    }
    /// 按统一屋顶朝向将横纵坐标映射到建筑局部 X/Y/Z。
    pub fn point(&self, u: f64, height: f64, v: f64) -> [f64; 3] {
        if self.along_z {
            [u, height, v]
        } else {
            [v, height, u]
        }
    }
}
/// 接受已校验的尺寸和几何配置，按屋顶类型计算朝向、坡高与屋脊。
#[derive(Default)]
pub struct RoofRule;
impl StructureRule for RoofRule {
    type Input = RoofInput;
    type Output = RoofStructure;
    fn equivalent(&self, previous: &RoofInput, next: &RoofInput) -> bool {
        previous == next
    }
    fn resolve(&self, input: &RoofInput) -> RoofStructure {
        let r = input.footprint;
        let along_z = r.depth >= r.width;
        let eave = if input.kind == Roof::Flat {
            0.
        } else {
            input.profile.eave
        };
        let (u, v) = if along_z {
            (
                [r.x - eave, r.right() + eave],
                [r.z - eave, r.back() + eave],
            )
        } else {
            (
                [r.z - eave, r.back() + eave],
                [r.x - eave, r.right() + eave],
            )
        };
        let half = (u[1] - u[0]) * 0.5;
        // 按屋顶类型运行不同规则，公共消费者不再各自判断坡高与屋脊端点。
        let pitched_rise = (half * input.profile.roof_pitch_degrees.to_radians().tan())
            .min(input.profile.max_roof_rise);
        let (rise, ridge) = match input.kind {
            Roof::Flat => (0., v),
            Roof::Gabled => (pitched_rise, v),
            Roof::Hipped => (pitched_rise, [v[0] + half, v[1] - half]),
        };
        RoofStructure {
            kind: input.kind,
            along_z,
            u,
            v,
            rise,
            ridge,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use garden_domain::{BuildingEdit, BuildingId, Placement, sample_building};
    #[test]
    fn building_cache_ignores_world_pose_but_recomputes_structural_changes() {
        let b = Building::try_new(sample_building(BuildingId::new(1).unwrap(), 6., 3.)).unwrap();
        let mut cache = RuleCache::<BuildingRule>::default();
        let original = cache.resolve(&b).blocks.as_ptr();
        let moved = b
            .edited(BuildingEdit::Move(Placement {
                x: 4.,
                z: 2.,
                elevation: 1.,
                yaw: 0.5,
            }))
            .unwrap();
        assert_eq!(cache.resolve(&moved).blocks.as_ptr(), original);
        let changed = moved
            .edited(BuildingEdit::SetRoof {
                block: BlockId::new(1).unwrap(),
                roof: Roof::Flat,
            })
            .unwrap();
        assert_eq!(cache.resolve(&changed).blocks[0].effective_roof, Roof::Flat);
    }
    #[test]
    fn roof_types_orientation_and_profile_use_one_resolved_contract() {
        let mut cache = RuleCache::<RoofRule>::default();
        let mut input = RoofInput {
            footprint: Rect {
                x: 1.,
                z: 2.,
                width: 4.,
                depth: 8.,
            },
            kind: Roof::Gabled,
            profile: GeometryProfile::default(),
        };
        let gable = *cache.resolve(&input);
        assert!(gable.along_z);
        assert_eq!(gable.ridge, gable.v);
        input.kind = Roof::Hipped;
        let hip = *cache.resolve(&input);
        assert!(hip.ridge[0] > hip.v[0] && hip.ridge[1] < hip.v[1]);
        input.profile.max_roof_rise = 0.1;
        assert_eq!(cache.resolve(&input).rise, 0.1);
        input.kind = Roof::Flat;
        assert_eq!(cache.resolve(&input).rise, 0.);
        assert_eq!(cache.resolve(&input).width(), 4.);
        input.footprint.width = 9.;
        assert!(!cache.resolve(&input).along_z);
    }
}
