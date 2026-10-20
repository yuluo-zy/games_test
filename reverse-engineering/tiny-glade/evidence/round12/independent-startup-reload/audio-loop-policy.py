"""Independent original-byte policy audit; no library Rust implementation.
The generic consumers are bounded only to trace whether the game-produced
range discards its first AudioInstance. No author oracle/result is imported.
"""
from pathlib import Path
import hashlib,struct,json,subprocess
R=Path('D:/game');O=Path(__file__).resolve().parent/'audio-loop-policy';O.mkdir(exist_ok=True)
exe=R/'ljxsj_92385/Tiny Glade/tiny-glade.rne';raw=exe.read_bytes()
sha=hashlib.sha256(raw).hexdigest();assert sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe=struct.unpack_from('<I',raw,60)[0];base=struct.unpack_from('<Q',raw,pe+48)[0];opt=struct.unpack_from('<H',raw,pe+20)[0]
sections=[struct.unpack_from('<IIII',raw,pe+24+opt+i*40+8) for i in range(struct.unpack_from('<H',raw,pe+6)[0])]
def read(a,n):
 for _,rva,size,off in sections:
  if rva<=a-base and a-base+n<=rva+size:return raw[off+a-base-rva:off+a-base-rva+n]
 raise ValueError(hex(a))
ranges=[]
for a,n in [(0x140a59220,459),(0x1412fafb0,711),(0x1414792b0,600)]:
 p=O/f'{a:x}.asm';p.write_bytes(subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d',f'--start-address={a:#x}',f'--stop-address={a+n:#x}',str(exe)],check=True,stdout=subprocess.PIPE).stdout)
 ranges.append({'va':hex(a),'size':n,'sha256':hashlib.sha256(read(a,n)).hexdigest()})
result={'original_sha256':sha,'mode':'static original CFG, independently read; not executed', 'ranges':ranges,
'observed':['140a59243/247 descriptor count incremented; range start0@25e/end count+1@266',
'1412fb06e..082 original collection writes first item, 091 len1; 0f9/106 continues same iterator',
'1414792d7/2db load start/end;2df/2e2 checks end;306/30b stores start+1 before actualinstantiate31c',
'140a59340/352 descriptor+18 copied to loop+30 threshold;59306 probe supplies flag59359'],
'inference':'descriptor count3 yields four retained instances; no first-item skip. Games own count+1 and threshold/probe policy; dependencies remain reused.',
'unknown':'this independent audit does not execute FMOD, failure variants, or the walking caller itself; author execution fixtures remain separate evidence'}
(O/'evidence.json').write_text(json.dumps(result,indent=2),encoding='utf8');print(json.dumps(result,indent=2))
