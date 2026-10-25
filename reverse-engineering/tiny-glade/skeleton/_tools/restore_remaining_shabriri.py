"""Materialize C's remaining generated bindings from cached PDB records."""
from collections import defaultdict
import gzip
import json
import re
from pathlib import Path
from restore_pdb import ROOT, PROJECT, Version, atomic_json, body, digest

PATTERN = re.compile(r'country-slice-private/(target/[^\n]+/release/build/((?:country-core|clutter-icon-pipe)-[^/]+)/out/shabriri\.rs)$')

def map_source(path):
    m = PATTERN.search(path.replace('\\', '/'))
    return '_generated/' + m[1] if m else None

def main():
    cache = ROOT / '_pdb/C'; dest = ROOT / 'C/_generated'; meta = dest / '_metadata'
    meta.mkdir(parents=True, exist_ok=True)
    baseline = json.loads((PROJECT / 'tracking/baselines.json').read_text())['baselines']['C']
    assert digest(cache / 'original.pdb') == baseline['pdb']['sha256']
    modules = json.loads((cache / 'modules.json').read_text())
    selected = [m for m in modules if any(map_source(f) for f in m['files'])]
    entries = {}
    for module in selected:
        with gzip.open(cache / 'modules' / f"{module['index']:05d}.jsonl.gz", 'rb') as f:
            for line in f:
                if b'"event":"checksum"' not in line: continue
                row = json.loads(line); original = row.get('path', ''); path = map_source(original)
                if path is None: continue
                entry = entries.setdefault(path, {'original_path': path, 'original_pdb_paths': set(),
                    'source_sha256': set(), 'version': 'C', 'kind': 'generated_metadata_source',
                    'source_kind': 'generated_shabriri_binding', 'dependency_class': 'game_generated_glue_metadata_only'})
                assert row['algorithm'] == 3
                entry['source_sha256'].add(row['checksum']); entry['original_pdb_paths'].add(original)
    assert len(entries) == 2
    for entry in entries.values():
        for key in ('source_sha256', 'original_pdb_paths'): entry[key] = sorted(entry[key])
        path = ROOT / 'C' / entry['original_path']; path.parent.mkdir(parents=True, exist_ok=True)
        if not path.exists(): path.touch()
    atomic_json(dest / 'file-manifest.json', {'files': list(entries.values())})
    obj = Version('C', baseline, {}, source_mapper=map_source, source_entries=entries, report_dir=meta)
    obj.initial_source_hash = baseline['pdb']['sha256']
    obj.complete = json.loads((cache / 'report.json').read_text())['native_completion']
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
    report['scope'] = 'Remaining C generated shabriri bindings; metadata only'
    report['source_modules_processed'] = len(selected)
    atomic_json(meta / 'report.json', report)
    from restore_rhapsody2 import verify
    verify(obj, selected, entries, report)
    text = f'''# C 版剩余 shabriri 生成绑定

来自原 PDB 的 country-core 和 clutter-icon-pipe OUT_DIR 文件，保留原 `target/.../out/shabriri.rs` 结构与源校验和。共 {report['annotated_files']} 个文件、{report['logical_symbol_and_type_pairs']} 个函数符号/类型条目。

`.rs` 保存函数身份、源码行、原地址及完整可用 TPI/IPI 引用图，含泛型/闭包作用域与编译上下文。同名压缩附档保留逐条行表、内联操作、符号字节和展开类型记录。完整 PDB 复用 `skeleton/_pdb/C/`。

另外 6 个 rhapsody2 生成文件在 `C/rhapsody2/_generated/`；本目录补齐 C PDB 中余下的两个。缺失的参数/返回类型、字段布局和函数体仍明确标注，不冒充作者生成器输出或可运行实现。

核对结果见 `_metadata/verification.json`。原始生成器与 Cargo build.rs 尚未恢复，此目录不合并进 reconstruction。
'''
    (dest / 'README.md').write_text(text)

if __name__ == '__main__': main()
