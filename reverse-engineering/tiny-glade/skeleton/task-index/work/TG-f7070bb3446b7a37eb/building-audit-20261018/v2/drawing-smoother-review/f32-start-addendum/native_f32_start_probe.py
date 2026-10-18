"""Original loop with absent start is still f32 interpolation, no None shortcut."""
from pathlib import Path
HERE=Path(__file__).resolve().parent
source=(HERE.parent/'native_smoother_probe.py').read_text()
# Reuse the frozen native entry invocation and independent reference only;
# exclude both original extraction writes and original fixture generation.
exec(compile(source.split('from capstone')[0],'frozen-harness-prefix','exec'),globals())
exec(compile('def f('+source.split('def f(',1)[1].split('rng=random.Random',1)[0],'frozen-smoother-functions','exec'),globals())
OUT=HERE
STATE,POINT,TIME,CAMERA,WINDOW=INPUT,INPUT+0x1000,INPUT+0x2000,INPUT+0x3000,INPUT+0x4000
cases=[]
for point,n in [([-816794.8125,0,-1],3),([235632.703125,0,-1],6),([683839.8125,0,-1],7),([-261540.328125,0,-1],16),([.1,0,-1],3),([100.1251,0,-1],7),([1000.0751,0,-1],6),([1e6,0,-1],7)]:
 point=list(map(f,point));actual=native(None,point,n,1,1080);expected,last=reference(None,point,n,1,1080)
 assert actual['points']==list(map(fb,expected)) and actual['last']==fb(last)
 cases.append({'previous':None,'point':fb(point),'ticks_60':n,'height':1080,'projection_scale':bits(1.),'output':actual})
(HERE/'native-f32-start-fixtures.json').write_text(json.dumps({'binary_sha256':hashlib.sha256(raw).hexdigest(),'method':__doc__,'cases':cases},indent=2)+'\n');print([(c['ticks_60'],len(c['output']['points']))for c in cases])
