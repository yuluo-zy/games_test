"""Read original B bytes and retain reproducible session/water lifecycle evidence."""
from pathlib import Path
import hashlib, json, struct, subprocess

ROOT = Path(__file__).resolve().parent
TARGET = Path('D:/game/ljxsj_92385/Tiny Glade/tiny-glade.rne')
blob = TARGET.read_bytes()
sha = hashlib.sha256(blob).hexdigest()
assert sha == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe = struct.unpack_from('<I', blob, 0x3c)[0]
section_count = struct.unpack_from('<H', blob, pe+6)[0]
section_offset = pe+24+struct.unpack_from('<H', blob, pe+20)[0]
sections = [struct.unpack_from('<IIII', blob, section_offset+40*i+8) for i in range(section_count)]
def read(va, size):
    rva = va-0x140000000
    for virtual_size, start, raw_size, raw in sections:
        if start <= rva and rva+size <= start+max(virtual_size,raw_size):
            return blob[raw+rva-start:raw+rva-start+size]
    raise ValueError(hex(va))

procs = {}
for line in Path('D:/game/reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv').read_text(encoding='utf8').splitlines():
    fields = line.split('\t')
    if len(fields)>=5:
        procs[int(fields[0],16)] = (int(fields[1]),fields[-1])
addresses = [0x1408397a0,0x140839b80,0x140941110,0x141452870,
    0x140b66830,0x140b66480,0x140b67620,0x140b67b50,
    0x140b3f550,0x140b3c930,0x140a11930,0x140a04980,
    0x140a7a5b0,0x1408b3510,0x140a43840,
    0x1409ebe80,0x1409ebf20,0x1409ecea0,0x141baefe0,
    0x140b1ae70,0x140b1b1b0,0x140b61d30,0x141bf16c0]
result = {'original_sha256':sha,'functions':{},'constants':{},'source_hashes':{}}
with (ROOT/'startup-session-lifecycle-original.asm').open('w',encoding='utf8') as out:
    for address in addresses:
        size, name = procs[address]
        result['functions'][hex(address)]={'size':size,'sha256':hashlib.sha256(read(address,size)).hexdigest(),'pdb_signature':name}
        asm = subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe','-d',
            f'--start-address={hex(address)}',f'--stop-address={hex(address+size)}',str(TARGET)],
            stdout=subprocess.PIPE,check=True).stdout.decode('utf8')
        out.write(asm)
for address in [0x142ac4d00,0x142ac4d10,0x142ac4d14,0x142ac4d20,
    0x142925870,0x1429345a8,0x142a83814,0x142eed8e0,0x142eed8f0]:
    value = read(address,4)
    result['constants'][hex(address)]={'hex':value.hex(),'f32':struct.unpack('<f',value)[0]}
for relative in [
    'tiny-glade/crates/tiny-glade/src/run_game.rs',
    'tiny-glade/crates/tiny-glade/src/app_systems.rs',
    'tiny-glade/crates/tiny-glade/src/gameplay_startup.rs',
    'tiny-glade/crates/country-core/src/startup/init_photomode_state.rs',
    'tiny-glade/crates/country-core/src/startup/startup_meadow.rs',
    'tiny-glade/crates/country-core/src/startup/clearing.rs',
    'tiny-glade/crates/country-core/src/asset_libraries/placement_data_library.rs',
    'tiny-glade/crates/country-core/src/resources/tree_storage.rs',
    'tiny-glade/crates/systems/forest/src/manage_tree_prefabs.rs',
    'tiny-glade/crates/country-core/src/systems/audio/water_ambience_audio.rs',
    'tiny-glade/crates/systems/water/src/water_audio_grid.rs',
    'tiny-glade/crates/country-core/src/systems/audio/game_loading_audio.rs',
]:
    result['source_hashes'][relative]=hashlib.sha256((Path('D:/game')/relative).read_bytes()).hexdigest()
(ROOT/'startup-session-lifecycle-evidence.json').write_text(json.dumps(result,indent=2),encoding='utf8')
print(json.dumps({'original_sha256':sha,'function_count':len(addresses),'constants':result['constants']},indent=2))
