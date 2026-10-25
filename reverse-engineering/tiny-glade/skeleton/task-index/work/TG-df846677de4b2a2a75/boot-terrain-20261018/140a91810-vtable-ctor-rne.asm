140a91810: push rbp
140a91811: push rsi
140a91812: push rdi
140a91813: sub rsp, 0x1d0
140a9181a: lea rbp, [rsp + 0x80]
140a91822: mov qword ptr [rbp + 0x148], 0xfffffffffffffffe
140a9182d: mov rsi, rcx
140a91830: xorps xmm0, xmm0
140a91833: movups xmmword ptr [rbp - 0x40], xmm0
140a91837: movups xmmword ptr [rbp - 0x28], xmm0
140a9183b: movups xmmword ptr [rbp - 0x10], xmm0
140a9183f: movups xmmword ptr [rbp + 8], xmm0
140a91843: movups xmmword ptr [rbp + 0x20], xmm0
140a91847: mov dword ptr [rbp + 0x30], 0
140a9184e: movups xmmword ptr [rbp + 0x58], xmm0
140a91852: movups xmmword ptr [rbp + 0x70], xmm0
140a91856: movups xmmword ptr [rbp + 0x88], xmm0
140a9185d: movups xmmword ptr [rbp + 0xa0], xmm0
140a91864: movups xmmword ptr [rbp + 0xb8], xmm0
140a9186b: mov dword ptr [rbp + 0xc8], 0
140a91875: mov qword ptr [rbp - 0x60], 0
140a9187d: mov qword ptr [rbp - 0x58], 8
140a91885: mov qword ptr [rbp - 0x50], 0
140a9188d: mov qword ptr [rbp - 0x48], 0x10
140a91895: mov qword ptr [rbp - 0x30], 0x10
140a9189d: mov qword ptr [rbp - 0x18], 0x10
140a918a5: mov qword ptr [rbp], 0x10
140a918ad: mov qword ptr [rbp + 0x18], 0x10
140a918b5: movabs rax, 0x8000000000000000
140a918bf: mov qword ptr [rbp + 0x38], rax
140a918c3: lea rcx, [rip + 0x2048896]
140a918ca: mov qword ptr [rbp + 0x40], rcx
140a918ce: mov qword ptr [rbp + 0x48], 0x33
140a918d6: mov qword ptr [rbp + 0x50], 0x10
140a918de: mov qword ptr [rbp + 0x68], 0x10
140a918e6: mov qword ptr [rbp + 0x80], 0x10
140a918f1: mov qword ptr [rbp + 0x98], 0x10
140a918fc: mov qword ptr [rbp + 0xb0], 0x10
140a91907: mov dword ptr [rbp + 0xd0], 0
140a91911: mov word ptr [rbp + 0xd4], 1
140a9191a: mov qword ptr [rbp + 0x100], rax
140a91921: mov dword ptr [rbp + 0x140], 0
140a9192b: movzx eax, byte ptr [rip + 0x2ba32d7]
140a91932: mov ecx, 0x1a8
140a91937: mov edx, 8
140a9193c: call 0x140613c10
140a91941: test rax, rax
140a91944: je 0x140a9197b
140a91946: mov rdi, rax
140a91949: lea rdx, [rbp - 0x60]
140a9194d: mov r8d, 0x1a8
140a91953: mov rcx, rax
140a91956: call 0x142923c00
140a9195b: lea r8, [rip + 0x20484ae]
140a91962: mov rcx, rsi
140a91965: mov rdx, rdi
140a91968: call 0x140ce4b10
140a9196d: mov rax, rsi
140a91970: add rsp, 0x1d0
140a91977: pop rdi
140a91978: pop rsi
140a91979: pop rbp
140a9197a: ret 
140a9197b: mov ecx, 8
140a91980: mov edx, 0x1a8
140a91985: call 0x1428d9000
140a9198a: ud2 
140a9198c: nop dword ptr [rax]
140a91990: mov qword ptr [rsp + 0x10], rdx
140a91995: push rbp
140a91996: push rsi
140a91997: push rdi
140a91998: sub rsp, 0x20
140a9199c: lea rbp, [rdx + 0x80]
140a919a3: lea rcx, [rbp - 0x60]
140a919a7: call 0x1405fb160
140a919ac: nop 
140a919ad: add rsp, 0x20
140a919b1: pop rdi
140a919b2: pop rsi
140a919b3: pop rbp
140a919b4: ret 
