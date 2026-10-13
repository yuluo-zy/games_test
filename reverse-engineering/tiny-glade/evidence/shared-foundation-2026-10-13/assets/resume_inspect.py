exec(open(__file__.replace('resume_inspect.py','disassemble.py')).read().split('rows=json')[0])
for addr,n in [(0x142fdc790,32),(0x142cdf570,112),(0x142cdd960,32),(0x142b011a0,16)]:
 print(hex(addr),read(addr,n),list(struct.unpack('<'+'I'*(n//4),read(addr,n))))
