"""Run with Blender 5.2. Builds source archives, editable collection assets and galleries."""
from pathlib import Path
import bpy, json, math, uuid, collections, hashlib
import numpy as np
from mathutils import Vector, Matrix

ROOT=Path(__file__).resolve().parents[1]
MANIFEST=json.loads((ROOT/'reports/manifest.json').read_text())
RIGS=json.loads((ROOT/'reports/tree-controls.json').read_text())
ROT=Matrix.Rotation(math.pi/2,4,'X')
REPORT=[]
CAT_UUID={c:str(uuid.uuid5(uuid.NAMESPACE_URL,'tinyglade-editable/'+c)) for c in sorted({a['category'] for a in MANIFEST['assets']})}
COLORS={'01_Vegetation':(.16,.30,.08,1),'02_Plants':(.20,.38,.09,1),'03_Architecture':(.62,.49,.34,1),
        '04_Props':(.42,.25,.12,1),'05_Terrain':(.35,.38,.28,1),'06_Animals':(.80,.72,.57,1),
        '07_Other':(.40,.48,.55,1),'08_AnimationFrames':(.81,.76,.61,1),'09_Helpers':(.12,.50,.62,1)}

def transformed(v):
    a=np.asarray(v,dtype=np.float32)
    return a[:,[0,2,1]]*np.array([1,-1,1],dtype=np.float32)

def material(name,color,vc=False,image=None):
    old=bpy.data.materials.get(name)
    if old:return old
    m=bpy.data.materials.new(name);m.diffuse_color=color;m.use_nodes=True
    nodes=m.node_tree.nodes;links=m.node_tree.links;bs=nodes.get('Principled BSDF')
    bs.inputs['Base Color'].default_value=color;bs.inputs['Roughness'].default_value=.78
    if vc:
        n=nodes.new('ShaderNodeVertexColor');n.layer_name='Vertex_Color';n.location=(-300,100)
        links.new(n.outputs['Color'],bs.inputs['Base Color'])
    if image and (ROOT/'textures'/image).exists():
        n=nodes.new('ShaderNodeTexImage');n.image=bpy.data.images.load(str(ROOT/'textures'/image),check_existing=True)
        n.image.pack();n.location=(-300,100);links.new(n.outputs['Color'],bs.inputs['Base Color'])
    return m

def leaf_material():
    m=material('MAT_Foliage_EditablePreview',(.18,.34,.065,1))
    if m.get('built'):return m
    m['built']=True;m['status']='Approximate leaf-card preview; game canopy shader not reproduced'
    n=m.node_tree.nodes;l=m.node_tree.links;bs=n.get('Principled BSDF')
    uv=n.new('ShaderNodeTexCoord');uv.location=(-700,0)
    dist=n.new('ShaderNodeVectorMath');dist.operation='DISTANCE';dist.inputs[1].default_value=(.5,.5,0);dist.location=(-480,0)
    mask=n.new('ShaderNodeMath');mask.operation='LESS_THAN';mask.inputs[1].default_value=.54;mask.location=(-240,-150)
    l.new(uv.outputs['UV'],dist.inputs[0]);l.new(dist.outputs['Value'],mask.inputs[0]);l.new(mask.outputs[0],bs.inputs['Alpha'])
    if hasattr(m,'surface_render_method'):m.surface_render_method='DITHERED'
    return m

def mesh_from(d,name,convert=False,orient=False):
    p=np.asarray(d['Vertex_Position']['buffer'],dtype=np.float32)
    verts=transformed(p) if convert else p
    ix=d['indices']['buffer'];edges=[];faces=[]
    if len(ix)%3==0:
        faces=np.asarray(ix,dtype=np.int32).reshape(-1,3)
        if orient and len(faces) and 'Vertex_Normal' in d:
            normals=transformed(d['Vertex_Normal']['buffer']) if convert else np.asarray(d['Vertex_Normal']['buffer'])
            sample=faces[:min(len(faces),500)]
            cross=np.cross(verts[sample[:,1]]-verts[sample[:,0]],verts[sample[:,2]]-verts[sample[:,0]])
            dot=np.sum(cross*normals[sample].mean(axis=1),axis=1)
            if np.sum(dot<0)>np.sum(dot>0):faces=faces[:,[0,2,1]]
        faces=faces.tolist()
    else:edges=list(zip(ix[:-1],ix[1:]))
    mesh=bpy.data.meshes.new(name);mesh.from_pydata(verts.tolist(),edges,faces);mesh.update()
    for key,val in d.items():
        if not isinstance(val,dict) or key=='indices':continue
        data=np.asarray(val['buffer']);base,width=val['type']
        if width==1:
            typ='INT' if base=='int' else 'FLOAT';prop='value';flat=data.astype(np.int32 if typ=='INT' else np.float32).ravel()
        elif width==2:typ='FLOAT2';prop='vector';flat=data.astype(np.float32).ravel()
        elif width==3:typ='FLOAT_VECTOR';prop='vector';flat=data.astype(np.float32).ravel()
        elif width==4:typ='FLOAT_COLOR';prop='color';flat=data.astype(np.float32).ravel()
        else:raise ValueError(f'Unsupported attribute {key}: {val["type"]}')
        # Keep every original buffer in original coordinate convention; editing positions are Z-up.
        attr=mesh.attributes.new(name='SRC_'+key,type=typ,domain='POINT');attr.data.foreach_set(prop,flat)
    if 'Vertex_Color' in d:
        rgb=np.asarray(d['Vertex_Color']['buffer'],dtype=np.float32)
        rgba=np.c_[rgb,np.ones(len(rgb))] if rgb.shape[1]==3 else rgb
        attr=mesh.color_attributes.new(name='Vertex_Color',type='FLOAT_COLOR',domain='POINT');attr.data.foreach_set('color',rgba.ravel())
    uv=d.get('Vertex_UV',{}).get('buffer')
    if uv is None and any(k in d for k in ['prim_center','appear_pos']) and 'Vertex_Color' in d:
        uv=np.asarray(d['Vertex_Color']['buffer'])[:,:2]
        mesh['derived_uv']='Tree preview UV inferred from Vertex_Color RG; original channels retained'
    if uv is not None and len(mesh.loops):
        layer=mesh.uv_layers.new(name='UVMap' if 'Vertex_UV' in d else 'PreviewUV_FromColorRG')
        vi=np.empty(len(mesh.loops),dtype=np.int32);mesh.loops.foreach_get('vertex_index',vi)
        layer.data.foreach_set('uv',np.asarray(uv,dtype=np.float32)[vi].ravel())
    if 'Vertex_Normal' in d and len(mesh.polygons):
        normals=transformed(d['Vertex_Normal']['buffer']) if convert else d['Vertex_Normal']['buffer']
        mesh.normals_split_custom_set_from_vertices(normals)
        for f in mesh.polygons:f.use_smooth=True
    return mesh

def shape_cage(obj,col,parent):
    verts=np.array([v.co[:] for v in obj.data.vertices]);lo=verts.min(axis=0);hi=verts.max(axis=0)
    span=np.maximum(hi-lo,.05)*1.03
    lat=bpy.data.lattices.new('LAT_'+obj.name);lat.points_u=lat.points_v=lat.points_w=3
    lat.interpolation_type_u=lat.interpolation_type_v=lat.interpolation_type_w='KEY_BSPLINE'
    cage=bpy.data.objects.new('CTRL_Shape_'+obj.name,lat);col.objects.link(cage)
    cage.parent=parent;cage.location=(lo+hi)/2;cage.scale=span;cage.display_type='WIRE';cage.hide_render=True
    cage['usage']='选择此 Lattice，Tab 编辑 27 个控制点；不改变原始网格拓扑'
    mod=obj.modifiers.new('EDIT_ShapeCage','LATTICE');mod.object=cage
    return cage

def curve_group():
    old=bpy.data.node_groups.get('GN_EditableBranchTube')
    if old:return old
    g=bpy.data.node_groups.new('GN_EditableBranchTube','GeometryNodeTree');g.use_fake_user=True
    g.interface.new_socket(name='Geometry',in_out='INPUT',socket_type='NodeSocketGeometry')
    s=g.interface.new_socket(name='Branch Radius',in_out='INPUT',socket_type='NodeSocketFloat');s.default_value=.06;s.min_value=.001;s.max_value=2
    g.interface.new_socket(name='Geometry',in_out='OUTPUT',socket_type='NodeSocketGeometry')
    n=g.nodes;l=g.links;i=n.new('NodeGroupInput');o=n.new('NodeGroupOutput')
    circle=n.new('GeometryNodeCurvePrimitiveCircle');circle.mode='RADIUS';circle.inputs['Resolution'].default_value=10
    tube=n.new('GeometryNodeCurveToMesh');tube.inputs['Fill Caps'].default_value=True
    frame=n.new('NodeFrame');frame.label='新增枝条：曲线控制形状，圆形截面控制粗细'
    circle.parent=frame;tube.parent=frame;i.location=(-550,180);circle.location=(-250,-100);tube.location=(30,100);o.location=(400,180)
    l.new(i.outputs['Geometry'],tube.inputs['Curve']);l.new(i.outputs['Branch Radius'],circle.inputs['Radius'])
    l.new(circle.outputs['Curve'],tube.inputs['Profile Curve']);l.new(tube.outputs['Mesh'],o.inputs['Geometry'])
    g['policy']='Process incoming curve; adds new branch geometry. Existing mesh remains intact.'
    g.asset_mark();g.asset_data.description='编辑曲线控制枝条形状；Branch Radius 控制粗细。'
    return g

def make_rig(asset,col,parent,obj,rigdata):
    arm=bpy.data.armatures.new('ARM_'+asset['id']);rig=bpy.data.objects.new('CTRL_Branches_'+asset['id'].replace('/','_'),arm)
    col.objects.link(rig);rig.parent=parent;rig.show_in_front=True;rig.display_type='WIRE';arm.display_type='STICK'
    bpy.context.view_layer.objects.active=rig;rig.select_set(True);bpy.ops.object.mode_set(mode='EDIT')
    for b in rigdata['bones']:
        eb=arm.edit_bones.new(b['name']);eb.head=b['head'];eb.tail=b['tail']
        if b['parent_bone'] and b['parent_bone'] in arm.edit_bones:eb.parent=arm.edit_bones[b['parent_bone']]
    bpy.ops.object.mode_set(mode='OBJECT');rig.select_set(False)
    rig['reconstruction']='推断的枝杈变形骨架，来自表面路径；不是原作骨骼。权重需按需要精修。'
    rig['usage']='Pose Mode 调整枝条；Edit Mode 调整骨架须重新绑定权重。新增枝条使用曲线工具。'
    skin(obj,rig)
    return rig

def skin(obj,rig):
    points=np.array([v.co[:] for v in obj.data.vertices]);bones=list(rig.data.bones)
    heads=np.array([b.head_local[:] for b in bones]);tails=np.array([b.tail_local[:] for b in bones]);directions=tails-heads
    delta=points[:,None,:]-heads[None,:,:];t=np.clip(np.sum(delta*directions,axis=2)/np.maximum(np.sum(directions**2,axis=1),1e-12),0,1)
    distances=np.linalg.norm(delta-t[:,:,None]*directions,axis=2)
    nearest=np.argsort(distances,axis=1)[:,:3];values=np.take_along_axis(distances,nearest,axis=1)
    weights=1/np.maximum(values,.03)**4;weights/=weights.sum(axis=1,keepdims=True)
    groups=[obj.vertex_groups.new(name=b.name) for b in bones]
    # Quantized weights make assignment fast while retaining a smooth blend.
    for rank in range(nearest.shape[1]):
        buckets=collections.defaultdict(list)
        for vi,(bi,w) in enumerate(zip(nearest[:,rank],weights[:,rank])):
            q=round(float(w)*200)
            if q:buckets[(int(bi),q)].append(vi)
        for (bi,q),ids in buckets.items():groups[bi].add(ids,q/200,'REPLACE')
    mod=obj.modifiers.new('EDIT_BranchRig','ARMATURE');mod.object=rig
    obj['weight_method']='Nearest 3 inferred bone segments, inverse-distance weights; approximate'

def branch_template(col,parent,rig=None):
    c=bpy.data.curves.new('CURVE_AddBranchTemplate','CURVE');c.dimensions='3D';c.resolution_u=12
    sp=c.splines.new('BEZIER');sp.bezier_points.add(2)
    for p,co in zip(sp.bezier_points,[(0,0,0),(.4,0,.6),(.9,0,1.2)]):
        p.co=co;p.handle_left_type=p.handle_right_type='AUTO'
    o=bpy.data.objects.new('TEMPLATE_AddBranch',c);col.objects.link(o);o.parent=parent;o.hide_render=True
    o['template_only']=True;o['usage']='用素材库工具新增枝条，或复制此曲线并启用渲染；Tab 编辑曲线控制点。'
    mod=o.modifiers.new('EDIT_BranchTube','NODES');mod.node_group=curve_group();mod.show_render=False;mod.show_viewport=False
    c.materials.append(material('MAT_Bark_EditablePreview',(.29,.15,.065,1)))
    return o

def assign_materials(obj,d,a):
    name=a['name'];cat=a['category'];color=COLORS[cat]
    hascolor='Vertex_Color' in d;tree=cat=='01_Vegetation' and any(k in d for k in ['prim_center','appear_pos'])
    if tree:
        bark=material('MAT_Bark_EditablePreview',(.29,.15,.065,1));leaf=leaf_material()
        obj.data.materials.append(bark);obj.data.materials.append(leaf)
        if hascolor:
            cs=np.asarray(d['Vertex_Color']['buffer']);
            for f in obj.data.polygons:f.material_index=1 if np.mean(cs[list(f.vertices),2])<.5 and 'olden' not in name else 0
        obj['material_status']='Approximate bark/foliage preview; original shader channels preserved'
    else:
        image={'tree_stump':'tree_stump.png','tree_log':'tree_log.png','wooden_plank':'wooden_plank.png','wooden_plank_lod1':'wooden_plank.png'}.get(name)
        if image:mat=material('MAT_Texture_'+name,color,image=image)
        elif 'glass' in name:
            mat=material('MAT_Glass_Preview',(.38,.58,.65,1));bs=mat.node_tree.nodes.get('Principled BSDF');bs.inputs['Transmission Weight'].default_value=.75;bs.inputs['Roughness'].default_value=.15
        else:mat=material('MAT_VertexColor_Preview' if hascolor else 'MAT_'+cat+'_Preview',color,vc=hascolor)
        obj.data.materials.append(mat)
        obj['material_status']='Vertex color or approximate preview; game material not reproduced'

def build_asset(a,defs,sourcecol):
    d=json.loads((ROOT/a['blob']).read_text());name=a['id'].replace('/','_')
    rawmesh=mesh_from(d,'SOURCE_'+name)
    raw=bpy.data.objects.new('SOURCE_'+name,rawmesh);sourcecol.objects.link(raw);raw.matrix_world=ROT
    raw['sha256']=a['sha256'];raw['source_blob']=a['blob'];raw['source_version']=a['version']
    raw['coordinate_system']='Exact source mesh coordinates; display rotation only'
    col=bpy.data.collections.new('ASSET_'+name);defs.children.link(col)
    root=bpy.data.objects.new('ROOT_'+name,None);col.objects.link(root);root.empty_display_size=.15
    editmesh=mesh_from(d,'EDIT_'+name,convert=True,orient=True)
    obj=bpy.data.objects.new('SM_'+name,editmesh);col.objects.link(obj);obj.parent=root
    obj['asset_id']=a['id'];obj['source_name']=a['name'];obj['source_blob']=a['blob'];obj['source_sha256']=a['sha256']
    obj['source_versions']=','.join(a['available_versions']);obj['coordinate_transform']='x,y,z -> x,-z,y; no inferred scale'
    obj['source_version']=a['version'];obj['double_sided_hint']=str(a['double_sided_hint'])
    root['asset_id']=a['id'];root['editing_level']='网格 + 形状笼'
    assign_materials(obj,d,a)
    cage=None;rig=None
    if a['category'] not in ['09_Helpers','08_AnimationFrames'] and len(editmesh.polygons):cage=shape_cage(obj,col,root)
    if a['id'] in RIGS:
        rig=make_rig(a,col,root,obj,RIGS[a['id']]);branch_template(col,root,rig);root['editing_level']='网格 + 形状笼 + 推断枝杈骨架 + 新增枝条曲线'
    col['asset_id']=a['id'];col['source_name']=a['name'];col['source_versions']=','.join(a['available_versions'])
    col['editable_mesh']=obj.name;col['shape_controller']=cage.name if cage else '';col['branch_controller']=rig.name if rig else ''
    col.asset_mark();col.asset_data.catalog_id=CAT_UUID[a['category']]
    col.asset_data.description=f'{a["name"]} | {root["editing_level"]} | 来源 {col["source_versions"]}。材质是预览适配。'
    for tag in [a['category'],a['version'],'Editable','Variant' if a['is_variant'] else 'Primary']:col.asset_data.tags.new(tag)
    record={'id':a['id'],'collection':col.name,'source_version':a['version'],'category':a['category'],
            'vertices':len(editmesh.vertices),'faces':len(editmesh.polygons),'edges':len(editmesh.edges),
            'source_attributes':[x for x in editmesh.attributes.keys() if x.startswith('SRC_')],
            'lattice':bool(cage),'bones':len(rig.data.bones) if rig else 0,'uv':bool(editmesh.uv_layers),
            'branch_curve_template':bool(rig),'material_status':obj['material_status']}
    REPORT.append(record)
    return col,obj,rig

def assemble_trees(defs,entries):
    assemblies=[]
    for crown,trunk in [('mid_distance_tree_v1','mid_distance_tree_v1_trunk'),('mid_distance_tree_v2','mid_distance_tree_v2_trunk'),
                        ('mid_tree_v3','mid_tree_v3_trunk'),('mid_tree_v4','mid_tree_v4_trunk'),('mid_tree_v5','mid_tree_v5_trunk')]:
        if crown not in entries or trunk not in entries:continue
        _,trunkobj,trunkrig=entries[trunk];_,crownobj,_=entries[crown]
        if not trunkrig:continue
        col=bpy.data.collections.new('TREE_Assembled_'+crown);defs.children.link(col)
        # Copies keep each assembled tree independent and all original meshes intact.
        objects=list(entries[trunk][0].objects);mapping={}
        for src in objects:
            o=src.copy();o.name=src.name+'_assembled';o.data=src.data.copy() if src.data else None;col.objects.link(o);mapping[src]=o
        for src,o in mapping.items():
            if src.parent in mapping:o.parent=mapping[src.parent]
            for mod in o.modifiers:
                if hasattr(mod,'object') and mod.object in mapping:mod.object=mapping[mod.object]
        root=next(o for o in col.objects if o.type=='EMPTY');rig=mapping[trunkrig]
        canopy=crownobj.copy();canopy.data=crownobj.data.copy();canopy.name='SM_Canopy_'+crown;col.objects.link(canopy);canopy.parent=root
        for m in list(canopy.modifiers):canopy.modifiers.remove(m)
        cage=next(o for o in col.objects if o.type=='LATTICE')
        positions=np.array([v.co[:] for o in [mapping[trunkobj],canopy] for v in o.data.vertices])
        cage.location=(positions.min(axis=0)+positions.max(axis=0))/2
        cage.scale=np.maximum(np.ptp(positions,axis=0),.05)*1.03
        m=canopy.modifiers.new('EDIT_SharedTreeShape','LATTICE');m.object=cage
        skin(canopy,rig)
        root['asset_id']='assembled/'+crown;root['editing_level']='树干/树冠组合；共享枝杈控制；叶片权重为推断'
        col['asset_id']='assembled/'+crown;col['editable_mesh']=mapping[trunkobj].name;col['branch_controller']=rig.name
        col['shape_controller']=next((o.name for o in col.objects if o.type=='LATTICE'),'')
        col.asset_mark();col.asset_data.catalog_id=CAT_UUID['01_Vegetation']
        col.asset_data.description='来自 prefab 中的树干/树冠对应关系；Pose Mode 改枝条，叶冠随权重变形。新增枝条用曲线。'
        col.asset_data.tags.new('AssembledTree');col.asset_data.tags.new('Editable')
        assemblies.append((col,mapping[trunkobj],rig))
        REPORT.append({'id':'assembled/'+crown,'collection':col.name,'category':'01_Vegetation','assembly':True,
                       'bones':len(rig.data.bones),'lattice':True,'vertices':len(mapping[trunkobj].data.vertices)+len(canopy.data.vertices),
                       'canopy_follows_rig':True,'material_status':'approximate foliage/bark'})
    return assemblies

def add_gallery(scene,assets):
    gallery=bpy.data.collections.new('COL_GalleryInstances');scene.collection.children.link(gallery)
    maxsize=max((max(obj.dimensions) for _,obj,_ in assets),default=2)
    spacing=min(max(maxsize*1.15,2.5),28);columns=min(10,max(3,math.ceil(math.sqrt(len(assets)))))
    for index,(col,obj,rig) in enumerate(assets):
        inst=bpy.data.objects.new('PREVIEW_'+col.name,None);inst.instance_type='COLLECTION';inst.instance_collection=col
        gallery.objects.link(inst);inst.location=((index%columns)*spacing,(index//columns)*spacing,0)
        inst['asset_id']=col.get('asset_id','');inst['usage']='选中后使用 TG 素材库面板 → 制作可编辑副本'
        # Text labels make the default scene navigable before any previews exist.
        c=bpy.data.curves.new('Label','FONT');c.body=col.name.replace('ASSET_','').replace('TREE_Assembled_','TREE ');c.size=spacing*.032
        label=bpy.data.objects.new('LABEL_'+str(index),c);gallery.objects.link(label);label.location=inst.location+Vector((-spacing*.42,-spacing*.3,.03))
    scene['usage']='Gallery 中是 collection instances；用附带 TG 素材库工具制作独立可编辑副本。'
    scene['grid_spacing']=spacing;scene['columns']=columns;scene['asset_count']=len(assets)
    scene.world=bpy.data.worlds.new('World_Gallery');scene.world.use_nodes=True;scene.world.node_tree.nodes.get('Background').inputs[0].default_value=(.15,.18,.22,1)
    scene.render.engine='BLENDER_EEVEE';scene.unit_settings.system='METRIC';scene.unit_settings.scale_length=1
    return gallery

def main():
    categories=sorted({a['category'] for a in MANIFEST['assets']})
    for category in categories:
        bpy.ops.wm.read_factory_settings(use_empty=True)
        # Build against the definitions scene, so armatures can enter Edit Mode.
        defs_scene=bpy.context.scene;defs_scene.name='AssetDefinitions'
        defs=bpy.data.collections.new('COL_EditableDefinitions');defs_scene.collection.children.link(defs)
        source_scene=bpy.data.scenes.new('SourceArchive');sourcecol=bpy.data.collections.new('COL_ExactSourceMeshes');source_scene.collection.children.link(sourcecol)
        entries={};assets=[]
        for a in [a for a in MANIFEST['assets'] if a['category']==category]:
            entry=build_asset(a,defs,sourcecol);assets.append(entry);entries[a['id']]=entry
        if category=='01_Vegetation':assets=assemble_trees(defs,entries)+assets
        scene=bpy.data.scenes.new('Gallery');add_gallery(scene,assets)
        # Templates belong to definitions and are hidden only in the gallery via hide_set.
        bpy.context.window.scene=scene
        for o in scene.objects:
            if o.get('template_only'):o.hide_set(True)
        doc=bpy.data.texts.new('阅读我_README');doc.write('原始网格：SourceArchive 场景。编辑资产定义：AssetDefinitions 场景。\nGallery 为预览实例。使用 TG 素材库面板的“制作可编辑副本”后，编辑 SM_ 网格、CTRL_Shape_ 笼、CTRL_Branches_ 骨架。\n树木骨架与权重是推断的；材质为预览适配。原始 JSON 与全部版本来源见 reports/manifest.json。\n新增枝条使用曲线；删除现有枝条需核对受影响顶点再手工编辑，不能把推断权重当作准确枝杈分段。')
        scene.render.resolution_x=1600;scene.render.resolution_y=1000
        # Set a useful opening view; no user file or editor state is touched.
        for screen in bpy.data.screens:
            for area in screen.areas:
                if area.type=='VIEW_3D':
                    space=area.spaces.active;space.shading.color_type='MATERIAL';space.overlay.show_extras=False
                    space.region_3d.view_distance=max(8,scene['grid_spacing']*scene['columns']*.65)
                    space.region_3d.view_location=(scene['grid_spacing']*(scene['columns']-1)*.5,scene['grid_spacing']*max(0,math.ceil(len(assets)/scene['columns'])-1)*.5,2)
        out=ROOT/'blends'/(category+'.blend');bpy.ops.wm.save_as_mainfile(filepath=str(out),compress=True)
        print('BUILT',category,len(assets),flush=True)
    (ROOT/'reports/build-report.json').write_text(json.dumps({'blender':bpy.app.version_string,'assets':REPORT,
        'summary':{'asset_entries':len(REPORT),'shape_cages':sum(x.get('lattice',False) for x in REPORT),
                   'branch_rigs':sum(bool(x.get('bones')) for x in REPORT),'assembled_trees':sum(x.get('assembly',False) for x in REPORT)}},ensure_ascii=False,indent=2))
    lines=['# This is an Asset Catalog Definition file for Blender.','VERSION 1','']
    for c,u in CAT_UUID.items():lines.append(f'{u}:{c}:{c}')
    (ROOT/'blends/blender_assets.cats.txt').write_text('\n'.join(lines)+'\n')
    print('LIBRARY_BUILD_COMPLETE',len(REPORT),flush=True)

if __name__=='__main__':main()
