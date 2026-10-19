0x140c91b30 4889c8 mov rax, rcx
0x140c91b33 f20f104204 movsd xmm0, qword ptr [rdx + 4]
0x140c91b38 f30f100a movss xmm1, dword ptr [rdx]
0x140c91b3c 0f16c8 movlhps xmm1, xmm0
0x140c91b3f 0fc6c858 shufps xmm1, xmm0, 0x58
0x140c91b43 0f2909 movaps xmmword ptr [rcx], xmm1
0x140c91b46 0f280573c5ca01 movaps xmm0, xmmword ptr [rip + 0x1cac573]
0x140c91b4d 0f294110 movaps xmmword ptr [rcx + 0x10], xmm0
0x140c91b51 c3 ret 
