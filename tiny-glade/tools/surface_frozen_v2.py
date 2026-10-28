"""Dispatch only two explicitly reviewed surface-v2 producer policies.

Legacy b175 validation and returned identities remain byte-for-byte compatible.
"""
import hashlib,importlib.util
from pathlib import Path
LEGACY_VALIDATOR_SHA='b175ac7a0a85d461b506d552d9f7907a2aab5f64af6d1673d6afdd5a7a16d4e8'
LOCAL_VALIDATOR_SHA='aca3c53a84b04113472c06e3eb7706e2acc40cdeaac4ff6e3d1994f514a6558e'
LOCAL_ADAPTER_SHA='bb04a7a790d8316c2dae5b95342f595c1e5bd8eab8e59e56ab7de6e85e00e591'
LOCAL_POLICY_SHA='906a266903aebf707576c25010372b0944d36733909b4bf87445229cb3ba444d'
def _load(filename,expected):
    path=Path(__file__).with_name(filename)
    if hashlib.sha256(path.read_bytes()).hexdigest()!=expected:raise ValueError('reviewed surface checker changed: '+filename)
    spec=importlib.util.spec_from_file_location('reviewed_'+path.stem,path)
    module=importlib.util.module_from_spec(spec);spec.loader.exec_module(module);return module
_legacy=_load('surface_frozen_v2_legacy.py',LEGACY_VALIDATOR_SHA)
ADAPTER_SHA=_legacy.ADAPTER_SHA
JOB_SHA=_legacy.JOB_SHA
COMMAND=_legacy.COMMAND
TERMINATED=_legacy.TERMINATED
signature=_legacy.signature
def validate(metadata,frozen,exe):
    if 'local_adapter_policy_sha256' in frozen:
        if frozen['local_adapter_policy_sha256']!=LOCAL_POLICY_SHA or frozen.get('adapter_sha256')!=LOCAL_ADAPTER_SHA:
            raise ValueError('unknown/mixed explicit local surface producer policy')
        result=_load('surface_frozen_local_v2.py',LOCAL_VALIDATOR_SHA).validate(metadata,frozen,exe)
        result['dispatcher_sha256']=hashlib.sha256(Path(__file__).read_bytes()).hexdigest()
        return result
    if frozen.get('adapter_sha256')!=ADAPTER_SHA:raise ValueError('new surface producer needs its explicit reviewed local policy')
    return _load('surface_frozen_v2_legacy.py',LEGACY_VALIDATOR_SHA).validate(metadata,frozen,exe)
