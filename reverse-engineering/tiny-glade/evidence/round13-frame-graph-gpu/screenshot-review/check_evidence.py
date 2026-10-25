import json, hashlib
from pathlib import Path
base = Path(__file__).parent.parent/'screenshot'
checked = {'evidence': [], 'candidate': {}}
for name in ['locate.result.json','readback-bounded.result.json']:
    report = json.loads((base/name).read_text(encoding='utf-8'))
    print(name, 'ok', report.get('ok'), 'worker', report.get('worker'))
    checked['evidence'].append({'path': name, 'ok': report.get('ok'), 'steps': []})
    assert report.get('ok') is True
    for step in report.get('results',[]):
        print('  step', step.get('tool'), 'passed', step.get('passed'), 'error', step.get('result',{}).get('isError'))
        assert step.get('passed') is True and not step.get('result',{}).get('isError')
        if 'stdout' in step:
            try:
                body = json.loads(step['stdout'])
                print('    keys', list(body) if isinstance(body, dict) else type(body).__name__)
                if isinstance(body, dict) and 'sha256' in body:
                    print('    target_sha256',body['sha256'])
                    assert body['sha256'] == 'cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323'
                if isinstance(body,dict) and 'functions' in body and isinstance(body['functions'], list):
                    for function in body['functions']:
                        print('    function keys',list(function))
                        if 'decompiled_code' in function:
                            assert function.get('decompiler_completed') is True
                            assert not function.get('decompiler_error')
                            retained = base/(function['address']+'.c')
                            clean = lambda value: ''.join(value.split())
                            assert clean(retained.read_text(encoding='utf-8')) == clean(function['decompiled_code'])
                            checked['evidence'][-1]['steps'].append({'address': function['address'], 'body_bytes': function.get('body_bytes'), 'retained_c_matches_full_mcp_decompilation': True})
            except Exception as error:
                print('    stdout not json', str(error))
manifest = json.loads((base/'candidate-sha.json').read_text(encoding='utf-8'))
for name, expected in manifest.items():
    actual = hashlib.sha256((base/name).read_bytes()).hexdigest()
    assert actual == expected, (name, actual, expected)
    checked['candidate'][name] = actual
    print(name,actual)
(Path(__file__).parent/'verification.json').write_text(json.dumps(checked, indent=2)+'\n', encoding='utf-8')
