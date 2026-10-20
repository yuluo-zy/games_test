from pathlib import Path
import hashlib
import json
import difflib

root = Path('D:/game')
out = root / 'reverse-engineering/tiny-glade/evidence/round13-frame-graph-gpu/gpu'
relative = 'tiny-glade/crates/tiny-glade/tests/original_loading_present.rs'
source = root / relative
target = out / 'frame-driver-candidate' / relative
candidate = target.read_bytes().decode('utf-8').replace(
    '//! ProductionRenderer through the same recovered outer `render` as the game.',
    '//! GameFrameDriver::frame and ProductionRenderer through the recovered outer render.',
).replace(
    '// This explicit World fixture has no App scheduler; call the same two-entry\r\n        // producer directly before the public helper. GameLoop uses real Startup.',
    '// Initialize this bounded fixture before its first scheduled frame with\r\n        // the real two-entry producer. GameLoop uses its actual Startup schedule.',
)
target.write_bytes(candidate.encode('utf-8'))
base = source.read_bytes()
(out / 'frame-driver-loading-wsi.patch').write_text(''.join(difflib.unified_diff(
    base.decode('utf-8').splitlines(keepends=True), candidate.splitlines(keepends=True),
    fromfile='a/' + relative, tofile='b/' + relative,
)), encoding='utf-8', newline='')
manifest = json.loads((out / 'frame-driver-manifest.json').read_text())
manifest['files'][0]['candidate_sha256'] = hashlib.sha256(candidate.encode('utf-8')).hexdigest()
manifest['validation'] = 'rustfmt parse/check passed; no Cargo run by worker; root owns compile and real GPU acceptance'
(out / 'frame-driver-manifest.json').write_text(json.dumps(manifest, indent=2), encoding='utf-8')
print(json.dumps(manifest))
