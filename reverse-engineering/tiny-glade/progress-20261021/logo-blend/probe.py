from pathlib import Path
import hashlib, json, pefile, capstone
p=Path('D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe')
d=p.read_bytes()
assert hashlib.sha256(d).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=pefile.PE(data=d,fast_load=True)
cs=capstone.Cs(capstone.CS_ARCH_X86,capstone.CS_MODE_64)
out=Path('D:/game/reverse-engineering/tiny-glade/progress-20261021/logo-blend')
out.mkdir(exist_ok=True)
records=[]
for name,addr,size in [('env37',0x1409b1b00,903),('env38',0x1409b9080,1223),('ui-pass',0x140837c40,929),('loading-ui-pass',0x140837ff0,929),('raster-default',0x140be6750,6)]:
    b=pe.get_data(addr-pe.OPTIONAL_HEADER.ImageBase,size)
    s='\n'.join(f'{i.address:#x}: {i.mnemonic} {i.op_str}' for i in cs.disasm(b,addr))+'\n'
    (out/(name+'.asm')).write_text(s,encoding='utf-8')
    records.append({'name':name,'address':hex(addr),'pdb_bytes':size,'body_sha256':hashlib.sha256(b).hexdigest(),'assembly':name+'.asm','assembly_sha256':hashlib.sha256(s.encode()).hexdigest()})
(out/'manifest.json').write_text(json.dumps({'original':str(p),'original_sha256':hashlib.sha256(d).hexdigest(),'pdb_ledger':'evidence/country-core-audit/data/procs.tsv','evidence':'bounded static instruction decoding; not original game runtime','functions':records},indent=2)+'\n',encoding='utf-8')
(out/'probe.py').write_text(Path(__file__).read_text(encoding='utf-8'),encoding='utf-8')
print(out)
