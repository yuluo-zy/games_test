"""Export an ARTIST-EDITED .blend, not just the original generator recipe.

blender --background --factory-startup --python-exit-code 1 --python
  art-source/tools/export-house-kit.py -- --source art-source/models/house-kit-v4.blend
  --revision v5 [--check-only]
Requires the four named module collections and applied transforms. Never writes
back to the input .blend. New versions only, with the same strict budget gate.
"""
from pathlib import Path
import argparse
import importlib.util
import json
import sys
import bpy


def main():
    if not bpy.app.background:
        raise RuntimeError('Background only; preserves interactive Blender work')
    parser = argparse.ArgumentParser()
    parser.add_argument('--source', required=True)
    parser.add_argument('--revision', required=True)
    parser.add_argument('--check-only', action='store_true')
    args = parser.parse_args(sys.argv[sys.argv.index('--')+1:])
    assert args.revision.isalnum()
    project = Path(__file__).resolve().parents[2]
    source = Path(args.source).resolve()
    assert source.is_file() and source.suffix == '.blend'
    target = project / 'assets/themes/warm-stone' / ('house-kit-' + args.revision)
    new_source = project / 'art-source/models' / ('house-kit-' + args.revision + '.blend')
    if not args.check_only and (target.exists() or new_source.exists()):
        raise FileExistsError('New version only; refusing to overwrite assets or source')
    spec = importlib.util.spec_from_file_location('house_kit_builder', Path(__file__).with_name('build-house-kit.py'))
    builder = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(builder)
    bpy.ops.wm.open_mainfile(filepath=str(source))
    modules, reports = {}, {}
    for name, budget, mats in [('window-basic',500,2),('window-shutter',800,2),('door-timber',500,2),('chimney-stone',500,1)]:
        col = bpy.data.collections['COL_' + name.replace('-', '_')]
        # Export snapshots apply modifiers without changing artist source data.
        # Current kit forbids non-identity transforms; apply them in Blender first.
        for obj in col.objects:
            for modifier in list(obj.modifiers):
                col.hide_viewport = False
                bpy.context.view_layer.objects.active = obj
                bpy.ops.object.modifier_apply(modifier=modifier.name)
        modules[name], reports[name] = builder.audit_and_compile(col, budget, mats)
    if args.check_only:
        print('BLEND_SOURCE_CHECK ' + json.dumps(reports))
        return
    target.mkdir(parents=True)
    # Images edited/packed in Blender are saved from loaded pixel data to the
    # NEW revision, with their explicit sRGB / Non-Color settings preserved.
    for role in ['Stone','Plaster','Timber','Roof']:
        for suffix in ['BC','N','ORM']:
            image = bpy.data.images['T_WarmStone_'+role+'_'+suffix]
            if not image.has_data:
                _ = image.pixels[0]  # Force lazy loading BEFORE changing the path.
            image.filepath_raw = str(target / (image.name + '.png'))
            image.file_format = 'PNG'
            image.save()
    for name in modules:
        col = bpy.data.collections['COL_' + name.replace('-', '_')]
        col.hide_viewport = False
        col.hide_render = False
        bpy.ops.object.select_all(action='DESELECT')
        for obj in col.objects:
            obj.select_set(True)
        bpy.context.view_layer.objects.active = list(col.objects)[0]
        bpy.ops.export_scene.gltf(filepath=str(target/(name+'.glb')), export_format='GLB', use_selection=True, export_yup=True, export_apply=True, export_animations=False, export_extras=True, export_texcoords=True, export_normals=True, export_tangents=True)
        col.hide_viewport = True
        col.hide_render = True
    contract = dict(schema_version=1, units='meter', up_axis='+Y', front_axis='+Z', window_opening=[.9,1.2], modules=modules)
    (target/'kit.json').write_text(json.dumps(contract,separators=(',',':')),encoding='utf-8')
    (target/'validation.json').write_text(json.dumps(dict(blender=bpy.app.version_string,assets=reports,input_source=str(source)),indent=2),encoding='utf-8')
    for image in bpy.data.images:
        if image.filepath:
            image.filepath = bpy.path.relpath(image.filepath,start=str(new_source.parent))
    bpy.ops.wm.save_as_mainfile(filepath=str(new_source))
    print('EXPORTED_NEW_REVISION '+str(target))


if __name__ == '__main__':
    main()
