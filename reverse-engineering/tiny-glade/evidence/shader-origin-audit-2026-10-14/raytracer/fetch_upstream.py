import hashlib, json, urllib.request
from pathlib import Path

OUT = Path(__file__).resolve().parent
BASE = 'https://raw.githubusercontent.com/jan-van-bergen/GPU-Raytracer/3525cd4f96fba01c8976c7f1a8465dac7bd3d60f/'
FILES = ['README.md', 'LICENSE.txt', 'Src/CUDA/Raytracing/BVH8.h', 'Src/CUDA/Raytracing/BVH.h', 'Src/CUDA/Raytracing/Triangle.h', 'Src/CUDA/Raytracing/Mesh.h', 'Src/CUDA/Raytracing/Ray.h', 'Src/BVH/Converters/BVH8Converter.cpp', 'Src/CUDA/SVGF/SVGF.h', 'Src/CUDA/SVGF/TAA.h']
records=[]
for rel in FILES:
    url=BASE+rel
    request=urllib.request.Request(url, headers={'User-Agent':'TinyGlade-shader-origin-audit'})
    data=urllib.request.urlopen(request, timeout=30).read()
    target=OUT/'upstream-v1.3.0'/rel
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_bytes(data)
    records.append({'path':rel, 'url':url, 'sha256':hashlib.sha256(data).hexdigest(), 'size':len(data)})
license_file=Path('D:/game/ljxsj_92385/Tiny Glade/third-party-licenses.html')
raw=license_file.read_bytes()
lines=raw.decode('utf-8').splitlines()
evidence={'distribution_license':{'path':str(license_file),'sha256':hashlib.sha256(raw).hexdigest(),'line_start':20841,'line_end':20871,'excerpt':'\n'.join(lines[20840:20871])},'upstream_repository':'https://github.com/jan-van-bergen/GPU-Raytracer','tag':'v1.3.0','tag_commit':'3525cd4f96fba01c8976c7f1a8465dac7bd3d60f','fetched_files':records}
(OUT/'upstream_manifest.json').write_text(json.dumps(evidence,indent=2),encoding='utf-8')
print(json.dumps({'fetched_files':len(records), 'manifest':str(OUT/'upstream_manifest.json')}))
