"""Bounded fresh B PDB scope/inline search; no Ghidra database mutations."""
from pathlib import Path
import hashlib, json, mmap, struct, sys
sys.path.insert(0, 'D:/game/tools/reverse-runtime/utils-gap-20261014/python-deps')
import pefile
from capstone import Cs, CS_ARCH_X86, CS_MODE_64

OUT = Path(__file__).parent
ROOT = OUT.parents[3]
TARGET = ROOT / 'ljxsj_92385/Tiny Glade'
raw = (TARGET / 'tiny-glade.rne').read_bytes()
assert hashlib.sha256(raw).hexdigest() == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe = pefile.PE(data=raw, fast_load=True)
base = pe.OPTIONAL_HEADER.ImageBase
with open(TARGET / 'tiny_glade.pdb', 'rb') as fp:
    m = mmap.mmap(fp.fileno(), 0, access=mmap.ACCESS_READ)
    assert m[:24] == b'Microsoft C/C++ MSF 7.00'
    bs, _, _, ndir, _, bmap = struct.unpack_from('<6I', m, 32)
    db = struct.unpack_from(f'<{(ndir+bs-1)//bs}I', m, bmap*bs)
    d = b''.join(m[b*bs:(b+1)*bs] for b in db)[:ndir]
    ns = struct.unpack_from('<I', d)[0]
    sizes = struct.unpack_from(f'<{ns}I', d, 4)
    p = 4+4*ns
    blocks = []
    for size in sizes:
        n = (max(0, size if size != 0xffffffff else 0)+bs-1)//bs
        blocks.append(struct.unpack_from(f'<{n}I', d, p)); p += 4*n
    def stream(i):
        return b''.join(m[b*bs:(b+1)*bs] for b in blocks[i])[:sizes[i]]
    ipi = stream(4)
    hs, first, end = struct.unpack_from('<3I', ipi, 4)
    records = {}; p = hs
    for index in range(first, end):
        size, kind = struct.unpack_from('<HH', ipi, p)
        records[index] = (kind, ipi[p+4:p+2+size]); p += size+2
    def z(b): return b.split(b'\0',1)[0].decode('utf8','replace')
    def scope(index):
        if not index: return ''
        kind, b = records.get(index,(0,b''))
        if kind == 0x1605: return z(b[4:])
        return f'ipi:{index:#x}'
    wanted = {}
    for index, (kind,b) in records.items():
        if kind != 0x1601: continue
        parent, typ = struct.unpack_from('<II', b)
        name, namespace = z(b[8:]), scope(parent)
        if any(s in namespace for s in ['utils::natural_cubic::Spline2D', 'utils::wall_space::WallSpaceCoord', 'utils::index_condenser::', 'utils::swizzlers::']):
            wanted[index] = {'ipi':hex(index), 'scope':namespace, 'name':name, 'type':hex(typ), 'hosts':[]}
    dbi = stream(3); modsz = struct.unpack_from('<i', dbi,24)[0]
    mods = []; p = 64
    while p < 64+modsz:
        ss,sb = struct.unpack_from('<HI',dbi,p+34)
        q=p+64; n1=dbi.index(b'\0',q); n2=dbi.index(b'\0',n1+1)
        mods.append((z(dbi[q:n1]),z(dbi[n1+1:n2]),ss,sb)); p=(n2+4)&~3
    procs=[]
    for mi,(module,obj,ss,sb) in enumerate(mods):
        if ss == 0xffff or not sb: continue
        s=stream(ss); q=4; symbols={}
        while q < sb:
            ln,kind=struct.unpack_from('<HH',s,q)
            symbols[q]=(kind,s[q+4:q+ln+2]); q+=ln+2
        for record,(kind,b) in symbols.items():
            if kind in (0x1110,0x110f,0x1147,0x1146):
                size=struct.unpack_from('<I',b,12)[0]; off,seg=struct.unpack_from('<IH',b,28)
                name=z(b[35:])
                if seg and seg<=len(pe.sections):
                    a=base+pe.sections[seg-1].VirtualAddress+off
                    if any(x in name for x in ['Spline2D','WallSpaceCoord','index_condenser','as_vec2','extend_no_data','create_instances']):
                        procs.append({'module':mi,'record':record,'name':name,'address':hex(a),'size':size})
            if kind not in (0x114d,0x115d): continue
            parent,_,inlinee=struct.unpack_from('<III',b)
            if inlinee not in wanted: continue
            chain=[record]; ancestor=parent
            while ancestor in symbols and symbols[ancestor][0] not in (0x1110,0x110f,0x1147,0x1146):
                chain.append(ancestor); ancestor=struct.unpack_from('<I',symbols[ancestor][1])[0]
            if ancestor not in symbols: continue
            pb=symbols[ancestor][1]; off,seg=struct.unpack_from('<IH',pb,28)
            a=base+pe.sections[seg-1].VirtualAddress+off
            wanted[inlinee]['hosts'].append({'module':mi,'record':record,'procedure_record':ancestor,'procedure_name':z(pb[35:]),'address':hex(a),'size':struct.unpack_from('<I',pb,12)[0],'annotation_hex':b[12:].hex()})
    report={'original_sha256':hashlib.sha256(raw).hexdigest(),'pdb_sha256':hashlib.sha256(m).hexdigest(),'modules_scanned':len(mods),'ipi_records':end-first,'interfaces':list(wanted.values()),'procedures':procs,'limitation':'Absence is bounded to IPI LF_FUNC_ID scopes and compiled PROC/INLINESITE records; it does not prove unused/dead source.'}
    (OUT/'pdb-scan.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n',encoding='utf8')
    for v in wanted.values():
        print(v['ipi'],v['scope'],v['name'],'hosts',len(v['hosts']))
    print('PROCS',json.dumps(procs,ensure_ascii=False))
md=Cs(CS_ARCH_X86,CS_MODE_64)
for a,size in [(0x140992280,2200),(0x14156d0d0,2600),(0x140a19f50,561)]:
    data=pe.get_data(a-base,size)
    (OUT/f'{a:#x}.asm.txt').write_text('\n'.join(f'{i.address:#x}: {i.mnemonic} {i.op_str}' for i in md.disasm(data,a))+'\n',encoding='utf8')
