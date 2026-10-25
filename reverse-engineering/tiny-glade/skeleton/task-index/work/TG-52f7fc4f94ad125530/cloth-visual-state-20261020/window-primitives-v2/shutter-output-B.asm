1407ee98b: 488d0cbf lea rcx, [rdi + rdi*4]
1407ee98f: 48c1e104 shl rcx, 4
1407ee993: 66c704080201 mov word ptr [rax + rcx], 0x102
1407ee999: 89740804 mov dword ptr [rax + rcx + 4], esi
1407ee99d: 440f11640810 movups xmmword ptr [rax + rcx + 0x10], xmm12
1407ee9a3: 0f13440820 movlps qword ptr [rax + rcx + 0x20], xmm0
1407ee9a8: f30f114c0828 movss dword ptr [rax + rcx + 0x28], xmm1
1407ee9ae: 488d9520010000 lea rdx, [rbp + 0x120]
1407ee9b5: f30f6f420c movdqu xmm0, xmmword ptr [rdx + 0xc]
1407ee9ba: f30f7f44082c movdqu xmmword ptr [rax + rcx + 0x2c], xmm0
1407ee9c0: 8b521c mov edx, dword ptr [rdx + 0x1c]
1407ee9c3: 8954083c mov dword ptr [rax + rcx + 0x3c], edx
1407ee9c7: 488b9578020000 mov rdx, qword ptr [rbp + 0x278]
1407ee9ce: 4889540840 mov qword ptr [rax + rcx + 0x40], rdx
1407ee9d3: 488b9580020000 mov rdx, qword ptr [rbp + 0x280]
1407ee9da: 4889540848 mov qword ptr [rax + rcx + 0x48], rdx
1407ee9df: 48ffc7 inc rdi
1407ee9e2: 4989b808010000 mov qword ptr [r8 + 0x108], rdi
1407ee9e9: e93cdaffff jmp 0x1407ec42a ; unknown
