"""Create reviewable Cargo manifests from C's locked dependency graph.

Original features/optional/build/target classifications are unavailable. The
locked edges are normalized into [dependencies] and this choice is explicit.
"""
from collections import defaultdict, Counter
import hashlib
import json
import os
from pathlib import Path
import subprocess
import tomllib
from urllib.parse import urlsplit, parse_qs
from restore_pdb import ROOT, PROJECT, atomic_json, digest

DEST = ROOT / 'C/rhapsody2'
META = DEST / '_metadata'
PATHS = {
    'rhapsody': '.', 'aftermath-rs': 'crates/aftermath-rs',
    'dxgi-enumerate-gpus': 'crates/dxgi-enumerate-gpus',
    'gpu-allocator': 'crates/gpu-allocator', 'gpu-profiler': 'crates/gpu-profiler',
    'vk-sync-fork': 'crates/vk-sync-rs',
    **{f'rhapsody-{name}': f'render-std/rhapsody-{name}' for name in ('core', 'dof', 'gi', 'post', 'shadow', 'taa')},
    'rhapsody-deferred': '_unlocated-packages/rhapsody-deferred',
}
HEADER = '# Generated dependency reconstruction from original C Cargo.lock; not author Cargo.toml.\n'
LOCAL_TOOL_PATHS = {
    'shabriri': '../shabriri/_unlocated-packages/shabriri',
    'shabriri-rt': '../shabriri/crates/shabriri-rt',
    'shaderface': '../shabriri/crates/shaderface',
}

def q(value): return json.dumps(value, ensure_ascii=False)
def table(value): return '{ ' + ', '.join(f'{key} = {q(v)}' for key, v in value.items()) + ' }'

def main():
    source = PROJECT / 'evidence/multi-version/C-dependencies.json'
    document = json.loads(source.read_text()); packages = document['packages']
    repo_packages = [p for p in packages if 'rhapsody2.git' in p.get('source', '')]
    assert {p['name'] for p in repo_packages} == set(PATHS)
    by_name = defaultdict(list)
    for p in packages: by_name[p['name']].append(p)
    def resolve(edge):
        parts = edge.split(' ', 2); candidates = by_name[parts[0]]
        if len(parts) > 1: candidates = [p for p in candidates if p['version'] == parts[1]]
        if len(parts) > 2: candidates = [p for p in candidates if '(' + p.get('source', '') + ')' == parts[2]]
        assert len(candidates) == 1, (edge, candidates)
        return candidates[0]
    pool = {}; edges = []
    for package in repo_packages:
        for edge in package.get('dependencies', []):
            dependency = resolve(edge); name = dependency['name']
            if name in pool: assert pool[name] == dependency
            pool[name] = dependency
            edges.append({'owner': package['name'], 'lock_edge': edge, 'dependency': dependency,
                'identity_provenance': 'C original lock', 'original_dependency_kind': None,
                'features': None, 'optional': None, 'target_condition': None})
    revisions = {p['source'].rsplit('#', 1)[1] for p in repo_packages}
    assert len(revisions) == 1; revision = revisions.pop()
    configured = {}
    unresolved_local = []
    for name, dependency in sorted(pool.items()):
        value = {'version': '=' + dependency['version']}
        if name in PATHS: value['path'] = PATHS[name]
        elif name in LOCAL_TOOL_PATHS and (DEST / LOCAL_TOOL_PATHS[name] / 'Cargo.toml').is_file():
            local = tomllib.loads((DEST / LOCAL_TOOL_PATHS[name] / 'Cargo.toml').read_text())
            assert local['package']['name'] == name and local['package']['version'] == dependency['version']
            assert local['package']['metadata']['pdb-reconstruction']['original-revision'] == dependency['source'].rsplit('#', 1)[1]
            value['path'] = LOCAL_TOOL_PATHS[name]
        else:
            origin = dependency.get('source', '')
            if origin.startswith('git+'):
                before, commit = origin[4:].rsplit('#', 1)
                value['git'] = before.split('?', 1)[0]; value['rev'] = commit
            elif origin.startswith('registry+'):
                assert origin == 'registry+https://github.com/rust-lang/crates.io-index'
            else: unresolved_local.append(name)
        configured[name] = value
    assert not unresolved_local, unresolved_local
    source_lock = Path(document['lockfile'])
    if not source_lock.exists(): source_lock = PROJECT / 'evidence/multi-version/C-original-Cargo.lock'
    assert digest(source_lock) == document['lock_sha256']
    reference = META / 'C-original-Cargo.lock'
    reference.write_bytes(source_lock.read_bytes())
    assert digest(reference) == document['lock_sha256']
    root_workspace = ['[workspace]', 'resolver = "2"',
        'members = ['] + ['    ' + q(path) + ',' for path in sorted(set(PATHS.values()))] + [']', '',
        '[workspace.package]', '# Chosen for this metadata workspace; original edition is not recorded.',
        'edition = "2021"', '', '[workspace.dependencies]',
        '# Exact C versions and Git revisions. Internal packages refer to this isolated tree.',
        '# Feature sets/default-features/optional/target/dev/build classifications are unknown.']
    root_workspace += [q(name) + ' = ' + table(value) for name, value in sorted(configured.items())]
    generated_manifests = []; placeholders = []
    for package in sorted(repo_packages, key=lambda p: p['name']):
        name = package['name']; directory = DEST / PATHS[name]; directory.mkdir(parents=True, exist_ok=True)
        manifest = directory / 'Cargo.toml'
        if manifest.exists() and manifest.stat().st_size and not manifest.read_text().startswith(HEADER):
            raise RuntimeError(f'Refuse overwrite manual Cargo.toml: {manifest}')
        lib_path = 'src/lib.rs'
        if not (directory / lib_path).exists():
            lib_path = '_metadata/unrecovered-lib.rs'
            lib = directory / lib_path; lib.parent.mkdir(parents=True, exist_ok=True)
            if not lib.exists(): lib.write_text('// Metadata-only Cargo target. Original crate root was not recorded in the PDB.\n')
            placeholders.append({'package': name, 'target': lib.relative_to(DEST).as_posix(), 'original_crate_root': None})
        lines = [HEADER.rstrip(), '# Versions/sources/dependency edges: original C lock.',
            '# [dependencies] is a normalized edge list, not a recovered build/target/dev split.',
            '# Original features, optional flags, build.rs and edition remain unconfirmed.', '',
            '[package]', 'name = ' + q(name), 'version = ' + q(package['version']),
            'edition.workspace = true', 'publish = false', '',
            '[lib]', 'path = ' + q(lib_path), '', '[dependencies]']
        lines += [q(resolve(edge)['name']) + ' = { workspace = true }' for edge in sorted(package.get('dependencies', []))]
        lines += ['', '[package.metadata.pdb-reconstruction]', 'identity-source = "original C Cargo.lock"',
            'original-repository = "PounceLight/rhapsody2"', 'original-revision = ' + q(revision),
            'original-manifest-recovered = false', 'original-features-recovered = false',
            'original-dependency-kinds-recovered = false', 'edition-is-reconstruction-default = true',
            'normalized-dependencies = true', 'function-bodies-recovered = false',
            'original-lock-reference = ' + q(os.path.relpath(reference, directory).replace('\\', '/'))]
        if name == 'rhapsody': lines += ['', *root_workspace]
        manifest.write_text('\n'.join(lines) + '\n'); generated_manifests.append(manifest)
    b = json.loads((PROJECT / 'evidence/multi-version/B-dependencies.json').read_text())
    b_by = {p['name']: p for p in b['packages']}
    comparison = []
    for p in repo_packages:
        if p['name'] in b_by:
            old = b_by[p['name']]; comparison.append({'name': p['name'], 'B_dependencies': old.get('dependencies', []),
                'C_dependencies': p.get('dependencies', []), 'policy': 'C evidence takes precedence; B consulted as reference only'})
    provenance = {'repository': 'PounceLight/rhapsody2', 'revision': revision,
        'input_manifest_sha256': digest(source), 'original_C_lock_sha256': document['lock_sha256'],
        'packages': repo_packages, 'dependency_edges': edges, 'workspace_dependencies': configured,
        'package_directories': PATHS, 'unlocated_packages': ['rhapsody-deferred'],
        'normalized_metadata_lib_targets': placeholders, 'A_lock_available': False,
        'B_reference_comparison': comparison, 'A_or_B_dependency_fallbacks_used': [],
        'private_manifest_lookup': {'exact_commit_raw_http': '404', 'authenticated_git_https': 'credentials unavailable'},
        'unrecovered_fields': ['original edition and resolver', 'dependency kind (normal/build/dev)',
            'dependency aliases if renamed', 'features/default-features/optional', 'target cfg conditions', 'build.rs', 'workspace patches'],
        'scope': 'Complete observed C lock dependency graph; no claim of original manifest text or compilable engine'}
    provenance['local_private_tool_skeletons'] = {name: spec['path'] for name, spec in configured.items()
        if name in LOCAL_TOOL_PATHS and 'path' in spec}
    atomic_json(META / 'cargo-dependency-provenance.json', provenance)
    verify(generated_manifests, repo_packages, resolve, configured, document)
    update_docs()

def verify(manifests, packages, resolve, configured, original):
    root = tomllib.loads((DEST / 'Cargo.toml').read_text())
    assert root['workspace']['dependencies'] == configured
    checked = []; count = 0
    expected_packages = {p['name']: p for p in packages}
    for manifest in manifests:
        doc = tomllib.loads(manifest.read_text()); name = doc['package']['name']; package = expected_packages[name]
        assert doc['package']['version'] == package['version']
        dependencies = doc.get('dependencies', {})
        assert set(dependencies) == {resolve(edge)['name'] for edge in package.get('dependencies', [])}
        assert all(spec == {'workspace': True} for spec in dependencies.values())
        for edge in package.get('dependencies', []):
            dep = resolve(edge); spec = configured[dep['name']]
            assert spec['version'] == '=' + dep['version']; count += 1
            if dep.get('source', '').startswith('git+') and dep['name'] not in PATHS:
                if 'path' in spec:
                    local = tomllib.loads((DEST / spec['path'] / 'Cargo.toml').read_text())
                    assert local['package']['metadata']['pdb-reconstruction']['original-revision'] == dep['source'].rsplit('#', 1)[1]
                else: assert spec['rev'] == dep['source'].rsplit('#', 1)[1]
        # Cargo's own parser checks workspace inheritance and target structure;
        # no private dependency fetch or lock resolution is required.
        result = subprocess.run(['cargo', 'read-manifest', '--offline', '--manifest-path', str(manifest)], capture_output=True, text=True)
        if result.returncode: raise RuntimeError(f'Cargo parser rejected {manifest}: {result.stderr}')
        parsed = json.loads(result.stdout)
        assert parsed['name'] == name and parsed['version'] == package['version']
        assert len(parsed['dependencies']) == len(dependencies)
        checked.append({'package': name, 'manifest': manifest.relative_to(DEST).as_posix(),
            'dependency_count': len(dependencies), 'cargo_parser_ok': True, 'toml_parse_ok': True})
    assert len(checked) == 13 and count == 88 and len(configured) == 54
    atomic_json(META / 'cargo-verification.json', {'ok': True, 'manifests': checked, 'package_count': 13,
        'direct_dependency_edges': count, 'workspace_dependency_identities': len(configured),
        'all_versions_and_git_revisions_match_C_lock': True, 'original_lock_reference_sha256': original['lock_sha256'],
        'cargo_manifest_parser_checked': True, 'dependency_resolution_or_build_verified': False,
        'original_features_or_dependency_kinds_verified': False})
    print(json.dumps({'cargo_ok': True, 'packages': 13, 'dependency_edges': count, 'dependency_identities': len(configured)}), flush=True)

def update_docs():
    text = '''# Cargo 依赖重建

13 个包的 Cargo.toml 已补齐；全部 88 条依赖边和 54 个 workspace 依赖身份来自 C 原始锁文件。Registry 依赖精确锁版本；外部 Git 依赖锁定原 commit；rhapsody2 内部依赖指向当前独立目录。

原始锁文件参考保存在 `C-original-Cargo.lock`，SHA-256 与原作一致。它不是这个诊断 workspace 新解出的 Cargo.lock。Cargo 自身解析器已经检查全部清单、workspace 继承及目标声明；没有执行依赖下载、构建或游戏验证。

shabriri、shabriri-rt、shaderface 已接到 C/shabriri 的本地元数据骨架。其版本与原始 Git commit 继续从对应包 metadata 和本报告核对；path 替换不表示工具实现已恢复。

原仓库的清单文本无法访问。Cargo.lock 不提供 features、可选标志、target 条件或 normal/build/dev 分类，所以 `[dependencies]` 是完整已知边的规范化汇总，不声称复原了作者分类。edition 2021/resolver 2 是诊断 workspace 的明确选择，未从 PDB 推断。包没有已记录 crate root 时使用 `_metadata/unrecovered-lib.rs` 诊断占位目标。

A 没有锁文件。B 已作为对照参考，但 C 的身份及边均完整，无需 A/B 填补；例如 C 的 gpu-allocator 依赖 thiserror 2.0.11，不能混用 B 的 thiserror 1.0.60 和旧 GUI 依赖。

身份、依赖边、B 对照及未知字段见 `cargo-dependency-provenance.json`；核对结果见 `cargo-verification.json`。这些清单不构成恢复了引擎实现或作者原 Cargo 配置的证明。
'''
    (META / 'CARGO_RECONSTRUCTION.md').write_text(text)

if __name__ == '__main__': main()
