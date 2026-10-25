from pathlib import Path
import struct,json,hashlib
p=Path('ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes();pe=struct.unpack_from('<I',p,60)[0];sh=pe+24+struct.unpack_from('<H',p,pe+20)[0];sections=[struct.unpack_from('<4I',p,sh+i*40+8)for i in range(struct.unpack_from('<H',p,pe+6)[0])]
rva=0xa093e0;size=8031
for vs,vr,rs,ro in sections:
 if vr<=rva<vr+rs:raw=p[ro+rva-vr:ro+rva-vr+size];break
job=[{'tool':'prepare_functions','args':{'program':'tiny-glade.exe','targets':[{'address':'0x140a093e0','size':size,'bytes_sha256':hashlib.sha256(raw).hexdigest(),'range_basis':'PDB module537 S_GPROC32 code_size8031; procs.tsv authoritative author range, original RNE matched patched'}]}},{'tool':'get_functions','args':{'program':'tiny-glade.exe','functions':'0x140a093e0','fields':'signature,decompiled_code,callees'}}]
Path('reverse-engineering/tiny-glade/evidence/round12/wall-gpu/sheep-startup-job.json').write_text(json.dumps(job))
