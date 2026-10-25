140c99570: mov rax, rcx
140c99573: movss xmm0, dword ptr [rdx]
140c99577: movss xmm1, dword ptr [rdx + 4]
140c9957c: movss dword ptr [rcx], xmm0
140c99580: mov dword ptr [rcx + 4], 0
140c99587: movss dword ptr [rcx + 8], xmm1
140c9958c: xorps xmm1, xmmword ptr [rip + 0x1c8c35d]
140c99593: movss dword ptr [rcx + 0x18], xmm1
140c99598: mov dword ptr [rcx + 0x1c], 0
140c9959f: movss dword ptr [rcx + 0x20], xmm0
140c995a4: movabs rcx, 0x3f80000000000000
140c995ae: mov qword ptr [rax + 0xc], rcx
140c995b2: mov dword ptr [rax + 0x14], 0
140c995b9: ret 
