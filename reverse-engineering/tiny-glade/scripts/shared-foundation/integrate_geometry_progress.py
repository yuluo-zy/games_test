"""Register source-module mappings from reviewed B geometry interface evidence."""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
AUDIT = 'evidence/shared-foundation-2026-10-13/geometry-curves/interface-coverage.json'
MASTER = 'tracking/shared-foundation-interfaces.json'
path = ROOT / 'tracking/module-progress-overrides.json'
config = json.loads(path.read_text(encoding='utf-8'))
audit = json.loads((ROOT / AUDIT).read_text(encoding='utf-8'))
master = json.loads((ROOT / MASTER).read_text(encoding='utf-8'))
master_interfaces = {x['interface_id']:x for x in master['interfaces']}
for module in ('crates/utils/src/components', 'crates/utils/src/curve', 'crates/utils/src/geometry'):
    existing = next((x for x in config['modules'] if x['module'] == module), None)
    if existing is None:
        existing = {'module': module}
        config['modules'].append(existing)
    rows = [x for x in audit['interfaces'] if x['original_source'].startswith(module + '/')]
    implemented = [x for x in rows if x['has_recovered_implementation']]
    counts = master['modules'][module]
    source_glob = 'reconstruction/shared-foundation/' + module + '/*.rs'
    globs = existing.get('restoration_globs', [])
    if source_glob not in globs:
        globs.append(source_glob)
    existing.update(
        status='candidate_partial', ownership='private_game_source_with_pending_trait_schema_classification',
        scope=f"B原接口台账：{counts['observed_entries']}项已观察，{counts['business_analyzed_partial']}项已有部分业务分析，{counts['untouched']}项未触及；具体状态见逐接口证据。",
        restoration_globs=globs,
        functions=[x['interface_id'] for x in implemented if master_interfaces[x['interface_id']]['count_as_business_interface']],
        generated_or_type_mapped_interfaces=[x['interface_id'] for x in implemented if not master_interfaces[x['interface_id']]['count_as_business_interface']],
        verification='原指令对照覆盖指定实例与输入；静态候选、归属待定项及原消费者链分别记录，不能据此验收整个目录。',
        gaps=['尚未分析的接口逐项列于 INTERFACE_PROGRESS.md', '泛型策略及边界输入尚未全部对照', '原游戏消费者链尚未验收'],
        next_action='按原接口缺口补充业务分析、复用原依赖实现并对照修正。',
        evidence=list(dict.fromkeys(existing.get('evidence',[]) + [AUDIT, MASTER, 'evidence/shared-foundation-2026-10-13/geometry-curves/interface-batch-02/coverage-delta.json'])),
        integration='candidate_crate_exported_original_game_consumer_not_accepted')

    for original_file in sorted({x['original_source'] for x in implemented}):
        restored = sorted({x['restored_file'] for x in implemented if x['original_source'] == original_file and x.get('restored_file')})
        for target in restored:
            entry = {'original': original_file, 'restoration': target,
                     'basis': 'B_PDB_definition_source_and_reviewed_interface_contract',
                     'verification_scope': '仅映射该文件已分析的接口子集，不代表原文件全部函数已恢复。',
                     'evidence': [AUDIT, MASTER]}
            prior = next((x for x in config['file_bindings'] if x['original'] == original_file and x['restoration'] == target), None)
            if prior is None:
                config['file_bindings'].append(entry)
            else:
                prior.update(entry)

path.write_text(json.dumps(config, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
print('Registered reviewed B components/curve/geometry mappings; no full-module acceptance.')
