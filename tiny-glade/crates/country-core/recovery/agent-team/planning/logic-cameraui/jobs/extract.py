import json, sys
from pathlib import Path
def main(job, result):
    r = json.load(open(result))
    for step in r.get('results', []):
        for c in step.get('result',{}).get('content',[]):
            text = c.get('text','')
            if not text or text[0] not in '{[': continue
            try: j = json.loads(text)
            except: continue
            if 'functions' in j:
                for addr, f in j['functions'].items():
                    name = f.get('name','?')
                    code = f.get('decompiled_code','')
                    callees = f.get('callees','')
                    Path(f'/tmp/{addr}.txt').write_text(
                        f"=== {addr} {name}\nsig: {f.get('signature')}\ncallees: {callees}\n{code}\n")
            if 'data' in j and 'address' in j:
                Path(f'/tmp/mem_{j["address"]}.txt').write_text(json.dumps(j, indent=1))
main(None, 'b01-camera.result.json')
print("done")
