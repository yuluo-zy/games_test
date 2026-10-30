"""Identity-bound catalog and immutable assembly shards (stdlib + local LLVM)."""
from __future__ import annotations
import bisect
import ctypes as C
import gzip
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import shutil
import sqlite3
import struct
import subprocess
import time
import zipfile

WORKSPACE = Path(__file__).resolve().parents[4]
PROJECT = WORKSPACE / 'reverse-engineering/tiny-glade'
SKELETON = PROJECT / 'skeleton'
RUNTIME = WORKSPACE / 'tools/code-map-runtime'
COLLECTOR = Path('/Users/yuluo/.codex/skills/native-evidence-pack/scripts/collect.py')
LLVM = Path('/opt/homebrew/opt/llvm')
SCHEMA = 1

def canonical(x):
    return json.dumps(x, ensure_ascii=False, sort_keys=True, separators=(',', ':')).encode()

def digest(p):
    with Path(p).open('rb') as f:
        return hashlib.file_digest(f, 'sha256').hexdigest()

def load(p):
    p = Path(p)
    with (gzip.open(p, 'rt') if p.suffix == '.gz' else p.open()) as f:
        return json.load(f)

def lines(p):
    with gzip.open(p, 'rt') as f:
        for line in f:
            yield json.loads(line)

def atomic(p, value):
    p = Path(p)
    p.parent.mkdir(parents=True, exist_ok=True)
    temp = p.with_name(p.name + f'.{os.getpid()}.tmp')
    with temp.open('xb') as f:
        f.write(canonical(value) + b'\n')
        f.flush()
        os.fsync(f.fileno())
    os.replace(temp, p)

def space(required=5):
    free = shutil.disk_usage(RUNTIME).free / 2**30
    if free < required:
        raise RuntimeError(f'Free space {free:.1f} GiB < required {required} GiB; resumable stop')
    return free

def pack_module():
    spec = importlib.util.spec_from_file_location('code_map_native_pack', COLLECTOR)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module

def baseline(v, verify=True):
    b = load(PROJECT / 'tracking/baselines.json')['baselines'][v]
    if verify:
        pack = pack_module()
        for k in ('binary', 'pdb'):
            if digest(b[k]['path']) != b[k]['sha256']:
                raise ValueError(f'{v} {k} SHA mismatch')
        pe = pack.PE(b['binary']['path'])
        identity = pack.pdb_identity(b['pdb']['path'])
        if not any(x['guid'] == identity['guid'] and x['age'] == identity['age'] for x in pe.rsds):
            raise ValueError(f'{v}: PE/PDB GUID/Age mismatch')
    return b

def db(readonly=False):
    p = RUNTIME / 'catalog.sqlite3'
    con = sqlite3.connect(p.as_uri() + '?mode=ro' if readonly else p, uri=readonly, timeout=120)
    con.row_factory = sqlite3.Row
    return con

def module_for(path):
    return path.split('/src/', 1)[0] if '/src/' in path else path.rsplit('/', 1)[0]

def inventory():
    RUNTIME.mkdir(parents=True, exist_ok=True)
    space(50)
    if (RUNTIME / 'catalog.sqlite3').exists():
        raise RuntimeError('Catalog already exists; use its checkpoint, not an implicit rebuild')
    started = time.monotonic()
    temp = RUNTIME / 'catalog-building.sqlite3'
    if temp.exists():
        raise RuntimeError('Incomplete inventory exists; preserve/audit it before retry')
    con = sqlite3.connect(temp)
    con.executescript('''
      CREATE TABLE functions(version TEXT,rva INTEGER,sha TEXT,PRIMARY KEY(version,rva));
      CREATE TABLE spans(version TEXT,entry INTEGER,start INTEGER,end INTEGER,origin TEXT,
                         evidence TEXT,PRIMARY KEY(version,entry,start,end,origin));
      CREATE TABLE symbols(version TEXT,entry INTEGER,record TEXT,name TEXT,sources TEXT,
                           evidence TEXT,PRIMARY KEY(version,record));
      CREATE TABLE inputs(path TEXT PRIMARY KEY,sha TEXT,kind TEXT,version TEXT);
      CREATE TABLE assemblies(version TEXT,start INTEGER,end INTEGER,blob TEXT,member TEXT,
                              sha TEXT,status TEXT,instructions INTEGER,gaps INTEGER,
                              PRIMARY KEY(version,start,end));
      CREATE TABLE refs(version TEXT,entry INTEGER,site INTEGER,target INTEGER,kind TEXT,
                        evidence TEXT,PRIMARY KEY(version,entry,site,kind));
      CREATE INDEX spans_entry ON spans(version,entry);
      CREATE INDEX refs_target ON refs(version,target);
      CREATE INDEX symbols_entry ON symbols(version,entry);
    ''')
    identities = {}
    def register(p, kind, v):
        con.execute('INSERT OR IGNORE INTO inputs VALUES(?,?,?,?)',
                    (str(p), digest(p), kind, v))
    for v in 'ABC':
        b = baseline(v)
        identities[v] = b
        index = SKELETON / f'_pdb/{v}/enrichment/procedure-index.json.gz'
        stage = load(index.parent / 'metadata-stage.json')
        if stage['inputs']['pdb_sha256'] != b['pdb']['sha256']:
            raise ValueError(f'{v}: stale PDB procedure index')
        register(index, 'pdb_procedures', v)
        # Import all existing A/C symbol metadata, but extract new ranges only for B.
        for p in load(index):
            entry = p['rva']
            con.execute('INSERT OR IGNORE INTO functions VALUES(?,?,?)', (v, entry, b['binary']['sha256']))
            con.execute('INSERT OR IGNORE INTO spans VALUES(?,?,?,?,?,?)',
                        (v, entry, entry, entry + p['code_size'], 'pdb', str(index)))
            con.execute('INSERT OR IGNORE INTO symbols VALUES(?,?,?,?,?,?)',
                        (v, entry, 'pdb:' + p['procedure_id'], p['name'],
                         json.dumps(p.get('source_files', [])), str(index)))
        existing = SKELETON / f'_assembly/roof-v1/{v}/index.json.gz'
        if existing.exists():
            register(existing, 'assembly_index', v)
            for r in load(existing)['ranges']:
                p = SKELETON / r['evidence_path']
                if p.exists():
                    register(p, 'existing_assembly', v)
        calls = SKELETON / f'_dependencies/{v}/native-calls.jsonl.gz'
        if calls.exists():
            meta = load(calls.parent / 'native-input.json')
            if meta['binary']['sha256'] != b['binary']['sha256']:
                raise ValueError(f'{v}: dependency binary mismatch')
            register(calls, 'native_references', v)
        con.commit()
    g = RUNTIME / 'ghidra-functions.jsonl.gz'
    if not g.exists():
        raise RuntimeError('Identity-bound Ghidra export is required')
    register(g, 'ghidra_functions', 'B')
    rows = lines(g)
    identity = next(rows)
    if identity['sha256'] != 'cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323':
        raise ValueError('Wrong Ghidra analysis identity')
    base = identity['image_base']
    b = identities['B']
    original = pack_module().PE(b['binary']['path'])
    patched = pack_module().PE(WORKSPACE / 'B版本内容/tiny-glade.exe')
    analysis_differences = []
    for p in rows:
        if p['external']:
            continue
        entry = p['va'] - base
        try:
            original.section_for(entry, 1, executable=True)
        except Exception:
            continue
        con.execute('INSERT OR IGNORE INTO functions VALUES(?,?,?)', ('B', entry, b['binary']['sha256']))
        con.execute('INSERT OR IGNORE INTO symbols VALUES(?,?,?,?,?,?)',
                    ('B', entry, f'ghidra:{entry:x}', p['name'], '[]', str(g)))
        for start, end in p['ranges']:
            start, end = start - base, end - base
            try:
                raw = original.read_rva(start, end - start)
                equal = raw == patched.read_rva(start, end - start)
                original.section_for(start, end - start, executable=True)
            except Exception:
                equal = False
            if not equal:
                analysis_differences.append([entry, start, end])
                continue
            con.execute('INSERT OR IGNORE INTO spans VALUES(?,?,?,?,?,?)',
                        ('B', entry, start, end, 'ghidra_equal_bytes_candidate', str(g)))
    # PE .pdata entries are coverage ranges, never independent function proof.
    for s in original.sections:
        if s['name'] == '.pdata':
            raw = original.read_rva(s['rva'], min(s['raw_size'], s['virtual_size']))
            for off in range(0, len(raw) - 11, 12):
                start, end, unwind = struct.unpack_from('<III', raw, off)
                if start and end > start:
                    try:
                        original.section_for(start, end-start, executable=True)
                    except Exception:
                        continue
                    con.execute('INSERT OR IGNORE INTO spans VALUES(?,?,?,?,?,?)',
                                ('B', start, start, end, 'pe_unwind_range_candidate', b['binary']['path']))
    con.commit()
    inputs = [list(x) for x in con.execute('SELECT * FROM inputs ORDER BY path')]
    snapshot = hashlib.sha256(canonical({'schema': SCHEMA, 'inputs': inputs,
                                        'identities': identities})).hexdigest()[:24]
    counts = {v: con.execute('SELECT count(*) FROM functions WHERE version=?', (v,)).fetchone()[0] for v in 'ABC'}
    con.close()
    os.replace(temp, RUNTIME / 'catalog.sqlite3')
    report = dict(schema=SCHEMA, snapshot=snapshot, identities=identities, inputs=inputs,
                  physical_entries=counts, ghidra_original_byte_differences=analysis_differences,
                  elapsed_seconds=time.monotonic()-started, semantic_review_complete=False)
    atomic(RUNTIME / 'inventory.json', report)
    return {k: v for k, v in report.items() if k not in ('inputs','identities','ghidra_original_byte_differences')}

class Decoder:
    def __init__(self):
        self.library = (LLVM / 'lib/libLLVM.dylib').resolve()
        self.lib = C.CDLL(str(self.library))
        for name in ('LLVMInitializeX86TargetInfo','LLVMInitializeX86Target','LLVMInitializeX86TargetMC',
                     'LLVMInitializeX86Disassembler','LLVMInitializeX86AsmPrinter'):
            getattr(self.lib, name)()
        self.lib.LLVMCreateDisasm.argtypes = [C.c_char_p,C.c_void_p,C.c_int,C.c_void_p,C.c_void_p]
        self.lib.LLVMCreateDisasm.restype = C.c_void_p
        self.context = self.lib.LLVMCreateDisasm(b'x86_64-pc-windows-msvc', None, 0, None, None)
        if not self.context:
            raise RuntimeError('LLVM x86 decoder unavailable')
        self.lib.LLVMDisasmInstruction.argtypes = [C.c_void_p,C.c_void_p,C.c_uint64,C.c_uint64,C.c_char_p,C.c_size_t]
        self.lib.LLVMDisasmInstruction.restype = C.c_size_t
        self.lib.LLVMDisasmDispose.argtypes = [C.c_void_p]
        self.out = C.create_string_buffer(16384)

    def close(self):
        self.lib.LLVMDisasmDispose(self.context)

    def decode(self, data, start, base):
        buf = C.create_string_buffer(data)
        offset = 0
        rows, gaps = [], []
        while offset < len(data):
            length = self.lib.LLVMDisasmInstruction(self.context, C.byref(buf, offset),
                       len(data)-offset, base+start+offset, self.out, len(self.out))
            if not length:
                gaps.append(start+offset)
                offset += 1
                continue
            raw = data[offset:offset+length]
            rows.append([start+offset, raw.hex(), self.out.value.decode().strip()])
            offset += length
        return rows, gaps

def reference(row):
    rva, h, text = row
    raw = bytes.fromhex(h)
    if raw[0] == 0xe8 and len(raw) == 5:
        return 'CALLS', rva+5+int.from_bytes(raw[1:], 'little', signed=True)
    if raw[0] == 0xe9 and len(raw) == 5:
        return 'TAIL_CANDIDATE', rva+5+int.from_bytes(raw[1:], 'little', signed=True)
    if raw[0] == 0xeb and len(raw) == 2:
        return 'JUMP_CANDIDATE', rva+2+int.from_bytes(raw[1:], 'little', signed=True)
    if text.startswith(('call','lcall')):
        return 'INDIRECT_UNRESOLVED', None
    return None

def capacity():
    """Measure real decoder + compression costs on a stratified fixed sample."""
    con=db(True);b=baseline('B');pe=pack_module().PE(b['binary']['path']);decoder=Decoder()
    ranges=con.execute("SELECT DISTINCT start,end FROM spans WHERE version='B' ORDER BY end-start,start").fetchall()
    selected=[ranges[min(len(ranges)-1,int(i*(len(ranges)-1)/127))] for i in range(128)]
    total=sum(r['end']-r['start'] for r in ranges);raw_bytes=compressed=0;began=time.monotonic();gaps=0
    import zlib
    for row in selected:
        data=pe.read_rva(row['start'],row['end']-row['start'])
        rows,unknown=decoder.decode(data,row['start'],pe.image_base)
        raw_bytes+=len(data);compressed+=len(zlib.compress(canonical(rows),1));gaps+=len(unknown)
    seconds=time.monotonic()-began;decoder.close();con.close()
    estimate=total*compressed/max(raw_bytes,1)
    report={'sample_ranges':len(selected),'sample_bytes':raw_bytes,'sample_compressed_bytes':compressed,
            'sample_seconds':seconds,'sample_undecoded_bytes':gaps,'distinct_ranges':len(ranges),
            'total_requested_bytes_including_overlaps':total,'estimated_assembly_gib':estimate/2**30,
            'estimated_decode_seconds':seconds*total/max(raw_bytes,1),
            'planning_peak_gib':(estimate*3+(RUNTIME/'catalog.sqlite3').stat().st_size*12)/2**30,
            'scope':'Stratified range-size sample; graph expansion is a planning multiplier, not a measured total'}
    atomic(RUNTIME/'capacity.json',report)
    if space()<report['planning_peak_gib']+5:raise RuntimeError('Estimated capacity exceeds safe budget')
    return report

def enrich_catalog():
    """Preserve existing PDB inline/type/assembly evidence without inventing semantics."""
    con=db()
    con.executescript('''CREATE TABLE IF NOT EXISTS extra_nodes(key TEXT PRIMARY KEY,payload TEXT);
      CREATE TABLE IF NOT EXISTS extra_links(id TEXT PRIMARY KEY,payload TEXT);
      CREATE TABLE IF NOT EXISTS enrich_inputs(path TEXT PRIMARY KEY,sha TEXT);''')
    def node(row):
        con.execute('INSERT OR IGNORE INTO extra_nodes VALUES(?,?)',(row['key'],canonical(row).decode()))
    def edge(a,b,kind,evidence,status='observed'):
        row=dict(source=a,target=b,kind=kind,evidence=evidence,status=status)
        con.execute('INSERT OR IGNORE INTO extra_links VALUES(?,?)',(hashlib.sha256(canonical(row)).hexdigest(),canonical(row).decode()))
    for v in 'ABC':
        # Source-files schema varies by retained exporter; enumerate only the version source tree.
        files=sorted((SKELETON/v).rglob('*.rs.enriched.json.gz'))
        for p in files:
            old=con.execute('SELECT sha FROM enrich_inputs WHERE path=?',(str(p),)).fetchone()
            sha=digest(p)
            if old:
                if old['sha']!=sha:raise ValueError('Enrichment input changed; explicit snapshot migration required')
                continue
            doc=load(p)
            for f in doc.get('functions',[]):
                for host in f.get('inline_hosts',[]):
                    physical=f'function:{v}:{host["host_rva"]:x}'
                    if not con.execute('SELECT 1 FROM functions WHERE version=? AND rva=?',(v,host['host_rva'])).fetchone():continue
                    key=f'inline:{v}:{host["module"]}:{host["inline_record_offset"]}'
                    node({'key':key,'kind':'InlineInstance','name':f['name'],'version':v,
                          'host_rva':host['host_rva'],'module_record':host['module'],
                          'source_line':host.get('source_line'),'annotation_data':host.get('annotation_data',''),
                          'evidence':str(p),'status':'observed','exclusive_instruction_range':'unknown'})
                    edge(physical,key,'INLINES',str(p))
            con.execute('INSERT INTO enrich_inputs VALUES(?,?)',(str(p),sha))
        p=SKELETON/f'_pdb/{v}/structures/type-definitions.json.gz'
        if p.exists() and not con.execute('SELECT 1 FROM enrich_inputs WHERE path=?',(str(p),)).fetchone():
            for t in load(p):
                key='type:'+t['id']
                node({'key':key,'kind':'Type','name':t['name'],'version':v,
                      'definition_status':t['definition_status'],'evidence':str(p),
                      'size_bytes':t.get('size_bytes'),'layout_complete':False})
                edge(f'binary:{v}',key,'HAS_TYPE',str(p))
            con.execute('INSERT INTO enrich_inputs VALUES(?,?)',(str(p),digest(p)))
        idx=SKELETON/f'_assembly/roof-v1/{v}/index.json.gz'
        for span in load(idx)['ranges']:
            p=SKELETON/span['evidence_path']
            key=f'existing-assembly:{v}:{span["id"]}'
            node({'key':key,'kind':'Evidence','name':span['id'],'version':v,'evidence':str(p),
                  'sha':digest(p),'source_inline_frames_available':True})
            if con.execute('SELECT 1 FROM functions WHERE version=? AND rva=?',(v,span['rva'])).fetchone():
                edge(f'function:{v}:{span["rva"]:x}',key,'SUPPORTED_BY',str(p))
        con.commit()
    count=con.execute('SELECT count(*) FROM extra_nodes').fetchone()[0]
    con.close();return {'extra_nodes':count}

def read_blob(row):
    with zipfile.ZipFile(RUNTIME / row['blob']) as z:
        raw = z.read(row['member'])
    if hashlib.sha256(raw).hexdigest() != row['sha']:
        raise ValueError('Assembly blob integrity failure')
    return json.loads(raw)

def extract(limit=None):
    """Extract each distinct range, keeping origin/ownership separate from decoding."""
    space()
    con = db()
    inv = load(RUNTIME / 'inventory.json')
    decoder = Decoder()
    identity = {'library': str(decoder.library), 'sha256': digest(decoder.library),
                'collector_sha256': digest(COLLECTOR), 'extractor_sha256': digest(Path(__file__)),
                'version': subprocess.check_output([str(LLVM/'bin/llvm-objdump'),'--version'],text=True).splitlines()[0]}
    tool_path = RUNTIME / 'extractor.json'
    if tool_path.exists() and load(tool_path) != identity:
        if con.execute("SELECT count(*) FROM assemblies WHERE version='B'").fetchone()[0]:
            raise RuntimeError('Extractor identity changed; preserve old shards and explicitly migrate catalog')
        # A failed first batch has published no data. Preserve its descriptor for audit.
        old=load(tool_path)
        atomic(RUNTIME/'extractor-revisions'/f'{hashlib.sha256(canonical(old)).hexdigest()}.json',old)
    atomic(tool_path, identity)
    b = baseline('B')
    pe = pack_module().PE(b['binary']['path'])
    pending = con.execute('''SELECT DISTINCT s.start,s.end FROM spans s
        LEFT JOIN assemblies a ON a.version=s.version AND a.start=s.start AND a.end=s.end
        WHERE s.version='B' AND a.start IS NULL ORDER BY s.start,s.end''').fetchall()
    if limit:
        pending = pending[:limit]
    root = RUNTIME / 'assembly'
    root.mkdir(exist_ok=True)
    began = time.monotonic()
    done = 0
    failures = []
    section_cache = {s['rva']:pe.read_rva(s['rva'],s['raw_size']) for s in pe.sections if s['executable']}
    def raw_range(start,end):
        s=pe.section_for(start,end-start,executable=True)
        return section_cache[s['rva']][start-s['rva']:end-s['rva']]
    for batch in range(0,len(pending),512):
        space()
        group = pending[batch:batch+512]
        key = hashlib.sha256(canonical([inv['snapshot'],identity,[[r['start'],r['end']] for r in group]])).hexdigest()[:24]
        relative = f'assembly/{key}.zip'
        final = RUNTIME / relative
        temp = final.with_suffix('.tmp')
        records, refs = [], []
        # A committed shard is immutable; orphaned files are never trusted without verification.
        if temp.exists():
            temp.unlink()
        with zipfile.ZipFile(temp,'x',compression=zipfile.ZIP_DEFLATED,compresslevel=1) as archive:
            for span in group:
                start,end=span['start'],span['end']
                try:
                    raw=raw_range(start,end)
                    rows,gaps=decoder.decode(raw,start,pe.image_base)
                    # Check every decoded byte against the identity-bound original backing range.
                    for at,h,_ in rows:
                        if bytes.fromhex(h) != raw[at-start:at-start+len(h)//2]:
                            raise ValueError('Decoded byte mismatch')
                    doc={'schema':SCHEMA,'binary_sha256':b['binary']['sha256'],
                         'start':start,'end':end,'instructions':rows,'undecoded':gaps,
                         'range_sha256':hashlib.sha256(raw).hexdigest(),
                         'function_boundary_verified':False,'source_inline_mapping':'PDB metadata linked separately'}
                    payload=canonical(doc); sha=hashlib.sha256(payload).hexdigest();member=sha+'.json'
                    archive.writestr(member,payload)
                    records.append(('B',start,end,relative,member,sha,'partial' if gaps else 'ok',len(rows),len(gaps)))
                    for row in rows:
                        ref=reference(row)
                        if ref:
                            refs.append(('B',start,row[0],ref[1],ref[0],f'{relative}#{member}'))
                except Exception as e:
                    failures.append({'start':start,'end':end,'error':str(e)})
        if final.exists():
            with zipfile.ZipFile(final) as old, zipfile.ZipFile(temp) as new:
                if sorted(old.namelist())!=sorted(new.namelist()) or any(old.read(n)!=new.read(n) for n in old.namelist()):
                    raise RuntimeError('Existing shard conflict; refusing overwrite')
            temp.unlink()
        else:
            temp.replace(final)
        con.executemany('INSERT INTO assemblies VALUES(?,?,?,?,?,?,?,?,?)',records)
        con.executemany('INSERT OR IGNORE INTO refs VALUES(?,?,?,?,?,?)',refs)
        con.commit();done+=len(records)
        print(json.dumps({'extracted_ranges':done,'pending_at_start':len(pending),'seconds':round(time.monotonic()-began,1)}),flush=True)
    decoder.close()
    # Existing A/C roof packs retain exact source/inline metadata in their payloads.
    for v in 'AC':
        base=baseline(v)
        pe_v=pack_module().PE(base['binary']['path'])
        idx=SKELETON/f'_assembly/roof-v1/{v}/index.json.gz'
        if not idx.exists():continue
        for r in load(idx)['ranges']:
            if con.execute('SELECT 1 FROM assemblies WHERE version=? AND start=? AND end=?',(v,r['rva'],r['rva']+r['length'])).fetchone():continue
            source=SKELETON/r['evidence_path'];doc=load(source)
            payload=canonical(doc);sha=hashlib.sha256(payload).hexdigest();relative=f'assembly/{v}-{sha[:24]}.zip';member=sha+'.json'
            validate_existing(doc,pe_v,base['binary']['sha256'])
            with zipfile.ZipFile(RUNTIME/relative,'x',compression=zipfile.ZIP_DEFLATED) as archive:archive.writestr(member,payload)
            con.execute('INSERT INTO assemblies VALUES(?,?,?,?,?,?,?,?,?)',(v,r['rva'],r['rva']+r['length'],relative,member,sha,'existing_verified',r['instruction_count'],0))
        con.commit()
    report={'new_ranges':done,'failures':failures,'seconds':time.monotonic()-began,
            'remaining_B_ranges':con.execute("SELECT count(*) FROM (SELECT DISTINCT start,end FROM spans WHERE version='B' EXCEPT SELECT start,end FROM assemblies WHERE version='B')").fetchone()[0]}
    atomic(RUNTIME/'extraction-latest.json',report)
    con.close()
    return report

def validate_existing(doc,pe,sha):
    if doc['artifact']['sha256'] != sha:
        raise ValueError('Existing assembly identity mismatch')
    for row in doc['instructions']:
        at=row.get('va',row.get('address'))
        if isinstance(at,str):at=int(at,0)
        raw=bytes.fromhex(row.get('bytes',row.get('bytes_hex',row.get('raw_hex',''))))
        if not raw or pe.read_rva(at-pe.image_base,len(raw))!=raw:
            raise ValueError('Existing assembly byte mismatch')

def coverage():
    con=db(True)
    out={}
    for v in 'ABC':
        entries=con.execute('SELECT count(*) FROM functions WHERE version=?',(v,)).fetchone()[0]
        complete=con.execute('''SELECT count(*) FROM functions f WHERE version=? AND EXISTS
          (SELECT 1 FROM spans s WHERE s.version=f.version AND s.entry=f.rva) AND NOT EXISTS
          (SELECT 1 FROM spans s LEFT JOIN assemblies a ON a.version=s.version AND a.start=s.start AND a.end=s.end
           WHERE s.version=f.version AND s.entry=f.rva AND (a.status IS NULL OR a.status NOT IN ('ok','existing_verified')))''',(v,)).fetchone()[0]
        out[v]={'identified_entries':entries,'entries_all_retained_ranges_decoded':complete,
                'ranges':con.execute('SELECT count(*) FROM assemblies WHERE version=?',(v,)).fetchone()[0],
                'undecoded_bytes':con.execute('SELECT coalesce(sum(gaps),0) FROM assemblies WHERE version=?',(v,)).fetchone()[0]}
    con.close()
    return {'versions':out,'semantic_review_complete':False,'scope':'decoded ranges are not complete CFG or semantic verification'}
