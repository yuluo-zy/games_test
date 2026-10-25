from pathlib import Path
import struct,re,json,gzip,bisect,pefile,capstone
ROOT=Path('/Users/yuluo/Desktop/game');P=Path(__file__).resolve().parent;F=ROOT/'ljxsj_92385/Tiny Glade/tiny-glade.rne';pe=pefile.PE(str(F),fast_load=True);BASE=0x140000000;PD=ROOT/'games_test/reverse-engineering/tiny-glade/skeleton/_pdb/B';A=json.load(gzip.open(PD/'enrichment/address-aliases.json.gz','rt'));keys=sorted(map(int,A));md=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64)
def extract(va):
 alias=A[str(va-BASE)][0];mod,off=map(int,alias['procedure_id'].split(':'));rec=next(v for l in gzip.open(PD/'modules'/f'{mod:05d}.jsonl.gz','rt') if (v:=json.loads(l)).get('class')=='procedure' and v.get('record_offset')==off);b=pe.get_data(va-BASE,rec['code_size']);(P/f'{va:x}.bin').write_bytes(b);(P/f'{va:x}.pdb.json').write_text(json.dumps(rec,indent=2));ins=list(md.disasm(b,va));(P/f'{va:x}.asm').write_text('\n'.join(f'{i.address:#x} {i.bytes.hex()} {i.mnemonic} {i.op_str}' for i in ins)+'\n');return rec
r=extract(0x140ee2560);s=next(x for x in pe.sections if x.Name.startswith(b'.text'));text=s.get_data();base=BASE+s.VirtualAddress;out=[]
for k in (m.start() for m in re.finditer(b'[\xe8\xe9]',text)):
 if k+5<=len(text) and base+k+5+struct.unpack_from('<i',text,k+1)[0]==0x140ee2560:
  prev=keys[bisect.bisect_right(keys,base+k-BASE)-1];out.append({'site':hex(base+k),'bytes':text[k:k+5].hex(),'enclosing_alias_rva':hex(prev),'aliases':A[str(prev)]});extract(BASE+prev)
(P/'chain-call-xrefs.json').write_text(json.dumps(out,indent=2)+'\n')
print('helper bytes',r['code_size'])
for x in out:print(x['site'],x['enclosing_alias_rva'],[(a['procedure_id'],a['name'][:180]) for a in x['aliases']])
