#!/usr/bin/env python3
import json
RNE = "/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne"
steps = [{"tool":"open_file","args":{"file_path":RNE},"timeout_seconds":120},
         {"tool":"show_info","args":{},"timeout_seconds":120}]
def hx(addr,size):
    steps.append({"tool":"hexdump","args":{"address":addr,"size":size},"timeout_seconds":120})
def da(addr,n):
    steps.append({"tool":"disassemble","args":{"address":addr,"num_instructions":n},"timeout_seconds":120})
# water ambience names
hx("0x142ac51e1","64")
hx("0x142ac521e","64")
hx("0x142c77b58","64")
hx("0x142ac4d00","64")
hx("0x142ac4d20","64")
hx("0x142ac5298","64")
# menu init names + extra case names
hx("0x142ac6db8","80")
da("0x140a0b64c",22)    # menu kind 0
da("0x140a0b7c6",26)    # menu kind 7
da("0x140a0b8d4",30)    # menu kind 8
# sheep remaining kinds
da("0x1409d6014",30)
# clutter common block
da("0x1408c0990",40)
steps.append({"tool":"close_file","args":{},"timeout_seconds":120})
out = "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/tool-usage/jobs/audio-extra/radare-oracle3.json"
with open(out,"w") as fh: json.dump(steps,fh,indent=2); fh.write("\n")
print("written",out,len(steps),"steps")
