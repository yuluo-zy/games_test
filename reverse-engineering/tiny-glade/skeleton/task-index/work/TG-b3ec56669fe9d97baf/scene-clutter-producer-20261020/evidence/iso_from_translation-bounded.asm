0x140c91b30 4889c8 mov rax, rcx
0x140c91b33 f20f104204 movsd xmm0, qword ptr [rdx + 4]
0x140c91b38 f30f100a movss xmm1, dword ptr [rdx]
0x140c91b3c 0f16c8 movlhps xmm1, xmm0
0x140c91b3f 0fc6c858 shufps xmm1, xmm0, 0x58
0x140c91b43 0f2909 movaps xmmword ptr [rcx], xmm1
0x140c91b46 0f280573c5ca01 movaps xmm0, xmmword ptr [rip + 0x1cac573]
0x140c91b4d 0f294110 movaps xmmword ptr [rcx + 0x10], xmm0
0x140c91b51 c3 ret 
0x140c91b52 cc int3 
0x140c91b53 cc int3 
0x140c91b54 cc int3 
0x140c91b55 cc int3 
0x140c91b56 cc int3 
0x140c91b57 cc int3 
0x140c91b58 cc int3 
0x140c91b59 cc int3 
0x140c91b5a cc int3 
0x140c91b5b cc int3 
0x140c91b5c cc int3 
0x140c91b5d cc int3 
0x140c91b5e cc int3 
0x140c91b5f cc int3 
0x140c91b60 4889c8 mov rax, rcx
0x140c91b63 f20f104204 movsd xmm0, qword ptr [rdx + 4]
0x140c91b68 f30f100a movss xmm1, dword ptr [rdx]
0x140c91b6c 0f16c8 movlhps xmm1, xmm0
0x140c91b6f 0fc6c858 shufps xmm1, xmm0, 0x58
0x140c91b73 0f2909 movaps xmmword ptr [rcx], xmm1
0x140c91b76 410f2800 movaps xmm0, xmmword ptr [r8]
0x140c91b7a 0f294110 movaps xmmword ptr [rcx + 0x10], xmm0
0x140c91b7e c3 ret 
0x140c91b7f cc int3 
