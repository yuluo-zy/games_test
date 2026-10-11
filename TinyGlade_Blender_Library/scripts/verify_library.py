from pathlib import Path
import bpy, json, importlib.util, math
import numpy as np

ROOT=Path(__file__).resolve().parents[1]
MANIFEST=json.loads((ROOT/'reports/manifest.json').read_text())
spec=importlib.util.spec_from_file_location('tinyglade_asset_tools_test',ROOT/'addon/tinyglade_asset_tools.py')
addon=importlib.util.module_from_spec(spec);spec.loader.exec_module(addon);addon.register()
REPORT={'blender':bpy.app.version_string,'files':[],'behavior':{},'failures':[]}

def coords(obj,evaluated=False):
    target=obj.evaluated_get(bpy.context.evaluated_depsgraph_get()) if evaluated else obj
    temporary=obj.type=='CURVE'
    mesh=target.to_mesh() if temporary else target.data
    a=np.empty(len(mesh.vertices)*3,dtype=np.float32);mesh.vertices.foreach_get('co',a)
    if temporary:target.to_mesh_clear()
    return a.reshape(-1,3)

def expect(value,message):
    if not value:REPORT['failures'].append(message)

def verify_file(path):
    bpy.ops.wm.open_mainfile(filepath=str(path),load_ui=False)
    cat=path.stem;assets=[a for a in MANIFEST['assets'] if a['category']==cat]
    sources={o.get('source_blob'):o for o in bpy.data.scenes['SourceArchive'].objects}
    defs={c.get('asset_id'):c for c in bpy.data.collections['COL_EditableDefinitions'].children}
    bpy.context.window.scene=bpy.data.scenes['AssetDefinitions'];bpy.context.view_layer.update()
    maxerr=0;maxrest=0;attr_checks=0
    for a in assets:
        d=json.loads((ROOT/a['blob']).read_text());p=np.asarray(d['Vertex_Position']['buffer'],dtype=np.float32)
        raw=sources[a['blob']];expect(len(raw.data.vertices)==len(p),a['id']+' source count')
        error=float(np.abs(coords(raw)-p).max()) if len(p) else 0;maxerr=max(maxerr,error)
        expect(error==0,a['id']+' original coordinates changed')
        for key,source in d.items():
            if not isinstance(source,dict) or key=='indices':continue
            attr=raw.data.attributes.get('SRC_'+key);expect(attr is not None,a['id']+' missing '+key)
            if attr:
                width=source['type'][1];prop='value' if width==1 else 'color' if width==4 else 'vector'
                dtype=np.int32 if attr.data_type=='INT' else np.float32
                buffer=np.empty(len(p)*width,dtype=dtype);attr.data.foreach_get(prop,buffer)
                wanted=np.asarray(source['buffer'],dtype=dtype).ravel()
                expect(np.array_equal(buffer,wanted),a['id']+' original attribute changed '+key);attr_checks+=1
        col=defs[a['id']];obj=bpy.data.objects[col['editable_mesh']]
        expected=p[:,[0,2,1]]*np.array([1,-1,1],dtype=np.float32)
        error=float(np.abs(coords(obj,True)-expected).max()) if len(p) else 0;maxrest=max(maxrest,error)
        expect(error<2e-4,a['id']+f' rest geometry changed {error}')
        expect(np.isfinite(coords(obj,True)).all(),a['id']+' nonfinite evaluated geometry')
        expect(col.asset_data is not None,a['id']+' asset mark missing')
    REPORT['files'].append({'file':path.name,'assets_checked':len(assets),'source_attribute_buffers_checked':attr_checks,
                            'max_original_coordinate_error':maxerr,'max_evaluated_rest_error':maxrest,'opened':True})
    print('VERIFIED',path.name,len(assets),maxerr,maxrest,flush=True)

def behavior():
    bpy.ops.wm.open_mainfile(filepath=str(ROOT/'blends/01_Vegetation.blend'),load_ui=False)
    bpy.context.window.scene=bpy.data.scenes['Gallery']
    instance=next(o for o in bpy.context.scene.objects if o.instance_collection and o.instance_collection.get('asset_id')=='assembled/mid_tree_v4')
    addon.select_only(bpy.context,instance);result=bpy.ops.tg.make_editable();expect(result=={'FINISHED'},'editable-copy operation failed')
    obj=bpy.context.object;root=addon.root_of(obj);items=addon.objects_for(root,bpy.context.scene)
    originals=[o for o in items if o.type=='MESH'];rig=next(o for o in items if o.type=='ARMATURE');cage=next(o for o in items if o.type=='LATTICE')
    bpy.context.view_layer.update();before={o.name:coords(o,True).copy() for o in originals}
    point=max(cage.data.points,key=lambda p:p.co_deform.z);point.co_deform.x+=.25;bpy.context.view_layer.update()
    cage_delta=max(float(np.abs(coords(o,True)-before[o.name]).max()) for o in originals)
    expect(cage_delta>.005,'lattice edit did not deform geometry');point.co_deform.x-=.25;bpy.context.view_layer.update()
    # Move a high branch that influences canopy vertices, then measure both parts.
    top=max(rig.pose.bones,key=lambda b:b.tail.z);top.rotation_mode='XYZ';top.rotation_euler.y=.18
    bpy.context.view_layer.update();delta={o.name:float(np.abs(coords(o,True)-before[o.name]).max()) for o in originals}
    expect(any(v>.001 for k,v in delta.items() if 'Canopy' in k),'canopy failed to follow branch edit')
    expect(any(v>.001 for k,v in delta.items() if 'Canopy' not in k),'trunk failed to follow branch edit')
    addon.select_only(bpy.context,rig);bpy.ops.object.mode_set(mode='POSE');rig.data.bones.active=rig.data.bones[top.name]
    top.select=True;result=bpy.ops.tg.add_branch();expect(result=={'FINISHED'},'add branch failed')
    curve=bpy.context.object;bpy.ops.object.mode_set(mode='OBJECT');bpy.context.view_layer.update()
    mod=next(m for m in curve.modifiers if m.type=='NODES');socket=next(s for s in mod.node_group.interface.items_tree if getattr(s,'in_out',None)=='INPUT' and s.name=='Branch Radius')
    input_value=getattr(mod.properties.inputs,socket.identifier)
    input_value.value=.04;curve.update_tag();bpy.context.view_layer.update();thin=coords(curve,True).copy()
    input_value.value=.12;curve.update_tag();bpy.context.view_layer.update();thick=coords(curve,True).copy()
    expect(len(thin)>0,'curve produced no branch mesh');radius_delta=float(np.abs(thick-thin).max());expect(radius_delta>.03,'radius control had no effect')
    curve.data.splines[0].bezier_points[-1].co.x+=.6;bpy.context.view_layer.update();shape_delta=float(np.abs(coords(curve,True)-thick).max())
    expect(shape_delta>.1,'curve edit did not affect branch shape')
    REPORT['behavior']={'editable_copy':True,'lattice_max_delta':cage_delta,'tree_part_pose_deltas':delta,
                        'new_branch_vertices':len(thin),'radius_change_delta':radius_delta,'curve_edit_delta':shape_delta,
                        'new_branch_bone_parent':curve.parent_bone,'notes':'Measured on a temporary copy; library assets remain unchanged.'}
    print('BEHAVIOR',REPORT['behavior'],flush=True)

def main():
    for p in sorted((ROOT/'blends').glob('0[1-9]_*.blend')):verify_file(p)
    behavior()
    REPORT['passed']=not REPORT['failures']
    (ROOT/'reports/verification.json').write_text(json.dumps(REPORT,ensure_ascii=False,indent=2))
    print('VERIFICATION_PASSED',REPORT['passed'],'FAILURES',REPORT['failures'],flush=True)
    if REPORT['failures']:raise RuntimeError('Library verification failed')

if __name__=='__main__':main()
