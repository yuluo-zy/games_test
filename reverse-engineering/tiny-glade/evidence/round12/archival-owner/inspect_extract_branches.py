from pathlib import Path
import re
O=Path(__file__).resolve().parent
s=(O/'extract-140b2be70.c').read_text().splitlines()
for i,line in enumerate(s):
 if re.search(r'param_[567]|FUN_|func_|return|code_r|pauStack_|param_1\[',line) and i>170:
  print(f'{i+1}: {line}')
