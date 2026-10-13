"""Compare observed B interfaces with reviewed evidence, not declaration counts."""
from pathlib import Path
import hashlib
import json
import re

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'evidence/shared-foundation-2026-10-13/small-interfaces'
OUT.mkdir(exist_ok=True)


def canonical(name):
    name = re.sub(r'::h[0-9a-f]{16}$', '', name)
    depth = 0
    result = []
    for c in name:
        if c == '<':
            depth += 1
        elif c == '>':
            depth -= 1
        elif not depth:
            result.append(c)
    return ''.join(result).strip()


rows = {}
closures = []
inputs = []
for crate in ('paths', 'lang-lib', 'tabloid', 'theme-defs'):
    for source in sorted((ROOT / 'skeleton/B/crates' / crate / 'src').rglob('*.rs')):
        text = source.read_text(encoding='utf-8')
        rel = source.relative_to(ROOT).as_posix()
        inputs.append({'path': rel, 'sha256': hashlib.sha256(source.read_bytes()).hexdigest()})
        for block in re.split(r'(?m)^// 函数符号: ', text)[1:]:
            name = block.splitlines()[0]
            if '::closure$' in name:
                closures.append({'name': name, 'source': rel})
                continue
            key = canonical(name)
            item = rows.setdefault(key, {'interface_id': key, 'crate': crate,
                'original_names': [], 'pdb_provenance': [], 'original_addresses': [],
                'original_typed_signature_verified': False, 'business_analysis_started': False,
                'has_recovered_implementation': False, 'static_business_compared': False,
                'direct_original_entry_differential': False, 'original_consumer_chain_verified': False,
                'status': 'untouched', 'restoration': None, 'comparison_scope': None,
                'evidence': [], 'gaps': []})
            item['original_names'].append(name)
            head = block.split('// BEGIN GENERATED DEPENDENCY ANNOTATIONS')[0]
            loc = re.search(r'(?m)^// 首条位置: (.+)$', head)
            location = json.loads(loc[1]) if loc else {}
            item['pdb_provenance'].append({'source': rel,
                'inventory_line': text[:text.find('// 函数符号: ' + name)].count('\n') + 1,
                'record_forms': re.search(r'(?m)^// 记录形式: (.+)$', head)[1],
                'source_line': re.search(r'(?m)^// 源码行: (.+)$', head)[1],
                'ipi_ids': re.search(r'(?m)^// IPI IDs: (.+)$', head)[1], 'first_location': location})
            if location.get('va'):
                item['original_addresses'].append(hex(location['va']))

prefix = 'evidence/shared-foundation-2026-10-13/'
path_delivery = json.loads((ROOT / (prefix + 'paths-language/delivery.json')).read_text(encoding='utf-8'))
for f in path_delivery['packages'][0]['functions']:
    key = canonical(f['name'])
    item = rows[key]
    item.update(business_analysis_started=True, has_recovered_implementation=True,
        static_business_compared=True, status='implemented_static',
        restoration={'file': path_delivery['packages'][0]['recovered_files'][0],
                     'method': canonical(f['recovered_symbol'])},
        comparison_scope='Original static Windows path policy and candidate contracts; no native entry differential',
        evidence=[prefix+'paths-language/delivery.json', prefix+'paths-language/CONTRACTS.md', prefix+'paths-language/validation.json'],
        gaps=['Full startup consumer integration and original runtime comparison remain'])

lang = json.loads((ROOT / (prefix+'lang-small/summary.json')).read_text(encoding='utf-8'))
for f in lang['original_emitted_private_functions']:
    item = rows[canonical(f['name'])]
    item.update(business_analysis_started=True, has_recovered_implementation=True,
        static_business_compared=True, status='implemented_static',
        restoration={'file':'reconstruction/shared-foundation/'+f['original_file'], 'method':canonical(f['name'])},
        comparison_scope='Original static locale policy; candidate real asset/process probes; no native behavior execution',
        evidence=[prefix+'lang-small/summary.json', prefix+'lang-small/validation.json', prefix+'lang-small/native-result.json'],
        gaps=['Full game initialization/consumer integration remains'])
for method in ('locale', 'try_get'):
    item = rows['lang_lib::language_library::LanguageLibrary::'+method]
    item.update(business_analysis_started=True, has_recovered_implementation=True,
        static_business_compared=True, status='implemented_static',
        restoration={'file':'reconstruction/shared-foundation/crates/lang-lib/src/language_library.rs', 'method':'LanguageLibrary::'+method},
        comparison_scope='Getter policy inferred/compared in original game_locale/lookup/get caller bodies; separate native getter not executed',
        evidence=[prefix+'paths-language/CONTRACTS.md', prefix+'paths-language/lang-root.asm.txt', prefix+'paths-language/language-library.asm.txt'],
        gaps=['Inline getter contract comparison is partial; typed signature not directly established'])
rows['lang_lib::language_library::LanguageLibrary::new']['gaps'] = [
    'Original inline constructor has metadata; no separately reviewed original constructor contract or mapped standalone implementation',
    'Candidate parse_yml constructs the object, but lexical constructor code does not prove this original interface']
constructor_report = ROOT / (prefix+'small-interfaces/interface-batch-02/language-constructor.json')
if constructor_report.exists():
    constructor = json.loads(constructor_report.read_text(encoding='utf-8'))
    source = ROOT / 'reconstruction/shared-foundation/crates/lang-lib/src/language_library.rs'
    source_hash = hashlib.sha256(source.read_bytes()).hexdigest()
    constructor_evidence = [constructor_report.relative_to(ROOT).as_posix()]
    if source_hash != constructor['source_sha256']:
        # Keep the original snapshot intact. A changed file needs an explicit
        # current review and validation before retaining its constructor claim.
        review_path = ROOT / 'evidence/shared-foundation-review-2026-10-13/small-modules/review.json'
        review = json.loads(review_path.read_text(encoding='utf-8'))
        current = review['constructor_review']
        assert current['source_sha256'] == source_hash
        assert current['static_business_compared'] and current['candidate_constructor_test_passed']
        validation_path = ROOT / review['validation']
        validation = json.loads(validation_path.read_text(encoding='utf-8'))
        assert all(x['exit_code'] == 0 for x in validation['commands'])
        assert any(x['path'] == source.relative_to(ROOT).as_posix() and x['sha256'] == source_hash
                   for x in validation['source_files'])
        constructor_evidence += [review_path.relative_to(ROOT).as_posix(), validation_path.relative_to(ROOT).as_posix()]
        constructor['comparison_kind'] = current['comparison_kind']
    for evidence_path in constructor_evidence:
        inputs.append({'path':evidence_path, 'sha256':hashlib.sha256((ROOT/evidence_path).read_bytes()).hexdigest()})
    rows['lang_lib::language_library::LanguageLibrary::new'].update(
        business_analysis_started=True, has_recovered_implementation=True,
        static_business_compared=True, status='implemented_static',
        restoration={'file':source.relative_to(ROOT).as_posix(), 'method':'LanguageLibrary::new'},
        comparison_scope=constructor['comparison_kind'],
        evidence=constructor_evidence,
        gaps=['Full original ownership/lifecycle/caller comparison remains partial; native constructor entry not executed'])

tab = json.loads((ROOT/(prefix+'utils-tabloid/delivery.json')).read_text(encoding='utf-8'))
for owner, method in (('Publisher','publish'), ('Publisher','subscribe'), ('Subscriber','take')):
    item = rows['tabloid::'+owner+'::'+method]
    item.update(business_analysis_started=True, has_recovered_implementation=True,
        static_business_compared=True, status='implemented_static',
        restoration={'file':'reconstruction/shared-foundation/crates/tabloid/src/lib.rs', 'method':owner+'::'+method},
        comparison_scope='Original publication policy review + candidate contracts, not native publication execution',
        evidence=[prefix+'utils-tabloid/delivery.json',prefix+'utils-tabloid/CONTRACT.md'],
        gaps=['Real DirtyTiles/ColliderDiff consumer integration and original runtime comparison remain'])
for key, candidate in [('tabloid::impl$0::default','Publisher::default'),
                       ('tabloid::impl$2::drop','owned-guard/Arc RAII'),
                       ('tabloid::impl$3::deref','PublicationReader::deref')]:
    rows[key]['unverified_candidate_mapping'] = candidate
    rows[key]['gaps'] = ['Original inline impl owner/behavior needs independent comparison; candidate behavior alone does not count as analysis',
        'Private drop retained in inventory: not automatically excluded as compiler/library glue']

for number, ty, method in [(0,'EventDataError','clone'),(2,'EventDataError','eq'),(5,'ActiveEvent','clone'),(7,'ActiveEvent','eq')]:
    item = rows[f'theme_defs::impl${number}::{method}']
    item.update(business_analysis_started=True, has_recovered_implementation=True,
        static_business_compared=True, status='implemented_static',
        restoration={'file':'reconstruction/shared-foundation/crates/theme-defs/src/lib.rs','method':ty+'::'+method+' (derive)'},
        comparison_scope='IPI-owned Clone/Eq payload fields; names/variants partly inferred, no direct original interface execution',
        evidence=[prefix+'paths-language/delivery.json',prefix+'paths-language/CONTRACTS.md'],
        gaps=['Error/transport contract remains partial; original full producer-consumer comparison pending'])

interfaces = sorted(rows.values(), key=lambda x:x['interface_id'])
summary = []
for crate in ('paths','lang-lib','tabloid','theme-defs'):
    subset = [x for x in interfaces if x['crate']==crate]
    summary.append({'crate':crate,'observed_interfaces':len(subset),
        'business_analyzed_partial':sum(x['business_analysis_started'] for x in subset),
        'untouched':sum(not x['business_analysis_started'] for x in subset),
        'candidate_implementation_present':sum(x['has_recovered_implementation'] for x in subset),
        'direct_original_entry_differential':sum(x['direct_original_entry_differential'] for x in subset),
        'original_consumer_chain_verified':0})
doc = {'baseline':'B','denominator_complete':False,'count_policy':'PDB compiled+inline/declaration source-owned observed interfaces; merge generic repetitions and same-key multi-file observations. Explicit private default/drop/deref and Clone/Eq retained. Closures separate. Finding a name or decompilation does not imply business analysis.',
    'unknown_limits':['PDB not complete author source/API inventory','void debug prototypes are not verified Rust ABI','Inline impl$N type ownership can remain unresolved','Native validation and candidate tests are separate'],
    'statistics':summary,'interfaces':interfaces,'closures':closures,'inputs':inputs}
(OUT/'interface-coverage.json').write_text(json.dumps(doc,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
lines=['# B 小模块原接口对照','','分母是已观察PDB接口，不是完整作者API；仅定位名字不算业务分析。','','| 模块 | 原接口 | 已分析 | 未触及 | 有映射实现 | 原入口对照 |','|---|---:|---:|---:|---:|---:|']
for x in summary:
    lines.append('| '+' | '.join(str(x[k]) for k in ['crate','observed_interfaces','business_analyzed_partial','untouched','candidate_implementation_present','direct_original_entry_differential'])+' |')
lines+=['','| 原接口 | 状态 | 实现 | 剩余工作 |','|---|---|---|---|']
for x in interfaces:
    lines.append(f"| `{x['interface_id']}` | {x['status']} | {x['restoration']['method'] if x['restoration'] else '未核实映射'} | {'; '.join(x['gaps'])} |")
(OUT/'interface-coverage.md').write_text('\n'.join(lines)+'\n',encoding='utf-8')
print(json.dumps(summary,ensure_ascii=False))
