"""Check index completeness, evidence labels, grouped tasks and claim isolation."""
import argparse
from contextlib import redirect_stdout
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import sqlite3
import tempfile

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / 'task-index'

def main():
    db = sqlite3.connect(OUT / 'index.sqlite3'); summary = json.loads((OUT / 'summary.json').read_text())
    actual = {(v, p.relative_to(ROOT / v).as_posix()) for v in 'ABC' for p in (ROOT / v).rglob('*.rs')}
    indexed = set(db.execute('SELECT version,path FROM files'))
    assert actual == indexed
    assert db.execute('SELECT count(*) FROM functions').fetchone()[0] == summary['function_variants']
    assert db.execute('SELECT count(*) FROM tasks').fetchone()[0] == summary['tasks']
    assert db.execute('SELECT count(*) FROM edges e LEFT JOIN functions f ON e.caller=f.id WHERE f.id IS NULL').fetchone()[0] == 0
    assert db.execute('SELECT count(*) FROM edges e LEFT JOIN functions f ON e.callee=f.id WHERE f.id IS NULL').fetchone()[0] == 0
    assert db.execute("SELECT count(*) FROM files WHERE excluded_reason IS NOT NULL AND task_id IS NOT NULL").fetchone()[0] == 0
    assert db.execute("SELECT count(*) FROM functions WHERE path LIKE 'crates/frozen/%' AND category IS NOT NULL").fetchone()[0] == 0
    bad_semantic_labels = 0; function_pair_records = 0
    for payload, in db.execute('SELECT payload FROM functions'):
        fn = json.loads(payload)
        for pair in fn['cross_version_pairs']:
            function_pair_records += 1
            if pair['behavior_equivalence'] != 'unverified': bad_semantic_labels += 1
            if pair['same_native_byte_sets']: assert pair['some_native_instances_identical'] and pair['shared_body_sha256']
        if fn['category'] == 'tools':
            assert fn['body_sizes'] and max(fn['body_sizes']) <= 256
            assert fn['metrics']['unique_private_callees'] <= 2
            assert fn['indirect_call_groups'] == 0 and fn['ambiguous_alias_groups'] == 0
    assert bad_semantic_labels == 0
    audits = 0
    for payload, in db.execute('SELECT payload FROM tasks'):
        task = json.loads(payload)
        assert task['target_version'] == next(v for v in 'CBA' if v in task['versions'])
        assert task['assignment_unit'].startswith('one source-file family')
        audit = json.loads((ROOT / task['handoff_audit']).read_text())
        assert audit['ok'] and audit['semantic_acceptance'] is False
        audits += 1
        # Each target variant resolves to exactly this one file-family lock.
        for v in task['versions']:
            row = db.execute('SELECT task_id FROM files WHERE version=? AND path=?', (v, task['source_path'])).fetchone()
            assert row[0] == task['id']
    spec = importlib.util.spec_from_file_location('queue_under_test', ROOT / '_tools/task_queue.py')
    queue = importlib.util.module_from_spec(spec); spec.loader.exec_module(queue)
    with tempfile.TemporaryDirectory(prefix='tg-index-claim-test-') as temp:
        fixture = Path(temp); (fixture / 'claims').mkdir(); source = fixture / 'source.rs'; source.write_text('// immutable fixture\n')
        fixture_db = sqlite3.connect(fixture / 'index.sqlite3'); fixture_db.execute('CREATE TABLE tasks(id TEXT PRIMARY KEY,payload TEXT)')
        data = {'id': 'TG-fixture', 'source_path': 'crates/utils/src/example.rs', 'target_version': 'C', 'versions': ['A', 'B', 'C'], 'claim_mode': 'candidate_reconstruction',
            'source_inputs': {v: {'path': str(source), 'sha256': hashlib.sha256(source.read_bytes()).hexdigest()} for v in 'ABC'}, 'handoff': 'fixture-handoff.json'}
        fixture_db.execute('INSERT INTO tasks VALUES(?,?)', (data['id'], json.dumps(data))); fixture_db.commit(); fixture_db.close()
        queue.INDEX = fixture
        owner = argparse.Namespace(command='claim', id='TG-fixture', agent='owner_a', function=None)
        with redirect_stdout(io.StringIO()): queue.run(owner)
        lock = fixture / 'claims/TG-fixture.json'; assert json.loads(lock.read_text())['versions'] == ['A', 'B', 'C']
        try:
            with redirect_stdout(io.StringIO()): queue.run(argparse.Namespace(command='claim', id='TG-fixture', agent='owner_b', function=None))
            raise AssertionError('Duplicate cross-version owner was accepted')
        except FileExistsError: pass
        try:
            with redirect_stdout(io.StringIO()): queue.run(argparse.Namespace(command='release', id='TG-fixture', agent='owner_b', reason='wrong owner'))
            raise AssertionError('Wrong owner released the lock')
        except ValueError: pass
        assert lock.exists()
        with redirect_stdout(io.StringIO()): queue.run(argparse.Namespace(command='release', id='TG-fixture', agent='owner_a', reason='fixture finished'))
        assert not lock.exists() and list((fixture / 'claim-history').glob('*.json'))
        source.write_text('// changed source\n')
        try:
            with redirect_stdout(io.StringIO()): queue.run(owner)
            raise AssertionError('Stale source snapshot was accepted')
        except ValueError: pass
    report = {'ok': True, 'actual_source_files_indexed': len(actual), 'function_variants': summary['function_variants'], 'audited_cross_version_tasks': audits,
        'evolution_pair_records_checked': function_pair_records, 'no_behavior_equivalence_inferred': True,
        'exclusive_cross_version_claim_test': True, 'wrong_owner_release_rejected': True, 'stale_snapshot_claim_rejected': True,
        'real_tasks_claimed_by_verifier': 0, 'scope': 'Index and coordination checks only; no game build/oracle/runtime execution'}
    (OUT / 'verification.json').write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n'); print(json.dumps(report, ensure_ascii=False))

if __name__ == '__main__': main()
