1412bf670: push rbp
1412bf671: push rsi
1412bf672: push rdi
1412bf673: sub rsp, 0x140
1412bf67a: lea rbp, [rsp + 0x80]
1412bf682: mov qword ptr [rbp + 0xb8], 0xfffffffffffffffe
1412bf68d: mov rsi, rcx
1412bf690: lea rcx, [rbp - 0x58]
1412bf694: xor r8d, r8d
1412bf697: call 0x1412e5800
1412bf69c: movzx eax, byte ptr [rip + 0x2375566]
1412bf6a3: mov ecx, 0x90
1412bf6a8: mov edx, 8
1412bf6ad: call 0x140613c10
1412bf6b2: test rax, rax
1412bf6b5: je 0x1412bf782
1412bf6bb: mov rdi, rax
1412bf6be: lea rdx, [rbp - 0x58]
1412bf6c2: mov r8d, 0x90
1412bf6c8: mov rcx, rax
1412bf6cb: call 0x142923c00
1412bf6d0: lea rax, [rip + 0x17de128]
1412bf6d7: mov qword ptr [rbp + 0x68], rax
1412bf6db: mov qword ptr [rbp + 0x70], 0x3a
1412bf6e3: mov qword ptr [rbp + 0x78], rdi
1412bf6e7: lea rcx, [rip + 0x196d32a]
1412bf6ee: mov qword ptr [rbp + 0x80], rcx
1412bf6f5: mov qword ptr [rbp + 0x98], 0
1412bf700: mov byte ptr [rbp + 0xae], 5
1412bf707: mov qword ptr [rbp + 0x88], rax
1412bf70e: mov qword ptr [rbp + 0x90], 0x3a
1412bf719: movzx eax, byte ptr [rip + 0x23754e9]
1412bf720: mov ecx, 0x40
1412bf725: mov edx, 8
1412bf72a: call 0x140613c10
1412bf72f: test rax, rax
1412bf732: je 0x1412bf793
1412bf734: movups xmm0, xmmword ptr [rbp + 0x78]
1412bf738: movups xmm1, xmmword ptr [rbp + 0x88]
1412bf73f: movups xmm2, xmmword ptr [rbp + 0x98]
1412bf746: movups xmm3, xmmword ptr [rbp + 0xa8]
1412bf74d: movups xmmword ptr [rax + 0x30], xmm3
1412bf751: movups xmmword ptr [rax + 0x20], xmm2
1412bf755: movups xmmword ptr [rax + 0x10], xmm1
1412bf759: movups xmmword ptr [rax], xmm0
1412bf75c: movabs rdx, 0xeb575d061b90ebf5
1412bf766: mov rcx, rsi
1412bf769: mov r8, rax
1412bf76c: call 0x140969bf0
1412bf771: cmp rax, 1
1412bf775: je 0x1412bf7a4
1412bf777: add rsp, 0x140
1412bf77e: pop rdi
1412bf77f: pop rsi
1412bf780: pop rbp
1412bf781: ret 
1412bf782: mov ecx, 8
1412bf787: mov edx, 0x90
1412bf78c: call 0x1428d9000
1412bf791: jmp 0x1412bf7a2
1412bf793: mov ecx, 8
1412bf798: mov edx, 0x40
1412bf79d: call 0x1428d9000
1412bf7a2: ud2 
1412bf7a4: mov rax, rdx
1412bf7a7: add rdx, 0x10
1412bf7ab: mov rsi, rdx
1412bf7ae: cmp qword ptr [rax + 0x18], 0x3a
1412bf7b3: jne 0x1412bf842
1412bf7b9: mov rdx, qword ptr [rsi]
1412bf7bc: lea rcx, [rip + 0x17de03c]
1412bf7c3: mov r8d, 0x3a
1412bf7c9: call 0x142923bf0
1412bf7ce: test eax, eax
1412bf7d0: jne 0x1412bf842
1412bf7d2: lea rax, [rip + 0x196e5e7]
1412bf7d9: mov qword ptr [rbp + 0x38], rax
1412bf7dd: lea rax, [rip - 0xd8bf44]
1412bf7e4: mov qword ptr [rbp + 0x40], rax
1412bf7e8: lea rax, [rbp + 0x68]
1412bf7ec: mov qword ptr [rbp + 0x48], rax
1412bf7f0: lea rax, [rip - 0x11b5967]
1412bf7f7: mov qword ptr [rbp + 0x50], rax
1412bf7fb: lea rax, [rip + 0x196de26]
1412bf802: mov qword ptr [rbp + 0x78], rax
1412bf806: mov qword ptr [rbp + 0x80], 3
1412bf811: mov qword ptr [rbp + 0x98], 0
1412bf81c: lea rax, [rbp + 0x38]
1412bf820: mov qword ptr [rbp + 0x88], rax
1412bf827: mov qword ptr [rbp + 0x90], 2
1412bf832: lea rdx, [rip + 0x196de1f]
1412bf839: lea rcx, [rbp + 0x78]
1412bf83d: call 0x1428d9390
1412bf842: lea rax, [rip + 0x196e577]
1412bf849: mov qword ptr [rbp + 0x78], rax
1412bf84d: lea rax, [rip - 0xd8bfb4]
1412bf854: mov qword ptr [rbp + 0x80], rax
1412bf85b: mov qword ptr [rbp + 0x88], rsi
1412bf862: lea rax, [rip - 0x11b59d9]
1412bf869: mov qword ptr [rbp + 0x90], rax
1412bf870: lea rcx, [rbp + 0x68]
1412bf874: mov qword ptr [rbp + 0x98], rcx
1412bf87b: mov qword ptr [rbp + 0xa0], rax
1412bf882: lea rax, [rip + 0x196dcdf]
1412bf889: mov qword ptr [rbp + 0x38], rax
1412bf88d: mov qword ptr [rbp + 0x40], 3
1412bf895: mov qword ptr [rbp + 0x58], 0
1412bf89d: lea rax, [rbp + 0x78]
1412bf8a1: mov qword ptr [rbp + 0x48], rax
1412bf8a5: mov qword ptr [rbp + 0x50], 3
1412bf8ad: lea rdx, [rip + 0x196dd1c]
1412bf8b4: lea rcx, [rbp + 0x38]
1412bf8b8: call 0x1428d9390
1412bf8bd: int3 
1412bf8be: nop 
1412bf8c0: mov qword ptr [rsp + 0x10], rdx
1412bf8c5: push rbp
1412bf8c6: push rsi
1412bf8c7: push rdi
1412bf8c8: sub rsp, 0x20
1412bf8cc: lea rbp, [rdx + 0x80]
1412bf8d3: lea rcx, [rbp - 0x58]
1412bf8d7: call 0x14087fc00
1412bf8dc: nop 
1412bf8dd: add rsp, 0x20
1412bf8e1: pop rdi
1412bf8e2: pop rsi
1412bf8e3: pop rbp
1412bf8e4: ret 
1412bf8e5: nop word ptr cs:[rax + rax]
1412bf8f0: mov qword ptr [rsp + 0x10], rdx
1412bf8f5: push rbp
1412bf8f6: push rsi
1412bf8f7: push rdi
1412bf8f8: sub rsp, 0x20
1412bf8fc: lea rbp, [rdx + 0x80]
1412bf903: lea rcx, [rbp + 0x78]
1412bf907: call 0x1400c04b0
1412bf90c: nop 
1412bf90d: add rsp, 0x20
1412bf911: pop rdi
1412bf912: pop rsi
1412bf913: pop rbp
1412bf914: ret 
