import io, json

OUT = r"D:/game/reverse-engineering/tiny-glade/evidence/country-core-deep/round3"
s = json.load(open(OUT + "/O-sample-round2.json", encoding="utf-8"))
buf = io.StringIO()
for d in sorted(set(r["domain"] for r in s)):
    buf.write("=" * 20 + " " + d + " " + "=" * 20 + "\n")
    for r in [x for x in s if x["domain"] == d]:
        buf.write(f"  {r['rel']} | {r['addr']} | {r['fn'][:100]}\n")
        buf.write(f"      claim: {r['claim'][:200]}\n")
        if r["evid"]:
            buf.write(f"      evid : {r['evid'][:200]}\n")
io.open(OUT + "/sample_dump.txt", "w", encoding="utf-8").write(buf.getvalue())
print("ok")
