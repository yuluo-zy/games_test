mod structures {#[derive(Clone)] pub struct MaterializedWallSegment {pub positions:Vec<[f32;3]>}}
#[path="D:/game/reverse-engineering/tiny-glade/evidence/round12/wall-gpu/candidates-v23/crates/country-core/src/systems/wall/construction/arch_stats.rs"] mod arch_stats;
fn main(){
{let segments=vec![structures::MaterializedWallSegment{positions:vec![]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(1065353216u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(1065353216u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(1065353216u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(0u32),0.],[0.,f32::from_bits(2147483648u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(0u32),0.],[0.,f32::from_bits(2147483648u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(0u32),0.],[0.,f32::from_bits(2147483648u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(1065353216u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(1065353216u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(1065353216u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(0u32),0.],[0.,f32::from_bits(2147483648u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(0u32),0.],[0.,f32::from_bits(2147483648u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(0u32),0.],[0.,f32::from_bits(2147483648u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(1065353216u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(1065353216u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(1065353216u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(0u32),0.],[0.,f32::from_bits(2147483648u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(0u32),0.],[0.,f32::from_bits(2147483648u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(0u32),0.],[0.,f32::from_bits(2147483648u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(1065353216u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(1065353216u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(1065353216u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(0u32),0.],[0.,f32::from_bits(2147483648u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(0u32),0.],[0.,f32::from_bits(2147483648u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(0u32),0.],[0.,f32::from_bits(2147483648u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,false);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(1065353216u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(1065353216u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(1065353216u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(0u32),0.],[0.,f32::from_bits(2147483648u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(0u32),0.],[0.,f32::from_bits(2147483648u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(0u32),0.],[0.,f32::from_bits(2147483648u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};0];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,1u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,1u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,1080033280u32,0u32]);
assert_eq!(s.gates_count,1u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(1065353216u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,1u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(1065353216u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,1u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(1065353216u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,1080033280u32,0u32]);
assert_eq!(s.gates_count,1u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,1u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,1u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,1080033280u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,1u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,1u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,1u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,1080033280u32,0u32]);
assert_eq!(s.gates_count,1u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,1u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,1u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,1080033280u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,1u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,1u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,1u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,1080033280u32,0u32]);
assert_eq!(s.gates_count,1u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(0u32),0.],[0.,f32::from_bits(2147483648u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,1u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(0u32),0.],[0.,f32::from_bits(2147483648u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,1u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(0u32),0.],[0.,f32::from_bits(2147483648u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};1];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,1080033280u32,0u32]);
assert_eq!(s.gates_count,1u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(1065353216u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(1065353216u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(1065353216u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(0u32),0.],[0.,f32::from_bits(2147483648u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(0u32),0.],[0.,f32::from_bits(2147483648u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(0u32),0.],[0.,f32::from_bits(2147483648u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};2];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(1065353216u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(1065353216u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(1065353216u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(0u32),0.],[0.,f32::from_bits(2147483648u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(0u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(0u32),0.],[0.,f32::from_bits(2147483648u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(2147483648u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[0u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
{let segments=vec![structures::MaterializedWallSegment{positions:vec![[0.,f32::from_bits(0u32),0.],[0.,f32::from_bits(2147483648u32),0.],[0.,f32::from_bits(3212836864u32),0.],[0.,f32::from_bits(1065353216u32),0.]]};3];let mut s=arch_stats::WallArchMaterialStats::default();
s.observe_mask(f32::from_bits(1080033280u32),&segments,true);
assert_eq!([s.arches_length.to_bits(),s.gates_length.to_bits(),s.sewer_length.to_bits()],[1080033280u32,0u32,0u32]);
assert_eq!(s.gates_count,0u64);assert_eq!(s.sewer_count,0u64);}
println!("168 original arch material classification cases matched bitwise lengths and gate/sewer counters");}