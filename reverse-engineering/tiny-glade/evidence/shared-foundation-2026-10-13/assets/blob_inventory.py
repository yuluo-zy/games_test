import pathlib,struct,json,hashlib
root=pathlib.Path(r'D:\game\ljxsj_92385\Tiny Glade\compiled-assets\textures')
out=[]
for p in root.rglob('*.texture'):
 b=p.read_bytes(); root_offset=len(b)-28
 out.append({'path':str(p),'size':len(b),'sha256':hashlib.sha256(b).hexdigest(),'magic':hex(struct.unpack_from('<I',b)[0]),'version':struct.unpack_from('<I',b,4)[0],'root_hex':b[-28:].hex(),'root':list(struct.unpack('<i5I2B2x',b[-28:]))})
pathlib.Path(__file__).with_name('blob-inventory.json').write_text(json.dumps(out,indent=2))
print(json.dumps(out[:4]+[r for r in out if 'array_roof' in r['path'] or 'black.texture' in r['path']],indent=2))
