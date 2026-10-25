"""Fresh bounded tg-text IPI/PROC inventory; PDB placeholder types are not ABI."""
import hashlib, json, mmap, struct as s
from pathlib import Path
OUT=Path(__file__).parent
target=Path('D:/game/ljxsj_92385/Tiny Glade')
with open(target/'tiny_glade.pdb','rb') as fp:
    m=mmap.mmap(fp.fileno(),0,access=mmap.ACCESS_READ)
    bs,_,_,ndir,_,bmap=s.unpack_from('<6I',m,32)
    blocks=s.unpack_from(f'<{(ndir+bs-1)//bs}I',m,bmap*bs)
    directory=b''.join(m[b*bs:(b+1)*bs] for b in blocks)[:ndir]
    n=s.unpack_from('<I',directory)[0]; sizes=s.unpack_from(f'<{n}I',directory,4)
    p=4+4*n; streams=[]
    for size in sizes:
        count=((0 if size==0xffffffff else size)+bs-1)//bs
        streams.append(s.unpack_from(f'<{count}I',directory,p));p+=count*4
    def stream(i):return b''.join(m[b*bs:(b+1)*bs] for b in streams[i])[:sizes[i]]
    def z(v):return v.split(b'\0',1)[0].decode('utf8','replace')
    ipi=stream(4);hs,first,end=s.unpack_from('<3I',ipi,4);p=hs;records={}
    for index in range(first,end):
        size,kind=s.unpack_from('<HH',ipi,p);records[index]=(kind,ipi[p+4:p+size+2]);p+=size+2
    scopes=[];ids=[]
    for index,(kind,b) in records.items():
        if kind==0x1605 and 'tg_text' in z(b[4:]):scopes.append({'index':hex(index),'scope':z(b[4:])})
        if kind==0x1601:
            parent,typ=s.unpack_from('<II',b);pk,pb=records.get(parent,(0,b''));namespace=z(pb[4:]) if pk==0x1605 else ''
            if 'tg_text' in namespace:ids.append({'ipi':hex(index),'scope':namespace,'name':z(b[8:]),'type':hex(typ)})
    dbi=stream(3);modsz=s.unpack_from('<i',dbi,24)[0];mods=[];p=64
    while p<64+modsz:
        ss,sb=s.unpack_from('<HI',dbi,p+34);q=p+64;n1=dbi.index(b'\0',q);n2=dbi.index(b'\0',n1+1)
        mods.append((z(dbi[q:n1]),ss,sb));p=(n2+4)&~3
    procs=[];conversion_hosts=[]
    for mi,(mod,ss,sb) in enumerate(mods):
        if ss==0xffff or not sb:continue
        data=stream(ss);q=4;symbols={}
        while q<sb:
            ln,kind=s.unpack_from('<HH',data,q);b=data[q+4:q+ln+2]
            symbols[q]=(kind,b)
            if kind in (0x1110,0x110f,0x1147,0x1146):
                name=z(b[35:])
                if 'tg_text' in name:
                    size=s.unpack_from('<I',b,12)[0];off,seg=s.unpack_from('<IH',b,28)
                    procs.append({'module':mi,'name':name,'address':hex(0x140001000+off) if seg==1 else None,'size':size,'symbol_kind':hex(kind)})
            q+=ln+2
        for q,(kind,b) in symbols.items():
            if kind not in (0x114d,0x115d):continue
            parent,_,ipi=s.unpack_from('<III',b)
            if ipi not in (0x86aa,0x86ac):continue
            ancestor=parent
            while ancestor in symbols and symbols[ancestor][0] not in (0x1110,0x110f,0x1147,0x1146):ancestor=s.unpack_from('<I',symbols[ancestor][1])[0]
            if ancestor not in symbols:continue
            pb=symbols[ancestor][1];off,seg=s.unpack_from('<IH',pb,28)
            conversion_hosts.append({'ipi':hex(ipi),'module':mi,'name':z(pb[35:]),'address':hex(0x140001000+off) if seg==1 else None,'size':s.unpack_from('<I',pb,12)[0],'annotations':b[12:].hex()})
    report={'pdb_sha256':hashlib.sha256(m).hexdigest(),'modules_scanned':len(mods),'scopes':scopes,'functions':ids,'procedures':procs,'conversion_hosts':conversion_hosts,'limitation':'Complete IPI scope and PROC name inventory only; no full Rust field types; no dead-source completeness claim'}
    (OUT/'pdb-inventory.json').write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
    print(json.dumps({'scopes':len(scopes),'ipi_functions':len(ids),'procedures':len(procs),'conversion_hosts':conversion_hosts,'metrics_scopes':[r for r in scopes if r['scope'].startswith('tg_text') and 'etric' in r['scope']],'new_metrics':[r for r in procs if 'TextFragment::new' in r['name'] or 'TextFragment::set_metrics' in r['name'] or 'tg_text::impl$6::from' in r['name']]}))
