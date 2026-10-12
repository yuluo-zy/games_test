"""Reconstruct manifest dependency identities without inventing original features.

B/C use original Cargo.lock edges. A materializes PDB-observed candidates only;
missing private package versions and git URLs remain explicitly unknown.
"""
from collections import Counter, defaultdict
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tomllib
from urllib.parse import urlsplit, urlunsplit

ROOT = Path(__file__).resolve().parents[1]
PROJECT = ROOT.parent
MARKER = '# Generated dependency reconstruction: skeleton/_tools/restore_cargo_manifests.py'
STD = {'std', 'core', 'alloc'}
CRATES_IO = 'registry+https://github.com/rust-lang/crates.io-index'

def quote(value): return json.dumps(value, ensure_ascii=False)
def digest(path): return hashlib.sha256(path.read_bytes()).hexdigest()
def save(path, data):
    tmp = path.with_suffix(path.suffix + '.tmp')
    tmp.write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n'); tmp.replace(path)
def inline(fields): return '{ ' + ', '.join(k + ' = ' + quote(v) for k, v in fields.items()) + ' }'
def identity(pkg): return (pkg['name'], pkg.get('version'), pkg.get('source', 'local_or_path'))

def module_name(relative):
    p = relative.parts
    if p[0] == 'rhapsody2':
        if len(p) == 1: return 'rhapsody'
        if p[-1] == 'vk-sync-rs': return 'vk-sync-fork'
        return p[-1]
    if len(p) > 2 and p[1] == 'systems':
        name = p[2]
        if name in {'forest', 'toolbar', 'timberframe'}: return name
        if name == 'tree-tool': return 'system-tree'
        return 'system-' + name
    return p[-1]

def read_imports(path):
    # The import section precedes the large function/alias arrays.
    parts = []
    with path.open() as f:
        for line in f:
            if line.startswith('  "functions":'): break
            parts.append(line)
    data = json.loads(''.join(parts).rstrip().removesuffix(',') + '\n}')
    return data['imports']

class Builder:
    def __init__(self, version):
        self.v = version; self.root = ROOT / version
        self.input = PROJECT / 'evidence/multi-version' / f'{version}-dependencies.json'
        self.lock = json.loads(self.input.read_text()); self.packages = self.lock['packages']
        self.by_name = defaultdict(list)
        for p in self.packages: self.by_name[p['name']].append(p)
        self.paths = sorted(p for p in self.root.rglob('Cargo.toml') if p.parent != self.root)
        self.names = {p: module_name(p.parent.relative_to(self.root)) for p in self.paths}
        self.local = {name: path.parent for path, name in self.names.items()}
        self.nested_workspaces = [p.parent for p in self.paths if p.stat().st_size and 'workspace' in tomllib.loads(p.read_text())]
        self.registry = defaultdict(set); self.git = defaultdict(set)
        sources = json.loads((PROJECT / 'evidence/multi-version' / f'{version}-sources.json').read_text())
        for path in sources['dependency_source_paths']:
            s = path.replace('\\', '/')
            m = re.search(r'/registry/src/[^/]+/(.+)-(\d[^/]*)/', s)
            if m: self.registry[m[1].replace('-', '_')].add((m[1], m[2], CRATES_IO))
        self.manifests = []; self.stub_files = []; self.edges = []
        self.observed = defaultdict(lambda: defaultdict(lambda: {'packages': set(), 'source_paths': set(), 'source_files': set()}))

    def observations(self):
        inventory = json.loads((ROOT / '_pdb' / self.v / 'source-files.json').read_text())
        for file in inventory:
            source = self.root / file['path']; manifest = next((p for p in self.paths if source.is_relative_to(p.parent)), None)
            annotations = source.with_suffix('.rs.dependencies.json')
            if manifest is None or not annotations.exists(): continue
            for imp in read_imports(annotations):
                if imp['crate'] in STD: continue
                item = self.observed[manifest][imp['crate']]
                item['packages'].update((p['name'], p.get('version'), p.get('source')) for p in imp['packages'])
                item['source_paths'].update(imp['source_files']); item['source_files'].add(file['path'])

    def resolve(self, token):
        bits = token.split(); name = bits[0]
        version = bits[1] if len(bits) > 1 else None
        source = ' '.join(bits[2:]).strip('()') if len(bits) > 2 else None
        result = [p for p in self.by_name[name] if (version is None or p['version'] == version) and (source is None or p.get('source') == source)]
        return result[0] if len(result) == 1 else None

    def spec(self, owner, pkg):
        source = pkg.get('source'); fields = {}
        if pkg.get('version'): fields['version'] = '=' + pkg['version']
        if source == CRATES_IO: return fields, None
        if source and source.startswith('git+'):
            url = urlsplit(source[4:]); rev = url.fragment
            if not re.fullmatch('[0-9a-fA-F]{40}', rev): return None, 'git commit is not a confirmed full SHA'
            fields.update(git=urlunsplit((url.scheme, url.netloc, url.path, '', '')), rev=rev)
            return fields, None
        if not source or source == 'local_or_path':
            if pkg['name'] in self.local:
                fields['path'] = os.path.relpath(self.local[pkg['name']], owner.parent).replace(os.sep, '/')
                return fields, None
            return None, 'original local/path package has no source module in this isolated skeleton'
        return None, 'registry name/configuration cannot be reconstructed from this source URL'

    def candidates_a(self, manifest):
        result = []; unresolved = []
        own = self.names[manifest].replace('-', '_')
        local_by_crate = {name.replace('-', '_'): name for name in self.local}
        for crate, obs in sorted(self.observed[manifest].items()):
            if crate == own: continue
            registry = set(); git = set()
            for path in obs['source_paths']:
                path = path.replace('\\', '/')
                m = re.search(r'/registry/src/[^/]+/(.+)-(\d[^/]*)/', path)
                if m: registry.add((m[1], m[2], CRATES_IO))
                m = re.search(r'/git/checkouts/(.+)-[0-9a-f]+/([^/]+)/', path)
                if m: git.add((m[1], m[2]))
            if not registry and not git and crate not in local_by_crate:
                # A unique artifact-wide registry identity is a candidate, not a direct edge proof.
                if len(self.registry[crate]) == 1: registry.update(self.registry[crate])
            for name, version, source in sorted(registry):
                result.append({'name': name, 'version': version, 'source': source, 'proof': 'A_PDB_observed_candidate', 'namespace': crate})
            if not registry and crate in local_by_crate:
                result.append({'name': local_by_crate[crate], 'source': 'local_or_path', 'proof': 'A_PDB_observed_candidate', 'namespace': crate})
            for repo, rev in sorted(git):
                unresolved.append({'name': crate, 'version': 'unknown', 'source': 'git_checkout', 'checkout_repository': repo,
                    'checkout_revision': rev, 'reason': 'A Cargo.lock absent; original git URL, full commit and package version are not recorded', 'proof': 'A_PDB_git_checkout'})
            if not registry and not git and crate not in local_by_crate:
                unresolved.append({'name': crate, 'version': 'unknown', 'source': 'unknown', 'reason': 'namespace observation lacks a unique package/version binding', 'proof': 'A_PDB_namespace'})
        return result, unresolved

    def target(self, manifest, text):
        module = manifest.parent
        if (module / 'src/lib.rs').exists() or (module / 'src/main.rs').exists(): return
        stub = module / '_manifest_stub.rs'
        if not stub.exists(): stub.write_text('// Manifest inspection target only; original crate entry source is absent from the PDB inventory.\n// No library implementation has been recovered here.\n')
        self.stub_files.append(stub.relative_to(ROOT).as_posix())
        text += ['# Inspection-only target; the original entry file is not available.', '[lib]', 'path = "_manifest_stub.rs"', '']

    def write_module(self, manifest):
        name = self.names[manifest]; matches = self.by_name[name]
        package = matches[0] if len(matches) == 1 else None
        if self.v != 'A' and package is None: raise RuntimeError(f'Package identity ambiguous: {self.v} {manifest} {name}')
        if manifest.stat().st_size and not manifest.read_text().startswith(MARKER):
            self.reuse_module(manifest, package)
            return
        text = [MARKER, '# Evidence-derived manifest, not the original author Cargo.toml.',
            '# Features, dependency kind (normal/build/dev), target cfg and original aliases are not recovered.',
            '# Locked edges are materialized in [dependencies] as an inspection union.',
            '# PDB-observed transitive namespaces do not create extra B/C direct dependencies.', '', '[package]', 'name = ' + quote(name)]
        if package: text += ['version = ' + quote(package['version'])]
        else: text += ['# Original package version: unknown (A has no Cargo.lock).', '# Cargo defaults the omitted version to 0.0.0; this is not an original version.']
        text += ['edition = "2021" # Reconstruction setting; original edition unknown.', 'publish = false # Isolated evidence scaffold.', '']
        self.target(manifest, text)
        candidates = []; missing = []
        if package:
            for token in package.get('dependencies', []):
                dep = self.resolve(token)
                if dep: candidates.append({**dep, 'proof': 'original_lock_edge', 'lock_token': token})
                else: missing.append({'name': token.split()[0], 'version': token.split()[1] if len(token.split()) > 1 else 'unknown',
                    'source': 'ambiguous_lock_edge', 'lock_token': token, 'reason': 'Original lock reference did not bind a unique package identity'})
        else: candidates, missing = self.candidates_a(manifest)
        groups = Counter(p['name'] for p in candidates); used_keys = set(); active = []
        text += ['[dependencies]']
        for dep in sorted(candidates, key=lambda p: identity(p)):
            fields, reason = self.spec(manifest, dep)
            if fields is None:
                missing.append({k: v for k, v in {**dep, 'reason': reason}.items() if k not in ('dependencies', 'checksum')}); continue
            key = dep['name']
            if groups[key] > 1:
                key += '_v' + re.sub('[^A-Za-z0-9]', '_', dep.get('version', 'unknown'))
                if key in used_keys: key += '_' + hashlib.sha256(str(dep.get('source')).encode()).hexdigest()[:8]
                fields['package'] = dep['name']
                text += ['# Reconstructed alias for multiple locked identities; original alias unknown.']
            used_keys.add(key)
            text += ['# ' + dep['proof'] + '; exact package version: ' + str(dep.get('version', 'unknown'))]
            if dep.get('source') == 'local_or_path' or not dep.get('source'):
                text += ['# path points to the isolated skeleton; original filesystem path unknown.']
            if '+' in dep.get('version', ''): text += ['# Cargo ignores semver build metadata in requirements; full identity remains in the original lock and evidence.']
            text += [quote(key) + ' = ' + inline(fields)]
            active.append({'key': key, 'package': {k: dep[k] for k in ('name', 'version', 'source', 'checksum') if k in dep}, 'manifest_spec': fields, 'proof': dep['proof'], 'lock_token': dep.get('lock_token')})
        text += ['', '[package.metadata.reconstruction]', 'version = ' + quote(self.v),
            'manifest_status = ' + quote('locked_dependency_identity_union' if package else 'PDB_dependency_candidates_only'),
            'original_package_version_known = ' + str(package is not None).lower(),
            'original_features_known = false', 'original_dependency_kinds_known = false', 'original_target_cfg_known = false',
            'original_aliases_known = false', 'original_edition_known = false', 'complete_build_config_recovered = false',
            'dependency_evidence = ' + quote(self.input.relative_to(PROJECT).as_posix()),
            'dependency_evidence_sha256 = ' + quote(digest(self.input))]
        if package and package.get('source'): text += ['original_package_source = ' + quote(package['source'])]
        if (manifest.parent / '_manifest_stub.rs').exists(): text += ['target_status = "inspection_stub_original_entry_missing"']
        for dep in missing:
            text += ['', '[[package.metadata.reconstruction.unresolved_dependencies]]']
            for key in ('name', 'version', 'source', 'checkout_repository', 'checkout_revision', 'lock_token', 'proof', 'reason'):
                if dep.get(key) is not None: text += [key + ' = ' + quote(dep[key])]
        observed = []
        for crate, obs in sorted(self.observed[manifest].items()):
            entry = {'namespace': crate, 'package_candidates': [list(p) for p in sorted(obs['packages'], key=str)],
                'source_files': sorted(obs['source_files']), 'definition_source_paths': sorted(obs['source_paths']), 'not_a_direct_dependency_edge_proof': True}
            observed.append(entry)
        data = '\n'.join(text) + '\n'; parsed = tomllib.loads(data)
        old = manifest.read_text()
        if old and not old.startswith(MARKER): raise RuntimeError(f'Refuse overwrite manually edited manifest: {manifest}')
        tmp = manifest.with_suffix('.toml.tmp'); tmp.write_text(data); tmp.replace(manifest)
        record = {'version': self.v, 'path': manifest.relative_to(ROOT).as_posix(), 'package_name': name,
            'package_identity': {k: package[k] for k in ('name', 'version', 'source') if k in package} if package else {'name': name, 'version': None},
            'locked_dependencies': package.get('dependencies', []) if package else None,
            'active_dependencies': active, 'unresolved_dependencies': missing, 'observed_namespace_candidates': observed,
            'active_dependency_count': len(active), 'unresolved_dependency_count': len(missing),
            'original_feature_and_target_config_recovered': False, 'manifest_sha256': digest(manifest)}
        save(manifest.with_suffix('.toml.evidence.json'), record); self.manifests.append(record)
        assert len(parsed.get('dependencies', {})) == len(active)

    def reuse_module(self, manifest, package):
        """Preserve already reconstructed independent workspaces after auditing edges."""
        parsed = tomllib.loads(manifest.read_text())
        if package is None or parsed.get('package', {}).get('name') != package['name'] or parsed['package'].get('version') != package['version']:
            raise RuntimeError(f'Existing manifest package identity differs from retained lock: {manifest}')
        root = next((p for p in self.nested_workspaces if manifest.is_relative_to(p)), None)
        if root is None: raise RuntimeError(f'Existing manifest has no auditable independent workspace: {manifest}')
        workspace = tomllib.loads((root / 'Cargo.toml').read_text())
        expected = [self.resolve(t) for t in package.get('dependencies', [])]
        if any(p is None for p in expected): raise RuntimeError('Ambiguous retained edge in existing module')
        active = []; actual = []
        for key, value in parsed.get('dependencies', {}).items():
            fields = workspace['workspace']['dependencies'][key] if isinstance(value, dict) and value.get('workspace') else value
            if isinstance(fields, str): fields = {'version': fields}
            dep_name = fields.get('package', key); dep_version = fields.get('version', '').removeprefix('=')
            candidates = [p for p in expected if p['name'] == dep_name and p['version'] == dep_version]
            if len(candidates) != 1: raise RuntimeError(f'Existing dependency mismatch: {manifest} {key}')
            dep = candidates[0]; source = dep.get('source')
            if 'path' in fields:
                target = (root / fields['path'] if value.get('workspace') else manifest.parent / fields['path']).resolve()
                target_doc = tomllib.loads((target / 'Cargo.toml').read_text())
                assert target_doc['package']['name'] == dep_name and target_doc['package']['version'] == dep_version
                if source and source.startswith('git+'):
                    metadata = workspace.get('package', {}).get('metadata', {}).get('pdb-reconstruction', {})
                    assert metadata.get('original-revision') == urlsplit(source[4:]).fragment
            elif source and source.startswith('git+'):
                wanted = self.spec(manifest, dep)[0]
                assert fields.get('git') == wanted['git'] and fields.get('rev') == wanted['rev']
            else: assert source == CRATES_IO
            actual.append(identity(dep))
            active.append({'key': key, 'package': {k: dep[k] for k in ('name', 'version', 'source', 'checksum') if k in dep},
                'manifest_spec': fields, 'proof': 'original_lock_edge_existing_workspace', 'lock_token': next(t for t in package['dependencies'] if identity(self.resolve(t)) == identity(dep))})
        assert Counter(actual) == Counter(identity(p) for p in expected)
        # Keep workspace inheritance, but show every effective version in each module.
        lookup = {r['key']: r['package'] for r in active}
        lines = []; section = ''
        for line in manifest.read_text().splitlines():
            if line.startswith('# Locked dependency identity (original C lock):'): continue
            if line.startswith('['): section = line
            if section == '[dependencies]':
                match = re.match(r'"([^\"]+)"\s*=', line)
                if match and match[1] in lookup:
                    dep = lookup[match[1]]
                    note = '# Locked dependency identity (original C lock): ' + dep['name'] + ' ' + dep['version']
                    if dep.get('source', '').startswith('git+'): note += '; commit=' + urlsplit(dep['source'][4:]).fragment
                    lines.append(note)
            lines.append(line)
        manifest.write_text('\n'.join(lines) + '\n')
        record = {'version': self.v, 'path': manifest.relative_to(ROOT).as_posix(), 'package_name': package['name'],
            'package_identity': {k: package[k] for k in ('name', 'version', 'source') if k in package},
            'locked_dependencies': package.get('dependencies', []), 'active_dependencies': active, 'unresolved_dependencies': [],
            'active_dependency_count': len(active), 'unresolved_dependency_count': 0, 'reused_existing_manifest': True,
            'independent_workspace': root.relative_to(ROOT).as_posix(), 'manifest_sha256': digest(manifest)}
        save(manifest.with_suffix('.toml.evidence.json'), record); self.manifests.append(record)

    def workspace(self):
        manifest = self.root / 'Cargo.toml'; old = manifest.read_text()
        if old and not old.startswith(MARKER): raise RuntimeError(f'Refuse overwrite workspace {manifest}')
        members = [p.parent.relative_to(self.root).as_posix() for p in self.paths if not any(p.is_relative_to(w) for w in self.nested_workspaces)]
        text = [MARKER, '# Reconstructed workspace membership; not the original workspace layout.', '[workspace]', 'resolver = "2" # Reconstruction setting; original resolver unknown.', 'members = [']
        text += ['    ' + quote(p) + ',' for p in members]
        text += [']', '', '[workspace.metadata.reconstruction]', 'version = ' + quote(self.v), 'original_workspace_recovered = false',
            'original_features_profiles_patches_and_target_cfg_recovered = false', 'source_dependency_inventory = ' + quote(self.input.relative_to(PROJECT).as_posix())]
        if self.nested_workspaces:
            # Explicitly isolate already reconstructed nested workspaces.
            at = text.index(']') + 1
            text[at:at] = ['exclude = [' + ', '.join(quote(p.relative_to(self.root).as_posix()) for p in self.nested_workspaces) + ']']
        lock_source = PROJECT / 'evidence/multi-version' / f'{self.v}-original-Cargo.lock'
        if lock_source.exists():
            expected = tomllib.loads(lock_source.read_text())['package']; assert expected == self.packages
            dest = self.root / 'Cargo.lock'
            if dest.exists() and dest.read_bytes() != lock_source.read_bytes(): raise RuntimeError('Refuse overwrite a modified skeleton Cargo.lock')
            shutil.copyfile(lock_source, dest)
            text += ['original_lock_sha256 = ' + quote(digest(lock_source)), 'original_lock_copy_preserved = true']
        else: text += ['original_lock_available = false']
        manifest.write_text('\n'.join(text) + '\n')
        return {'path': manifest.relative_to(ROOT).as_posix(), 'members': members, 'manifest_sha256': digest(manifest)}

    def run(self):
        self.observations()
        for path in self.paths: self.write_module(path)
        workspace = self.workspace()
        result = {'version': self.v, 'module_manifests': len(self.manifests), 'workspace': workspace,
            'known_package_versions': sum(r['package_identity'].get('version') is not None for r in self.manifests),
            'active_dependencies': sum(r['active_dependency_count'] for r in self.manifests),
            'unresolved_dependencies': sum(r['unresolved_dependency_count'] for r in self.manifests),
            'lock_evidence_available': bool(self.packages), 'inspection_stub_files': self.stub_files, 'manifests': self.manifests}
        save(ROOT / '_cargo' / f'{self.v}-report.json', result)
        print(json.dumps({k: v for k, v in result.items() if k not in ('manifests', 'workspace')}), flush=True)
        return result

def main():
    (ROOT / '_cargo').mkdir(exist_ok=True)
    scope = ROOT / 'dependency-analysis-scope.json'
    versions = sys.argv[1:] or (json.loads(scope.read_text())['active_versions'] if scope.exists() else ['B', 'C'])
    if any(v not in ('A', 'B', 'C') for v in versions): raise ValueError('Versions must be A, B or C')
    results = [Builder(v).run() for v in versions]
    summary = {'scope': 'Isolated dependency identity reconstruction; original full Cargo manifests unavailable',
        'module_manifests': sum(r['module_manifests'] for r in results), 'workspace_manifests': len(results),
        'active_dependency_entries': sum(r['active_dependencies'] for r in results),
        'unresolved_dependency_entries': sum(r['unresolved_dependencies'] for r in results),
        'known_module_versions': sum(r['known_package_versions'] for r in results),
        'scope_versions': versions,
        'production_project_modified': False,
        'versions': [{k: v for k, v in r.items() if k != 'manifests'} for r in results]}
    save(ROOT / 'cargo-restoration-report.json', summary)

if __name__ == '__main__': main()
