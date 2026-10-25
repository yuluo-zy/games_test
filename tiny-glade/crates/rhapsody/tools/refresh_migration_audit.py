"""One-time recovery audit/import bookkeeping; original inputs remain read-only."""
from pathlib import Path
import hashlib, json, shutil, subprocess, tomllib

root = Path(__file__).resolve().parents[4]
formal = root / 'tiny-glade'
source = root / 'reverse-engineering/tiny-glade/reconstruction/rhapsody-rebuild'
crate = formal / 'crates/rhapsody'
entries = []
def record(src, dst, kind='recovered_scoped'):
    entries.append(dict(path=dst.relative_to(formal).as_posix(), source_path=src.relative_to(root).as_posix(),
        source_sha256=hashlib.sha256(src.read_bytes()).hexdigest(), sha256=hashlib.sha256(dst.read_bytes()).hexdigest(), kind=kind,
        commit=subprocess.check_output(['git','log','-1','--format=%H','--',str(src.relative_to(root))], cwd=root, text=True).strip()))

tests = ['allocator','attachments','buffer_desc','budget_snapshot','crash','managed','pass_api','pass_bindings',
    'pipeline_cache','state','parallel_chunks']
for name in tests:
    src = source / f'tests/{name}.rs'
    dst = crate / f'tests/{name}.rs'
    if dst.exists(): continue
    shutil.copyfile(src,dst)
    if name == 'parallel_chunks':
        text = dst.read_text(encoding='utf8').replace('ChunkStrategy, balanced_chunks, original_budget', 'ChunkStrategy, original_budget')
        text = text.replace('fn ids(n:', '#[cfg(feature = "candidate-optimizations")]\nuse rhapsody::rg::parallel::balanced_chunks;\n\nfn ids(n:')
        for function in ['balanced_respects_max_chunks_and_minimizes_the_heaviest_chunk',
                         'balanced_estimates_unknown_costs_from_the_known_mean_and_caps_chunks']:
            text = text.replace(f'#[test]\nfn {function}', f'#[cfg(feature = "candidate-optimizations")]\n#[test]\nfn {function}')
        # The mixed property test covers both policies; feature-gate only candidate branch.
        text = text.replace('for chunks in [', 'for chunks in [')
        dst.write_text(text,encoding='utf8')

readable = root / 'reverse-engineering/tiny-glade/reconstruction/human-readable-shaders/shaders'
for src in readable.rglob('*.hlsl'):
    dst = formal / 'shaders' / src.relative_to(readable)
    dst.parent.mkdir(parents=True,exist_ok=True)
    if not dst.exists(): shutil.copyfile(src,dst)
    record(src,dst,'recovered_shader_scoped')

for branch in ['raw','rg']:
    for src in (source/'src'/branch).rglob('*.rs'):
        dst=crate/'src'/branch/src.relative_to(source/'src'/branch)
        if dst.exists():
            kind='recovered_with_candidate_adapter_boundaries'
            if src.name=='format_table.rs': kind='public_dependency_registry_data'
            elif src.name=='suballoc.rs': kind='candidate_adapter_feature_disabled'
            elif src.name=='graveyard.rs': kind='candidate_lifetime_integration_not_original_bucket_algorithm'
            record(src,dst,kind)
for src in (source/'src/shader_pipeline').glob('*.rs'):
    if src.name=='mod.rs': continue
    record(src,crate/'crates/shader-pipeline/src'/src.name)
record(source/'src/shader_compiler.rs',crate/'crates/shader-compiler/src/lib.rs')
for src in (source/'src/shader_compiler').glob('*.rs'):
    record(src,crate/'crates/shader-compiler/src'/src.name,'recovered_compiler_contract_with_explicit_adapter')
for name in tests: record(source/f'tests/{name}.rs',crate/f'tests/{name}.rs','portable_contract_test')
record(source/'tests/shader_compiler_contract.rs',crate/'crates/shader-compiler/tests/original_contract.rs','original_shipped_contract_test')
record(root/'reverse-engineering/tiny-glade/reconstruction/render/work-packages/descriptor-binding-r8/src/lib.rs',
    crate/'src/raw/descriptor_binding.rs','recovered_original_sparse_tag_array_dynamic_offset_contract')

# Restore all recovered source files without treating source completion as whole-frame acceptance.
status=json.loads((root/'reverse-engineering/tiny-glade/reconstruction/shader-source/SHADER_STATUS.json').read_text(encoding='utf8'))
manifest_path=formal/'shaders/manifest.json'
manifest=json.loads(manifest_path.read_text(encoding='utf8'))
manifest['policy']='All recovered HLSL sources and include support are retained. Structural/execution evidence is scoped; EDGE, original-empty/dead depth variants and unresolved interfaces remain explicit.'
manifest['files']=status
for row in manifest['files']:
    if row['file']=='final_post.raster.hlsl':
        row['integration_limit']='Color variant recovered; original depth variant has no output, with unresolved dead-computation/resource-reflection preservation. No invented side effects added.'
    if row['file']=='water/ice_refl_resolve.cs.hlsl':
        row['integration_limit']='Recovered implementation retained. EDGE is attributed by strict and -Gis comparison to expression reassociation/FMA; default compiler/driver bit equivalence is not claimed.'
manifest['include_files']=[row['file'] for row in status if not row['modules']]
native=json.loads((root/'reverse-engineering/tiny-glade/evidence/shader-gap-investigation-2026-10-13/corpus-final-native/report.json').read_text(encoding='utf8'))['rows']
by_bin={row['file']:row for row in native}
for row in status:
    if not row['modules']: continue
    builds=manifest['build'].setdefault(row['file'],[])
    for module in row['modules']:
        if any(item['bin']==module['bin'] and item['entry']==module['entry'] for item in builds): continue
        recipe=by_bin.get(module['bin'],{}).get('recipe_defines',[])
        builds.append(dict(bin=module['bin'],defines=[key if value=='1' else f'{key}={value}' for key,value in recipe],entry=module['entry'],profile=dict(CS='cs_6_4',VS='vs_6_4',PS='ps_6_4')[module['stage']]))
    for item in builds:
        original=by_bin.get(item['bin'])
        if original:
            item['artifact_defines']=original['defines']
            item['artifact_configuration_recovered']=original['variant_config_recovered']
            item['artifact_configuration_validation_scope']=original['variant_validation_scope']
        item['recipe_defines_are_original']=False
manifest_path.write_text(json.dumps(manifest,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
packages=tomllib.loads((formal/'Cargo.lock').read_text(encoding='utf8'))['package']
adapter=next(p for p in packages if p['name']=='gpu-allocator' and p['version']=='0.27.0')
(formal/'provenance/dependency-exceptions.json').write_text(json.dumps(dict(schema_version=1,exceptions=[dict(
    package={key:adapter.get(key) for key in ['name','version','source','checksum']},
    manifest='crates/rhapsody/Cargo.toml',dependency='gpu-allocator',feature='candidate-suballocation',
    reason='Original local 0.26 fork is unidentified and not ash-0.38 compatible with public 0.26; optional public 0.27 is an explicit candidate adapter, never original identity equivalence.',
    evidence='../../reverse-engineering/tiny-glade/reconstruction/rhapsody-rebuild/DEPENDENCY_AUDIT.md'
)]),ensure_ascii=False,indent=2)+'\n',encoding='utf8')
(formal/'provenance/shader-exceptions.json').write_text(json.dumps(dict(schema_version=1,exceptions=[dict(
    file='final_post.raster.hlsl',bin='_final_post.raster.hlsl_f0adff76827e73ff.bin',entry='ps_main',stage='PS',gate=False,execution='SAME',
    reason='Original depth PS has no output. Lifting and new DXC remove dead computations and resources; retain the recovered color implementation and record the unresolved depth interface.',
    evidence='../../reverse-engineering/tiny-glade/evidence/shader-gap-investigation-2026-10-13/GAP_ANALYSIS.md'),dict(
    file='water/ice_refl_resolve.cs.hlsl',bin='_water_ice_refl_resolve.cs.hlsl_cs_main_0.bin',entry='cs_main',stage='CS',gate=True,execution='EDGE',
    reason='Strict/NoContraction and -Gis comparisons identify expression reassociation/FMA as the bounded EDGE. The observed execution status is retained verbatim.',
    evidence='../../reverse-engineering/tiny-glade/reconstruction/shader-source/SHADER_STATUS.md'
)]),ensure_ascii=False,indent=2)+'\n',encoding='utf8')
(crate/'migration-sources.json').write_text(json.dumps(dict(schema_version=1,baseline='B',files=entries),ensure_ascii=False,indent=2)+'\n',encoding='utf8')
print(f'Recorded {len(entries)} source mappings; HLSL files: {len(list((formal/"shaders").rglob("*.hlsl")))}')
