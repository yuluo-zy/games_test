from pathlib import Path
import json,hashlib
root=Path('/Users/yuluo/Desktop/game/games_test');w=Path(__file__).parent
sha=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
rows=[]
for task,owner,candidate,manifest,source in [
 ('TG-52f7fc4f94ad125530','cloth-visual-state-20261020','window-producer-root-candidate-v2','SOURCE_CANDIDATE_MANIFEST_V2.json','cottage_wall_window.rs'),
 ('TG-22f30e59ceb8a973c8','cloth-visual-state-20261020','corner-window-root-candidate-v1','SOURCE_CANDIDATE_MANIFEST_V1.json','cottage_corner_window.rs'),
 ('TG-73d8f60997815a3b36','scene-clutter-producer-20261020','canonical-direct-caller-delegated-20261021','MANIFEST_V1.json','gothic_wall_window.rs')]:
 d=root/'reverse-engineering/tiny-glade/skeleton/task-index/work'/task/owner/candidate
 rel=Path('tiny-glade/crates/systems/decorator/src/decorator_visual')/source
 b=d/'before'/rel;p=d/'candidate'/rel
 if not b.exists():b=d/'before'/source
 assert b.exists() and p.exists(),(b,p)
 rows.append({'task_id':task,'owner_retained':owner,'work_dir':str(d),'source':str(rel),'before':str(b),'before_sha256':sha(b),
  'candidate':str(p),'candidate_sha256':sha(p),'patch':str(d/'SOURCE.patch'),'patch_sha256':sha(d/'SOURCE.patch'),
  'manifest':str(d/manifest),'manifest_sha256':sha(d/manifest),
  'current_actual_before_matches':sha(root/rel)==sha(b),'current_actual_sha256':sha(root/rel),
  'scope':'complete wall/corner original body; no host mount' if task!='TG-73d8f60997815a3b36' else 'only typed direct caller migration of old partial body; NOT complete gothic; DO NOT MOUNT'})
review=root/'tiny-glade/reports/main-loop-render-recovery-2026-10-19/independent-validation/current-render-20261020/WALL_CORNER_WINDOW_SOURCE_PAIR_REVIEW_V1.json'
assert review.exists()
deps=[]
for rel,checks in [
 ('tiny-glade/crates/country-core/src/resources/window_lights.rs',['pub struct WindowLight','pub struct GlobalWindowLightsControl']),
 ('tiny-glade/crates/systems/decorator/src/window_lights.rs',['EventReader<LightWindowCmd>','pub fn window_lights']),
 ('tiny-glade/crates/systems/decorator/src/utils/visual_entity_builder.rs',['pub window_light: Option<u64>','if let Some(a) = self.window_light']),
 ('tiny-glade/crates/country-core/src/render/nani/systems.rs',['SolidVertexColorWindowGlassInstanceData','&WindowLight']),
 ('tiny-glade/crates/systems/decorator/src/utils/mod.rs',['pub fn subdivide_wall_decorator']),
 ('tiny-glade/crates/utils/src/geometry/rectangle.rs',['pub fn get_edge_dirs_from_corner_id']),
 ('tiny-glade/crates/country-core/src/resources/walls/decorator.rs',['pub fn window_extra']),
 ('tiny-glade/crates/systems/decorator/src/blueprint.rs',['pub fn insert_for','pub fn set_positioned_colliders','pub fn set_cached_transform'])]:
 p=root/rel;text=p.read_text();deps.append({'source':str(p),'current_actual_sha256':sha(p),'required_symbols':{x:x in text for x in checks},'claim':'actual source existence/interface checked read-only; no new compile/CPU/GPU execution'})
o={'generated_from':'actual source and frozen candidates; no statistics/progress summaries used','window_sources':rows,
 'wall_corner_independent_review':{'path':str(review),'sha256':sha(review)},'gothic_review':'requested core_business_lead; not complete native body',
 'already_present_production_dependencies':deps,'canonical_schema_dependency':'core_business_lead unique four-table+26loader+ID producer atomic compilation group',
 'circle_closest84_dependency':False,'circle_note':'these 3 callers do not read Circle.closest_point; new location/display/validator and Circle84 increment excluded from minimal compilation group',
 'host_installation':'EXCLUDED; no App caller/plugin registration added by this manifest','tests':'wall12/corner6 authored but not executed by author; gothic no new tests, parse only; root owns actual validation'}
p=w/'CANONICAL_WINDOW_CALLERS_20261021.json';p.write_text(json.dumps(o,ensure_ascii=False,indent=2)+'\n');print(str(p));print(sha(p));print([(x['source'],x['current_actual_before_matches']) for x in rows]);print([(x['source'].split('/')[-1],all(x['required_symbols'].values())) for x in deps])
