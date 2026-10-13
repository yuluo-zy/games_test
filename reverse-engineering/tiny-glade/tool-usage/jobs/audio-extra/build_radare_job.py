#!/usr/bin/env python3
"""Build radare2 (RNE oracle) job for audio extra pass: jump tables, kind tables, strings, floats."""
import json
RNE = "/Users/yuluo/Desktop/game/ljxsj_92385/Tiny Glade/tiny-glade.rne"
steps = [{"tool":"open_file","args":{"file_path":RNE},"timeout_seconds":120},
         {"tool":"show_info","args":{},"timeout_seconds":120}]
def hx(addr,size):
    steps.append({"tool":"hexdump","args":{"address":addr,"size":size},"timeout_seconds":120})
# wall kind jump table + wall transient/param strings
hx("0x142ab4998","120")     # 30 entries
hx("0x142ab41c8","360")     # wall strings region (0x168)
# building: kind tables + Shapes names + tail jump table + loop strings + param strings
hx("0x142ce4c88","64")      # name-len table (8B entries)
hx("0x142ce4ca0","64")      # name-ptr table (8B entries)
hx("0x142ce48f8","96")      # Shapes names region
hx("0x142ce4d30","64")      # tail jump table (10 entries)
hx("0x142ce4632","400")     # building loop strings + params region
# music: param strings + float constants
hx("0x142ad6398","64")      # MusicRichness/NewGladeTime/PreviewGladeType
hx("0x1429cd0f8","8")       # richness div const
hx("0x142a7fdf8","8")
hx("0x142925950","8")
hx("0x142ad58d4","16")      # 4 tier consts
hx("0x142925870","8")
hx("0x1429edb90","8")
hx("0x142a948d8","8")
hx("0x142a16cb8","8")
hx("0x142925904","8")
hx("0x142a7de00","8")       # wall inverse_lerp const
hx("0x142ad58e4","8")
hx("0x142925988","8")       # wall volume powf consts
hx("0x142925984","8")
hx("0x1429d18cc","8")       # wall volume div const
# stairs: jt + strings + params (verify)
hx("0x142bad9f0","104")     # stairs jt
hx("0x142bad6c8","160")     # stairs transient event names
hx("0x142bad818","136")     # stairs param names
steps.append({"tool":"close_file","args":{},"timeout_seconds":120})
out = "/Users/yuluo/Desktop/game/games_test/reverse-engineering/tiny-glade/tool-usage/jobs/audio-extra/radare-oracle.json"
with open(out,"w") as fh: json.dump(steps,fh,indent=2); fh.write("\n")
print("written",out,len(steps),"steps")
