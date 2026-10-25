1412c32d0: push rbp
1412c32d1: push rsi
1412c32d2: sub rsp, 0xd8
1412c32d9: lea rbp, [rsp + 0x80]
1412c32e1: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412c32e9: mov rsi, rcx
1412c32ec: lea rcx, [rbp - 0x58]
1412c32f0: xor r8d, r8d
1412c32f3: call 0x141615d80
1412c32f8: mov qword ptr [rbp - 0x60], 0
1412c3300: movzx eax, byte ptr [rip + 0x2371902]
1412c3307: mov ecx, 0x30
1412c330c: mov edx, 8
1412c3311: call 0x140613c10
1412c3316: test rax, rax
1412c3319: je 0x1412c33cf
1412c331f: movups xmm0, xmmword ptr [rbp - 0x60]
1412c3323: movups xmm1, xmmword ptr [rbp - 0x50]
1412c3327: movups xmm2, xmmword ptr [rbp - 0x40]
1412c332b: movups xmmword ptr [rax + 0x20], xmm2
1412c332f: movups xmmword ptr [rax + 0x10], xmm1
1412c3333: movups xmmword ptr [rax], xmm0
1412c3336: lea rcx, [rip + 0x17d9eb3]
1412c333d: mov qword ptr [rbp], rcx
1412c3341: mov qword ptr [rbp + 8], 0x4e
1412c3349: mov qword ptr [rbp + 0x10], rax
1412c334d: lea rax, [rip + 0x1969c04]
1412c3354: mov qword ptr [rbp + 0x18], rax
1412c3358: mov qword ptr [rbp + 0x30], 0
1412c3360: mov byte ptr [rbp + 0x46], 5
1412c3364: mov qword ptr [rbp + 0x20], rcx
1412c3368: mov qword ptr [rbp + 0x28], 0x4e
1412c3370: movzx eax, byte ptr [rip + 0x2371892]
1412c3377: mov ecx, 0x40
1412c337c: mov edx, 8
1412c3381: call 0x140613c10
1412c3386: test rax, rax
1412c3389: je 0x1412c33e0
1412c338b: movups xmm0, xmmword ptr [rbp + 0x10]
1412c338f: movups xmm1, xmmword ptr [rbp + 0x20]
1412c3393: movups xmm2, xmmword ptr [rbp + 0x30]
1412c3397: movups xmm3, xmmword ptr [rbp + 0x40]
1412c339b: movups xmmword ptr [rax + 0x30], xmm3
1412c339f: movups xmmword ptr [rax + 0x20], xmm2
1412c33a3: movups xmmword ptr [rax + 0x10], xmm1
1412c33a7: movups xmmword ptr [rax], xmm0
1412c33aa: movabs rdx, 0x321bc3dfb10d5554
1412c33b4: mov rcx, rsi
1412c33b7: mov r8, rax
1412c33ba: call 0x140969bf0
1412c33bf: cmp rax, 1
1412c33c3: je 0x1412c33f1
1412c33c5: add rsp, 0xd8
1412c33cc: pop rsi
1412c33cd: pop rbp
1412c33ce: ret 
1412c33cf: mov ecx, 8
1412c33d4: mov edx, 0x30
1412c33d9: call 0x1428d9000
1412c33de: jmp 0x1412c33ef
1412c33e0: mov ecx, 8
1412c33e5: mov edx, 0x40
1412c33ea: call 0x1428d9000
1412c33ef: ud2 
1412c33f1: mov rax, rdx
1412c33f4: add rdx, 0x10
1412c33f8: mov rsi, rdx
1412c33fb: cmp qword ptr [rax + 0x18], 0x4e
1412c3400: jne 0x1412c347e
1412c3402: mov rdx, qword ptr [rsi]
1412c3405: lea rcx, [rip + 0x17d9de4]
1412c340c: mov r8d, 0x4e
1412c3412: call 0x142923bf0
1412c3417: test eax, eax
1412c3419: jne 0x1412c347e
1412c341b: lea rax, [rip + 0x196a3a6]
1412c3422: mov qword ptr [rbp - 0x30], rax
1412c3426: lea rax, [rip - 0xd8fb8d]
1412c342d: mov qword ptr [rbp - 0x28], rax
1412c3431: mov rax, rbp
1412c3434: mov qword ptr [rbp - 0x20], rax
1412c3438: lea rax, [rip - 0x11b95af]
1412c343f: mov qword ptr [rbp - 0x18], rax
1412c3443: lea rax, [rip + 0x196a1de]
1412c344a: mov qword ptr [rbp + 0x10], rax
1412c344e: mov qword ptr [rbp + 0x18], 3
1412c3456: mov qword ptr [rbp + 0x30], 0
1412c345e: lea rax, [rbp - 0x30]
1412c3462: mov qword ptr [rbp + 0x20], rax
1412c3466: mov qword ptr [rbp + 0x28], 2
1412c346e: lea rdx, [rip + 0x196a1e3]
1412c3475: lea rcx, [rbp + 0x10]
1412c3479: call 0x1428d9390
1412c347e: lea rax, [rip + 0x196a343]
1412c3485: mov qword ptr [rbp + 0x10], rax
1412c3489: lea rax, [rip - 0xd8fbf0]
1412c3490: mov qword ptr [rbp + 0x18], rax
1412c3494: mov qword ptr [rbp + 0x20], rsi
1412c3498: lea rax, [rip - 0x11b960f]
1412c349f: mov qword ptr [rbp + 0x28], rax
1412c34a3: mov rcx, rbp
1412c34a6: mov qword ptr [rbp + 0x30], rcx
1412c34aa: mov qword ptr [rbp + 0x38], rax
1412c34ae: lea rax, [rip + 0x196a0b3]
1412c34b5: mov qword ptr [rbp - 0x30], rax
1412c34b9: mov qword ptr [rbp - 0x28], 3
1412c34c1: mov qword ptr [rbp - 0x10], 0
1412c34c9: lea rax, [rbp + 0x10]
1412c34cd: mov qword ptr [rbp - 0x20], rax
1412c34d1: mov qword ptr [rbp - 0x18], 3
1412c34d9: lea rdx, [rip + 0x196a0f0]
1412c34e0: lea rcx, [rbp - 0x30]
1412c34e4: call 0x1428d9390
1412c34e9: int3 
1412c34ea: nop word ptr [rax + rax]
1412c34f0: mov qword ptr [rsp + 0x10], rdx
1412c34f5: push rbp
1412c34f6: push rsi
1412c34f7: sub rsp, 0x28
1412c34fb: lea rbp, [rdx + 0x80]
1412c3502: lea rcx, [rbp - 0x58]
1412c3506: call 0x140be24b0
1412c350b: nop 
1412c350c: add rsp, 0x28
1412c3510: pop rsi
1412c3511: pop rbp
1412c3512: ret 
1412c3513: nop word ptr cs:[rax + rax]
1412c3520: mov qword ptr [rsp + 0x10], rdx
1412c3525: push rbp
1412c3526: push rsi
1412c3527: sub rsp, 0x28
1412c352b: lea rbp, [rdx + 0x80]
1412c3532: lea rcx, [rbp + 0x10]
1412c3536: call 0x1400c04b0
1412c353b: nop 
1412c353c: add rsp, 0x28
1412c3540: pop rsi
1412c3541: pop rbp
1412c3542: ret 
