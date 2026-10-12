"""Materialize C's locked rhapsody2 source/metadata tree from retained PDB records."""
from collections import Counter, defaultdict
import gzip
import hashlib
import json
from pathlib import Path, PurePosixPath
import re
import time

from restore_pdb import ROOT, PROJECT, Version, atomic_json, body, digest

VERSION = 'C'
REPOSITORY = 'PounceLight/rhapsody2'
PREFIX = 'rhapsody2/'
GIT_PATH = re.compile(r'/\.cargo/git/checkouts/rhapsody2-[^/]+/([^/]+)/(.+)$')
GENERATED_PATH = re.compile(r'country-slice-private/(target/[^\n]+/release/build/(rhapsody-[^/]+)/out/[^/]+\.rs)$')

def map_source(path):
    path = path.replace('\\', '/')
    m = GIT_PATH.search(path)
    if m:
        rel = PurePosixPath(m[2])
        if rel.is_absolute() or '..' in rel.parts: raise ValueError(path)
        return PREFIX + str(rel)
    m = GENERATED_PATH.search(path)
    if m: return PREFIX + '_generated/' + m[1]
    return None

def main():
    started = time.monotonic()
    cache = ROOT / '_pdb' / VERSION
    dest = ROOT / VERSION / 'rhapsody2'
    meta = dest / '_metadata'
    meta.mkdir(parents=True, exist_ok=True)
    baseline = json.loads((PROJECT / 'tracking/baselines.json').read_text())['baselines'][VERSION]
    assert digest(cache / 'original.pdb') == baseline['pdb']['sha256']
    deps = json.loads((PROJECT / 'evidence/multi-version/C-dependencies.json').read_text())
    packages = [p for p in deps['packages'] if 'rhapsody2.git' in p.get('source', '')]
    revisions = {p['source'].rsplit('#', 1)[1] for p in packages}
    assert len(revisions) == 1
    revision = revisions.pop()
    modules = json.loads((cache / 'modules.json').read_text())
    selected = [m for m in modules if any(map_source(p) for p in m['files'])]
    entries = {}
    checksum_counts = Counter()
    crate_roots = set()
    for module in selected:
        with gzip.open(cache / 'modules' / f"{module['index']:05d}.jsonl.gz", 'rb') as f:
            for line in f:
                if b'"event":"checksum"' not in line: continue
                row = json.loads(line)
                path = row.get('path', '')
                target = map_source(path)
                if target is None: continue
                kind = 'generated_binding' if '/_generated/' in target else 'repository_source'
                git = GIT_PATH.search(path)
                if git and not revision.startswith(git[1]): raise RuntimeError('Checkout revision mismatch')
                if row['algorithm'] != 3: raise RuntimeError(f'Unexpected checksum kind: {path}')
                record = entries.setdefault(target, {'original_path': target, 'original_pdb_paths': set(),
                    'source_sha256': set(), 'version': VERSION, 'kind': 'dependency_metadata_source',
                    'dependency_class': 'dependency_reuse_metadata_only', 'repository': REPOSITORY,
                    'revision': revision, 'source_kind': kind})
                record['original_pdb_paths'].add(path)
                record['source_sha256'].add(row['checksum'])
                checksum_counts[target] += 1
                if kind == 'repository_source':
                    relative = target[len(PREFIX):]
                    if relative.startswith('src/'): crate_roots.add('')
                    elif '/src/' in relative: crate_roots.add(relative.split('/src/', 1)[0])
    expected = {p for m in modules for p in m['files'] if GIT_PATH.search(p)}
    observed = {p for e in entries.values() for p in e['original_pdb_paths'] if GIT_PATH.search(p)}
    assert expected == observed
    assert len(observed) == 85
    for entry in entries.values():
        entry['original_pdb_paths'] = sorted(entry['original_pdb_paths'])
        entry['source_sha256'] = sorted(entry['source_sha256'])
        output = ROOT / VERSION / entry['original_path']
        output.parent.mkdir(parents=True, exist_ok=True)
        if not output.exists(): output.touch()
    placeholder_manifests = []
    for relative in sorted(crate_roots):
        p = dest / relative / 'Cargo.toml'
        if not p.exists(): p.touch()
        placeholder_manifests.append(p.relative_to(dest).as_posix())
    # The lock names this package, but no file in it has a PDB source path.
    known_names = {PurePosixPath(r).name for r in crate_roots if r} | {'rhapsody', 'vk-sync-fork'}
    unlocated = [p for p in packages if p['name'] not in known_names]
    for pkg in unlocated:
        folder = dest / '_unlocated-packages' / pkg['name']
        folder.mkdir(parents=True, exist_ok=True)
        manifest = folder / 'Cargo.toml'
        if not manifest.exists(): manifest.touch()
        placeholder_manifests.append(manifest.relative_to(dest).as_posix())
        atomic_json(folder / 'package.json', {'package': pkg, 'original_source_directory': None,
            'reason': 'Package exists in the original C lock; no source file recorded in the C PDB',
            'cargo_manifest_contents_recovered': False})
    atomic_json(meta / 'repository.json', {'repository': REPOSITORY, 'revision': revision,
        'packages': packages, 'pdb': baseline['pdb'], 'source_files': len(entries),
        'repository_source_files': len(observed), 'generated_binding_files': sum(e['source_kind'] == 'generated_binding' for e in entries.values()),
        'source_checksums': checksum_counts, 'placeholder_manifests': placeholder_manifests,
        'unlocated_packages': unlocated, 'production_integration': False, 'implementation_recovered': False})
    manifest = {'files': list(entries.values())}
    atomic_json(dest / 'file-manifest.json', manifest)
    obj = Version(VERSION, baseline, manifest, source_mapper=map_source, source_entries=entries, report_dir=meta)
    obj.initial_source_hash = baseline['pdb']['sha256']
    original_report = json.loads((cache / 'report.json').read_text())
    obj.complete = original_report['native_completion']
    with gzip.open(cache / 'types.jsonl.gz', 'rt') as f:
        for line in f:
            row = json.loads(line)
            obj.types[row['namespace']][row['index']] = row
    with gzip.open(cache / 'global-symbols.jsonl.gz', 'rt') as f:
        for line in f:
            row = json.loads(line); d = row.get('decoded', {}); b = body(d)
            if d.get('Kind') == 'S_PUB32':
                obj.aliases[(b.get('Segment'), b.get('Offset'))].append({'name': b.get('Name'), 'record_offset': row['record_offset'], 'flags': b.get('Flags')})
    for i, module in enumerate(selected):
        archive = cache / 'modules' / f"{module['index']:05d}.jsonl.gz"
        with gzip.open(archive, 'rt') as f: rows = [json.loads(line) for line in f]
        obj.process_module(rows, archive)
        if i % 50 == 0:
            print(json.dumps({'modules_processed': i + 1, 'selected_modules': len(selected), 'seconds': round(time.monotonic() - started, 1)}), flush=True)
    obj.writer.close()
    report = obj.write_sources()
    report.update({'scope': 'C rhapsody2 dependency metadata, isolated from game implementation',
        'repository': REPOSITORY, 'revision': revision, 'source_modules_processed': len(selected),
        'global_pdb_archive': '../../_pdb/C/', 'repository_source_files': len(observed),
        'generated_binding_files': sum(e['source_kind'] == 'generated_binding' for e in entries.values()),
        'cargo_manifest_contents_recovered': False, 'implementation_recovered': False})
    atomic_json(meta / 'report.json', report)
    verify(obj, selected, entries, report)
    readme(dest, report, unlocated)

def verify(obj, selected, entries, report):
    actual = defaultdict(set)
    counts = Counter()
    for relative in entries:
        source = ROOT / VERSION / relative
        assert source.is_file() and source.stat().st_size > 0
        summary = json.loads(source.with_suffix('.rs.pdb.json').read_text())
        assert summary['checksums'] == entries[relative]['source_sha256']
        n = 0
        with gzip.open(Path(str(source) + '.pdb.jsonl.gz'), 'rt') as f:
            for line in f:
                row = json.loads(line); n += 1
                if row.get('class') == 'procedure' and row.get('source_mapping') == 'line_table_overlap':
                    actual[(row['module'], row['record_offset'])].add(relative)
        assert n == summary['metadata_records']
    # Independently inspect every selected physical procedure against all its
    # line contributions, including multiple equal-start source file blocks.
    expected = defaultdict(set)
    for module in selected:
        mi = module['index']; checks = {}; lines = []; procs = []
        archive = ROOT / '_pdb/C/modules' / f'{mi:05d}.jsonl.gz'
        with gzip.open(archive, 'rb') as f:
            for line in f:
                if b'"event":"checksum"' in line:
                    r = json.loads(line); checks[r['file_id']] = obj.source_mapper(r.get('path', ''))
                elif b'"event":"lines"' in line: lines.append(json.loads(line))
                elif b'"class":"procedure"' in line: procs.append(json.loads(line))
        for proc in procs:
            a = proc['section_offset']; b = a + proc['code_size']
            for block in lines:
                target = checks.get(block['file_id'])
                if target not in entries or block['section'] != proc['section']: continue
                if a < block['section_offset'] + block['code_size'] and block['section_offset'] < b:
                    expected[(mi, proc['record_offset'])].add(target)
    assert dict(expected) == dict(actual)
    total = sum(len(paths) for paths in expected.values())
    assert total == report['mapping_counts']['procedure_line_overlap']
    counts.update({'source_files': len(entries), 'physical_procedures': len(actual), 'procedure_file_links': total})
    # Check that all formerly missing symbols sourced within rhapsody2 are now visible.
    causes = json.loads((ROOT / '_pdb/C/unfilled-source-path-causes.json').read_text())
    eligible = {r['name'] for r in causes['records'] if any(obj.source_mapper(h['path']) for h in r['source_candidates'])}
    names = {name for relative in entries for name in re.findall(r'^// 函数符号: (.+)$', (ROOT / VERSION / relative).read_text(), re.M)}
    assert eligible <= names
    atomic_json(obj.dir / 'verification.json', {'ok': True, 'counts': counts,
        'repository_paths_and_checksums_complete': True, 'all_physical_procedure_source_links_checked': True,
        'all_file_metadata_records_counted': True, 'formerly_unfilled_distinct_names_now_present': len(eligible),
        'previously_missing_eligible_names_remaining': 0, 'implementation_verified': False,
        'pdb_sha256': obj.initial_source_hash})
    print(json.dumps({'verification_ok': True, 'counts': counts, 'formerly_unfilled_names_now_present': len(eligible)}), flush=True)

def readme(dest, report, unlocated):
    cargo_note = ('Cargo.toml 已依据 C 原锁文件补齐依赖边和版本/commit；具体来源、未知 features 与 build/target 分类见 `_metadata/CARGO_RECONSTRUCTION.md`。'
        if (dest / 'Cargo.toml').stat().st_size else
        'Cargo.toml 是空占位，PDB 无完整原清单、features 或 workspace 配置。')
    text = f'''# C 版 rhapsody2 PDB 骨架

仓库：`{REPOSITORY}`；原作 C 锁定 commit：`{report['revision']}`。

已按 PDB 源路径创建 {report['repository_source_files']} 个仓库源文件，以及 {report['generated_binding_files']} 个相关生成文件；写入 {report['logical_symbol_and_type_pairs']} 个按文件归属的函数符号/类型条目。函数实现尚未恢复；缺失的参数和返回类型仍标为 unknown。

## 目录

- `src/`：rhapsody 主包，保留 raw、rg 等原目录。
- `render-std/`：core、DOF、GI、post、shadow、TAA 等 PDB 记录的包。
- `crates/`：原仓库附属包，包括 GPU profiler、allocator、同步和诊断接口。
- `_generated/target/`：6 个原 OUT_DIR 的 shabriri.rs，保留原构建路径与校验和。
- `_unlocated-packages/`：锁文件有包、PDB 无源路径的条目（{', '.join(p['name'] for p in unlocated) or '无'}）；该目录不是推断的作者原目录。
- `_metadata/`：仓库/锁定依赖身份、文件清单、报告及全量来源映射验证。

各 `.rs` 使用注释保存函数名、泛型/闭包作用域、TPI/IPI、源码行、地址与类型引用图。同名 `.rs.pdb.jsonl.gz` 保存逐条行表、内联操作、原始符号、公开别名与编译上下文。同名 `.rs.pdb.json` 保存文件摘要。

完整原 PDB、类型、公共符号和全部模块记录仍复用 `skeleton/_pdb/C/`，未重复提取或更改。{cargo_note} 本目录是明确授权的依赖元数据骨架，不集成到 `reconstruction/`，也不计作游戏业务或渲染实现恢复。

复现入口：`skeleton/_tools/restore_rhapsody2.py`。覆盖核对见 `_metadata/verification.json`。
'''
    (dest / 'README.md').write_text(text)

if __name__ == '__main__': main()
