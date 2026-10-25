import json,psutil
rows=[]
for p in psutil.process_iter(["pid","cmdline","create_time"]):
 c=p.info["cmdline"] or []
 if "review-integrity-20261013" in c and any(x.endswith("daemon.py") for x in c):
  rows.append({"pid":p.pid,"created":p.create_time(),"cmdline":c})
print(json.dumps(rows))
