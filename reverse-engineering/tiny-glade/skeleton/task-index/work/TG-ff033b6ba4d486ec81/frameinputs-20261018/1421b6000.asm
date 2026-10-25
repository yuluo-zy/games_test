1421b6000: push rbp
1421b6001: push r15
1421b6003: push r14
1421b6005: push r12
1421b6007: push rsi
1421b6008: push rdi
1421b6009: push rbx
1421b600a: sub rsp, 0x30
1421b600e: lea rbp, [rsp + 0x30]
1421b6013: mov qword ptr [rbp - 8], 0xfffffffffffffffe
1421b601b: mov rdi, rdx
1421b601e: mov rsi, rcx
1421b6021: mov ecx, dword ptr [rcx + 0x50]
1421b6024: mov rdx, qword ptr [rsi + 8]
1421b6028: cmp rdx, rcx
1421b602b: jbe 0x1421b623b
1421b6031: mov rdx, qword ptr [rsi]
1421b6034: movzx r8d, byte ptr [rdx + rcx]
1421b6039: cmp r8, 0x20
1421b603d: jae 0x1421b60b2
1421b603f: mov rax, rcx
1421b6042: shl rax, 5
1421b6046: or rax, r8
1421b6049: inc r8d
1421b604c: mov byte ptr [rdx + rcx], r8b
1421b6050: mov rdx, qword ptr [rsi + 0x30]
1421b6054: cmp rax, rdx
1421b6057: jae 0x1421b6247
1421b605d: mov rdx, qword ptr [rsi + 0x28]
1421b6061: shl rax, 6
1421b6065: movups xmm0, xmmword ptr [rdi]
1421b6068: movups xmm1, xmmword ptr [rdi + 0x10]
1421b606c: movups xmm2, xmmword ptr [rdi + 0x20]
1421b6070: movups xmm3, xmmword ptr [rdi + 0x30]
1421b6074: movups xmmword ptr [rdx + rax + 0x30], xmm3
1421b6079: movups xmmword ptr [rdx + rax + 0x20], xmm2
1421b607e: movups xmmword ptr [rdx + rax + 0x10], xmm1
1421b6083: movups xmmword ptr [rdx + rax], xmm0
1421b6087: mov rdx, qword ptr [rsi + 0x48]
1421b608b: mov eax, ecx
1421b608d: shr eax, 6
1421b6090: cmp rax, rdx
1421b6093: jae 0x1421b6256
1421b6099: mov rdx, qword ptr [rsi + 0x40]
1421b609d: mov r8d, 1
1421b60a3: shl r8, cl
1421b60a6: lock or qword ptr [rdx + rax*8], r8
1421b60ab: xor esi, esi
1421b60ad: jmp 0x1421b620b
1421b60b2: mov r14, qword ptr [rsi + 0x38]
1421b60b6: mov ecx, 8
1421b60bb: xor eax, eax
1421b60bd: lock cmpxchg qword ptr [r14], rcx
1421b60c2: jne 0x1421b621c
1421b60c8: mov rcx, r14
1421b60cb: call 0x140c81c70
1421b60d0: mov rbx, r14
1421b60d3: or rbx, 1
1421b60d7: mov rcx, rbx
1421b60da: call 0x140c81c70
1421b60df: mov rax, qword ptr [r14 + 0x18]
1421b60e3: test rax, rax
1421b60e6: je 0x1421b61ea
1421b60ec: lea rcx, [rax - 1]
1421b60f0: mov qword ptr [r14 + 0x18], rcx
1421b60f4: mov qword ptr [rbp - 0x10], r14
1421b60f8: mov rcx, qword ptr [r14 + 0x10]
1421b60fc: mov r14d, dword ptr [rcx + rax*4 - 4]
1421b6101: mov r15, qword ptr [rsi + 0x20]
1421b6105: mov r12, qword ptr [r15 + 0x10]
1421b6109: cmp r12, qword ptr [r15]
1421b610c: jne 0x1421b611d
1421b610e: lea rdx, [rip + 0xe42cfb]
1421b6115: mov rcx, r15
1421b6118: call 0x140b5f870
1421b611d: mov rax, qword ptr [r15 + 8]
1421b6121: mov dword ptr [rax + r12*4], r14d
1421b6125: inc r12
1421b6128: mov qword ptr [r15 + 0x10], r12
1421b612c: mov rdx, qword ptr [rsi + 8]
1421b6130: cmp rdx, r14
1421b6133: jbe 0x1421b6265
1421b6139: mov rcx, r14
1421b613c: shl rcx, 5
1421b6140: mov rax, qword ptr [rsi]
1421b6143: mov byte ptr [rax + r14], 1
1421b6148: mov rdx, qword ptr [rsi + 0x30]
1421b614c: cmp rcx, rdx
1421b614f: jae 0x1421b6276
1421b6155: mov rax, qword ptr [rsi + 0x28]
1421b6159: shl rcx, 6
1421b615d: movups xmm0, xmmword ptr [rdi]
1421b6160: movups xmm1, xmmword ptr [rdi + 0x10]
1421b6164: movups xmm2, xmmword ptr [rdi + 0x20]
1421b6168: movups xmm3, xmmword ptr [rdi + 0x30]
1421b616c: movups xmmword ptr [rax + rcx + 0x30], xmm3
1421b6171: movups xmmword ptr [rax + rcx + 0x20], xmm2
1421b6176: movups xmmword ptr [rax + rcx + 0x10], xmm1
1421b617b: movups xmmword ptr [rax + rcx], xmm0
1421b617f: mov rdx, qword ptr [rsi + 0x18]
1421b6183: mov eax, r14d
1421b6186: shr eax, 6
1421b6189: cmp rax, rdx
1421b618c: jae 0x1421b6284
1421b6192: mov r8d, 1
1421b6198: mov ecx, r14d
1421b619b: shl r8, cl
1421b619e: mov rcx, qword ptr [rsi + 0x10]
1421b61a2: lock or qword ptr [rcx + rax*8], r8
1421b61a7: mov rdx, qword ptr [rsi + 0x48]
1421b61ab: cmp rax, rdx
1421b61ae: jae 0x1421b6295
1421b61b4: mov rcx, qword ptr [rsi + 0x40]
1421b61b8: lock or qword ptr [rcx + rax*8], r8
1421b61bd: mov dword ptr [rsi + 0x50], r14d
1421b61c1: mov rdi, qword ptr [rbp - 0x10]
1421b61c5: mov rcx, rdi
1421b61c8: call 0x140c81e10
1421b61cd: mov rcx, rbx
1421b61d0: call 0x140c81e10
1421b61d5: xor esi, esi
1421b61d7: mov eax, 8
1421b61dc: lock cmpxchg qword ptr [rdi], rsi
1421b61e1: je 0x1421b620b
1421b61e3: xor esi, esi
1421b61e5: mov rcx, rdi
1421b61e8: jmp 0x1421b6232
1421b61ea: mov rcx, r14
1421b61ed: call 0x140c81e10
1421b61f2: mov rcx, rbx
1421b61f5: call 0x140c81e10
1421b61fa: xor ecx, ecx
1421b61fc: mov eax, 8
1421b6201: lock cmpxchg qword ptr [r14], rcx
1421b6206: mov sil, 1
1421b6209: jne 0x1421b622f
1421b620b: mov eax, esi
1421b620d: add rsp, 0x30
1421b6211: pop rbx
1421b6212: pop rdi
1421b6213: pop rsi
1421b6214: pop r12
1421b6216: pop r14
1421b6218: pop r15
1421b621a: pop rbp
1421b621b: ret 
1421b621c: mov rcx, r14
1421b621f: mov r8d, 0x3b9aca00
1421b6225: call 0x1428cb5f0
1421b622a: jmp 0x1421b60c8
1421b622f: mov rcx, r14
1421b6232: xor edx, edx
1421b6234: call 0x1428cbf60
1421b6239: jmp 0x1421b620b
1421b623b: lea r8, [rip + 0xe42bb6]
1421b6242: call 0x1428d9518
1421b6247: lea r8, [rip + 0xe42c0a]
1421b624e: mov rcx, rax
1421b6251: call 0x1428d9518
1421b6256: lea r8, [rip + 0xe42b3b]
1421b625d: mov rcx, rax
1421b6260: call 0x1428d9518
1421b6265: lea r8, [rip + 0xe42bbc]
1421b626c: mov rcx, r14
1421b626f: call 0x1428d9518
1421b6274: jmp 0x1421b62a4
1421b6276: lea r8, [rip + 0xe42bc3]
1421b627d: call 0x1428d9518
1421b6282: jmp 0x1421b62a4
1421b6284: lea r8, [rip + 0xe42b0d]
1421b628b: mov rcx, rax
1421b628e: call 0x1428d9518
1421b6293: jmp 0x1421b62a4
1421b6295: lea r8, [rip + 0xe42afc]
1421b629c: mov rcx, rax
1421b629f: call 0x1428d9518
1421b62a4: ud2 
1421b62a6: nop word ptr cs:[rax + rax]
1421b62b0: mov qword ptr [rsp + 0x10], rdx
1421b62b5: push rbp
1421b62b6: push r15
1421b62b8: push r14
1421b62ba: push r12
1421b62bc: push rsi
1421b62bd: push rdi
1421b62be: push rbx
1421b62bf: sub rsp, 0x20
1421b62c3: lea rbp, [rdx + 0x30]
1421b62c7: mov rcx, qword ptr [rbp - 0x10]
1421b62cb: call 0x1400412d0
1421b62d0: nop 
1421b62d1: add rsp, 0x20
1421b62d5: pop rbx
1421b62d6: pop rdi
1421b62d7: pop rsi
1421b62d8: pop r12
1421b62da: pop r14
1421b62dc: pop r15
1421b62de: pop rbp
1421b62df: ret 
