"""Software geometry thumbnails; materials are indicative, never claimed as game renders."""
from pathlib import Path
import json, hashlib
import numpy as np
from PIL import Image, ImageDraw

ROOT=Path(__file__).resolve().parents[1]
manifest=json.loads((ROOT/'reports/manifest.json').read_text());byid={a['id']:a for a in manifest['assets']}
report=json.loads((ROOT/'reports/build-report.json').read_text());index={}

def data_for(row):
    name=row['id']
    if name in byid:return [json.loads((ROOT/byid[name]['blob']).read_text())]
    if name.startswith('assembled/'):
        base=name.split('/',1)[1];return [json.loads((ROOT/byid[n]['blob']).read_text()) for n in [base,base+'_trunk']]
    if name.startswith('kit/'):
        base=name[4:];return [json.loads((ROOT/byid[n]['blob']).read_text()) for n in [base,base+'_glass',base+'_hat'] if n in byid]
    return []

for row in report['assets']:
    im=Image.new('RGB',(160,160),(31,38,47));draw=ImageDraw.Draw(im)
    allp=[];allf=[];offset=0;colors=[]
    for d in data_for(row):
        p=np.asarray(d['Vertex_Position']['buffer']);p=p[:,[0,2,1]]*np.array([1,-1,1]);allp.append(p)
        ix=d['indices']['buffer']
        if len(ix)%3==0:
            fs=np.asarray(ix,dtype=int).reshape(-1,3);allf.extend((fs+offset).tolist())
            cs=np.asarray(d.get('Vertex_Color',{}).get('buffer',[]))
            for face in fs:
                if row['category']=='01_Vegetation' and 'prim_center' in d:
                    colors.append((65,118,47) if len(cs) and cs[face,2].mean()<.5 else (131,88,49))
                else:colors.append((177,160,128))
        offset+=len(p)
    if allp:
        p=np.vstack(allp)
        view=np.array([[.707,-.707,0],[.35,.35,-.866],[.612,.612,.5]])
        q=p@view.T;span=np.maximum(np.ptp(q[:,:2],axis=0),.001);scale=126/span.max()
        xy=(q[:,:2]-(q[:,:2].min(axis=0)+q[:,:2].max(axis=0))/2)*scale+np.array([80,76])
        if allf:
            f=np.asarray(allf);order=np.argsort(q[f,2].mean(axis=1))
            # Limit huge terrain previews while keeping their silhouette samples.
            if len(order)>12000:order=order[np.linspace(0,len(order)-1,12000,dtype=int)]
            for i in order:
                face=f[i];n=np.cross(p[face[1]]-p[face[0]],p[face[2]]-p[face[0]])
                shade=.68+.32*abs(n.dot(np.array([.3,-.5,.8])))/max(np.linalg.norm(n),1e-8)
                c=tuple(int(x*shade) for x in colors[i]);draw.polygon([tuple(x) for x in xy[face]],fill=c)
        else:
            for x,y in xy[::max(1,len(xy)//1000)]:draw.ellipse((x-1,y-1,x+1,y+1),fill=(100,195,215))
    draw.rectangle((0,140,160,160),fill=(20,25,31));draw.text((5,145),row['id'].split('/')[-1][:23],fill=(220,225,230))
    file=ROOT/'previews'/(hashlib.sha1(row['id'].encode()).hexdigest()+'.png');im.save(file)
    index[row['collection']]=str(file.relative_to(ROOT))
(ROOT/'reports/thumbnail-index.json').write_text(json.dumps(index,indent=2))
print('THUMBNAILS',len(index),flush=True)
