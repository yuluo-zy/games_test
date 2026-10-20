from pathlib import Path
import hashlib,json,subprocess
here=Path(__file__).resolve().parent
root=here.parents[4]
original=root/'reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe'
assert hashlib.sha256(original.read_bytes()).hexdigest()=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
data=json.loads((here.parent/'history/reveal-forest-original.json').read_text(encoding='utf8'))
row=data['functions']['0x140a78aa0']
full=bytes.fromhex(row['raw_hex'])
assert len(full)==6913 and hashlib.sha256(full).hexdigest()==row['sha256']
start,end=0x140a79683,0x140a79760
part=full[start-0x140a78aa0:end-0x140a78aa0]
asm=subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d',f'--start-address={hex(start)}',f'--stop-address={hex(end)}',str(original)],check=True,capture_output=True).stdout
(here/'emitter-upload-branch-original.asm').write_bytes(asm)
(here/'emitter-upload-branch-evidence.json').write_text(json.dumps({
 'original_pe_sha256':hashlib.sha256(original.read_bytes()).hexdigest(),
 'start':hex(start),'end_exclusive':hex(end),'sha256':hashlib.sha256(part).hexdigest(),
 'lookup':'140a796a4 call1412c7380 AssetSsboLibrary::get_mut<ParticleEmitterSsbo>',
 'none':'140a796b0 test rax,rax;140a796b3 je140a79739',
 'empty':'140a796b9 test rdi,rdi;140a796bc je140a79739',
 'present_nonempty':'140a796c5 compares complete byte length against buffer+18;140a79723 uploads whole vector',
 'publish':'140a79739..140a7975d min(vector_len,1100) on None/empty/upload-success paths',
 'acceptance':'Preserve optional original branch; actual App GPU acceptance must independently require the canonical startup SSBO allocation and a nonempty upload.'
},indent=2),encoding='utf8')
