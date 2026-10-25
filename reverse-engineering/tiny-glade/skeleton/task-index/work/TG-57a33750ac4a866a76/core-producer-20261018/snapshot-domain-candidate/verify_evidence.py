"""Static original-B identity, complete-range bytes, text, PDB and patch checks.

No machine code is executed. The two intentionally bounded field snippets are
explicitly labelled and are not represented as complete native procedures.
"""
from pathlib import Path
import argparse, difflib, gzip, hashlib, json, re, subprocess, tempfile
import capstone, pefile

WORK = Path(__file__).resolve().parent
ROOT = next(p for p in WORK.parents if (p/'games_test/tiny-glade').is_dir())
BIN = ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne'
PDB = BIN.with_name('tiny_glade.pdb')
IDENTITY = {
    'binary': 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03',
    'pdb': '2d9686b8eaaf5c672c8dbc3aed85fbfeaf2a98a0dc64353191cd1a123a5d8911',
}

def digest(p):
    h=hashlib.sha256()
    with p.open('rb') as f:
        for b in iter(lambda:f.read(1024*1024),b''): h.update(b)
    return h.hexdigest()

def records(path):
    with gzip.open(path,'rt') as f:
        yield from (json.loads(line) for line in f)

def main():
    parser=argparse.ArgumentParser();parser.add_argument('--write',action='store_true')
    args=parser.parse_args()
    assert digest(BIN)==IDENTITY['binary'];assert digest(PDB)==IDENTITY['pdb']
    pe=pefile.PE(str(BIN),fast_load=True)
    imagebase=pe.OPTIONAL_HEADER.ImageBase
    md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64)
    symbols=json.loads((WORK/'symbol-identities.json').read_text())
    by_va={int(v['Address'],16):v for v in symbols}
    procedures={}
    # Source-bound archives include exact procedure identity, aliases and sizes.
    for v in symbols:
        source=v['Symbol'][0].get('FileName','')
        if '/crates/' not in source: continue
        path=ROOT/'games_test/reverse-engineering/tiny-glade/skeleton/B'/('crates/'+source.split('/crates/',1)[1]+'.pdb.jsonl.gz')
        if path.exists():
            for j in records(path):
                if j.get('class')=='procedure': procedures[j['va']]=j
    # The support/corner snippets are dependencies folded into serde helpers.
    module=ROOT/'games_test/reverse-engineering/tiny-glade/skeleton/_pdb/B/modules/01628.jsonl.gz'
    for j in records(module):
        if j.get('class')=='procedure' and j.get('section')==1:
            procedures[imagebase+0x1000+j['section_offset']]=j
    ranges=[];instruction_count=0;byte_count=0
    for raw in sorted(WORK.glob('*.bin')):
        text=raw.with_suffix('.asm')
        expected=text.read_text()
        start=int(expected.split(':',1)[0],16)
        data=raw.read_bytes()
        assert pe.get_data(start-imagebase,len(data))==data, raw.name
        ins=list(md.disasm(data,start))
        assert sum(i.size for i in ins)==len(data), raw.name
        rendered=''.join(f'{i.address:x}: {i.bytes.hex()}  {i.mnemonic} {i.op_str}\n' for i in ins)
        assert [line.rstrip() for line in rendered.splitlines()]==[line.rstrip() for line in expected.splitlines()], raw.name
        bounded='bounded' in raw.stem
        proc=procedures.get(start)
        if not bounded:
            assert proc is not None, ('missing source-bound PDB proc',raw.name,hex(start))
            assert proc['code_size']==len(data),(raw.name,proc['code_size'],len(data))
        row={'name':raw.stem,'start':hex(start),'length':len(data),
             'sha256':digest(raw),'instructions':len(ins),'complete_procedure':not bounded,
             'pdb':proc if proc else next((p for a,p in procedures.items()
                 if a<=start<a+p['code_size']),None)}
        ranges.append(row);instruction_count+=len(ins);byte_count+=len(data)
    report={'identity':IDENTITY,'ranges':ranges,'bytes':byte_count,
            'instructions':instruction_count,'native_execution':False}
    names=json.loads((WORK/'clutter-mesh-string-bindings.json').read_text())
    table=int(names['table_va'],16)
    for i,row in enumerate(names['bindings']):
        assert row['index']==i
        offset=int.from_bytes(pe.get_data(table-imagebase+4*i,4),'little',signed=True)
        assert table+offset==int(row['case_va'],16)
        assert pe.get_data(int(row['string_va'],16)-imagebase,row['length']).decode()==row['value']
    for row in json.loads((WORK/'field-string-bindings.json').read_text()):
        assert pe.get_data(int(row['va'],16)-imagebase,row['length']).decode()==row['value']
        lea=next(md.disasm(pe.get_data(int(row['lea'],16)-imagebase,15),int(row['lea'],16)))
        length=next(md.disasm(pe.get_data(int(row['length_site'],16)-imagebase,15),int(row['length_site'],16)))
        assert lea.mnemonic=='lea' and length.mnemonic=='mov'
    callmap=json.loads((WORK/'schema-call-map.json').read_text())
    capture={line.split(':',1)[0]:line for line in (WORK/'capture-original.asm').read_text().splitlines()}
    for row in callmap['metadata']:
        line=capture[row['serialize_or_insert_call'][2:]]
        assert 'call ' in line
        if row['typed_helper']:assert line.endswith('call '+row['typed_helper'])
    for name in ['Snapshot','Terrain']:
        row=callmap['chunks'][name]
        assert pe.get_data(int(row['tag_va'],16)-imagebase,row['length']).decode()==name
    for row in json.loads((WORK/'dependency-inputs.json').read_text()):
        p=Path(row.get('path',row.get('manifest')))
        assert digest(p)==row['sha256'] and p.stat().st_size==row['size']
    if args.write:
        (WORK/'native-ranges.json').write_text(json.dumps(report,indent=2)+'\n')
    else:
        assert json.loads((WORK/'native-ranges.json').read_text())==report
    # Frozen candidates are authoritative; the scaffold script is not a build.
    owned='crates/country-core/src/systems/save/snapshot.rs'
    shared={
        'stairs_state.rs':'crates/country-core/src/resources/stairs/stairs_state.rs',
        'Cargo.toml':'crates/country-core/Cargo.toml',
    }
    def patch(name,path):
        before=(WORK/'base'/name).read_text().splitlines(keepends=True)
        after=(WORK/'candidate'/name).read_text().splitlines(keepends=True)
        return ''.join(difflib.unified_diff(before,after,fromfile='a/'+path,tofile='b/'+path))
    patches={'candidate.patch':patch('snapshot.rs',owned),
             'integration-owner.patch':''.join(patch(n,p) for n,p in shared.items())}
    for name,data in patches.items():
        if args.write:(WORK/name).write_text(data)
        else: assert (WORK/name).read_text()==data,name
    with tempfile.TemporaryDirectory(prefix='snapshot-domain-patch-check-') as folder:
        temp=Path(folder)
        for name,path in [('snapshot.rs',owned),*shared.items()]:
            dest=temp/path;dest.parent.mkdir(parents=True,exist_ok=True)
            dest.write_bytes((WORK/'base'/name).read_bytes())
        patchfiles=[str(WORK/name) for name in patches]
        subprocess.run(['git','apply','--check',*patchfiles],cwd=temp,check=True)
        subprocess.run(['git','apply',*patchfiles],cwd=temp,check=True)
        for name,path in [('snapshot.rs',owned),*shared.items()]:
            assert (temp/path).read_bytes()==(WORK/'candidate'/name).read_bytes()
    print(json.dumps({'ok':True,'ranges':len(ranges),'bytes':byte_count,
                      'instructions':instruction_count,'native_execution':False}))

if __name__=='__main__':main()
