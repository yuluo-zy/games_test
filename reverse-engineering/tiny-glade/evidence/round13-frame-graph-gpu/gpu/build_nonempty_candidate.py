from pathlib import Path
import hashlib
import json
import difflib

root = Path('D:/game')
out = root / 'reverse-engineering/tiny-glade/evidence/round13-frame-graph-gpu/gpu'
relative = 'tiny-glade/crates/tiny-glade/tests/original_gameplay_chain.rs'
source = root / relative
base = source.read_bytes()
text = base.decode('utf-8')
newline = '\r\n' if '\r\n' in text else '\n'
needle = '    assert!(world.contains_resource::<tiny_glade::loading_graph::LoadingGpuResources>());'
assert text.count(needle) == 1
insert = '''    anyhow::ensure!(
        lists.total_len() > 0,
        "shipped gameplay acceptance requires actual nonempty DrawLists before production GPU warmup"
    );
'''.replace('\n', newline)
candidate = text.replace(needle, insert + needle)
target = out / 'nonempty-candidate' / relative
target.parent.mkdir(parents=True, exist_ok=True)
target.write_bytes(candidate.encode('utf-8'))
(out / 'nonempty-gameplay.patch').write_text(''.join(difflib.unified_diff(
    text.splitlines(keepends=True), candidate.splitlines(keepends=True),
    fromfile='a/' + relative, tofile='b/' + relative,
)), encoding='utf-8', newline='')
(out / 'nonempty-manifest.json').write_text(json.dumps({
    'scope': 'require actual nonempty gameplay draw lists; this does not establish rendered pixel correctness',
    'production_written': False,
    'files': [{'path': relative, 'base_sha256': hashlib.sha256(base).hexdigest(),
               'candidate_sha256': hashlib.sha256(candidate.encode('utf-8')).hexdigest()}],
    'validation': 'root owns Cargo and actual GPU acceptance',
}, indent=2), encoding='utf-8')
print('nonempty-gameplay.patch generated')
