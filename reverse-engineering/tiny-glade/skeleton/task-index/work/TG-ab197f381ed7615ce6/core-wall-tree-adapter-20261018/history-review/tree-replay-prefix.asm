141bc34f0: push rbp
141bc34f1: push r15
141bc34f3: push r14
141bc34f5: push r13
141bc34f7: push r12
141bc34f9: push rsi
141bc34fa: push rdi
141bc34fb: push rbx
141bc34fc: sub rsp, 0x168
141bc3503: lea rbp, [rsp + 0x80]
141bc350b: movaps xmmword ptr [rbp + 0xd0], xmm9
141bc3513: movaps xmmword ptr [rbp + 0xc0], xmm8
141bc351b: movaps xmmword ptr [rbp + 0xb0], xmm7
141bc3522: movaps xmmword ptr [rbp + 0xa0], xmm6
141bc3529: mov qword ptr [rbp + 0x98], 0xfffffffffffffffe
141bc3534: mov r14, r9
141bc3537: mov rdi, r8
141bc353a: mov rbx, rdx
141bc353d: mov rsi, rcx
141bc3540: mov r15, qword ptr [rcx]
141bc3543: mov rcx, r15
141bc3546: call 0x140949170
141bc354b: movss xmm6, dword ptr [r15 + 0x28]
141bc3551: mov eax, dword ptr [rsi + 0x1c]
141bc3554: mov rcx, qword ptr [rsi + 0x10]
141bc3558: mov dword ptr [rcx], eax
141bc355a: mov rcx, r15
141bc355d: call 0x1409f1a80
141bc3562: test rax, rax
141bc3565: je 0x141bc3d45
141bc356b: mov rsi, rax
141bc356e: mov rax, qword ptr [rbp + 0x150]
141bc3575: mov qword ptr [rbp + 0x60], 0
141bc357d: mov qword ptr [rbp + 0x68], 8
141bc3585: mov qword ptr [rbp + 0x70], 0
141bc358d: mov r15, qword ptr [rsi + 8]
141bc3591: mov rcx, qword ptr [rsi + 0x10]
141bc3595: lea r11, [rcx + rcx*2]
141bc3599: shl r11, 4
141bc359d: add r11, r15
141bc35a0: mov rax, qword ptr [rax]
141bc35a3: mov qword ptr [rbp + 0x78], rax
141bc35a7: mov eax, dword ptr [r14 + 0x1c]
141bc35ab: mov dword ptr [rbp + 0x4c], eax
141bc35ae: mov rax, qword ptr [r14]
141bc35b1: mov rcx, qword ptr [r14 + 0x10]
141bc35b5: mov qword ptr [rbp - 0x38], rcx
141bc35b9: mov ecx, dword ptr [rbx + 0x1c]
141bc35bc: mov dword ptr [rbp + 0x54], ecx
141bc35bf: mov rcx, qword ptr [rbx]
141bc35c2: mov rdx, qword ptr [rbx + 0x10]
141bc35c6: mov qword ptr [rbp - 0x28], rdx
141bc35ca: mov edx, dword ptr [rdi + 0x1c]
141bc35cd: mov dword ptr [rbp + 0x50], edx
141bc35d0: mov rdx, qword ptr [rdi]
141bc35d3: mov r8, qword ptr [rdi + 0x10]
141bc35d7: mov qword ptr [rbp - 0x30], r8
141bc35db: xorps xmm7, xmm7
141bc35de: movss xmm8, dword ptr [rip + 0xd6d789]
141bc35e7: mov qword ptr [rbp + 0x80], rax
141bc35ee: add rax, 0x20
141bc35f2: mov qword ptr [rbp - 0x20], rax
141bc35f6: mov qword ptr [rbp + 8], rcx
141bc35fa: lea rax, [rcx + 0x20]
141bc35fe: mov qword ptr [rbp - 0x10], rax
141bc3602: mov qword ptr [rbp], rdx
141bc3606: lea rax, [rdx + 0x20]
141bc360a: mov qword ptr [rbp - 0x18], rax
141bc360e: xor r13d, r13d
141bc3611: mov qword ptr [rbp - 8], r11
141bc3615: jmp 0x141bc3644
141bc3617: mov rax, qword ptr [rbp + 0x68]
141bc361b: mov qword ptr [rax + rbx*8], rdi
141bc361f: inc rbx
141bc3622: mov qword ptr [rbp + 0x70], rbx
141bc3626: mov rbx, qword ptr [rbp + 0x90]
141bc362d: movss xmm0, dword ptr [rbx + 0x18]
141bc3632: mov rax, qword ptr [rbp + 0x78]
141bc3636: addss xmm0, dword ptr [rax + 0x28]
141bc363b: movss dword ptr [rbx + 0x18], xmm0
141bc3640: mov r11, qword ptr [rbp - 8]
141bc3644: mov rbx, r15
141bc3647: cmp r15, r11
141bc364a: je 0x141bc3a52
141bc3650: mov rdi, r13
141bc3653: lea r15, [rbx + 0x30]
141bc3657: inc r13
141bc365a: cmp byte ptr [rbx], 8
141bc365d: jne 0x141bc3644
141bc365f: mov r12, qword ptr [rbx + 0x10]
141bc3663: mov eax, dword ptr [r12 + 0x10]
141bc3668: cmp eax, 2
141bc366b: mov qword ptr [rbp + 0x90], rbx
141bc3672: je 0x141bc36e5
141bc3674: cmp eax, 1
141bc3677: jne 0x141bc37aa
141bc367d: movss xmm0, dword ptr [rbx + 0x18]
141bc3682: ucomiss xmm0, xmm7
141bc3685: jne 0x141bc39cc
141bc368b: jp 0x141bc39cc
141bc3691: cmp byte ptr [rbx + 0x1c], 0
141bc3695: je 0x141bc3804
141bc369b: mov rbx, qword ptr [r12 + 0x30]
141bc36a0: mov eax, dword ptr [r12 + 0x1c]
141bc36a5: mov dword ptr [rbp + 0x18], eax
141bc36a8: mov rax, qword ptr [r12 + 0x14]
141bc36ad: mov qword ptr [rbp + 0x10], rax
141bc36b1: mov eax, dword ptr [rbp + 0x50]
141bc36b4: mov rcx, qword ptr [rbp - 0x30]
141bc36b8: mov dword ptr [rcx], eax
141bc36ba: mov rax, qword ptr [rbp]
141bc36be: mov r14, qword ptr [rax + 0x30]
141bc36c2: mov r12, qword ptr [rax + 0x40]
141bc36c6: cmp r14, qword ptr [rax + 0x20]
141bc36ca: jne 0x141bc3845
141bc36d0: mov rcx, qword ptr [rbp - 0x18]
141bc36d4: lea rdx, [rip + 0x1471635]
141bc36db: call 0x140562c30
141bc36e0: jmp 0x141bc3845
141bc36e5: movss xmm0, dword ptr [rbx + 0x18]
141bc36ea: ucomiss xmm0, xmm7
141bc36ed: jne 0x141bc39cc
141bc36f3: jp 0x141bc39cc
141bc36f9: cmp byte ptr [rbx + 0x1c], 0
141bc36fd: je 0x141bc3879
141bc3703: lea rdx, [r12 + 0x20]
141bc3708: mov r14, qword ptr [r12 + 0x18]
141bc370d: mov eax, dword ptr [r12 + 0x40]
141bc3712: lea rcx, [rbp + 0x28]
141bc3716: mov dword ptr [rcx + 8], eax
141bc3719: mov rax, qword ptr [r12 + 0x38]
141bc371e: mov qword ptr [rcx], rax
141bc3721: movsd xmm9, qword ptr [r12 + 0x44]
141bc3728: mov ebx, dword ptr [r12 + 0x4c]
141bc372d: lea rcx, [rbp + 0x10]
141bc3731: call 0x140d7bd90
141bc3736: movzx eax, byte ptr [r12 + 0x50]
141bc373c: movlps qword ptr [rbp + 0x34], xmm9
141bc3741: mov dword ptr [rbp + 0x3c], ebx
141bc3744: mov byte ptr [rbp + 0x40], al
141bc3747: mov rax, qword ptr [rbp + 0x10]
141bc374b: mov qword ptr [rbp + 0x88], rax
141bc3752: mov rax, qword ptr [rbp + 0x18]
141bc3756: mov qword ptr [rbp + 0x58], rax
141bc375a: lea rax, [rbp + 0x28]
141bc375e: movups xmm0, xmmword ptr [rax - 8]
141bc3762: movups xmm1, xmmword ptr [rax + 8]
141bc3766: movaps xmmword ptr [rbp - 0x60], xmm0
141bc376a: movaps xmmword ptr [rbp - 0x50], xmm1
141bc376e: mov rax, qword ptr [rax + 0x18]
141bc3772: mov qword ptr [rbp - 0x40], rax
141bc3776: mov eax, dword ptr [rbp + 0x54]
141bc3779: mov rcx, qword ptr [rbp - 0x28]
141bc377d: mov dword ptr [rcx], eax
141bc377f: mov rax, qword ptr [rbp + 8]
141bc3783: mov r12, qword ptr [rax + 0x30]
141bc3787: mov rbx, qword ptr [rax + 0x40]
141bc378b: cmp r12, qword ptr [rax + 0x20]
141bc378f: jne 0x141bc3975
141bc3795: mov rcx, qword ptr [rbp - 0x10]
141bc3799: lea rdx, [rip + 0x1471570]
141bc37a0: call 0x140b5f9f0
141bc37a5: jmp 0x141bc3975
141bc37aa: movss xmm0, dword ptr [rbx + 0x18]
141bc37af: ucomiss xmm0, xmm7
141bc37b2: jne 0x141bc39cc
141bc37b8: jp 0x141bc39cc
141bc37be: cmp byte ptr [rbx + 0x1c], 0
141bc37c2: je 0x141bc38d7
141bc37c8: mov r14, qword ptr [r12 + 0x18]
141bc37cd: mov eax, dword ptr [rbp + 0x4c]
141bc37d0: mov rcx, qword ptr [rbp - 0x38]
141bc37d4: mov dword ptr [rcx], eax
141bc37d6: mov rax, qword ptr [rbp + 0x80]
141bc37dd: mov rbx, qword ptr [rax + 0x30]
141bc37e1: mov r12, qword ptr [rax + 0x40]
141bc37e5: cmp rbx, qword ptr [rax + 0x20]
141bc37e9: jne 0x141bc38ac
141bc37ef: mov rcx, qword ptr [rbp - 0x20]
141bc37f3: lea rdx, [rip + 0x1471516]
141bc37fa: call 0x140562630
141bc37ff: jmp 0x141bc38ac
141bc3804: mov rbx, qword ptr [r12 + 0x30]
141bc3809: mov eax, dword ptr [r12 + 0x28]
141bc380e: mov dword ptr [rbp + 0x18], eax
141bc3811: mov rax, qword ptr [r12 + 0x20]
141bc3816: mov qword ptr [rbp + 0x10], rax
141bc381a: mov eax, dword ptr [rbp + 0x50]
141bc381d: mov rcx, qword ptr [rbp - 0x30]
141bc3821: mov dword ptr [rcx], eax
141bc3823: mov rax, qword ptr [rbp]
141bc3827: mov r14, qword ptr [rax + 0x30]
141bc382b: mov r12, qword ptr [rax + 0x40]
141bc382f: cmp r14, qword ptr [rax + 0x20]
141bc3833: jne 0x141bc3845
141bc3835: mov rcx, qword ptr [rbp - 0x18]
141bc3839: lea rdx, [rip + 0x14714d0]
141bc3840: call 0x140562c30
141bc3845: mov r8, qword ptr [rbp]
141bc3849: mov rax, qword ptr [r8 + 0x28]
141bc384d: mov rcx, r14
141bc3850: shl rcx, 5
141bc3854: mov qword ptr [rax + rcx], r12
141bc3858: mov qword ptr [rax + rcx + 8], rbx
141bc385d: mov edx, dword ptr [rbp + 0x18]
141bc3860: mov dword ptr [rax + rcx + 0x18], edx
141bc3864: mov rdx, qword ptr [rbp + 0x10]
141bc3868: mov qword ptr [rax + rcx + 0x10], rdx
141bc386d: inc r14
141bc3870: mov qword ptr [r8 + 0x30], r14
141bc3874: jmp 0x141bc39c1
141bc3879: mov r14, qword ptr [r12 + 0x18]
141bc387e: mov eax, dword ptr [rbp + 0x4c]
141bc3881: mov rcx, qword ptr [rbp - 0x38]
141bc3885: mov dword ptr [rcx], eax
141bc3887: mov rax, qword ptr [rbp + 0x80]
141bc388e: mov rbx, qword ptr [rax + 0x30]
141bc3892: mov r12, qword ptr [rax + 0x40]
141bc3896: cmp rbx, qword ptr [rax + 0x20]
141bc389a: jne 0x141bc38ac
141bc389c: mov rcx, qword ptr [rbp - 0x20]
141bc38a0: lea rdx, [rip + 0x1471469]
141bc38a7: call 0x140562630
141bc38ac: mov rdx, qword ptr [rbp + 0x80]
141bc38b3: mov rax, qword ptr [rdx + 0x28]
141bc38b7: mov rcx, rbx
141bc38ba: shl rcx, 4
141bc38be: mov qword ptr [rax + rcx], r12
141bc38c2: mov qword ptr [rax + rcx + 8], r14
141bc38c7: inc rbx
141bc38ca: mov qword ptr [rdx + 0x30], rbx
141bc38ce: inc qword ptr [rdx + 0x40]
141bc38d2: jmp 0x141bc39c5
141bc38d7: lea rdx, [r12 + 0x20]
141bc38dc: mov r14, qword ptr [r12 + 0x18]
141bc38e1: mov eax, dword ptr [r12 + 0x40]
141bc38e6: lea rcx, [rbp + 0x28]
141bc38ea: mov dword ptr [rcx + 8], eax
141bc38ed: mov rax, qword ptr [r12 + 0x38]
141bc38f2: mov qword ptr [rcx], rax
141bc38f5: movsd xmm9, qword ptr [r12 + 0x44]
141bc38fc: mov ebx, dword ptr [r12 + 0x4c]
141bc3901: lea rcx, [rbp + 0x10]
141bc3905: call 0x140d7bd90
141bc390a: movzx eax, byte ptr [r12 + 0x50]
141bc3910: movlps qword ptr [rbp + 0x34], xmm9
141bc3915: mov dword ptr [rbp + 0x3c], ebx
141bc3918: mov byte ptr [rbp + 0x40], al
141bc391b: mov rax, qword ptr [rbp + 0x10]
141bc391f: mov qword ptr [rbp + 0x88], rax
141bc3926: mov rax, qword ptr [rbp + 0x18]
141bc392a: mov qword ptr [rbp + 0x58], rax
141bc392e: lea rax, [rbp + 0x28]
141bc3932: movups xmm0, xmmword ptr [rax - 8]
141bc3936: movups xmm1, xmmword ptr [rax + 8]
141bc393a: movaps xmmword ptr [rbp - 0x60], xmm0
141bc393e: movaps xmmword ptr [rbp - 0x50], xmm1
141bc3942: mov rax, qword ptr [rax + 0x18]
141bc3946: mov qword ptr [rbp - 0x40], rax
141bc394a: mov eax, dword ptr [rbp + 0x54]
141bc394d: mov rcx, qword ptr [rbp - 0x28]
141bc3951: mov dword ptr [rcx], eax
141bc3953: mov rax, qword ptr [rbp + 8]
141bc3957: mov r12, qword ptr [rax + 0x30]
141bc395b: mov rbx, qword ptr [rax + 0x40]
141bc395f: cmp r12, qword ptr [rax + 0x20]
141bc3963: jne 0x141bc3975
141bc3965: mov rcx, qword ptr [rbp - 0x10]
141bc3969: lea rdx, [rip + 0x14713a0]
141bc3970: call 0x140b5f9f0
141bc3975: mov r8, qword ptr [rbp + 8]
141bc3979: mov rax, qword ptr [r8 + 0x28]
141bc397d: lea rcx, [r12 + r12*8]
141bc3981: mov rdx, qword ptr [rbp + 0x88]
141bc3988: mov qword ptr [rax + rcx*8], rdx
141bc398c: mov rdx, qword ptr [rbp + 0x58]
141bc3990: mov qword ptr [rax + rcx*8 + 8], rdx
141bc3995: movaps xmm0, xmmword ptr [rbp - 0x60]
141bc3999: movaps xmm1, xmmword ptr [rbp - 0x50]
141bc399d: movups xmmword ptr [rax + rcx*8 + 0x10], xmm0
141bc39a2: movups xmmword ptr [rax + rcx*8 + 0x20], xmm1
141bc39a7: mov rdx, qword ptr [rbp - 0x40]
141bc39ab: mov qword ptr [rax + rcx*8 + 0x30], rdx
141bc39b0: mov qword ptr [rax + rcx*8 + 0x38], r14
141bc39b5: mov qword ptr [rax + rcx*8 + 0x40], rbx
141bc39ba: inc r12
141bc39bd: mov qword ptr [r8 + 0x30], r12
141bc39c1: inc qword ptr [r8 + 0x40]
141bc39c5: mov rbx, qword ptr [rbp + 0x90]
141bc39cc: test byte ptr [rbx + 8], 1
141bc39d0: je 0x141bc3a20
141bc39d2: movss xmm1, dword ptr [rbx + 0xc]
141bc39d7: divss xmm1, xmm6
141bc39db: movss xmm0, dword ptr [rbx + 0x18]
141bc39e0: maxss xmm1, xmm8
141bc39e5: ucomiss xmm0, xmm1
141bc39e8: jbe 0x141bc3632
141bc39ee: mov rbx, qword ptr [rbp + 0x70]
141bc39f2: cmp rbx, qword ptr [rbp + 0x60]
141bc39f6: jne 0x141bc3617
141bc39fc: lea rcx, [rbp + 0x60]
141bc3a00: lea rdx, [rip + 0x132f9f9]
141bc3a07: call 0x140b81df0
141bc3a0c: jmp 0x141bc3617
141bc3a11: nop word ptr cs:[rax + rax]
141bc3a20: mov rbx, qword ptr [rbp + 0x70]
141bc3a24: cmp rbx, qword ptr [rbp + 0x60]
141bc3a28: jne 0x141bc3a3a
141bc3a2a: lea rcx, [rbp + 0x60]
141bc3a2e: lea rdx, [rip + 0x132f9b3]
141bc3a35: call 0x140b81df0
141bc3a3a: mov rax, qword ptr [rbp + 0x68]
141bc3a3e: mov qword ptr [rax + rbx*8], rdi
141bc3a42: inc rbx
141bc3a45: mov qword ptr [rbp + 0x70], rbx
141bc3a49: mov r11, qword ptr [rbp - 8]
141bc3a4d: jmp 0x141bc3644
141bc3a52: mov r10, qword ptr [rbp + 0x68]
141bc3a56: mov rax, qword ptr [rbp + 0x70]
141bc3a5a: cmp rax, 2
141bc3a5e: mov qword ptr [rbp + 0x90], rbx
141bc3a65: jae 0x141bc3a84
141bc3a67: mov qword ptr [rbp + 0x78], r10
141bc3a6b: mov rcx, qword ptr [rbp + 0x60]
141bc3a6f: mov qword ptr [rbp + 0x80], rcx
141bc3a76: test rax, rax
141bc3a79: jne 0x141bc3b5a
141bc3a7f: jmp 0x141bc3d26
141bc3a84: mov rcx, rax
141bc3a87: shr rcx, 1
141bc3a8a: cmp rax, 8
141bc3a8e: jae 0x141bc3a94
141bc3a90: xor edx, edx
141bc3a92: jmp 0x141bc3b06
141bc3a94: mov rdx, rcx
141bc3a97: and rdx, 0xfffffffffffffffc
141bc3a9b: lea r8, [r10 + rax*8]
141bc3a9f: add r8, -0x10
141bc3aa3: xor r9d, r9d
141bc3aa6: nop word ptr cs:[rax + rax]
141bc3ab0: movdqu xmm0, xmmword ptr [r10 + r9*8]
141bc3ab6: movdqu xmm1, xmmword ptr [r10 + r9*8 + 0x10]
141bc3abd: movdqu xmm2, xmmword ptr [r8 - 0x10]
141bc3ac3: movdqu xmm3, xmmword ptr [r8]
141bc3ac8: pshufd xmm3, xmm3, 0x4e
141bc3acd: pshufd xmm2, xmm2, 0x4e
141bc3ad2: movdqu xmmword ptr [r10 + r9*8], xmm3
141bc3ad8: movdqu xmmword ptr [r10 + r9*8 + 0x10], xmm2
141bc3adf: pshufd xmm0, xmm0, 0x4e
141bc3ae4: movdqu xmmword ptr [r8], xmm0
141bc3ae9: pshufd xmm0, xmm1, 0x4e
141bc3aee: movdqu xmmword ptr [r8 - 0x10], xmm0
141bc3af4: add r9, 4
141bc3af8: add r8, -0x20
141bc3afc: cmp rdx, r9
141bc3aff: jne 0x141bc3ab0
141bc3b01: cmp rcx, rdx
141bc3b04: je 0x141bc3b3a
141bc3b06: neg rcx
141bc3b09: lea r8, [r10 + rdx*8]
141bc3b0d: neg rdx
141bc3b10: lea rax, [r10 + rax*8]
141bc3b14: add rax, -8
141bc3b18: nop dword ptr [rax + rax]
141bc3b20: mov r9, qword ptr [r8]
141bc3b23: mov r10, qword ptr [rax + rdx*8]
141bc3b27: mov qword ptr [r8], r10
141bc3b2a: mov qword ptr [rax + rdx*8], r9
141bc3b2e: dec rdx
141bc3b31: add r8, 8
141bc3b35: cmp rcx, rdx
141bc3b38: jne 0x141bc3b20
141bc3b3a: mov rax, qword ptr [rbp + 0x68]
141bc3b3e: mov qword ptr [rbp + 0x78], rax
141bc3b42: mov rax, qword ptr [rbp + 0x70]
141bc3b46: mov rcx, qword ptr [rbp + 0x60]
141bc3b4a: mov qword ptr [rbp + 0x80], rcx
141bc3b51: test rax, rax
141bc3b54: je 0x141bc3d26
141bc3b5a: mov r13, qword ptr [rbp + 0x78]
141bc3b5e: lea r15, [rax*8]
141bc3b66: add r15, r13
141bc3b69: lea rdi, [rbp + 0x18]
