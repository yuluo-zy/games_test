"""Fresh exhaustive B IPI/PROC/INLINESITE and C13 roof-support search."""
from pathlib import Path
import sys, json, struct, hashlib
sys.path.insert(0, str(Path(__file__).parents[2]/'utils-deep-followup-2026-10-14'))
from pdb_reader import Pdb,z
OUT=Path(__file__).parent
p=Pdb('D:/game/ljxsj_92385/Tiny Glade/tiny_glade.pdb')
ids={i:(k,b) for i,k,b in p.types(4)}
def method(i):
    k,b=ids.get(i,(0,b''))
    if k!=0x1601:return ''
    parent=struct.unpack_from('<I',b)[0];sk,sb=ids.get(parent,(0,b''))
    return (z(sb[4:])+'::' if sk==0x1605 else '')+z(b[8:])
def wanted(s):return any(k in s for k in ['country_core::systems::ui::hover_intersection','country_core::systems::ui::ui_update'])
selected={i:method(i) for i in ids if wanted(method(i))}
procs=[];sites=[];line_rows=[];checks_rows=[];declarations=[]
names=p.stream(4886);nsz=struct.unpack_from('<I',names,8)[0];strings=names[12:12+nsz]
db=p.stream(3);end=64+struct.unpack_from('<I',db,24)[0];pos=64;mi=0;symbol_count=0
while pos<end:
    ss,sym,c11,c13=struct.unpack_from('<HIII',db,pos+34)
    a=pos+64;b=db.index(b'\0',a);c=db.index(b'\0',b+1);module=z(db[a:b]);pos=(c+4)&~3
    if ss==0xffff:mi+=1;continue
    s=p.stream(ss);syms={}
    for q,k,v in p.symbols(ss,sym):syms[q]=(k,v)
    symbol_count+=len(syms)
    for q,(k,v) in syms.items():
        if k in (0x1110,0x110f,0x1147,0x1146) and wanted(z(v[35:])):
            off,sec=struct.unpack_from('<IH',v,28)
            procs.append(dict(module=mi,record=q,name=z(v[35:]),section=sec,offset=off,va=hex(0x140001000+off) if sec==1 else None,size=struct.unpack_from('<I',v,12)[0]))
        if k in (0x114d,0x115d):
            parent,_,inlinee=struct.unpack_from('<III',v)
            if inlinee in selected:
                ancestor=parent
                while ancestor in syms and syms[ancestor][0] not in (0x1110,0x110f,0x1147,0x1146):ancestor=struct.unpack_from('<I',syms[ancestor][1])[0]
                host=syms.get(ancestor,(0,b''))[1]
                sites.append(dict(module=mi,inlinee=hex(inlinee),name=selected[inlinee],host=z(host[35:]) if host else None,annotation_hex=v[12:].hex()))
    data=s[sym+c11:sym+c11+c13];q=0;subs=[];checks={}
    while q<len(data):
        kind,n=struct.unpack_from('<II',data,q);subs.append((kind,data[q+8:q+8+n]));q=(q+8+n+3)&~3
    for kind,body in subs:
        if kind!=0xf4:continue
        q=0
        while q<len(body):
            off,n,ck=struct.unpack_from('<IBB',body,q);path=z(strings[off:]);checks[q]=dict(path=path,checksum=body[q+6:q+6+n].hex())
            if ('/systems/ui/hover_intersection.rs' in path or '/systems/ui/ui_update.rs' in path):checks_rows.append(dict(module=mi,**checks[q]))
            q=(q+6+n+3)&~3
    for kind,body in subs:
        if kind==0xf6:
            signature=struct.unpack_from('<I',body)[0];q=4
            while q<len(body):
                identity,fi,line=struct.unpack_from('<III',body,q);q+=12
                if signature:n=struct.unpack_from('<I',body,q)[0];q+=4+4*n
                info=checks.get(fi,{})
                if any(info.get('path','').endswith('/'+f) for f in ['hover_intersection.rs','ui_update.rs']) or identity in selected:declarations.append(dict(module=mi,identity=hex(identity),name=method(identity),line=line,**info))
        if kind!=0xf2:continue
        off,section,flags,length=struct.unpack_from('<IHHI',body);q=12
        while q<len(body):
            fi,n,sz=struct.unpack_from('<III',body,q);info=checks.get(fi,{})
            if any(info.get('path','').endswith('/'+f) for f in ['hover_intersection.rs','ui_update.rs']):
                entries=[dict(offset=off+struct.unpack_from('<II',body,q+12+j*8)[0],line=struct.unpack_from('<II',body,q+12+j*8)[1]&0xffffff) for j in range(n)]
                line_rows.append(dict(module=mi,section=section,range_offset=off,range_size=length,entries=entries,**info))
            q+=sz
    mi+=1
r=dict(pdb_sha256=hashlib.sha256(p.raw).hexdigest(),modules_scanned=mi,symbols_scanned=symbol_count,ipi_records=len(ids),selected_ipi=[dict(id=hex(i),name=n) for i,n in selected.items()],procedures=procs,inline_sites=sites,source_checksums=checks_rows,source_lines=line_rows,inline_declarations=declarations,limitation='Only compiled B CodeView evidence; missing records do not prove author source dead or runtime unreachable.')
(OUT/'pdb-ui-picker.json').write_text(json.dumps(r,indent=2),encoding='utf8')
print(json.dumps({k:v if not isinstance(v,list) else len(v) for k,v in r.items()},indent=2))
