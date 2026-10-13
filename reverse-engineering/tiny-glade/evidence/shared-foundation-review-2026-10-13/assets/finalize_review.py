"""Record this review's scope and current source identities; never rewrite old proofs."""
import hashlib
import json
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
OUT = Path(__file__).resolve().parent
WORKSPACE = ROOT / 'reconstruction/shared-foundation'

def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()

def dump(name, value):
    (OUT / name).write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')

source_hashes = {
    str(p.relative_to(ROOT)).replace('\\', '/'): sha(p)
    for crate in ['asset', 'asset-pipe']
    for folder in ['src', 'tests']
    for p in sorted((WORKSPACE / 'crates' / crate / folder).rglob('*.rs'))
}
coverage_path = ROOT / 'evidence/shared-foundation-2026-10-13/assets-small-first/interface-coverage.json'
coverage = json.loads(coverage_path.read_text(encoding='utf-8'))
original_lock = ROOT / 'tracking/original-Cargo.lock'
current_lock = WORKSPACE / 'Cargo.lock'
original = tomllib.loads(original_lock.read_text(encoding='utf-8'))['package']
current = tomllib.loads(current_lock.read_text(encoding='utf-8'))['package']

def identity(p):
    return tuple(p.get(k) for k in ['name', 'version', 'source', 'checksum'])

by_name = {}
for package in current:
    by_name.setdefault(package['name'], []).append(package)
queue = [next(p for p in current if p['name'] == name) for name in ['asset', 'asset-pipe']]
seen = set()
reachable = []
while queue:
    p = queue.pop()
    key = identity(p)
    if key in seen:
        continue
    seen.add(key)
    reachable.append(p)
    for dep in p.get('dependencies', []):
        name, *rest = dep.split()
        matches = by_name[name]
        if rest:
            matches = [candidate for candidate in matches if candidate['version'] == rest[0]]
        assert len(matches) == 1, (dep, matches)
        queue.append(matches[0])
original_identities = {identity(p) for p in original}
external = [dict(zip(['name', 'version', 'source', 'checksum'], identity(p))) for p in reachable if p.get('source')]
for p in external:
    p['original_lock_exact_match'] = tuple(p[k] for k in ['name', 'version', 'source', 'checksum']) in original_identities
assert all(p['original_lock_exact_match'] for p in external)

findings = [
    {
        'id': 'AR-01', 'severity': 'P2', 'category': 'confirmed_performance_defect', 'status': 'fixed',
        'original_interface': 'asset_pipe::texture::{process_regular_image,with_rgba8,resize_image}',
        'source': 'crates/asset-pipe/src/texture.rs:102,133,171',
        'evidence_kind': 'original bounded decompilation plus measured candidate allocations',
        'evidence': ['evidence/shared-foundation-2026-10-13/assets-small-first/bounded-report.artifacts/001.stdout.txt: 0x1420e3080 linear compressed branch directly calls 0x1417bcd60 on the existing image buffer', 'crates/asset-pipe/tests/allocation_review.rs'],
        'reproduction': '512x512 owned RGBA8 image; Linear, Color/BC7, no resize or mips. Before patch two 1048576-byte Rust pixel-buffer allocations. After patch zero.',
        'fix': 'Consume an already owned decoded RGBA8 image; Cow borrows the linear compression view; consume resized RGBA8 instead of cloning it. sRGB continues to own the transfer workspace.',
        'verification': 'Counter probe failed before fix, passed after fix; BC7 bytes equal a direct call to the original Intel dependency; all 20 package tests and strict Clippy passed.',
        'performance_limit': 'Measured allocation count and cumulative bytes only. No original producer timing, wall-time speedup, or total peak RSS claim. Native ISPC allocations are outside the Rust allocator probe.',
    },
    {
        'id': 'AR-02', 'severity': 'P1', 'category': 'functional_completeness_gap', 'status': 'open_known_gap',
        'original_interface': 'B prefab Inherit/concrete_default and remaining 19 indexed interfaces',
        'source': 'crates/asset/src/prefab/config.rs:121 and original interface inventory',
        'evidence_kind': 'existing original PDB inventory; candidate source inspection',
        'evidence': ['evidence/shared-foundation-2026-10-13/assets-small-first/interface-coverage.json'],
        'reproduction': 'Partial RT/audio configs may still contain Inheritable::Inherit after currently restored outer defaults. No complete original default contract or original consumer execution has proved all required fields resolve.',
        'fix': None,
        'verification': 'Published prefab parsing/merge regression passes, but does not turn untouched concrete_default methods or unresolved consumer leaves into complete functionality.',
        'limit': 'Not a newly proven behavioral divergence; original defaults must be recovered before choosing values. Do not invent unit scales/colors or map opaque impl owner numbers by guess.',
    },
    {
        'id': 'AR-03', 'severity': 'P1', 'category': 'original_behavior_validation_gap', 'status': 'open',
        'original_interface': 'asset_pipe::texture::{process_regular_image,process_image_array,process_dds_image,compile_texture_asset}',
        'source': 'crates/asset-pipe/src/texture.rs:150,267,321',
        'evidence_kind': 'original input inventory and current test scope',
        'evidence': ['evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md', 'crates/asset/tests/shipped_textures.rs', 'crates/asset-pipe/tests/pipeline_contract.rs'],
        'reproduction': 'Original source tree has RON options but no source PNG/JPG/DDS corpus. Archive decode/rearchive cannot establish original pixel filtering/compression producer equality.',
        'fix': None,
        'verification': '388 shipped archives roundtrip, 28 prefab input files parse, and synthetic contracts pass. Full producer and country-core consumer remain unaccepted.',
    },
    {
        'id': 'AR-04', 'severity': 'P2', 'category': 'adapter_scope_limitation', 'status': 'open',
        'original_interface': 'asset::inheritable::deserialize_inheritable_option',
        'source': 'crates/asset/src/inheritable.rs:80',
        'evidence_kind': 'source-confirmed limitation; original broader consumer scope unknown',
        'evidence': ['crates/asset/src/inheritable.rs', 'evidence/shared-foundation-2026-10-13/assets-small-first/DELIVERY.md'],
        'reproduction': 'Callback captures ron::RawValue then creates a fresh RON decoder; it requires DeserializeOwned. Generic non-RON deserializers, borrowed values and outer RON extension propagation are not supported/proved.',
        'fix': None,
        'verification': 'Shipped RON and bare/Some/None/unit cases pass. Original generic signatures do not prove the game used another format. No private replacement parser has been introduced.',
    },
]

performance_risks = [
    {
        'id': 'AP-01', 'severity': 'P2', 'status': 'original_policy_preserved_unbenchmarked',
        'source': 'crates/asset-pipe/src/texture.rs:369',
        'rule': 'Original easy-parallel each/run schedules one task per descriptor and joins every task before collecting errors.',
        'risk': 'Large image batches may have high simultaneous decode/compression memory and many threads. No reconstructed private pool; do not silently add a worker cap while claiming original scheduling equivalence.',
        'evidence': 'evidence/shared-foundation-2026-10-13/assets-small-first/parallel-policy.json',
    },
    {
        'id': 'AP-02', 'severity': 'P2', 'status': 'original_cache_policy_preserved',
        'source': 'crates/asset-pipe/src/texture.rs:328',
        'rule': 'Cache mtime is metadata(desc.path).modified(), including the array directory, not maximum recursive source_files() mtime; header-only hit is permitted.',
        'risk': 'Editing existing array-layer contents without changing the directory mtime can keep an old cache hit. Eight valid header bytes can also accept a missing payload. These are documented original rules, not candidate fixes authorized to change business behavior.',
        'evidence': 'bounded original compile_texture_asset 0x1417b8f50 metadata/caching sequence; current cache test',
    },
    {
        'id': 'AP-03', 'severity': 'P3', 'status': 'allocation_risk_not_benchmarked',
        'source': 'crates/asset/src/texture/blob.rs:27',
        'rule': 'Load creates an aligned archive copy and then an owned Vec payload through original rkyv.',
        'risk': 'Peak memory includes input, aligned archive and decoded payload. Alignment and ownership justify the safe adapter; no runtime memory-budget or original loader-equivalence proof yet.',
    },
]
reuse = {
    'algorithm_reimplementations_found': [],
    'original_lock_sha256': sha(original_lock), 'current_lock_sha256': sha(current_lock),
    'reachable_external_packages': len(external), 'all_exact_original_identity': True,
    'external_packages': sorted(external, key=lambda p: (p['name'], p['version'])),
    'delegates': ['image codec/conversion/CatmullRom', 'Intel BC4/5/7 compressor and presets', 'ddsfile decoding', 'RON parsing/RawValue', 'serde typed schema', 'rkyv validation/archive', 'easy-parallel scheduling/join', 'std Arc/HashMap/filesystem/SystemTime', 'relative-path operations'],
    'private_policy_retained': ['inheritance/missing/default state transitions', 'RGB-only transfer constants/branches', 'compression eligibility and formats', 'mip sizing and repeated-edge padding', 'scan extension/array index policy', 'metadata/cache/compile error ordering', 'prefab dependency association'],
    'limits': ['Not an audit of older unrelated restoration directories', 'Cargo.lock does not establish original complete compile-time feature set'],
}
review = {
    'schema_version': 1, 'baseline': 'B', 'scope': 'asset + asset-pipe src/tests only',
    'business_equivalence_complete': False,
    'counts': {'confirmed_fixed_defects': 1, 'new_confirmed_business_logic_divergences': 0, 'open_function_or_validation_findings': 3},
    'interface_baseline': coverage['counts'], 'untouched_interfaces': coverage['untouched_interfaces'],
    'review_does_not_promote_interface_coverage': True,
    'findings': findings, 'performance_risks': performance_risks,
    'dependency_reuse': reuse, 'current_source_sha256': source_hashes,
    'original_comparison_scope': 'Reused bounded original instructions/PDB and prior differential artifacts; new allocation/BC7 checks run reconstructed code + original dependency, not original game runtime.',
}
dump('review.json', review)
dump('validation.json', {
    'schema_version': 1,
    'commands': [
        {'command': 'cargo test -p asset-pipe --test allocation_review --locked --offline -- --nocapture', 'before_fix_exit': 1, 'before_full_rgba_allocations': 2, 'after_fix_exit': 0, 'after_full_rgba_allocations': 0, 'source_image_bytes': 1048576, 'cumulative_rgba_copy_allocation_bytes_avoided': 2097152, 'dependency_bc7_bytes_equal': True},
        {'command': 'cargo test -p asset -p asset-pipe --locked --offline', 'exit_code': 0, 'tests_passed': 20, 'scope': '9 asset tests + 11 asset-pipe tests; includes 388 original archive files, prefab input/merge, pipeline and allocation regression'},
        {'command': 'cargo clippy -p asset -p asset-pipe --all-targets --locked --offline -- -D warnings', 'exit_code': 0},
        {'command': 'rustfmt --edition 2024 crates/asset-pipe/src/texture.rs crates/asset-pipe/tests/allocation_review.rs', 'exit_code': 0},
    ],
    'native_original_runtime_rerun': False, 'full_producer_accepted': False, 'original_game_consumer_accepted': False,
    'current_source_sha256': source_hashes, 'current_lock_sha256': sha(current_lock),
    'dependency_reuse': {'reachable_external_packages': len(external), 'all_exact_original_identity': True},
    'historical_reports': 'Kept unchanged; previous source hashes are historical after this performance patch. Current review validation is separate.',
})
print(f'Wrote review/validation: {len(source_hashes)} source/test hashes, {len(external)} original-matching external packages')
