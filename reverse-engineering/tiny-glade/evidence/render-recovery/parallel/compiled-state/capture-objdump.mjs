import {spawnSync} from 'node:child_process';
import {writeFileSync} from 'node:fs';
const target='D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe';
const ranges=[['0x140be95e0','0x140be9695'],['0x1402c4280','0x1402c4401'],['0x1402c3240','0x1402c34c0']];
const results=ranges.map(([start,stop])=>{
 const args=['-d','--x86-asm-syntax=intel','--start-address='+start,'--stop-address='+stop,target];
 const result=spawnSync('C:/Program Files/LLVM/bin/llvm-objdump.exe',args,{encoding:'utf8'});
 if(result.status!==0)throw Error(result.stderr);
 return {start,stop,command:'llvm-objdump',args,stdout:result.stdout};
});
writeFileSync(new URL('objdump-core.json',import.meta.url),JSON.stringify({target:'workspace/input/tiny-glade.exe',evidence_mode:'static',results},null,2));
