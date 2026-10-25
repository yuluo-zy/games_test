"""Format/parse the new block while preserving the owner's unrelated bytes."""
from pathlib import Path
import difflib, subprocess, tempfile

WORK = Path(__file__).resolve().parent
base = (WORK / 'base/snapshot.rs').read_text()
candidate_path = WORK / 'candidate/snapshot.rs'
with tempfile.TemporaryDirectory(prefix='state-reader-format-') as directory:
    formatted = Path(directory) / 'snapshot.rs'
    formatted.write_bytes(candidate_path.read_bytes())
    subprocess.run(['rustfmt', '--edition', '2021', str(formatted)], check=True)
    candidate = formatted.read_text()
start = base.index('/// 快照 Decorators 节 addr 判别')
end = base.index('impl TerrainSectionJson {', start)
new_start = candidate.index('/// B snapshot addresses use unit "Stairs"')
new_end = candidate.index('impl TerrainSectionJson {', new_start)
result = base[:start] + candidate[new_start:new_end] + base[end:]
old = '        assert!(section.into_storage_snapshot().is_err());'
assert result.count(old) == 1
result = result.replace(old,
    '        let snapshot = section.into_storage_snapshot().unwrap();\n        assert!(snapshot.states.is_empty());')
candidate_path.write_text(result)
relative = 'crates/country-core/src/systems/save/snapshot.rs'
(WORK / 'candidate.patch').write_text(''.join(difflib.unified_diff(
    base.splitlines(keepends=True), result.splitlines(keepends=True),
    fromfile='a/' + relative, tofile='b/' + relative)))
print('Only State/addr wire block and the existing empty-section test are changed.')
