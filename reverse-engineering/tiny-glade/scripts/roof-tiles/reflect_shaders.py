from pathlib import Path
import struct,json,hashlib
R=Path('D:/game/reverse-engineering/tiny-glade');O=R/'evidence/roof-tiles';S=Path('D:/game/ljxsj_92385/Tiny Glade/compiled-shaders')
records=[]
def string(ws):return struct.pack('<'+'I'*len(ws),*ws).split(b'\0')[0].decode('utf-8',errors='replace')
for p in sorted(S.glob('*roof*.bin')):
 b=p.read_bytes();cursor=0;stages=[]
 while True:
  start=b.find(b'\x03\x02\x23\x07',cursor)
  if start<0:break
  n=struct.unpack_from('<I',b,start-4)[0];payload=b[start:start+n*4];assert len(payload)==n*4
  ws=struct.unpack('<'+'I'*n,payload);names={};members={};offsets={};strides={};structs={};arrays={};entries=[];ops=[];i=5
  while i<n:
   count=ws[i]>>16;op=ws[i]&65535;assert count>0 and i+count<=n;args=ws[i+1:i+count];ops.append(op)
   if op==5:names[args[0]]=string(args[1:])
   if op==6:members.setdefault(args[0],{})[args[1]]=string(args[2:])
   if op==72 and args[2]==35:offsets.setdefault(args[0],{})[args[1]]=args[3]
   if op==71 and args[1]==6:strides[args[0]]=args[2]
   if op==30:structs[args[0]]=list(args[1:])
   if op in [28,29]:arrays[args[0]]=args[1]
   if op==15:entries.append(dict(model=args[0],entry=string(args[2:])))
   i+=count
  relevant=[]
  for id,name in names.items():
   if 'roof' in name.lower() or 'tile' in name.lower():
    relevant.append(dict(id=id,name=name,members=[dict(index=k,name=v,offset=offsets.get(id,{}).get(k)) for k,v in members.get(id,{}).items()],array_stride=strides.get(id),arrays_of_type=[dict(array_id=a,stride=strides.get(a)) for a,t in arrays.items() if t==id],member_types=structs.get(id)))
  stages.append(dict(payload_offset=start,payload_words=n,payload_sha256=hashlib.sha256(payload).hexdigest(),entries=entries,instruction_count=len(ops),relevant_names=relevant,structs_with_roof_members=[dict(id=k,name=names.get(k),members=[dict(index=i,name=name,offset=offsets.get(k,{}).get(i)) for i,name in v.items()]) for k,v in members.items() if any('roof' in s.lower() or 'tile' in s.lower() for s in v.values())]))
  cursor=start+n*4
 records.append(dict(file=p.name,sha256=hashlib.sha256(b).hexdigest(),stages=stages))
(O/'shader-interfaces.json').write_text(json.dumps(dict(files=records,scope='Original SPIR-V debug names, struct field offset decorations and array strides; instruction streams parsed to payload boundary',limitations=['Reflection establishes layout and carrier identity, not full rendering or compute behavior recovery','Debug spelling does not prove CPU field correspondence unless offsets/type match']),indent=2),encoding='utf-8');print(json.dumps(records,indent=2)[:1000])
