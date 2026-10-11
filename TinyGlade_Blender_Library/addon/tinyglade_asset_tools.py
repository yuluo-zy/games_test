bl_info = {'name':'Tiny Glade Editable Asset Library','author':'Local asset workflow',
           'version':(1,0,0),'blender':(5,2,0),'location':'View3D > Sidebar > TG 素材库',
           'category':'Object','description':'Editable copies, lattice controls and reconstructed branch tools'}
import bpy
from mathutils import Vector, Matrix

def root_of(obj):
    if not obj:return None
    while obj.parent:obj=obj.parent
    return obj if obj.get('asset_id') else None

def objects_for(root,scene):
    return [o for o in scene.objects if root_of(o)==root]

def select_only(context,obj):
    if context.object and context.object.mode!='OBJECT':bpy.ops.object.mode_set(mode='OBJECT')
    bpy.ops.object.select_all(action='DESELECT');obj.hide_set(False);obj.select_set(True);context.view_layer.objects.active=obj

class TG_OT_make_editable(bpy.types.Operator):
    bl_idname='tg.make_editable';bl_label='制作可编辑副本';bl_options={'REGISTER','UNDO'}
    @classmethod
    def poll(cls,ctx):return ctx.object is not None and ctx.object.instance_type=='COLLECTION' and ctx.object.instance_collection is not None
    def execute(self,ctx):
        inst=ctx.object;source=inst.instance_collection
        col=bpy.data.collections.new('EDIT_'+source.name);ctx.scene.collection.children.link(col)
        mapping={}
        for src in source.all_objects:
            o=src.copy();o.name='EDIT_'+src.name;o.data=src.data.copy() if src.data else None;col.objects.link(o);mapping[src]=o
        for src,o in mapping.items():
            if src.parent in mapping:o.parent=mapping[src.parent]
            else:o.matrix_world=inst.matrix_world @ src.matrix_world
            for mod in o.modifiers:
                if hasattr(mod,'object') and mod.object in mapping:mod.object=mapping[mod.object]
            if o.get('template_only'):o.hide_set(True)
        roots=[o for o in col.objects if o.type=='EMPTY' and o.parent is None]
        if roots:roots[0]['library_source']=source.name
        active=next((o for o in col.objects if o.type=='MESH'),next(iter(col.objects),None))
        inst.hide_set(True)
        if active:select_only(ctx,active)
        self.report({'INFO'},'独立副本已创建，原始资产保留。');return {'FINISHED'}

class TG_OT_select_cage(bpy.types.Operator):
    bl_idname='tg.select_cage';bl_label='编辑整体形状笼';bl_options={'REGISTER','UNDO'}
    def execute(self,ctx):
        root=root_of(ctx.object);items=objects_for(root,ctx.scene) if root else []
        cage=next((o for o in items if o.type=='LATTICE'),None)
        if not cage:self.report({'WARNING'},'先制作可编辑副本；该资产可能没有形状笼。');return {'CANCELLED'}
        select_only(ctx,cage);bpy.ops.object.mode_set(mode='EDIT');return {'FINISHED'}

class TG_OT_pose_branches(bpy.types.Operator):
    bl_idname='tg.pose_branches';bl_label='调整现有枝条';bl_options={'REGISTER','UNDO'}
    def execute(self,ctx):
        root=root_of(ctx.object);items=objects_for(root,ctx.scene) if root else []
        rig=next((o for o in items if o.type=='ARMATURE'),None)
        if not rig:self.report({'WARNING'},'该资产没有枝条骨架；可使用整体形状笼。');return {'CANCELLED'}
        select_only(ctx,rig);bpy.ops.object.mode_set(mode='POSE');return {'FINISHED'}

class TG_OT_add_branch(bpy.types.Operator):
    bl_idname='tg.add_branch';bl_label='新增可编辑枝条';bl_options={'REGISTER','UNDO'}
    def execute(self,ctx):
        active=ctx.object;root=root_of(active)
        if not root:self.report({'WARNING'},'先制作可编辑副本并选中树木部件。');return {'CANCELLED'}
        items=objects_for(root,ctx.scene);rig=next((o for o in items if o.type=='ARMATURE'),None)
        template=next((o for o in items if o.get('template_only')),None)
        if not template:self.report({'WARNING'},'该资产未提供枝条模板。');return {'CANCELLED'}
        bone=ctx.active_pose_bone if active==rig else None
        # A bone parent makes the new branch follow later edits to the selected branch.
        parent_bone=bone.name if bone else (rig.data.bones[0].name if rig and rig.data.bones else None)
        attach=Vector((0,0,0))
        if rig and parent_bone:attach=rig.pose.bones[parent_bone].tail.copy()
        o=template.copy();o.data=template.data.copy();o.name='EDIT_NewBranch';o['template_only']=False
        ctx.collection.objects.link(o);o.hide_render=False;o.hide_set(False)
        for mod in o.modifiers:mod.show_viewport=True;mod.show_render=True
        if rig and parent_bone:
            o.parent=rig;o.parent_type='BONE';o.parent_bone=parent_bone
            ctx.view_layer.update()
            o.matrix_world=rig.matrix_world @ Matrix.Translation(attach)
        else:o.parent=root;o.location=attach
        o['usage']='Tab 编辑曲线控制点；修改器 Branch Radius 调整粗细；此对象是新增几何，可独立删除。'
        select_only(ctx,o);bpy.ops.object.mode_set(mode='EDIT');return {'FINISHED'}

class TG_OT_select_influence(bpy.types.Operator):
    bl_idname='tg.select_branch_influence';bl_label='查看此枝条影响的顶点';bl_options={'REGISTER','UNDO'}
    def execute(self,ctx):
        rig=ctx.object;bone=ctx.active_pose_bone
        if not rig or rig.type!='ARMATURE' or not bone:self.report({'WARNING'},'在 Pose Mode 中先选择一根骨骼。');return {'CANCELLED'}
        name=bone.name;items=objects_for(root_of(rig),ctx.scene)
        mesh=next((o for o in items if o.type=='MESH' and o.vertex_groups.get(name)),None)
        if not mesh:return {'CANCELLED'}
        select_only(ctx,mesh);idx=mesh.vertex_groups[name].index
        for v in mesh.data.vertices:v.select=any(g.group==idx and g.weight>.25 for g in v.groups)
        for e in mesh.data.edges:e.select=False
        for f in mesh.data.polygons:f.select=False
        bpy.ops.object.mode_set(mode='EDIT');self.report({'INFO'},'选择的是推断权重影响范围，删除前请核对；并非准确的独立枝条分区。');return {'FINISHED'}

class TG_PT_library(bpy.types.Panel):
    bl_label='Tiny Glade 可编辑素材库';bl_idname='TG_PT_library';bl_space_type='VIEW_3D';bl_region_type='UI';bl_category='TG 素材库'
    def draw(self,ctx):
        l=self.layout;o=ctx.object
        l.operator('tg.make_editable',icon='DUPLICATE')
        root=root_of(o)
        if root:
            l.label(text=root.get('asset_id','资产'))
            l.operator('tg.select_cage',icon='MOD_LATTICE')
            l.operator('tg.pose_branches',icon='ARMATURE_DATA')
            l.operator('tg.add_branch',icon='CURVE_BEZCURVE')
            if o and o.type=='ARMATURE':l.operator('tg.select_branch_influence')
        if o and o.type=='CURVE':
            for m in o.modifiers:
                if m.type=='NODES' and m.node_group:
                    for s in m.node_group.interface.items_tree:
                        if getattr(s,'in_out',None)=='INPUT' and s.name=='Branch Radius':
                            l.prop(getattr(m.properties.inputs,s.identifier),'value',text='枝条粗细')
        l.separator();l.label(text='骨架/权重为重建推断，需按需精修。',icon='INFO')
        l.label(text='原始网格保存在 SourceArchive 场景。')

CLASSES=[TG_OT_make_editable,TG_OT_select_cage,TG_OT_pose_branches,TG_OT_add_branch,TG_OT_select_influence,TG_PT_library]
def register():
    for cls in CLASSES:bpy.utils.register_class(cls)
def unregister():
    for cls in reversed(CLASSES):bpy.utils.unregister_class(cls)
if __name__=='__main__':register()
