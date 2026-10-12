from pathlib import Path
import bpy, addon_utils, shutil
ROOT=Path(__file__).resolve().parents[1]
addons=Path(bpy.utils.user_resource('SCRIPTS',path='addons',create=True))
shutil.copy2(ROOT/'addon/tinyglade_asset_tools.py',addons/'tinyglade_asset_tools.py')
addon_utils.enable('tinyglade_asset_tools',default_set=True,persistent=True)
libraries=bpy.context.preferences.filepaths.asset_libraries
old=next((lib for lib in libraries if lib.name=='Tiny Glade Editable'),None)
if old:old.path=str(ROOT/'blends')
else:libraries.new(name='Tiny Glade Editable',directory=str(ROOT/'blends'))
bpy.ops.wm.save_userpref()
print('INSTALLED_LIBRARY',[(lib.name,lib.path) for lib in libraries if lib.name=='Tiny Glade Editable'])
print('ADDON_ENABLED',bool(bpy.context.preferences.addons.get('tinyglade_asset_tools')))
