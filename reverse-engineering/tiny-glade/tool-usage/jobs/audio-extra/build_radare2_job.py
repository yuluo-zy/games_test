#!/usr/bin/env python3
import json
RNE = "/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne"
steps = [{"tool":"open_file","args":{"file_path":RNE},"timeout_seconds":120},
         {"tool":"show_info","args":{},"timeout_seconds":120}]
def hx(addr,size):
    steps.append({"tool":"hexdump","args":{"address":addr,"size":size},"timeout_seconds":120})
def da(addr,n):
    steps.append({"tool":"disassemble","args":{"address":addr,"num_instructions":n},"timeout_seconds":120})
# jump tables
hx("0x142ac2cc8","128")   # sheep
hx("0x142ac706c","256")   # menu
hx("0x142aa1688","192")   # clutter
# terrain_stairs strings
hx("0x142a9dfc7","96")
hx("0x142c62e70","64")
hx("0x142c62e90","64")
hx("0x142c62240","64")
hx("0x142c62220","64")
hx("0x142c62943","32")
hx("0x142c62ea8","32")
# water ambience event/param strings
hx("0x142ac5298","48")
hx("0x142ac52c8","48")
hx("0x142ac52e0","48")
hx("0x142ac5268","48")
hx("0x142ac5280","48")
hx("0x142ac52b0","48")
hx("0x142c77bb0","48")
hx("0x142ac4d00","48")
hx("0x142ac4d20","48")
# menu init event names
hx("0x142ac6340","64")
hx("0x142ac6360","64")
hx("0x142ac6db8","64")
hx("0x142ac6e00","64")
hx("0x142ac6e18","64")
# sheep/water other strings
hx("0x142ac23b0","64")
hx("0x142ac2cc8","64")
# disassemble case regions
da("0x1409d5f9c",30)   # sheep cases
da("0x140a0b660",60)   # menu cases
da("0x1408c0690",80)   # clutter cases
steps.append({"tool":"close_file","args":{},"timeout_seconds":120})
out = "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/tool-usage/jobs/audio-extra/radare-oracle2.json"
with open(out,"w") as fh: json.dump(steps,fh,indent=2); fh.write("\n")
print("written",out,len(steps),"steps")
