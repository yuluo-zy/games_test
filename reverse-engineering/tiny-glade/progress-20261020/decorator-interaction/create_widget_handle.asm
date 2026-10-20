0x141282300: push r15
0x141282302: push r14
0x141282304: push r12
0x141282306: push rsi
0x141282307: push rdi
0x141282308: push rbp
0x141282309: push rbx
0x14128230a: sub rsp, 0x190
0x141282311: mov ebx, r8d
0x141282314: mov rsi, rdx
0x141282317: mov rdi, rcx
0x14128231a: mov r14, qword ptr [rsp + 0x1f8]
0x141282322: cmp byte ptr [rsp + 0x1f0], 0x3f
0x14128232a: sete bpl
0x14128232e: shl bpl, 3
0x141282332: movss dword ptr [rsp + 0x34], xmm3
0x141282338: mov byte ptr [rsp + 0x30], 5
0x14128233d: mov byte ptr [rsp + 0x20], 0
0x141282342: lea r15, [rsp + 0xe0]
0x14128234a: lea rdx, [rsp + 0x30]
0x14128234f: lea r8, [rsp + 0x20]
0x141282354: mov rcx, r15
0x141282357: call 0x1409a8e10
0x14128235c: or bpl, byte ptr [rsp + 0x189]
0x141282364: or bpl, 2
0x141282368: mov byte ptr [rsp + 0x189], bpl
0x141282370: lea r12, [rsp + 0x30]
0x141282375: mov rcx, r12
0x141282378: mov rdx, r15
0x14128237b: mov r8, r14
0x14128237e: call 0x1409a8e80
0x141282383: mov rcx, rdi
0x141282386: mov rdx, rsi
0x141282389: mov r8d, ebx
0x14128238c: mov r9, r12
0x14128238f: call 0x14125d180
0x141282394: mov qword ptr [rsp + 0x20], rax
0x141282399: mov qword ptr [rsp + 0x28], rdx
0x14128239e: lea rcx, [rsp + 0x20]
0x1412823a3: call 0x1406868b0
0x1412823a8: nop 
0x1412823a9: add rsp, 0x190
0x1412823b0: pop rbx
0x1412823b1: pop rbp
0x1412823b2: pop rdi
0x1412823b3: pop rsi
0x1412823b4: pop r12
0x1412823b6: pop r14
0x1412823b8: pop r15
0x1412823ba: ret 
