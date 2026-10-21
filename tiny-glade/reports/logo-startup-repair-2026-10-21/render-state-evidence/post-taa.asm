0x1409c40b0: push rbp
0x1409c40b1: push r15
0x1409c40b3: push r14
0x1409c40b5: push r13
0x1409c40b7: push r12
0x1409c40b9: push rsi
0x1409c40ba: push rdi
0x1409c40bb: push rbx
0x1409c40bc: sub rsp, 0x938
0x1409c40c3: lea rbp, [rsp + 0x80]
0x1409c40cb: mov qword ptr [rbp + 0x8b0], 0xfffffffffffffffe
0x1409c40d6: mov rax, qword ptr [r9 + 0x1a8]
0x1409c40dd: test rax, rax
0x1409c40e0: je 0x1409c43da
0x1409c40e6: mov rdi, r9
0x1409c40e9: mov r14, rdx
0x1409c40ec: mov rbx, rcx
0x1409c40ef: mov r12, rax
0x1409c40f2: lea rcx, [r8 + 4]
0x1409c40f6: mov rsi, r8
0x1409c40f9: call 0x140bd5790
0x1409c40fe: mov dword ptr [rbp + 0x3b8], 0
0x1409c4108: mov qword ptr [rbp + 0x3c0], rsi
0x1409c410f: mov dword ptr [rbp + 0x7a0], 2
0x1409c4119: mov qword ptr [rsp + 0x40], rbx
0x1409c411e: mov qword ptr [rsp + 0x38], r14
0x1409c4123: mov qword ptr [rsp + 0x30], rax
0x1409c4128: lea rax, [rbp + 0x7a0]
0x1409c412f: mov qword ptr [rsp + 0x28], rax
0x1409c4134: mov qword ptr [rsp + 0x20], 1
0x1409c413d: lea rdx, [rip + 0x20fbd96]
0x1409c4144: lea rcx, [rbp - 0x30]
0x1409c4148: lea r9, [rbp + 0x3b8]
0x1409c414f: mov r8d, 0x15
0x1409c4155: call 0x140867970
0x1409c415a: mov qword ptr [rbp + 0x8a0], r14
0x1409c4161: lea rcx, [r14 + 0x38]
0x1409c4165: mov byte ptr [rbp + 0x8af], 1
0x1409c416c: movabs rdx, 0x4bc362195b80f729
0x1409c4176: movabs r8, 0x982798ce80cbcefb
0x1409c4180: call 0x1400dd6f0
0x1409c4185: test rax, rax
0x1409c4188: mov r8, qword ptr [rbp + 0x8a0]
0x1409c418f: je 0x1409c4404
0x1409c4195: cmp qword ptr [r8 + 0x218], rdx
0x1409c419c: jbe 0x1409c4404
0x1409c41a2: mov rax, qword ptr [r8 + 0x210]
0x1409c41a9: mov rcx, qword ptr [rax + rdx*8]
0x1409c41ad: test rcx, rcx
0x1409c41b0: je 0x1409c4404
0x1409c41b6: mov rax, qword ptr [r8 + 0x1e0]
0x1409c41bd: not rcx
0x1409c41c0: lea rcx, [rcx + rcx*2]
0x1409c41c4: shl rcx, 5
0x1409c41c8: cmp qword ptr [rax + rcx + 0x20], 0
0x1409c41ce: je 0x1409c4404
0x1409c41d4: add rax, rcx
0x1409c41d7: mov rax, qword ptr [rax + 0x10]
0x1409c41db: mov rcx, qword ptr [rax]
0x1409c41de: test rcx, rcx
0x1409c41e1: js 0x1409c41ea
0x1409c41e3: cvtsi2ss xmm0, rcx
0x1409c41e8: jmp 0x1409c41ff
0x1409c41ea: mov rdx, rcx
0x1409c41ed: shr rdx, 1
0x1409c41f0: and ecx, 1
0x1409c41f3: or rcx, rdx
0x1409c41f6: cvtsi2ss xmm0, rcx
0x1409c41fb: addss xmm0, xmm0
0x1409c41ff: cvtsi2ss xmm1, dword ptr [rax + 8]
0x1409c4204: divss xmm1, dword ptr [rip + 0x1fe3434]
0x1409c420c: addss xmm1, xmm0
0x1409c4210: movss dword ptr [rbp + 0x898], xmm1
0x1409c4218: mov byte ptr [rbp + 0x89c], 1
0x1409c421f: mov rdi, qword ptr [rdi + 0x1a0]
0x1409c4226: shl r12, 7
0x1409c422a: add r12, rdi
0x1409c422d: lea rbx, [rbp + 0x3b8]
0x1409c4234: lea r14, [rbp + 0x7a0]
0x1409c423b: cmp rdi, r12
0x1409c423e: je 0x1409c43b2
0x1409c4244: nop word ptr cs:[rax + rax]
0x1409c4250: mov r15, qword ptr [rdi + 0x50]
0x1409c4254: mov rsi, qword ptr [rdi + 0x58]
0x1409c4258: mov byte ptr [rbp + 0x8af], 1
0x1409c425f: call 0x140be6750
0x1409c4264: mov r13d, eax
0x1409c4267: mov byte ptr [rbp + 0x8af], 1
0x1409c426e: xor ecx, ecx
0x1409c4270: call 0x1400d9b20
0x1409c4275: test rax, rax
0x1409c4278: je 0x1409c43ef
0x1409c427e: mov dword ptr [rbp + 0x848], r13d
0x1409c4285: mov word ptr [rbp + 0x850], 0
0x1409c428e: mov dword ptr [rbp + 0x84c], 0x3f800000
0x1409c4298: mov qword ptr [rbp + 0x7a0], 1
0x1409c42a3: mov qword ptr [rbp + 0x7a8], r15
0x1409c42aa: mov qword ptr [rbp + 0x840], rsi
0x1409c42b1: mov qword ptr [rbp + 0x7b0], 0
0x1409c42bc: mov qword ptr [rbp + 0x7b8], 4
0x1409c42c7: mov qword ptr [rbp + 0x7c0], 0
0x1409c42d2: movups xmm0, xmmword ptr [rax]
0x1409c42d5: inc qword ptr [rax]
0x1409c42d8: movups xmm1, xmmword ptr [rip + 0x20d2451]
0x1409c42df: lea rax, [rbp + 0x810]
0x1409c42e6: movups xmmword ptr [rax + 0x10], xmm1
0x1409c42ea: movups xmm1, xmmword ptr [rip + 0x20d242f]
0x1409c42f1: movups xmmword ptr [rax], xmm1
0x1409c42f4: movups xmmword ptr [rbp + 0x830], xmm0
0x1409c42fb: movabs rax, 0x100000003
0x1409c4305: mov qword ptr [rbp + 0x7e0], rax
0x1409c430c: mov qword ptr [rbp + 0x7c8], 0
0x1409c4317: mov qword ptr [rbp + 0x7d0], 1
0x1409c4322: mov qword ptr [rbp + 0x7d8], 0
0x1409c432d: mov byte ptr [rbp + 0x8af], 1
0x1409c4334: mov rcx, rbx
0x1409c4337: mov rdx, r14
0x1409c433a: lea r8, [rbp - 0x30]
0x1409c433e: xor r9d, r9d
0x1409c4341: call 0x140905be0
0x1409c4346: mov byte ptr [rbp + 0x8af], 1
0x1409c434d: mov rcx, r14
0x1409c4350: mov rdx, rbx
0x1409c4353: lea r8, [rbp + 0x898]
0x1409c435a: mov r9, rdi
0x1409c435d: call 0x14090b460
0x1409c4362: and byte ptr [rbp + 0x848], 0xbf
0x1409c4369: movzx eax, byte ptr [rbp + 0x84b]
0x1409c4370: and al, 0x9f
0x1409c4372: or al, 0x40
0x1409c4374: mov byte ptr [rbp + 0x84b], al
0x1409c437a: mov rsi, qword ptr [rdi + 0x70]
0x1409c437e: mov r8d, 0xf8
0x1409c4384: mov rcx, rbx
0x1409c4387: mov rdx, r14
0x1409c438a: call 0x142923c00
0x1409c438f: mov byte ptr [rbp + 0x8af], 1
0x1409c4396: mov rcx, rbx
0x1409c4399: mov rdx, rdi
0x1409c439c: mov r8, qword ptr [rbp + 0x8a0]
0x1409c43a3: call rsi
0x1409c43a5: sub rdi, -0x80
0x1409c43a9: cmp rdi, r12
0x1409c43ac: jne 0x1409c4250
0x1409c43b2: lea rsi, [rbp + 0x3b8]
0x1409c43b9: lea rdx, [rbp - 0x30]
0x1409c43bd: mov r8d, 0x3e8
0x1409c43c3: mov rcx, rsi
0x1409c43c6: call 0x142923c00
0x1409c43cb: mov byte ptr [rbp + 0x8af], 0
0x1409c43d2: mov rcx, rsi
0x1409c43d5: call 0x140868370
0x1409c43da: nop 
0x1409c43db: add rsp, 0x938
0x1409c43e2: pop rbx
0x1409c43e3: pop rdi
0x1409c43e4: pop rsi
0x1409c43e5: pop r12
0x1409c43e7: pop r13
0x1409c43e9: pop r14
0x1409c43eb: pop r15
0x1409c43ed: pop rbp
0x1409c43ee: ret 
0x1409c43ef: mov byte ptr [rbp + 0x8af], 1
0x1409c43f6: lea rcx, [rip + 0x22766a3]
0x1409c43fd: call 0x1428d80d0
0x1409c4402: jmp 0x1409c4417
0x1409c4404: mov byte ptr [rbp + 0x8af], 1
0x1409c440b: lea rcx, [rip + 0x20e17c6]
0x1409c4412: call 0x1428d9310
0x1409c4417: ud2 
0x1409c4419: nop dword ptr [rax]
0x1409c4420: mov qword ptr [rsp + 0x10], rdx
0x1409c4425: push rbp
0x1409c4426: push r15
0x1409c4428: push r14
0x1409c442a: push r13
0x1409c442c: push r12
0x1409c442e: push rsi
0x1409c442f: push rdi
0x1409c4430: push rbx
0x1409c4431: sub rsp, 0x48
0x1409c4435: lea rbp, [rdx + 0x80]
0x1409c443c: cmp byte ptr [rbp + 0x8af], 0
0x1409c4443: jne 0x1409c4456
0x1409c4445: add rsp, 0x48
0x1409c4449: pop rbx
0x1409c444a: pop rdi
0x1409c444b: pop rsi
0x1409c444c: pop r12
0x1409c444e: pop r13
0x1409c4450: pop r14
0x1409c4452: pop r15
0x1409c4454: pop rbp
0x1409c4455: ret 
0x1409c4456: lea rcx, [rbp - 0x30]
0x1409c445a: call 0x1408923c0
0x1409c445f: jmp 0x1409c4445
