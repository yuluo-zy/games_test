140d62810: push rbp
140d62811: push r15
140d62813: push r14
140d62815: push rsi
140d62816: push rdi
140d62817: push rbx
140d62818: sub rsp, 0x48
140d6281c: lea rbp, [rsp + 0x40]
140d62821: mov qword ptr [rbp], 0xfffffffffffffffe
140d62829: mov rdi, r8
140d6282c: test r8, r8
140d6282f: js 0x140d628ba
140d62835: mov rbx, r9
140d62838: mov rsi, rcx
140d6283b: je 0x140d62867
140d6283d: movzx eax, byte ptr [rip + 0x28d23c5]
140d62844: mov r14d, 1
140d6284a: mov r15, rdx
140d6284d: mov edx, 1
140d62852: mov rcx, rdi
140d62855: call 0x140613c10
140d6285a: mov rdx, r15
140d6285d: mov r15, rax
140d62860: test rax, rax
140d62863: jne 0x140d6286d
140d62865: jmp 0x140d628bd
140d62867: mov r15d, 1
140d6286d: mov r14, qword ptr [rbp + 0x60]
140d62871: mov rcx, r15
140d62874: mov r8, rdi
140d62877: call 0x142923c00
140d6287c: mov qword ptr [rbp - 0x20], rdi
140d62880: mov qword ptr [rbp - 0x18], r15
140d62884: mov qword ptr [rbp - 0x10], rdi
140d62888: mov byte ptr [rbp - 8], 0
140d6288c: lea rcx, [rbp - 0x20]
140d62890: mov rdx, rbx
140d62893: mov r8, r14
140d62896: call 0x140d60a70
140d6289b: movups xmm0, xmmword ptr [rbp - 0x20]
140d6289f: movups xmm1, xmmword ptr [rbp - 0x10]
140d628a3: movups xmmword ptr [rsi + 0x10], xmm1
140d628a7: movups xmmword ptr [rsi], xmm0
140d628aa: mov rax, rsi
140d628ad: add rsp, 0x48
140d628b1: pop rbx
140d628b2: pop rdi
140d628b3: pop rsi
140d628b4: pop r14
140d628b6: pop r15
140d628b8: pop rbp
140d628b9: ret 
140d628ba: xor r14d, r14d
140d628bd: lea r8, [rip + 0x1de3adc]
140d628c4: mov rcx, r14
140d628c7: mov rdx, rdi
140d628ca: call 0x1428d8fe3
140d628cf: int3 
140d628d0: mov qword ptr [rsp + 0x10], rdx
140d628d5: push rbp
140d628d6: push r15
140d628d8: push r14
140d628da: push rsi
140d628db: push rdi
140d628dc: push rbx
140d628dd: sub rsp, 0x28
140d628e1: lea rbp, [rdx + 0x40]
140d628e5: mov rdx, qword ptr [rbp - 0x20]
140d628e9: test rdx, rdx
140d628ec: je 0x140d628fd
140d628ee: mov rcx, qword ptr [rbp - 0x18]
140d628f2: mov r8d, 1
140d628f8: call 0x140613c20
140d628fd: nop 
140d628fe: add rsp, 0x28
140d62902: pop rbx
140d62903: pop rdi
140d62904: pop rsi
140d62905: pop r14
140d62907: pop r15
140d62909: pop rbp
140d6290a: ret 
