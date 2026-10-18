140ceafa0: mov eax, edx
140ceafa2: mov r8, qword ptr [rcx + 0x10]
140ceafa6: cmp r8, rax
140ceafa9: jbe 0x140ceafbc
140ceafab: mov rcx, qword ptr [rcx + 8]
140ceafaf: lea rax, [rax + rax*4]
140ceafb3: mov eax, dword ptr [rcx + rax*4]
140ceafb6: shl rax, 0x20
140ceafba: jmp 0x140ceafe2
140ceafbc: mov rcx, qword ptr [rcx + 0x30]
140ceafc0: mov r9, rcx
140ceafc3: dec r9
140ceafc6: sets r9b
140ceafca: sub r8, rcx
140ceafcd: cmp r8, rax
140ceafd0: seta al
140ceafd3: test r9b, al
140ceafd6: je 0x140ceaff6
140ceafd8: movabs rax, 0x100000000
140ceafe2: xor rax, rdx
140ceafe5: movabs rcx, 0x7fffffff00000000
140ceafef: test rax, rcx
140ceaff2: sete al
140ceaff5: ret 
140ceaff6: xor eax, eax
140ceaff8: ret 
140ceaff9: int3 
140ceaffa: int3 
140ceaffb: int3 
140ceaffc: int3 
140ceaffd: int3 
140ceaffe: int3 
140ceafff: int3 
140ceb000: push rbp
140ceb001: push r15
140ceb003: push r14
140ceb005: push r13
140ceb007: push r12
140ceb009: push rsi
140ceb00a: push rdi
140ceb00b: push rbx
140ceb00c: sub rsp, 0x88
140ceb013: lea rbp, [rsp + 0x80]
