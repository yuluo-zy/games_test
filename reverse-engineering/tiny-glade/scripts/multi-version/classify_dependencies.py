"""Exclude known dependency provenance, preserve private game candidates."""
from collections import Counter, defaultdict
from pathlib import Path
import hashlib, json, re

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'evidence/multi-version'
BUILDING = ['crates/country-core/src/resources/walls/', 'crates/country-core/src/resources/roofs/',
    'crates/country-core/src/systems/wall/', 'crates/systems/roof/',
    'crates/systems/wall-constructor/', 'crates/systems/decorator/', 'crates/timberframe/']
# Established library/SDK ownership stays excluded even when vendored as path
# packages. This does not claim their shipped patches match public upstream.
KNOWN_LOCAL_DEPENDENCIES = {'FidelityFX', 'GPU-Raytracer', 'XeGTAO', 'gpu-allocator', 'vk-sync-fork'}

def main():
    packages = defaultdict(dict); sources = defaultdict(dict); dependencies = []; inputs = {}
    summaries = {}
    for version in ['A','B','C']:
        package_file = OUT / f'{version}-dependencies.json'
        source_file = OUT / f'{version}-sources.json'
        for path in [package_file,source_file]:
            inputs[path.relative_to(ROOT).as_posix()] = hashlib.sha256(path.read_bytes()).hexdigest()
        manifest = json.loads(package_file.read_text()); pdb = json.loads(source_file.read_text())
        policies = Counter()
        for package in manifest['packages']:
            source = package.get('source')
            policy = 'dependency_reuse' if source or package['name'] in KNOWN_LOCAL_DEPENDENCIES else 'ownership_check_required'
            key = (package['name'],package['version'],source or 'local_or_path')
            packages[key][version] = {**package,'policy':policy}
            policies[policy] += 1
        for row in pdb['files']:
            sources[row['path']][version] = row['sha256']
        path_counts = Counter()
        for path in pdb['dependency_source_paths']:
            if '/registry/src/' in path:
                kind = 'registry_source'
                match = re.search(r'/registry/src/[^/]+/([^/]+)-([0-9][^/]*)/',path)
                owner = {'name':match[1],'version':match[2]} if match else {}
            elif '/git/checkouts/' in path:
                kind = 'git_checkout_source'
                match = re.search(r'/git/checkouts/(.+)-[0-9a-f]+/([^/]+)/',path)
                owner = {'checkout_repository':match[1],'checkout_revision':match[2]} if match else {}
            else:
                kind = 'cargo_source_unresolved'; owner = {}
            dependencies.append({'baseline':version,'path':path,'provenance':kind,
                'policy':'dependency_reuse' if owner else 'ownership_check_required',**owner})
            path_counts[kind] += 1
        summaries[version] = {'lockfile_available':not manifest['unavailable'],
            'package_policies':dict(policies),'pdb_dependency_paths':dict(path_counts),
            'rustc_commits':pdb['rustc_commits']}
    rows=[]
    for path, versions in sorted(sources.items()):
        same = len(versions)>1 and len({tuple(hashes) for hashes in versions.values()})==1
        building = any(path.startswith(prefix) for prefix in BUILDING)
        rows.append({'path':path,'versions':versions,'same_source_checksum':same,
            'policy':'building_business_candidate' if building else 'private_source_ownership_check',
            'dependency_inferred_from_same_checksum':False})
    package_rows=[{'name':key[0],'version':key[1],'source':key[2], 'versions':values,
        'policy':'dependency_reuse' if key[2]!='local_or_path' or key[0] in KNOWN_LOCAL_DEPENDENCIES else 'ownership_check_required'}
        for key,values in sorted(packages.items())]
    report={'schema_version':1,'input_sha256':inputs,'baselines':summaries,'packages':package_rows,
        'pdb_dependency_paths':dependencies,'private_source_candidates':rows,
        'definition_namespaces_always_excluded':['std','core','alloc'],
        'known_vendored_dependency_names':sorted(KNOWN_LOCAL_DEPENDENCIES),
        'external_package_definition_roots':sorted({row['name'].replace('-','_') for row in package_rows if row['policy']=='dependency_reuse'}),
        'generic_rule':'Classify by defining crate/source; game types inside a dependency generic instance do not make it business code',
        'scope':'Artifact package/PDB source provenance; not complete symbol-address ownership or behavior equivalence',
        'limitations':['A lockfile absent: package versions inferred only where PDB paths explicitly encode them',
            'Private/local ownership and fork deltas require inspection','Source paths do not establish source availability',
            'Same private source checksum does not imply dependency; monomorphization behavior not established']}
    target=ROOT/'tracking/dependency-exclusions.json';target.write_text(json.dumps(report,ensure_ascii=False,indent=2)+'\n')
    print(json.dumps({'baseline_summary':summaries,'package_identities':len(package_rows),
        'building_source_candidates':sum(row['policy']=='building_business_candidate' for row in rows),
        'private_shared_source_paths':sum(row['same_source_checksum'] for row in rows)},ensure_ascii=False))

if __name__=='__main__': main()
