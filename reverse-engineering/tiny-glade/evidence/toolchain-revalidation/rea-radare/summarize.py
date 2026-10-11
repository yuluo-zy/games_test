import json,pathlib,hashlib,subprocess
p=pathlib.Path(__file__).parent
load=lambda name:json.loads((p/name).read_text())
rea1,rea2=load('rea-one.json'),load('rea-two.json')
r1,r2=load('radare-one.json'),load('radare-two.json')
g1,g2=load('radare-ghidra-one.json'),load('radare-ghidra-two.json')
call=lambda j,n:next(c for c in j['calls'] if c['name']==n)
rea_result=lambda j,n:call(j,n)['result']['structuredContent']
text=lambda c:'\n'.join(b.get('text','') for b in c['result'].get('content',[]) if b['type']=='text')
original_sha=hashlib.sha256(pathlib.Path('/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne').read_bytes()).hexdigest()
checks={
 'original_B_unchanged':original_sha=='f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03',
 'REA_two_clients_overlap':max(rea1['started'],rea2['started'])<min(rea1['finished'],rea2['finished']),
 'REA_both_original_B_identity':all(rea_result(j,'open_binary')['result']['sha256']==original_sha for j in [rea1,rea2]),
 'REA_one_authentic_aspect_bytes':rea_result(rea1,'read_bytes')['raw_result']['bytes_hex']=='8b01f3480f2ac08b4104f3480f2ac8f30f5ec1c3',
 'REA_two_different_bytes_same_va':rea_result(rea2,'read_bytes')['raw_result']['bytes_hex']=='b825000000c30000000000000000000000000000',
 'REA_independent_target_digest':rea_result(rea1,'analyze_function')['subject']['digest']!=rea_result(rea2,'analyze_function')['subject']['digest'],
 'REA_aspect_signature_recovered':rea_result(rea1,'analyze_function')['raw_result']['procedure']['signature']=='float __fastcall entry(uint * param_1)',
 'REA_native_Pcode_available':rea_result(rea1,'analyze_function')['raw_result']['native_value_flow']['available'],
 'REA_other_function_return_37':'return 0x25' in rea_result(rea2,'analyze_function')['raw_result']['pseudocode'],
 'r2_two_clients_overlap':max(r1['started'],r2['started'])<min(r1['finished'],r2['finished']),
 'r2_original_B_aspect_disassembly':all('0x140949c20' in text(call(j,'disassemble')) and 'divss' in text(call(j,'disassemble')) for j in [r1,r2]),
 'r2_fixture_isolation':any('divss' in text(c) for c in r1['calls'] if c['name']=='disassemble' and c['args']['address']=='0x140001000') and any('mov eax, 0x25' in text(c) for c in r2['calls'] if c['name']=='disassemble' and c['args']['address']=='0x140001000'),
 'r2ghidra_two_clients_overlap':max(g1['started'],g2['started'])<min(g1['finished'],g2['finished']),
 'r2ghidra_aspect_division':'return *arg1 / *(arg1 + 4)' in text(call(g1,'decompile_function')),
 'r2ghidra_other_returns_37':'0x25' in text(call(g2,'decompile_function')),
 'r2_actual_call_xref':'0x140001020 [CALL:--x]' in text(call(g1,'xrefs_to')),
}
for name,passed in checks.items():assert passed,name
report={'scope':'Tool and concurrent target-session validation. REA native decompilation used explicit 1KB PE fixtures; no full-game REA autoanalysis or reconstruction claim.', 'checks':checks,
 'rea':{'version':'6.3.0','startup':{'command':'/Users/yuluo/Desktop/game/tools/node/bin/node','args':['/Users/yuluo/Desktop/game/tools/node/bin/rea','--mcp'],'env':{'GHIDRA_INSTALL_DIR':'/Users/yuluo/Desktop/game/tools/ghidra_12.1.4_PUBLIC','JAVA_HOME':'/opt/homebrew/opt/openjdk@21/libexec/openjdk.jdk/Contents/Home','REA_ANALYSIS_PROVIDER':'ghidra','REA_GHIDRA_STARTUP_TIMEOUT_MS':'120000'}},'tool_count':len(load('rea-inventory.json')['tools']),'schema_compact_bytes':len(json.dumps(load('rea-inventory.json')['tools'],separators=(',',':')).encode()),'cold_analysis_ms':[call(j,'analyze_function')['elapsed_ms'] for j in [rea1,rea2]],'usable':'PE identity, native function dossier/decompile, exact bytes, references, recovered ABI and high P-code def-use work; useful as optional independent evidence provider.','limits':['One active target per MCP server process; independent process per agent isolates target/database.','Imports a fresh ephemeral Ghidra project; closes discard annotations; cannot directly reuse existing persistent project.','Default analysis must finish before native queries; fixture cold startup about 93 seconds during concurrent build load.','inspect_binary_layout initial profile only Linux amd64 ELF, not this Mac PE target.','139-tool full schema is excessive for current Rust task; use narrow gate/lazy routing.']},
 'radare2':{'r2_version':'6.2.4','mcp_version':'1.8.8','mcp_source_commit':'d48078bb5440d5564374e102c28e1c8892784de8','r2ghidra_version':'6.2.4','r2ghidra_commit':'d690abdbfded6fa9237dec4732a9f288a1f9a887','sleigh_release':'6.1.4','startup':{'command':'/opt/homebrew/bin/r2pm','args':['-r','r2mcp','-r','-d','pdg','-C','both']},'tool_count':len(load('radare-inventory.json')['tools']),'schema_compact_bytes':len(json.dumps(load('radare-inventory.json')['tools'],separators=(',',':')).encode()),'usable':'Actual B PE identity/sections/PDB GUID, disassembly, local function creation, fixture call reference and mature pdg backend validated.','fixed':['Deleted r2mcp installation rebuilt at same version and exact repository commit.','Default pdc is assembly-like pseudo-code; missing r2ghidra + SLEIGH fixed to enable native Ghidra decompiler.'],'pdb_limits':['Default internal MCP sandbox prevents raw idp/idpi reading local PDB. disk,files grain still insufficient; isolated trusted test with -g all reached parser.','Parser then reports There is no tpi stream in current pdb, whereas LLVM PDB summary says Has Types/IDs/Globals/Publics=true. Not a missing original PDB; r2 PDB type parsing compatibility limit.','PDB import is not validated as successful; no recovered WindowSize type or signature claim.','Native r2 CLI prints publics while reporting missing TPI; retain LLVM for authoritative complete debug information extraction.'],'concurrency':'Use independent stdio process per agent. Each owns RCore; tested different binary bytes and results at identical virtual address. Shared HTTP without -X shares state; if HTTP is needed require -X and per-client X-Session-ID.','limits':['Pdg recovers useful division but arg1 type remains int64_t, unlike complete Ghidra uint*.','run_command returns textual ERROR inside successful MCP envelope; always validate content, not merely isError.','No original executable modifications; source SHA checked again.']},
 'recommendation':'Restore installations as usable optional tools. Keep Ghidra/ReVa persistent database as primary workflow and LLVM for PDB. Expose REA/r2 through per-agent lazy/narrow routing rather than every schema globally.'}
(p/'validation-summary.json').write_text(json.dumps(report,indent=2,ensure_ascii=False))
print(json.dumps({'all_checks_passed':True,'checks':len(checks),'report':str((p/'validation-summary.json').resolve())}))
