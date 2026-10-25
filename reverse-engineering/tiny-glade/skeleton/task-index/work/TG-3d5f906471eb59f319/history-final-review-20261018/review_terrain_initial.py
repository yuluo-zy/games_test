from pathlib import Path
import hashlib
import json
import struct

ROOT = Path('/Users/yuluo/Desktop/game')
WORK = ROOT / 'games_test/reverse-engineering/tiny-glade/skeleton/task-index/work'
AUTHOR = WORK / 'TG-2f47009d20e6633209/terrain-curve-20261018'
PACKAGE = WORK / 'TG-3d532bb8325ae1fe35/terrain-curve-20261018/initial-height'
OUT = Path(__file__).resolve().parent
manifest = json.loads((PACKAGE / 'frozen-inputs.json').read_text())
for entry in manifest['files']:
    assert hashlib.sha256(Path(entry['path']).read_bytes()).hexdigest() == entry['sha256'], entry
source = (AUTHOR / 'native_initial_height_oracle.py').read_text()
scope = {'__file__': str(AUTHOR / 'native_initial_height_oracle.py'), '__name__': 'independent_review'}
# Load definitions only; never execute the author's report-writing footer.
exec(compile(source.split('\ncases=')[0], scope['__file__'], 'exec'), scope)
fixture = json.loads((AUTHOR / 'native-initial-height-fixtures.json').read_text())
def scalar(bits):
    return struct.unpack('<f', struct.pack('<I', bits))[0]
cases = []
for index, expected in enumerate(fixture['cases']):
    actual = scope['case'](expected['mouse_index'], expected['rock'], scalar(expected['drag_bits']))
    assert actual == expected, (index, actual, expected)
    audio_words = list(struct.unpack('<5I', scope['u'].mem_read(scope['BUF2']+8, 20)))
    cases.append({'case': index, 'result': 'exact_match', 'height_bits': actual['height_bits'], 'audio_tag_xyz_height_bits': audio_words})
extra = [scope['case'](-2, rock, value) for rock in [False, True] for value in [-0.0, float('-inf'), float('inf')]]
assert all(case['editing_active'] == case['ui_active'] == case['changed_count'] == case['placement_count'] == 1 for case in extra)
assert all(case['placement_tag'] == 5 for case in extra)
raw = scope['raw']
assert hashlib.sha256(raw).hexdigest() == fixture['binary_sha256']
import pefile
from capstone import Cs, CS_ARCH_X86, CS_MODE_64
binary = ROOT / 'ljxsj_92385/Tiny Glade/tiny-glade.rne'
patched = ROOT / 'ljxsj_92385/Tiny Glade/tiny-glade.exe'
pe = pefile.PE(str(binary), fast_load=True)
pe_patched = pefile.PE(str(patched), fast_load=True)
base = pe.OPTIONAL_HEADER.ImageBase
ranges = [(0x140aa00e9, 0x140aa076d), (0x14161cd80, 0x14161d0e0)]
matches = []
for start, end in ranges:
    original = pe.get_data(start-base, end-start)
    current = pe_patched.get_data(start-base, end-start)
    assert original == current
    matches.append({'start':hex(start), 'end_exclusive':hex(end), 'sha256':hashlib.sha256(original).hexdigest(), 'patched_identical':True})
for start, length, name in [(0x140316660,2813,'original-edittype-serialize.asm'),(0x140b3b630,803,'original-edittype-visitor.asm'),(0x1404f41b0,1324,'original-terrain-serialize.asm'),(0x140b3ad60,2246,'original-edittype-deserialize.asm')]:
    data = pe.get_data(start-base,length)
    md=Cs(CS_ARCH_X86,CS_MODE_64)
    lines=[f'{ins.address:x}: {ins.mnemonic} {ins.op_str}' for ins in md.disasm(data,start)]
    (OUT/name).write_text('\n'.join(lines)+'\n')
report = {'reviewer':'history_audit','target':'B','binary_sha256':hashlib.sha256(raw).hexdigest(), 'manifest_sha256':hashlib.sha256((PACKAGE/'frozen-inputs.json').read_bytes()).hexdigest(), 'frozen_files_checked':len(manifest['files']), 'cases':cases, 'extra_signed_index_and_extremes':extra,'ranges':matches,'boundary':'Original state1 instructions and bit sampling execute under retained bridge with a resolved new_stroke and preallocated native map/queues; allocator/world/type dispatch are supplied. No Rust compilation, real GPU, new_stroke lookup or native global scheduling is accepted.'}
(OUT/'terrain-state1-independent-review.json').write_text(json.dumps(report,indent=2)+'\n')
print(json.dumps({'native_cases':len(cases),'exact_matches':len(cases),'extra_cases':len(extra),'frozen_files_checked':len(manifest['files']),'raw_ranges_identical':len(matches)}))
