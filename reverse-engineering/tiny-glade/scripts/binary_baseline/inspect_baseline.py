"""Read-only identity refresh and bounded PDB index; never executes the target."""
from pathlib import Path
import hashlib, json, re, struct, subprocess

ROOT = Path('D:/game/ljxsj_92385/Tiny Glade')
PRIOR = Path('D:/game/TinyGlade_逆向初查')
OUT = Path('D:/game/reverse-engineering/tiny-glade/evidence/binary')
LLVM = Path('C:/Program Files/LLVM/bin')
prior = json.loads((PRIOR/'initial-analysis.json').read_text(encoding='utf-8-sig'))
manifest = dict(json.loads((ROOT/'build-info/manifest.json').read_text(encoding='utf-8-sig'))['files'])
identities = []
for old in prior['E01_file_identities']:
    p = ROOT/old['file']
    sha = hashlib.file_digest(p.open('rb'), 'sha256').hexdigest()
    identities.append({**old, 'sha256':sha, 'unchanged_since_prior_sha256':sha == old['sha256'],
                       'blake3_method':'Reused prior BLAKE3 only when fresh SHA256 proves bytes unchanged',
                       'manifest_expected_blake3':manifest[old['manifest_entry']]})

def tool(file, args):
    result = subprocess.run([str(LLVM/args[0]), *map(str,args[1:])], text=True, encoding='utf-8', errors='replace', capture_output=True, check=True)
    (OUT/file).write_text(result.stdout, encoding='utf-8')
    return result.stdout

pe = {name:tool(name+'.headers-debug.txt', ['llvm-readobj.exe','--file-headers','--sections','--coff-debug-directory', ROOT/name]) for name in ['tiny-glade.exe','tiny-glade.rne']}
pdb = tool('pdb-summary.txt', ['llvm-pdbutil.exe','dump','--summary', ROOT/'tiny_glade.pdb'])
modules = tool('pdb-modules.txt', ['llvm-pdbutil.exe','dump','--modules', ROOT/'tiny_glade.pdb'])
guids = {name:re.search(r'PDBGUID:\s*(\{[^}]+\})', text).group(1) for name,text in pe.items()}
ages = {name:int(re.search(r'PDBAge:\s*(\d+)',text).group(1)) for name,text in pe.items()}
guids['pdb'] = re.search(r'GUID:\s*(\{[^}]+\})',pdb).group(1)
ages['pdb'] = int(re.search(r'Age:\s*(\d+)',pdb).group(1))

def sections(data):
    off = struct.unpack_from('<I',data,0x3c)[0]
    count = struct.unpack_from('<H',data,off+6)[0]
    optlen = struct.unpack_from('<H',data,off+20)[0]
    base = struct.unpack_from('<Q',data,off+24+24)[0]
    rows=[]
    for i in range(count):
        p=off+24+optlen+40*i
        name=data[p:p+8].split(b'\0')[0].decode(errors='replace')
        vs,va,rs,rp=struct.unpack_from('<IIII',data,p+8)
        rows.append(dict(name=name,virtual_size=vs,rva=va,raw_size=rs,raw_pointer=rp))
    return base,rows

exe=(ROOT/'tiny-glade.exe').read_bytes(); rne=(ROOT/'tiny-glade.rne').read_bytes()
base, secs=sections(rne)
diff=[]; start=None
for i,(a,b) in enumerate(zip(exe,rne)):
    if a!=b and start is None: start=i
    if a==b and start is not None: diff.append((start,i));start=None
if start is not None: diff.append((start,len(exe)))
diff_summary={ 'equal_size':len(exe)==len(rne), 'different_bytes':sum(end-start for start,end in diff),
               'contiguous_changed_ranges':len(diff),
               'per_region':{s['name']:sum(max(0,min(end,s['raw_pointer']+s['raw_size'])-max(start,s['raw_pointer'])) for start,end in diff) for s in secs},
               'range_sample':[{'file_offset':hex(start),'byte_count':end-start} for start,end in diff[:20]],
               'scope':'Byte comparison only; attribution/purpose of modifications not established'}

# Existing architecture samples are reused; this one streaming pass fills history/save/render gaps.
groups={
 'history':re.compile(r'country_core.*(?:resources.*history|undo|redo)'),
 'save':re.compile(r'(?:country_core|tiny_glade).*(?:save_glade|save_session|load_glade|load_session|saving|session.*(?:save|load)|serialize.*(?:Scene|Session))'),
 'render':re.compile(r'country_core.*render_loop.*(?:render_frame|render_loop|nani|wall|roof|depth|shadow|deferred|indirect)'),
 'shape_relationships':re.compile(r'(?:country_core|system_roof|system_wall_constructor).*(?:intersect|overlap|neighbor|wall.*join|roof.*join|attach|rebuild|update_roof)'),
}
samples={k:[] for k in groups}; counts={k:0 for k in groups}
proc=subprocess.Popen([str(LLVM/'llvm-pdbutil.exe'),'dump','--publics',str(ROOT/'tiny_glade.pdb')], stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,encoding='utf-8',errors='replace')
active=None; total=0
for line in proc.stdout:
    if 'S_PUB32' in line:
        total+=1; active={'record':line.strip(),'pdb_record_offset':int(line.strip().split('|')[0]),'address':None}
        for k,patt in groups.items():
            if patt.search(line) and 'drop_in_place' not in line and '$u20$serde' not in line:
                counts[k]+=1
                if len(samples[k])<12: samples[k].append(active)
    elif active is not None and 'addr =' in line:
        active['address']=line.strip()
        match=re.search(r'addr = (\d+):(\d+)',line)
        if match and int(match.group(1))<=len(secs):
            sec=secs[int(match.group(1))-1]; offset=int(match.group(2))
            active['rva']=hex(sec['rva']+offset); active['va']=hex(base+sec['rva']+offset)
        active=None
proc.wait()
if proc.returncode: raise RuntimeError(proc.stderr.read())
(OUT/'focused-symbols.json').write_text(json.dumps({'record_count':total,'search_counts':counts,'samples':samples,
 'limitation':'Symbol labels and addresses are observed; algorithm, call order, runtime behavior and original Rust layouts remain unproven'},ensure_ascii=False,indent=2),encoding='utf-8')
report={'target_execution':False, 'identity':identities, 'pdb_match':{'guids':guids,'ages':ages,'all_match':len(set(guids.values()))==1 and len(set(ages.values()))==1},
 'exe_rne_difference':diff_summary,'section_mapping':secs,'image_base':hex(base),
 'toolchain':prior['E03_toolchain'], 'prior_report_sha256':hashlib.sha256((PRIOR/'initial-analysis.json').read_bytes()).hexdigest(),
 'reuse_sources':[str(PRIOR/'initial-analysis.json'),str(PRIOR/'pdb-types.txt'),str(PRIOR/'roof-entry-disassembly.txt')],
 'type_coverage':prior['E09_type_coverage'], 'local_version':prior['E11_local_version'],
 'provenance_limit':'Matching supplied manifest proves internal consistency, not independently verified official origin',
 'outputs':['pdb-summary.txt','pdb-modules.txt','tiny-glade.exe.headers-debug.txt','tiny-glade.rne.headers-debug.txt','focused-symbols.json']}
(OUT/'baseline.json').write_text(json.dumps(report,ensure_ascii=False,indent=2),encoding='utf-8')
print(json.dumps({'pdb_match':report['pdb_match'],'identities_unchanged':all(x['unchanged_since_prior_sha256'] for x in identities), 'exe_rne_difference':diff_summary, 'focused_symbol_counts':counts},ensure_ascii=False,indent=2))
