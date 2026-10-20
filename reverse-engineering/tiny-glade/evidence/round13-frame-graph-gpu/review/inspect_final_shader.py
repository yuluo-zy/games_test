import pathlib,struct,json
p=pathlib.Path('reverse-engineering/tiny-glade/evidence/render-passes/shaders/oracle-bin/_final_post.raster.hlsl_b903e43ffb3da915.bin');b=p.read_bytes();pos=0
while True:
 pos=b.find(bytes.fromhex('03022307'),pos)
 if pos<0:break
 print(pos,b[max(0,pos-12):pos].hex());pos+=4
