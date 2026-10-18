1412cce90: push r15
1412cce92: push r14
1412cce94: push r12
1412cce96: push rsi
1412cce97: push rdi
1412cce98: push rbx
1412cce99: sub rsp, 0x98
1412ccea0: mov rbx, r8
1412ccea3: mov rdi, rdx
1412ccea6: mov rsi, rcx
1412ccea9: movabs rax, 0x153e2eb3e8651534
1412cceb3: mov qword ptr [rsp + 0x60], rax
1412cceb8: lea rdx, [rip + 0x19614a1]
1412ccebf: mov rcx, rdi
1412ccec2: call 0x140969720
1412ccec7: test rax, rax
1412cceca: je 0x1412cd110
1412cced0: mov r15, rax
1412cced3: mov r14, rax
1412cced6: add r14, 0x20
1412cceda: mov rax, qword ptr [rax + 0x20]
1412ccede: mov ecx, eax
1412ccee0: and ecx, 8
1412ccee3: shr ecx, 3
1412ccee6: cmp rax, -0x10
1412cceea: setae dl
1412cceed: or dl, cl
1412cceef: je 0x1412ccf03
1412ccef1: mov rcx, r14
1412ccef4: xor edx, edx
1412ccef6: mov r9d, 0x3b9aca00
1412ccefc: call 0x1428cc4b0
1412ccf01: jmp 0x1412ccf0e
1412ccf03: lea rcx, [rax + 0x10]
1412ccf07: lock cmpxchg qword ptr [r14], rcx
1412ccf0c: jne 0x1412ccef1
1412ccf0e: mov rcx, r14
1412ccf11: call 0x140c81c70
1412ccf16: mov r12, r14
1412ccf19: or r12, 1
1412ccf1d: mov rcx, r12
1412ccf20: call 0x140c81c70
1412ccf25: cmp byte ptr [r15 + 0x36], 5
1412ccf2a: jne 0x1412cd060
1412ccf30: mov rcx, r14
1412ccf33: call 0x140c81e10
1412ccf38: mov rcx, r12
1412ccf3b: call 0x140c81e10
1412ccf40: mov rax, 0xfffffffffffffff0
1412ccf47: lock xadd qword ptr [r14], rax
1412ccf4c: and rax, 0xfffffffffffffff2
1412ccf50: cmp rax, 0x12
1412ccf54: je 0x1412cd0aa
1412ccf5a: lea rdx, [rip + 0x19613ff]
1412ccf61: mov rcx, rdi
1412ccf64: call 0x140969720
1412ccf69: test rax, rax
1412ccf6c: je 0x1412cd1a1
1412ccf72: mov r14, rax
1412ccf75: lea rax, [rip + 0x17d0ebd]
1412ccf7c: mov qword ptr [rsp + 0x68], rax
1412ccf81: mov qword ptr [rsp + 0x70], 0x3d
1412ccf8a: cmp qword ptr [r14 + 0x18], 0x3d
1412ccf8f: jne 0x1412cd0c1
1412ccf95: mov rdx, qword ptr [r14 + 0x10]
1412ccf99: lea rcx, [rip + 0x17d0e99]
1412ccfa0: mov r8d, 0x3d
1412ccfa6: call 0x142923bf0
1412ccfab: test eax, eax
1412ccfad: jne 0x1412cd0c1
1412ccfb3: mov rcx, qword ptr [r14]
1412ccfb6: mov rax, qword ptr [r14 + 8]
1412ccfba: call qword ptr [rax + 0x30]
1412ccfbd: lea rcx, [rsp + 0x30]
1412ccfc2: mov rdx, rax
1412ccfc5: mov r8, rbx
1412ccfc8: mov r9b, 0x37
1412ccfcb: call 0x140bd4fc0
1412ccfd0: lea rdx, [rsp + 0x60]
1412ccfd5: mov rcx, rdi
1412ccfd8: call 0x140969720
1412ccfdd: test rax, rax
1412ccfe0: je 0x1412cd1ad
1412ccfe6: mov rbx, rax
1412ccfe9: mov rdi, rax
1412ccfec: add rdi, 0x20
1412ccff0: mov ecx, 8
1412ccff5: xor eax, eax
1412ccff7: lock cmpxchg qword ptr [rbx + 0x20], rcx
1412ccffd: jne 0x1412cd0ee
1412cd003: mov rcx, rdi
1412cd006: call 0x140c81c70
1412cd00b: mov r14, rdi
1412cd00e: or r14, 1
1412cd012: mov rcx, r14
1412cd015: call 0x140c81c70
1412cd01a: mov eax, dword ptr [rsp + 0x40]
1412cd01e: movups xmm0, xmmword ptr [rsp + 0x30]
1412cd023: movups xmmword ptr [rbx + 0x28], xmm0
1412cd027: mov dword ptr [rbx + 0x38], eax
1412cd02a: mov rcx, rdi
1412cd02d: call 0x140c81e10
1412cd032: mov rcx, r14
1412cd035: call 0x140c81e10
1412cd03a: xor ecx, ecx
1412cd03c: mov eax, 8
1412cd041: lock cmpxchg qword ptr [rbx + 0x20], rcx
1412cd047: jne 0x1412cd101
1412cd04d: mov rax, qword ptr [rsp + 0x40]
1412cd052: mov qword ptr [rsi + 0x10], rax
1412cd056: movups xmm0, xmmword ptr [rsp + 0x30]
1412cd05b: movups xmmword ptr [rsi], xmm0
1412cd05e: jmp 0x1412cd096
1412cd060: mov eax, dword ptr [r15 + 0x38]
1412cd064: mov dword ptr [rsi + 0x10], eax
1412cd067: movups xmm0, xmmword ptr [r15 + 0x28]
1412cd06c: movups xmmword ptr [rsi], xmm0
1412cd06f: mov rcx, r14
1412cd072: call 0x140c81e10
1412cd077: mov rcx, r12
1412cd07a: call 0x140c81e10
1412cd07f: mov rax, 0xfffffffffffffff0
1412cd086: lock xadd qword ptr [r15 + 0x20], rax
1412cd08c: and rax, 0xfffffffffffffff2
1412cd090: cmp rax, 0x12
1412cd094: je 0x1412cd0b7
1412cd096: mov rax, rsi
1412cd099: add rsp, 0x98
1412cd0a0: pop rbx
1412cd0a1: pop rdi
1412cd0a2: pop rsi
1412cd0a3: pop r12
1412cd0a5: pop r14
1412cd0a7: pop r15
1412cd0a9: ret 
1412cd0aa: mov rcx, r14
1412cd0ad: call 0x1428ccf50
1412cd0b2: jmp 0x1412ccf5a
1412cd0b7: mov rcx, r14
1412cd0ba: call 0x1428ccf50
1412cd0bf: jmp 0x1412cd096
1412cd0c1: mov qword ptr [rsp + 0x30], 0
1412cd0ca: add r14, 0x10
1412cd0ce: lea rax, [rip + 0x19610ab]
1412cd0d5: mov qword ptr [rsp + 0x20], rax
1412cd0da: lea rdx, [rsp + 0x68]
1412cd0df: lea r9, [rsp + 0x30]
1412cd0e4: xor ecx, ecx
1412cd0e6: mov r8, r14
1412cd0e9: call 0x1428cab41
1412cd0ee: mov rcx, rdi
1412cd0f1: mov r8d, 0x3b9aca00
1412cd0f7: call 0x1428cb5f0
1412cd0fc: jmp 0x1412cd003
1412cd101: mov rcx, rdi
1412cd104: xor edx, edx
1412cd106: call 0x1428cbf60
1412cd10b: jmp 0x1412cd04d
1412cd110: lea rax, [rip + 0x17d0d22]
1412cd117: mov qword ptr [rsp + 0x88], rax
1412cd11f: mov qword ptr [rsp + 0x90], 0x3d
1412cd12b: lea rax, [rip + 0x1960cf6]
1412cd132: mov qword ptr [rsp + 0x68], rax
1412cd137: lea rax, [rip - 0xd9989e]
1412cd13e: mov qword ptr [rsp + 0x70], rax
1412cd143: lea rax, [rsp + 0x88]
1412cd14b: mov qword ptr [rsp + 0x78], rax
1412cd150: lea rax, [rip - 0x11c32c7]
1412cd157: mov qword ptr [rsp + 0x80], rax
1412cd15f: lea rax, [rip + 0x196133a]
1412cd166: mov qword ptr [rsp + 0x30], rax
1412cd16b: mov qword ptr [rsp + 0x38], 3
1412cd174: mov qword ptr [rsp + 0x50], 0
1412cd17d: lea rax, [rsp + 0x68]
1412cd182: mov qword ptr [rsp + 0x40], rax
1412cd187: mov qword ptr [rsp + 0x48], 2
1412cd190: lea rdx, [rip + 0x1961339]
1412cd197: lea rcx, [rsp + 0x30]
1412cd19c: call 0x1428d9390
1412cd1a1: lea rcx, [rip + 0x1961288]
1412cd1a8: call 0x1428d9310
1412cd1ad: lea rcx, [rip + 0x1961294]
1412cd1b4: call 0x1428d9310
1412cd1b9: int3 
