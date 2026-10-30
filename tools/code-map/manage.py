#!/usr/bin/env python3
"""Portable setup, evidence transfer, graph rebuild and offline dump management."""
from __future__ import annotations
import argparse
from contextlib import contextmanager
import fcntl
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import sqlite3
import subprocess
import sys
import time
from datetime import datetime, timezone

HERE = Path(__file__).resolve().parent
REPO = HERE.parents[1]
SOURCE = REPO/'reverse-engineering/tiny-glade/tools/code-map'
sys.path.insert(0, str(SOURCE))
from settings import WORKSPACE, RUNTIME, PROJECT, JAVA, LLVM, COLLECTOR, resolve_evidence, origin_workspace, llvm_library
from artifacts import atomic, canonical, digest, db, load, space
from graph import NEO, ARCHIVE_SHA

FORMAT = 1
RUNTIME_FILES = (
    'inventory.json', 'ghidra-functions.jsonl.gz', 'extractor.json', 'capacity.json',
    'extraction-latest.json', 'associated-import.json', 'raw-assembly-manifest.json',
    'range-disputes.json', 'dispatch-boundary-289b680.json', 'dispatch-boundary-289b690.json',
    'import-latest.json', 'published-catalog-binding.json', 'optimization-graph-signature.json',
    'current-graph-signature.json','graph-prune-result.json','graph-filter-audit.json','module-map.json',
    'module-map.graphml', 'module-reviews.json', 'byte-coverage.json', 'receipt-audit.json')

def sha(path):
    with Path(path).open('rb') as handle:
        return hashlib.file_digest(handle, 'sha256').hexdigest()

def run(argv, **kwargs):
    subprocess.run([str(x) for x in argv], check=True, **kwargs)

def cli(*args):
    run([sys.executable, SOURCE/'code_map.py', *args], cwd=WORKSPACE)

@contextmanager
def writer_lock():
    RUNTIME.mkdir(parents=True, exist_ok=True)
    with (RUNTIME/'catalog-writer.lock').open('a') as handle:
        try:
            fcntl.flock(handle, fcntl.LOCK_EX | fcntl.LOCK_NB)
        except BlockingIOError:
            raise RuntimeError('Catalog writer active; do not package a changing catalog')
        yield

def doctor():
    checks = []
    def check(name, operation):
        try:
            value = operation()
            checks.append({'name':name, 'ok':True, 'value':value})
        except Exception as error:
            checks.append({'name':name, 'ok':False, 'error':str(error)})
    def java():
        text = subprocess.check_output([str(JAVA/'bin/java'), '-version'], stderr=subprocess.STDOUT, text=True)
        if not re.search(r'version "21[.\"]', text):
            raise ValueError('An independent Java 21 installation is required')
        return text.splitlines()[0]
    def llvm():
        text = subprocess.check_output([str(LLVM/'bin/llvm-objdump'), '--version'], text=True).splitlines()[0]
        from artifacts import Decoder
        decoder = Decoder()
        try:
            ins, gaps = decoder.decode(bytes.fromhex('e805000000c3'), 0x1000, 0x140000000)
            if gaps or ins[0][1] != 'e805000000':
                raise ValueError('x86_64 decoder smoke failed')
        finally:
            decoder.close()
        return {'version':text, 'library':str(llvm_library()), 'library_sha256':sha(llvm_library())}
    def collector():
        from artifacts import pack_module
        if not hasattr(pack_module(), 'PE'):
            raise ValueError('Collector lacks PE reader')
        return {'path':str(COLLECTOR), 'sha256':sha(COLLECTOR)}
    def disk():
        ancestor = RUNTIME
        while not ancestor.exists():
            ancestor = ancestor.parent
        free = shutil.disk_usage(ancestor).free/2**30
        if free < 50:
            raise RuntimeError(f'{free:.2f} GiB available; require 50 GiB before setup')
        return round(free, 2)
    check('python_3_11_8+', lambda: sys.version if sys.version_info >= (3,11,8) else (_ for _ in ()).throw(ValueError('Use Python 3.11.8+')))
    check('java_21', java)
    check('llvm_x86_64', llvm)
    check('collector', collector)
    check('ripgrep', lambda: subprocess.check_output(['rg', '--version'], text=True).splitlines()[0])
    check('free_gib', disk)
    result = {'ok':all(x['ok'] for x in checks), 'workspace':str(WORKSPACE), 'runtime':str(RUNTIME), 'checks':checks}
    print(json.dumps(result, ensure_ascii=False, indent=2))
    if not result['ok']:
        raise RuntimeError('Fix failed prerequisites; setup has not changed the runtime')
    return result

def setup(mcp=False):
    doctor()
    run([sys.executable, SOURCE/'bootstrap.py'])
    if mcp:
        venv = RUNTIME/'venv'
        run([sys.executable, '-m', 'venv', venv])
        # Match the direct network policy without altering global settings.
        env = {k:v for k,v in os.environ.items() if k.lower() not in ('http_proxy','https_proxy','all_proxy')}
        run([venv/'bin/python', '-m', 'pip', '--isolated', 'install', '--index-url',
             'https://pypi.org/simple', '--cache-dir', RUNTIME/'pip-cache',
             '--require-hashes', '-r', HERE/'requirements-mcp.lock'], env=env)

def safe_relative(value):
    path = Path(value)
    if path.is_absolute() or not path.parts or any(x in ('..', '.') for x in path.parts):
        raise ValueError('Unsafe bundle path: '+str(value))
    return path

def bundle_manifest(folder):
    folder = Path(folder).resolve()
    manifest = json.loads((folder/'manifest.json').read_text())
    if manifest.get('format') != FORMAT or manifest.get('complete') is not True:
        raise ValueError('Unpublished or unsupported transfer bundle')
    seen = set()
    for record in manifest['files']:
        relative = safe_relative(record['path'])
        if str(relative) in seen:
            raise ValueError('Duplicate manifest path')
        seen.add(str(relative))
        path = folder/relative
        if not path.resolve().is_relative_to(folder) or path.is_symlink():
            raise ValueError('Bundle symlink or escape rejected: '+str(relative))
        if path.stat().st_size != record['bytes'] or sha(path) != record['sha256']:
            raise ValueError('Bundle integrity failure: '+str(relative))
    all_paths = list(folder.rglob('*'))
    if any(p.is_symlink() for p in all_paths):
        raise ValueError('Bundle symlink rejected')
    actual = {str(p.relative_to(folder)) for p in all_paths if p.is_file() and p != folder/'manifest.json'}
    if actual != seen:
        raise ValueError('Bundle has unlisted or missing files')
    return manifest

def inputs_for_package():
    """An explicit dependency closure; never walk the whole workspace."""
    files = {}
    expected = {}
    def add(path, wanted=None):
        path = resolve_evidence(path).resolve()
        if not path.is_relative_to(WORKSPACE):
            raise ValueError('Input outside workspace requires explicit relocation: '+str(path))
        if not path.is_file():
            raise FileNotFoundError(path)
        relative = path.relative_to(WORKSPACE)
        if path == RUNTIME/'connection.json':
            raise ValueError('Credentials cannot be packaged')
        files[str(relative)] = path
        if wanted:
            expected[str(relative)] = wanted
    con = db(True)
    try:
        for table in ('inputs','enrich_inputs'):
            for record in con.execute('SELECT path,sha FROM '+table):
                add(record['path'], record['sha'])
    finally:
        con.close()
    baselines = load(PROJECT/'tracking/baselines.json')['baselines']
    for b in baselines.values():
        for kind in ('binary','pdb'):
            add(b[kind]['path'], b[kind]['sha256'])
    add(WORKSPACE/'B版本内容/tiny-glade.exe', 'cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323')
    for name in ('baselines.json','dependency-exclusions.json','module-coverage.json'):
        add(PROJECT/'tracking'/name)
    for version in 'ABC':
        add(PROJECT/f'skeleton/_pdb/{version}/enrichment/metadata-stage.json')
        p = PROJECT/f'skeleton/_dependencies/{version}/native-input.json'
        if p.exists():
            add(p)
    for row in load(SOURCE/'chain.json')['evidence']:
        add(row['path'])  # Current implementation may be stale; retain its honest status.
    # Retain the filtered source-association results as analysis outputs.
    for name in ('audit_b_module_inventory.py','audit_m06_business_functions.py','test_inventory_associations.py'):
        add(WORKSPACE/'tiny-glade/tools'/name)
    add(PROJECT/'skeleton/task-index/functions.jsonl.gz')
    for name in ('B-MODULE-INVENTORY.json','B-MODULE-INVENTORY.csv','B-MODULE-INVENTORY.md',
                 'M06-BUSINESS-FUNCTIONS-2026-10-30.json','M06-BUSINESS-FUNCTIONS-2026-10-30.md'):
        add(WORKSPACE/'tiny-glade/reports/business/2026-10-23-module-recovery'/name)
    raw = load(RUNTIME/'raw-assembly-manifest.json')
    for row in raw['files']:
        add(row['path'], row['sha256'])
    # Query/verification code and portable launcher are small source-only trees.
    for root in (SOURCE, HERE):
        for path in root.rglob('*'):
            if path.is_file() and '__pycache__' not in path.parts and path.suffix != '.pyc':
                add(path)
    for name in RUNTIME_FILES:
        p = RUNTIME/name
        if p.exists():
            add(p)
    # Only known immutable artifact stores, never Neo4j live files/logs.
    for name in ('assembly','raw-assembly','proof-cas','extractor-revisions'):
        for path in (RUNTIME/name).rglob('*'):
            if path.is_file():
                add(path)
    for path in RUNTIME.glob('dispatch-boundary-*.json'):
        add(path)
    archive = RUNTIME/'downloads/neo4j-community-5.26.32-unix.tar.gz'
    if archive.exists():
        add(archive, ARCHIVE_SHA)
    return files, expected

def require_stopped():
    env = dict(os.environ, JAVA_HOME=str(JAVA), NEO4J_HOME=str(NEO))
    p = subprocess.run([str(NEO/'bin/neo4j'), 'status'], env=env, capture_output=True, text=True)
    if p.returncode != 3:
        raise RuntimeError('Offline operation requires Neo4j status=3 (not running); no service was stopped')
    return env

def package(output, with_dump=False):
    output = Path(output).expanduser().resolve()
    if output.is_relative_to(SOURCE) or output.is_relative_to(HERE) or output.is_relative_to(RUNTIME):
        raise ValueError('Bundle must be outside tool source and runtime')
    if output.exists():
        raise FileExistsError('Choose a new output directory; partial bundles are preserved')
    with writer_lock():
        files, expected = inputs_for_package()
        needed = sum(p.stat().st_size for p in files.values()) + (RUNTIME/'catalog.sqlite3').stat().st_size
        output.parent.mkdir(parents=True, exist_ok=True)
        reserve = needed + 5*2**30
        if with_dump:
            env = require_stopped()
            reserve += sum(p.stat().st_size for p in (NEO/'data/databases/neo4j').rglob('*') if p.is_file())
        if shutil.disk_usage(output.parent).free < reserve:
            raise RuntimeError('Not enough space for bundle plus 5 GiB reserve')
        # Check identity-bound inputs BEFORE creating a candidate bundle.
        for name,wanted in expected.items():
            if sha(files[name]) != wanted:
                raise ValueError('Recorded input changed; preserve/audit before packaging: '+name)
        output.mkdir()
        payload = output/'payload'
        for i,(name,path) in enumerate(sorted(files.items())):
            target = payload/name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(path, target)
            if name in expected and sha(target) != expected[name]:
                raise ValueError('Input changed while copying: '+name)
            if i % 1000 == 0:
                print(json.dumps({'packaged_files':i}), flush=True)
        runtime_relative = RUNTIME.relative_to(WORKSPACE)
        # SQLite online backup API; never copy a live DB/WAL byte stream.
        catalog = payload/runtime_relative/'catalog.sqlite3'
        catalog.parent.mkdir(parents=True, exist_ok=True)
        with db(True) as source, sqlite3.connect(catalog) as target:
            source.backup(target)
            if target.execute('PRAGMA integrity_check').fetchone()[0] != 'ok':
                raise ValueError('Catalog backup integrity check failed')
        binding_path = payload/runtime_relative/'published-catalog-binding.json'
        binding = load(RUNTIME/'published-catalog-binding.json')
        if binding['catalog_sha256'] != sha(RUNTIME/'catalog.sqlite3'):
            raise ValueError('Catalog advanced after publication; publish/audit before packaging')
        binding['catalog_sha256'] = sha(catalog)
        atomic(binding_path, binding)
        if with_dump:
            dump = output/'dumps'
            dump.mkdir()
            run([NEO/'bin/neo4j-admin','database','dump','neo4j','--to-path='+str(dump)], env=env)
        (output/'README.txt').write_text(
            'Tiny Glade code-map transfer bundle\n'
            'Verify: python3 payload/tools/code-map/manage.py verify-bundle .\n'
            'Restore: python3 payload/tools/code-map/manage.py restore . --workspace /your/game\n'
            'Then read tools/code-map/README.md in the restored workspace.\n'
            'Credentials and live Neo4j files are excluded. Original local artifacts are included.\n', encoding='utf-8')
        records = []
        for path in sorted(output.rglob('*')):
            if path.is_file():
                records.append({'path':str(path.relative_to(output)), 'bytes':path.stat().st_size, 'sha256':sha(path)})
        signature_path = RUNTIME/'current-graph-signature.json'
        if not signature_path.exists():signature_path = RUNTIME/'optimization-graph-signature.json'
        reference_signature = load(signature_path) if signature_path.exists() else None
        if reference_signature and reference_signature['snapshot'] != binding['snapshot']:
            reference_signature = None
        manifest = {'format':FORMAT, 'complete':True, 'created_at_utc':datetime.now(timezone.utc).isoformat(),
                    'origin_workspace':str(origin_workspace()), 'packaged_workspace':str(WORKSPACE),
                    'runtime_relative':str(runtime_relative), 'snapshot':binding['snapshot'],
                    'mode':'offline-dump' if with_dump else 'checkpoint-rebuild',
                    'neo4j':'5.26.32', 'archive_sha256':ARCHIVE_SHA,
                    'recorded_reference_signature':reference_signature,
                    'files':records, 'bytes':sum(x['bytes'] for x in records),
                    'semantic_review_complete':False}
        atomic(output/'manifest.json', manifest)
        print(json.dumps({'ok':True,'bundle':str(output),'mode':manifest['mode'],
                          'files':len(records),'bytes':manifest['bytes'],'snapshot':manifest['snapshot']}, ensure_ascii=False))

def restore(folder, destination):
    folder = Path(folder).resolve()
    manifest = bundle_manifest(folder)  # Verify all bytes before any destination writes.
    destination = Path(destination).expanduser().resolve()
    if destination.exists() and any(destination.iterdir()):
        raise FileExistsError('Restore into an empty directory; existing workspaces are never overwritten')
    ancestor = destination.parent
    while not ancestor.exists():
        ancestor = ancestor.parent
    required = max(50*2**30, manifest['bytes']+5*2**30)
    if shutil.disk_usage(ancestor).free < required:
        raise RuntimeError('Destination needs at least 50 GiB and transfer bytes plus 5 GiB reserve')
    destination.mkdir(parents=True, exist_ok=True)
    for row in manifest['files']:
        relative = safe_relative(row['path'])
        if relative.parts[0] != 'payload':
            continue
        target = destination/relative.relative_to('payload')
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(folder/relative, target)
        if sha(target) != row['sha256']:
            raise ValueError('Restore copy integrity failure: '+str(relative))
    runtime = destination/safe_relative(manifest['runtime_relative'])
    runtime.mkdir(parents=True, exist_ok=True)
    atomic(runtime/'relocation.json', {'origin_workspace':manifest['origin_workspace'], 'restored_from':str(folder)})
    # A custom runtime is made explicit rather than silently defaulting elsewhere.
    atomic(destination/'tools/code-map.local.json', {'runtime':manifest['runtime_relative']})
    atomic(runtime/'transfer-receipt.json', {'snapshot':manifest['snapshot'], 'mode':manifest['mode'],
        'recorded_reference_signature':manifest.get('recorded_reference_signature'),
        'bundle_manifest_sha256':sha(folder/'manifest.json'), 'workspace':str(destination),
        'graph_restored':False, 'game_behavior_revalidated':False})
    print(json.dumps({'ok':True,'workspace':str(destination),'next':'Set Java/LLVM paths, then run manage.py setup; see README'},ensure_ascii=False))

def load_dump(folder):
    manifest = bundle_manifest(folder)
    folder = Path(folder).resolve()
    if manifest['mode'] != 'offline-dump':
        raise ValueError('This bundle contains a checkpoint, not a database dump; use rebuild')
    binding = load(RUNTIME/'published-catalog-binding.json')
    if binding['snapshot'] != manifest['snapshot'] or binding['catalog_sha256'] != sha(RUNTIME/'catalog.sqlite3'):
        raise ValueError('Local catalog does not match transfer snapshot')
    env = require_stopped()
    store = NEO/'data/databases/neo4j'
    if store.exists() and any(store.iterdir()):
        raise RuntimeError('A target database already exists; replacement is refused')
    run([NEO/'bin/neo4j-admin','database','load','neo4j','--from-path='+str(Path(folder).resolve()/'dumps')],env=env)
    print(json.dumps({'ok':True,'loaded_snapshot':manifest['snapshot'],'next':'Run console, then check'}))

def rebuild(from_scratch=False):
    if from_scratch:
        if (RUNTIME/'catalog.sqlite3').exists():
            raise RuntimeError('Fresh rebuild requires a new runtime; existing evidence is preserved')
        for command in ('inventory','capacity','enrich','associated','archive-existing','dispatch-boundaries','extract'):
            cli(command)
    elif not (RUNTIME/'catalog.sqlite3').is_file():
        raise RuntimeError('Restore a checkpoint first, or use --from-scratch with prepared inputs')
    # Retained shards are portable; do not rerun extraction with a different LLVM
    # identity merely to migrate an already complete catalog.
    for command in ('import','optimize-search','export-map','module-reviews','byte-coverage'):
        cli(command)

def check(full=False):
    from integration_checks import graph_signature
    from api import query_tool
    signature = graph_signature()
    receipt_path = RUNTIME/'transfer-receipt.json'
    if receipt_path.exists():
        receipt = load(receipt_path)
        if receipt['mode'] == 'offline-dump' and signature['snapshot'] != receipt['snapshot']:
            raise ValueError('Restored dump snapshot differs from transfer receipt')
        reference = receipt.get('recorded_reference_signature')
        if reference and any(signature[k] != reference[k] for k in ('nodes','edges','sample')):
            raise ValueError('Restored graph differs from the recorded reference signature')
    for tool,args in [('search',{'text':'0x1408cdc60'}), ('assembly',{'key':'function:B:8cc870','limit':100}),
                      ('state_users',{'key':'state:B:WorldRoofTable'}), ('evidence_pack',{'key':'function:B:8cdc60'})]:
        answer = query_tool(tool,args)
        if not answer.get('rows') or answer['source_binding']['original_evidence_errors']:
            raise ValueError('Restored real-chain query failed: '+tool)
    cli('verify', *(['--full'] if full else []))
    atomic(RUNTIME/'transfer-validation.json', {'ok':True, 'signature':signature,
           'validated_on_host':True, 'game_behavior_revalidated':False})
    print(json.dumps({'ok':True,'signature':signature},ensure_ascii=False))

def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest='command', required=True)
    sub.add_parser('doctor')
    p=sub.add_parser('setup');p.add_argument('--mcp',action='store_true')
    sub.add_parser('console')
    p=sub.add_parser('package');p.add_argument('--output',required=True);p.add_argument('--dump',action='store_true')
    p=sub.add_parser('verify-bundle');p.add_argument('bundle')
    p=sub.add_parser('restore');p.add_argument('bundle');p.add_argument('--workspace',required=True)
    p=sub.add_parser('load-dump');p.add_argument('bundle')
    p=sub.add_parser('rebuild');p.add_argument('--from-scratch',action='store_true')
    p=sub.add_parser('check');p.add_argument('--full',action='store_true')
    a=parser.parse_args()
    if a.command=='doctor':doctor()
    elif a.command=='setup':setup(a.mcp)
    elif a.command=='console':cli('runtime','console')
    elif a.command=='package':package(a.output,a.dump)
    elif a.command=='verify-bundle':
        m=bundle_manifest(a.bundle);print(json.dumps({'ok':True,'files':len(m['files']),'mode':m['mode'],'snapshot':m['snapshot']}))
    elif a.command=='restore':restore(a.bundle,a.workspace)
    elif a.command=='load-dump':load_dump(a.bundle)
    elif a.command=='rebuild':rebuild(a.from_scratch)
    else:check(a.full)

if __name__=='__main__':
    try:
        main()
    except Exception as error:
        print(json.dumps({'ok':False,'error':type(error).__name__,'message':str(error)},ensure_ascii=False),file=sys.stderr)
        sys.exit(2)
