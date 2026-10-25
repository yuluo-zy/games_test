"""Merge independent code reviews without promoting original-interface acceptance."""
import hashlib
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
BASE = 'evidence/shared-foundation-review-2026-10-13/'
OWNERS = ('small-modules', 'utils-base', 'geometry-curves', 'assets')


def source_path(value, owner):
    if not isinstance(value, str):
        return None
    value = value.replace('\\', '/').split(':', 1)[0]
    if value.startswith('crates/'):
        value = 'reconstruction/shared-foundation/' + value
    elif owner == 'utils-base' and '/' not in value and value.endswith('.rs'):
        value = 'reconstruction/shared-foundation/crates/utils/src/' + value
    return value if value.startswith('reconstruction/shared-foundation/') else None


def collect_hashes(value, owner, output):
    if isinstance(value, list):
        for child in value:
            collect_hashes(child, owner, output)
    elif isinstance(value, dict):
        path = next((source_path(value.get(k), owner) for k in ('path', 'source', 'file')
                     if source_path(value.get(k), owner)), None)
        if path and isinstance(value.get('sha256'), str):
            output[path] = value['sha256']
        for key, child in value.items():
            path = source_path(key, owner)
            if path and isinstance(child, str) and len(child) == 64:
                output[path] = child
            elif isinstance(child, (dict, list)):
                collect_hashes(child, owner, output)


reviews = []
findings = []
source_hashes = {}
for owner in OWNERS:
    relative = BASE + owner + '/review.json'
    raw = (ROOT / relative).read_bytes()
    data = json.loads(raw)
    own_hashes = {}
    # Only current code inventories, not native inputs or historical snapshots.
    for key in ('source_files', 'files', 'source_hashes', 'current_source_sha256', 'current_sources'):
        collect_hashes(data.get(key), owner, own_hashes)
    assert own_hashes, f'Missing current reviewed source inventory: {owner}'
    for path, expected in own_hashes.items():
        assert hashlib.sha256((ROOT / path).read_bytes()).hexdigest() == expected, f'Review source changed: {path}'
        assert path not in source_hashes or source_hashes[path] == expected, f'Conflicting review: {path}'
        source_hashes[path] = expected
    reviews.append({'owner': owner, 'json': relative, 'sha256': hashlib.sha256(raw).hexdigest(),
                    'report': BASE + owner + '/REVIEW.md', 'reviewed_source_files': sorted(own_hashes)})
    for item in data.get('findings', []):
        findings.append({'owner': owner, 'id': item['id'], 'severity': item.get('severity'),
                         'category': item.get('category', item.get('kind')), 'status': item.get('status'),
                         'title': item.get('title', item.get('problem', item.get('description', item.get('before', item.get('gap', ''))))),
                         'source_file': source_path(item.get('file', item.get('source', item.get('location'))), owner),
                         'interfaces': item.get('interfaces', [item.get('interface', item.get('original_interface', ''))]),
                         'evidence_report': relative, 'details': item})

doc = {'schema_version': 1, 'baseline': 'B', 'reviews': reviews, 'findings': findings,
       'current_reviewed_sources': source_hashes,
       'full_original_functionality_accepted': False, 'original_game_consumer_chain_accepted': False,
       'statistics': {'review_groups': len(reviews), 'reviewed_source_files': len(source_hashes),
                      'fixed_findings': sum(x['status'] == 'fixed' for x in findings),
                      'fixed_dependency_reimplementation_findings': sum(x['status'] == 'fixed' and x['category'] == 'dependency_reimplementation' for x in findings)},
       'policy': 'Code inspection and bounded regressions do not fill untouched original interfaces. Historical comparisons stay immutable; this review supplies a separate current-source validation scope.'}
(ROOT / 'tracking/shared-foundation-code-review.json').write_text(json.dumps(doc, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
print(json.dumps(doc['statistics']))
