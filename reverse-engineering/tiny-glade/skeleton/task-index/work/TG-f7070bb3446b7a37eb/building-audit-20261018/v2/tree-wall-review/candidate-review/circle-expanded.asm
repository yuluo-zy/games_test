140c980c0: 4889c8                       mov rax, rcx
140c980c3: f30f5915a5d7c801             mulss xmm2, dword ptr [rip + 0x1c8d7a5]
140c980cb: f30f585208                   addss xmm2, dword ptr [rdx + 8]
140c980d0: f30f10420c                   movss xmm0, dword ptr [rdx + 0xc]
140c980d5: f20f100a                     movsd xmm1, qword ptr [rdx]
140c980d9: f20f1109                     movsd qword ptr [rcx], xmm1
140c980dd: f30f115108                   movss dword ptr [rcx + 8], xmm2
140c980e2: f30f11410c                   movss dword ptr [rcx + 0xc], xmm0
140c980e7: c3                           ret 
140c980e8: cc                           int3 
140c980e9: cc                           int3 
140c980ea: cc                           int3 
140c980eb: cc                           int3 
140c980ec: cc                           int3 
140c980ed: cc                           int3 
140c980ee: cc                           int3 
140c980ef: cc                           int3 
