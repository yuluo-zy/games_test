"""Infer conservative deformation skeletons without modifying source topology."""
from pathlib import Path
import json, heapq
import numpy as np
from scipy.sparse import coo_matrix
from scipy.sparse.csgraph import connected_components, dijkstra
from scipy.spatial import cKDTree

ROOT=Path(__file__).resolve().parents[1]
TARGETS=['mid_distance_tree_v1_trunk','mid_distance_tree_v2_trunk','mid_tree_v3_trunk',
         'mid_tree_v4_trunk','mid_tree_v5_trunk','branchy_tree_v1','branchy_tree_v2','central_tree',
         'naked_tree_1','naked_tree_1_no_mushrooms','naked_tree_2','naked_tree_2_no_mushrooms',
         'naked_tree_2_lowpoly','olden_tree','olden_tree_large','fallen_tree','tree_log','tree_stump']

def infer(d):
    raw=np.asarray(d['Vertex_Position']['buffer'],dtype=float)
    pts=raw[:,[0,2,1]]*np.array([1,-1,1])
    face=np.asarray(d['indices']['buffer'],dtype=int).reshape(-1,3)
    color=d.get('Vertex_Color',{}).get('buffer')
    if color is not None:
        c=np.asarray(color)
        # Matching paired trunk/canopy meshes verify the convention for these assets.
        if np.any(c[:,2]==0) and np.any(c[:,2]==1):
            face=face[np.all(c[face,2]>.5,axis=1)]
    unique,inverse=np.unique(np.round(pts,5),axis=0,return_inverse=True)
    f=inverse[face];edges=np.vstack([f[:,[0,1]],f[:,[1,2]],f[:,[2,0]]])
    edges=edges[edges[:,0]!=edges[:,1]]
    edges=np.unique(np.sort(edges,axis=1),axis=0)
    length=np.linalg.norm(unique[edges[:,0]]-unique[edges[:,1]],axis=1)
    g=coo_matrix((np.r_[length,length],(np.r_[edges[:,0],edges[:,1]],np.r_[edges[:,1],edges[:,0]])),shape=(len(unique),len(unique))).tocsr()
    nc,labels=connected_components(g)
    ids=np.flatnonzero(labels==np.bincount(labels).argmax())
    if len(ids)<12: return None
    root=int(ids[np.argmin(np.linalg.norm(unique[ids,:2],axis=1)+np.abs(unique[ids,2])*.6)])
    dist,pred=dijkstra(g,indices=root,return_predecessors=True)
    finite=np.flatnonzero(np.isfinite(dist));scale=max(np.ptp(unique[ids],axis=0).max(),.1)
    used={root};joint_ids={root:'root'};joints={'root':unique[root].tolist()};bones=[]
    tree=cKDTree(unique[ids])
    # Surface geodesics infer approximate branch paths; they are editing aids, not original rigs.
    for iteration in range(14):
        candidates=sorted(finite,key=lambda x:dist[x],reverse=True)
        best=None;best_len=0
        for endpoint in candidates[:min(len(candidates),1200)]:
            path=[];v=int(endpoint);walk=0
            while v not in used and v>=0 and walk<len(unique):
                path.append(v);v=int(pred[v]);walk+=1
                if v == -9999: break
            if not path or v<0: continue
            path.append(v);path.reverse()
            ln=float(np.linalg.norm(np.diff(unique[path],axis=0),axis=1).sum())
            if ln>best_len:best,best_len=path,ln
        if best is None or best_len<scale*.14:break
        attach=best[0]
        # Attach to an already sampled control joint along the existing predecessor chain.
        a=attach
        while a not in joint_ids and a>=0:a=int(pred[a])
        parent=joint_ids.get(a,'root');last=np.asarray(joints[parent]);last_vid=attach
        selected=[];accum=0
        for n in range(1,len(best)):
            accum+=np.linalg.norm(unique[best[n]]-unique[best[n-1]])
            if accum>=scale*.09 or n==len(best)-1:selected.append(best[n]);accum=0
        for vid in selected:
            near=tree.query_ball_point(unique[vid],scale*.025)
            p=unique[ids[near]].mean(axis=0) if near else unique[vid]
            if np.linalg.norm(p-last)<scale*.008:continue
            name=f'branch_{len(bones):03d}'
            bones.append({'name':name,'head':last.tolist(),'tail':p.tolist(),'parent_joint':parent,
                          'parent_bone':next((b['name'] for b in reversed(bones) if b.get('end_joint')==parent),None),
                          'end_joint':name+'_tip'})
            parent=name+'_tip';joints[parent]=p.tolist();joint_ids[vid]=parent;last=p;last_vid=vid
        for vid in best:used.add(vid)
        if len(bones)>90:break
    if not bones:return None
    return {'method':'surface-geodesic branch controls, approximate; no source topology modified',
            'bones':bones,'root':joints['root'],'analysis_components':int(nc),'analyzed_vertices':len(ids)}

def main():
    manifest=json.loads((ROOT/'reports/manifest.json').read_text());out={}
    for a in manifest['assets']:
        if a['name'] not in TARGETS:continue
        d=json.loads((ROOT/a['blob']).read_text());rig=infer(d)
        if rig:out[a['id']]=rig
    (ROOT/'reports/tree-controls.json').write_text(json.dumps(out,indent=2))
    print('Tree deformation skeletons:',len(out),'bones:',sum(len(x['bones']) for x in out.values()),flush=True)

if __name__=='__main__':main()
