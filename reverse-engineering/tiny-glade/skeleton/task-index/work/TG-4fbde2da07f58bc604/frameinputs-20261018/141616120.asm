141616120: push rbp
141616121: push r15
141616123: push r14
141616125: push r12
141616127: push rsi
141616128: push rdi
141616129: push rbx
14161612a: sub rsp, 0x100
141616131: lea rbp, [rsp + 0x80]
141616139: mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
141616141: mov rsi, rcx
141616144: test r8, r8
141616147: sete al
14161614a: cmp r9, 0x2711
141616151: setb cl
141616154: or cl, al
141616156: je 0x1416161f7
14161615c: mov r14, r9
14161615f: mov rbx, r8
141616162: mov rdi, rdx
141616165: lea r15, [rbp + 0x10]
141616169: mov edx, 0x75300
14161616e: mov rcx, r15
141616171: mov r8d, 0x22
141616177: mov r9b, 1
14161617a: call 0x140bd4de0
14161617f: lea r12, [rbp + 0x68]
141616183: mov r8d, 4
141616189: mov rcx, r12
14161618c: mov rdx, r15
14161618f: call 0x140bd4f20
141616194: shl r14, 4
141616198: lea rax, [r14 + r14*2]
14161619c: mov qword ptr [rsp + 0x30], rax
1416161a1: mov qword ptr [rsp + 0x28], rbx
1416161a6: mov qword ptr [rsp + 0x20], r12
1416161ab: lea r8, [rip + 0x166972e]
1416161b2: lea rcx, [rbp - 0x40]
1416161b6: mov r9d, 0xf
1416161bc: mov rdx, rdi
1416161bf: call 0x1416a2dc0
1416161c4: cmp dword ptr [rbp - 0x40], 0xb
1416161c8: jne 0x141616258
1416161ce: mov rax, qword ptr [rbp - 0x18]
1416161d2: mov qword ptr [rsi + 0x20], rax
1416161d6: movups xmm0, xmmword ptr [rbp - 0x38]
1416161da: movups xmm1, xmmword ptr [rbp - 0x28]
1416161de: movups xmmword ptr [rsi + 0x10], xmm1
1416161e2: movups xmmword ptr [rsi], xmm0
1416161e5: add rsp, 0x100
1416161ec: pop rbx
1416161ed: pop rdi
1416161ee: pop rsi
1416161ef: pop r12
1416161f1: pop r14
1416161f3: pop r15
1416161f5: pop rbp
1416161f6: ret 
1416161f7: lea rax, [rip + 0x1488989]
1416161fe: mov qword ptr [rbp - 0x40], rax
141616202: mov qword ptr [rbp - 0x38], 0x3d
14161620a: lea rax, [rbp - 0x40]
14161620e: mov qword ptr [rbp + 0x68], rax
141616212: lea rax, [rip - 0x150c389]
141616219: mov qword ptr [rbp + 0x70], rax
14161621d: lea rax, [rip + 0x16695a4]
141616224: mov qword ptr [rbp + 0x10], rax
141616228: mov qword ptr [rbp + 0x18], 1
141616230: mov qword ptr [rbp + 0x30], 0
141616238: lea rax, [rbp + 0x68]
14161623c: mov qword ptr [rbp + 0x20], rax
141616240: mov qword ptr [rbp + 0x28], 1
141616248: lea rdx, [rip + 0x1669589]
14161624f: lea rcx, [rbp + 0x10]
141616253: call 0x1428d9390
141616258: movups xmm0, xmmword ptr [rbp]
14161625c: movaps xmmword ptr [rbp + 0x50], xmm0
141616260: movups xmm0, xmmword ptr [rbp - 0x40]
141616264: movups xmm1, xmmword ptr [rbp - 0x30]
141616268: movups xmm2, xmmword ptr [rbp - 0x20]
14161626c: movups xmm3, xmmword ptr [rbp - 0x10]
141616270: movaps xmmword ptr [rbp + 0x40], xmm3
141616274: movaps xmmword ptr [rbp + 0x30], xmm2
141616278: movaps xmmword ptr [rbp + 0x20], xmm1
14161627c: movaps xmmword ptr [rbp + 0x10], xmm0
141616280: lea rax, [rip + 0x1669f21]
141616287: mov qword ptr [rsp + 0x20], rax
14161628c: lea rcx, [rip + 0x1668855]
141616293: lea r9, [rip + 0x166882e]
14161629a: lea r8, [rbp + 0x10]
14161629e: mov edx, 0x2b
1416162a3: call 0x1428d9760
1416162a8: ud2 
1416162aa: nop word ptr [rax + rax]
1416162b0: mov qword ptr [rsp + 0x10], rdx
1416162b5: push rbp
1416162b6: push r15
1416162b8: push r14
1416162ba: push r12
1416162bc: push rsi
1416162bd: push rdi
1416162be: push rbx
1416162bf: sub rsp, 0x40
1416162c3: lea rbp, [rdx + 0x80]
1416162ca: lea rcx, [rbp + 0x10]
1416162ce: call 0x1407cea70
1416162d3: nop 
1416162d4: add rsp, 0x40
1416162d8: pop rbx
1416162d9: pop rdi
1416162da: pop rsi
1416162db: pop r12
1416162dd: pop r14
1416162df: pop r15
1416162e1: pop rbp
1416162e2: ret 
