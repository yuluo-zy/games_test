14123ffc0: f30f10642428             movss xmm4, dword ptr [rsp + 0x28]
14123ffc6: 0f2ed4                   ucomiss xmm2, xmm4
14123ffc9: 0f97c1                   seta cl
14123ffcc: f30f10542430             movss xmm2, dword ptr [rsp + 0x30]
14123ffd2: 0fc2ca01                 cmpltps xmm1, xmm2
14123ffd6: 0fc2c401                 cmpltps xmm0, xmm4
14123ffda: 0f54c1                   andps xmm0, xmm1
14123ffdd: 0f2eda                   ucomiss xmm3, xmm2
14123ffe0: 0f97c0                   seta al
14123ffe3: 660f7ec2                 movd edx, xmm0
14123ffe7: 20c8                     and al, cl
14123ffe9: 20d0                     and al, dl
14123ffeb: c3                       ret 
