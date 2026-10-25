"""Write the scoped contract beside the existing C source; no native/GPU execution."""
import hashlib
import json
from pathlib import Path

WORK = Path(__file__).resolve().parent
PROJECT = WORK.parents[4]
SOURCE_DIR = PROJECT / 'skeleton/C/crates/country-core/src/systems/render_loop/compute_passes'

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def relative(path):
    return str(path.relative_to(PROJECT))

def native(pack, name, rva):
    path = WORK / pack / (name + '.json')
    data = json.loads(path.read_text())
    index = next(i for i, row in enumerate(data['instructions']) if int(row['rva'], 16) == rva)
    return dict(path=relative(path), sha256=sha(path), pointer=f'/instructions/{index}',
                rva=hex(rva), bytes=data['instructions'][index]['bytes'])

def shader(word):
    path = WORK / 'half_res.original.spirv.txt'
    assert f'{word:04d} ' in path.read_text()
    return dict(path=relative(path), sha256=sha(path), pointer=f'word_offset:{word}')

rows = [
 ('F01', 'RCX=sret48 bytes, RDX=GraphApi*, R8=GBuffer handle*, R9=depth handle*; author Rust ABI/name/type spelling unknown', None, [('native-pack','halfres-c',0xaed358),('native-chain','render-world',0x56c48c)], []),
 ('F02', 'Depth handle+8/+10 are u16 dimensions; max(ceil-div2,1)', 'half_dimensions', [('native-pack','halfres-c',0xaed364)], []),
 ('F03', 'Create depth then normals with the same half dimensions; preserve 15 written descriptor bytes and labels; no unknown padding defaults', 'extract_half_res_depth_normals', [('native-pack','halfres-c',0xaed38b),('native-pack','halfres-c',0xaed3cc),('native-pack','halfres-c',0xaed46d)], []),
 ('F04', 'Check GraphApi+0x24c==1 separately after each create; only then init_as_debug_garbage', 'extract_half_res_depth_normals', [('native-pack','halfres-c',0xaed3ed),('native-pack','halfres-c',0xaed48e)], []),
 ('F05', 'Stencil view uses with_stencil_aspect(0)=0x800000; retains depth resource ID/description, does not inherit input view bits', 'RhapsodyHalfResApi::stencil_view_from_default', [('native-pack','halfres-c',0xaed4c9),('native-chain','stencil-view',0xe21150),('native-pack','halfres-c',0xaed4d0)], []),
 ('F06', 'Actual URI country_core/extract_half_res_depth_normals:main; permutation storage words[0,8,0], internal key container semantics unknown', 'SHADER_URI', [('native-pack','halfres-c',0xaed4f8),('native-pack','halfres-c',0xaed523),('native-pack','halfres-c',0xaed53b)], []),
 ('F07', 'Tuple stencil,normals-output,depth-input,depth-output,GBuffer; graph registrations stencil read,b5 write,b2 read,b4 write,b3 read', 'EntryBindings', [('native-pack','halfres-c',0xaed561),('native-pack','halfres-c',0xaed5ae),('native-bindings','entry-bindings',0x1ab97ed),('native-bindings','entry-bindings',0x1ab99d6)], []),
 ('F08', 'DispatchThreads([halfWidth,halfHeight,1]); declared local-size8x8x1; group conversion stays in dependency', 'extract_half_res_depth_normals', [('native-pack','halfres-c',0xaed5bb),('native-pack','halfres-c',0xaed5e5)], [26]),
 ('F09', 'Return depth bytes0..23 then normals24..47; caller stores rbp+6f0 and rbp+5b0', 'HalfResOutputs', [('native-pack','halfres-c',0xaed5ea),('native-chain','render-world',0x56c4a0)], []),
 ('F10', 'Offsets00,01,11,10 indexed by frame.index&3; source=2*output+offset with 32-bit wrap', 'source_pixel', [], [988,1005]),
 ('F11', 'Stencil low3 bits==1 or2 sets float-depth lowest bit; other values clear it', 'shader_write_values', [], [1046,1094,1111]),
 ('F12', 'GBuffer.x low three bytes -> f32 bits998277249 scale -> multiply2/subtract1; no depth averaging/min/max', 'shader_write_values', [], [1158,1168,1177,1235,1240]),
 ('F13', 'normal4(w0) row-vector times world_to_view then Normalize.xyz; alpha sequence[-.5,+.5,+1,-1]', 'shader_write_values', [], [1245,1259,1272,1324]),
 ('F14', 'Uses only frame.index@0 and world_to_view@256 RowMajor/MatrixStride16; complete view declaration spans1024 bytes', 'UsedShaderParameters::from_uploaded_blocks', [], [301,405,414,599,979,1245]),
 ('F15', 'Output ImageFormat R32f/Rgba8Snorm; sampled GBuffer declares Rg32ui; original entry has no bounds guard', 'half_res.recovered.comp::main', [], [777,790,918,944]),
 ('F16', 'GBuffer created at56b074 (gbuffer,R32G32_UINT), copied7b0->5f0->198; writer submissions main_pass/main_pass_oc2/grass2/late_forward precede halfres; exact last shader writes unknown', None, [('native-chain','render-world',0x56b039),('native-chain','render-world',0x56b074),('native-chain','render-world',0x56b1ca),('native-chain','render-world',0x56c09c),('native-chain','render-world',0x56c252),('native-chain','render-world',0x56c46f)], []),
 ('F17', 'Depth is upper closure get_or_create_temporal(main depth) at399be5; passes via stack+30 to render_world, then7c8->608->1b0; exact shader writer chain unknown', None, [('native-producers','render-world-caller',0x399be5),('native-producers','render-world-caller',0x399caf),('native-producers','render-world-caller',0x399cdd),('native-chain','render-world',0x56b1df),('native-chain','render-world',0x56b1ea)], []),
 ('F18', 'Frame uniform: RenderTemporalState payload+0x108 u32 -> push frame_counter at398438; full TypeId match with get_res<RenderTemporalState>; field name/increment/reset unknown', 'UsedShaderParameters::from_resource_payloads', [('native-producers','render-world-caller',0x3982fc),('native-producers','render-world-caller',0x398315),('native-owners','get-temporal-state',0x647b1c),('native-owners','get-temporal-state',0x647b35),('native-producers','render-world-caller',0x398403),('native-producers','render-world-caller',0x398438)], []),
 ('F19', 'render_world copies first1024 bytes of GameViews at56ac29, init_binding_set(current_view) at56ac47; full TypeId match with get_res<GameViews>; matrix computation/update rule unknown', 'UsedShaderParameters::from_resource_payloads', [('native-chain','render-world',0x56ab35),('native-chain','render-world',0x56ab4e),('native-owners','get-game-views',0x647a1c),('native-owners','get-game-views',0x647a35),('native-chain','render-world',0x56ac15),('native-chain','render-world',0x56ac29),('native-chain','render-world',0x56ac47)], []),
]
facts = [dict(id=i, statement=s, version='C', classification='observed_static',
              evidence=[native(*n) for n in ns]+[shader(x) for x in ws],
              rust_symbol=('recovered_c::'+rust if rust else None),
              status='implemented_unverified' if rust else 'evidence_ready',
              review='not_performed', validation='not_run') for i,s,rust,ns,ws in rows]
consumers = []
for order, name, rva, target, depth, normals in [
 (1,'rhapsody_gi::calculate_gtao',0x56c6b7,0xccbe10,'R8=rbp+6f0','R9=rbp+5b0'),
 (2,'country_core::systems::render_loop::compute_passes::ray_trace_gi::ray_trace_gi',0x56c8c7,0xaebaa0,'R9=rbp+6f0','stack+20=rbp+5b0'),
 (3,'rhapsody_gi::eval_gi_screen_radiance',0x56c90c,0xccc590,'R8=rbp+6f0','R9=rbp+5b0'),
 (4,'country_core::systems::render_loop::compute_passes::restir_di::restir_sample_direct_lights',0x56c9bd,0xaed630,'R9=rbp+6f0','stack+20=rbp+5b0')]:
    consumers.append(dict(order_in_observed_caller_path=order, symbol=name, callsite_rva=hex(rva),
        target_rva=hex(target), depth_argument=depth, normals_argument=normals,
        evidence=native('native-chain','render-world',rva), status='evidence_ready',
        downstream_shader_reads='unknown', actual_gpu_order='unknown'))
gaps = [
 ('G01','Matching rhapsody commit adapter/ShaderPermutationKey API missing; trait has no real engine implementation','Obtain locked API and implement adapter without replacing engine internals'),
 ('G02','FrameCounter from RenderTemporalState+108; field original name and index increment/reset unknown','Trace RenderTemporalState+108 writes/reset; do not default frame index'),
 ('G03','Current view from GameViews first1024 bytes; CPU matrix production/update rule unknown','Trace GameViews primary view stores; distinguish other GameViews+400 branch upload'),
 ('G04','Input depth/stencil/GBuffer final shader writers, resource versions and synchronization unknown','Trace temporal main depth plus main_pass/grass2/late_forward actual shader bindings'),
 ('G05','Four consumers precise shader accesses/conditions and resize/reuse/disposal incomplete','Forward slice real arguments; reuse rhapsody lifetime API; preserve indirect alias gaps'),
 ('G06','Tail groups/OOB, odd-size edges, SNORM quantization and GLSL Lod0 vs original no-Lod fetch unresolved','Check real views/mips/robustness/dispatch then authorized GPU comparison; no clamp/zero defaults'),
 ('G07','Compilation, native differential, GPU/full chain runtime and independent review not performed','Verification remains paused; future runs bind frozen sources and actual C inputs'),
 ('G08','Author parameter names/complete Rust ABI/handle+21..23 padding and descriptor field names unknown','Confirm original dependency types; retain semantic projections without fake repr(C)'),
]
bindings = [dict(set=0,binding=n,name=name,access=access) for n,name,access in [
 (1,'stencil_tex','read'),(2,'depth_input_tex','read'),(3,'gbuffer_input_tex','read'),
 (4,'depth_output_tex','write'),(5,'normals_output_tex','write')]]
bindings += [dict(set=2,binding=0,name='frame_counter',access='uniform_read',used='index@0'),
             dict(set=4,binding=0,name='current_view',access='uniform_read',used='world_to_view@256 RowMajor,stride16')]
contract = dict(schema_version=1,date='2026-10-12',task_id='TG-b1237e511d78c48e8a',target_version='C',
 status='implemented_unverified',complete_pass=False,semantic_acceptance=False,source=relative(SOURCE_DIR/'half_res.rs'),
 author_source_text_recovered=False,original_binary_sha256='d8f9261b1a4cb677fe91b8f5017bcff63b1312228b3f3264a2071ca7d3ad80a9',
 original_pdb_sha256='df472d8e6ae642baed96127151b18685291468518ea8dc71557bce4508c78902',
 base_source_sha256='74503995689f9a008f4333b866b637a4e03a3e0423302a8ae7222e02f4d2c0bf',
 facts=facts,bindings=bindings,consumers=consumers,gaps=[dict(id=i,status='unknown',item=x,next=n) for i,x,n in gaps],
 authorization=dict(implementation=True,direct_existing_ABC_delivery=True,new_delivery_directory=False,validation='paused',agents_dispatched=False),
 dependency_reuse=dict(rhapsody_commit='48efa87114d22e484592a0e9efe866faafa6a876',glam_version='0.30.10',engine_ecs_allocator_math_internals_reimplemented=False),
 shader=dict(uri='country_core/extract_half_res_depth_normals:main',payload_offset=132,word_count=1330,instruction_count=276,
 payload_sha256='2a962aef4c657065c834d9f475fa22c2cc4977ab64c06b05cab44b202672e4b8',
 precomp_sha256='4e36b589039aeaaf9b7fa9761c004561a3e61b859956be51546fcb23471df455',local_size=[8,8,1]),
 static_scope='C original bytes/PDB/inline/SPIRV only; target instructions and GPU not executed',
 resource_owner_evidence=dict(path=relative(WORK/'resource-owner-matches.json'),sha256=sha(WORK/'resource-owner-matches.json')),
 raw_string_evidence=dict(path=relative(WORK/'static-literals.json'),sha256=sha(WORK/'static-literals.json')),
 earlier_generated_annotations='Historical metadata; superseded only for explicitly proved scopes by F01..F19',
 resource_lifecycle='Graph-owned outputs; actual graph/temporal resize/reuse/disposal unknown. No game edit/history callbacks invented',
 all_indirect_consumers_claimed=False,production_registration='Not performed; current generated skeleton is not a working render runtime',
 index_source_snapshot='Original pre-edit snapshot retained in task claim history; current generated index must be refreshed after direct integration',
 delivered_files=[relative(SOURCE_DIR/name) for name in ['half_res.rs','half_res.recovered.comp','half_res.recovery.md']])
(SOURCE_DIR/'half_res.rs.recovery.json').write_text(json.dumps(contract,ensure_ascii=False,indent=2)+'\n')
print(json.dumps(dict(facts=len(facts),bindings=len(bindings),consumers=len(consumers),gaps=len(gaps))))
