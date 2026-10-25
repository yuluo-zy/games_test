import json,sys
from pathlib import Path
job=json.loads(Path(sys.argv[2]).read_text());kind=sys.argv[-1]
funcs={"0x140":{"name":"fixture_a","signature":"float fixture_a(float x)","decompiled_code":"float fixture_a(float x) { return x + 1.0f; }"},"0x150":{"name":"fixture_b","signature":"float fixture_b(float x)","decompiled_code":"float fixture_b(float x) { return x + 2.0f; }"}}
if kind=="placeholder":
 for f in funcs.values():f.update(signature="void pet(void)",decompiled_code="void pet(void) { return; }")
payload={"functions":funcs,"count":2,"requested":2}
if kind=="nestederror":payload["functions"]["0x150"]={"success":False,"error":"fixture failure"}
result={"isError":False,"structuredContent":payload,"content":[{"type":"text","text":json.dumps(payload)}]}
report={"ok":True,"actual_mcp_connection":True,"results":[{"tool":job[0]["tool"],"args":job[0]["args"],"result":result}]}
Path(sys.argv[3]).write_text(json.dumps(report))
