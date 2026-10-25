from pathlib import Path
import struct,re,json,gzip,bisect,pefile,capstone
ROOT=Path('/Users/yuluo/Desktop/game');P=Path(__file__).resolve().parent;F=ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne';raw=F.read_bytes();pe=pefile.PE(str(F),fast_load=True);A=json.load(gzip.open(ROOT/'games_test/reverse-engineering/tiny-glade/skeleton/_pdb/B/enrichment/address-aliases.json.gz','rt'));keys=sorted(map(int,A));name=b'country_core::startup::startup_terrain::startup_terrain';pos=raw.find(name);assert pos>=0;targets=[]
while pos>=0:
 targets.append(pe.get_rva_from_offset(pos)+0x140000000);pos=raw.find(name,pos+len(name))
target=targets[-1]
sec=next(s for s in pe.sections if s.Name.startswith(b'.text'));text=sec.get_data();base=0x140000000+sec.VirtualAddress;refs=[]
for m in re.finditer(rb'[\x48\x4c]\x8d[\x05\x0d\x15\x1d\x25\x2d\x35\x3d]',text):
 k=m.start()
 if k+7<=len(text) and base+k+7+struct.unpack_from('<i',text,k+3)[0] in targets:
  rva=base+k-0x140000000;prev=keys[bisect.bisect_right(keys,rva)-1];refs.append({'site':hex(base+k),'bytes':text[k:k+7].hex(),'enclosing_alias_rva':hex(prev),'aliases':A[str(prev)]})
(P/'startup-terrain-name-xrefs.json').write_text(json.dumps({'name':name.decode(),'name_va':hex(target),'refs':refs},indent=2)+'\n')
for r in refs:print(r['site'],r['enclosing_alias_rva'],[(x['procedure_id'],x['name'][:160]) for x in r['aliases']])
