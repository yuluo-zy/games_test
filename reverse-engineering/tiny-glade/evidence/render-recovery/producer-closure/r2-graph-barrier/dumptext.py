import json,sys
p=sys.argv[1]; idx=int(sys.argv[2])
d=json.load(open(p,encoding='utf-8'))
r=d['results'][idx]
sc=r.get('result',{}).get('structuredContent',{})
print("FUNC:",sc.get('function_name'),sc.get('start'),sc.get('end'),"nins=",sc.get('instruction_count'))
for ins in sc.get('instructions',[]):
    if isinstance(ins,dict):
        print(ins.get('address'), ins.get('mnemonic') or ins.get('text') or ins)
    else:
        print(ins)
