#!/usr/bin/env python3
"""Exercise ownership/scope/snapshot gates using isolated synthetic metadata."""
import hashlib
import json
from pathlib import Path
import sqlite3
import subprocess
import sys
import tempfile
import unittest

FLOW = Path(__file__).with_name('workflow.py')


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


class WorkflowTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix='tg-workflow-fixture-')
        self.root = Path(self.tmp.name)
        (self.root / 'AI_REVERSE_TOOLS.md').write_text('Synthetic guide only')
        self.index = self.root / 'skeleton/task-index'
        (self.index / 'claims').mkdir(parents=True)
        self.work = self.index / 'work/TG-fixture/author'
        self.work.mkdir(parents=True)
        self.source = self.root / 'skeleton/C/game/helper.rs'
        self.source.parent.mkdir(parents=True)
        self.source.write_text('// Synthetic metadata, no game code\n')
        self.task = {'id': 'TG-fixture', 'source_path': 'game/helper.rs',
                     'target_version': 'C', 'claim_mode': 'active',
                     'function_ids': ['f-one', 'f-two'], 'cross_version_pairs': [],
                     'source_inputs': {'C': {'path': str(self.source),
                                            'sha256': digest(self.source),
                                            'binary_sha256': 'a' * 64,
                                            'pdb_sha256': 'b' * 64}},
                     'handoff': 'task-index/handoffs/TG-fixture.json'}
        self.dbpath = self.index / 'index.sqlite3'
        with sqlite3.connect(self.dbpath) as db:
            db.execute('CREATE TABLE tasks(id TEXT, payload TEXT)')
            db.execute('CREATE TABLE functions(id TEXT, path TEXT, version TEXT, '
                       'category TEXT, group_id TEXT, priority INT, payload TEXT)')
            db.execute('INSERT INTO tasks VALUES (?,?)', ('TG-fixture', json.dumps(self.task)))
            for number in ('one', 'two'):
                item = {'id': 'f-' + number, 'logical_group_id': 'g-' + number,
                        'path': 'game/helper.rs', 'version': 'C', 'category': 'tools',
                        'name': 'synthetic::' + number, 'metrics': {}, 'native_instances': []}
                db.execute('INSERT INTO functions VALUES (?,?,?,?,?,?,?)',
                           (item['id'], item['path'], 'C', 'tools',
                            item['logical_group_id'], 1, json.dumps(item)))
        self.claim_path = self.index / 'claims/TG-fixture.json'
        self.claim = {'owner': 'author', 'status': 'claimed', 'work_dir': str(self.work),
                      'worker': 'idx-tg-fixture-author',
                      'source_inputs': self.task['source_inputs'],
                      'function_scope': {'logical_function_group': 'g-one'}}

    def tearDown(self):
        self.tmp.cleanup()

    def call(self, *args, ok=True):
        result = subprocess.run([sys.executable, str(FLOW), *map(str, args)],
                                text=True, capture_output=True)
        self.assertEqual(result.returncode, 0 if ok else 2, result.stderr + result.stdout)
        return result

    def prepare(self, *args, ok=True):
        return self.call('prepare', '--project', self.root, '--task', 'TG-fixture',
                         '--role', 'restore-tools', *args, ok=ok)

    def save_claim(self):
        self.claim_path.write_text(json.dumps(self.claim))

    def snapshot(self):
        file = self.work / 'evidence.txt'
        file.write_text('Synthetic evidence, no native claim')
        manifest = self.work / 'delivery.json'
        self.call('freeze', '--root', self.work, '--file', 'evidence.txt',
                  '--output', manifest)
        return file, manifest

    def test_preview_has_safe_defaults_and_creates_no_claim(self):
        result = self.prepare()
        self.assertIn('forbidden', result.stdout)
        self.assertIn('paused', result.stdout)
        self.assertIn('preview_only', result.stdout)
        self.assertFalse(self.claim_path.exists())
        self.assertEqual(list(self.work.iterdir()), [])

    def test_all_roles_render_in_preview_without_side_effects(self):
        for role in ('coordinator', 'evidence', 'restore-tools', 'restore-business',
                     'restore-core', 'reviewer', 'validator', 'integrator'):
            result = self.call('prepare', '--project', self.root, '--task', 'TG-fixture',
                               '--role', role)
            self.assertIn('forbidden', result.stdout)
            self.assertIn('paused', result.stdout)
            self.assertNotIn('{{COMMON}}', result.stdout)
        self.assertFalse(self.claim_path.exists())
        self.assertEqual(list(self.work.iterdir()), [])

    def test_output_needs_matching_owner_and_preserves_claim(self):
        output = self.work / 'prompt.md'
        self.prepare('--owner', 'author', '--output', output, ok=False)
        self.save_claim()
        before = self.claim_path.read_bytes()
        self.prepare('--owner', 'other', '--output', output, ok=False)
        self.prepare('--owner', 'author', '--output', output)
        self.assertEqual(self.claim_path.read_bytes(), before)
        self.assertNotIn('synthetic::two', output.read_text())
        self.prepare('--owner', 'author', '--output', output, ok=False)

    def test_scope_stale_source_and_claim_are_rejected(self):
        self.save_claim()
        self.prepare('--owner', 'author', '--function', 'f-two', ok=False)
        self.claim['source_inputs'] = {}
        self.save_claim()
        self.prepare('--owner', 'author', ok=False)
        self.source.write_text('Changed synthetic source')
        self.prepare(ok=False)

    def test_output_escape_and_symlink_are_rejected(self):
        self.save_claim()
        self.prepare('--owner', 'author', '--output', self.root / 'escape.md', ok=False)
        (self.work / 'escape').symlink_to(self.root, target_is_directory=True)
        self.prepare('--owner', 'author', '--output', self.work / 'escape/evil.md', ok=False)
        self.assertFalse((self.root / 'evil.md').exists())

    def test_reviewer_requires_independent_actor_and_valid_delivery(self):
        self.save_claim()
        file, manifest = self.snapshot()
        def review(*args, ok=True):
            return self.call('prepare', '--project', self.root, '--task', 'TG-fixture',
                             '--role', 'reviewer', '--owner', 'author',
                             '--output', self.work / 'review-prompt.md', *args, ok=ok)
        review('--actor', 'reviewer', ok=False)
        review('--actor', 'author', '--delivery', manifest, ok=False)
        review('--actor', 'reviewer', '--delivery', manifest)
        self.assertIn('idx-tg-fixture-reviewer', (self.work / 'review-prompt.md').read_text())
        file.write_text('Changed evidence')
        self.call('check', '--manifest', manifest, ok=False)

    def test_snapshot_rejects_escape_and_overwrite(self):
        file, manifest = self.snapshot()
        self.call('check', '--manifest', manifest)
        self.call('freeze', '--root', self.work, '--file', 'evidence.txt',
                  '--output', manifest, ok=False)
        self.call('freeze', '--root', self.work, '--file', '../../../../../AI_REVERSE_TOOLS.md',
                  '--output', self.work / 'escape.json', ok=False)
        data = json.loads(manifest.read_text())
        data['files'] = {'../evil': digest(file)}
        manifest.write_text(json.dumps(data))
        self.call('check', '--manifest', manifest, ok=False)

    def test_deferred_unknown_and_empty_target_are_not_saved(self):
        self.save_claim()
        with sqlite3.connect(self.dbpath) as db:
            self.task['claim_mode'] = 'deferred'
            db.execute('UPDATE tasks SET payload=?', (json.dumps(self.task),))
        self.prepare('--owner', 'author', '--output', self.work / 'prompt.md', ok=False)
        self.prepare('--function', 'unknown', ok=False)
        with sqlite3.connect(self.dbpath) as db:
            db.execute('DELETE FROM functions')
        self.prepare(ok=False)


if __name__ == '__main__':
    unittest.main()
