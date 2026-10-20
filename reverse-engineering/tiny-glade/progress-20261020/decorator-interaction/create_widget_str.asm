0x1412823c0: push r15
0x1412823c2: push r14
0x1412823c4: push r12
0x1412823c6: push rsi
0x1412823c7: push rdi
0x1412823c8: push rbp
0x1412823c9: push rbx
0x1412823ca: sub rsp, 0x190
0x1412823d1: mov rsi, r8
0x1412823d4: mov rdi, rdx
0x1412823d7: mov rbx, rcx
0x1412823da: mov r14, qword ptr [rsp + 0x1f8]
0x1412823e2: cmp byte ptr [rsp + 0x1f0], 0x3f
0x1412823ea: sete bpl
0x1412823ee: shl bpl, 3
0x1412823f2: movss dword ptr [rsp + 0x34], xmm3
0x1412823f8: mov byte ptr [rsp + 0x30], 5
0x1412823fd: mov byte ptr [rsp + 0x20], 0
0x141282402: lea r15, [rsp + 0xe0]
0x14128240a: lea rdx, [rsp + 0x30]
0x14128240f: lea r8, [rsp + 0x20]
0x141282414: mov rcx, r15
0x141282417: call 0x1409a8e10
0x14128241c: or bpl, byte ptr [rsp + 0x189]
0x141282424: or bpl, 2
0x141282428: mov byte ptr [rsp + 0x189], bpl
0x141282430: lea r12, [rsp + 0x30]
0x141282435: mov rcx, r12
0x141282438: mov rdx, r15
0x14128243b: mov r8, r14
0x14128243e: call 0x1409a8e80
0x141282443: mov rcx, rbx
0x141282446: mov rdx, rdi
0x141282449: mov r8, rsi
0x14128244c: mov r9, r12
0x14128244f: call 0x1409236d0
0x141282454: mov qword ptr [rsp + 0x20], rax
0x141282459: mov qword ptr [rsp + 0x28], rdx
0x14128245e: lea rcx, [rsp + 0x20]
0x141282463: call 0x1406868b0
0x141282468: nop 
0x141282469: add rsp, 0x190
0x141282470: pop rbx
0x141282471: pop rbp
0x141282472: pop rdi
0x141282473: pop rsi
0x141282474: pop r12
0x141282476: pop r14
0x141282478: pop r15
0x14128247a: ret 
