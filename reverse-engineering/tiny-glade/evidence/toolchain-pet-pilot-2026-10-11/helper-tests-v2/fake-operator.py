import sys,json
from pathlib import Path
steps=json.loads(Path(sys.argv[2]).read_text())
results=[]
for step in steps:
 payload={"executable_path":step["args"].get("path","fixture")}
 if step["tool"]=="get_functions":payload={"functions":{ref:{"name":"fixture","size":1,"signature":"void f(void)","decompiled_code":"return;"} for ref in step["args"]["functions"].split(",")}}
 if step["args"].get("nested_failure"):payload={"success":False,"error":"fixture nested rejection"}
 results.append({"tool":step["tool"],"args":step["args"],"result":{"isError":False,"content":[{"type":"text","text":json.dumps(payload)}]}})
Path(sys.argv[3]).write_text(json.dumps({"ok":True,"actual_mcp_connection":True,"results":results}))
