import json, os
OUT = "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/reconstruction/country-core-render/wp4/_analysis"
job = [{
    "tool": "get_functions",
    "args": {
        "program": "tiny-glade.exe",
        "functions": "0x14093c200,0x14092d8d0,0x14093c970,0x141209a70,0x14093cf30",
        "fields": "signature,decompiled_code,callees",
        "include_call_context": False
    },
    "timeout_seconds": 300
}]
with open(os.path.join(OUT, "job_E.json"), "w") as f:
    json.dump(job, f, indent=1)
print("written")
