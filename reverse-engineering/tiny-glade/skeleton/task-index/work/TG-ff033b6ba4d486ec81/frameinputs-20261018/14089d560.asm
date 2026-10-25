14089d560: mov rax, rcx
14089d563: movups xmm0, xmmword ptr [rdx + 4]
14089d567: movups xmm1, xmmword ptr [rdx + 0x14]
14089d56b: movss xmm2, dword ptr [rdx + 0x10]
14089d570: shufps xmm2, xmm0, 0xd4
14089d574: unpcklpd xmm0, xmmword ptr [rdx]
14089d578: movsd xmm3, qword ptr [rdx + 0x34]
14089d57d: shufps xmm0, xmm2, 0x22
14089d581: unpcklpd xmm1, xmmword ptr [rdx + 0x20]
14089d586: movss xmm2, dword ptr [rdx + 0x30]
14089d58b: movhps xmm2, qword ptr [rdx + 0x28]
14089d58f: shufps xmm2, xmm3, 0x42
14089d593: movups xmmword ptr [rcx], xmm0
14089d596: movups xmmword ptr [rcx + 0x10], xmm1
14089d59a: movups xmmword ptr [rcx + 0x20], xmm2
14089d59e: ret 
