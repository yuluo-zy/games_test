"""Git delivery byte checks in an isolated synthetic repo, no builds or GPU.

The shipped .gitattributes must override a Windows autocrlf=true checkout.
Every shipped source/policy is compared after git apply and after checkout.
"""
import difflib
import hashlib
import os
from pathlib import Path
import subprocess
import tempfile
import unittest
import freeze_surface_local_target as producer
import surface_frozen_local_v2 as local
import surface_frozen_v2 as dispatcher

HERE=Path(__file__).resolve().parent
DELIVERY=(
    'freeze_headless_rich_debug.py','freeze_surface_local_target.py','prepare_surface_local_guard.py',
    'surface_frozen_v2_legacy.py','surface_frozen_local_v2.py','surface_frozen_v2.py',
    'run_surface_pixels.py','run_surface_local_pair.py',
    'fixtures/surface-local-adapter-policy.json','fixtures/surface-local-build-adapter.py',
    'test_surface_local_freeze.py','test_surface_frozen_v2.py','test_surface_delivery.py')

def sha(path):return hashlib.sha256(path.read_bytes()).hexdigest()

class SurfaceDeliveryTests(unittest.TestCase):
    def test_all_delivery_bytes_survive_git_apply_and_checkout(self):
        base=HERE.parent/'.perf/delivery-tests';base.mkdir(parents=True,exist_ok=True)
        with tempfile.TemporaryDirectory(prefix='surface-lf-',dir=base) as temp:
            root=Path(temp).resolve();self.assertEqual(root.parent,base.resolve())
            env=producer.guard_api().isolated_git_env()
            def git(*args):
                return subprocess.run(['git','-c','safe.directory='+root.as_posix(),'-C',str(root),*args],
                    env=env,check=True,capture_output=True,text=True)
            git('init','-q');git('config','core.autocrlf','true');git('config','core.eol','crlf')
            git('config','user.name','Synthetic delivery test');git('config','user.email','delivery@example.invalid')
            git('commit','-q','--allow-empty','-m','empty synthetic base')
            patch='';expected={}
            sources={'.gitattributes':HERE.parent/'.gitattributes',**{'tools/'+name:HERE/name for name in DELIVERY}}
            for path,source in sources.items():
                raw=source.read_bytes()
                self.assertNotIn(b'\r',raw,path+' must use canonical LF')
                expected[path]=sha(source)
                patch+='diff --git a/'+path+' b/'+path+'\nnew file mode 100644\n'
                patch+=''.join(difflib.unified_diff([],raw.decode('utf-8').splitlines(keepends=True),
                    fromfile='/dev/null',tofile='b/'+path))
            patchfile=root/'delivery.patch';patchfile.write_bytes(patch.encode('utf-8'))
            # The actual installation has autocrlf=false. Bootstrap the new
            # attributes with those same apply bytes; committed attributes must
            # then survive the ordinary autocrlf=true checkout below.
            git('-c','core.autocrlf=false','apply','--check',str(patchfile))
            git('-c','core.autocrlf=false','apply',str(patchfile))
            self.assertEqual({name:sha(root/name) for name in expected},expected)
            git('add','.gitattributes','tools');git('commit','-q','-m','exact delivered sources')
            # Explicitly recreate every tracked file through Git checkout conversion.
            for name in expected:
                target=(root/name).resolve();self.assertTrue(target.is_relative_to(root));target.unlink()
            git('checkout-index','--all','--force')
            self.assertEqual({name:sha(root/name) for name in expected},expected)
            self.assertEqual(git('status','--porcelain','--untracked-files=no').stdout,'')
            for name in ('crates/rhapsody/src/rg/mod.rs','tools/example.py','Cargo.toml','Cargo.lock',
                         'tools/fixtures/example.json','tools/fixtures/deep/example.json'):
                self.assertEqual(git('check-attr','eol','--',name).stdout.strip(),name+': eol: lf')
            for name in ('history.md','example.exe','example.png'):
                self.assertEqual(git('check-attr','eol','--',name).stdout.strip(),name+': eol: unspecified')
    def test_canonical_policy_and_sealed_dependency_chain(self):
        policy=HERE/'fixtures/surface-local-adapter-policy.json'
        self.assertNotIn(b'\r',policy.read_bytes())
        self.assertEqual(sha(policy),local.POLICY_SHA)
        self.assertEqual(local.POLICY_SHA,dispatcher.LOCAL_POLICY_SHA)
        self.assertEqual(sha(HERE/'surface_frozen_local_v2.py'),dispatcher.LOCAL_VALIDATOR_SHA)
        self.assertEqual(sha(HERE/'freeze_surface_local_target.py'),dispatcher.LOCAL_ADAPTER_SHA)
        self.assertEqual(sha(HERE/'fixtures/surface-local-build-adapter.py'),dispatcher.LOCAL_ADAPTER_SHA)
    def test_crlf_policy_variant_is_rejected_rather_than_copied_as_fix(self):
        base=HERE.parent/'.perf/delivery-tests';base.mkdir(parents=True,exist_ok=True)
        with tempfile.TemporaryDirectory(prefix='surface-crlf-',dir=base) as temp:
            root=Path(temp).resolve();self.assertEqual(root.parent,base.resolve())
            policy=root/'policy.json'
            policy.write_bytes((HERE/'fixtures/surface-local-adapter-policy.json').read_bytes().replace(b'\n',b'\r\n'))
            with self.assertRaisesRegex(ValueError,'policy SHA'):
                producer.read_policy(policy,local.POLICY_SHA,producer.guard_api())

if __name__=='__main__':unittest.main()
