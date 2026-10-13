"""Refresh source-structured progress; never run game tests or promote acceptance.

Maintained claims live in tracking/module-progress-overrides.json. Generated
outputs are a complete PDB file inventory plus module views. Rust declarations
are an implementation inventory, not evidence of original function equivalence.
"""
from __future__ import annotations

import argparse
import datetime as dt
import fnmatch
import hashlib
import json
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CONFIG = ROOT / 'tracking/module-progress-overrides.json'
OUTPUT = ROOT / 'tracking/module-progress.json'
REPORT = ROOT / 'MODULE_PROGRESS.md'
DETAILS = ROOT / 'tracking/module-progress'
LABELS = {
    'not_assessed': '未逐项评估', 'analysis_partial': '部分分析',
    'draft_partial': '部分草稿', 'candidate_partial': '部分候选实现',
    'integrated_partial': '部分接线', 'reported_missing': '记录有交付，本机缺失',
    'dependency_boundary': '依赖/归属边界', 'metadata_only': '构建元数据',
    'verified_module': '模块限定版本验收通过',
}


def read_json(path):
    return json.loads(path.read_text(encoding='utf-8-sig'))


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def localize(value):
    value = str(value).replace('\\', '/')
    for marker in ('/reverse-engineering/tiny-glade/',):
        if marker in value:
            return value.split(marker, 1)[1]
    if value.startswith(str(ROOT).replace('\\', '/') + '/'):
        return value[len(str(ROOT).replace('\\', '/')) + 1:]
    return value


def module_id(source):
    parts = source.split('/')
    if source.startswith('crates/country-core/src/'):
        if len(parts) > 5 and parts[3] in ('systems', 'resources'):
            return '/'.join(parts[:5])
        return '/'.join(parts[:4]) if len(parts) > 4 else 'crates/country-core/src'
    # 原嵌套 shader-pipeline 是独立 crate，格式/消费合同按原五个文件单列。
    if source.startswith('crates/rhapsody/crates/shader-pipeline/src/'):
        return 'crates/rhapsody/crates/shader-pipeline/src'
    if source.startswith('crates/rhapsody/src/'):
        if source.startswith('crates/rhapsody/src/raw/managed/'):
            return 'crates/rhapsody/src/raw/managed'
        return '/'.join(parts[:4]) if len(parts) > 4 else 'crates/rhapsody/src'
    if source.startswith('crates/systems/wall-constructor/src/'):
        return '/'.join(parts[:5]) if len(parts) > 5 else '/'.join(parts[:4])
    if source.startswith('crates/systems/'):
        return '/'.join(parts[:4])
    if source.startswith('crates/utils/src/'):
        return '/'.join(parts[:4]) if len(parts) > 4 else 'crates/utils/src'
    return '/'.join(parts[:3]) if source.startswith('crates/') else parts[0]


def rust_inventory(path):
    source = path.read_text(encoding='utf-8-sig', errors='replace')
    # Omit conventional trailing test modules. This is deliberately a lexical
    # inventory; macro-generated methods and full impl/signature parsing are unknown.
    production = re.split(r'#\[cfg\(test\)\]', source, maxsplit=1)[0]
    declarations = []
    for match in re.finditer(r'(?m)^\s*(?:pub(?:\([^\n)]*\))?\s+)?(?:unsafe\s+)?(?:async\s+)?(?:const\s+)?(?:extern\s+"[^\n"]+"\s+)?fn\s+(\w+)', production):
        line = production.count('\n', 0, match.start()) + 1
        # Whitespace in the regex can include the preceding newline.
        declaration_line = production.count('\n', 0, match.end()) + 1
        declarations.append({'name': match.group(1), 'line': declaration_line,
                             'original_correspondence': 'not_established_by_scan',
                             'equivalence': 'not_inferred'})
    gaps = []
    for line_number, line in enumerate(source.splitlines(), 1):
        if re.search(r'TODO|todo!\s*\(|unimplemented!\s*\(|\bunknown\b|typed.?gap', line, re.I):
            gaps.append({'line': line_number, 'text': line.strip()[:240]})
    return {'sha256': digest(path), 'rust_declarations': declarations,
            'gap_marker_count': len(gaps), 'gap_markers': gaps[:30],
            'scan_limit': '词法定义清单；不覆盖宏展开，不证明原名、ABI或行为；最多展示30条缺口注释'}


def resolve_hash_path(raw_path, report_path):
    relative = localize(raw_path)
    if Path(relative).is_absolute() or re.match(r'^[A-Za-z]:/', relative):
        return None
    candidates = [ROOT / relative]
    # Reports under a crate usually record src/..., Cargo.lock or scripts/... .
    for parent in report_path.parents:
        if parent == ROOT:
            break
        if (parent / 'Cargo.toml').exists():
            candidates.append(parent / relative)
    for candidate in candidates:
        if candidate.is_file():
            return candidate
    return None


def evidence_info(relative):
    relative = localize(relative)
    path = ROOT / relative
    result = {'path': relative, 'exists': path.is_file(),
              'freshness': 'not_recorded', 'checked_path_hashes': [],
              'scope': '沿用报告限定范围；本次未重跑业务验收'}
    if not path.is_file():
        result['freshness'] = 'missing_report'
        return result
    result['report_sha256'] = digest(path)
    if path.suffix != '.json':
        return result
    try:
        document = read_json(path)
    except (ValueError, OSError):
        result['freshness'] = 'invalid_json'
        return result
    if isinstance(document, dict):
        result['reported_status'] = {k: document[k] for k in ('status', 'ok', 'result', 'verification', 'limits', 'coverage_limits')
                                     if k in document and (not isinstance(document[k], (list, dict)) or k in ('verification', 'limits', 'coverage_limits'))}
    checks = []

    def visit(value):
        if isinstance(value, dict):
            if isinstance(value.get('path'), str) and isinstance(value.get('sha256'), str) and re.fullmatch(r'[0-9a-fA-F]{64}', value['sha256']):
                resolved = resolve_hash_path(value['path'], path)
                checks.append({'recorded_path': value['path'],
                               'resolved_path': resolved.relative_to(ROOT).as_posix() if resolved and resolved.is_relative_to(ROOT) else None,
                               'state': 'matched' if resolved and digest(resolved) == value['sha256'].lower() else 'changed' if resolved else 'unresolved'})
            for key, item in value.items():
                if (isinstance(item, str) and re.fullmatch(r'[0-9a-fA-F]{64}', item)
                        and re.search(r'\.(?:rs|lock|toml|py|mjs|json|hlsl|exe|rne)$', key)):
                    resolved = resolve_hash_path(key, path)
                    checks.append({'recorded_path': key,
                                   'resolved_path': resolved.relative_to(ROOT).as_posix() if resolved and resolved.is_relative_to(ROOT) else None,
                                   'state': 'matched' if resolved and digest(resolved) == item.lower() else 'changed' if resolved else 'unresolved'})
                visit(item)
        elif isinstance(value, list):
            for item in value:
                visit(item)
    visit(document)
    result['checked_path_hashes'] = checks
    if any(c['state'] == 'changed' for c in checks):
        result['freshness'] = 'historical_inputs_changed'
    elif checks and all(c['state'] == 'matched' for c in checks):
        result['freshness'] = 'explicit_path_hashes_matched'
    elif checks:
        result['freshness'] = 'partly_unresolved'
    return result


def escape(value):
    return str(value).replace('|', '\\|').replace('\n', '<br>')


def shader_inventory():
    commands_path = ROOT / 'evidence/render-passes/validation/compile-commands.json'
    commands = read_json(commands_path)['compile_commands']
    rows = {}
    for command in commands:
        for original in command['embedded_source_paths']:
            row = rows.setdefault(original, {'original_path': original, 'baseline': 'B',
                'restoration': 'reconstruction/shader-source/' + original,
                'original_bins': [], 'original_entries': [], 'validation_reports': [],
                'verification': 'reports_only; freshness_not_established; no_live_shader_equivalence',
                'gaps': ['语义/指纹残差与真实资源绑定/GPU消费者需逐项关闭']})
            if original == command['main_source_path']:
                row['original_bins'].append(command['file'])
                for module in command['modules']:
                    entry = {'name': module['entry'], 'stage': module['stage'], 'profile': module['profile']}
                    if entry not in row['original_entries']:
                        row['original_entries'].append(entry)
    for row in rows.values():
        path = ROOT / row['restoration']
        row['exists'] = path.is_file()
        row['ownership'] = 'public_dependency_boundary' if re.search(r'XeGTAO|vaShared|ffx_|FidelityFX', row['original_path'], re.I) else 'shader_game_candidate_or_shared_helper'
        row['role'] = 'main' if row['original_bins'] else 'include_or_embedded_source'
        if path.is_file():
            row['sha256'] = digest(path)
            source = path.read_text(encoding='utf-8', errors='replace')
            row['candidate_function_definitions'] = [
                {'name': m.group(1), 'line': source.count('\n', 0, m.end()) + 1,
                 'original_name': 'not_proven_for_inlined_helpers'}
                for m in re.finditer(r'(?m)^\s*(?:(?:static|inline|export|precise)\s+)*(?:[\w:]+(?:<[^\n>]+>)?)\s+(\w+)\s*\([^;{}]*\)\s*(?::[^\n{]+)?\s*\{', source)
                if m.group(1) not in ('if','for','while','switch')]
        else:
            row['gaps'].insert(0, '嵌入路径对应本机源缺失；不计实现')
    # Retain all directly matched reports; no filename-order or mtime "latest"
    # promotion, and no fingerprint PASS is promoted to behavior equivalence.
    for directory in ('evidence/render-passes/integration', 'evidence/render-passes/recovery'):
        for path in sorted((ROOT / directory).rglob('*.json')):
            try:
                report = read_json(path)
            except (ValueError, OSError):
                continue
            if not isinstance(report, dict) or not isinstance(report.get('hlsl'), str):
                continue
            relative = localize(report['hlsl'])
            original = relative.removeprefix('reconstruction/shader-source/')
            if original in rows and relative == rows[original]['restoration']:
                rows[original]['validation_reports'].append({
                    'path': path.relative_to(ROOT).as_posix(),
                    'report_sha256': digest(path),
                    'summary': report.get('summary'), 'freshness': 'not_recorded',
                    'scope': 'recompile structural report; not runtime equivalence'})
    return {'source': commands_path.relative_to(ROOT).as_posix(),
            'input_sha256': digest(commands_path), 'files': list(rows.values())}


def link(path, from_dir=ROOT, line=None):
    # Relative links keep checked-in generated docs usable after moving hosts.
    import os
    relative = Path(os.path.relpath(ROOT / path, from_dir)).as_posix()
    suffix = f'#L{line}' if line else ''
    return f'[{escape(path)}]({relative}{suffix})'


def build():
    config = read_json(CONFIG)
    interface_path = config.get('interface_progress_source')
    interface_doc = read_json(ROOT / interface_path) if interface_path else {}
    business_path = config.get('business_function_progress_source')
    business_doc = read_json(ROOT / business_path) if business_path else {}
    inventory = read_json(ROOT / 'tracking/version-map.json')
    notes = {row['module']: row for row in config['modules']}
    files = {row['path']: {'original_path': row['path'], 'versions': row['versions'],
                          'module': module_id(row['path']), 'source_coverage': 'not_assessed',
                          'restoration_refs': [], 'known_original_functions': [],
                          'gaps': ['原文件函数全集与恢复覆盖尚未逐项核对；不按文件存在判完成'],
                          'verification': 'not_file_level_verified', 'verification_scopes': []} for row in inventory['source_files']}
    restorations = {}
    module_files = {}
    evidence_cache = {}
    issues = []

    for note in config['modules']:
        module = note['module']
        module_files.setdefault(module, [])
        for pattern in note.get('restoration_globs', []):
            matched = sorted(ROOT.glob(pattern))
            if not matched:
                issues.append({'module': module, 'kind': 'missing_recorded_restoration', 'path': pattern})
            for path in matched:
                if path.is_file() and path.suffix == '.rs':
                    relative = path.relative_to(ROOT).as_posix()
                    if relative not in restorations:
                        restorations[relative] = {'path': relative, 'modules': [], **rust_inventory(path)}
                    restorations[relative]['modules'].append(module)
                    module_files[module].append(relative)
        for evidence in note.get('evidence', []):
            evidence_cache.setdefault(evidence, evidence_info(evidence))

    def bind(original, recovered, method, symbols=None):
        if original not in files:
            issues.append({'kind': 'original_path_not_in_inventory', 'path': original, 'restoration': recovered})
            return
        recovered = localize(recovered)
        exists = (ROOT / recovered).is_file()
        row = files[original]
        if exists and recovered not in restorations:
            restorations[recovered] = {'path': recovered, 'modules': [row['module']], **rust_inventory(ROOT / recovered)}
        if exists:
            module_files.setdefault(row['module'], []).append(recovered)
        row['restoration_refs'].append({'path': recovered, 'exists': exists, 'mapping_basis': method,
                                        'scope': 'partial_or_unknown; not whole-file equivalence'})
        row['source_coverage'] = 'candidate_present' if any(r['exists'] for r in row['restoration_refs']) else 'recorded_missing'
        if symbols:
            row['known_original_functions'].extend(symbols)

    for binding in config.get('file_bindings', []):
        bind(binding['original'], binding['restoration'], binding['basis'])
        if binding.get('verification_scope') and binding['original'] in files:
            files[binding['original']]['verification_scopes'].append({
                'scope': binding['verification_scope'], 'evidence': binding.get('evidence', []),
                'acceptance': 'reported_scoped_only; currentness_separate; not whole-file acceptance'})
            files[binding['original']]['verification'] = 'reported_scoped_verification'
            for evidence in binding.get('evidence', []):
                evidence_cache.setdefault(evidence, evidence_info(evidence))
    # Preserved A/B/C candidate handoffs carry original symbols and source paths.
    for mapping_path in config.get('candidate_mappings', []):
        document = read_json(ROOT / mapping_path)
        for row in document:
            original = row.get('original_source_path', row.get('source_path'))
            symbol = {'name': row['original_symbol'], 'evidence': mapping_path,
                      'status': row.get('status', 'implemented_unverified'),
                      'versions': row.get('variants', {}),
                      'original_signature': 'not_verified', 'behavior_equivalence': False}
            bind(original, row['candidate_path'], 'explicit_candidate_delivery_mapping', [symbol])
    for metadata_path in config.get('original_function_sources', []):
        evidence_cache.setdefault(metadata_path, evidence_info(metadata_path))
        for procedure in read_json(ROOT / metadata_path).get('procedures', []):
            for candidate in procedure.get('source_candidates', []):
                original = candidate['original_path']
                if '/crates/' in original:
                    original = 'crates/' + original.split('/crates/', 1)[1]
                if original in files:
                    files[original]['known_original_functions'].append({
                        'name': procedure['name'], 'va': procedure.get('va'), 'baseline': 'B',
                        'evidence': metadata_path, 'status': 'original_located_source_candidate',
                        'original_signature': 'not_verified', 'behavior_equivalence': 'only_separate_scoped_reports'})
    for function in config.get('function_bindings', []):
        for evidence in function.get('evidence', []):
            evidence_cache.setdefault(evidence, evidence_info(evidence))
        original = function.get('original_source')
        if original in files:
            if not any(r['path'] == function['restoration'] for r in files[original]['restoration_refs']):
                bind(original, function['restoration'], 'explicit_scoped_function_binding')
            files[original]['known_original_functions'].append({
                'name': function['original_symbol'], 'status': function['status'],
                'baseline': function['baseline'], 'rust_function': function['rust_function'],
                'scope': function['scope'], 'evidence': function['evidence'],
                'original_signature': 'not_implied_by_local_verification'})
            files[original]['verification_scopes'].append({'scope': function['original_symbol'] + '：' + function['scope'],
                'evidence': function['evidence'], 'acceptance': function['status']})
            files[original]['verification'] = 'reported_scoped_verification'
    for module, paths in module_files.items():
        module_files[module] = sorted(set(paths))

    modules = []
    module_names = sorted({row['module'] for row in files.values()} | set(notes))
    for name in module_names:
        note = notes.get(name, {})
        original_files = [r for r in files.values() if r['module'] == name]
        if name.startswith('target'):
            default_status = 'metadata_only'
        else:
            default_status = 'candidate_partial' if module_files.get(name) else 'not_assessed'
        status = note.get('status', default_status)
        if status == 'not_assessed' and module_files.get(name):
            status = 'candidate_partial'
        module = {'module': name, 'status': status,
                  'interface_progress': interface_doc.get('modules', {}).get(name),
                  'business_function_progress': business_doc.get('libraries', {}).get(name.split('/')[1]) if name.startswith('crates/') and len(name.split('/'))==3 else None,
                  'ownership': note.get('ownership', 'private_source_candidate; ownership_requires_dependency_filter'),
                  'recovered_scope': note.get('scope', '仅有源码清单或候选交付；完整模块行为尚未评估'),
                  'versions': sorted({version for row in original_files for version in row['versions']}),
                  'original_files': [row['original_path'] for row in original_files],
                  'restoration_files': module_files.get(name, []),
                  'known_functions': note.get('functions', []),
                  'function_bindings': [f for f in config.get('function_bindings', []) if f['module'] == name],
                  'function_inventory_limit': 'known_functions为人工证据摘要；文件Rust定义另列，不是原函数全集或逐函数验收',
                  'integration': note.get('integration', 'not_established'),
                  'verification_summary': note.get('verification', '未建立模块/文件级验收；候选交付不等于原作等价'),
                  'evidence': note.get('evidence', []),
                  'gaps': note.get('gaps', ['原函数全集、类型/ABI、生产者/消费者和版本适配尚未核对']),
                  'next_action': note.get('next_action', '先确认定义归属与实际消费链，补齐本模块文件/函数覆盖，不自动启动逆向'),
                  'full_module_accepted': note.get('full_module_accepted', False),
                  'full_module_acceptance': note.get('full_module_acceptance')}
        if module['full_module_accepted']:
            acceptance = module['full_module_acceptance']
            assert acceptance and acceptance.get('baseline') and acceptance.get('scope') and acceptance.get('evidence'), 'explicit module acceptance needs version/scope/evidence'
            assert all((ROOT / p).is_file() for p in acceptance['evidence']), 'acceptance evidence missing'
            module['status'] = 'verified_module'
        modules.append(module)
    doc = {'schema_version': 1, 'updated_at': dt.datetime.now(dt.timezone(dt.timedelta(hours=8))).isoformat(),
           'authority': 'source_structured_progress', 'primary_baseline': 'B', 'delivery_target': 'C v1.16.0 (separate adaptation acceptance)',
           'overall_completion_percent': None, 'business_tests_run_by_refresh': False,
           'behavior_recovery_policy': config.get('behavior_recovery_policy', {}),
           'inventory_scope': 'PDB observed A/B/C paths, not complete author repository or function denominator',
           'inputs': {relative: digest(ROOT / relative) for relative in [
               'tracking/version-map.json', 'tracking/module-progress-overrides.json',
               'scripts/update-module-progress.py', *config.get('candidate_mappings', []),
               *config.get('original_function_sources', []), *([interface_path] if interface_path else []), *([business_path] if business_path else [])]},
           'modules': modules, 'files': list(files.values()), 'restorations': list(restorations.values()),
           'evidence': list(evidence_cache.values()), 'issues': issues,
           'shader_layer': config.get('shader_layer', {})}
    for evidence in doc['shader_layer'].get('evidence', []):
        if evidence not in evidence_cache:
            doc['evidence'].append(evidence_info(evidence))
    doc['shader_layer']['inventory'] = shader_inventory()
    validate(doc)
    return doc


def validate(doc):
    paths = [row['original_path'] for row in doc['files']]
    assert len(paths) == len(set(paths)), 'duplicate original path'
    assert sum(len(m['original_files']) for m in doc['modules']) == len(paths), 'file partition mismatch'
    assert all(not m['full_module_accepted'] or m['full_module_acceptance'] for m in doc['modules']), 'refresh cannot infer full acceptance'
    assert not doc['business_tests_run_by_refresh']
    for item in doc['restorations']:
        assert (ROOT / item['path']).is_file(), item['path']
        assert digest(ROOT / item['path']) == item['sha256'], f"source changed during refresh: {item['path']}"
    restoration_lookup = {r['path']: r for r in doc['restorations']}
    for module in doc['modules']:
        for function in module['function_bindings']:
            assert function['restoration'] in restoration_lookup, f"function source not inventoried: {function['restoration']}"
            assert function['rust_function'] in {f['name'] for f in restoration_lookup[function['restoration']]['rust_declarations']}, f"Rust function missing: {function['rust_function']}"
    shader = doc['shader_layer']['inventory']
    assert digest(ROOT / shader['source']) == shader['input_sha256'], 'shader inventory input changed'
    assert len({r['original_path'] for r in shader['files']}) == len(shader['files'])
    for row in shader['files']:
        if row['exists']:
            assert digest(ROOT / row['restoration']) == row['sha256'], f"shader source changed: {row['restoration']}"
        for report in row['validation_reports']:
            assert digest(ROOT / report['path']) == report['report_sha256'], f"shader report changed: {report['path']}"


def render(doc):
    DETAILS.mkdir(exist_ok=True)
    source_rows = {r['original_path']: r for r in doc['files']}
    restored = {r['path']: r for r in doc['restorations']}
    overview = ['# Tiny Glade 源码模块进度表', '', f"更新：{doc['updated_at']}。以原作PDB的crate、目录、文件为主键；B是主要恢复基线，C适配单独验收。", '',
                '这是后续逆向推进的统一入口。历史里程碑和批次报告保留为证据，不再作为任务划分或完成率依据。', '',
                '共享基础优先按[业务功能与当前源码对照](FOUNDATION_BUSINESS_COMPARISON.md)推进：库内逻辑缺失、原验证缺口和库外接线分别列出。[原函数接口库存](INTERFACE_PROGRESS.md)用于定位和映射；不要求原函数同名，等价组合实现也登记为已有代码，宏/getter不单列功能任务。', '',
                '用户重要指示：不要求源码级别1:1复刻。先分析真实业务逻辑，自主实现，再与原作实际规则对照、记录差异、修复并复验；源码写法与原Rust ABI不作为必须相同的目标。[业务对照流程](evidence/shared-foundation-2026-10-13/BEHAVIOR_WORKFLOW.md)。', '',
                '包级上下游与层次：[交互式包引用图](package-dependencies.html)。图使用B原锁依赖，模块表更新后运行 `py -3 scripts/build-package-graph.py` 刷新图中状态。', '',
                '实现、接线、原码验证、GPU验证分别登记；候选函数和适配器不等于作者原函数。完整验收只能由人工明确登记版本、范围和报告，刷新不会自动升格。本次刷新只核对清单、文件和显式路径哈希，不运行游戏测试。', '',
                '## 维护方式', '',
                '1. 先打开目标模块的明细，选择实际生产者→消费者闭包；不按孤立函数或新增candidate crate派工。',
                '2. 更新 [人工范围与映射](tracking/module-progress-overrides.json) 中的文件、函数、缺口、接线和证据；精确文件映射放入file_bindings，逐函数合同/证明放入function_bindings。原文件/原名未知时留空，不靠词法扫描补造。',
                '3. 运行 `py -3 scripts/update-module-progress.py` 生成总览、完整文件台账和模块明细；再运行 `py -3 scripts/update-module-progress.py --check` 检查输入和清单。',
                '4. 源码、依赖或验收输入变化时，仅重验受影响范围；保留旧报告，不改旧hash冒认当前通过。已匹配的显式路径hash也不证明报告绑定了全部输入。', '',
                '状态：部分分析/草稿/候选/接线仅描述已列范围；未逐项评估不代表从未研究；“记录有交付，本机缺失”不计入本机实现。Rust函数定义扫描不提供原名/ABI/逐函数验收。', '',
                '## 模块总览', '',
                '| 原源码模块 | 状态 | B原接口：已观察/已分析/未触及 | 原文件 A/B/C | 本机恢复文件引用数 | 已恢复范围 | 接线与验证 | 下一闭包 |',
                '|---|---|---|---|---|---|---|---|']
    for module in doc['modules']:
        slug = re.sub(r'[^\w.-]+', '__', module['module'])
        path = DETAILS / f'{slug}.md'
        counts = '/'.join(str(sum(v in source_rows[p]['versions'] for p in module['original_files'])) for v in ('A', 'B', 'C'))
        interface = module['interface_progress']
        interface_counts = '/'.join(str(interface[k]) for k in ('observed_entries','business_analyzed_partial','untouched')) if interface else '未盘点'
        overview.append('| ' + ' | '.join([f"[{module['module']}](tracking/module-progress/{slug}.md)",
                        LABELS.get(module['status'], module['status']), interface_counts, counts, str(len(module['restoration_files'])),
                        escape(module['recovered_scope']), escape(module['verification_summary']), escape(module['next_action'])]) + ' |')
        lines = [f"# {module['module']}", '', f"状态：{LABELS.get(module['status'], module['status'])}。完整模块验收：{'见明确版本/范围登记' if module['full_module_accepted'] else '未通过'}。", '',
                 'B原接口已观察/已分析/未触及：' + interface_counts + '。[逐接口进度](../../INTERFACE_PROGRESS.md)。', '',
                 f"定义归属：{module['ownership']}。", '', module['recovered_scope'], '',
                 '## 接线与验证', '', f"接线：{module['integration']}。", '', module['verification_summary'], '',
                 '## 已恢复的函数和类型范围', '']
        if module['business_function_progress']:
            lines[6:6] = ['[业务功能对照](../../FOUNDATION_BUSINESS_COMPARISON.md)：' +
                          json.dumps(module['business_function_progress'],ensure_ascii=False) +
                          '。此为功能行分类，不是完成率；源码缺失与原验证/库外接线分开。', '']
        lines += [f'- {x}' for x in module['known_functions']] or ['原函数全集尚未核对。下面的Rust定义清单仅登记实现符号。']
        if module['function_bindings']:
            lines += ['', '| 原函数 | Rust接口 | 版本与限定范围 | 状态与证据 |', '|---|---|---|---|']
            for function in module['function_bindings']:
                lines.append('| ' + ' | '.join([
                    escape(function['original_symbol']), link(function['restoration'], DETAILS) + '：`' + function['rust_function'] + '`',
                    function['baseline'] + '：' + escape(function['scope']), function['status'] + '：' + ', '.join(link(p, DETAILS) for p in function['evidence'])]) + ' |')
        lines += ['', '## 明确缺口', ''] + [f'- {x}' for x in module['gaps']]
        lines += ['', '下一闭包：' + module['next_action'], '', '## 证据', '']
        lines += [f'- {link(p, DETAILS)}' + ('（当前缺失）' if not (ROOT / p).is_file() else '') for p in module['evidence']] or ['尚未登记模块级证明。']
        lines += ['', '## 原作文件逐项覆盖', '', '| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |', '|---|---|---|---|---|']
        for original in module['original_files']:
            row = source_rows[original]
            refs = '<br>'.join(link(r['path'], DETAILS) + ('' if r['exists'] else '（缺失）') + '：' + r['mapping_basis'] for r in row['restoration_refs']) or '未建立精确映射；模块级恢复文件另列'
            funcs = '<br>'.join(escape(f['name']) + '（' + f['status'] + '；业务等价以限定证据为准）' for f in row['known_original_functions']) or '全集/逐函数映射待核对'
            validation = '<br>'.join(escape(v['scope']) + '：' + ', '.join(link(p, DETAILS) for p in v['evidence']) for v in row['verification_scopes']) or row['verification']
            lines.append(f"| `{original}` | {','.join(row['versions'])} | {refs} | {funcs} | {row['source_coverage']}；{validation} |")
        lines += ['', '## 本机恢复文件与Rust定义', '', '下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。', '']
        for relative in module['restoration_files']:
            row = restored[relative]
            lines += [f'### {relative}', '', link(relative, DETAILS), '', f"SHA256：`{row['sha256']}`；缺口注释标记数：{row['gap_marker_count']}。", '',
                      '| Rust定义 | 定义行 | 原名/等价状态 |', '|---|---|---|']
            lines += [f"| `{r['name']}` | {r['line']} | 未由扫描证明；以证据限定范围为准 |" for r in row['rust_declarations']]
            if row['gap_markers']:
                lines += ['', '缺口注释摘录（最多30条）：', ''] + [f"- L{x['line']}：{escape(x['text'])}" for x in row['gap_markers']]
        path.write_text('\n'.join(lines) + '\n', encoding='utf-8')
    overview += ['', '文件数量只是库存，不用于计算完成率；同一恢复文件可服务多个原模块，引用数不可相加。', '',
                 '## HLSL源码层', '', doc['shader_layer'].get('scope', ''), '',
                 doc['shader_layer'].get('verification', ''), '',
                 '剩余：' + '；'.join(doc['shader_layer'].get('gaps', [])), '',
                 '[Shader逐文件、入口、函数与验证报告](tracking/module-progress/shaders.md)。公开算法包含文件只登记依赖边界，不计私有恢复。', '',
                 '## 证据当前性', '', '| 报告 | 显式路径哈希检查 | 报告存在 |', '|---|---|---|']
    overview += [f"| {link(e['path'])} | {e['freshness']} | {e['exists']} |" for e in doc['evidence']]
    overview += ['', '## 待核对与本机缺失', '']
    overview += [f"- {escape(i.get('module', ''))}：{i['kind']}，`{i['path']}`。" for i in doc['issues']] or ['没有登记的路径缺失。']
    overview += ['', '机器可读全集：[module-progress.json](tracking/module-progress.json)。原路径来源：[version-map.json](tracking/version-map.json)。', '']
    REPORT.write_text('\n'.join(overview), encoding='utf-8')
    shader_rows = doc['shader_layer']['inventory']['files']
    lines = ['# HLSL 原源码文件进度', '', doc['shader_layer']['scope'], '',
             doc['shader_layer']['verification'], '',
             '原入口/阶段来自bin嵌入编译记录；重建helper名字为词法扫描，不能证明原作者名字或内联算法等价。每个直接匹配报告均保留，没有自动选择最新PASS。', '',
             '| 原嵌入源码路径 | 本机恢复文件 | 原入口/阶段 | 报告与缺口 |', '|---|---|---|---|']
    for row in shader_rows:
        entries = ', '.join(e['name'] + '/' + e['stage'] for e in row['original_entries']) or 'include/嵌入源'
        reports = '<br>'.join(link(r['path'], DETAILS) + '：' + escape(r['summary']) for r in row['validation_reports']) or '未建立直接文件级报告；消费main/族报告另查'
        lines.append(f"| `{row['original_path']}` | {link(row['restoration'], DETAILS)} {'存在' if row['exists'] else '缺失'} | {entries} | {reports}<br>{row['ownership']}；GPU等价未验收 |")
    lines += ['', '## 重建源码函数定义清单', '']
    for row in shader_rows:
        if not row['exists']:
            continue
        lines += [f"### {row['original_path']}", '', f"SHA256：`{row['sha256']}`。", '',
                  ', '.join(f"{f['name']}@L{f['line']}" for f in row['candidate_function_definitions']) or '无可扫描函数定义（宏、include或数据声明）；不据此判空实现。', '']
    (DETAILS / 'shaders.md').write_text('\n'.join(lines), encoding='utf-8')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true', help='read-only ledger integrity and source/input checks')
    args = parser.parse_args()
    if args.check:
        doc = read_json(OUTPUT)
        validate(doc)
        for relative, expected in doc['inputs'].items():
            assert digest(ROOT / relative) == expected, f'input changed: {relative}'
        for record in doc['evidence']:
            if record['exists']:
                assert digest(ROOT / record['path']) == record['report_sha256'], f"report changed: {record['path']}"
        assert REPORT.is_file()
        current = build()
        for field in ('inputs', 'modules', 'files', 'restorations', 'evidence', 'issues', 'shader_layer'):
            assert current[field] == doc[field], f'ledger view changed, refresh required: {field}'
        for module in doc['modules']:
            slug = re.sub(r'[^\w.-]+', '__', module['module'])
            assert (DETAILS / f'{slug}.md').is_file(), f'missing module view: {slug}'
        assert (DETAILS / 'shaders.md').is_file()
        coverage = read_json(ROOT / 'tracking/module-coverage.json')
        assert coverage['updated_at'] == doc['updated_at'], 'coverage view is stale'
        assert [(m['path'], m['source_status']) for m in coverage['modules']] == [(m['module'], m['status']) for m in doc['modules']], 'coverage view mismatch'
    else:
        doc = build()
        OUTPUT.write_text(json.dumps(doc, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
        render(doc)
        coverage = {'schema_version': 2, 'updated_at': doc['updated_at'],
                    'authority': 'MODULE_PROGRESS.md + tracking/module-progress.json',
                    'scope': '源码结构库存与限定恢复状态；不是功能/原函数验收分母',
                    'overall_completion_percent': None,
                    'modules': [{
                        'path': m['module'], 'source_status': m['status'],
                        'source_counts': {v: sum(v in row['versions'] for row in doc['files'] if row['module'] == m['module']) for v in ('A', 'B', 'C')},
                        'system_acceptance': 'explicit_scoped_acceptance' if m['full_module_accepted'] else 'not_verified',
                        'restoration_file_refs': len(m['restoration_files']),
                        'detail': 'tracking/module-progress/' + re.sub(r'[^\w.-]+', '__', m['module']) + '.md'
                    } for m in doc['modules']]}
        (ROOT / 'tracking/module-coverage.json').write_text(json.dumps(coverage, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps({'ok': True, 'modules': len(doc['modules']), 'original_files': len(doc['files']),
                      'restoration_files': len(doc['restorations']), 'rust_declarations': sum(len(r['rust_declarations']) for r in doc['restorations']),
                      'issues': len(doc['issues']), 'business_tests_run': False}, ensure_ascii=False))


if __name__ == '__main__':
    main()
