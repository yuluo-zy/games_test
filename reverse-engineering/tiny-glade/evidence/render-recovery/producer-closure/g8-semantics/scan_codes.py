import struct, json, re, sys

EXE = 'workspace/input/tiny-glade.exe'
IB = 0x140000000
d = open(EXE,'rb').read()
e = struct.unpack_from('<I', d, 0x3c)[0]
nsec = struct.unpack_from('<H', d, e+6)[0]
optsz = struct.unpack_from('<H', d, e+20)[0]
secs = []
off = e + 24 + optsz
for i in range(nsec):
    nm = d[off:off+8].rstrip(b'\x00').decode('latin1')
    vsz, va, rsz, ra = struct.unpack_from('<IIII', d, off+8)
    secs.append((nm, va, vsz, ra, rsz)); off += 40
text = next(s for s in secs if s[0]=='.text')

sym = json.load(open('evidence/render-recovery/producer-closure/c1-country-core/country_core_render_symbols.json'))
accessors = set()
for s in sym['render_symbols']:
    if 'pass_resource_access' in s['name'] and ('access_buffer' in s['name'] or 'access_texture' in s['name']):
        accessors.add(int(s['va'],16))
print('accessor count', len(accessors))

nm, va, vsz, ra, rsz = text
data = d[ra:ra+max(vsz,rsz)]
want = {0x17,0x19,0x1d,0x1e,0x1a,0x24,0x37,0x02,0x34}
hits = 0
for i in range(len(data)-5):
    if data[i]==0xE8:
        rel = struct.unpack_from('<i', data, i+1)[0]
        site = IB + va + i
        tgt = site + 5 + rel
        if tgt in accessors:
            # look back 40 bytes for an immediate load of a code constant into a reg/mem
            back = data[max(0,i-40):i]
            ctx = ' '.join('%02X'%b for b in back[-16:])
            # find mov r32,imm32 (B8-BF) or 6A (push imm8) or C6/C7 or 41 B8
            found=None
            for j in range(len(back)-5):
                if 0xB8<=back[j]<=0xBF:
                    imm=struct.unpack_from('<I',back,j+1)[0]
                    if imm in want: found=('movr',imm)
                if back[j]==0x6A:
                    if back[j+1] in want: found=('push',back[j+1])
                if back[j]==0x41 and j+5<len(back) and 0xB8<=back[j+1]<=0xBF:
                    imm=struct.unpack_from('<I',back,j+2)[0]
                    if imm in want: found=('movr8',imm)
            if found:
                hits+=1
                print('site %X -> %X  code=%s  ctx=%s' % (site,tgt,found,ctx))
print('hits', hits)
