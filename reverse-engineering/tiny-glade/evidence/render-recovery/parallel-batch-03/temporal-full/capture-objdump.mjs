import {spawnSync} from 'node:child_process';
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
const target=process.cwd()+'/workspace/input/tiny-glade.exe';
const sha256=createHash('sha256').update(readFileSync(target)).digest('hex');
if(sha256!=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03')throw Error('Target identity mismatch');
const results=[['0x14096c310',1396],['0x14096c890',1532],['0x14096ce90',1532],['0x14096d490',610]].map(([va,size])=>{
 const args=['-d','--x86-asm-syntax=intel','--start-address='+va,'--stop-address=0x'+(BigInt(va)+BigInt(size)).toString(16),target];
 const result=spawnSync('C:/Program Files/LLVM/bin/llvm-objdump.exe',args,{encoding:'utf8'});
 if(result.status!==0)throw Error(result.stderr);
 writeFileSync(new URL(va+'.asm.txt',import.meta.url),result.stdout);
 return {va,size,args,stdout:result.stdout};
});
writeFileSync(new URL('objdump-core.json',import.meta.url),JSON.stringify({sha256,target_execution:false,results},null,2));
