import pathlib,json
from ghidra.app.decompiler import DecompInterface
from ghidra.app.cmd.disassemble import DisassembleCommand
from ghidra.program.model.address import AddressSet
from ghidra.program.model.symbol import SourceType
p=pathlib.Path(r'D:\game\reverse-engineering\tiny-glade\evidence\shared-foundation-2026-10-13\assets')
identity={'program':currentProgram.getName(),'sha256':currentProgram.getExecutableSHA256(),'path':currentProgram.getExecutablePath(),'database':'agent foundation_assets isolated imported B'}
assert identity['sha256']=='cf048e27acdc266b2efc7c094dd435bacecb3f26c2afe54de4d09cd3f7036323'
(p/'identity.json').write_text(json.dumps(identity,indent=2))
rows=json.loads((p/'functions.json').read_text())
fm=currentProgram.getFunctionManager()
target=[r for r in rows if r['name'].startswith(('asset::','asset_pipe::')) and '::impl$0::' not in r['name'] and '::fmt' not in r['name']]
target += [r for r in rows if r['name'].startswith('rkyv::util::to_bytes<asset::texture::blob')]
tx=currentProgram.startTransaction('foundation-assets bounded PDB bodies')
try:
 for r in target:
  monitor.checkCancelled(); a=currentProgram.getAddressFactory().getAddress(r['va'][2:]); body=AddressSet(a,a.add(r['size']-1)); cmd=DisassembleCommand(a,body,True); cmd.applyTo(currentProgram,monitor)
  for other in list(fm.getFunctions(body,True)):
   if other.getEntryPoint()!=a:
    if other.getBody().getNumAddresses()!=1:raise RuntimeError('non-stub overlaps '+str(other.getEntryPoint()))
    fm.removeFunction(other.getEntryPoint())
  fn=fm.getFunctionAt(a)
  if fn is None:fn=fm.createFunction('asset_recovery_'+r['va'][2:],a,body,SourceType.USER_DEFINED)
  else:fn.setBody(body)
 currentProgram.endTransaction(tx,True)
except:
 currentProgram.endTransaction(tx,False); raise
d=DecompInterface(); d.openProgram(currentProgram); report=[]
for r in target:
 monitor.checkCancelled(); a=currentProgram.getAddressFactory().getAddress(r['va'][2:]); result=d.decompileFunction(fm.getFunctionAt(a),60,monitor)
 if result.decompileCompleted():
  (p/('decomp-'+r['va']+'.c')).write_text(result.getDecompiledFunction().getC(),encoding='utf8'); report.append({'name':r['name'],'va':r['va'],'success':True})
 else:report.append({'name':r['name'],'va':r['va'],'success':False,'error':result.getErrorMessage()})
d.dispose(); (p/'decomp-report.json').write_text(json.dumps(report,indent=2)); print(json.dumps({'functions':len(report),'success':sum(r['success'] for r in report)}))
