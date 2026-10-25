1412be770: push rbp
1412be771: push rsi
1412be772: sub rsp, 0xe8
1412be779: lea rbp, [rsp + 0x80]
1412be781: mov qword ptr [rbp + 0x60], 0xfffffffffffffffe
1412be789: mov rsi, rcx
1412be78c: lea rcx, [rbp - 0x58]
1412be790: xor r8d, r8d
1412be793: call 0x14161e220
1412be798: movzx eax, byte ptr [rip + 0x237646a]
1412be79f: mov ecx, 0x38
1412be7a4: mov edx, 8
1412be7a9: call 0x140613c10
1412be7ae: test rax, rax
1412be7b1: je 0x1412be86f
1412be7b7: mov rcx, qword ptr [rbp - 0x28]
1412be7bb: mov qword ptr [rax + 0x30], rcx
1412be7bf: movups xmm0, xmmword ptr [rbp - 0x58]
1412be7c3: movups xmm1, xmmword ptr [rbp - 0x48]
1412be7c7: movups xmm2, xmmword ptr [rbp - 0x38]
1412be7cb: movups xmmword ptr [rax + 0x20], xmm2
1412be7cf: movups xmmword ptr [rax + 0x10], xmm1
1412be7d3: movups xmmword ptr [rax], xmm0
1412be7d6: lea rcx, [rip + 0x17df47f]
1412be7dd: mov qword ptr [rbp + 0x10], rcx
1412be7e1: mov qword ptr [rbp + 0x18], 0x42
1412be7e9: mov qword ptr [rbp + 0x20], rax
1412be7ed: lea rax, [rip + 0x196e0d4]
1412be7f4: mov qword ptr [rbp + 0x28], rax
1412be7f8: mov qword ptr [rbp + 0x40], 0
1412be800: mov byte ptr [rbp + 0x56], 5
1412be804: mov qword ptr [rbp + 0x30], rcx
1412be808: mov qword ptr [rbp + 0x38], 0x42
1412be810: movzx eax, byte ptr [rip + 0x23763f2]
1412be817: mov ecx, 0x40
1412be81c: mov edx, 8
1412be821: call 0x140613c10
1412be826: test rax, rax
1412be829: je 0x1412be880
1412be82b: movups xmm0, xmmword ptr [rbp + 0x20]
1412be82f: movups xmm1, xmmword ptr [rbp + 0x30]
1412be833: movups xmm2, xmmword ptr [rbp + 0x40]
1412be837: movups xmm3, xmmword ptr [rbp + 0x50]
1412be83b: movups xmmword ptr [rax + 0x30], xmm3
1412be83f: movups xmmword ptr [rax + 0x20], xmm2
1412be843: movups xmmword ptr [rax + 0x10], xmm1
1412be847: movups xmmword ptr [rax], xmm0
1412be84a: movabs rdx, 0x9bf5d0159630ec93
1412be854: mov rcx, rsi
1412be857: mov r8, rax
1412be85a: call 0x140969bf0
1412be85f: cmp rax, 1
1412be863: je 0x1412be891
1412be865: add rsp, 0xe8
1412be86c: pop rsi
1412be86d: pop rbp
1412be86e: ret 
1412be86f: mov ecx, 8
1412be874: mov edx, 0x38
1412be879: call 0x1428d9000
1412be87e: jmp 0x1412be88f
1412be880: mov ecx, 8
1412be885: mov edx, 0x40
1412be88a: call 0x1428d9000
1412be88f: ud2 
1412be891: mov rax, rdx
1412be894: add rdx, 0x10
1412be898: mov rsi, rdx
1412be89b: cmp qword ptr [rax + 0x18], 0x42
1412be8a0: jne 0x1412be91f
1412be8a2: mov rdx, qword ptr [rsi]
1412be8a5: lea rcx, [rip + 0x17df3b0]
1412be8ac: mov r8d, 0x42
1412be8b2: call 0x142923bf0
1412be8b7: test eax, eax
1412be8b9: jne 0x1412be91f
1412be8bb: lea rax, [rip + 0x196f6b6]
1412be8c2: mov qword ptr [rbp - 0x20], rax
1412be8c6: lea rax, [rip - 0xd8b02d]
1412be8cd: mov qword ptr [rbp - 0x18], rax
1412be8d1: lea rax, [rbp + 0x10]
1412be8d5: mov qword ptr [rbp - 0x10], rax
1412be8d9: lea rax, [rip - 0x11b4a50]
1412be8e0: mov qword ptr [rbp - 8], rax
1412be8e4: lea rax, [rip + 0x196ed3d]
1412be8eb: mov qword ptr [rbp + 0x20], rax
1412be8ef: mov qword ptr [rbp + 0x28], 3
1412be8f7: mov qword ptr [rbp + 0x40], 0
1412be8ff: lea rax, [rbp - 0x20]
1412be903: mov qword ptr [rbp + 0x30], rax
1412be907: mov qword ptr [rbp + 0x38], 2
1412be90f: lea rdx, [rip + 0x196ed42]
1412be916: lea rcx, [rbp + 0x20]
1412be91a: call 0x1428d9390
1412be91f: lea rax, [rip + 0x196f652]
1412be926: mov qword ptr [rbp + 0x20], rax
1412be92a: lea rax, [rip - 0xd8b091]
1412be931: mov qword ptr [rbp + 0x28], rax
1412be935: mov qword ptr [rbp + 0x30], rsi
1412be939: lea rax, [rip - 0x11b4ab0]
1412be940: mov qword ptr [rbp + 0x38], rax
1412be944: lea rcx, [rbp + 0x10]
1412be948: mov qword ptr [rbp + 0x40], rcx
1412be94c: mov qword ptr [rbp + 0x48], rax
1412be950: lea rax, [rip + 0x196ec11]
1412be957: mov qword ptr [rbp - 0x20], rax
1412be95b: mov qword ptr [rbp - 0x18], 3
1412be963: mov qword ptr [rbp], 0
1412be96b: lea rax, [rbp + 0x20]
1412be96f: mov qword ptr [rbp - 0x10], rax
1412be973: mov qword ptr [rbp - 8], 3
1412be97b: lea rdx, [rip + 0x196ec4e]
1412be982: lea rcx, [rbp - 0x20]
1412be986: call 0x1428d9390
1412be98b: int3 
1412be98c: nop dword ptr [rax]
1412be990: mov qword ptr [rsp + 0x10], rdx
1412be995: push rbp
1412be996: push rsi
1412be997: sub rsp, 0x28
1412be99b: lea rbp, [rdx + 0x80]
1412be9a2: lea rcx, [rbp - 0x48]
1412be9a6: call 0x140be24b0
1412be9ab: nop 
1412be9ac: add rsp, 0x28
1412be9b0: pop rsi
1412be9b1: pop rbp
1412be9b2: ret 
1412be9b3: nop word ptr cs:[rax + rax]
1412be9c0: mov qword ptr [rsp + 0x10], rdx
1412be9c5: push rbp
1412be9c6: push rsi
1412be9c7: sub rsp, 0x28
1412be9cb: lea rbp, [rdx + 0x80]
1412be9d2: lea rcx, [rbp + 0x20]
1412be9d6: call 0x1400c04b0
1412be9db: nop 
1412be9dc: add rsp, 0x28
1412be9e0: pop rsi
1412be9e1: pop rbp
1412be9e2: ret 
