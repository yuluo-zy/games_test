140aa65e0: f30f2a4948                   cvtsi2ss xmm1, dword ptr [rcx + 0x48]
140aa65e5: f30f104150                   movss xmm0, dword ptr [rcx + 0x50]
140aa65ea: f30f5ec1                     divss xmm0, xmm1
140aa65ee: f30f2a514c                   cvtsi2ss xmm2, dword ptr [rcx + 0x4c]
140aa65f3: f30f104954                   movss xmm1, dword ptr [rcx + 0x54]
140aa65f8: f30f5eca                     divss xmm1, xmm2
140aa65fc: c3                           ret 
