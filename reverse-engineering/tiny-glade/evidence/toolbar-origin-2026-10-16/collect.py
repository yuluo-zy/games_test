"""Collect shipped lock/PDB provenance; compare Taffy source digests, no game execution."""
import hashlib
import io
import json
from pathlib import Path
import re
import ssl
import tarfile
import tomllib
import urllib.request

ROOT = Path(__file__).resolve().parents[2]
OUT = Path(__file__).resolve().parent
GAME = ROOT.parents[1] / 'ljxsj_92385/Tiny Glade'

def digest(p):
    with p.open('rb') as f:
        return hashlib.file_digest(f, 'sha256').hexdigest()

lock = tomllib.loads((GAME / 'build-info/Cargo.lock').read_text())
names = {'toolbar','taffy','tg-text','cosmic-text','etagere','egui','iced',
         'iced_core','tg-iced','crash-reporter-gui','country-core','tg-gpu-selector'}
packages = [p for p in lock['package'] if p['name'] in names]
sources = json.loads((ROOT/'evidence/multi-version/B-sources.json').read_text())
data = {
    'scope': 'B original distribution only; no tiny-garden',
    'identity': {str(p.relative_to(GAME)):digest(p) for p in [GAME/'tiny-glade.rne',GAME/'tiny-glade.exe',GAME/'tiny_glade.pdb',GAME/'build-info/Cargo.lock']},
    'packages': packages,
    'private_sources':[f for f in sources['files'] if '/toolbar/' in f['path'] or '/tg-text/' in f['path']],
    'dependency_paths':[p for p in sources['dependency_source_paths'] if any(t in p for t in ['/taffy-','/cosmic-text-','/iced-','/egui-'])],
    'lock_reverse_consumers':{name:[p['name'] for p in lock['package'] if any(d.split()[0]==name for d in p.get('dependencies',[]))] for name in ['taffy','tg-text','egui','iced','tg-iced','cosmic-text']},
}

# Original LLVM -files export retains dependency source hashes omitted by the summary JSON.
text = (OUT/'original-pdb-files.txt').read_text(encoding='utf-8-sig')
matches = re.findall(r'\(SHA-256: ([0-9a-fA-F]{64})\) ([^\r\n]+/taffy-0\.3\.13/[^\r\n]+)',text)
pdb = {}
for sha,path in matches:
    rel=path.split('/taffy-0.3.13/')[1]
    pdb.setdefault(rel,set()).add(sha.lower())
data['taffy_pdb_source_hashes']={p:sorted(s) for p,s in pdb.items()}

archive=OUT/'taffy-0.3.13.crate'
if not archive.exists():
    req=urllib.request.Request('https://static.crates.io/crates/taffy/taffy-0.3.13.crate',headers={'User-Agent':'TinyGlade-local-provenance-audit'})
    context=ssl.SSLContext(ssl.PROTOCOL_TLS_CLIENT)
    context.load_default_certs()
    with urllib.request.urlopen(req,timeout=45,context=context) as r:
        archive.write_bytes(r.read())
expected=next(p['checksum'] for p in packages if p['name']=='taffy')
actual=digest(archive)
assert expected==actual,(expected,actual)
with tarfile.open(archive,mode='r:gz') as tar:
    files={m.name.split('/',1)[1]:tar.extractfile(m).read() for m in tar.getmembers() if m.isfile()}
    comparisons=[{'path':p,'pdb_sha256':sorted(hashes),'upstream_sha256':hashlib.sha256(files[p]).hexdigest(),
                  'matches':hashlib.sha256(files[p]).hexdigest() in hashes} for p,hashes in pdb.items() if p in files]
    data['taffy_upstream_comparison']={'url':'https://static.crates.io/crates/taffy/taffy-0.3.13.crate','archive_sha256':actual,
       'matches_locked_checksum':True,'source_count':len(comparisons),'all_sources_match':bool(comparisons) and all(c['matches'] for c in comparisons),'files':comparisons}
    for p in ['Cargo.toml.orig','README.md','src/node.rs','src/compute/leaf.rs']:
        dest=OUT/'upstream-taffy'/p
        dest.parent.mkdir(parents=True,exist_ok=True)
        dest.write_bytes(files[p])
(OUT/'provenance.json').write_text(json.dumps(data,ensure_ascii=False,indent=2)+'\n',encoding='utf-8')
job=[{'tool':'get_metadata','args':{'program':'tiny-glade.exe'}},
     {'tool':'find_functions','args':{'program':'tiny-glade.exe','pattern':'taffy::node','limit':12}},
     {'tool':'get_functions','args':{'program':'tiny-glade.exe','functions':'0x141e4c650,0x141e4c6c0,0x140683ec0,0x14068e850','fields':'signature,decompiled_code,callees'}}]
(OUT/'query.json').write_text(json.dumps(job,indent=2),encoding='utf-8')
print(json.dumps({'identity':data['identity'],'taffy_comparison':{k:v for k,v in data['taffy_upstream_comparison'].items() if k!='files'},'reverse_consumers':data['lock_reverse_consumers']},indent=2))
