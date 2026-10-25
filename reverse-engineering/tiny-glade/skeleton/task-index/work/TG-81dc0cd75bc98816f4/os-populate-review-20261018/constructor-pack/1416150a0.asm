1416150a0: push rbp
1416150a1: push r15
1416150a3: push r14
1416150a5: push r12
1416150a7: push rsi
1416150a8: push rdi
1416150a9: push rbx
1416150aa: sub rsp, 0x100
1416150b1: lea rbp, [rsp + 0x80]
1416150b9: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
1416150c1: mov rbx, r8
1416150c4: mov rdi, rdx
1416150c7: mov rsi, rcx
1416150ca: test r8, r8
1416150cd: je 0x14161512a
1416150cf: movabs rcx, 0xf83e0f83e0f83e1
1416150d9: mov rax, r9
1416150dc: mul rcx
1416150df: shr rdx, 5
1416150e3: imul rax, rdx, 0x210
1416150ea: mov rcx, r9
1416150ed: sub rcx, rax
1416150f0: mov qword ptr [rbp - 0x40], rcx
1416150f4: jne 0x1416151ba
1416150fa: mov rax, rbx
1416150fd: and rax, 3
141615101: mov qword ptr [rbp - 0x40], rax
141615105: jne 0x1416151cb
14161510b: xor r12d, r12d
14161510e: cmp r9, 0x210
141615115: setae al
141615118: cmp r9, 0x420
14161511f: jae 0x141615247
141615125: mov r12b, al
141615128: jmp 0x14161512a
14161512a: lea r14, [rbp + 0x10]
14161512e: mov edx, 0x210
141615133: mov rcx, r14
141615136: mov r8d, 0x22
14161513c: mov r9b, 1
14161513f: call 0x140bd4de0
141615144: lea r15, [rbp + 0x68]
141615148: mov r8d, 4
14161514e: mov rcx, r15
141615151: mov rdx, r14
141615154: call 0x140bd4f20
141615159: imul rax, r12, 0x210
141615160: mov qword ptr [rsp + 0x30], rax
141615165: mov qword ptr [rsp + 0x28], rbx
14161516a: mov qword ptr [rsp + 0x20], r15
14161516f: lea r8, [rip + 0x166a749]
141615176: lea rcx, [rbp - 0x40]
14161517a: mov r9d, 0x10
141615180: mov rdx, rdi
141615183: call 0x1416a2dc0
141615188: cmp dword ptr [rbp - 0x40], 0xb
14161518c: jne 0x1416151f5
14161518e: mov rax, qword ptr [rbp - 0x18]
141615192: mov qword ptr [rsi + 0x20], rax
141615196: movups xmm0, xmmword ptr [rbp - 0x38]
14161519a: movups xmm1, xmmword ptr [rbp - 0x28]
14161519e: movups xmmword ptr [rsi + 0x10], xmm1
1416151a2: movups xmmword ptr [rsi], xmm0
1416151a5: mov rax, rsi
1416151a8: add rsp, 0x100
1416151af: pop rbx
1416151b0: pop rdi
1416151b1: pop rsi
1416151b2: pop r12
1416151b4: pop r14
1416151b6: pop r15
1416151b8: pop rbp
1416151b9: ret 
1416151ba: mov qword ptr [rbp + 0x10], 0
1416151c2: lea rax, [rip + 0x16b4d7f]
1416151c9: jmp 0x1416151da
1416151cb: mov qword ptr [rbp + 0x10], 0
1416151d3: lea rax, [rip + 0x16b4d86]
1416151da: mov qword ptr [rsp + 0x20], rax
1416151df: lea r8, [rip + 0x16b4d5a]
1416151e6: lea rdx, [rbp - 0x40]
1416151ea: lea r9, [rbp + 0x10]
1416151ee: xor ecx, ecx
1416151f0: call 0x1428d9596
1416151f5: movups xmm0, xmmword ptr [rbp]
1416151f9: movaps xmmword ptr [rbp + 0x50], xmm0
1416151fd: movups xmm0, xmmword ptr [rbp - 0x40]
141615201: movups xmm1, xmmword ptr [rbp - 0x30]
141615205: movups xmm2, xmmword ptr [rbp - 0x20]
141615209: movups xmm3, xmmword ptr [rbp - 0x10]
14161520d: movaps xmmword ptr [rbp + 0x40], xmm3
141615211: movaps xmmword ptr [rbp + 0x30], xmm2
141615215: movaps xmmword ptr [rbp + 0x20], xmm1
141615219: movaps xmmword ptr [rbp + 0x10], xmm0
14161521d: lea rax, [rip + 0x166af84]
141615224: mov qword ptr [rsp + 0x20], rax
141615229: lea rcx, [rip + 0x16698b8]
141615230: lea r9, [rip + 0x1669891]
141615237: lea r8, [rbp + 0x10]
14161523b: mov edx, 0x2b
141615240: call 0x1428d9760
141615245: ud2 
141615247: lea rax, [rip + 0x148985e]
14161524e: mov qword ptr [rbp - 0x40], rax
141615252: mov qword ptr [rbp - 0x38], 0x30
14161525a: lea rax, [rbp - 0x40]
14161525e: mov qword ptr [rbp + 0x68], rax
141615262: lea rax, [rip - 0x150b3d9]
141615269: mov qword ptr [rbp + 0x70], rax
14161526d: lea rax, [rip + 0x166a554]
141615274: mov qword ptr [rbp + 0x10], rax
141615278: mov qword ptr [rbp + 0x18], 1
141615280: mov qword ptr [rbp + 0x30], 0
141615288: lea rax, [rbp + 0x68]
14161528c: mov qword ptr [rbp + 0x20], rax
141615290: mov qword ptr [rbp + 0x28], 1
141615298: lea rdx, [rip + 0x166a539]
14161529f: lea rcx, [rbp + 0x10]
1416152a3: call 0x1428d9390
1416152a8: int3 
1416152a9: nop dword ptr [rax]
1416152b0: mov qword ptr [rsp + 0x10], rdx
1416152b5: push rbp
1416152b6: push r15
1416152b8: push r14
1416152ba: push r12
1416152bc: push rsi
1416152bd: push rdi
1416152be: push rbx
1416152bf: sub rsp, 0x40
1416152c3: lea rbp, [rdx + 0x80]
1416152ca: lea rcx, [rbp + 0x10]
1416152ce: call 0x1407cea70
1416152d3: nop 
1416152d4: add rsp, 0x40
1416152d8: pop rbx
1416152d9: pop rdi
1416152da: pop rsi
1416152db: pop r12
1416152dd: pop r14
1416152df: pop r15
1416152e1: pop rbp
1416152e2: ret 
