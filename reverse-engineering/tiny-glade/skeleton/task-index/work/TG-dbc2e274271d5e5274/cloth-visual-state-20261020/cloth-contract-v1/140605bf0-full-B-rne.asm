140605bf0: 4157 push r15
140605bf2: 4156 push r14
140605bf4: 4154 push r12
140605bf6: 56 push rsi
140605bf7: 57 push rdi
140605bf8: 53 push rbx
140605bf9: 4883ec28 sub rsp, 0x28
140605bfd: 4889ce mov rsi, rcx
140605c00: 488b7920 mov rdi, qword ptr [rcx + 0x20]
140605c04: 4885ff test rdi, rdi
140605c07: 0f84d5000000 je 0x140605ce2
140605c0d: 488b5e30 mov rbx, qword ptr [rsi + 0x30]
140605c11: 4885db test rbx, rbx
140605c14: 0f8496000000 je 0x140605cb0
140605c1a: 4c8b7618 mov r14, qword ptr [rsi + 0x18]
140605c1e: 66410f6f06 movdqa xmm0, xmmword ptr [r14]
140605c23: 66440fd7e0 pmovmskb r12d, xmm0
140605c28: 41f7d4 not r12d
140605c2b: 4d8d7e10 lea r15, [r14 + 0x10]
140605c2f: eb22 jmp 0x140605c53 ; unknown
140605c31: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140605c40: 48ffcb dec rbx
140605c43: 418d4424ff lea eax, [r12 - 1]
140605c48: 4421e0 and eax, r12d
140605c4b: 4189c4 mov r12d, eax
140605c4e: 4885db test rbx, rbx
140605c51: 745d je 0x140605cb0
140605c53: 664585e4 test r12w, r12w
140605c57: 7528 jne 0x140605c81
140605c59: 0f1f8000000000 nop dword ptr [rax]
140605c60: 66410f6f07 movdqa xmm0, xmmword ptr [r15]
140605c65: 66440fd7e0 pmovmskb r12d, xmm0
140605c6a: 4981c680fdffff add r14, -0x280
140605c71: 4983c710 add r15, 0x10
140605c75: 4181fcffff0000 cmp r12d, 0xffff
140605c7c: 74e2 je 0x140605c60
140605c7e: 41f7d4 not r12d
140605c81: f3410fbcc4 tzcnt eax, r12d
140605c86: 48f7d8 neg rax
140605c89: 488d0480 lea rax, [rax + rax*4]
140605c8d: 498b54c6e8 mov rdx, qword ptr [r14 + rax*8 - 0x18]
140605c92: 4885d2 test rdx, rdx
140605c95: 74a9 je 0x140605c40
140605c97: 498d04c6 lea rax, [r14 + rax*8]
140605c9b: 488b48f0 mov rcx, qword ptr [rax - 0x10]
140605c9f: 48c1e203 shl rdx, 3
140605ca3: 41b808000000 mov r8d, 8
140605ca9: e872df0000 call 0x140613c20 ; unknown
140605cae: eb90 jmp 0x140605c40 ; unknown
140605cb0: 488d4701 lea rax, [rdi + 1]
140605cb4: b928000000 mov ecx, 0x28
140605cb9: 48f7e1 mul rcx
140605cbc: 4883c00f add rax, 0xf
140605cc0: 4883e0f0 and rax, 0xfffffffffffffff0
140605cc4: 4801c7 add rdi, rax
140605cc7: 4883c711 add rdi, 0x11
140605ccb: 7415 je 0x140605ce2
140605ccd: 488b4e18 mov rcx, qword ptr [rsi + 0x18]
140605cd1: 4829c1 sub rcx, rax
140605cd4: 41b810000000 mov r8d, 0x10
140605cda: 4889fa mov rdx, rdi
140605cdd: e83edf0000 call 0x140613c20 ; unknown
140605ce2: 488b7e50 mov rdi, qword ptr [rsi + 0x50]
140605ce6: 4885ff test rdi, rdi
140605ce9: 0f84ca000000 je 0x140605db9
140605cef: 488b5e60 mov rbx, qword ptr [rsi + 0x60]
140605cf3: 4885db test rbx, rbx
140605cf6: 0f8491000000 je 0x140605d8d
140605cfc: 4c8b7648 mov r14, qword ptr [rsi + 0x48]
140605d00: 66410f6f06 movdqa xmm0, xmmword ptr [r14]
140605d05: 66440fd7e0 pmovmskb r12d, xmm0
140605d0a: 41f7d4 not r12d
140605d0d: 4d8d7e10 lea r15, [r14 + 0x10]
140605d11: eb20 jmp 0x140605d33 ; unknown
140605d13: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140605d20: 48ffcb dec rbx
140605d23: 418d4424ff lea eax, [r12 - 1]
140605d28: 4421e0 and eax, r12d
140605d2b: 4189c4 mov r12d, eax
140605d2e: 4885db test rbx, rbx
140605d31: 745a je 0x140605d8d
140605d33: 664585e4 test r12w, r12w
140605d37: 7528 jne 0x140605d61
140605d39: 0f1f8000000000 nop dword ptr [rax]
140605d40: 66410f6f07 movdqa xmm0, xmmword ptr [r15]
140605d45: 66440fd7e0 pmovmskb r12d, xmm0
140605d4a: 4981c600feffff add r14, -0x200
140605d51: 4983c710 add r15, 0x10
140605d55: 4181fcffff0000 cmp r12d, 0xffff
140605d5c: 74e2 je 0x140605d40
140605d5e: 41f7d4 not r12d
140605d61: f3410fbccc tzcnt ecx, r12d
140605d66: c1e105 shl ecx, 5
140605d69: 4c89f0 mov rax, r14
140605d6c: 4829c8 sub rax, rcx
140605d6f: 488b50e8 mov rdx, qword ptr [rax - 0x18]
140605d73: 4885d2 test rdx, rdx
140605d76: 74a8 je 0x140605d20
140605d78: 488b48f0 mov rcx, qword ptr [rax - 0x10]
140605d7c: 48c1e203 shl rdx, 3
140605d80: 41b808000000 mov r8d, 8
140605d86: e895de0000 call 0x140613c20 ; unknown
140605d8b: eb93 jmp 0x140605d20 ; unknown
140605d8d: 4889f8 mov rax, rdi
140605d90: 48c1e005 shl rax, 5
140605d94: 4801c7 add rdi, rax
140605d97: 4883c731 add rdi, 0x31
140605d9b: 741c je 0x140605db9
140605d9d: 488b4e48 mov rcx, qword ptr [rsi + 0x48]
140605da1: 48f7d8 neg rax
140605da4: 4801c1 add rcx, rax
140605da7: 4883c1e0 add rcx, -0x20
140605dab: 41b810000000 mov r8d, 0x10
140605db1: 4889fa mov rdx, rdi
140605db4: e867de0000 call 0x140613c20 ; unknown
140605db9: 488b06 mov rax, qword ptr [rsi]
140605dbc: 4885c0 test rax, rax
140605dbf: 7424 je 0x140605de5
140605dc1: 488b4e08 mov rcx, qword ptr [rsi + 8]
140605dc5: 48c1e004 shl rax, 4
140605dc9: 488d1480 lea rdx, [rax + rax*4]
140605dcd: 41b810000000 mov r8d, 0x10
140605dd3: 4883c428 add rsp, 0x28
140605dd7: 5b pop rbx
140605dd8: 5f pop rdi
140605dd9: 5e pop rsi
140605dda: 415c pop r12
140605ddc: 415e pop r14
140605dde: 415f pop r15
140605de0: e93bde0000 jmp 0x140613c20 ; unknown
140605de5: 90 nop 
140605de6: 4883c428 add rsp, 0x28
140605dea: 5b pop rbx
140605deb: 5f pop rdi
140605dec: 5e pop rsi
140605ded: 415c pop r12
140605def: 415e pop r14
140605df1: 415f pop r15
140605df3: c3 ret 
