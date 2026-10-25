"""Read and claim a cross-version file task with an exclusive, persistent lock."""
import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import re
import sqlite3

ROOT = Path(__file__).resolve().parents[1]
INDEX = ROOT / 'task-index'

def emit(value): print(json.dumps(value, ensure_ascii=False, indent=2))
def read_payload(db, table, key):
    row = db.execute(f'SELECT payload FROM {table} WHERE id=?', (key,)).fetchone()
    if row is None: raise ValueError('Unknown task/function ID')
    return json.loads(row[0])
def digest(path):
    h = hashlib.sha256()
    with Path(path).open('rb') as f:
        for b in iter(lambda: f.read(1024 * 1024), b''): h.update(b)
    return h.hexdigest()
def claim_status(task_id):
    path = INDEX / 'claims' / (task_id + '.json')
    return json.loads(path.read_text()) if path.exists() else None

def run(args):
    db = sqlite3.connect(f'file:{INDEX / "index.sqlite3"}?mode=ro', uri=True)
    if args.command == 'list':
        table = 'functions' if args.functions else 'tasks'
        where = ['category IS NOT NULL']; values = []
        if args.category: where.append('category=?'); values.append(args.category)
        if args.version:
            where.append(('version' if args.functions else 'target_version') + '=?'); values.append(args.version)
        elif args.functions: where.append('version=?'); values.append('C')
        if not args.functions: where.append("claim_mode!='deferred'")
        if args.search:
            where.append(('(name LIKE ? OR path LIKE ?)' if args.functions else "json_extract(payload,'$.source_path') LIKE ?"))
            values.extend(['%' + args.search + '%'] * (2 if args.functions else 1))
        result = []
        for row in db.execute(f'SELECT payload FROM {table} WHERE ' + ' AND '.join(where) + ' ORDER BY priority DESC', values):
            obj = json.loads(row[0])
            if args.functions:
                file = db.execute('SELECT task_id FROM files WHERE version=? AND path=?', (obj['version'], obj['path'])).fetchone()
                task_id = file[0] if file else None
                if task_id and claim_status(task_id): continue
                if task_id and read_payload(db, 'tasks', task_id)['claim_mode'] == 'deferred': continue
                result.append({k: obj.get(k) for k in ('id', 'logical_group_id', 'version', 'path', 'name', 'category', 'difficulty', 'metrics', 'cross_version_pairs')} | {'file_task_id': task_id})
            else:
                claim = claim_status(obj['id'])
                if claim: continue
                result.append({k: obj.get(k) for k in ('id', 'source_path', 'category', 'target_version', 'versions', 'claim_mode', 'priority_score', 'cross_version_pairs')})
            if len(result) >= args.limit: break
        emit(result)
    elif args.command == 'show-function':
        fn = read_payload(db, 'functions', args.id)
        siblings = [json.loads(row[0]) for row in db.execute('SELECT payload FROM functions WHERE group_id=?', (fn['logical_group_id'],))]
        emit({'function': fn, 'version_counterparts': siblings})
    elif args.command == 'show':
        task = read_payload(db, 'tasks', args.id); task['claim'] = claim_status(args.id)
        if args.functions:
            where = 'path=? AND category IS NOT NULL'; values = [task['source_path']]
            if args.version: where += ' AND version=?'; values.append(args.version)
            task['function_preview'] = [json.loads(row[0]) for row in db.execute('SELECT payload FROM functions WHERE ' + where + ' ORDER BY priority DESC LIMIT ?', values + [args.limit])]
        emit(task)
    elif args.command == 'claim':
        if not re.fullmatch(r'[A-Za-z0-9][A-Za-z0-9_-]{0,31}', args.agent): raise ValueError('Agent name must be 1..32 safe ASCII letters/digits/_/-')
        task = read_payload(db, 'tasks', args.id)
        if task['claim_mode'] == 'deferred' and not args.authorized_deferred_reason:
            raise ValueError('This task is deferred by current project scope; it is excluded from active claims')
        scope = None
        if args.function:
            fn = read_payload(db, 'functions', args.function)
            if fn['path'] != task['source_path'] or not fn['category']: raise ValueError('Function must be an eligible member of this file task')
            scope = {'logical_function_group': fn['logical_group_id'], 'name': fn['name'], 'requested_variant_id': fn['id']}
        for v, source in task['source_inputs'].items():
            if digest(source['path']) != source['sha256']: raise ValueError(f'Stale source snapshot for {v}; rebuild the index before claiming')
        lock = INDEX / 'claims' / (task['id'] + '.json')
        work = INDEX / 'work' / task['id'] / args.agent
        record = {'task_id': task['id'], 'owner': args.agent, 'status': 'claimed', 'claimed_at_utc': datetime.now(timezone.utc).isoformat(),
            'target_version': task['target_version'], 'versions': task['versions'], 'function_scope': scope,
            'work_dir': str(work), 'worker': ('idx-' + task['id'].lower() + '-' + args.agent)[:64],
            'source_inputs': task['source_inputs'], 'handoff': task['handoff'],
            'rule': 'one owner for this A/B/C file family; deliver independent candidate/patch/contract, never overwrite another owner or original artifacts'}
        if args.authorized_deferred_reason:
            record['authorized_deferred_reason'] = args.authorized_deferred_reason
        fd = os.open(lock, os.O_CREAT | os.O_EXCL | os.O_WRONLY, 0o600)
        with os.fdopen(fd, 'w') as f: json.dump(record, f, ensure_ascii=False, indent=2); f.write('\n'); f.flush(); os.fsync(f.fileno())
        work.mkdir(parents=True, exist_ok=True); emit(record)
    elif args.command == 'release':
        lock = INDEX / 'claims' / (args.id + '.json'); record = json.loads(lock.read_text())
        if record['owner'] != args.agent: raise ValueError('Only the recorded owner may release this claim')
        history = INDEX / 'claim-history'; history.mkdir(exist_ok=True)
        target = history / (args.id + '-' + datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%S%fZ') + '.json')
        record['status'] = 'released'; record['release_reason'] = args.reason
        target.write_text(json.dumps(record, ensure_ascii=False, indent=2) + '\n'); lock.unlink()
        emit({'task_id': args.id, 'status': 'released', 'claim_history': str(target)})

def main():
    parser = argparse.ArgumentParser(description=__doc__); sub = parser.add_subparsers(dest='command', required=True)
    p = sub.add_parser('list'); p.add_argument('--category', choices=['tools', 'business', 'core']); p.add_argument('--version', choices=list('ABC')); p.add_argument('--functions', action='store_true'); p.add_argument('--search'); p.add_argument('--limit', type=int, default=10)
    p = sub.add_parser('show'); p.add_argument('id'); p.add_argument('--functions', action='store_true'); p.add_argument('--version', choices=list('ABC')); p.add_argument('--limit', type=int, default=15)
    p = sub.add_parser('show-function'); p.add_argument('id')
    p = sub.add_parser('claim'); p.add_argument('id'); p.add_argument('--agent', required=True); p.add_argument('--function'); p.add_argument('--authorized-deferred-reason', help='Record explicit human authorization for a deferred scope; does not change its index classification')
    p = sub.add_parser('release'); p.add_argument('id'); p.add_argument('--agent', required=True); p.add_argument('--reason', required=True)
    args = parser.parse_args()
    if hasattr(args, 'limit') and not 1 <= args.limit <= 200: parser.error('--limit must be between 1 and 200')
    try: run(args)
    except (ValueError, OSError, KeyError) as exc:
        emit({'ok': False, 'error': str(exc)}); raise SystemExit(2)

if __name__ == '__main__': main()
