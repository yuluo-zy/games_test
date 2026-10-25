1412c7550: push r15
1412c7552: push r14
1412c7554: push r12
1412c7556: push rsi
1412c7557: push rdi
1412c7558: push rbx
1412c7559: sub rsp, 0x98
1412c7560: mov rbx, r8
1412c7563: mov rdi, rdx
1412c7566: mov rsi, rcx
1412c7569: movabs rax, 0xd0f0827e7ff22799
1412c7573: mov qword ptr [rsp + 0x60], rax
1412c7578: lea rdx, [rip + 0x1966d21]
1412c757f: mov rcx, rdi
1412c7582: call 0x140969720
1412c7587: test rax, rax
1412c758a: je 0x1412c77d0
1412c7590: mov r15, rax
1412c7593: mov r14, rax
1412c7596: add r14, 0x20
1412c759a: mov rax, qword ptr [rax + 0x20]
1412c759e: mov ecx, eax
1412c75a0: and ecx, 8
1412c75a3: shr ecx, 3
1412c75a6: cmp rax, -0x10
1412c75aa: setae dl
1412c75ad: or dl, cl
1412c75af: je 0x1412c75c3
1412c75b1: mov rcx, r14
1412c75b4: xor edx, edx
1412c75b6: mov r9d, 0x3b9aca00
1412c75bc: call 0x1428cc4b0
1412c75c1: jmp 0x1412c75ce
1412c75c3: lea rcx, [rax + 0x10]
1412c75c7: lock cmpxchg qword ptr [r14], rcx
1412c75cc: jne 0x1412c75b1
1412c75ce: mov rcx, r14
1412c75d1: call 0x140c81c70
1412c75d6: mov r12, r14
1412c75d9: or r12, 1
1412c75dd: mov rcx, r12
1412c75e0: call 0x140c81c70
1412c75e5: cmp byte ptr [r15 + 0x36], 5
1412c75ea: jne 0x1412c7720
1412c75f0: mov rcx, r14
1412c75f3: call 0x140c81e10
1412c75f8: mov rcx, r12
1412c75fb: call 0x140c81e10
1412c7600: mov rax, 0xfffffffffffffff0
1412c7607: lock xadd qword ptr [r14], rax
1412c760c: and rax, 0xfffffffffffffff2
1412c7610: cmp rax, 0x12
1412c7614: je 0x1412c776a
1412c761a: lea rdx, [rip + 0x1966c7f]
1412c7621: mov rcx, rdi
1412c7624: call 0x140969720
1412c7629: test rax, rax
1412c762c: je 0x1412c7861
1412c7632: mov r14, rax
1412c7635: lea rax, [rip + 0x17d6e1b]
1412c763c: mov qword ptr [rsp + 0x68], rax
1412c7641: mov qword ptr [rsp + 0x70], 0x55
1412c764a: cmp qword ptr [r14 + 0x18], 0x55
1412c764f: jne 0x1412c7781
1412c7655: mov rdx, qword ptr [r14 + 0x10]
1412c7659: lea rcx, [rip + 0x17d6df7]
1412c7660: mov r8d, 0x55
1412c7666: call 0x142923bf0
1412c766b: test eax, eax
1412c766d: jne 0x1412c7781
1412c7673: mov rcx, qword ptr [r14]
1412c7676: mov rax, qword ptr [r14 + 8]
1412c767a: call qword ptr [rax + 0x30]
1412c767d: lea rcx, [rsp + 0x30]
1412c7682: mov rdx, rax
1412c7685: mov r8, rbx
1412c7688: mov r9b, 0x37
1412c768b: call 0x140bd4fc0
1412c7690: lea rdx, [rsp + 0x60]
1412c7695: mov rcx, rdi
1412c7698: call 0x140969720
1412c769d: test rax, rax
1412c76a0: je 0x1412c786d
1412c76a6: mov rbx, rax
1412c76a9: mov rdi, rax
1412c76ac: add rdi, 0x20
1412c76b0: mov ecx, 8
1412c76b5: xor eax, eax
1412c76b7: lock cmpxchg qword ptr [rbx + 0x20], rcx
1412c76bd: jne 0x1412c77ae
1412c76c3: mov rcx, rdi
1412c76c6: call 0x140c81c70
1412c76cb: mov r14, rdi
1412c76ce: or r14, 1
1412c76d2: mov rcx, r14
1412c76d5: call 0x140c81c70
1412c76da: mov eax, dword ptr [rsp + 0x40]
1412c76de: movups xmm0, xmmword ptr [rsp + 0x30]
1412c76e3: movups xmmword ptr [rbx + 0x28], xmm0
1412c76e7: mov dword ptr [rbx + 0x38], eax
1412c76ea: mov rcx, rdi
1412c76ed: call 0x140c81e10
1412c76f2: mov rcx, r14
1412c76f5: call 0x140c81e10
1412c76fa: xor ecx, ecx
1412c76fc: mov eax, 8
1412c7701: lock cmpxchg qword ptr [rbx + 0x20], rcx
1412c7707: jne 0x1412c77c1
1412c770d: mov rax, qword ptr [rsp + 0x40]
1412c7712: mov qword ptr [rsi + 0x10], rax
1412c7716: movups xmm0, xmmword ptr [rsp + 0x30]
1412c771b: movups xmmword ptr [rsi], xmm0
1412c771e: jmp 0x1412c7756
1412c7720: mov eax, dword ptr [r15 + 0x38]
1412c7724: mov dword ptr [rsi + 0x10], eax
1412c7727: movups xmm0, xmmword ptr [r15 + 0x28]
1412c772c: movups xmmword ptr [rsi], xmm0
1412c772f: mov rcx, r14
1412c7732: call 0x140c81e10
1412c7737: mov rcx, r12
1412c773a: call 0x140c81e10
1412c773f: mov rax, 0xfffffffffffffff0
1412c7746: lock xadd qword ptr [r15 + 0x20], rax
1412c774c: and rax, 0xfffffffffffffff2
1412c7750: cmp rax, 0x12
1412c7754: je 0x1412c7777
1412c7756: mov rax, rsi
1412c7759: add rsp, 0x98
1412c7760: pop rbx
1412c7761: pop rdi
1412c7762: pop rsi
1412c7763: pop r12
1412c7765: pop r14
1412c7767: pop r15
1412c7769: ret 
1412c776a: mov rcx, r14
1412c776d: call 0x1428ccf50
1412c7772: jmp 0x1412c761a
1412c7777: mov rcx, r14
1412c777a: call 0x1428ccf50
1412c777f: jmp 0x1412c7756
1412c7781: mov qword ptr [rsp + 0x30], 0
1412c778a: add r14, 0x10
1412c778e: lea rax, [rip + 0x19669eb]
1412c7795: mov qword ptr [rsp + 0x20], rax
1412c779a: lea rdx, [rsp + 0x68]
1412c779f: lea r9, [rsp + 0x30]
1412c77a4: xor ecx, ecx
1412c77a6: mov r8, r14
1412c77a9: call 0x1428cab41
1412c77ae: mov rcx, rdi
1412c77b1: mov r8d, 0x3b9aca00
1412c77b7: call 0x1428cb5f0
1412c77bc: jmp 0x1412c76c3
1412c77c1: mov rcx, rdi
1412c77c4: xor edx, edx
1412c77c6: call 0x1428cbf60
1412c77cb: jmp 0x1412c770d
1412c77d0: lea rax, [rip + 0x17d6c80]
1412c77d7: mov qword ptr [rsp + 0x88], rax
1412c77df: mov qword ptr [rsp + 0x90], 0x55
1412c77eb: lea rax, [rip + 0x1966156]
1412c77f2: mov qword ptr [rsp + 0x68], rax
1412c77f7: lea rax, [rip - 0xd93f5e]
1412c77fe: mov qword ptr [rsp + 0x70], rax
1412c7803: lea rax, [rsp + 0x88]
1412c780b: mov qword ptr [rsp + 0x78], rax
1412c7810: lea rax, [rip - 0x11bd987]
1412c7817: mov qword ptr [rsp + 0x80], rax
1412c781f: lea rax, [rip + 0x1966c7a]
1412c7826: mov qword ptr [rsp + 0x30], rax
1412c782b: mov qword ptr [rsp + 0x38], 3
1412c7834: mov qword ptr [rsp + 0x50], 0
1412c783d: lea rax, [rsp + 0x68]
1412c7842: mov qword ptr [rsp + 0x40], rax
1412c7847: mov qword ptr [rsp + 0x48], 2
1412c7850: lea rdx, [rip + 0x1966c79]
1412c7857: lea rcx, [rsp + 0x30]
1412c785c: call 0x1428d9390
1412c7861: lea rcx, [rip + 0x1966bc8]
1412c7868: call 0x1428d9310
1412c786d: lea rcx, [rip + 0x1966bd4]
1412c7874: call 0x1428d9310
1412c7879: int3 
