"""Read-only identity, or an exclusive exact-clean-HEAD local-source guard."""
import argparse,json,re,subprocess
from pathlib import Path
import freeze_surface_local_target as local
QUALIFIED={
    'perf_build_target.py':'7fdd373b73872490d52a4e1ce8a41832dbfc2d997c45688b6c1c7dd8dfb02b88',
    'perf_clock_policy.py':'76a75670bfde9b8326f210d11884d5a1fbd7554c3a67e5991cadcae93a2485dc',
    'perf_headless.py':'770a08d7c31a442ba087a78525dc6951ff42a09d1ad67e63c0adf329db100ed8',
    'perf_headless_account.py':'f20af03985b526cc6fcd2306157ee8591cc7b066bee0c4b1ef05816fa90a0395',
    'perf_etw.py':'abdfd9165cd49d3814b48cca3ca882315ef1467dad683b1b72f20e3c79ccb97d',
    'perf_trace_archive.py':'90062b4a1982e41344adfc8cdf3a7b008bfd36410c3d51eea0de30bbe8c27702',
    'perf_windows.py':'19ca12a556214b8211312157603585dff1f6f5e758830be23f1dedaba4d6e3db'}
def main():
    p=argparse.ArgumentParser();p.add_argument('--root',type=Path,default=Path('C:/game'))
    p.add_argument('--expected-commit',required=True);p.add_argument('--expected-source-sha256')
    p.add_argument('--label');p.add_argument('--local-policy',type=Path);p.add_argument('--local-policy-sha256')
    p.add_argument('--out',type=Path);a=p.parse_args();api=local.guard_api();root=a.root.resolve(strict=True)
    before=api.snapshot(root)
    if not re.fullmatch('[0-9a-f]{40}',a.expected_commit) or before['commit']!=a.expected_commit or before['status']:raise ValueError('requires explicit exact clean HEAD')
    if a.expected_source_sha256 is not None and a.expected_source_sha256!=before['content_sha256']:raise ValueError('source content SHA differs')
    for name,expected in QUALIFIED.items():
        if api.digest(root/'tools'/name)!=expected:raise ValueError('qualified tool changed: '+name)
    if a.out is None:
        print(json.dumps({'commit':before['commit'],'source_content_sha256':before['content_sha256'],'files':len(before['files'])},indent=2));return
    if not a.expected_source_sha256 or not a.label or not re.fullmatch('[A-Za-z0-9_-]+',a.label) or not a.local_policy or not a.local_policy_sha256:raise ValueError('guard requires exact content/label/reviewed local policy')
    policy=local.read_policy(a.local_policy,a.local_policy_sha256,api)
    fixed={**policy['fixed_environment'],'CARGO_TARGET_DIR':str(root/'target')}
    result={'schema':'surface-local-exact-source-guard-v1','root':str(root),'cache':str((root/'target').resolve(strict=True)),
        'label':a.label,'expected_source_identity':before,'qualified_tools_sha256':QUALIFIED,
        'rustc_verbose':subprocess.check_output(['rustc','-Vv'],cwd=root,text=True),'fixed_environment':fixed,
        'reject_layout_environment':['CARGO_BUILD_TARGET','CARGO_BUILD_TARGET_DIR','CARGO_BUILD_BUILD_DIR'],
        'expected_cargo_command':local.COMMAND,'expected_artifact_features':policy['artifact_features'],
        'fixture_entry':local.ENTRY,'local_adapter_policy_sha256':a.local_policy_sha256,
        'scope':policy['acceptance_scope']}
    if api.snapshot(root)!=before:raise ValueError('source changed during guard preparation')
    with a.out.open('x',encoding='utf-8',newline='\n') as stream:json.dump(result,stream,indent=2);stream.write('\n')
    print(json.dumps({'guard':str(a.out.resolve()),'guard_sha256':api.digest(a.out),'source_content_sha256':before['content_sha256']},indent=2))
if __name__=='__main__':main()
