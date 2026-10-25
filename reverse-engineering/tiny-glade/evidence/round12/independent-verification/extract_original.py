"""Fresh original B bytes and bounded disassembly; no author evidence imported."""
from pathlib import Path
import hashlib, json, struct, subprocess

ROOT = Path('D:/game')
OUT = Path(__file__).resolve().parent
ORIGINAL = ROOT / 'ljxsj_92385/Tiny Glade/tiny-glade.rne'
data = ORIGINAL.read_bytes()
digest = hashlib.sha256(data).hexdigest()
assert digest == 'f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03'
pe = struct.unpack_from('<I', data, 60)[0]
nsec = struct.unpack_from('<H', data, pe + 6)[0]
optsize = struct.unpack_from('<H', data, pe + 20)[0]
base = struct.unpack_from('<Q', data, pe + 48)[0]
sections = [struct.unpack_from('<IIII', data, pe + 24 + optsize + i * 40 + 8) for i in range(nsec)]

def read(va, length):
    rva = va - base
    for _, start, size, raw in sections:
        if start <= rva and rva + length <= start + size:
            return data[raw + rva - start:raw + rva - start + length]
    raise ValueError(hex(va))

ranges = [
    ('audio-channel-caller', 0x140a557a0, 0x55),
    ('audio-channel-constructor', 0x1408d9d30, 777),
    ('audio-recv', 0x140a57100, 318),
    ('audio-closure', 0x140a520b0, 1258),
    ('array-recv', 0x141679d70, 912),
    ('array-send', 0x14167b9b0, 944),
    ('wall-serializer-1', 0x140314dc0, 5944),
    ('wall-serializer-2', 0x141400220, 5944),
    ('migration-writer-window', 0x14147263c, 0x169),
    ('migration-writer-arrow', 0x1414729c9, 0xe9),
    ('migration-wrapper', 0x141472e8e, 0x29),
    ('migration-consumer', 0x14146eb31, 0x2db),
    ('migration-window-cold-block', 0x14146f6d8, 0x1fe),
    ('load-game-complete', 0x14146d030, 15561),
    ('parse-game-complete', 0x141470d00, 14275),
    ('decorator-serializer', 0x1402de600, 691),
    ('roof-three-system-tuple', 0x140ddffb0, 1131),
    ('roof-plugin-complete', 0x14062fcb0, 2357),
    ('roof-support-new', 0x1412c2650, 627),
    ('roof-support-buffer-new', 0x14161a590, 451),
    ('roof-support-draw', 0x1409d98a0, 788),
    ('roof-decorator-archive', 0x1421a0470, 9894),
    ('decorator-cull-allow-wall', 0x140b2bc00, 292),
    ('decorator-cull-set-extend', 0x140b76260, 154),
    ('roof-edit-archive-callsite', 0x141a1a0a5, 0x66),
]
report = {'original': str(ORIGINAL), 'sha256': digest, 'ranges': [], 'tables': {}, 'strings': {}}
for name, va, size in ranges:
    asm = subprocess.run(['C:/Program Files/LLVM/bin/llvm-objdump.exe', '-d', f'--start-address={va:#x}', f'--stop-address={va+size:#x}', str(ORIGINAL)], check=True, stdout=subprocess.PIPE).stdout
    (OUT / f'{name}.asm').write_bytes(asm)
    report['ranges'].append({'name': name, 'va': hex(va), 'size': size, 'sha256': hashlib.sha256(read(va,size)).hexdigest()})

for table, tags in [(0x1429bb2fc, [9,10,12,16]), (0x142c5badc, [9,10,12,16]), (0x1429b3460, [0,1,2])]:
    rows = []
    for tag in tags:
        target = table + struct.unpack('<i', read(table + tag * 4, 4))[0]
        code = read(target, 140)
        names = []
        for i in range(len(code)-13):
            if code[i:i+3] == b'\x4c\x8d\x05' and code[i+7:i+9] == b'\x41\xb9':
                va = target+i+7+struct.unpack_from('<i',code,i+3)[0]
                length = struct.unpack_from('<I',code,i+9)[0]
                names.append({'pointer':hex(va), 'length':length, 'text':read(va,length).decode('ascii')})
                break
        rows.append({'tag':tag, 'target':hex(target), 'names':names})
    report['tables'][hex(table)] = rows

for ptr in [0x142c64a90, 0x142c64af0]:
    va, length = struct.unpack('<QQ', read(ptr,16))
    report['strings'][hex(ptr)] = {'pointer':hex(va), 'length':length, 'text':read(va,length).decode()}
for ptr, length in [(0x142c5b69b,23), (0x142c5b6b2,22)]:
    report['strings'][hex(ptr)] = {'text':read(ptr,length).decode()}

report['production_sources'] = {
    p:hashlib.sha256((ROOT/p).read_bytes()).hexdigest() for p in [
        'tiny-glade/crates/country-core/src/resources/audio_resources.rs',
        'tiny-glade/crates/country-core/src/resources/audio_instance.rs',
        'tiny-glade/crates/country-core/src/utils/audio_utils.rs',
    ]
}
symbols = []
wanted = {'0x1408d9d30','0x141679d70','0x14167b9b0','0x141583920','0x141588150','0x141585160','0x140ddffb0'}
for line in (ROOT/'reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv').read_text(encoding='utf8').splitlines():
    fields=line.split('\t')
    if fields[0] in wanted: symbols.append({'va':fields[0],'length':int(fields[1]),'symbol':fields[-1]})
report['pdb_resolved_callees'] = symbols
import re
procs = {}
for line in (ROOT/'reverse-engineering/tiny-glade/evidence/country-core-audit/data/procs.tsv').read_text(encoding='utf8').splitlines():
    fields=line.split('\t')
    if len(fields) >= 5: procs.setdefault(fields[0],[]).append(fields[-1])
plugin = (OUT/'roof-plugin-complete.asm').read_text()
report['roof_plugin_direct_calls'] = [
    {'target':address,'symbols':procs.get(address,[])}
    for address in re.findall(r'callq\s+(0x[0-9a-f]+)',plugin)
]
report['roof_archive_direct_calls'] = [
    {'target':address,'symbols':procs.get(address,[])}
    for address in re.findall(r'callq\s+(0x[0-9a-f]+)',(OUT/'roof-decorator-archive.asm').read_text())
]
needle = struct.pack('<Q',0x83773598f5ecfead)
occurrences = []
for _,start,size,raw in sections:
    at = raw
    while True:
        at = data.find(needle,at,raw+size)
        if at < 0: break
        occurrences.append(hex(base+start+at-raw)); at += 1
report['roof_support_ssbo_identity_occurrences'] = occurrences
(OUT/'original-evidence.json').write_text(json.dumps(report,indent=2),encoding='utf8')
print(json.dumps({'ranges':len(ranges), 'tables':report['tables'], 'strings':report['strings']},indent=2))
