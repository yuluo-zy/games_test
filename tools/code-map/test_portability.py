"""Transfer corruption, relocation, isolation and non-overwrite regressions."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch, Mock

import manage

class TransferTests(unittest.TestCase):
    def bundle(self, root):
        root.mkdir()
        content=root/'payload/tools/code-map-runtime/inventory.json'
        content.parent.mkdir(parents=True)
        content.write_text('{}\n')
        manifest={'format':1,'complete':True,'mode':'checkpoint-rebuild','snapshot':'frozen-id',
            'origin_workspace':'/old/game','runtime_relative':'tools/code-map-runtime',
            'bytes':content.stat().st_size,'files':[{'path':str(content.relative_to(root)),
                'sha256':manage.sha(content),'bytes':content.stat().st_size}]}
        (root/'manifest.json').write_text(json.dumps(manifest))
        return content

    def test_corruption_cannot_write_destination(self):
        with tempfile.TemporaryDirectory() as d:
            root=Path(d);content=self.bundle(root/'bundle');content.write_text('tamper')
            with self.assertRaisesRegex(ValueError,'integrity'):
                manage.restore(root/'bundle',root/'target')
            self.assertFalse((root/'target').exists())

    def test_restore_rejects_nonempty_destination(self):
        with tempfile.TemporaryDirectory() as d:
            root=Path(d);self.bundle(root/'bundle');target=root/'target';target.mkdir()
            original=target/'user-data';original.write_text('keep')
            with self.assertRaises(FileExistsError):manage.restore(root/'bundle',target)
            self.assertEqual(original.read_text(),'keep')

    def test_restore_maps_provenance_without_rewriting_bytes(self):
        with tempfile.TemporaryDirectory() as d:
            root=Path(d);content=self.bundle(root/'bundle');target=root/'新电脑 game'
            with patch.object(manage.shutil,'disk_usage',return_value=Mock(free=100*2**30)):
                manage.restore(root/'bundle',target)
            self.assertEqual((target/'tools/code-map-runtime/inventory.json').read_bytes(),content.read_bytes())
            relocation=json.loads((target/'tools/code-map-runtime/relocation.json').read_text())
            self.assertEqual(relocation['origin_workspace'],'/old/game')
            self.assertFalse((target/'tools/code-map-runtime/connection.json').exists())

    def test_manifest_rejects_path_traversal_and_unlisted_files(self):
        with tempfile.TemporaryDirectory() as d:
            root=Path(d);self.bundle(root/'bundle')
            (root/'bundle/extra').write_text('unlisted')
            with self.assertRaisesRegex(ValueError,'unlisted'):manage.bundle_manifest(root/'bundle')
            for name in ('../outside','/absolute','payload/../../outside'):
                with self.assertRaises(ValueError):manage.safe_relative(name)

    def test_manifest_rejects_symlink_and_unpublished_bundle(self):
        with tempfile.TemporaryDirectory() as d:
            root=Path(d);content=self.bundle(root/'bundle')
            (root/'bundle/link').symlink_to(content)
            with self.assertRaisesRegex(ValueError,'symlink'):manage.bundle_manifest(root/'bundle')
            (root/'bundle/link').unlink()
            manifest=root/'bundle/manifest.json'
            doc=json.loads(manifest.read_text());doc['complete']=False;manifest.write_text(json.dumps(doc))
            with self.assertRaisesRegex(ValueError,'Unpublished'):manage.bundle_manifest(root/'bundle')

    def test_offline_guard_does_not_stop_a_running_service(self):
        with patch.object(manage.subprocess,'run',return_value=Mock(returncode=0)) as run:
            with self.assertRaisesRegex(RuntimeError,'requires Neo4j status=3'):manage.require_stopped()
            self.assertEqual(run.call_count,1)
            self.assertEqual(run.call_args.args[0][-1],'status')

    def test_resolver_from_moved_code_and_chinese_workspace(self):
        with tempfile.TemporaryDirectory() as d:
            root=Path(d).resolve()/'另一台电脑 game';root.mkdir()
            source=root/'reverse-engineering/tiny-glade/tools/code-map';source.mkdir(parents=True)
            shutil.copyfile(manage.SOURCE/'settings.py',source/'settings.py')
            runtime=root/'tools/code-map-runtime';runtime.mkdir(parents=True)
            (runtime/'relocation.json').write_text('{"origin_workspace":"/old/game"}')
            code='import settings,json;print(json.dumps([str(settings.WORKSPACE),str(settings.resolve_evidence("/old/game/B版本内容/tiny-glade.rne"))]))'
            env={k:v for k,v in os.environ.items() if not k.startswith('CODEMAP_')}
            value=json.loads(subprocess.check_output([sys.executable,'-c',code],cwd=source,env=env,text=True))
            self.assertEqual(value,[str(root),str(root/'B版本内容/tiny-glade.rne')])

    def test_nested_restore_does_not_apply_destination_twice(self):
        from settings import resolve_evidence
        import settings
        parent=Path('/old/game');target=parent/'tools/migration-check'
        with patch.object(settings,'WORKSPACE',target),patch.object(settings,'RUNTIME',target/'tools/code-map-runtime'),\
             patch.object(settings,'origin_workspace',return_value=parent):
            self.assertEqual(resolve_evidence(target/'tools/code-map-runtime/catalog.sqlite3'),target/'tools/code-map-runtime/catalog.sqlite3')
            self.assertEqual(resolve_evidence(parent/'B版本内容/tiny-glade.rne'),target/'B版本内容/tiny-glade.rne')

if __name__=='__main__':unittest.main()
