141293730: 56 push rsi
141293731: 57 push rdi
141293732: 53 push rbx
141293733: 4883ec30 sub rsp, 0x30
141293737: 440f29442420 movaps xmmword ptr [rsp + 0x20], xmm8
14129373d: 0f297c2410 movaps xmmword ptr [rsp + 0x10], xmm7
141293742: 0f293424 movaps xmmword ptr [rsp], xmm6
141293746: 4d8b08 mov r9, qword ptr [r8]
141293749: 4d8b5008 mov r10, qword ptr [r8 + 8]
14129374d: 4839d1 cmp rcx, rdx
141293750: 0f8472010000 je 0x1412938c8
141293756: 4829ca sub rdx, rcx
141293759: 49bbcdcccccccccccccc movabs r11, 0xcccccccccccccccd
141293763: 4889d0 mov rax, rdx
141293766: 49f7e3 mul r11
141293769: 4b8d0492 lea rax, [r10 + r10*4]
14129376d: 48c1e004 shl rax, 4
141293771: 49034010 add rax, qword ptr [r8 + 0x10]
141293775: 48c1ea06 shr rdx, 6
141293779: 48f7da neg rdx
14129377c: 41bb40000000 mov r11d, 0x40
141293782: 4531c0 xor r8d, r8d
141293785: 0f2805345a7f01 movaps xmm0, xmmword ptr [rip + 0x17f5a34]
14129378c: f30f100d3c5a7f01 movss xmm1, dword ptr [rip + 0x17f5a3c]
141293794: f30f10154c696c01 movss xmm2, dword ptr [rip + 0x16c694c]
14129379c: f30f101d64216901 movss xmm3, dword ptr [rip + 0x1692164]
1412937a4: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1412937b0: 428b3419 mov esi, dword ptr [rcx + r11]
1412937b4: 4a8b7c19f0 mov rdi, qword ptr [rcx + r11 - 0x10]
1412937b9: 420f286419c0 movaps xmm4, xmmword ptr [rcx + r11 - 0x40]
1412937bf: 0f28f4 movaps xmm6, xmm4
1412937c2: 0f59f4 mulps xmm6, xmm4
1412937c5: f30f16ee movshdup xmm5, xmm6
1412937c9: f30f58ee addss xmm5, xmm6
1412937cd: 0f28fe movaps xmm7, xmm6
1412937d0: 660f15fe unpckhpd xmm7, xmm6
1412937d4: f30f58fd addss xmm7, xmm5
1412937d8: 0fc6ff00 shufps xmm7, xmm7, 0
1412937dc: 0fc6f6ff shufps xmm6, xmm6, 0xff
1412937e0: 0f5cf7 subps xmm6, xmm7
1412937e3: 0f59f0 mulps xmm6, xmm0
1412937e6: 0f28ec movaps xmm5, xmm4
1412937e9: 0f59e8 mulps xmm5, xmm0
1412937ec: f30f16fd movshdup xmm7, xmm5
1412937f0: f30f58fd addss xmm7, xmm5
1412937f4: 0f12ed movhlps xmm5, xmm5
1412937f7: f30f58ef addss xmm5, xmm7
1412937fb: f30f58ed addss xmm5, xmm5
1412937ff: 0fc6ed00 shufps xmm5, xmm5, 0
141293803: 0f59ec mulps xmm5, xmm4
141293806: 0f58ee addps xmm5, xmm6
141293809: 0f28f4 movaps xmm6, xmm4
14129380c: 0f59f1 mulps xmm6, xmm1
14129380f: 0f28fc movaps xmm7, xmm4
141293812: 0fc6fcc9 shufps xmm7, xmm4, 0xc9
141293816: 0f59f9 mulps xmm7, xmm1
141293819: 0fc6f6d2 shufps xmm6, xmm6, 0xd2
14129381d: 0f5cfe subps xmm7, xmm6
141293820: 0f28f4 movaps xmm6, xmm4
141293823: 0f58f4 addps xmm6, xmm4
141293826: 0fc6f6ff shufps xmm6, xmm6, 0xff
14129382a: 0f59f7 mulps xmm6, xmm7
14129382d: 0f58f5 addps xmm6, xmm5
141293830: 0f28ee movaps xmm5, xmm6
141293833: 660f15ee unpckhpd xmm5, xmm6
141293837: f3420f586c19d8 addss xmm5, dword ptr [rcx + r11 - 0x28]
14129383e: f3420f107c19d4 movss xmm7, dword ptr [rcx + r11 - 0x2c]
141293845: f3460f104419d0 movss xmm8, dword ptr [rcx + r11 - 0x30]
14129384c: 420f296418c0 movaps xmmword ptr [rax + r11 - 0x40], xmm4
141293852: f3420f116c18d8 movss dword ptr [rax + r11 - 0x28], xmm5
141293859: f2420f106419f8 movsd xmm4, qword ptr [rcx + r11 - 8]
141293860: f30f16ec movshdup xmm5, xmm4
141293864: f30f59ea mulss xmm5, xmm2
141293868: f30f59eb mulss xmm5, xmm3
14129386c: f30f58ef addss xmm5, xmm7
141293870: 440f14c5 unpcklps xmm8, xmm5
141293874: 440f58c6 addps xmm8, xmm6
141293878: 460f134418d0 movlps qword ptr [rax + r11 - 0x30], xmm8
14129387e: 420f106c19dc movups xmm5, xmmword ptr [rcx + r11 - 0x24]
141293884: 420f116c18dc movups xmmword ptr [rax + r11 - 0x24], xmm5
14129388a: 428b5c19ec mov ebx, dword ptr [rcx + r11 - 0x14]
14129388f: 42895c18ec mov dword ptr [rax + r11 - 0x14], ebx
141293894: 4a897c18f0 mov qword ptr [rax + r11 - 0x10], rdi
141293899: f2420f116418f8 movsd qword ptr [rax + r11 - 8], xmm4
1412938a0: 42893418 mov dword ptr [rax + r11], esi
1412938a4: 4ac744180400000000 mov qword ptr [rax + r11 + 4], 0
1412938ad: 6642c744180c0001 mov word ptr [rax + r11 + 0xc], 0x100
1412938b5: 49ffc8 dec r8
1412938b8: 4983c350 add r11, 0x50
1412938bc: 4c39c2 cmp rdx, r8
1412938bf: 0f85ebfeffff jne 0x1412937b0
1412938c5: 4d29c2 sub r10, r8
1412938c8: 4d8911 mov qword ptr [r9], r10
1412938cb: 0f283424 movaps xmm6, xmmword ptr [rsp]
1412938cf: 0f287c2410 movaps xmm7, xmmword ptr [rsp + 0x10]
1412938d4: 440f28442420 movaps xmm8, xmmword ptr [rsp + 0x20]
1412938da: 4883c430 add rsp, 0x30
1412938de: 5b pop rbx
1412938df: 5f pop rdi
1412938e0: 5e pop rsi
1412938e1: c3 ret 
