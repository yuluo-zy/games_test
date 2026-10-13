"""Keep generated code and unresolved trait provenance out of business progress."""
import hashlib
import json
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
BASE = 'evidence/shared-foundation-review-2026-10-13/'
paths = [BASE + owner + '/interface-classification.json'
         for owner in ('small-modules', 'utils-base', 'geometry-curves', 'assets')]
valid = {'generated_reuse', 'schema_policy', 'custom_business', 'unresolved'}
entries = {}
inputs = []
schema_groups = []
for path in paths:
    raw = (ROOT / path).read_bytes()
    data = json.loads(raw)
    for group in data.get('schema_policy_groups', []):
        schema_groups.append(group | {'classification_report':path})
    inputs.append({'path': path, 'sha256': hashlib.sha256(raw).hexdigest()})
    for entry in data['entries']:
        assert entry['classification'] in valid
        key = entry['interface_id']
        assert key not in entries, f'Duplicate classifier: {key}'
        entries[key] = entry | {'classification_report':path}

# The four small modules' ordinary APIs are already individually reviewed.
# Preserve them explicitly so this ledger classifies the entire raw inventory.
small_path = 'evidence/shared-foundation-2026-10-13/small-interfaces/interface-coverage.json'
small_raw = (ROOT/small_path).read_bytes()
inputs.append({'path':small_path, 'sha256':hashlib.sha256(small_raw).hexdigest()})
for item in json.loads(small_raw)['interfaces']:
    key = item['interface_id']
    if key not in entries:
        assert item['crate'] in {'paths','lang-lib'} or key in {'tabloid::Publisher::publish','tabloid::Publisher::subscribe','tabloid::Subscriber::take'}
        entries[key] = {'interface_id':key, 'classification':'custom_business',
                        'reason':'Individually reviewed source-owned ordinary path/language/publication policy API; no generated trait body inferred.',
                        'evidence':[small_path], 'classification_report':small_path}

doc = {'schema_version':1, 'baseline':'B', 'entries':entries, 'inputs':inputs,
       'schema_policy_groups':schema_groups,
       'categories': {
           'generated_reuse':'宏派生、标准字段操作及确认的依赖转发；直接使用derive/原库，不计独立业务函数。',
           'schema_policy':'宏参数、字段/枚举/default/错误模板规则；恢复类型或配置合同，不逐个逆向生成helper。',
           'custom_business':'已确认手写或包含自定义策略的业务接口；计入业务进度。',
           'unresolved':'尚无法确认生成或自定义；单独待分类，不冒充未恢复业务接口。'},
       'policy':'Never classify solely by method name. Keep raw PDB records and historical audits. Derived field/schema contracts remain work when unknown; generated implementation bodies are not reverse-engineering tasks.',
       'explicit_classification_statistics':dict(Counter(x['classification'] for x in entries.values())),
       'ordinary_unlisted_api_policy':'Source-owned ordinary methods retain the existing business classification; uncertain traits never fall through as business.'}
(ROOT/'tracking/shared-foundation-interface-classification.json').write_text(json.dumps(doc,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
print(json.dumps(doc['explicit_classification_statistics']))
