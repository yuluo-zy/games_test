from pathlib import Path
s=Path('D:/game/reverse-engineering/tiny-glade/evidence/round12/roof-support/scan_support.py').read_text(encoding='utf8')
s=s.replace("['RoofSupportBeam','system_roof::visual::place_support_beams']", "['country_core::systems::ui::hover_intersection','country_core::systems::ui::ui_update']")
s=s.replace("'/roof/src/visual/place_support_beams.rs' in path", "('/systems/ui/hover_intersection.rs' in path or '/systems/ui/ui_update.rs' in path)")
s=s.replace("'/roof/src/visual/place_support_beams.rs' in info.get('path','')", "any(info.get('path','').endswith('/'+f) for f in ['hover_intersection.rs','ui_update.rs'])")
s=s.replace('pdb-support.json','pdb-ui-picker.json')
Path(__file__).with_name('scan_picker.py').write_text(s,encoding='utf8')
