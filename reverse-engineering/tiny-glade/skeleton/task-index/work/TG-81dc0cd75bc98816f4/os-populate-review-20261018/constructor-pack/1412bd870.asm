1412bd870: push rbp
1412bd871: push rsi
1412bd872: sub rsp, 0xd8
1412bd879: lea rbp, [rsp + 0x80]
1412bd881: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412bd889: mov rsi, rcx
1412bd88c: lea rcx, [rbp - 0x58]
1412bd890: xor r8d, r8d
1412bd893: call 0x141612650
1412bd898: movzx eax, byte ptr [rip + 0x237736a]
1412bd89f: mov ecx, 0x28
1412bd8a4: mov edx, 8
1412bd8a9: call 0x140613c10
1412bd8ae: test rax, rax
1412bd8b1: je 0x1412bd967
1412bd8b7: mov rcx, qword ptr [rbp - 0x38]
1412bd8bb: mov qword ptr [rax + 0x20], rcx
1412bd8bf: movups xmm0, xmmword ptr [rbp - 0x58]
1412bd8c3: movups xmm1, xmmword ptr [rbp - 0x48]
1412bd8c7: movups xmmword ptr [rax + 0x10], xmm1
1412bd8cb: movups xmmword ptr [rax], xmm0
1412bd8ce: lea rcx, [rip + 0x17dfdc9]
1412bd8d5: mov qword ptr [rbp], rcx
1412bd8d9: mov qword ptr [rbp + 8], 0x46
1412bd8e1: mov qword ptr [rbp + 0x10], rax
1412bd8e5: lea rax, [rip + 0x196ee8c]
1412bd8ec: mov qword ptr [rbp + 0x18], rax
1412bd8f0: mov qword ptr [rbp + 0x30], 0
1412bd8f8: mov byte ptr [rbp + 0x46], 5
1412bd8fc: mov qword ptr [rbp + 0x20], rcx
1412bd900: mov qword ptr [rbp + 0x28], 0x46
1412bd908: movzx eax, byte ptr [rip + 0x23772fa]
1412bd90f: mov ecx, 0x40
1412bd914: mov edx, 8
1412bd919: call 0x140613c10
1412bd91e: test rax, rax
1412bd921: je 0x1412bd978
1412bd923: movups xmm0, xmmword ptr [rbp + 0x10]
1412bd927: movups xmm1, xmmword ptr [rbp + 0x20]
1412bd92b: movups xmm2, xmmword ptr [rbp + 0x30]
1412bd92f: movups xmm3, xmmword ptr [rbp + 0x40]
1412bd933: movups xmmword ptr [rax + 0x30], xmm3
1412bd937: movups xmmword ptr [rax + 0x20], xmm2
1412bd93b: movups xmmword ptr [rax + 0x10], xmm1
1412bd93f: movups xmmword ptr [rax], xmm0
1412bd942: movabs rdx, 0x22b9961e6e76bec0
1412bd94c: mov rcx, rsi
1412bd94f: mov r8, rax
1412bd952: call 0x140969bf0
1412bd957: cmp rax, 1
1412bd95b: je 0x1412bd989
1412bd95d: add rsp, 0xd8
1412bd964: pop rsi
1412bd965: pop rbp
1412bd966: ret 
1412bd967: mov ecx, 8
1412bd96c: mov edx, 0x28
1412bd971: call 0x1428d9000
1412bd976: jmp 0x1412bd987
1412bd978: mov ecx, 8
1412bd97d: mov edx, 0x40
1412bd982: call 0x1428d9000
1412bd987: ud2 
1412bd989: mov rax, rdx
1412bd98c: add rdx, 0x10
1412bd990: mov rsi, rdx
1412bd993: cmp qword ptr [rax + 0x18], 0x46
1412bd998: jne 0x1412bda16
1412bd99a: mov rdx, qword ptr [rsi]
1412bd99d: lea rcx, [rip + 0x17dfcfa]
1412bd9a4: mov r8d, 0x46
1412bd9aa: call 0x142923bf0
1412bd9af: test eax, eax
1412bd9b1: jne 0x1412bda16
1412bd9b3: lea rax, [rip + 0x196ffc6]
1412bd9ba: mov qword ptr [rbp - 0x30], rax
1412bd9be: lea rax, [rip - 0xd8a125]
1412bd9c5: mov qword ptr [rbp - 0x28], rax
1412bd9c9: mov rax, rbp
1412bd9cc: mov qword ptr [rbp - 0x20], rax
1412bd9d0: lea rax, [rip - 0x11b3b47]
1412bd9d7: mov qword ptr [rbp - 0x18], rax
1412bd9db: lea rax, [rip + 0x196fc46]
1412bd9e2: mov qword ptr [rbp + 0x10], rax
1412bd9e6: mov qword ptr [rbp + 0x18], 3
1412bd9ee: mov qword ptr [rbp + 0x30], 0
1412bd9f6: lea rax, [rbp - 0x30]
1412bd9fa: mov qword ptr [rbp + 0x20], rax
1412bd9fe: mov qword ptr [rbp + 0x28], 2
1412bda06: lea rdx, [rip + 0x196fc4b]
1412bda0d: lea rcx, [rbp + 0x10]
1412bda11: call 0x1428d9390
1412bda16: lea rax, [rip + 0x196ff63]
1412bda1d: mov qword ptr [rbp + 0x10], rax
1412bda21: lea rax, [rip - 0xd8a188]
1412bda28: mov qword ptr [rbp + 0x18], rax
1412bda2c: mov qword ptr [rbp + 0x20], rsi
1412bda30: lea rax, [rip - 0x11b3ba7]
1412bda37: mov qword ptr [rbp + 0x28], rax
1412bda3b: mov rcx, rbp
1412bda3e: mov qword ptr [rbp + 0x30], rcx
1412bda42: mov qword ptr [rbp + 0x38], rax
1412bda46: lea rax, [rip + 0x196fb1b]
1412bda4d: mov qword ptr [rbp - 0x30], rax
1412bda51: mov qword ptr [rbp - 0x28], 3
1412bda59: mov qword ptr [rbp - 0x10], 0
1412bda61: lea rax, [rbp + 0x10]
1412bda65: mov qword ptr [rbp - 0x20], rax
1412bda69: mov qword ptr [rbp - 0x18], 3
1412bda71: lea rdx, [rip + 0x196fb58]
1412bda78: lea rcx, [rbp - 0x30]
1412bda7c: call 0x1428d9390
1412bda81: int3 
1412bda82: nop word ptr cs:[rax + rax]
1412bda90: mov qword ptr [rsp + 0x10], rdx
1412bda95: push rbp
1412bda96: push rsi
1412bda97: sub rsp, 0x28
1412bda9b: lea rbp, [rdx + 0x80]
1412bdaa2: lea rcx, [rbp - 0x58]
1412bdaa6: call 0x140be24b0
1412bdaab: nop 
1412bdaac: add rsp, 0x28
1412bdab0: pop rsi
1412bdab1: pop rbp
1412bdab2: ret 
1412bdab3: nop word ptr cs:[rax + rax]
1412bdac0: mov qword ptr [rsp + 0x10], rdx
1412bdac5: push rbp
1412bdac6: push rsi
1412bdac7: sub rsp, 0x28
1412bdacb: lea rbp, [rdx + 0x80]
1412bdad2: lea rcx, [rbp + 0x10]
1412bdad6: call 0x1400c04b0
1412bdadb: nop 
1412bdadc: add rsp, 0x28
1412bdae0: pop rsi
1412bdae1: pop rbp
1412bdae2: ret 
