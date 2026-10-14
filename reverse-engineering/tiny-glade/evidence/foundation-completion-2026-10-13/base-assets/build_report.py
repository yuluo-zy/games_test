"""Current completion overlay; never rewrites historical review evidence."""
import hashlib
import json
from pathlib import Path

OUT = Path(__file__).resolve().parent
ROOT = OUT.parents[2]
PREFIX = 'evidence/foundation-completion-2026-10-13/base-assets/'
features = []

def feature(fid, status, scope, missing, unresolved, gaps, mapping, evidence):
    features.append(dict(feature_id=fid, current_status=status,
                         implemented_scope=scope, missing_library_logic=missing,
                         unresolved_contracts=unresolved, validation_gaps=gaps,
                         source_mapping=mapping,
                         evidence=[PREFIX + p for p in evidence]))

feature('utils:06', 'implemented_scoped',
        ['ComparativeSpace defaults: interpolate via subtract then project_clamp; strict distance < sum of half-extents intersection; ordered subtract < 0 side decision.',
         'ZeroBoundedComparativeSpace implements linear distance, inclusive projection, clamp and subtraction; existing circular strategy reused.'],
        [], ['classify_side original enum/ABI is not recovered; ComparativeSide expresses the observed consumer decision.'],
        ['New predicates checked against bounded original disassembly and contract cases, not a complete original function ABI harness.',
         'Arbitrary nonpositive/NaN periods and invalid bounds have not been accepted as game inputs.'],
        ['utils/src/comparative_space.rs', 'Original inline lerp module183 record2292; regions module432 records14416/14500 and module3064 records3352/3436; classify module3064 record3584.',
         'Linear strategy inline module761 and compiled WallSpace project/subtract callees.'],
        ['inline-bindings.json','ComparativeSpace-lerp_clamp-183-2292.asm.txt','regions_intersect-432-14500.asm.txt','classify_side-3064-3584.asm.txt','comparative_tests.rs'])

feature('utils:13', 'implemented_scoped',
        ['LoopRange::map_slice, borrowed LoopSlice, forward/backward iteration, exact remaining iterator size, signed wrapping physical indexing.',
         '60 original traces compare values and start/end/period after every mixed-direction step.'],
        [], ['Invalid period/range combinations retain underlying signed index/panic policy; consumer admissibility is not established.'],
        ['Index consumer has bounded instruction evidence; invalid period and physical out-of-bounds panic messages/ABI are not directly compared.'],
        ['utils/src/loop_range.rs', 'Original complete next 0x141208e10; next_back consumer prefix 0x141237cbc; Index module1419/390 inlines.'],
        ['native-base-fixtures.json','native_base.py','loop_slice_tests.rs','native-0x141208e10.c','LoopSlice-next_back-1452-10148.asm.txt','LoopSlice-index-1419-528.asm.txt'])

feature('utils:17', 'partial_missing_logic',
        ['Existing 2D/3D constructors, natural/cyclic solve, segment polynomial position/tangent evaluation, and 3D global clamped selector remain implemented.',
         'Vector subtraction/length delegates to original glam. No new guessed 2D selector was added.'],
        ['Spline2D global find_segment_and_dt / corresponding global query policy remains absent.'],
        ['Original B Spline2D IPI0xAF37 is a declaration only (source line164), with no compiled entry or inline host found; 3D IPI0xAF46 at line285 is separate evidence.'],
        ['A 3D selector cannot be reported as direct original verification of the unobserved 2D policy.',
         'No original game-edit consumer integration closure is claimed.'],
        ['utils/src/natural_cubic.rs', 'PDB module762; Spline2D IPI0xAF37 declaration, Spline3D IPI0xAF46 compiled inline in position/tangent.'],
        ['pdb-focus.json','B-module-762.txt'])

feature('utils:18', 'implemented_scoped',
        ['Rdp::new uses observed Vec2 default threshold 0.1 and root=0.',
         'stitch_with keeps the old prefix through the cut, unconditionally drops the new seam point/index, appends points, and wrapping-rebases new indices by the old cut index.',
         'Existing generic RDP geometry and options implementation retained.'],
        [], ['The 0.1 default is directly observed for Vec2; other dimensions share the reconstructed API policy but have no direct default-producer oracle.',
             'Negative epsilon infinite splitting is shared with original machine code; caller nonnegative precondition remains unresolved.'],
        ['48 original Vec2 default constructors compared; bounded original stitch executed for four cuts; the current Rust regression embeds one of those cuts.',
         'Real editing consumer ownership/error paths remain outside this library closure.'],
        ['utils/src/rdp.rs', 'Original default producer 0x140ad1fe0; stitch IPI0x7EBF/module567 record29048 in compute_final_curve, bounded original execution 0x140ad47ab..0x140ad4bab.'],
        ['native-base-fixtures.json','native_base.py','rdp_tests.rs','native-0x140ad1fe0.c','Rdp-stitch_with-567-29048.asm.txt'])

feature('utils:23', 'partial_missing_logic',
        ['Existing scalar x0y/xvy/xvz/x0z coordinate adapters use original glam; borrowed Vec3SliceView2 transports source without allocation.'],
        ['Projected borrowed-view consumer access/index/iteration remains absent.'],
        ['as_vec2 original 4-byte shared identity entry 0x14067dd80 only proves pointer transport; projected axes, stride and author return type are not established.',
         'No unsafe compact Vec2 slice cast or guessed XZ projection was introduced.'],
        ['No typed original consumer was located to validate projected slice semantics.'],
        ['utils/src/swizzlers.rs', 'PDB module757 record848 as_vec2; shared identity machine entry cannot by itself identify its return consumer contract.'],
        ['B-module-757.txt'])

default_scope = [
    'Concrete render default retains required mesh/shader markers and no mesh processor.',
    'Enabled RT inherits parent first then resolves scale and all seasonal albedo to unit triples, semantic None, mesh required marker; absent optional object remains absent.',
    'Enabled audio resolves nested trigger scale to unit triple and shape/kind to required markers; exhausted outer trigger is rejected, while missing required nested fields remain markers until extraction.',
    'Default finalization runs even when parent optional RT/audio is disabled; original Option specializations drove the authored Inherit::finish_inheritance hook.',
    'All these operations remain game configuration policy; serde/RON parsing and generic dependency internals are reused.'
]
for fid in ['asset:01', 'asset:03']:
    feature(fid, 'implemented_scoped', default_scope, [],
            ['No invented Aabb/string defaults for required RT mesh or audio shape/kind.',
             'This scope completes the observed render/RT/audio/trigger default owners; it does not assert all consumers or every author-defined asset type have been surveyed.'],
            ['200 complete original RT option cases and 36 complete audio option cases compare parent priority, default tags/payloads and nine panic outcomes.',
             'The audio fixture grid currently uses absent parent; enabled-audio parent inheritance also has authored RON integration coverage, not a complete native grid.',
             'Original GPU/audio runtime consumption and malformed ABI-state acceptance are outside this completion scope.'],
            ['asset/src/inheritable.rs', 'asset/src/prefab/config.rs',
             'Original Option RT complete 0x14268a2e0; Option audio complete 0x14268a700; RT field inheritance 0x142879de0; render 0x142879c20.',
             'Module4590 nested default inlines at0x14268a790; PrefabPart/render default module4091 record6228.'],
            ['asset-inline-bindings.json','native-prefab-fixtures.json','native_prefab.py','prefab_default_tests.rs','prefab_native_tests.rs','PrefabRt-inherit-4590-5564.asm.txt','AudioTrigger-concrete_default-4590-6808.asm.txt'])

owned = [ROOT/'reconstruction/shared-foundation/crates/utils/src'/name for name in
         ['comparative_space.rs','loop_range.rs','natural_cubic.rs','rdp.rs','swizzlers.rs']]
for crate in ['asset','asset-pipe']:
    owned.extend(sorted((ROOT/'reconstruction/shared-foundation/crates'/crate/'src').rglob('*.rs')))
sha = {p.relative_to(ROOT).as_posix(): hashlib.sha256(p.read_bytes()).hexdigest() for p in owned}
report = dict(schema_version=1, scope='base-assets completion overlay; historical reports immutable',
              original_version='B', original_rne_sha256='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03',
              features=features, source_sha256=sha,
              validation=dict(asset_completion='5 tests passed; 200 RT + 36 audio native cases',
                              asset_and_asset_pipe='25 tests passed --locked --offline; includes current completion tests',
                              rdp_completion='2 tests passed; 48 native constructors and seam regression',
                              loop_slice='60 native transition traces passed previously in this completion run',
                              comparative='contract cases passed previously in this completion run',
                              clippy='Blocked by peer geometry rectangle.rs manual_range_contains and geometry/native_tests.rs bool_assert_comparison; no owner error found in current log',
                              logs=[PREFIX+p for p in ['asset-test-current.log','asset-all-current.log','rdp-test-current.log','clippy-current.log','clippy-lib-current.log']]))
(OUT/'report.json').write_text(json.dumps(report, indent=2)+'\n',encoding='utf-8')
print('features',len(features),'source hashes',len(sha))
