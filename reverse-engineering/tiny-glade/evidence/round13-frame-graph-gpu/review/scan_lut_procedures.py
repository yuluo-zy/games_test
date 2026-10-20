import pathlib,sys,struct,json
root=pathlib.Path('D:/game');sys.path.insert(0,str(root/'reverse-engineering/tiny-glade/evidence/utils-deep-followup-2026-10-14'));from pdb_reader import Pdb,z
p=Pdb(root/'ljxsj_92385/Tiny Glade/tiny_glade.pdb');targets=[0x140184627,0x141394957,0x14154a756];rows=[]
for mi,name,ss,sb in p.modules():
 if ss==65535:continue
 for ro,k,b in p.symbols(ss,sb):
  if k in [0x110f,0x1110,0x1146,0x1147] and len(b)>35:
   size=struct.unpack_from('<I',b,12)[0];off,sec=struct.unpack_from('<IH',b,28);va=0x140001000+off
   if sec==1 and any(va<=t<va+size for t in targets):rows.append({'module':mi,'module_name':name,'va':hex(va),'size':size,'type':hex(struct.unpack_from('<I',b,24)[0]),'name':z(b[35:])})
out=root/'reverse-engineering/tiny-glade/evidence/round13-frame-graph-gpu/review/custom-lut-pdb-procedures.json';out.write_text(json.dumps(rows,indent=2));print(json.dumps(rows,indent=2))
