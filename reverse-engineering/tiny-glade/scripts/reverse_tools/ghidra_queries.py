"""Bounded compatibility queries, executed by the real ReVa/Ghidra runtime.

This is a project adapter, not a claim that the former ghidra MCP is installed.
Names are paired with original LLVM PDB evidence; inferred prototypes stay flagged.
"""
import json
from common import ROOT

READ = {'get_metadata', 'find_functions', 'get_functions', 'get_xrefs_to', 'get_xrefs_from', 'read_memory'}
WRITE = {'prepare_functions', 'save_program'}
SYMBOLS = ROOT / 'evidence/render-architecture/public-symbols.json'

def schema(name):
    properties={'program':{'type':'string','const':'tiny-glade.exe'}}
    extra={
        'get_metadata':{},
        'find_functions':{'pattern':{'type':'string'},'regex':{'type':'boolean'},'limit':{'type':'integer','minimum':1,'maximum':100},'offset':{'type':'integer','minimum':0}},
        'get_functions':{'functions':{'type':'string','description':'1..20 comma-separated addresses'},'function':{'type':'string'},'fields':{'type':'string','description':'signature,parameters,locals,decompiled_code,callees,callers,body,name'}},
        'read_memory':{'address':{'type':'string'},'length':{'type':'integer','minimum':1,'maximum':4096}},
        'get_xrefs_to':{'address':{'type':'string'},'limit':{'type':'integer','minimum':1,'maximum':100}},
        'get_xrefs_from':{'address':{'type':'string'},'limit':{'type':'integer','minimum':1,'maximum':100}},
        'prepare_functions':{'targets':{'type':'array','minItems':1,'maxItems':20,'description':'Each row: address, size (1..24000), bytes_sha256 of original code, range_basis'}},
        'save_program':{}}
    if name not in extra:raise ValueError('Unknown adapter query: '+name)
    properties.update(extra[name])
    required=['program']
    if name in {'read_memory','get_xrefs_to','get_xrefs_from'}:required+=['address']
    if name=='read_memory':required+=['length']
    if name=='prepare_functions':required+=['targets']
    return {'type':'object','properties':properties,'required':required,
            'description':'Windows compatibility adapter over the actual ReVa/Ghidra runtime'}

def script(name, values):
    if values.get('program') != 'tiny-glade.exe':
        raise ValueError('Specify program="tiny-glade.exe"')
    if name not in READ | WRITE:
        raise ValueError('Unsupported Windows Ghidra adapter query: ' + name)
    code = 'import json\np=currentProgram\nfm=p.getFunctionManager()\naf=p.getAddressFactory()\n'
    code += 'v=json.loads(' + repr(json.dumps(values)) + ')\n'
    code += 'def addr(a):\n    return af.getAddress(str(a).removeprefix("0x"))\n'
    if name == 'get_metadata':
        code += 'print(json.dumps({"name":p.getName(),"sha256":p.getExecutableSHA256(),"path":p.getDomainFile().getPathname(),"functions":fm.getFunctionCount(),"language":str(p.getLanguageID()),"adapter":"Windows ReVa transport / Ghidra engine","preexisting_Mac_overlays":False}))\n'
    elif name == 'find_functions':
        # Original names are searchable without a whole-program native analysis.
        code += 'from pathlib import Path\nimport re\nsymbols=json.loads(Path(' + repr(str(SYMBOLS)) + ').read_text(encoding="utf-8"))\n'
        code += 'pattern=v.get("pattern",v.get("query",v.get("name","")))\n'
        code += 'limit=int(v.get("limit",20))\nif not 1<=limit<=100: raise ValueError("limit must be 1..100")\n'
        code += 'rx=re.compile(pattern) if v.get("regex",False) else None\nrows=[]\n'
        code += 'for s in symbols:\n    n=s.get("demangled","")\n    if s.get("va") and ((rx.search(n) if rx else pattern.lower() in n.lower())):\n        rows.append({"address":s["va"],"name":n,"basis":"original LLVM PDB public symbol; not ABI"})\n'
        code += 'offset=int(v.get("offset",0))\nprint(json.dumps({"total_matches":len(rows),"functions":rows[offset:offset+limit]}))\n'
    elif name == 'read_memory':
        code += 'from ghidra.program.flatapi import FlatProgramAPI\nn=int(v["length"])\nif not 1<=n<=4096: raise ValueError("length must be 1..4096")\n'
        code += 'b=FlatProgramAPI(p,monitor).getBytes(addr(v["address"]),n)\nprint(json.dumps({"address":v["address"],"length":n,"hex":bytes(x&255 for x in b).hex()}))\n'
    elif name in {'get_xrefs_to','get_xrefs_from'}:
        method='getReferencesTo' if name=='get_xrefs_to' else 'getReferencesFrom'
        code += 'limit=int(v.get("limit",20))\nif not 1<=limit<=100: raise ValueError("limit must be 1..100")\nrows=[]\n'
        code += 'for r in p.getReferenceManager().' + method + '(addr(v["address"])):\n    rows.append({"from":str(r.getFromAddress()),"to":str(r.getToAddress()),"type":str(r.getReferenceType()),"source":str(r.getSource())})\n'
        code += 'print(json.dumps({"references":rows[:limit],"observed_count":len(rows),"limit":limit,"absence_is_not_no_caller":True}))\n'
    elif name == 'get_functions':
        functions = str(values.get('functions',values.get('function',''))).split(',')
        if not 1<=len(functions)<=20 or not functions[0]: raise ValueError('Provide 1..20 comma-separated function addresses')
        fields = str(values.get('fields','signature')).split(',')
        allowed={'signature','parameters','locals','decompiled_code','callees','callers','body','name'}
        if set(fields)-allowed: raise ValueError('Unknown fields: '+str(set(fields)-allowed))
        code += 'from ghidra.app.decompiler import DecompInterface\ndi=DecompInterface()\ndi.openProgram(p)\nrows=[]\ntry:\n'
        code += '    for a in ' + repr(functions) + ':\n        monitor.checkCancelled()\n        f=fm.getFunctionAt(addr(a))\n        if f is None:\n            rows.append({"address":a,"error":"No function at address"}); continue\n'
        code += '        row={"address":a,"name":f.getName(),"signature":str(f.getSignature()),"signature_source":str(f.getSignatureSource()),"body_bytes":f.getBody().getNumAddresses(),"prototype_is_not_verified_Rust_ABI":True}\n'
        if 'parameters' in fields: code += '        row["parameters"]=[str(x) for x in f.getParameters()]\n'
        if 'locals' in fields: code += '        row["locals"]=[str(x) for x in f.getLocalVariables()]\n'
        if 'callees' in fields: code += '        row["callees"]=[{"address":str(x.getEntryPoint()),"name":x.getName()} for x in f.getCalledFunctions(monitor)]\n'
        if 'callers' in fields: code += '        row["callers"]=[{"address":str(x.getEntryPoint()),"name":x.getName()} for x in f.getCallingFunctions(monitor)]\n'
        if 'decompiled_code' in fields:
            code += '        if f.getBody().getNumAddresses()<=1:\n            row["decompilation_gap"]="Imported one-byte function body; prepare original bounded code in a write worker first"\n        else:\n            d=di.decompileFunction(f,30,monitor)\n            row["decompiler_completed"]=d.decompileCompleted()\n            row["decompiled_code"]=d.getDecompiledFunction().getC() if d.decompileCompleted() else None\n            row["decompiler_error"]=str(d.getErrorMessage())\n'
        code += '        rows.append(row)\nfinally:\n    di.dispose()\nprint(json.dumps({"functions":rows}))\n'
    elif name == 'save_program':
        code += 'if p.isChanged(): p.save("Explicit Windows worker checkpoint",monitor)\nprint(json.dumps({"changed_after_save":p.isChanged()}))\n'
    elif name == 'prepare_functions':
        targets=values.get('targets',[])
        if not 1<=len(targets)<=20: raise ValueError('Provide 1..20 original bounded targets')
        for row in targets:
            if not 1<=int(row['size'])<=24000 or len(row['bytes_sha256'])!=64:
                raise ValueError('Each target needs bounded size and original byte SHA256')
        code += 'import hashlib\nfrom ghidra.program.disassemble import Disassembler\nfrom ghidra.program.model.address import AddressSet\nfrom ghidra.program.flatapi import FlatProgramAPI\napi=FlatProgramAPI(p,monitor)\nrows=[]\n'
        code += 'for row in v["targets"]:\n    monitor.checkCancelled()\n    a=addr(row["address"])\n    size=int(row["size"])\n    raw=bytes(x&255 for x in api.getBytes(a,size))\n    if hashlib.sha256(raw).hexdigest()!=row["bytes_sha256"]: raise ValueError("Original byte identity mismatch at "+row["address"])\n'
        code += '    tx=p.startTransaction("Bounded original code preparation")\n    commit=False\n    try:\n        decoded=Disassembler.getDisassembler(p,monitor,None).disassemble(a,AddressSet(a,a.add(size-1)),True)\n        f=fm.getFunctionAt(a)\n        if f is None: f=api.createFunction(a,None)\n        if f is None: raise ValueError("Cannot create function at "+str(a))\n        for other in fm.getFunctionsOverlapping(decoded):\n            if other.getEntryPoint()!=a: decoded.delete(other.getBody())\n        if not decoded.isEmpty(): f.setBody(decoded)\n        rows.append({"address":row["address"],"requested_bytes":size,"body_bytes":f.getBody().getNumAddresses(),"range_basis":row.get("range_basis"),"prototype_changed":False})\n        commit=True\n    finally:\n        p.endTransaction(tx,commit)\nprint(json.dumps({"prepared":rows,"no_whole_program_analysis":True}))\n'
    return code
