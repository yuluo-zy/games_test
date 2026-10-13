"""Build explicit per-file review scope and original dependency identity audit."""
import collections,hashlib,json,subprocess,tomllib
from pathlib import Path
OUT=Path(__file__).parent;ROOT=OUT.parents[2];WS=ROOT/'reconstruction/shared-foundation'
source=WS/'crates/utils/src';evidence=ROOT/'evidence/shared-foundation-2026-10-13/utils-base'
audit=json.loads((evidence/'interface-coverage.json').read_text(encoding='utf-8'))
validation=json.loads((OUT/'validation.json').read_text(encoding='utf-8'))
def line(file,text):return (source/file).read_text(encoding='utf-8').split(text)[0].count('\n')+1
findings=[
 {'id':'UB-R01','severity':'none','category':'dismissed_false_positive','status':'dismissed_no_source_change',
  'interface':'LoopRange safe ExactSizeIterator adapter; original LoopRange::subslice/len retained',
  'file':'loop_range.rs','line':line('loop_range.rs','impl ExactSizeIterator'),
  'problem':'Inherent len preserves the original signed wrapping result on reversed subslices. Initial review wrongly assumed ExactSizeIterator default len called this inherent method; standard default instead reads size_hint and already returns zero.',
  'reproduction':'LoopRange::new_line(0,3,4).subslice(Range {start:2,end:1}); inherent len = usize::MAX; iterator next/next_back/count = empty.',
  'evidence':['../shared-foundation-2026-10-13/utils-tabloid/141200350.asm.txt','../shared-foundation-2026-10-13/utils-tabloid/141200510.asm.txt'],
  'original_evidence_scope':'Original len signed-extends end-start; original subslice checks start<len and end<=len but not start<=end. ExactSizeIterator surface was a reconstruction adapter.',
  'fix':'No fix needed; temporary explicit trait override and redundant test were removed before delivery.',
  'verification':'Original instruction len/subslice review plus std ExactSizeIterator default implementation; existing native LoopIdx tests pass.',
  'original_business_delta':False},
 {'id':'UB-R02','severity':'P2','category':'dependency_reimplementation','status':'fixed',
  'interface':'Spline2D/3D chord lengths in build_knots_open/open3/closed',
  'file':'natural_cubic.rs','line':line('natural_cubic.rs','let length = match N'),
  'problem':'Game knots accumulated a scalar hand-written vector dot/sqrt, while contract said vector arithmetic reused original glam. Dot ordering comment gave source-level rationale that is unnecessary for business equivalence.',
  'reproduction':'Read knots before review: array scalar subtraction, d[1]^2+d[0]^2(+d[2]^2), sqrt.',
  'evidence':['../shared-foundation-2026-10-13/utils-base/spline-rdp-native-fixtures.json','../../../tracking/original-Cargo.lock'],
  'fix':'Use original glam 0.29.3 Vec2/Vec3 subtraction and length; retain game chord sequence and scalar spline solver.',
  'verification':'44 original constructor snapshots and 120 position/120 tangent retained-fixture comparisons pass existing 1e-5 gate.',
  'original_business_delta':False,'unverified':'NaN payloads, repeated/nonfinite controls and extreme magnitudes not covered by retained fixtures.'},
 {'id':'UB-R03','severity':'P2','category':'confirmed_performance_issue','status':'fixed',
  'interface':'SelectiveDeserialize::visit_map key matching',
  'file':'selective_deserialize.rs','line':line('selective_deserialize.rs','struct SelectedKey'),
  'problem':'Each ignored map key was decoded into a fresh owned String solely to compare one field name. Allocation count scaled with skipped key count.',
  'reproduction':'selector-allocation-probe.rs: 50,000 unknown keys plus History/snapshots, same [7,9] payload.',
  'evidence':['selector-allocation.log','selector-policy.log','selective-before.rs'],
  'fix':'Serde decodes the key and the game visitor matches borrowed text; retain deserialize_string entry, UTF-8 forms, field order and error/default policy.',
  'verification':'50,003 allocations before, 1 after. All 12 existing selector policy cases plus escaped-key/late-error regression pass.',
  'original_business_delta':False,'unverified':'Original complete native visitor execution remains unverified; comparison here is candidate policy before/after, not native Save-GC benchmark.'},
 {'id':'UB-R04','severity':'P1','category':'original_shared_input_hazard','status':'open_consumer_precondition',
  'interface':'Rdp::new_with_opts',
  'file':'rdp.rs','line':line('rdp.rs','if maximum > epsilon'),
  'problem':'Negative epsilon with zero admissible deviation picks split_offset=0, requeues unchanged (first,last), growing output without progress.',
  'reproduction':'[Vec3::ZERO,Vec3::X,Vec3::new(2,0,0)], epsilon=-1, spacing=0; also noncollinear points with epsilon=-1 and spacing=10.',
  'evidence':['rdp-boundary-probe.json','../shared-foundation-2026-10-13/utils-base/rdp.legacy.c'],
  'original_evidence_scope':'Original entry 0x141fe0b90 also failed to return in 3,000,000 instructions for both negative cases. This supports an original-shared hazard rather than a reconstruction mismatch.',
  'fix':'No algorithm change: establish real caller nonnegative epsilon precondition before deciding to reject invalid inputs in adapter.',
  'verification':'Two positive/degenerate controls returned; 108 retained native RDP fixtures pass. Negative candidate was not run unbounded.',
  'original_business_delta':False},
 {'id':'UB-R05','severity':'P1','category':'missing_functionality','status':'open',
  'interface':'Vec3SliceAs2::as_vec2',
  'file':'swizzlers.rs','line':line('swizzlers.rs','pub struct Vec3SliceView2'),
  'problem':'Current view can expose &[Vec3] only; it has no projected Vec2 element API. Native four-byte entry proves pointer/length passthrough, not projected axes/stride or consumer semantics.',
  'reproduction':'Call points.as_vec2(): cannot index/iterate projected Vec2 through the returned API.',
  'evidence':['../shared-foundation-2026-10-13/utils-base/interface-batch-02/as_vec2.asm.txt'],
  'fix':'Leave explicitly partial until original caller proves return type, element stride and axes; never cast Vec3 stride 12 to a tightly-packed Vec2 slice stride 8.',
  'verification':'Borrowed source transport tests pass only; projected consumers are absent.'},
 {'id':'UB-R06','severity':'P1','category':'missing_functionality','status':'open',
  'interface':'22-file original interface closure, notably WallSpaceCoord bounds/offset/affine, LoopSlice/map_slice, Rdp new/stitch, Spline2D segment selection',
  'file':'wall_space.rs','line':line('wall_space.rs','pub struct WallSpaceCoord'),
  'problem':'191 observed original interfaces: 99 analyzed partially, 92 without accepted business analysis, 88 without mapped candidate. Some are trait/schema helpers pending attribution, so 92 is not 92 proven gameplay APIs.',
  'reproduction':'Original WallSpaceCoord::{new,bound,is_bounded,with_offset*,to_affine3}, WallSpaceRegion::intersects, LoopRange::map_slice/LoopSlice and Rdp::{new,into_indices,stitch_with} have no named reconstructed interfaces.',
  'evidence':['../shared-foundation-2026-10-13/utils-base/interface-coverage.json'],
  'fix':'Retain missing/untouched inventory; original caller/type evidence must drive implementation, not a guessed generic wrapper.',
  'verification':'Full original consumers, Bevy Time/SystemParam wiring, terrain/grid owner and snapshot payload type remain unverified.'},
 {'id':'UB-R07','severity':'P2','category':'performance_risk','status':'unmeasured',
  'interface':'Rdp::new_with_opts; Spline constructors; local_time cache',
  'file':'rdp.rs','line':line('rdp.rs','while let Some'),
  'problem':'RDP repeated segment rescans retain worst-case quadratic work. Spline uses several O(n) buffers and repeats knot prefix accumulation. Local-time conversion acquires a read lock per call.',
  'reproduction':'Adversarial RDP split chains or large control lists; frequency/size in real game consumers unknown.',
  'evidence':['rdp.rs static loop','natural_cubic.rs static allocation/solver review','local_time.rs static read lock'],
  'fix':'No speculative optimization: first recover representative consumer sizes/frequency and profile; keep game split policy and linear spline solve.',
  'verification':'No game frame timing/memory pressure measurement; allocation probe only validates selector-specific issue.'}
]
files=[]
for path in sorted(source.glob('*.rs')):
    if path.name=='lib.rs':continue
    rows=[r for r in audit['interfaces'] if r['module']==path.stem]
    files.append({'source':path.relative_to(ROOT).as_posix(),'sha256':hashlib.sha256(path.read_bytes()).hexdigest(),
                  'observed_original_interfaces':len(rows),'analyzed_partial':sum(r['analysis_status']=='analyzed_partial' for r in rows),
                  'untouched':sum(r['analysis_status']=='untouched' for r in rows),
                  'candidate_mapped':sum(r['candidate_implementation'] is not None for r in rows),
                  'whole_business_complete':False,
                  'untouched_interfaces':[r['canonical_key'] for r in rows if r['analysis_status']=='untouched']})
meta=subprocess.run(['cargo','metadata','--format-version','1','--locked','--offline'],cwd=WS,capture_output=True,text=True,encoding='utf-8')
assert meta.returncode==0,meta.stderr
metadata=json.loads(meta.stdout);packages={p['id']:p for p in metadata['packages']};nodes={n['id']:n for n in metadata['resolve']['nodes']}
root=next(p['id'] for p in packages.values() if p['name']=='utils' and Path(p['manifest_path']).resolve()==(WS/'crates/utils/Cargo.toml').resolve())
pending=[root];reachable=set()
while pending:
    package=pending.pop()
    if package in reachable:continue
    reachable.add(package);pending+=nodes[package]['dependencies']
original=tomllib.loads((ROOT/'tracking/original-Cargo.lock').read_text(encoding='utf-8'))['package']
current=tomllib.loads((WS/'Cargo.lock').read_text(encoding='utf-8'))['package']
ident=lambda p:(p['name'],p['version'],p.get('source'))
orig={ident(p):p for p in original};cur={ident(p):p for p in current}
dependencies=[]
for key in sorted(reachable):
    package=packages[key]
    if package['source'] is None:continue
    identity=ident(package);now=cur[identity];old=orig.get(identity)
    dependencies.append({'name':package['name'],'version':package['version'],'source':package['source'],
                         'checksum':now.get('checksum'),'matches_original_lock':old is not None and now.get('checksum')==old.get('checksum')})
for finding in findings:
    finding['evidence']=[value.replace('../shared-foundation-2026-10-13/','../../shared-foundation-2026-10-13/') for value in finding['evidence']]
report={'schema':'shared-foundation-code-review-v1','owner':'review_utils_base','baseline':'B','scope':'22 utils top-level files excluding lib.rs and peer geometry/curve/components',
        'source_text_1_to_1_required':False,'verification_authority':'Original interface/caller evidence; compile/test count is not original functionality completion.',
        'findings':findings,'statistics':{'files_reviewed':len(files),'original_interface_inventory':audit['statistics'],
                                        'fixed_items':2,'dismissed_false_positives':1,'open_original_shared_hazards':1,'open_missing_functionality_groups':2,'unmeasured_performance_risk_groups':1},
        'files':files,'dependency_identity':{'external_reachable_packages':len(dependencies),'all_original_identity_matched':all(p['matches_original_lock'] for p in dependencies),'packages':dependencies},
        'dependency_wheel_audit':{'scope':'All 22 owned root files inspected; no claim for all historic reconstruction code.',
                                'reused':['glam 0.29.3 vectors/quaternions/matrices','fastrand 1.8.0 RNG','half 2.4.1 f16','serde 1.0.200 parser/derive protocols','xxhash-rust 0.8.10 xxh3','time 0.3.36 offset/time conversion','std Vec/HashMap/Arc/locks/iterators/IO'],
                                'fixed':['Spline knot vector length scalar duplication'],
                                'retained_private_algorithms':['RDP game split/spacing/force rules','natural/periodic scalar spline solve','RGB9E5 game packing policy','angular/range/grid/filter game strategies'],
                                'not_business_deliverables':['Unicorn memory/allocation hooks','SDK orchestration','review GlobalAlloc counter']},
        'validation':'validation.json','limits':['No full original game/ECS/asset consumer chain acceptance.','Some untouched impl/derive/schema interfaces may be compiler/dependency forwarding; remain unresolved instead of fabricated gameplay API counts.','Prior verification.json source hashes for two edited files are historical; current hashes/checks are in review validation.json.','RDP instruction-budget exhaustion supports a hazard but is not formal nontermination proof.','Selector allocation comparison is candidate before/after using original Serde/serde_json, not full native visitor execution.']}
(OUT/'review.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print(report['statistics']);print('external dependencies',len(dependencies),report['dependency_identity']['all_original_identity_matched'])
