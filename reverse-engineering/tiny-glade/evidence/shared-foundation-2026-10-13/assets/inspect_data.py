exec(open(__file__.replace('inspect_data.py','disassemble.py')).read().split('rows=json')[0])
for addr,n in [(0x143150340,128),(0x1431503b0,48),(0x143150438,128),(0x142a10e28,20),(0x142fdc090,32),(0x142cdf728,360)]:
 q=read(addr,n); print(hex(addr),q, 'floats',struct.unpack('<'+'f'*(len(q)//4),q[:len(q)//4*4]) if n==20 else '')
 for i in range(0,len(q)-15,16):
  a,l=struct.unpack_from('<QQ',q,i)
  if base<=a<base+0x5000000 and l<500:print('string',hex(a),l,read(a,l))
