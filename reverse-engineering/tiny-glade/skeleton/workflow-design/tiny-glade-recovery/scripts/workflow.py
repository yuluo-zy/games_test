#!/usr/bin/env python3
"""Prepare indexed role prompts and content snapshots; never claim or run MCP."""
import argparse
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import re
import sqlite3
import sys

SKILL = Path(__file__).resolve().parents[1]
ROLES = ('coordinator', 'evidence', 'restore-tools', 'restore-business',
         'restore-core', 'reviewer', 'validator', 'integrator')
REVIEW_ROLES = {'reviewer', 'validator', 'integrator'}


def sha(path):
    h = hashlib.sha256()
    with Path(path).open('rb') as stream:
        for block in iter(lambda: stream.read(1024 * 1024), b''):
            h.update(block)
    return h.hexdigest()


def json_text(value):
    return json.dumps(value, ensure_ascii=False, indent=2)


def payload_sha(value):
    return hashlib.sha256(json.dumps(value, ensure_ascii=False,
                                    sort_keys=True).encode()).hexdigest()


def safe_name(value):
    if not re.fullmatch(r'[A-Za-z0-9][A-Za-z0-9_-]{0,31}', value):
        raise ValueError('Owner/actor must be 1..32 safe ASCII letters/digits/_/-')
    return value


def inside(root, value):
    root = Path(root).resolve()
    path = Path(value)
    path = (path if path.is_absolute() else root / path).resolve()
    if not path.is_relative_to(root):
        raise ValueError('Path escapes the allowed root: ' + str(value))
    return path


def write_new(path, content):
    path.parent.mkdir(parents=True, exist_ok=True)
    # Exclusive creation: do not overwrite prompts, snapshots or prior reviews.
    with path.open('x', encoding='utf-8') as stream:
        stream.write(content)


def project_root(value=None):
    candidates = [Path(value).resolve()] if value else list(SKILL.parents)
    for root in candidates:
        if ((root / 'AI_REVERSE_TOOLS.md').is_file()
                and (root / 'skeleton/task-index/index.sqlite3').is_file()):
            return root
    raise ValueError('No Tiny Glade project root with guide and existing index')


def read_payload(db, table, key):
    row = db.execute(f'SELECT payload FROM {table} WHERE id=?', (key,)).fetchone()
    if row is None:
        raise ValueError('Unknown indexed task/function ID: ' + key)
    return json.loads(row[0])


def check_snapshot(path):
    path = Path(path).resolve()
    data = json.loads(path.read_text())
    if (data.get('schema') != 'tiny-glade-delivery/v1'
            or data.get('semantic_acceptance') is not False
            or not isinstance(data.get('files'), dict) or not data['files']):
        raise ValueError('Invalid delivery snapshot schema')
    root = Path(data['root']).resolve()
    errors = []
    for name, expected in data['files'].items():
        if not re.fullmatch(r'[0-9a-f]{64}', str(expected)):
            raise ValueError('Invalid SHA-256 in snapshot: ' + name)
        file = inside(root, name)
        if not file.is_file():
            errors.append('Missing file: ' + name)
        elif sha(file) != expected:
            errors.append('Changed file: ' + name)
    return {'ok': not errors, 'semantic_acceptance': False,
            'manifest_sha256': sha(path), 'root': str(root),
            'file_count': len(data['files']), 'errors': errors,
            'scope': 'Only listed content hashes; no semantic or closure proof'}


def freeze(args):
    root = args.root.resolve()
    if not root.is_dir():
        raise ValueError('Snapshot root must be an existing isolated work directory')
    output = inside(root, args.output)
    files = {}
    for name in args.file:
        file = inside(root, name)
        if file == output:
            raise ValueError('Snapshot cannot include itself')
        if not file.is_file():
            raise ValueError('Missing delivery file: ' + name)
        files[str(file.relative_to(root))] = sha(file)
    data = {'schema': 'tiny-glade-delivery/v1', 'root': str(root),
            'created_at_utc': datetime.now(timezone.utc).isoformat(),
            'semantic_acceptance': False, 'files': files,
            'scope': 'Declared files only; reviewer must check dependency closure'}
    write_new(output, json_text(data) + '\n')
    print(json_text(check_snapshot(output)))


def render(template, fields):
    def replace(match):
        name = match.group(1)
        if name not in fields:
            raise ValueError('Unknown template field: ' + name)
        return str(fields[name])
    # One pass: inserted metadata text cannot expand as a template instruction.
    return re.sub(r'\{\{([A-Z_]+)\}\}', replace, template)


def prepare(args):
    project = project_root(args.project)
    skeleton = project / 'skeleton'
    index = skeleton / 'task-index'
    with sqlite3.connect((index / 'index.sqlite3').as_uri() + '?mode=ro', uri=True) as db:
        task = read_payload(db, 'tasks', args.task)
        selected = read_payload(db, 'functions', args.function) if args.function else None
        if selected and (selected['id'] not in task['function_ids']
                         or not selected.get('category')):
            raise ValueError('Function is not an eligible member of this task')
        if task.get('claim_mode') == 'deferred' and args.output:
            raise ValueError('Deferred task may be previewed only')
        # Check only this task's declared inputs, not an expensive full-index rehash.
        for version, source in task['source_inputs'].items():
            if sha(source['path']) != source['sha256']:
                raise ValueError('Stale indexed source: ' + version)
        claim_path = index / 'claims' / (task['id'] + '.json')
        claim = json.loads(claim_path.read_text()) if claim_path.exists() else None
        claim_hash = sha(claim_path) if claim else None
        owner = safe_name(args.owner) if args.owner else None
        actor = safe_name(args.actor) if args.actor else owner
        owned = bool(claim and owner and claim['owner'] == owner
                     and claim.get('status') == 'claimed')
        if args.output and not owned:
            raise ValueError('Saving a prompt requires the existing matching claim owner')
        scope = claim.get('function_scope') if owned else None
        if owned and claim['source_inputs'] != task['source_inputs']:
            raise ValueError('Claim source snapshot differs from current index')
        if scope and selected and selected['logical_group_id'] != scope['logical_function_group']:
            raise ValueError('Requested function escapes the owner function scope')
        group = (scope['logical_function_group'] if scope else
                 selected['logical_group_id'] if selected else None)
        query = 'SELECT payload FROM functions WHERE path=? AND version=? AND category IS NOT NULL'
        values = [task['source_path'], task['target_version']]
        if group:
            query += ' AND group_id=?'
            values.append(group)
        query += ' ORDER BY priority DESC'
        rows = [json.loads(row[0]) for row in db.execute(query, values)]
    if not rows and args.role.startswith('restore-'):
        raise ValueError('No eligible target-version function in requested scope')
    if args.output and args.role == 'reviewer' and (not actor or actor == owner):
        raise ValueError('Reviewer output requires a distinct --actor')
    delivery = 'none; review/integration is not ready'
    delivery_info = None
    if args.delivery:
        if not owned:
            raise ValueError('Delivery task packets require a matching claim owner')
        delivery_path = inside(Path(claim['work_dir']), args.delivery)
        delivery_info = check_snapshot(delivery_path)
        if not delivery_info['ok']:
            raise ValueError('Delivery changed: ' + '; '.join(delivery_info['errors']))
        if Path(delivery_info['root']).resolve() != Path(claim['work_dir']).resolve():
            raise ValueError('Delivery root must match the claim work_dir')
        delivery = str(delivery_path) + ' sha256=' + delivery_info['manifest_sha256']
    if args.output and args.role in REVIEW_ROLES and not delivery_info:
        raise ValueError('This role output requires a valid --delivery snapshot')
    function_keys = ('id', 'logical_group_id', 'version', 'name', 'category', 'difficulty',
                     'metrics', 'native_instances', 'reasons', 'cross_version_pairs',
                     'existing_rust_name_candidates', 'retained_B_native_evidence')
    functions = [{key: row.get(key) for key in function_keys} for row in rows[:args.limit]]
    for item in functions:
        instances = item.get('native_instances') or []
        item['native_instance_count'] = len(instances)
        item['native_instances_truncated'] = len(instances) > 6
        item['native_instances'] = instances[:6]
    sources = {}
    for version, source in task['source_inputs'].items():
        file = Path(source['path'])
        sidecars = [str(file) + suffix for suffix in
                    ('.pdb.json', '.pdb.jsonl.gz', '.dependencies.json',
                     '.dependencies.jsonl.gz', '.enriched.json.gz', '.structures.json.gz')]
        sources[version] = source | {'existing_sidecars': [p for p in sidecars if Path(p).is_file()]}
    summary = {'task_payload_sha256': payload_sha(task),
               'selected_function_payload_sha256': payload_sha(rows[:args.limit]),
               'claim_sha256': claim_hash, 'claim_mode': task.get('claim_mode'),
               'source_inputs': sources, 'function_scope': scope or group,
               'eligible_target_function_count': len(rows),
               'shown_functions': len(functions), 'scope_truncated': len(rows) > args.limit,
               'functions': functions, 'cross_version_pairs': task['cross_version_pairs'],
               'related_callee_task_count': len(task.get('related_callee_tasks', [])),
               'related_caller_task_count': len(task.get('related_caller_tasks', [])),
               'related_tasks_preview_limit': 12,
               'related_tasks_truncated': any(len(task.get(k, [])) > 12 for k in
                                               ('related_callee_tasks', 'related_caller_tasks')),
               'related_callee_tasks': task.get('related_callee_tasks', [])[:12],
               'related_caller_tasks': task.get('related_caller_tasks', [])[:12],
               'existing_reconstruction_candidates': task.get('existing_reconstruction_candidates', []),
               'data_structure_metadata': task.get('data_structure_metadata', {}),
               'sqlite': str(index / 'index.sqlite3'),
               'task_descriptor': str(index / 'tasks' / (task['id'] + '.json')),
               'handoff': str(skeleton / task['handoff']),
               'semantic_acceptance': False}
    work = Path(claim['work_dir']).resolve() if owned else None
    if actor and owned:
        worker = (claim['worker'] if actor == owner else
                  'idx-' + task['id'].lower() + '-' + actor)
        if len(worker) > 64:
            raise ValueError('Actor worker exceeds MCP 64-character bound')
    else:
        worker = 'not allocated; preview only'
    entries = ['AGENTS.md', 'PROJECT_MEMORY.md', 'TASK_BACKLOG.md', 'DEPENDENCY_FILTERING.md',
               'AI_REVERSE_TOOLS.md', 'tracking/dependency-exclusions.json',
               'tracking/baselines.json', 'tracking/parallel-implementation-conflicts',
               'skeleton/task-index/README.md', 'skeleton/task-index/AGENTS.md']
    fields = {'SKILL_PATH': SKILL / 'SKILL.md', 'ROLE': args.role, 'TASK_ID': task['id'],
              'TARGET_VERSION': task['target_version'], 'SOURCE_PATH': task['source_path'],
              'OWNER': owner or 'unassigned', 'ACTOR': actor or 'unassigned',
              'CLAIM_STATE': 'matching_owner' if owned else 'preview_only',
              'WORK_DIR': str(work) if work else 'none; claim first', 'WORKER': worker,
              'IMPLEMENTATION': args.implementation, 'VALIDATION': args.validation,
              'DELIVERY': delivery,
              'ENTRY_PATHS': '\n'.join('- ' + str(project / p) for p in entries),
              'INDEX_SUMMARY': json_text(summary)}
    common = (SKILL / 'assets/tasks/common.md').read_text()
    role = (SKILL / 'assets/tasks' / (args.role + '.md')).read_text()
    fields['COMMON'] = render(common, fields)
    prompt = render(role, fields)
    if args.output:
        output = inside(work, args.output)
        # Recheck ownership at publication, not just when reading the index.
        if not claim_path.is_file() or sha(claim_path) != claim_hash:
            raise ValueError('Claim changed while preparing the prompt')
        write_new(output, prompt)
        print(json_text({'ok': True, 'prompt': str(output), 'sha256': sha(output),
                         'task_id': task['id'], 'role': args.role,
                         'semantic_acceptance': False, 'mcp_executed': False,
                         'claim_created': False}))
    else:
        print(prompt)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest='command', required=True)
    p = sub.add_parser('prepare')
    p.add_argument('--project', type=Path)
    p.add_argument('--task', required=True)
    p.add_argument('--role', choices=ROLES, required=True)
    p.add_argument('--function')
    p.add_argument('--owner')
    p.add_argument('--actor')
    p.add_argument('--limit', type=int, default=8)
    p.add_argument('--implementation', choices=('forbidden', 'allowed'), default='forbidden')
    p.add_argument('--validation', choices=('paused', 'enabled'), default='paused')
    p.add_argument('--delivery', type=Path)
    p.add_argument('--output', type=Path)
    p = sub.add_parser('freeze')
    p.add_argument('--root', type=Path, required=True)
    p.add_argument('--file', action='append', required=True)
    p.add_argument('--output', type=Path, required=True)
    p = sub.add_parser('check')
    p.add_argument('--manifest', type=Path, required=True)
    args = parser.parse_args()
    if args.command == 'prepare' and not 1 <= args.limit <= 20:
        parser.error('--limit must be between 1 and 20')
    try:
        if args.command == 'prepare':
            prepare(args)
        elif args.command == 'freeze':
            freeze(args)
        else:
            result = check_snapshot(args.manifest)
            print(json_text(result))
            if not result['ok']:
                return 2
    except (ValueError, KeyError, TypeError, OSError, sqlite3.Error) as exc:
        print(json_text({'ok': False, 'semantic_acceptance': False, 'error': str(exc)}),
              file=sys.stderr)
        return 2
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
