import collections
import hashlib
import html
import json
import re
import struct
import sys
import tomllib
from pathlib import Path

ROOT = Path('D:/game/ljxsj_92385/Tiny Glade')
OUT = Path(__file__).resolve().parent
sys.stdout.reconfigure(encoding='utf-8')
sys.path.insert(0, 'D:/game/tmp/shader-origin-blake3')
import blake3

def sha(data):
    return hashlib.sha256(data).hexdigest()

def read_pipeline(path):
    data = path.read_bytes()
    pos = 0
    def u32():
        nonlocal pos
        value = struct.unpack_from('<I', data, pos)[0]
        pos += 4
        return value
    def string():
        nonlocal pos
        n = u32()
        assert n <= len(data) - pos
        value = data[pos:pos+n].decode('utf-8')
        pos += n
        return value
    stages = []
    for _ in range(u32()):
        kind = u32()
        assert kind <= 2
        entry = string()
        n = u32()
        assert n * 4 <= len(data) - pos
        module_offset = pos
        words = struct.unpack_from(f'<{n}I', data, pos)
        pos += n * 4
        assert words[0] == 0x07230203
        opcodes = collections.Counter()
        debug_strings = {}
        source_records = []
        line_sites = collections.defaultdict(list)
        offset = 5
        while offset < n:
            wc, op = words[offset] >> 16, words[offset] & 0xffff
            assert wc > 0 and offset + wc <= n
            opcodes[op] += 1
            operands = words[offset+1:offset+wc]
            if op == 7:
                debug_strings[operands[0]] = struct.pack(f'<{len(operands)-1}I', *operands[1:]).split(b'\0')[0].decode('utf-8')
            if op == 8:
                line_sites[operands[0]].append((operands[1],operands[2]))
            if op == 3:
                raw_text = struct.pack(f'<{len(operands)-3}I', *operands[3:]).split(b'\0')[0] if len(operands)>3 else b''
                source_records.append({'language': operands[0], 'version': operands[1], 'file_id': operands[2] if len(operands)>2 else None, 'source_text_bytes': len(raw_text), 'non_whitespace_bytes':len(raw_text.strip()), 'linefeeds':raw_text.count(b'\n'), 'source_text_preview': raw_text[:180].decode('utf-8',errors='replace') if raw_text.strip() else '(whitespace only)'})
            offset += wc
        for source in source_records:
            source['file'] = debug_strings.get(source['file_id'])
        stages.append({'stage': ['Compute','Vertex','Fragment'][kind], 'entry': entry,
                       'module_offset':module_offset, 'spirv_sha256':sha(data[module_offset:module_offset+n*4]),
                       'spirv_words': n, 'instruction_count': sum(opcodes.values()),
                       'source_records': source_records,
                       'debug_line_files': [{'file':debug_strings.get(file_id),'line_site_count':len(sites),'unique_lines':sorted(set(s[0] for s in sites))} for file_id,sites in sorted(line_sites.items())],
                       'source_continued_count': opcodes[2],
                       'debug_source_paths': sorted(set(x for x in debug_strings.values() if 'shader' in x.lower()))})
    dep_count = u32()
    assert dep_count < 2000
    dependencies = [string() for _ in range(dep_count)]
    return {'file': str(path.relative_to(ROOT)).replace('\\','/'), 'sha256': sha(data),
            'bytes':len(data), 'stages':stages, 'source_dependencies':dependencies,
            'reflection_offset':pos}

def main():
    manifest_path = ROOT/'build-info/manifest.json'
    manifest = json.loads(manifest_path.read_text('utf-8'))
    expected = dict(manifest['files'])
    modules = [read_pipeline(p) for p in sorted((ROOT/'compiled-shaders').glob('*.bin'))]
    sources = collections.defaultdict(list)
    source_lines = collections.defaultdict(set)
    mismatches = []
    for module in modules:
        for source in set(module['source_dependencies']):
            sources[source].append(module['file'])
        module['manifest_blake3'] = expected.get(module['file'])
        module['actual_blake3'] = blake3.blake3((ROOT/module['file']).read_bytes()).hexdigest()
        if module['actual_blake3'] != module['manifest_blake3']:
            mismatches.append(module['file'])
        for stage in module['stages']:
            for source in stage['source_records']:
                if source['file']:
                    source_lines[source['file']].add(source['linefeeds'])
    legal = (ROOT/'third-party-licenses.html').read_text('utf-8')
    legal_hits = {}
    for name in ['kajiya','FidelityFX','GPU-Raytracer','XeGTAO','FXAA','egui','shader-prepper','rhapsody']:
        hits = []
        for match in re.finditer(re.escape(name), legal, re.I):
            fragment = legal[max(0,match.start()-180):match.end()+500]
            text = html.unescape(re.sub('<[^>]+>', ' ', fragment))
            hits.append({'offset':match.start(),'text':re.sub(r'\s+',' ',text),
                         'links':re.findall(r'href=[\"\x27]([^\"\x27]+)', fragment)})
        legal_hits[name] = hits[:25]
    inventory = [{'source':source,'pipeline_count':len(files),'source_linefeed_counts':sorted(source_lines[source]),'files':files} for source,files in sorted(sources.items())]
    summary = {'original_root':str(ROOT),'pipeline_count':len(modules),
               'stage_count':sum(len(x['stages']) for x in modules),
               'unique_source_dependency_count':len(sources),
               'manifest_hash_algorithm':'BLAKE3',
               'shader_manifest_hash_mismatches':mismatches,
               'shader_manifest_missing_entries':[m['file'] for m in modules if not m['manifest_blake3']],
               'stages_with_non_whitespace_source_text':sum(any(s['non_whitespace_bytes'] for s in x['source_records']) for m in modules for x in m['stages']),
               'raw_artifact_hashes':{str(p.relative_to(ROOT)).replace('\\','/'):sha(p.read_bytes()) for p in [manifest_path,ROOT/'build-info/Cargo.lock',ROOT/'credits-crates.txt',ROOT/'third-party-licenses.html']}}
    summary['metadata_manifest_checks'] = {rel: {'expected':expected.get(rel), 'actual_blake3':blake3.blake3((ROOT/rel).read_bytes()).hexdigest()} for rel in ['build-info/Cargo.lock','credits-crates.txt','third-party-licenses.html']}
    packages = tomllib.loads((ROOT/'build-info/Cargo.lock').read_text('utf-8'))['package']
    summary['original_selected_packages'] = [p for p in packages if p['name'] in ['FidelityFX','GPU-Raytracer','XeGTAO','egui','shader-prepper','rhapsody','nani','country-core','kajiya']]
    summary['main_shader_source_count'] = sum(s.endswith(('.cs.hlsl','.raster.hlsl')) for s in sources)
    summary['source_directory_counts'] = dict(collections.Counter(s.split('/')[1] if '/' in s else s for s in sources))
    oracle_root = Path('D:/game/reverse-engineering/tiny-glade/evidence/render-passes/shaders/oracle-bin')
    summary['oracle_copy_check'] = {'compared':0, 'mismatched':[], 'missing':[]}
    for m in modules:
        path = oracle_root/Path(m['file']).name
        if not path.exists():
            summary['oracle_copy_check']['missing'].append(m['file'])
        else:
            summary['oracle_copy_check']['compared'] += 1
            if sha(path.read_bytes()) != m['sha256']:
                summary['oracle_copy_check']['mismatched'].append(m['file'])
    ray_report = OUT/'raytracer/raw_spirv_fingerprints.json'
    if ray_report.exists():
        fingerprints = json.loads(ray_report.read_text('utf-8'))
        authoritative = {(Path(m['file']).name,s['module_offset'],s['spirv_sha256']) for m in modules for s in m['stages']}
        candidates = [(m['file'],s['offset'],s['sha256']) for m in fingerprints['files'] for s in m['modules']]
        summary['raytracer_scan_structural_boundary_crosscheck'] = {'checked':len(candidates),'unmatched':[c for c in candidates if c not in authoritative]}
    for name,obj in [('original-shader-inventory.json',modules),('original-source-dependencies.json',inventory),('original-license-links.json',legal_hits),('summary.json',summary)]:
        (OUT/name).write_text(json.dumps(obj,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
    print(json.dumps({k:v for k,v in summary.items() if k not in ['original_selected_packages','source_directory_counts']},ensure_ascii=False,indent=2))
    for name,hits in legal_hits.items():
        print(name, json.dumps(hits[:4],ensure_ascii=False))
    print('UPSTREAM-LIKE DEPENDENCIES')
    for item in inventory:
        if re.search('fidelity|gtao|vashared|raytrac|fxaa|ffx_|vendor|third',item['source'],re.I):
            print(item['source'], item['pipeline_count'], item['source_linefeed_counts'])

if __name__ == '__main__':
    main()
