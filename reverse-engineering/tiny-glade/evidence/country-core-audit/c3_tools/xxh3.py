import sys,os
sys.path.insert(0,'/home/ubuntu/test/games_test/reverse-engineering/tiny-glade/tools/country-core-audit')
from rdva import rd
SEC=rd(0x142b2ebb0,192)
M=(1<<64)-1
P32_1=0x9E3779B1;P32_2=0x85EBCA77;P32_3=0xC2B2AE3D
P64_1=0x9E3779B185EBCA87;P64_2=0xC2B2AE3D27D4EB4F;P64_3=0x165667B19E3779F9;P64_4=0x85EBCA77C2B2AE63;P64_5=0x27D4EB2F165667C5
def r64(b,o): return int.from_bytes(b[o:o+8],'little')
def r32(b,o): return int.from_bytes(b[o:o+4],'little')
def fold(a,b):
    p=a*b; return (p&M)^(p>>64)
def mix16(i,o,so): return fold(r64(i,o)^r64(SEC,so), r64(i,o+8)^r64(SEC,so+8))
def avalanche(h):
    h^=h>>37; h=(h*0x165667919E3779F9)&M; h^=h>>32; return h
def xxh64_av(h):
    h^=h>>33; h=(h*P64_2)&M; h^=h>>29; h=(h*P64_3)&M; h^=h>>32; return h
def rotl(x,r): return ((x<<r)|(x>>(64-r)))&M
def xxh3(i):
    n=len(i)
    if n==0:
        return xxh64_av(r64(SEC,56)^r64(SEC,64))
    if n<=3:
        c1=i[0];c2=i[n>>1];c3=i[n-1]
        comb=(c1<<16)|(c2<<24)|(c3<<0)|(n<<8)
        bf=(r32(SEC,0)^r32(SEC,4))&M
        return xxh64_av((comb^bf)&M)
    if n<=8:
        in1=r32(i,0);in2=r32(i,n-4)
        bf=(r64(SEC,8)^r64(SEC,16))
        # seed 0
        in64=in2+(in1<<32)
        keyed=in64^bf
        h=keyed
        h^=rotl(h,49)^rotl(h,24)
        h=(h*0x9FB21C651E98DF25)&M
        h^=(h>>35)+n
        h=(h*0x9FB21C651E98DF25)&M
        h^=h>>28
        return h
    if n<=16:
        bf1=r64(SEC,24)^r64(SEC,32)
        bf2=r64(SEC,40)^r64(SEC,48)
        lo=r64(i,0)^bf1
        hi=r64(i,n-8)^bf2
        acc=(n+((lo<<0)&0)+ (int.from_bytes(lo.to_bytes(8,'little'),'little')<<0) )&M
        acc=(n + int.from_bytes(lo.to_bytes(8,'little'),'big') + hi + fold(lo,hi))&M
        return avalanche(acc)
    if n<=128:
        acc=(n*P64_1)&M
        if n>32:
            if n>64:
                if n>96:
                    acc+=mix16(i,48,96); acc+=mix16(i,n-64,112)
                acc+=mix16(i,32,64); acc+=mix16(i,n-48,80)
            acc+=mix16(i,16,32); acc+=mix16(i,n-32,48)
        acc+=mix16(i,0,0); acc+=mix16(i,n-16,16)
        return avalanche(acc&M)
    if n<=240:
        acc=(n*P64_1)&M
        for k in range(8): acc+=mix16(i,16*k,16*k)
        acc=avalanche(acc&M)
        for k in range(8,(n//16)): acc+=mix16(i,16*k,16*(k-8)+3)
        acc+=mix16(i,n-16,136-17)
        return avalanche(acc&M)
    raise ValueError
if __name__=='__main__':
    assert xxh3(b'Cobblestone3RasterStagingSsbo')==0x802ff23dee42ab47
    print(hex(xxh3(b'ColorLibrary')))
