from pathlib import Path
import hashlib
import json
import struct

author = Path('/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/task-index/work/TG-3d532bb8325ae1fe35/terrain-curve-20261018')
target = Path(__file__).resolve().parent
source = (author / 'native_terrain_extend_oracle.py').read_text()
scope = {'__file__': str(author / 'native_terrain_extend_oracle.py'), '__name__': 'review_original'}
# Import definitions only: do not run the author's report-writing footer.
exec(compile(source.split('\ninputs=')[0], str(author / 'native_terrain_extend_oracle.py'), 'exec'), scope)
fixtures = json.loads((author / 'native-extend-fixtures.json').read_text())
def scalar(value):
    return struct.unpack('<f', struct.pack('<I', value))[0]
results = []
for index, expected in enumerate(fixtures['cases']):
    points = [[scalar(value) for value in point] for point in expected['input']]
    point = [scalar(value) for value in expected['point']]
    actual = scope['case'](points, point, expected['inside'], expected['has_stroke'])
    assert actual == expected, (index, actual, expected)
    results.append({'case': index, 'status': 'exact_bits_equal', 'event_count': actual['event_count']})
raw = scope['raw']
assert hashlib.sha256(raw).hexdigest() == fixtures['binary_sha256']
report = {
    'reviewer': 'history_audit',
    'target': 'B',
    'native_cases': results,
    'binary_sha256': hashlib.sha256(raw).hexdigest(),
    'candidate_sha256': hashlib.sha256((author / 'ui_draw_terrain.rs').read_bytes()).hexdigest(),
    'oracle_sha256': hashlib.sha256((author / 'native_terrain_extend_oracle.py').read_bytes()).hexdigest(),
    'boundary': 'Existing read-only Unicorn bridge; last-stroke lookup supplied, point/event buffers preallocated; original state2/helper executes. No Rust candidate, ECS/hash lookup, grow/state1/state3 proof.'
}
(target / 'terrain-state2-independent-review.json').write_text(json.dumps(report, indent=2))
print(json.dumps({'independent_native_cases': len(results), 'exact_matches': len(results), 'events': [entry['event_count'] for entry in results]}, indent=2))
