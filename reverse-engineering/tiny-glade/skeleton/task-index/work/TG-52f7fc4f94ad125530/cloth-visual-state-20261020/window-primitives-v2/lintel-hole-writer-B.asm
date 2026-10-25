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
