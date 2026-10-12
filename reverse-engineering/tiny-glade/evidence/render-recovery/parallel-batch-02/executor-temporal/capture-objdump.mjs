import {spawnSync} from 'node:child_process';
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
const root='D:/game/reverse-engineering/tiny-glade';
const target=root+'/workspace/input/tiny-glade.exe';
const contract=JSON.parse(readFileSync(root+'/evidence/render-recovery/parallel-batch-02/contract.json','utf8'));
const sha256=createHash('sha256').update(readFileSync(target)).digest('hex');
if(sha256!==contract.identity.binary_sha256)throw Error('Target identity mismatch');
const results=contract.packages['R4-executor-temporal'].entries.map(entry=>{
 const stop='0x'+(BigInt(entry.va)+BigInt(entry.code_size)).toString(16);
 const args=['-d','--x86-asm-syntax=intel','--start-address='+entry.va,'--stop-address='+stop,target];
 const result=spawnSync('C:/Program Files/LLVM/bin/llvm-objdump.exe',args,{encoding:'utf8'});
 if(result.status!==0)throw Error(result.stderr);
 writeFileSync(new URL(entry.va+'.asm.txt',import.meta.url),result.stdout);
 return {va:entry.va,code_size:entry.code_size,names:entry.names,args,stdout:result.stdout};
});
writeFileSync(new URL('objdump-core.json',import.meta.url),JSON.stringify({target:'workspace/input/tiny-glade.exe',sha256,evidence_mode:'static',results},null,2));
