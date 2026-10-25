140c911d0: 4883ec18 sub rsp, 0x18
140c911d4: 0f293424 movaps xmmword ptr [rsp], xmm6
140c911d8: 4889c8 mov rax, rcx
140c911db: f30f1012 movss xmm2, dword ptr [rdx]
140c911df: f30f104a08 movss xmm1, dword ptr [rdx + 8]
140c911e4: 0f28c2 movaps xmm0, xmm2
140c911e7: f30f59c2 mulss xmm0, xmm2
140c911eb: 0f28d9 movaps xmm3, xmm1
140c911ee: f30f59d9 mulss xmm3, xmm1
140c911f2: f30f58d8 addss xmm3, xmm0
140c911f6: f30f51db sqrtss xmm3, xmm3
140c911fa: f30f10050247c901 movss xmm0, dword ptr [rip + 0x1c94702]
140c91202: 0f28e0 movaps xmm4, xmm0
140c91205: f30f5ee3 divss xmm4, xmm3
140c91209: 660f7ee1 movd ecx, xmm4
140c9120d: 85c9 test ecx, ecx
140c9120f: 0f99c2 setns dl
140c91212: 4189c8 mov r8d, ecx
140c91215: 4181e0ffffff7f and r8d, 0x7fffffff
140c9121c: 4181c0000080ff add r8d, 0xff800000
140c91223: 4181f80000007f cmp r8d, 0x7f000000
140c9122a: 410f92c0 setb r8b
140c9122e: 4120d0 and r8b, dl
140c91231: ffc9 dec ecx
140c91233: 81f9ffff7f00 cmp ecx, 0x7fffff
140c91239: 0f92c1 setb cl
140c9123c: 4408c1 or cl, r8b
140c9123f: 0f57db xorps xmm3, xmm3
140c91242: 7505 jne 0x140c91249
140c91244: 0f57c9 xorps xmm1, xmm1
140c91247: eb0b jmp 0x140c91254
140c91249: f30f59d4 mulss xmm2, xmm4
140c9124d: f30f59cc mulss xmm1, xmm4
140c91251: 0f28c2 movaps xmm0, xmm2
140c91254: 0f57e4 xorps xmm4, xmm4
140c91257: 0f57d2 xorps xmm2, xmm2
140c9125a: f30f10d1 movss xmm2, xmm1
140c9125e: f30f10e0 movss xmm4, xmm0
140c91262: 0f28ec movaps xmm5, xmm4
140c91265: 0f16ea movlhps xmm5, xmm2
140c91268: 0fc6e104 shufps xmm4, xmm1, 4
140c9126c: 0f59254dbed301 mulps xmm4, xmmword ptr [rip + 0x1d3be4d]
140c91273: f30f10158946c901 movss xmm2, dword ptr [rip + 0x1c94689]
140c9127b: 0fc6edd2 shufps xmm5, xmm5, 0xd2
140c9127f: 0fc6d2d2 shufps xmm2, xmm2, 0xd2
140c91283: 0f59d5 mulps xmm2, xmm5
140c91286: 0f5ce2 subps xmm4, xmm2
140c91289: 0f28d4 movaps xmm2, xmm4
140c9128c: 0fc6d4d2 shufps xmm2, xmm4, 0xd2
140c91290: 0f59e4 mulps xmm4, xmm4
140c91293: 0f28ec movaps xmm5, xmm4
140c91296: 660f15ec unpckhpd xmm5, xmm4
140c9129a: f30f58ec addss xmm5, xmm4
140c9129e: f30f16e4 movshdup xmm4, xmm4
140c912a2: f30f58e5 addss xmm4, xmm5
140c912a6: 0fc6e400 shufps xmm4, xmm4, 0
140c912aa: 0f51e4 sqrtps xmm4, xmm4
140c912ad: 0f5ed4 divps xmm2, xmm4
140c912b0: 0f28e2 movaps xmm4, xmm2
140c912b3: 660f15e2 unpckhpd xmm4, xmm2
140c912b7: f30f102d4546c901 movss xmm5, dword ptr [rip + 0x1c94645]
140c912bf: 0f2ed9 ucomiss xmm3, xmm1
140c912c2: 734a jae 0x140c9130e
140c912c4: 0f28da movaps xmm3, xmm2
140c912c7: f30f58dd addss xmm3, xmm5
140c912cb: f30f58cd addss xmm1, xmm5
140c912cf: 0f57ed xorps xmm5, xmm5
140c912d2: 0f2eeb ucomiss xmm5, xmm3
140c912d5: 0f8380000000 jae 0x140c9135b
140c912db: f30f58cb addss xmm1, xmm3
140c912df: 0f57ed xorps xmm5, xmm5
140c912e2: f30f51e9 sqrtss xmm5, xmm1
140c912e6: f30f101d8245c901 movss xmm3, dword ptr [rip + 0x1c94582]
140c912ee: f30f5edd divss xmm3, xmm5
140c912f2: f30f5cc4 subss xmm0, xmm4
140c912f6: f30f7ec0 movq xmm0, xmm0
140c912fa: 0fc6cad4 shufps xmm1, xmm2, 0xd4
140c912fe: 0fc6c122 shufps xmm0, xmm1, 0x22
140c91302: 0fc6db00 shufps xmm3, xmm3, 0
140c91306: 0f59d8 mulps xmm3, xmm0
140c91309: e9b7000000 jmp 0x140c913c5
140c9130e: 0f28f5 movaps xmm6, xmm5
140c91311: f30f5cf2 subss xmm6, xmm2
140c91315: f30f5ce9 subss xmm5, xmm1
140c91319: 0f57c9 xorps xmm1, xmm1
140c9131c: 0f2ece ucomiss xmm1, xmm6
140c9131f: 736a jae 0x140c9138b
140c91321: f30f16da movshdup xmm3, xmm2
140c91325: f30f58ee addss xmm5, xmm6
140c91329: 0f57d2 xorps xmm2, xmm2
140c9132c: f30f51d5 sqrtss xmm2, xmm5
140c91330: f30f10353845c901 movss xmm6, dword ptr [rip + 0x1c94538]
140c91338: f30f5ef2 divss xmm6, xmm2
140c9133c: f30f58d9 addss xmm3, xmm1
140c91340: f30f5cc4 subss xmm0, xmm4
140c91344: 0fc6f600 shufps xmm6, xmm6, 0
140c91348: 0f14dd unpcklps xmm3, xmm5
140c9134b: 0f57c9 xorps xmm1, xmm1
140c9134e: f30f10c8 movss xmm1, xmm0
140c91352: 0fc6d914 shufps xmm3, xmm1, 0x14
140c91356: 0f59de mulps xmm3, xmm6
140c91359: eb6a jmp 0x140c913c5
140c9135b: f30f5ccb subss xmm1, xmm3
140c9135f: 0f57ed xorps xmm5, xmm5
140c91362: f30f51e9 sqrtss xmm5, xmm1
140c91366: f30f101d0245c901 movss xmm3, dword ptr [rip + 0x1c94502]
140c9136e: f30f5edd divss xmm3, xmm5
140c91372: f30f58c4 addss xmm0, xmm4
140c91376: 0f57e4 xorps xmm4, xmm4
140c91379: f30f10e0 movss xmm4, xmm0
140c9137d: 0fc6cad4 shufps xmm1, xmm2, 0xd4
140c91381: 0fc6e184 shufps xmm4, xmm1, 0x84
140c91385: 0fc6db00 shufps xmm3, xmm3, 0
140c91389: eb37 jmp 0x140c913c2
140c9138b: f30f5cee subss xmm5, xmm6
140c9138f: 0f57c9 xorps xmm1, xmm1
140c91392: f30f51cd sqrtss xmm1, xmm5
140c91396: f30f1025d244c901 movss xmm4, dword ptr [rip + 0x1c944d2]
140c9139e: f30f5ee1 divss xmm4, xmm1
140c913a2: 0f57db xorps xmm3, xmm3
140c913a5: 0f57c9 xorps xmm1, xmm1
140c913a8: 0f14c8 unpcklps xmm1, xmm0
140c913ab: 0fc6d2e9 shufps xmm2, xmm2, 0xe9
140c913af: 0f58d1 addps xmm2, xmm1
140c913b2: 0fc6e400 shufps xmm4, xmm4, 0
140c913b6: f30f10dd movss xmm3, xmm5
140c913ba: 0fc6da4c shufps xmm3, xmm2, 0x4c
140c913be: 0fc6db78 shufps xmm3, xmm3, 0x78
140c913c2: 0f59dc mulps xmm3, xmm4
140c913c5: 0f2918 movaps xmmword ptr [rax], xmm3
140c913c8: 0f283424 movaps xmm6, xmmword ptr [rsp]
140c913cc: 4883c418 add rsp, 0x18
140c913d0: c3 ret 
