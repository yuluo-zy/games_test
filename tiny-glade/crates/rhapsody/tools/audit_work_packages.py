from pathlib import Path
import re
import json, hashlib, subprocess
root=Path(__file__).resolve().parents[4]
source=root/'reverse-engineering/tiny-glade/reconstruction/render/work-packages'
destination=root/'tiny-glade/crates/rhapsody/src'
pattern=re.compile(r'\b(?:pub(?:\([^)]*\))?\s+)?(?:unsafe\s+)?fn\s+([A-Za-z_][A-Za-z_0-9]*)\s*[<(]')
names=set().union(*(set(pattern.findall(p.read_text(encoding='utf8'))) for p in destination.rglob('*.rs')))
destinations={
    'barrier-record-r5':['raw/barrier.rs'],
    'compiled-state':['rg/compiled.rs','rg/compiled_graph.rs'],
    'descriptor-binding-r8':['raw/descriptor_binding.rs','raw/descriptor_set.rs'],
    'device-counters-r6':['raw/device.rs'],
    'execution-chain-r8':['rg/executor.rs'],
    'executor-temporal-r4':['rg/executor.rs','rg/recorder.rs','rg/vulkan.rs','rg/temporal.rs'],
    'gpu-lifecycle':[],
    'gpu-lifecycle-r3':['raw/managed/tracking.rs','raw/managed/handle.rs','raw/managed/lifecycle.rs','raw/managed/resources.rs'],
    'graph-compiler-r2':['rg/graph.rs','rg/resource_info.rs','rg/resource_rules.rs','rg/compiled.rs','rg/resource_registry.rs','rg/resource_pool.rs'],
    'pipeline-descriptors-r7':['raw/descriptor_set.rs'],
    'pipeline-interfaces-r6':['raw/shader.rs'],
    'queue-callers-r7':['raw/command_buffer.rs','raw/device.rs'],
    'queue-submit-r8':['raw/submit.rs'],
    'record-resources-r7':['rg/recorder.rs'],
    'record-submit-r6':['rg/recorder.rs','rg/executor.rs','raw/submit.rs'],
    'registry-pool':['rg/resource_registry.rs','rg/resource_pool.rs'],
    'resource-info':['rg/resource_info.rs','raw/resource_usage.rs'],
    'sequence-domain-r5':['raw/device.rs','raw/managed/lifecycle.rs'],
    'sequence-runtime-r8':['raw/managed/lifecycle.rs'],
    'temporal-lifecycle-r8':['rg/state.rs'],
    'temporal-r5':['rg/temporal.rs'],
}
notes={}
for line in (destination.parent/'WORK_PACKAGE_AUDIT.md').read_text(encoding='utf8').splitlines():
    if line.startswith('| `'):
        _,name,note,_=line.split('|')
        notes[name.strip().strip('`')]=note.strip()
rows=[]
for package in sorted(source.iterdir()):
    if not package.is_dir(): continue
    bodies=list(package.glob('src/**/*.rs'))+list(package.glob('*.rs'))
    text='\n'.join(p.read_text(encoding='utf8') for p in bodies)
    missing=sorted(set(pattern.findall(text))-names)
    inputs=[]
    for p in bodies:
        inputs.append(dict(path=p.relative_to(root).as_posix(),sha256=hashlib.sha256(p.read_bytes()).hexdigest(),
            commit=subprocess.check_output(['git','log','-1','--format=%H','--',str(p.relative_to(root))],cwd=root,text=True).strip()))
    targets=[(destination / target).relative_to(root/'tiny-glade').as_posix() for target in destinations[package.name]]
    if package.name=='temporal-lifecycle-r8': targets.append('crates/country-core/src/render/frame_constants.rs')
    rows.append(dict(package=package.name,source_files=inputs,formal_modules=targets,
        source_function_name_candidates=sorted(set(pattern.findall(text))),
        names_not_present_in_current_rhapsody_tree=missing,explanation=notes[package.name],
        name_matching_is_not_functional_acceptance=True))
(destination.parent/'work-package-migration.json').write_text(json.dumps(dict(schema_version=1,baseline='B',packages=rows),ensure_ascii=False,indent=2)+'\n',encoding='utf8')
print(f'Audited {len(rows)} work-package directories; maps in work-package-migration.json')
