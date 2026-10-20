"""Bounded native identity, event TypeId references, and forest lifecycle oracle.

Only bytes, PDB rows and disassembly; no emulator/library reconstruction.
"""
from pathlib import Path
import bisect,hashlib,json,struct,subprocess
HERE=Path(__file__).resolve().parent;ROOT=Path('D:/game')
exe=ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne';blob=exe.read_bytes()
sha=hashlib.sha256(blob).hexdigest()
assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=struct.unpack_from('<I',blob,0x3c)[0];sc=struct.unpack_from('<H',blob,pe+6)[0]
off=pe+24+struct.unpack_from('<H',blob,pe+20)[0]
sections=[struct.unpack_from('<IIII',blob,off+i*40+8) for i in range(sc)]
def read(va,n):
    for vs,start,rs,raw in sections:
        rva=va-0x140000000
        if start<=rva and rva+n<=start+rs:return blob[raw+rva-start:raw+rva-start+n]
    raise ValueError(hex(va))
procs={};rows=[]
for line in (ROOT/'reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv').open(encoding='utf8'):
    f=line.rstrip().split('\t');a=int(f[0],16);n=int(f[1]);name=f[-1]
    procs.setdefault(a,[]).append((n,name));rows.append((a,n,name))
rows.sort();starts=[r[0] for r in rows]
addresses=[0x1405734e0,0x140578b80,0x141343510,0x141343b80,0x1408a82f0,
    0x1408a8060,0x1408a8140,0x140a78aa0,0x141bf16c0,0x1404d9580,0x140b19d20,0x140b19d30,
    0x14059d450,0x1422785d0,0x142278a00,0x142278cf0,0x142278f00,
    0x14227a0d0,0x140aa18f0,0x140aa1f00]
result={'target':str(exe),'original_sha256':sha,'functions':{},'event_typeids':{},'slices':{}}
with (HERE/'reveal-forest-original.asm').open('w',encoding='utf8') as out:
    for a in addresses:
        n=procs[a][0][0];raw=read(a,n)
        result['functions'][hex(a)]={'length':n,'sha256':hashlib.sha256(raw).hexdigest(),
            'raw_hex':raw.hex(),'pdb_aliases':procs[a]}
        out.write('\n'+repr(procs[a])+'\n')
        out.write(subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d',
            f'--start-address={hex(a)}',f'--stop-address={hex(a+n)}',str(exe)],
            check=True,capture_output=True).stdout.decode('utf8'))
for name,words in [('Started',[0x76eef710ca5ee797,0x804a5169b0f39613]),
    ('Finished',[0xd8f93d064ea9d61c,0x6447617066d9832f])]:
    refs=[]
    for word in words:
        needle=struct.pack('<Q',word);at=0
        while True:
            at=blob.find(needle,at)
            if at<0:break
            for vs,start,rs,raw in sections:
                if raw<=at<raw+rs:
                    va=0x140000000+start+at-raw
                    i=bisect.bisect_right(starts,va)-1;owners=[]
                    while i>=0 and va-rows[i][0]<70000:
                        if rows[i][0]<=va<rows[i][0]+rows[i][1]:owners.append(rows[i])
                        i-=1
                    refs.append({'word':hex(word),'address':hex(va),'owners':owners})
            at+=8
    result['event_typeids'][name]={'words':[hex(x) for x in words],'all_native_refs':refs}
for name,(a,n) in {'handoff-started-write':(0x1404da0ae,0xb0),
    'vermeer-finished-write':(0x1408a89d6,0xc4),
    'run-game-prepared-finished-write':(0x1405a42b0,0xd4),
    'forest-init-processed-false':(0x140a790a0,0x30),
    'forest-manage-processed-read':(0x141bf1f10,0x40),
    'forest-manage-processed-true':(0x141bf2570,0x40)}.items():
    raw=read(a,n);result['slices'][name]={'address':hex(a),'length':n,'sha256':hashlib.sha256(raw).hexdigest(),'raw_hex':raw.hex()}
(HERE/'reveal-forest-original.json').write_text(json.dumps(result,indent=2),encoding='utf8')
print(json.dumps({'original_sha256':sha,'full_functions':len(addresses),'slice_count':len(result['slices']),
    'typeid_ref_counts':{k:len(v['all_native_refs']) for k,v in result['event_typeids'].items()}},indent=2))
