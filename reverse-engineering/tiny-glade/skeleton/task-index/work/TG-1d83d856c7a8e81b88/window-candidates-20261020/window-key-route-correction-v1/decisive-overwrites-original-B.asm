1407ee82b: 48c1e104 shl rcx, 4
1407ee82f: 66c704080200 mov word ptr [rax + rcx], 2
1407ee835: 89740804 mov dword ptr [rax + rcx + 4], esi
1407ee839: 440f11640810 movups xmmword ptr [rax + rcx + 0x10], xmm12
1407ee83f: 0f13440820 movlps qword ptr [rax + rcx + 0x20], xmm0
1407ee844: f30f11540828 movss dword ptr [rax + rcx + 0x28], xmm2
1407ee84a: 488d9520010000 lea rdx, [rbp + 0x120]
1407ee851: f30f6f420c movdqu xmm0, xmmword ptr [rdx + 0xc]
1407ee856: f30f7f44082c movdqu xmmword ptr [rax + rcx + 0x2c], xmm0
1407ee85c: 8b521c mov edx, dword ptr [rdx + 0x1c]
1407ee85f: 8954083c mov dword ptr [rax + rcx + 0x3c], edx
1407ee863: 488b9580020000 mov rdx, qword ptr [rbp + 0x280]
1407ee86a: 4889540840 mov qword ptr [rax + rcx + 0x40], rdx
1407ee86f: 488b95f8010000 mov rdx, qword ptr [rbp + 0x1f8]
1407ee876: 4889540848 mov qword ptr [rax + rcx + 0x48], rdx
1407ee87b: 48ffc7 inc rdi
1407ee87e: 4889bb08010000 mov qword ptr [rbx + 0x108], rdi
1407ee885: f3440f1085d0000000 movss xmm8, dword ptr [rbp + 0xd0]
1407ee88e: f3440f5905058f2902 mulss xmm8, dword ptr [rip + 0x2298f05]
1407ee897: 488b8578020000 mov rax, qword ptr [rbp + 0x278]
1407ee89e: 898510020000 mov dword ptr [rbp + 0x210], eax
1407ee8a4: 488d057dc02902 lea rax, [rip + 0x229c07d]
1407ee8ab: 48898518020000 mov qword ptr [rbp + 0x218], rax
1407ee8b2: 48c785200200000c000000 mov qword ptr [rbp + 0x220], 0xc
1407ee8bd: 488d8d10020000 lea rcx, [rbp + 0x210]
1407ee8c4: e867e3a900 call 0x14128cc30 ; utils::calculate_hash<tuple$<country_core::resources::walls::decorator::DecoratorId,ref$<str$> > >
1407ee8c9: 48898578020000 mov qword ptr [rbp + 0x278], rax
1407ee8d0: 488b8598020000 mov rax, qword ptr [rbp + 0x298]
1407ee8d7: 8b70e0 mov esi, dword ptr [rax - 0x20]
1407ee8da: 488b85f0010000 mov rax, qword ptr [rbp + 0x1f0]
1407ee8e1: 48898580020000 mov qword ptr [rbp + 0x280], rax
