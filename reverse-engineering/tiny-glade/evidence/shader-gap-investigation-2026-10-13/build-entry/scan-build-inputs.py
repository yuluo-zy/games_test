import hashlib,json,re,subprocess,tomllib
from pathlib import Path
ROOT=Path(r'D:/game/ljxsj_92385/Tiny Glade')
OUT=Path(__file__).parent
LLVM=Path(r'C:/Program Files/LLVM/bin/llvm-pdbutil.exe')
KEYS=('shader-compiler','shader_compiler','asset-pipe-bin','asset_pipe_bin','shader-pipeline','shader-prepper','shader_prepper','shader-variants')
def digest(path):
    h=hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda:stream.read(1024*1024),b''):h.update(block)
    return h.hexdigest()
manifest=json.loads((ROOT/'build-info/manifest.json').read_text())
current_files=sorted(p.relative_to(ROOT).as_posix() for p in ROOT.rglob('*') if p.is_file())
manifest_files=[row[0] for row in manifest['files']]
compiler_pattern=re.compile(r'(?:^|/)(?:asset[-_]pipe[-_]bin|shader[-_]compiler|dxc|dxcompiler)(?:[./_-]|$)',re.I)
locks=tomllib.loads((ROOT/'build-info/Cargo.lock').read_text())
scan={'source_identity':{p:digest(ROOT/p) for p in ['tiny-glade.rne','tiny_glade.pdb','build-info/manifest.json','build-info/Cargo.lock','build-info/toolchain-info.json']},'manifest_record_count':len(manifest_files),'current_file_count':len(current_files),'manifest_executables_or_libraries':[p for p in manifest_files if p.lower().endswith(('.exe','.dll','.pdb'))],'current_executables_or_libraries':[p for p in current_files if p.lower().endswith(('.exe','.dll','.pdb'))],'manifest_shader_build_tool_matches':[p for p in manifest_files if compiler_pattern.search(p)],'current_shader_build_tool_matches':[p for p in current_files if compiler_pattern.search(p)],'manifest_author_source_or_build_logs':[p for p in manifest_files if p.lower().endswith(('.rs','.hlsl','.log','.toml'))],'current_author_source_or_build_logs':[p for p in current_files if p.lower().endswith(('.rs','.hlsl','.log','.toml'))],'not_in_manifest':[p for p in current_files if p not in set(manifest_files)],'manifest_absent_current':[p for p in manifest_files if p not in set(current_files)],'packages':[p for p in locks['package'] if p['name'] in ('asset-pipe-bin','shader-compiler','shader-pipeline','shader-prepper','shader-variants','tempfile')],'toolchain_info':json.loads((ROOT/'build-info/toolchain-info.json').read_text()),'limits':['manifest paths are original integrity-record membership, not proof every listed current file has original bytes','Cargo local package does not prove a separate executable was shipped','Rust host/target do not establish the external DXC executable host or path'] }
(OUT/'release-build-inventory.json').write_text(json.dumps(scan,ensure_ascii=False,indent=2),encoding='utf8')
module=None;module_count=0;file_count=0;hits=[]
p=subprocess.Popen([str(LLVM),'dump','-files',str(ROOT/'tiny_glade.pdb')],stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,encoding='utf8',errors='replace')
with (OUT/'pdb-files-build-related.txt').open('w',encoding='utf8') as out:
    for line in p.stdout:
        m=re.match(r'^Mod (\d+) \| \x60(.*?)\x60',line)
        if m:module={'id':int(m[1]),'name':m[2]};module_count+=1
        f=re.match(r'^\s*- \(SHA-256: ([0-9A-Fa-f]+)\) (.+)$',line)
        if f:
            file_count+=1
            if any(key in f[2] for key in KEYS):
                record={'module':module,'path':f[2],'source_checksum_sha256':f[1].lower()};hits.append(record);out.write(json.dumps(record,ensure_ascii=False)+'\n')
    p.wait()
    err=p.stderr.read()
if p.returncode:raise RuntimeError(err)
report={'llvm_exit_code':p.returncode,'command':[str(LLVM),'dump','-files',str(ROOT/'tiny_glade.pdb')],'module_records':module_count,'source_file_records':file_count,'query_keys':KEYS,'matches':hits,'build_tool_author_path_matches':[h for h in hits if any(k in h['path'] for k in ('shader-compiler','shader_compiler','asset-pipe-bin','asset_pipe_bin'))],'limits':['PDB source checksums identify source files represented in this runtime PDB; not a claim no other development tool existed','No author text can be derived from a SHA256 source checksum']}
(OUT/'pdb-files-build-related.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf8')
print(json.dumps({'manifest_files':len(manifest_files),'current_files':len(current_files),'pdb_modules':module_count,'pdb_source_records':file_count,'matching_source_records':len(hits),'build_tool_author_source_hits':len(report['build_tool_author_path_matches'])}))
