"""C's PounceLight/shabriri repository metadata and locked Cargo graph."""
from collections import defaultdict, Counter
import gzip
import json
import os
from pathlib import PurePosixPath
import re
import subprocess
import tomllib

from restore_pdb import ROOT, PROJECT, Version, atomic_json, body, digest
from restore_rhapsody2 import verify
from restore_rhapsody2_cargo import q, table

DEST = ROOT / 'C/shabriri'
META = DEST / '_metadata'
PATTERN = re.compile(r'/\.cargo/git/checkouts/shabriri-[^/]+/([^/]+)/(.+)$')
PATHS = {'shabriri': '_unlocated-packages/shabriri', 'shabriri-rt': 'crates/shabriri-rt', 'shaderface': 'crates/shaderface'}
HEADER = '# Generated dependency reconstruction from original C Cargo.lock; not author Cargo.toml.\n'

def map_source(path):
    m = PATTERN.search(path.replace('\\', '/'))
    if not m: return None
    rel = PurePosixPath(m[2])
    if rel.is_absolute() or '..' in rel.parts: raise ValueError(path)
    return 'shabriri/' + str(rel)

def main():
    META.mkdir(parents=True, exist_ok=True)
    cache = ROOT / '_pdb/C'
    baseline = json.loads((PROJECT / 'tracking/baselines.json').read_text())['baselines']['C']
    assert digest(cache / 'original.pdb') == baseline['pdb']['sha256']
    locked = json.loads((PROJECT / 'evidence/multi-version/C-dependencies.json').read_text())
    packages = [p for p in locked['packages'] if p['name'] in PATHS]
    assert len(packages) == 3 and all('PounceLight/shabriri.git' in p['source'] for p in packages)
    revisions = {p['source'].rsplit('#', 1)[1] for p in packages}
    assert len(revisions) == 1; revision = revisions.pop()
    modules = json.loads((cache / 'modules.json').read_text())
    selected = [m for m in modules if any(map_source(f) for f in m['files'])]
    paths = {f for m in selected for f in m['files'] if map_source(f)}
    assert len(paths) == 9
    entries = {}
    for module in selected:
        with gzip.open(cache / 'modules' / f"{module['index']:05d}.jsonl.gz", 'rb') as f:
            for line in f:
                if b'"event":"checksum"' not in line: continue
                row = json.loads(line); original = row.get('path', ''); path = map_source(original)
                if path is None: continue
                assert revision.startswith(PATTERN.search(original)[1]) and row['algorithm'] == 3
                entry = entries.setdefault(path, {'original_path': path, 'original_pdb_paths': set(),
                    'source_sha256': set(), 'version': 'C', 'kind': 'dependency_metadata_source',
                    'source_kind': 'repository_source', 'dependency_class': 'PounceLight_private_tool_metadata',
                    'repository': 'PounceLight/shabriri', 'revision': revision})
                entry['original_pdb_paths'].add(original); entry['source_sha256'].add(row['checksum'])
    assert {p for e in entries.values() for p in e['original_pdb_paths']} == paths
    for e in entries.values():
        e['original_pdb_paths'] = sorted(e['original_pdb_paths']); e['source_sha256'] = sorted(e['source_sha256'])
        output = ROOT / 'C' / e['original_path']; output.parent.mkdir(parents=True, exist_ok=True)
        if not output.exists(): output.touch()
    compiled = {p: [m['index'] for m in modules if m['name'].startswith(p.replace('-', '_') + '-')] for p in PATHS}
    assert not compiled['shabriri']
    atomic_json(META / 'repository.json', {'repository': 'PounceLight/shabriri', 'revision': revision,
        'packages': packages, 'source_files': len(entries), 'pdb': baseline['pdb'],
        'package_directories': PATHS, 'package_compiled_modules': compiled,
        'main_shabriri_package_source_path_known': False, 'main_package_build_time_usage': 'inferred; not proven from lock alone',
        'implementation_recovered': False, 'production_integration': False})
    atomic_json(DEST / 'file-manifest.json', {'files': list(entries.values())})
    obj = Version('C', baseline, {}, source_mapper=map_source, source_entries=entries, report_dir=META)
    obj.initial_source_hash = baseline['pdb']['sha256']; obj.complete = json.loads((cache / 'report.json').read_text())['native_completion']
    with gzip.open(cache / 'types.jsonl.gz', 'rt') as f:
        for line in f:
            row = json.loads(line); obj.types[row['namespace']][row['index']] = row
    with gzip.open(cache / 'global-symbols.jsonl.gz', 'rt') as f:
        for line in f:
            row = json.loads(line); d = row.get('decoded', {}); b = body(d)
            if d.get('Kind') == 'S_PUB32': obj.aliases[(b.get('Segment'), b.get('Offset'))].append({'name': b.get('Name'), 'record_offset': row['record_offset'], 'flags': b.get('Flags')})
    for i, module in enumerate(selected):
        archive = cache / 'modules' / f"{module['index']:05d}.jsonl.gz"
        with gzip.open(archive, 'rt') as f: rows = [json.loads(line) for line in f]
        obj.process_module(rows, archive)
        if i % 50 == 0: print(json.dumps({'modules': i + 1, 'total': len(selected)}), flush=True)
    obj.writer.close(); report = obj.write_sources()
    report.update({'scope': 'C private shader tool/runtime metadata; not recovered implementation',
        'repository': 'PounceLight/shabriri', 'revision': revision, 'source_modules_processed': len(selected),
        'main_shabriri_source_available': False})
    atomic_json(META / 'report.json', report); verify(obj, selected, entries, report)
    cargo(packages, locked, revision)
    (DEST / 'README.md').write_text(f'''# C 版 PounceLight/shabriri 骨架

原作锁定 commit：`{revision}`；仓库包：shabriri、shabriri-rt、shaderface，均 0.1.0。

从 C PDB 创建 9 个已记录的源文件（shabriri-rt 3 个，shaderface 6 个），写入 {report['logical_symbol_and_type_pairs']} 个按文件归属的函数符号/类型条目及可用 TPI/IPI 图。每文件保留逐条压缩记录与源校验和；完整 PDB 继续引用 skeleton/_pdb/C/。

`crates/shabriri-rt/`、`crates/shaderface/` 保留原源目录。shabriri 主包在 C PDB 没有编译单元、源路径或函数命名空间记录；它可能仅构建时运行，但此用途仍是推断。主包置于 `_unlocated-packages/shabriri/`，只恢复锁文件身份、14 条依赖边和明确的诊断目标，未编造作者源码目录或生成器实现。

3 份包 Cargo.toml 和根 workspace 清单按 C 原锁文件补齐。版本、Git commit 与依赖身份可核对；edition/resolver 是诊断选择，features、optional、build/target/dev 分类、实际 build.rs 未恢复。来源和核对结果在 `_metadata/cargo-dependency-provenance.json` 与 `_metadata/cargo-verification.json`。

函数体、真实签名参数和字段布局在 PDB 缺失时保持 unknown。此目录是独立元数据骨架，不合并到 reconstruction，不宣称已经复现 shader 编译器。已有的 8 个消费者生成文件位于 C/_generated/ 和 C/rhapsody2/_generated/。
''')

def cargo(packages, locked, revision):
    lookup = defaultdict(list)
    for p in locked['packages']: lookup[p['name']].append(p)
    def resolve(edge):
        parts = edge.split(' ', 2); candidates = lookup[parts[0]]
        if len(parts) > 1: candidates = [p for p in candidates if p['version'] == parts[1]]
        if len(parts) > 2: candidates = [p for p in candidates if '(' + p.get('source', '') + ')' == parts[2]]
        assert len(candidates) == 1, edge
        return candidates[0]
    configured = {}; edges = []
    for package in packages:
        for edge in package.get('dependencies', []):
            p = resolve(edge); value = {'version': '=' + p['version']}
            if p['name'] in PATHS: value['path'] = PATHS[p['name']]
            elif p.get('source', '').startswith('git+'):
                before, commit = p['source'][4:].rsplit('#', 1); value.update(git=before.split('?', 1)[0], rev=commit)
            else: assert p['source'] == 'registry+https://github.com/rust-lang/crates.io-index'
            if p['name'] in configured: assert configured[p['name']] == value
            configured[p['name']] = value
            edges.append({'owner': package['name'], 'dependency': p, 'lock_edge': edge, 'dependency_kind': None, 'features': None})
    workspace = [HEADER.rstrip(), '# Normalized metadata workspace; original manifests are unavailable.',
        '[workspace]', 'resolver = "2"', 'members = ['] + ['    ' + q(p) + ',' for p in PATHS.values()] + [']', '',
        '[workspace.package]', 'edition = "2021"', '', '[workspace.dependencies]']
    workspace += [q(name) + ' = ' + table(spec) for name, spec in sorted(configured.items())]
    manifest = DEST / 'Cargo.toml'
    if manifest.exists() and manifest.stat().st_size and not manifest.read_text().startswith(HEADER): raise RuntimeError('Refuse overwrite manual workspace')
    manifest.write_text('\n'.join(workspace) + '\n')
    checked = []; written = []
    for package in packages:
        directory = DEST / PATHS[package['name']]; directory.mkdir(parents=True, exist_ok=True)
        lib = 'src/lib.rs'
        if not (directory / lib).exists():
            lib = '_metadata/unrecovered-lib.rs'; target = directory / lib; target.parent.mkdir(parents=True, exist_ok=True)
            if not target.exists(): target.write_text('// Metadata-only target. The original crate root and function bodies are not recovered.\n')
        lines = [HEADER.rstrip(), '# [dependencies] normalizes locked edges; original kinds/features are unknown.',
            '[package]', 'name = ' + q(package['name']), 'version = ' + q(package['version']), 'edition.workspace = true',
            'publish = false', '', '[lib]', 'path = ' + q(lib), '', '[dependencies]']
        lines += [q(resolve(edge)['name']) + ' = { workspace = true }' for edge in sorted(package.get('dependencies', []))]
        lines += ['', '[package.metadata.pdb-reconstruction]', 'original-repository = "PounceLight/shabriri"',
            'original-revision = ' + q(revision), 'original-manifest-recovered = false',
            'original-features-recovered = false', 'original-dependency-kinds-recovered = false',
            'edition-is-reconstruction-default = true', 'function-bodies-recovered = false']
        p = directory / 'Cargo.toml'
        if p.exists() and p.stat().st_size and not p.read_text().startswith(HEADER): raise RuntimeError(f'Refuse overwrite {p}')
        p.write_text('\n'.join(lines) + '\n')
        doc = tomllib.loads(p.read_text()); assert set(doc['dependencies']) == {resolve(edge)['name'] for edge in package.get('dependencies', [])}
        written.append((p, package))
    for p, package in written:
        result = subprocess.run(['cargo', 'read-manifest', '--offline', '--manifest-path', str(p)], capture_output=True, text=True)
        if result.returncode: raise RuntimeError(result.stderr)
        parsed = json.loads(result.stdout); assert parsed['name'] == package['name'] and parsed['version'] == package['version']
        assert len(parsed['dependencies']) == len(package.get('dependencies', []))
        checked.append({'package': package['name'], 'path': p.relative_to(DEST).as_posix(), 'dependencies': len(parsed['dependencies']), 'cargo_parser_ok': True})
    assert tomllib.loads(manifest.read_text())['workspace']['dependencies'] == configured
    reference = ROOT / 'C/rhapsody2/_metadata/C-original-Cargo.lock'
    assert digest(reference) == locked['lock_sha256']
    atomic_json(META / 'cargo-dependency-provenance.json', {'packages': packages, 'dependency_edges': edges,
        'workspace_dependencies': configured, 'package_directories': PATHS, 'original_C_lock_sha256': locked['lock_sha256'],
        'original_lock_reference': '../../rhapsody2/_metadata/C-original-Cargo.lock',
        'unrecovered': ['original dependency kinds', 'features/default-features/optional', 'target cfg', 'build.rs', 'edition/resolver', 'main shabriri source path'],
        'A_B_shabriri_information': 'No shabriri family entries in retained B lock; A lock unavailable'})
    atomic_json(META / 'cargo-verification.json', {'ok': True, 'package_count': 3, 'dependency_edges': len(edges),
        'dependency_identities': len(configured), 'all_edges_and_identities_match_C_lock': True,
        'cargo_parser_ok': True, 'manifests': checked, 'dependency_resolution_or_build_verified': False})
    print(json.dumps({'cargo_packages': 3, 'cargo_edges': len(edges), 'cargo_identities': len(configured), 'verification_ok': True}), flush=True)

if __name__ == '__main__': main()
