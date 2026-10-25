1407ef990: 4157 push r15
1407ef992: 4156 push r14
1407ef994: 4155 push r13
1407ef996: 4154 push r12
1407ef998: 56 push rsi
1407ef999: 57 push rdi
1407ef99a: 55 push rbp
1407ef99b: 53 push rbx
1407ef99c: 4881ecd8010000 sub rsp, 0x1d8
1407ef9a3: 440f29bc24c0010000 movaps xmmword ptr [rsp + 0x1c0], xmm15
1407ef9ac: 440f29b424b0010000 movaps xmmword ptr [rsp + 0x1b0], xmm14
1407ef9b5: 440f29ac24a0010000 movaps xmmword ptr [rsp + 0x1a0], xmm13
1407ef9be: 440f29a42490010000 movaps xmmword ptr [rsp + 0x190], xmm12
1407ef9c7: 440f299c2480010000 movaps xmmword ptr [rsp + 0x180], xmm11
1407ef9d0: 440f29942470010000 movaps xmmword ptr [rsp + 0x170], xmm10
1407ef9d9: 440f298c2460010000 movaps xmmword ptr [rsp + 0x160], xmm9
1407ef9e2: 440f29842450010000 movaps xmmword ptr [rsp + 0x150], xmm8
1407ef9eb: 0f29bc2440010000 movaps xmmword ptr [rsp + 0x140], xmm7
1407ef9f3: 0f29b42430010000 movaps xmmword ptr [rsp + 0x130], xmm6
1407ef9fb: 4c89ce mov rsi, r9
1407ef9fe: 4c89c7 mov rdi, r8
1407efa01: 4889d3 mov rbx, rdx
1407efa04: 89cd mov ebp, ecx
1407efa06: 4c8bb42458020000 mov r14, qword ptr [rsp + 0x258]
1407efa0e: f30f10b42468020000 movss xmm6, dword ptr [rsp + 0x268]
1407efa17: 488d1572b22902 lea rdx, [rip + 0x229b272]
1407efa1e: 4889d9 mov rcx, rbx
1407efa21: e83a380400 call 0x140833260
1407efa26: f30f1000 movss xmm0, dword ptr [rax]
1407efa2a: f30f104804 movss xmm1, dword ptr [rax + 4]
1407efa2f: f30f583509a52902 addss xmm6, dword ptr [rip + 0x229a509]
1407efa37: f30f5935315e1302 mulss xmm6, dword ptr [rip + 0x2135e31]
1407efa3f: 0f28d6 movaps xmm2, xmm6
1407efa42: e8892b4a00 call 0x140c925d0
1407efa47: 0f28f8 movaps xmm7, xmm0
1407efa4a: 440f28c1 movaps xmm8, xmm1
1407efa4e: 0fb65b44 movzx ebx, byte ptr [rbx + 0x44]
1407efa52: 498b16 mov rdx, qword ptr [r14]
1407efa55: 89e9 mov ecx, ebp
1407efa57: e844410400 call 0x140833ba0
1407efa5c: 4d8b7e08 mov r15, qword ptr [r14 + 8]
1407efa60: 89d9 mov ecx, ebx
1407efa62: 89c2 mov edx, eax
1407efa64: e827623300 call 0x140b25c90
1407efa69: 88842498000000 mov byte ptr [rsp + 0x98], al
1407efa70: 88942499000000 mov byte ptr [rsp + 0x99], dl
1407efa77: 4c8db42498000000 lea r14, [rsp + 0x98]
1407efa7f: 4c89f9 mov rcx, r15
1407efa82: 4c89f2 mov rdx, r14
1407efa85: e806d5a600 call 0x14125cf90
1407efa8a: 4885c0 test rax, rax
1407efa8d: 0f84db080000 je 0x1407f036e
1407efa93: f3440f104830 movss xmm9, dword ptr [rax + 0x30]
1407efa99: f3440f580da2a42902 addss xmm9, dword ptr [rip + 0x229a4a2]
1407efaa2: 4c8ba42450020000 mov r12, qword ptr [rsp + 0x250]
1407efaaa: 4d8bb424d8000000 mov r14, qword ptr [r12 + 0xd8]
1407efab2: 4d3bb424c8000000 cmp r14, qword ptr [r12 + 0xc8]
1407efaba: 7514 jne 0x1407efad0
1407efabc: 498d8c24c8000000 lea rcx, [r12 + 0xc8]
1407efac4: 488d15ddb12902 lea rdx, [rip + 0x229b1dd]
1407efacb: e8303bd7ff call 0x140563600
1407efad0: 498b8424d0000000 mov rax, qword ptr [r12 + 0xd0]
1407efad8: 4b8d0cb6 lea rcx, [r14 + r14*4]
1407efadc: f30f113c88 movss dword ptr [rax + rcx*4], xmm7
1407efae1: f3440f11448804 movss dword ptr [rax + rcx*4 + 4], xmm8
1407efae8: f3440f114c8808 movss dword ptr [rax + rcx*4 + 8], xmm9
1407efaef: c744880ccdcc4c3e mov dword ptr [rax + rcx*4 + 0xc], 0x3e4ccccd
1407efaf7: c644881000 mov byte ptr [rax + rcx*4 + 0x10], 0
1407efafc: 49ffc6 inc r14
1407efaff: 4d89b424d8000000 mov qword ptr [r12 + 0xd8], r14
1407efb07: 4885f6 test rsi, rsi
1407efb0a: 0f84eb030000 je 0x1407efefb
1407efb10: f30f10bc2478020000 movss xmm7, dword ptr [rsp + 0x278]
1407efb19: 448b8c2470020000 mov r9d, dword ptr [rsp + 0x270]
1407efb21: 440f280f movaps xmm9, xmmword ptr [rdi]
1407efb25: f3440f104718 movss xmm8, dword ptr [rdi + 0x18]
1407efb2b: f30f104714 movss xmm0, dword ptr [rdi + 0x14]
1407efb30: f30f58c6 addss xmm0, xmm6
1407efb34: f3440f105710 movss xmm10, dword ptr [rdi + 0x10]
1407efb3a: 440f14d0 unpcklps xmm10, xmm0
1407efb3e: 80fb02 cmp bl, 2
1407efb41: 0f82b1000000 jb 0x1407efbf8
1407efb47: 410f28c9 movaps xmm1, xmm9
1407efb4b: 410f59c9 mulps xmm1, xmm9
1407efb4f: f30f16c1 movshdup xmm0, xmm1
1407efb53: f30f58c1 addss xmm0, xmm1
1407efb57: 0f28d1 movaps xmm2, xmm1
1407efb5a: 660f15d1 unpckhpd xmm2, xmm1
1407efb5e: f30f58d0 addss xmm2, xmm0
1407efb62: 0fc6d200 shufps xmm2, xmm2, 0
1407efb66: 0fc6c9ff shufps xmm1, xmm1, 0xff
1407efb6a: 0f5cca subps xmm1, xmm2
1407efb6d: f30f1005d3a32902 movss xmm0, dword ptr [rip + 0x229a3d3]
1407efb75: 0f59c8 mulps xmm1, xmm0
1407efb78: 410f59c1 mulps xmm0, xmm9
1407efb7c: f30f16d0 movshdup xmm2, xmm0
1407efb80: f30f58d0 addss xmm2, xmm0
1407efb84: 0f12c0 movhlps xmm0, xmm0
1407efb87: f30f58c2 addss xmm0, xmm2
1407efb8b: f30f58c0 addss xmm0, xmm0
1407efb8f: 0fc6c000 shufps xmm0, xmm0, 0
1407efb93: 410f59c1 mulps xmm0, xmm9
1407efb97: 0f58c1 addps xmm0, xmm1
1407efb9a: 410f28c9 movaps xmm1, xmm9
1407efb9e: 410fc6c9d2 shufps xmm1, xmm9, 0xd2
1407efba3: 0f590da6a32902 mulps xmm1, xmmword ptr [rip + 0x229a3a6]
1407efbaa: 410f28d1 movaps xmm2, xmm9
1407efbae: 410fc6d1c9 shufps xmm2, xmm9, 0xc9
1407efbb3: 0f5915a6a32902 mulps xmm2, xmmword ptr [rip + 0x229a3a6]
1407efbba: 0f5cd1 subps xmm2, xmm1
1407efbbd: 410f28c9 movaps xmm1, xmm9
1407efbc1: 410f58c9 addps xmm1, xmm9
1407efbc5: 0fc6c9ff shufps xmm1, xmm1, 0xff
1407efbc9: 0f59ca mulps xmm1, xmm2
1407efbcc: 0f58c8 addps xmm1, xmm0
1407efbcf: 440f5cd1 subps xmm10, xmm1
1407efbd3: 0f12c9 movhlps xmm1, xmm1
1407efbd6: f3440f5cc1 subss xmm8, xmm1
1407efbdb: f3440f101d8ca32902 movss xmm11, dword ptr [rip + 0x229a38c]
1407efbe4: f30f5e3da0cc2802 divss xmm7, dword ptr [rip + 0x228cca0]
1407efbec: 488d4720 lea rax, [rdi + 0x20]
1407efbf0: 41f6c101 test r9b, 1
1407efbf4: 751a jne 0x1407efc10
1407efbf6: eb1d jmp 0x1407efc15
1407efbf8: f3440f105f1c movss xmm11, dword ptr [rdi + 0x1c]
1407efbfe: f30f5e3d86cc2802 divss xmm7, dword ptr [rip + 0x228cc86]
1407efc06: 488d4720 lea rax, [rdi + 0x20]
1407efc0a: 41f6c101 test r9b, 1
1407efc0e: 7405 je 0x1407efc15
1407efc10: f3440f59df mulss xmm11, xmm7
1407efc15: 440fb6942440020000 movzx r10d, byte ptr [rsp + 0x240]
1407efc1e: 448b9c2448020000 mov r11d, dword ptr [rsp + 0x248]
1407efc26: 498d8c24b0000000 lea rcx, [r12 + 0xb0]
1407efc2e: 48894c2468 mov qword ptr [rsp + 0x68], rcx
1407efc33: 48c744245c00000000 mov qword ptr [rsp + 0x5c], 0
1407efc3c: c744246400000000 mov dword ptr [rsp + 0x64], 0
1407efc44: 488b4c2459 mov rcx, qword ptr [rsp + 0x59]
1407efc49: 48894c2448 mov qword ptr [rsp + 0x48], rcx
1407efc4e: 488b4c2460 mov rcx, qword ptr [rsp + 0x60]
1407efc53: 48894c244f mov qword ptr [rsp + 0x4f], rcx
1407efc58: 488b4c2448 mov rcx, qword ptr [rsp + 0x48]
1407efc5d: 488b54244f mov rdx, qword ptr [rsp + 0x4f]
1407efc62: 48894c2438 mov qword ptr [rsp + 0x38], rcx
1407efc67: 488954243f mov qword ptr [rsp + 0x3f], rdx
1407efc6c: 0f2800 movaps xmm0, xmmword ptr [rax]
1407efc6f: 0f298424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm0
1407efc77: 488b442438 mov rax, qword ptr [rsp + 0x38]
1407efc7c: 488b4c243f mov rcx, qword ptr [rsp + 0x3f]
1407efc81: 48894c242f mov qword ptr [rsp + 0x2f], rcx
1407efc86: 4889442428 mov qword ptr [rsp + 0x28], rax
1407efc8b: 4d8bb424c0000000 mov r14, qword ptr [r12 + 0xc0]
1407efc93: 4d3bb424b0000000 cmp r14, qword ptr [r12 + 0xb0]
1407efc9b: 752a jne 0x1407efcc7
1407efc9d: 488d1544b02902 lea rdx, [rip + 0x229b044]
1407efca4: 488b4c2468 mov rcx, qword ptr [rsp + 0x68]
1407efca9: e802b70300 call 0x14082b3b0
1407efcae: 448b9c2448020000 mov r11d, dword ptr [rsp + 0x248]
1407efcb6: 440fb6942440020000 movzx r10d, byte ptr [rsp + 0x240]
1407efcbf: 448b8c2470020000 mov r9d, dword ptr [rsp + 0x270]
1407efcc7: 498b8424b8000000 mov rax, qword ptr [r12 + 0xb8]
1407efccf: 4969ce60010000 imul rcx, r14, 0x160
1407efcd6: 48c7040800000000 mov qword ptr [rax + rcx], 0
1407efcde: 48c744081000000000 mov qword ptr [rax + rcx + 0x10], 0
1407efce7: 48c744082000000000 mov qword ptr [rax + rcx + 0x20], 0
1407efcf0: c744083000000000 mov dword ptr [rax + rcx + 0x30], 0
1407efcf8: 488b94248c000000 mov rdx, qword ptr [rsp + 0x8c]
1407efd00: 4889540834 mov qword ptr [rax + rcx + 0x34], rdx
1407efd05: 8b942494000000 mov edx, dword ptr [rsp + 0x94]
1407efd0c: 8954083c mov dword ptr [rax + rcx + 0x3c], edx
1407efd10: c744084000000000 mov dword ptr [rax + rcx + 0x40], 0
1407efd18: 0f108424d0000000 movups xmm0, xmmword ptr [rsp + 0xd0]
1407efd20: 0f108c24e0000000 movups xmm1, xmmword ptr [rsp + 0xe0]
1407efd28: 0f109424f0000000 movups xmm2, xmmword ptr [rsp + 0xf0]
1407efd30: 0f109c24fc000000 movups xmm3, xmmword ptr [rsp + 0xfc]
1407efd38: 0f11440844 movups xmmword ptr [rax + rcx + 0x44], xmm0
1407efd3d: 0f114c0854 movups xmmword ptr [rax + rcx + 0x54], xmm1
1407efd42: 0f11540864 movups xmmword ptr [rax + rcx + 0x64], xmm2
1407efd47: 0f115c0870 movups xmmword ptr [rax + rcx + 0x70], xmm3
1407efd4c: c784088000000000000000 mov dword ptr [rax + rcx + 0x80], 0
1407efd57: 488b942480000000 mov rdx, qword ptr [rsp + 0x80]
1407efd5f: 4889940884000000 mov qword ptr [rax + rcx + 0x84], rdx
1407efd67: 8b942488000000 mov edx, dword ptr [rsp + 0x88]
1407efd6e: 8994088c000000 mov dword ptr [rax + rcx + 0x8c], edx
1407efd75: c784089000000000000000 mov dword ptr [rax + rcx + 0x90], 0
1407efd80: 488b542474 mov rdx, qword ptr [rsp + 0x74]
1407efd85: 4889940894000000 mov qword ptr [rax + rcx + 0x94], rdx
1407efd8d: 8b54247c mov edx, dword ptr [rsp + 0x7c]
1407efd91: 8994089c000000 mov dword ptr [rax + rcx + 0x9c], edx
1407efd98: 488d1521af2902 lea rdx, [rip + 0x229af21]
1407efd9f: 48899408a0000000 mov qword ptr [rax + rcx + 0xa0], rdx
1407efda7: 48c78408a800000024000000 mov qword ptr [rax + rcx + 0xa8], 0x24
1407efdb3: 4c8d2d46a82902 lea r13, [rip + 0x229a846]
1407efdba: 4c89ac08b0000000 mov qword ptr [rax + rcx + 0xb0], r13
1407efdc2: 48c78408b80000001e000000 mov qword ptr [rax + rcx + 0xb8], 0x1e
1407efdce: 440f298c08c0000000 movaps xmmword ptr [rax + rcx + 0xc0], xmm9
1407efdd7: 440f139408d0000000 movlps qword ptr [rax + rcx + 0xd0], xmm10
1407efde0: f3440f118408d8000000 movss dword ptr [rax + rcx + 0xd8], xmm8
1407efdea: f3440f119c08dc000000 movss dword ptr [rax + rcx + 0xdc], xmm11
1407efdf4: 0f288424c0000000 movaps xmm0, xmmword ptr [rsp + 0xc0]
1407efdfc: 0f298408e0000000 movaps xmmword ptr [rax + rcx + 0xe0], xmm0
1407efe04: c78408f000000001000000 mov dword ptr [rax + rcx + 0xf0], 1
1407efe0f: 44899c08f4000000 mov dword ptr [rax + rcx + 0xf4], r11d
1407efe17: c78408f800000000000000 mov dword ptr [rax + rcx + 0xf8], 0
1407efe22: 488b9424a8000000 mov rdx, qword ptr [rsp + 0xa8]
1407efe2a: 488994080c010000 mov qword ptr [rax + rcx + 0x10c], rdx
1407efe32: 0f10842498000000 movups xmm0, xmmword ptr [rsp + 0x98]
1407efe3a: 0f118408fc000000 movups xmmword ptr [rax + rcx + 0xfc], xmm0
1407efe42: c684081401000002 mov byte ptr [rax + rcx + 0x114], 2
1407efe4a: 0fb6542427 movzx edx, byte ptr [rsp + 0x27]
1407efe4f: 88940817010000 mov byte ptr [rax + rcx + 0x117], dl
1407efe56: 0fb7542425 movzx edx, word ptr [rsp + 0x25]
1407efe5b: 6689940815010000 mov word ptr [rax + rcx + 0x115], dx
1407efe63: c784081801000000000000 mov dword ptr [rax + rcx + 0x118], 0
1407efe6e: 0f10842410010000 movups xmm0, xmmword ptr [rsp + 0x110]
1407efe76: 0f108c2420010000 movups xmm1, xmmword ptr [rsp + 0x120]
1407efe7e: 0f118c082c010000 movups xmmword ptr [rax + rcx + 0x12c], xmm1
1407efe86: 0f1184081c010000 movups xmmword ptr [rax + rcx + 0x11c], xmm0
1407efe8e: c684083c01000002 mov byte ptr [rax + rcx + 0x13c], 2
1407efe96: 488b542428 mov rdx, qword ptr [rsp + 0x28]
1407efe9b: 4c8b44242f mov r8, qword ptr [rsp + 0x2f]
1407efea0: 4c89840844010000 mov qword ptr [rax + rcx + 0x144], r8
1407efea8: 488994083d010000 mov qword ptr [rax + rcx + 0x13d], rdx
1407efeb0: c684084c01000002 mov byte ptr [rax + rcx + 0x14c], 2
1407efeb8: 448894084e010000 mov byte ptr [rax + rcx + 0x14e], r10b
1407efec0: 66c784084f0100000009 mov word ptr [rax + rcx + 0x14f], 0x900
1407efeca: 488b9424b1000000 mov rdx, qword ptr [rsp + 0xb1]
1407efed2: 4c8b8424b8000000 mov r8, qword ptr [rsp + 0xb8]
1407efeda: 4889940851010000 mov qword ptr [rax + rcx + 0x151], rdx
1407efee2: 4c89840858010000 mov qword ptr [rax + rcx + 0x158], r8
1407efeea: 49ffc6 inc r14
1407efeed: 4d89b424c0000000 mov qword ptr [r12 + 0xc0], r14
1407efef5: 4883fe01 cmp rsi, 1
1407efef9: 756c jne 0x1407eff67
1407efefb: 0f28b42430010000 movaps xmm6, xmmword ptr [rsp + 0x130]
1407eff03: 0f28bc2440010000 movaps xmm7, xmmword ptr [rsp + 0x140]
1407eff0b: 440f28842450010000 movaps xmm8, xmmword ptr [rsp + 0x150]
1407eff14: 440f288c2460010000 movaps xmm9, xmmword ptr [rsp + 0x160]
1407eff1d: 440f28942470010000 movaps xmm10, xmmword ptr [rsp + 0x170]
1407eff26: 440f289c2480010000 movaps xmm11, xmmword ptr [rsp + 0x180]
1407eff2f: 440f28a42490010000 movaps xmm12, xmmword ptr [rsp + 0x190]
1407eff38: 440f28ac24a0010000 movaps xmm13, xmmword ptr [rsp + 0x1a0]
1407eff41: 440f28b424b0010000 movaps xmm14, xmmword ptr [rsp + 0x1b0]
1407eff4a: 440f28bc24c0010000 movaps xmm15, xmmword ptr [rsp + 0x1c0]
1407eff53: 4881c4d8010000 add rsp, 0x1d8
1407eff5a: 5b pop rbx
1407eff5b: 5d pop rbp
1407eff5c: 5f pop rdi
1407eff5d: 5e pop rsi
1407eff5e: 415c pop r12
1407eff60: 415d pop r13
1407eff62: 415e pop r14
1407eff64: 415f pop r15
1407eff66: c3 ret 
1407eff67: 4c8d3c76 lea r15, [rsi + rsi*2]
1407eff6b: 49c1e704 shl r15, 4
1407eff6f: 4901ff add r15, rdi
1407eff72: 4883c730 add rdi, 0x30
1407eff76: 4c8d74245c lea r14, [rsp + 0x5c]
1407eff7b: 4883c6fe add rsi, -2
1407eff7f: f3440f1015ec9f2902 movss xmm10, dword ptr [rip + 0x2299fec]
1407eff88: f3440f100db79f2902 movss xmm9, dword ptr [rip + 0x2299fb7]
1407eff91: f2440f101dc69f2902 movsd xmm11, qword ptr [rip + 0x2299fc6]
1407eff9a: f3440f1025cd9f2902 movss xmm12, dword ptr [rip + 0x2299fcd]
1407effa3: e93f020000 jmp 0x1407f01e7
1407effa8: 0f1f840000000000 nop dword ptr [rax + rax]
1407effb0: 498b8424b8000000 mov rax, qword ptr [r12 + 0xb8]
1407effb8: 4869cd60010000 imul rcx, rbp, 0x160
1407effbf: 48c7040800000000 mov qword ptr [rax + rcx], 0
1407effc7: 48c744081000000000 mov qword ptr [rax + rcx + 0x10], 0
1407effd0: 48c744082000000000 mov qword ptr [rax + rcx + 0x20], 0
1407effd9: c744083000000000 mov dword ptr [rax + rcx + 0x30], 0
1407effe1: 488b94248c000000 mov rdx, qword ptr [rsp + 0x8c]
1407effe9: 4889540834 mov qword ptr [rax + rcx + 0x34], rdx
1407effee: 8b942494000000 mov edx, dword ptr [rsp + 0x94]
1407efff5: 8954083c mov dword ptr [rax + rcx + 0x3c], edx
1407efff9: c744084000000000 mov dword ptr [rax + rcx + 0x40], 0
1407f0001: 0f108424d0000000 movups xmm0, xmmword ptr [rsp + 0xd0]
1407f0009: 0f108c24e0000000 movups xmm1, xmmword ptr [rsp + 0xe0]
1407f0011: 0f109424f0000000 movups xmm2, xmmword ptr [rsp + 0xf0]
1407f0019: 0f109c24fc000000 movups xmm3, xmmword ptr [rsp + 0xfc]
1407f0021: 0f11440844 movups xmmword ptr [rax + rcx + 0x44], xmm0
1407f0026: 0f114c0854 movups xmmword ptr [rax + rcx + 0x54], xmm1
1407f002b: 0f11540864 movups xmmword ptr [rax + rcx + 0x64], xmm2
1407f0030: 0f115c0870 movups xmmword ptr [rax + rcx + 0x70], xmm3
1407f0035: c784088000000000000000 mov dword ptr [rax + rcx + 0x80], 0
1407f0040: 488b942480000000 mov rdx, qword ptr [rsp + 0x80]
1407f0048: 4889940884000000 mov qword ptr [rax + rcx + 0x84], rdx
1407f0050: 8b942488000000 mov edx, dword ptr [rsp + 0x88]
1407f0057: 8994088c000000 mov dword ptr [rax + rcx + 0x8c], edx
1407f005e: c784089000000000000000 mov dword ptr [rax + rcx + 0x90], 0
1407f0069: 488b542474 mov rdx, qword ptr [rsp + 0x74]
1407f006e: 4889940894000000 mov qword ptr [rax + rcx + 0x94], rdx
1407f0076: 8b54247c mov edx, dword ptr [rsp + 0x7c]
1407f007a: 8994089c000000 mov dword ptr [rax + rcx + 0x9c], edx
1407f0081: 488d1538ac2902 lea rdx, [rip + 0x229ac38]
1407f0088: 48899408a0000000 mov qword ptr [rax + rcx + 0xa0], rdx
1407f0090: 48c78408a800000024000000 mov qword ptr [rax + rcx + 0xa8], 0x24
1407f009c: 4c89ac08b0000000 mov qword ptr [rax + rcx + 0xb0], r13
1407f00a4: 48c78408b80000001e000000 mov qword ptr [rax + rcx + 0xb8], 0x1e
1407f00b0: 440f29b408c0000000 movaps xmmword ptr [rax + rcx + 0xc0], xmm14
1407f00b9: 440f13bc08d0000000 movlps qword ptr [rax + rcx + 0xd0], xmm15
1407f00c2: f3440f11ac08d8000000 movss dword ptr [rax + rcx + 0xd8], xmm13
1407f00cc: f3440f118408dc000000 movss dword ptr [rax + rcx + 0xdc], xmm8
1407f00d6: 0f288424c0000000 movaps xmm0, xmmword ptr [rsp + 0xc0]
1407f00de: 0f298408e0000000 movaps xmmword ptr [rax + rcx + 0xe0], xmm0
1407f00e6: c78408f000000001000000 mov dword ptr [rax + rcx + 0xf0], 1
1407f00f1: 44899c08f4000000 mov dword ptr [rax + rcx + 0xf4], r11d
1407f00f9: c78408f800000000000000 mov dword ptr [rax + rcx + 0xf8], 0
1407f0104: 488b9424a8000000 mov rdx, qword ptr [rsp + 0xa8]
1407f010c: 488994080c010000 mov qword ptr [rax + rcx + 0x10c], rdx
1407f0114: 0f10842498000000 movups xmm0, xmmword ptr [rsp + 0x98]
1407f011c: 0f118408fc000000 movups xmmword ptr [rax + rcx + 0xfc], xmm0
1407f0124: c684081401000002 mov byte ptr [rax + rcx + 0x114], 2
1407f012c: 0fb6542427 movzx edx, byte ptr [rsp + 0x27]
1407f0131: 88940817010000 mov byte ptr [rax + rcx + 0x117], dl
1407f0138: 0fb7542425 movzx edx, word ptr [rsp + 0x25]
1407f013d: 6689940815010000 mov word ptr [rax + rcx + 0x115], dx
1407f0145: c784081801000000000000 mov dword ptr [rax + rcx + 0x118], 0
1407f0150: 0f10842410010000 movups xmm0, xmmword ptr [rsp + 0x110]
1407f0158: 0f108c2420010000 movups xmm1, xmmword ptr [rsp + 0x120]
1407f0160: 0f118c082c010000 movups xmmword ptr [rax + rcx + 0x12c], xmm1
1407f0168: 0f1184081c010000 movups xmmword ptr [rax + rcx + 0x11c], xmm0
1407f0170: c684083c01000002 mov byte ptr [rax + rcx + 0x13c], 2
1407f0178: 488b542428 mov rdx, qword ptr [rsp + 0x28]
1407f017d: 4c8b44242f mov r8, qword ptr [rsp + 0x2f]
1407f0182: 4c89840844010000 mov qword ptr [rax + rcx + 0x144], r8
1407f018a: 488994083d010000 mov qword ptr [rax + rcx + 0x13d], rdx
1407f0192: c684084c01000002 mov byte ptr [rax + rcx + 0x14c], 2
1407f019a: 448894084e010000 mov byte ptr [rax + rcx + 0x14e], r10b
1407f01a2: 66c784084f0100000009 mov word ptr [rax + rcx + 0x14f], 0x900
1407f01ac: 488b9424b1000000 mov rdx, qword ptr [rsp + 0xb1]
1407f01b4: 4c8b8424b8000000 mov r8, qword ptr [rsp + 0xb8]
1407f01bc: 4889940851010000 mov qword ptr [rax + rcx + 0x151], rdx
1407f01c4: 4c89840858010000 mov qword ptr [rax + rcx + 0x158], r8
1407f01cc: 48ffc5 inc rbp
1407f01cf: 4989ac24c0000000 mov qword ptr [r12 + 0xc0], rbp
1407f01d7: 48ffce dec rsi
1407f01da: 4883c730 add rdi, 0x30
1407f01de: 4c39ff cmp rdi, r15
1407f01e1: 0f8414fdffff je 0x1407efefb
1407f01e7: 440f2837 movaps xmm14, xmmword ptr [rdi]
1407f01eb: f3440f106f18 movss xmm13, dword ptr [rdi + 0x18]
1407f01f1: f30f104714 movss xmm0, dword ptr [rdi + 0x14]
1407f01f6: f30f58c6 addss xmm0, xmm6
1407f01fa: f3440f107f10 movss xmm15, dword ptr [rdi + 0x10]
1407f0200: 440f14f8 unpcklps xmm15, xmm0
1407f0204: 80fb01 cmp bl, 1
1407f0207: 7617 jbe 0x1407f0220
1407f0209: 4885f6 test rsi, rsi
1407f020c: 0f84bb000000 je 0x1407f02cd
1407f0212: 450f28c2 movaps xmm8, xmm10
1407f0216: 41f6c101 test r9b, 1
1407f021a: 7510 jne 0x1407f022c
1407f021c: eb13 jmp 0x1407f0231
1407f021e: 6690 nop 
1407f0220: f3440f10471c movss xmm8, dword ptr [rdi + 0x1c]
1407f0226: 41f6c101 test r9b, 1
1407f022a: 7405 je 0x1407f0231
1407f022c: f3440f59c7 mulss xmm8, xmm7
1407f0231: 41c7460800000000 mov dword ptr [r14 + 8], 0
1407f0239: 49c70600000000 mov qword ptr [r14], 0
1407f0240: 488b442459 mov rax, qword ptr [rsp + 0x59]
1407f0245: 488b4c2460 mov rcx, qword ptr [rsp + 0x60]
1407f024a: 48894c244f mov qword ptr [rsp + 0x4f], rcx
1407f024f: 4889442448 mov qword ptr [rsp + 0x48], rax
1407f0254: 488b442448 mov rax, qword ptr [rsp + 0x48]
1407f0259: 488b4c244f mov rcx, qword ptr [rsp + 0x4f]
1407f025e: 4889442438 mov qword ptr [rsp + 0x38], rax
1407f0263: 48894c243f mov qword ptr [rsp + 0x3f], rcx
1407f0268: 0f284720 movaps xmm0, xmmword ptr [rdi + 0x20]
1407f026c: 0f298424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm0
1407f0274: 488b442438 mov rax, qword ptr [rsp + 0x38]
1407f0279: 488b4c243f mov rcx, qword ptr [rsp + 0x3f]
1407f027e: 4889442428 mov qword ptr [rsp + 0x28], rax
1407f0283: 48894c242f mov qword ptr [rsp + 0x2f], rcx
1407f0288: 498bac24c0000000 mov rbp, qword ptr [r12 + 0xc0]
1407f0290: 493bac24b0000000 cmp rbp, qword ptr [r12 + 0xb0]
1407f0298: 0f8512fdffff jne 0x1407effb0
1407f029e: 488b4c2468 mov rcx, qword ptr [rsp + 0x68]
1407f02a3: 488d153eaa2902 lea rdx, [rip + 0x229aa3e]
1407f02aa: e801b10300 call 0x14082b3b0
1407f02af: 448b9c2448020000 mov r11d, dword ptr [rsp + 0x248]
1407f02b7: 440fb6942440020000 movzx r10d, byte ptr [rsp + 0x240]
1407f02c0: 448b8c2470020000 mov r9d, dword ptr [rsp + 0x270]
1407f02c8: e9e3fcffff jmp 0x1407effb0
1407f02cd: 410f28ce movaps xmm1, xmm14
1407f02d1: 410f59ce mulps xmm1, xmm14
1407f02d5: f30f16c1 movshdup xmm0, xmm1
1407f02d9: f30f58c1 addss xmm0, xmm1
1407f02dd: 0f28d1 movaps xmm2, xmm1
1407f02e0: 660f15d1 unpckhpd xmm2, xmm1
1407f02e4: f30f58d0 addss xmm2, xmm0
1407f02e8: 0fc6d200 shufps xmm2, xmm2, 0
1407f02ec: 0fc6c9ff shufps xmm1, xmm1, 0xff
1407f02f0: 0f5cca subps xmm1, xmm2
1407f02f3: 410f59c9 mulps xmm1, xmm9
1407f02f7: 410f28c6 movaps xmm0, xmm14
1407f02fb: 410f59c1 mulps xmm0, xmm9
1407f02ff: f30f16d0 movshdup xmm2, xmm0
1407f0303: f30f58d0 addss xmm2, xmm0
1407f0307: 0f12c0 movhlps xmm0, xmm0
1407f030a: f30f58c2 addss xmm0, xmm2
1407f030e: f30f58c0 addss xmm0, xmm0
1407f0312: 0fc6c000 shufps xmm0, xmm0, 0
1407f0316: 410f59c6 mulps xmm0, xmm14
1407f031a: 0f58c1 addps xmm0, xmm1
1407f031d: 410f28ce movaps xmm1, xmm14
1407f0321: 410fc6ced2 shufps xmm1, xmm14, 0xd2
1407f0326: 0f590d239c2902 mulps xmm1, xmmword ptr [rip + 0x2299c23]
1407f032d: 410f28d6 movaps xmm2, xmm14
1407f0331: 410fc6d6c9 shufps xmm2, xmm14, 0xc9
1407f0336: 410f59d3 mulps xmm2, xmm11
1407f033a: 0f5cd1 subps xmm2, xmm1
1407f033d: 410f28ce movaps xmm1, xmm14
1407f0341: 410f58ce addps xmm1, xmm14
1407f0345: 0fc6c9ff shufps xmm1, xmm1, 0xff
1407f0349: 0f59ca mulps xmm1, xmm2
1407f034c: 0f58c8 addps xmm1, xmm0
1407f034f: 440f58f9 addps xmm15, xmm1
1407f0353: 0f12c9 movhlps xmm1, xmm1
1407f0356: f3440f58e9 addss xmm13, xmm1
1407f035b: 450f28c4 movaps xmm8, xmm12
1407f035f: 41f6c101 test r9b, 1
1407f0363: 0f85c3feffff jne 0x1407f022c
1407f0369: e9c3feffff jmp 0x1407f0231
1407f036e: 4c89b42410010000 mov qword ptr [rsp + 0x110], r14
1407f0376: 488d0573cba600 lea rax, [rip + 0xa6cb73]
1407f037d: 4889842418010000 mov qword ptr [rsp + 0x118], rax
1407f0385: 488d05f4184302 lea rax, [rip + 0x24318f4]
1407f038c: 48898424d0000000 mov qword ptr [rsp + 0xd0], rax
1407f0394: 48c78424d800000001000000 mov qword ptr [rsp + 0xd8], 1
1407f03a0: 48c78424f000000000000000 mov qword ptr [rsp + 0xf0], 0
1407f03ac: 488d842410010000 lea rax, [rsp + 0x110]
1407f03b4: 48898424e0000000 mov qword ptr [rsp + 0xe0], rax
1407f03bc: 48c78424e800000001000000 mov qword ptr [rsp + 0xe8], 1
1407f03c8: 488d15c1184302 lea rdx, [rip + 0x24318c1]
1407f03cf: 488d8c24d0000000 lea rcx, [rsp + 0xd0]
1407f03d7: e8b48f0e02 call 0x1428d9390
1407f03dc: cc int3 
