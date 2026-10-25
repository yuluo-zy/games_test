140c90fd0: 4883ec18 sub rsp, 0x18
140c90fd4: 0f293424 movaps xmmword ptr [rsp], xmm6
140c90fd8: 4889c8 mov rax, rcx
140c90fdb: 0f28c1 movaps xmm0, xmm1
140c90fde: f30f59c1 mulss xmm0, xmm1
140c90fe2: 0f28da movaps xmm3, xmm2
140c90fe5: f30f59da mulss xmm3, xmm2
140c90fe9: f30f58d8 addss xmm3, xmm0
140c90fed: f30f51db sqrtss xmm3, xmm3
140c90ff1: f30f10050b49c901 movss xmm0, dword ptr [rip + 0x1c9490b]
140c90ff9: 0f28e0 movaps xmm4, xmm0
140c90ffc: f30f5ee3 divss xmm4, xmm3
140c91000: 660f7ee1 movd ecx, xmm4
140c91004: 85c9 test ecx, ecx
140c91006: 0f99c2 setns dl
140c91009: 4189c8 mov r8d, ecx
140c9100c: 4181e0ffffff7f and r8d, 0x7fffffff
140c91013: 4181c0000080ff add r8d, 0xff800000
140c9101a: 4181f80000007f cmp r8d, 0x7f000000
140c91021: 410f92c0 setb r8b
140c91025: 4120d0 and r8b, dl
140c91028: ffc9 dec ecx
140c9102a: 81f9ffff7f00 cmp ecx, 0x7fffff
140c91030: 0f92c1 setb cl
140c91033: 4408c1 or cl, r8b
140c91036: 0f57db xorps xmm3, xmm3
140c91039: 7505 jne 0x140c91040
140c9103b: 0f57d2 xorps xmm2, xmm2
140c9103e: eb0b jmp 0x140c9104b
140c91040: f30f59cc mulss xmm1, xmm4
140c91044: f30f59d4 mulss xmm2, xmm4
140c91048: 0f28c1 movaps xmm0, xmm1
140c9104b: 0f57e4 xorps xmm4, xmm4
140c9104e: 0f57c9 xorps xmm1, xmm1
140c91051: f30f10ca movss xmm1, xmm2
140c91055: f30f10e0 movss xmm4, xmm0
140c91059: 0f28ec movaps xmm5, xmm4
140c9105c: 0f16e9 movlhps xmm5, xmm1
140c9105f: 0fc6e204 shufps xmm4, xmm2, 4
140c91063: 0f592556c0d301 mulps xmm4, xmmword ptr [rip + 0x1d3c056]
140c9106a: f30f100d9248c901 movss xmm1, dword ptr [rip + 0x1c94892]
140c91072: 0fc6edd2 shufps xmm5, xmm5, 0xd2
140c91076: 0fc6c9d2 shufps xmm1, xmm1, 0xd2
140c9107a: 0f59cd mulps xmm1, xmm5
140c9107d: 0f5ce1 subps xmm4, xmm1
140c91080: 0f28cc movaps xmm1, xmm4
140c91083: 0fc6ccd2 shufps xmm1, xmm4, 0xd2
140c91087: 0f59e4 mulps xmm4, xmm4
140c9108a: 0f28ec movaps xmm5, xmm4
140c9108d: 660f15ec unpckhpd xmm5, xmm4
140c91091: f30f58ec addss xmm5, xmm4
140c91095: f30f16e4 movshdup xmm4, xmm4
140c91099: f30f58e5 addss xmm4, xmm5
140c9109d: 0fc6e400 shufps xmm4, xmm4, 0
140c910a1: 0f51e4 sqrtps xmm4, xmm4
140c910a4: 0f5ecc divps xmm1, xmm4
140c910a7: 0f28e1 movaps xmm4, xmm1
140c910aa: 660f15e1 unpckhpd xmm4, xmm1
140c910ae: f30f102d4e48c901 movss xmm5, dword ptr [rip + 0x1c9484e]
140c910b6: 0f2eda ucomiss xmm3, xmm2
140c910b9: 734a jae 0x140c91105
140c910bb: 0f28d9 movaps xmm3, xmm1
140c910be: f30f58dd addss xmm3, xmm5
140c910c2: f30f58d5 addss xmm2, xmm5
140c910c6: 0f57ed xorps xmm5, xmm5
140c910c9: 0f2eeb ucomiss xmm5, xmm3
140c910cc: 0f8380000000 jae 0x140c91152
140c910d2: f30f58d3 addss xmm2, xmm3
140c910d6: 0f57ed xorps xmm5, xmm5
140c910d9: f30f51ea sqrtss xmm5, xmm2
140c910dd: f30f101d8b47c901 movss xmm3, dword ptr [rip + 0x1c9478b]
140c910e5: f30f5edd divss xmm3, xmm5
140c910e9: f30f5cc4 subss xmm0, xmm4
140c910ed: f30f7ec0 movq xmm0, xmm0
140c910f1: 0fc6d1d4 shufps xmm2, xmm1, 0xd4
140c910f5: 0fc6c222 shufps xmm0, xmm2, 0x22
140c910f9: 0fc6db00 shufps xmm3, xmm3, 0
140c910fd: 0f59d8 mulps xmm3, xmm0
140c91100: e9b7000000 jmp 0x140c911bc
140c91105: 0f28f5 movaps xmm6, xmm5
140c91108: f30f5cf1 subss xmm6, xmm1
140c9110c: f30f5cea subss xmm5, xmm2
140c91110: 0f57d2 xorps xmm2, xmm2
140c91113: 0f2ed6 ucomiss xmm2, xmm6
140c91116: 736a jae 0x140c91182
140c91118: f30f16d9 movshdup xmm3, xmm1
140c9111c: f30f58ee addss xmm5, xmm6
140c91120: 0f57c9 xorps xmm1, xmm1
140c91123: f30f51cd sqrtss xmm1, xmm5
140c91127: f30f10354147c901 movss xmm6, dword ptr [rip + 0x1c94741]
140c9112f: f30f5ef1 divss xmm6, xmm1
140c91133: f30f58da addss xmm3, xmm2
140c91137: f30f5cc4 subss xmm0, xmm4
140c9113b: 0fc6f600 shufps xmm6, xmm6, 0
140c9113f: 0f14dd unpcklps xmm3, xmm5
140c91142: 0f57c9 xorps xmm1, xmm1
140c91145: f30f10c8 movss xmm1, xmm0
140c91149: 0fc6d914 shufps xmm3, xmm1, 0x14
140c9114d: 0f59de mulps xmm3, xmm6
140c91150: eb6a jmp 0x140c911bc
140c91152: f30f5cd3 subss xmm2, xmm3
140c91156: 0f57ed xorps xmm5, xmm5
140c91159: f30f51ea sqrtss xmm5, xmm2
140c9115d: f30f101d0b47c901 movss xmm3, dword ptr [rip + 0x1c9470b]
140c91165: f30f5edd divss xmm3, xmm5
140c91169: f30f58c4 addss xmm0, xmm4
140c9116d: 0f57e4 xorps xmm4, xmm4
140c91170: f30f10e0 movss xmm4, xmm0
140c91174: 0fc6d1d4 shufps xmm2, xmm1, 0xd4
140c91178: 0fc6e284 shufps xmm4, xmm2, 0x84
140c9117c: 0fc6db00 shufps xmm3, xmm3, 0
140c91180: eb37 jmp 0x140c911b9
140c91182: f30f5cee subss xmm5, xmm6
140c91186: 0f57d2 xorps xmm2, xmm2
140c91189: f30f51d5 sqrtss xmm2, xmm5
140c9118d: f30f1025db46c901 movss xmm4, dword ptr [rip + 0x1c946db]
140c91195: f30f5ee2 divss xmm4, xmm2
140c91199: 0f57db xorps xmm3, xmm3
140c9119c: 0f57d2 xorps xmm2, xmm2
140c9119f: 0f14d0 unpcklps xmm2, xmm0
140c911a2: 0fc6c9e9 shufps xmm1, xmm1, 0xe9
140c911a6: 0f58ca addps xmm1, xmm2
140c911a9: 0fc6e400 shufps xmm4, xmm4, 0
140c911ad: f30f10dd movss xmm3, xmm5
140c911b1: 0fc6d94c shufps xmm3, xmm1, 0x4c
140c911b5: 0fc6db78 shufps xmm3, xmm3, 0x78
140c911b9: 0f59dc mulps xmm3, xmm4
140c911bc: 0f2918 movaps xmmword ptr [rax], xmm3
140c911bf: 0f283424 movaps xmm6, xmmword ptr [rsp]
140c911c3: 4883c418 add rsp, 0x18
140c911c7: c3 ret 
