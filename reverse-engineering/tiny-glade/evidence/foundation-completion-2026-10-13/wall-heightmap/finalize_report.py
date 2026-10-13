from pathlib import Path
import json,hashlib
OUT=Path(__file__).resolve().parent;ROOT=OUT.parents[2]
files=['reconstruction/shared-foundation/crates/utils/src/wall_space.rs',
       'reconstruction/shared-foundation/crates/utils/src/geometry/split_heightmap.rs']
sources={p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in files}
features=[
 {'feature_id':'utils:27','current_status':'partial_missing_logic',
  'implemented_scope':['WallSpaceCoord::new uses original project_clamp','with_offset/clamped/unbounded, offset_x_clamped and subtract reuse WallSpace policies','WallSpaceRegion strict two-axis intersection with horizontal loop topology'],
  'source_mapping':[{'file':files[0],'function':'WallSpaceCoord::{new,with_offset,with_offset_clamped,with_offset_unbounded,offset_x_clamped,subtract};WallSpaceRegion::intersects'}],
  'missing_library_logic':[],
  'unresolved_contracts':['bound/is_bounded/to_affine3 have only inlinee declarations, no observed compiled procedure or inline instance in A/B/C inventories; B and C source hashes match but neither contains an executable body. Original semantics and frame orientation remain unknown; no guessed implementation.'],
  'validation_gaps':['Region representation and vertical/horizontal caller boundary have static original inline evidence and meaningful topology/touching regressions, not complete original struct/ABI acceptance.','with_offset and construction were observed at horizontal caller specializations; full vertical offset generic combinations not original-entry compared.'],
  'evidence':['evidence/foundation-completion-2026-10-13/wall-heightmap/wall-fixtures.json','evidence/foundation-completion-2026-10-13/base-assets/B-module-432.txt','evidence/foundation-completion-2026-10-13/wall-heightmap/B-module-453.txt']},
 {'feature_id':'utils:56','current_status':'implemented_scoped',
  'implemented_scope':['SplitReceiver supports above sample advancement and ordered enter/leave callbacks','RegionReceiver maintains first above region and updates consecutive above endpoints','Existing crossing-only API delegates to the same core; original receiver rejects invalid/NaN crossing fractions'],
  'source_mapping':[{'file':files[1],'function':'SplitReceiver;split_heightmap_into;RegionReceiver;split_heightmap_regions;split_heightmap_by_threshold'}],
  'missing_library_logic':[],'unresolved_contracts':[],
  'validation_gaps':['Generic reconstructed receiver protocol is a semantic API, not recovered Rust ABI. RowTrimmerSink row quantization and game wiring belong to the external wall-constructor package.'],
  'evidence':['evidence/foundation-completion-2026-10-13/wall-heightmap/heightmap-fixtures.json','evidence/foundation-completion-2026-10-13/wall-heightmap/lerp.asm.txt']}]
doc={'scope':'Wall space and full heightmap region protocol; seven libraries, B','features':features,'source_sha256':sources,
     'original_comparison':{'heightmap':{'cases':174,'original_callbacks_executed':True,'native_panics':9,'accepted_projection':'ordered callbacks and region start/end float bits'},'wall':{'cases':72,'accepted_projection':'observed inline wrapper to original projection helpers','bridge':'CRT fmodf via host math.fmod; NaN payload excluded'}},
     'behavior_fix':'First full heightmap comparison found endpoint ULP differences; corrected weighted lerp to original a+(b-a)*t then passed.',
     'verification':'Current final integration report supplies validation and current source hashes; local root regression seven tests passed before other owners completed.',
     'full_original_feature_accepted':False}
(OUT/'report.json').write_text(json.dumps(doc,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print(json.dumps({'features':len(features),'source_files':len(sources)}))
