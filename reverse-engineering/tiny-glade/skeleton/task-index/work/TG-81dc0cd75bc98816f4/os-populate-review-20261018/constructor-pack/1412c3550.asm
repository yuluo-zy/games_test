1412c3550: push rbp
1412c3551: push rsi
1412c3552: sub rsp, 0xd8
1412c3559: lea rbp, [rsp + 0x80]
1412c3561: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412c3569: mov rsi, rcx
1412c356c: lea rcx, [rbp - 0x58]
1412c3570: xor r8d, r8d
1412c3573: call 0x1416162f0
1412c3578: mov qword ptr [rbp - 0x60], 0
1412c3580: movzx eax, byte ptr [rip + 0x2371682]
1412c3587: mov ecx, 0x30
1412c358c: mov edx, 8
1412c3591: call 0x140613c10
1412c3596: test rax, rax
1412c3599: je 0x1412c364f
1412c359f: movups xmm0, xmmword ptr [rbp - 0x60]
1412c35a3: movups xmm1, xmmword ptr [rbp - 0x50]
1412c35a7: movups xmm2, xmmword ptr [rbp - 0x40]
1412c35ab: movups xmmword ptr [rax + 0x20], xmm2
1412c35af: movups xmmword ptr [rax + 0x10], xmm1
1412c35b3: movups xmmword ptr [rax], xmm0
1412c35b6: lea rcx, [rip + 0x17da673]
1412c35bd: mov qword ptr [rbp], rcx
1412c35c1: mov qword ptr [rbp + 8], 0x2c
1412c35c9: mov qword ptr [rbp + 0x10], rax
1412c35cd: lea rax, [rip + 0x19699bc]
1412c35d4: mov qword ptr [rbp + 0x18], rax
1412c35d8: mov qword ptr [rbp + 0x30], 0
1412c35e0: mov byte ptr [rbp + 0x46], 5
1412c35e4: mov qword ptr [rbp + 0x20], rcx
1412c35e8: mov qword ptr [rbp + 0x28], 0x2c
1412c35f0: movzx eax, byte ptr [rip + 0x2371612]
1412c35f7: mov ecx, 0x40
1412c35fc: mov edx, 8
1412c3601: call 0x140613c10
1412c3606: test rax, rax
1412c3609: je 0x1412c3660
1412c360b: movups xmm0, xmmword ptr [rbp + 0x10]
1412c360f: movups xmm1, xmmword ptr [rbp + 0x20]
1412c3613: movups xmm2, xmmword ptr [rbp + 0x30]
1412c3617: movups xmm3, xmmword ptr [rbp + 0x40]
1412c361b: movups xmmword ptr [rax + 0x30], xmm3
1412c361f: movups xmmword ptr [rax + 0x20], xmm2
1412c3623: movups xmmword ptr [rax + 0x10], xmm1
1412c3627: movups xmmword ptr [rax], xmm0
1412c362a: movabs rdx, 0xbea32db3bf3177c9
1412c3634: mov rcx, rsi
1412c3637: mov r8, rax
1412c363a: call 0x140969bf0
1412c363f: cmp rax, 1
1412c3643: je 0x1412c3671
1412c3645: add rsp, 0xd8
1412c364c: pop rsi
1412c364d: pop rbp
1412c364e: ret 
1412c364f: mov ecx, 8
1412c3654: mov edx, 0x30
1412c3659: call 0x1428d9000
1412c365e: jmp 0x1412c366f
1412c3660: mov ecx, 8
1412c3665: mov edx, 0x40
1412c366a: call 0x1428d9000
1412c366f: ud2 
1412c3671: mov rax, rdx
1412c3674: add rdx, 0x10
1412c3678: mov rsi, rdx
1412c367b: cmp qword ptr [rax + 0x18], 0x2c
1412c3680: jne 0x1412c36fe
1412c3682: mov rdx, qword ptr [rsi]
1412c3685: lea rcx, [rip + 0x17da5a4]
1412c368c: mov r8d, 0x2c
1412c3692: call 0x142923bf0
1412c3697: test eax, eax
1412c3699: jne 0x1412c36fe
1412c369b: lea rax, [rip + 0x196a8f6]
1412c36a2: mov qword ptr [rbp - 0x30], rax
1412c36a6: lea rax, [rip - 0xd8fe0d]
1412c36ad: mov qword ptr [rbp - 0x28], rax
1412c36b1: mov rax, rbp
1412c36b4: mov qword ptr [rbp - 0x20], rax
1412c36b8: lea rax, [rip - 0x11b982f]
1412c36bf: mov qword ptr [rbp - 0x18], rax
1412c36c3: lea rax, [rip + 0x1969f5e]
1412c36ca: mov qword ptr [rbp + 0x10], rax
1412c36ce: mov qword ptr [rbp + 0x18], 3
1412c36d6: mov qword ptr [rbp + 0x30], 0
1412c36de: lea rax, [rbp - 0x30]
1412c36e2: mov qword ptr [rbp + 0x20], rax
1412c36e6: mov qword ptr [rbp + 0x28], 2
1412c36ee: lea rdx, [rip + 0x1969f63]
1412c36f5: lea rcx, [rbp + 0x10]
1412c36f9: call 0x1428d9390
1412c36fe: lea rax, [rip + 0x196a893]
1412c3705: mov qword ptr [rbp + 0x10], rax
1412c3709: lea rax, [rip - 0xd8fe70]
1412c3710: mov qword ptr [rbp + 0x18], rax
1412c3714: mov qword ptr [rbp + 0x20], rsi
1412c3718: lea rax, [rip - 0x11b988f]
1412c371f: mov qword ptr [rbp + 0x28], rax
1412c3723: mov rcx, rbp
1412c3726: mov qword ptr [rbp + 0x30], rcx
1412c372a: mov qword ptr [rbp + 0x38], rax
1412c372e: lea rax, [rip + 0x1969e33]
1412c3735: mov qword ptr [rbp - 0x30], rax
1412c3739: mov qword ptr [rbp - 0x28], 3
1412c3741: mov qword ptr [rbp - 0x10], 0
1412c3749: lea rax, [rbp + 0x10]
1412c374d: mov qword ptr [rbp - 0x20], rax
1412c3751: mov qword ptr [rbp - 0x18], 3
1412c3759: lea rdx, [rip + 0x1969e70]
1412c3760: lea rcx, [rbp - 0x30]
1412c3764: call 0x1428d9390
1412c3769: int3 
1412c376a: nop word ptr [rax + rax]
1412c3770: mov qword ptr [rsp + 0x10], rdx
1412c3775: push rbp
1412c3776: push rsi
1412c3777: sub rsp, 0x28
1412c377b: lea rbp, [rdx + 0x80]
1412c3782: lea rcx, [rbp - 0x58]
1412c3786: call 0x140be24b0
1412c378b: nop 
1412c378c: add rsp, 0x28
1412c3790: pop rsi
1412c3791: pop rbp
1412c3792: ret 
1412c3793: nop word ptr cs:[rax + rax]
1412c37a0: mov qword ptr [rsp + 0x10], rdx
1412c37a5: push rbp
1412c37a6: push rsi
1412c37a7: sub rsp, 0x28
1412c37ab: lea rbp, [rdx + 0x80]
1412c37b2: lea rcx, [rbp + 0x10]
1412c37b6: call 0x1400c04b0
1412c37bb: nop 
1412c37bc: add rsp, 0x28
1412c37c0: pop rsi
1412c37c1: pop rbp
1412c37c2: ret 
