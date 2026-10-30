import json,sys
from pathlib import Path
log=Path(sys.argv[1]).read_text()
items=[line.split('NATIVE_GABLE_INPUT=',1)[1] for line in log.splitlines() if 'NATIVE_GABLE_INPUT=' in line]
if len(items)!=1:raise ValueError('Expected exactly one NATIVE_GABLE_INPUT line, got '+str(len(items)))
d=json.loads(items[0]);p={'shape_hex':bytes(d['shape']).hex(),'curve2_points':d['curve2_points'],'curve2_u':d['curve2_u'],'curve2_length':d['curve2_length'],'secondary_positions':d['mapped_points'],'secondary_u':d['mapped_u'],'secondary_length':d['mapped_length'],'prepared_positions':d['prepared_points'],'prepared_u':d['prepared_u'],'prepared_length':d['prepared_length'],'aux_heights':d['segment_values'],'wall_heights':d['arch'],'segment_heights':d['segment'],'prepared_y':d['path_y'],'max_y':d['max_y'],'max_y_gable':d['max_y_gable']}
for k,v in zip(('elevation','constrained_extra','height','wall_length'),d['public_280_words']):p[k]=v
out={'wall_id':d['wall_id'],'pws':p,'roof_88_hex':bytes(d['roof']).hex(),'production_export':d}
with Path(sys.argv[2]).open('x') as f:json.dump(out,f,indent=2);f.write('\n')
