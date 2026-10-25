from pathlib import Path
exec(Path('D:/game/reverse-engineering/tiny-glade/evidence/round12/independent-flow-wiring/extract_startup_followup.py').read_text().split('addresses=')[0])
va=0x14059d450;n,name=procs[va];ins=list(md.disasm(read(va,n),va));lines=[]
for i in ins:
 if i.mnemonic=='call' and i.op_str.startswith('0x'):suffix=' ; '+procs.get(int(i.op_str,16),(0,''))[1]
 else:suffix=''
 lines.append(f'{i.address:x}: {i.mnemonic} {i.op_str}{suffix}')
(out/'skip-splash-original.asm').write_text('\n'.join(lines)+'\n',encoding='utf-8')
selected=[]
for i in ins:
 if (0x1405a4150<=i.address<0x1405a43a0) or (i.mnemonic=='mov' and '+ 0x107]' in i.op_str) or (i.mnemonic=='call' and i.op_str in ['0x140127e40','0x1404d8100','0x1404d8250']):
  selected.append({'pc':hex(i.address),'bytes':i.bytes.hex(),'instruction':i.mnemonic+' '+i.op_str,'callee':procs.get(int(i.op_str,16),(0,''))[1] if i.mnemonic=='call' and i.op_str.startswith('0x') else None})
result={'original_sha256':sha,'full_function':{'va':hex(va),'size':n,'sha256':hashlib.sha256(read(va,n)).hexdigest(),'symbol':name},'selected':selected,'interpretation':['config+E4 skip_splash copied to stack+107; direct original native branch tests that byte','skip=true native path removes PendingEguiContextResource, converts identity, inserts EguiContextResource, appends StartupRevealFinished, runs gameplay GameLoop directly','skip=false builds distinct loading App, spawns async load, and invokes separate GameLoop construction; no synchronous skip-path warmup invented'],'type_ids':{'PendingEguiContextResource':['aed7b5777ef68ba3','5834a199b0692b72'],'StartupRevealFinished':['d8f93d064ea9d61c','6447617066d9832f']}}
(out/'skip-splash-original.json').write_text(json.dumps(result,indent=2),encoding='utf-8')
for i in ins:
 if 0x1405a4150<=i.address<0x1405a41bc:print(hex(i.address),i.mnemonic,i.op_str)
print('nativesha',result['full_function']['sha256'])
