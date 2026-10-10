from pathlib import Path
import json,re
OUT=Path('D:/game/reverse-engineering/tiny-glade/evidence/pdb-structure')
data=json.loads((OUT/'procedures.json').read_text(encoding='utf-8'))
for file in OUT.glob('roof-nearby-module*.txt'):
    lines=file.read_text(encoding='utf-8-sig').splitlines()
    for i,line in enumerate(lines):
        if 'S_GPROC32 ' in line:
            name=re.search(r'`([^`]+)`',line).group(1);sec,off,size=map(int,re.search(r'addr = (\d+):(\d+), code size = (\d+)',lines[i+1]).groups())
            data['procedures'].append({'name':name,'section':sec,'section_offset':off,'va':hex(0x140001000+off),'code_size':size,'end_exclusive':hex(0x140001000+off+size),'evidence':str(file),'line':i+1,'pdb_type_record':lines[i+2].strip()})
data['procedures']=list({(x['name'],x['va']):x for x in data['procedures']}.values())
(OUT/'procedures.json').write_text(json.dumps(data,ensure_ascii=False,indent=2),encoding='utf-8')
TPI=Path('D:/game/TinyGlade_逆向初查/pdb-types.txt'); text=TPI.read_text(encoding='utf-8-sig')
terms=['country_core','system_roof','system_decorator','system_wall_constructor','WindowSize','RoofShape','HistoryMutationToken','WallHistoryEdit','PublicWalls','PublicWallState','RoofTriggerVolume']
type_names=['country_core::resources::history::EditType','country_core::resources::history::wall_compound_edit::WallHistoryEdit','country_core::resources::history::roof_edit::RoofEdit','country_core::resources::history::decorator_edit::DecoratorHistoryEdit','country_core::resources::history::color_edit::ColorEdit','country_core::resources::history::tree_edit::TreeEdit','country_core::resources::history::stairs_history_edit::StairsHistoryEdit','country_core::resources::history::terrain_edit::TerrainEdit','country_core::resources::history::HistoryMutationToken','country_core::resources::roofs::roof_shape::Roof','country_core::resources::walls::public_walls::PublicWalls','country_core::resources::walls::public_wall_state::PublicWallState','country_core::resources::window::WindowSize']
coverage={'source':str(TPI),'scope':'All 6125 records in existing full TPI dump; literal text search, not a claim about every PDB stream',
 'literal_hits':{term:text.count(term) for term in terms},
 'false_positive_note':'WindowSize substring hit is the unrelated C field maxWindowSize at pdb-types.txt:7721; exact WindowSize type-name not found',
 'codeview_placeholder':{'index':'0x1001','kind':'LF_PROCEDURE','return':'void','args':0,'real_rust_signature_recovered':False},
 'observed_game_type_names':[{'name':t,'basis':'Compiled procedure/type-reference name, not a layout record','fields':None,'sizeof':None} for t in type_names],
 'window_aspect_ratio':{'entry':'0x140949c20','size':20,'end_exclusive':'0x140949c34','reads':[{'base_register':'RCX','offset':0,'read_bytes':4,'value_semantics':'zero-extended u32','field_name':None},{'base_register':'RCX','offset':4,'read_bytes':4,'value_semantics':'zero-extended u32','field_name':None}],'return_semantics':'float32(value_at_0) / float32(value_at_4)','zero_check':False,'total_struct_size':None,'evidence':'window-aspect-full-disassembly.txt:7'},
 'limitations':['Game type-name presence in generic procedure signatures does not provide field layouts','No S_LOCAL or S_GDATA32/S_LDATA32 records in five initially sampled modules','8-byte readable span does not establish WindowSize total sizeof or width/height field names','No target execution; arithmetic formula follows six observed instructions']}
(OUT/'type-layout-coverage.json').write_text(json.dumps(coverage,ensure_ascii=False,indent=2),encoding='utf-8')
print(json.dumps({'tpi_literal_hits':coverage['literal_hits'],'procedure_count':len(data['procedures']),'verified_window_reads':coverage['window_aspect_ratio']},indent=2))
