"""Capture all retained loose ASM under explicitly scoped evidence/report roots."""
import hashlib
import json
from pathlib import Path
import subprocess
import zipfile
from artifacts import RUNTIME,WORKSPACE,PROJECT,SKELETON,atomic,baseline,canonical,db,digest,load,module_for

def capture_existing():
    roots=[PROJECT/'evidence',WORKSPACE/'tiny-glade/reports']
    command=['rg','--files',*[str(p) for p in roots],'-g','*.asm','-g','*.asm.txt',
             '-g','*disasm*.txt','-g','*.s','-g','*.S','-g','!**/target/**']
    paths=sorted(Path(p) for p in subprocess.check_output(command,text=True).splitlines())
    records=[{'path':str(p),'sha256':digest(p),'bytes':p.stat().st_size} for p in paths]
    manifest_sha=hashlib.sha256(canonical(records)).hexdigest()
    folder=RUNTIME/'raw-assembly';folder.mkdir(exist_ok=True)
    target=folder/(manifest_sha+'.zip')
    if not target.exists():
        temp=target.with_suffix('.tmp')
        if temp.exists():raise RuntimeError('Incomplete archive exists; preserve/audit it before retry')
        seen=set()
        with zipfile.ZipFile(temp,'x',compression=zipfile.ZIP_DEFLATED,compresslevel=1) as archive:
            for row in records:
                raw=Path(row['path']).read_bytes()
                if hashlib.sha256(raw).hexdigest()!=row['sha256']:raise ValueError('Source changed during capture: '+row['path'])
                if row['sha256'] not in seen:archive.writestr(row['sha256'],raw);seen.add(row['sha256'])
        temp.replace(target)
    else:
        with zipfile.ZipFile(target) as archive:
            for row in records:
                if hashlib.sha256(archive.read(row['sha256'])).hexdigest()!=row['sha256']:
                    raise ValueError('Retained assembly archive integrity failure')
    con=db()
    def node(row):con.execute('INSERT OR IGNORE INTO extra_nodes VALUES(?,?)',(row['key'],canonical(row).decode()))
    def edge(a,b,kind,evidence):
        row={'source':a,'target':b,'kind':kind,'evidence':evidence,'status':'observed'}
        con.execute('INSERT OR IGNORE INTO extra_links VALUES(?,?)',(hashlib.sha256(canonical(row)).hexdigest(),canonical(row).decode()))
    archive_key='raw-archive:'+manifest_sha
    node({'key':archive_key,'kind':'EvidenceArchive','name':'Retained loose assembly archive','version':'unbound',
          'evidence':str(RUNTIME/'raw-assembly-manifest.json'),'sha':manifest_sha,'identity_status':'unbound_artifacts'})
    for row in records:
        key='evidence_file:'+hashlib.sha256(canonical(row)).hexdigest()
        node({'key':key,'kind':'EvidenceFile','name':str(Path(row['path']).relative_to(WORKSPACE)),
              'version':'unbound','sha':row['sha256'],'bytes':row['bytes'],'evidence':row['path'],
              'blob':str(target.relative_to(RUNTIME))+'#'+row['sha256'],
              'identity_status':'archive_only_not_a_verified_binary_binding'})
        edge(archive_key,key,'ARCHIVES',str(RUNTIME/'raw-assembly-manifest.json'))
    # Include source-only modules/files instead of silently omitting optimized-away entries.
    known_sources=set();known_modules=set()
    for row in con.execute('SELECT version,sources FROM symbols'):
        for path in json.loads(row['sources']):
            known_sources.add((row['version'],path));known_modules.add((row['version'],module_for(path)))
    new_sources=new_modules=0
    for v in 'ABC':
        b=baseline(v)
        index=SKELETON/f'_pdb/{v}/source-files.json'
        for record in load(index):
            if record['pdb_sha256']!=b['pdb']['sha256']:raise ValueError('Source inventory PDB identity mismatch')
            path=record['path'];module=module_for(path);s=f'source:{v}:{path}';m=f'module:{v}:{module}'
            if (v,path) not in known_sources:
                node({'key':s,'kind':'SourceFile','name':path,'version':v,'evidence':str(index),
                      'physical_mapping':'no standalone procedure attribution in current index'})
                known_sources.add((v,path));new_sources+=1
                edge(s,m,'IN_MODULE',str(index))
            if (v,module) not in known_modules:
                node({'key':m,'kind':'Module','name':module,'version':v,'evidence':str(index),
                      'semantic_status':'unreviewed','physical_mapping':'source inventory only'})
                known_modules.add((v,module));new_modules+=1
            edge(f'binary:{v}',m,'HAS_MODULE',str(index))
        con.execute('INSERT OR IGNORE INTO enrich_inputs VALUES(?,?)',(str(index),digest(index)))
    # Preserve the existing business-module hierarchy, not just Cargo crate roots.
    tracking_path=PROJECT/'tracking/module-coverage.json'
    tracking=load(tracking_path)['modules']
    for v in 'ABC':
        prefixes=sorted((r['path'].removesuffix('/src') if r['path'].endswith('/src') else r['path']
                         for r in tracking if r.get('source_counts',{}).get(v,0)),key=len)
        for prefix in prefixes:
            key=f'module:{v}:{prefix}'
            if (v,prefix) not in known_modules:
                node({'key':key,'kind':'Module','name':prefix,'version':v,'evidence':str(tracking_path),
                      'semantic_status':'unreviewed','module_level':'tracked_business_submodule',
                      'scope':'Existing structural inventory; not current business acceptance'})
                known_modules.add((v,prefix));new_modules+=1
            parents=[p for p in prefixes if prefix.startswith(p+'/')]
            if parents:edge(f'module:{v}:{max(parents,key=len)}',key,'HAS_SUBMODULE',str(tracking_path))
            else:edge(f'binary:{v}',key,'HAS_MODULE',str(tracking_path))
        for version,path in list(known_sources):
            if version!=v:continue
            matches=[p for p in prefixes if path.startswith(p+'/')]
            if matches:edge(f'source:{v}:{path}',f'module:{v}:{max(matches,key=len)}','IN_MODULE',str(tracking_path))
    con.execute('INSERT OR IGNORE INTO enrich_inputs VALUES(?,?)',(str(tracking_path),digest(tracking_path)))
    con.commit();con.close()
    report={'files':records,'archive':str(target),'archive_sha256':digest(target),
            'manifest_sha256':manifest_sha,'file_count':len(records),'total_original_bytes':sum(r['bytes'] for r in records),
            'additional_source_files':new_sources,'additional_modules':new_modules,
            'scope':'Opaque retention; byte identity preserved, binary/function bindings remain unverified'}
    atomic(RUNTIME/'raw-assembly-manifest.json',report)
    return {k:v for k,v in report.items() if k!='files'}
