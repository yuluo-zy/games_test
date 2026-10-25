"""Read the existing task database and index evidence shared by all A/B/C."""
from collections import Counter, defaultdict
import csv
from datetime import datetime, timezone
import gzip
import hashlib
import itertools
import json
from pathlib import Path
import sqlite3

ROOT = Path(__file__).resolve().parents[1]
INDEX = ROOT / 'task-index'
OUT = INDEX / 'ABC-三版共同项'
CATEGORIES = {
    'tools': '01-工具与低复杂度候选',
    'business': '02-常规业务与待补证',
    'core': '03-核心枢纽与高影响',
}
RANK = {'tools': 1, 'business': 2, 'core': 3}
PAIRS = {('A', 'B'), ('A', 'C'), ('B', 'C')}


def fingerprint(path):
    digest = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b''):
            digest.update(block)
    return digest.hexdigest()


def equal_nonempty(variants, field):
    values = [set(variants[v][field]) for v in 'ABC']
    return bool(values[0]) and values[0] == values[1] == values[2]


def save_json(path, value):
    temp = path.with_suffix(path.suffix + '.tmp')
    temp.write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n')
    temp.replace(path)


def escape(value):
    return str(value).replace('|', '\\|').replace('\n', ' ')


def build():
    database = INDEX / 'index.sqlite3'
    before = fingerprint(database)
    groups = defaultdict(dict)
    with sqlite3.connect(database.as_uri() + '?mode=ro', uri=True) as conn:
        conn.execute('BEGIN')
        for raw, in conn.execute('SELECT payload FROM functions WHERE category IS NOT NULL'):
            fn = json.loads(raw)
            members = groups[fn['logical_group_id']]
            assert fn['version'] not in members, 'duplicate version in logical group'
            members[fn['version']] = fn
        files = {}
        for raw, in conn.execute('SELECT payload FROM files'):
            file = json.loads(raw)
            files[file['version'], file['path']] = file
        tasks = {task_id: json.loads(raw) for task_id, raw in conn.execute('SELECT id,payload FROM tasks')}
    assert before == fingerprint(database), 'task database changed during snapshot; rebuild view'

    rows = []
    all_three_eligible = 0
    for group_id, variants in groups.items():
        if set(variants) != set('ABC'):
            continue
        all_three_eligible += 1
        assert len({fn['path'] for fn in variants.values()}) == 1
        assert len({fn['name'] for fn in variants.values()}) == 1
        source_equal = equal_nonempty(variants, 'source_sha256')
        native_equal = equal_nonempty(variants, 'body_hashes')
        # Recompute from actual sets, and check every stored comparison in every version.
        for fn in variants.values():
            pairs = {tuple(pair['versions']): pair for pair in fn['cross_version_pairs']}
            assert set(pairs) == PAIRS
            for a, b in itertools.combinations('ABC', 2):
                pair = pairs[a, b]
                for field, flag in [('source_sha256', 'same_file_source_checksum_set'),
                                    ('body_hashes', 'same_native_byte_sets')]:
                    left, right = set(variants[a][field]), set(variants[b][field])
                    assert pair[flag] == (bool(left) and left == right)
        if not source_equal and not native_equal:
            continue
        path = variants['C']['path']
        file_variants = {v: files[v, path] for v in 'ABC'}
        assert all(not f['excluded_reason'] for f in file_variants.values())
        task_ids = {f['task_id'] for f in file_variants.values()}
        assert len(task_ids) == 1
        task_id = task_ids.pop()
        task = tasks[task_id]
        assert all(fn['id'] in task['function_ids'] for fn in variants.values())
        category = max((fn['category'] for fn in variants.values()), key=RANK.get)
        rows.append({
            'logical_group_id': group_id, 'source_path': path, 'function_name': variants['C']['name'],
            'category': category, 'version_categories': {v: variants[v]['category'] for v in 'ABC'},
            'same_native_byte_sets_ABC': native_equal,
            'same_file_source_checksum_set_ABC': source_equal,
            'evidence_kind': 'source_and_native' if source_equal and native_equal else 'native_only' if native_equal else 'source_file_only',
            'behavior_equivalence_verified': False, 'abi_equivalence_verified': False,
            'file_task_id': task_id, 'file_task_category': task['category'], 'target_version': 'C',
            'claim_mode': task['claim_mode'], 'deferred': task['claim_mode'] == 'deferred',
            'priority_score': max(fn['priority_score'] for fn in variants.values()),
            'function_ids': {v: variants[v]['id'] for v in 'ABC'},
            'file_metadata': {v: {'source': file_variants[v]['source'], 'metadata_path': file_variants[v]['metadata_path']} for v in 'ABC'},
            'variants': variants,
        })
    rows.sort(key=lambda row: (RANK[row['category']], not row['same_native_byte_sets_ABC'],
                              row['deferred'], -row['priority_score'], row['source_path'], row['function_name']))
    assert len(rows) == len({row['logical_group_id'] for row in rows})
    counts = {}
    for category in CATEGORIES:
        selected = [row for row in rows if row['category'] == category]
        counts[category] = {
            'union': len(selected),
            'same_native_byte_sets_ABC': sum(row['same_native_byte_sets_ABC'] for row in selected),
            'same_file_source_checksum_set_ABC': sum(row['same_file_source_checksum_set_ABC'] for row in selected),
            'both': sum(row['same_native_byte_sets_ABC'] and row['same_file_source_checksum_set_ABC'] for row in selected),
            'source_file_only': sum(row['evidence_kind'] == 'source_file_only' for row in selected),
            'deferred': sum(row['deferred'] for row in selected),
        }
        count = counts[category]
        assert count['union'] == count['same_native_byte_sets_ABC'] + count['same_file_source_checksum_set_ABC'] - count['both']
    totals = dict(Counter({key: sum(count[key] for count in counts.values()) for key in next(iter(counts.values()))}))
    summary = {
        'generated_at_utc': datetime.now(timezone.utc).isoformat(), 'input_database_sha256': before,
        'all_three_eligible_logical_groups': all_three_eligible, 'categories': counts, 'total': totals,
        'matching_key': 'same source path and exact canonical symbol; generic instantiations preserved',
        'category_policy': 'highest function category among A/B/C; file task may have higher category',
        'source_scope': 'whole original source-file checksum set, not isolated function text',
        'native_scope': 'all observed standalone native body SHA-256 sets; missing bodies do not match',
        'behavior_equivalence_verified': False, 'abi_equivalence_verified': False,
        'new_claims_created': False, 'production_project_modified': False,
    }
    OUT.mkdir(parents=True, exist_ok=True)
    temp = OUT / 'functions.jsonl.gz.tmp'
    with gzip.open(temp, 'wt', encoding='utf-8') as stream:
        for row in rows:
            stream.write(json.dumps(row, ensure_ascii=False, separators=(',', ':')) + '\n')
    temp.replace(OUT / 'functions.jsonl.gz')
    fields = ['logical_group_id', 'source_path', 'function_name', 'category', 'evidence_kind',
              'same_native_byte_sets_ABC', 'same_file_source_checksum_set_ABC', 'file_task_id',
              'file_task_category', 'claim_mode', 'priority_score']
    fields += [f'{v}_{field}' for v in 'ABC' for field in ['function_id', 'category', 'callers', 'cross_file_callers', 'native_bytes', 'body_hashes', 'source_checksums']]
    fields += ['behavior_equivalence_verified', 'abi_equivalence_verified']
    for category, title in CATEGORIES.items():
        selected = [row for row in rows if row['category'] == category]
        with (OUT / (title + '.csv')).open('w', newline='', encoding='utf-8-sig') as stream:
            writer = csv.DictWriter(stream, fieldnames=fields)
            writer.writeheader()
            for row in selected:
                record = {key: row[key] for key in fields if key in row}
                for v in 'ABC':
                    fn = row['variants'][v]
                    record.update({f'{v}_function_id': fn['id'], f'{v}_category': fn['category'],
                                   f'{v}_callers': fn['metrics']['unique_indexed_callers'],
                                   f'{v}_cross_file_callers': fn['metrics']['cross_file_callers'],
                                   f'{v}_native_bytes': ';'.join(map(str, fn['body_sizes'])),
                                   f'{v}_body_hashes': ';'.join(fn['body_hashes']),
                                   f'{v}_source_checksums': ';'.join(fn['source_sha256'])})
                writer.writerow(record)
        count = counts[category]
        lines = [f'# A/B/C 三版共同项：{title}', '',
                 f"三版同码 {count['same_native_byte_sets_ABC']} 组；仅三版同源文件 {count['source_file_only']} 组；合并去重 {count['union']} 组。",
                 '', f'完整机器索引：[CSV]({title}.csv)；[证据与领取说明](README.md)。类别取三版函数分类中的最高影响级别。', '']
        for same_native, heading in [(True, '三版观察机器码集合相同'), (False, '仅三版所属源码文件相同，机器码未证相同')]:
            subset = [row for row in selected if row['same_native_byte_sets_ABC'] == same_native]
            lines += [f'## {heading}（{len(subset)} 组）', '',
                      '| 函数 / C 源文件 | 三版同源文件 | A/B/C 调用者 | 领取任务 / 状态 | C 函数 ID |',
                      '|---|---|---|---|---|']
            for row in subset:
                callers = '/'.join(str(row['variants'][v]['metrics']['unique_indexed_callers']) for v in 'ABC')
                lines.append(f"| {escape(row['function_name'])}<br>[{row['source_path']}](../../C/{row['source_path']}) | {'是' if row['same_file_source_checksum_set_ABC'] else '未证'} | {callers} | [{row['file_task_id']}](../tasks/{row['file_task_id']}.json)<br>{row['claim_mode']} | `{row['function_ids']['C']}` |")
            lines.append('')
        (OUT / (title + '.md')).write_text('\n'.join(lines) + '\n')
    navigation = ['# A/B/C 三版共同函数索引', '',
                  '只收录三个版本都存在、同路径且同符号，并满足三版完整源校验集合相同或三版观察过程字节集合相同的游戏候选函数。不会把仅 A=B 或 B=C 的记录混入此表。', '',
                  '| 类别 | 三版同码 | 三版同源文件 | 两者均满足 | 仅同源文件 | 合并去重 | 完整清单 |',
                  '|---|---:|---:|---:|---:|---:|---|']
    for category, title in CATEGORIES.items():
        count = counts[category]
        navigation.append(f"| {title[3:]} | {count['same_native_byte_sets_ABC']} | {count['same_file_source_checksum_set_ABC']} | {count['both']} | {count['source_file_only']} | {count['union']} | [全部函数]({title}.md) / [CSV]({title}.csv) |")
    navigation += [f"| 总计 | {totals['same_native_byte_sets_ABC']} | {totals['same_file_source_checksum_set_ABC']} | {totals['both']} | {totals['source_file_only']} | {totals['union']} | |", '',
                   '“三版同码”指每版所有已观察独立函数体的 SHA-256 集合相同；“三版同源文件”指 PDB 中整个原源文件的校验集合相同。两列有重叠。相同字节不保证跨版 ABI、地址指向、依赖、布局或行为等价；同源文件也不证明生成机器码相同。仅名字相同、部分实例相同或缺失字节范围不计入同码。', '',
                   '标准库、已确认公共依赖和通用容器继续排除。GPU/renderer 后置项保留 `deferred` 标记。类别取 A/B/C 函数中的最高影响级别，与包含它的整文件类别可能不同。每类 Markdown 均列出全部函数，不截断。', '',
                   '沿用原文件组 task ID 和 C 函数 ID，A/B/C 共用一个领取锁；这是原任务队列的筛选视图，没有新建重复任务或领取任何任务。先查看函数依赖、签名边界、已有 Rust 候选与保留 B 证据，再领取：', '',
                   '```sh',
                   'TG_PY=/Users/yuluo/Desktop/game/tools/mcp-servers/ghidra-mcp/.venv/bin/python',
                   'TG_QUEUE=/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/skeleton/_tools/task_queue.py',
                   '"$TG_PY" "$TG_QUEUE" show-function <C_FUNCTION_ID>',
                   '"$TG_PY" "$TG_QUEUE" claim <FILE_TASK_ID> --agent <UNIQUE_AGENT_NAME> --function <C_FUNCTION_ID>',
                   '```', '',
                   '[functions.jsonl.gz](functions.jsonl.gz) 按逻辑组保存完整 A/B/C 函数记录，包括原生范围/字节哈希、caller/callee ID、公库依赖、已有 Rust 候选、分类原因及原文件元数据路径。现有 `.rs.dependencies.json` / `.rs.pdb.jsonl.gz` 保留在各版骨架文件旁。', '',
                   '[summary.json](summary.json) 保存类别计数和输入 SQLite SHA-256；[verification.json](verification.json) 保存三版成员、完整匹配集合、任务复用和落盘覆盖核验。', '',
                   '主索引刷新时自动重建此视图；单独刷新可执行 `skeleton/_tools/build_abc_common_index.py`，仅读取已有数据库，不重新分析二进制。', '']
    (OUT / 'README.md').write_text('\n'.join(navigation))
    save_json(OUT / 'summary.json', summary)

    # Check delivered artifacts, not only the in-memory selection.
    with gzip.open(OUT / 'functions.jsonl.gz', 'rt') as stream:
        written = [json.loads(line) for line in stream]
    assert [r['logical_group_id'] for r in written] == [r['logical_group_id'] for r in rows]
    csv_counts = {}
    for category, title in CATEGORIES.items():
        with (OUT / (title + '.csv')).open(encoding='utf-8-sig') as stream:
            delivered = list(csv.DictReader(stream))
        expected = {r['logical_group_id'] for r in rows if r['category'] == category}
        assert len(delivered) == len(expected)
        assert {r['logical_group_id'] for r in delivered} == expected
        markdown = (OUT / (title + '.md')).read_text()
        assert all(f"`{row['function_ids']['C']}`" in markdown for row in rows if row['category'] == category)
        csv_counts[category] = len(delivered)
    save_json(OUT / 'verification.json', {
        'ok': True, 'input_database_sha256': before, 'eligible_ABC_groups_checked': all_three_eligible,
        'selected_groups': len(rows), 'csv_groups_by_category': csv_counts,
        'checks': ['nonempty hash/checksum sets recomputed for all three versions',
                   'AB, AC, BC pair flags independently checked in every version record',
                   'exact path and symbol identity; all versions eligible and non-excluded',
                   'one reused file task with all three function IDs',
                   'no duplicate group; category highest of A/B/C',
                   'full gzip/CSV/Markdown coverage verified after write'],
        'behavior_equivalence_verified': False, 'new_claims_created': False,
    })
    print(json.dumps({'ABC_common_categories': counts, 'total': totals}, ensure_ascii=False))
    return summary


if __name__ == '__main__':
    build()
