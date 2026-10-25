140a97a30: 4157 push r15
140a97a32: 4156 push r14
140a97a34: 56 push rsi
140a97a35: 57 push rdi
140a97a36: 55 push rbp
140a97a37: 53 push rbx
140a97a38: 4883ec38 sub rsp, 0x38
140a97a3c: 4c89ce mov rsi, r9
140a97a3f: 488b19 mov rbx, qword ptr [rcx]
140a97a42: 488b0b mov rcx, qword ptr [rbx]
140a97a45: 4883c110 add rcx, 0x10
140a97a49: 498b38 mov rdi, qword ptr [r8]
140a97a4c: 4c8b32 mov r14, qword ptr [rdx]
140a97a4f: 4c89742430 mov qword ptr [rsp + 0x30], r14
140a97a54: 48897c2428 mov qword ptr [rsp + 0x28], rdi
140a97a59: 48c744242013000000 mov qword ptr [rsp + 0x20], 0x13
140a97a62: 488d151f370402 lea rdx, [rip + 0x204371f]
140a97a69: 4c8d0d41370402 lea r9, [rip + 0x2043741]
140a97a70: 41b829000000 mov r8d, 0x29
140a97a76: e8c5830a00 call 0x140b3fe40
140a97a7b: 8b6e1c mov ebp, dword ptr [rsi + 0x1c]
140a97a7e: 4c8b7e10 mov r15, qword ptr [rsi + 0x10]
140a97a82: 41892f mov dword ptr [r15], ebp
140a97a85: 488b36 mov rsi, qword ptr [rsi]
140a97a88: 48c7461001000000 mov qword ptr [rsi + 0x10], 1
140a97a90: 48894618 mov qword ptr [rsi + 0x18], rax
140a97a94: 488b0b mov rcx, qword ptr [rbx]
140a97a97: 4883c110 add rcx, 0x10
140a97a9b: 4c89742430 mov qword ptr [rsp + 0x30], r14
140a97aa0: 48897c2428 mov qword ptr [rsp + 0x28], rdi
140a97aa5: 48c744242014000000 mov qword ptr [rsp + 0x20], 0x14
140a97aae: 488d150f370402 lea rdx, [rip + 0x204370f]
140a97ab5: 4c8d0d32370402 lea r9, [rip + 0x2043732]
140a97abc: 41b82a000000 mov r8d, 0x2a
140a97ac2: e879830a00 call 0x140b3fe40
140a97ac7: 41892f mov dword ptr [r15], ebp
140a97aca: 48c7463001000000 mov qword ptr [rsi + 0x30], 1
140a97ad2: 48894638 mov qword ptr [rsi + 0x38], rax
140a97ad6: 488b0b mov rcx, qword ptr [rbx]
140a97ad9: 4883c110 add rcx, 0x10
140a97add: 4c89742430 mov qword ptr [rsp + 0x30], r14
140a97ae2: 48897c2428 mov qword ptr [rsp + 0x28], rdi
140a97ae7: 48c744242013000000 mov qword ptr [rsp + 0x20], 0x13
140a97af0: 488d150b370402 lea rdx, [rip + 0x204370b]
140a97af7: 4c8d0d2d370402 lea r9, [rip + 0x204372d]
140a97afe: 41b829000000 mov r8d, 0x29
140a97b04: e837830a00 call 0x140b3fe40
140a97b09: 41892f mov dword ptr [r15], ebp
140a97b0c: 48c70601000000 mov qword ptr [rsi], 1
140a97b13: 48894608 mov qword ptr [rsi + 8], rax
140a97b17: 488b0b mov rcx, qword ptr [rbx]
140a97b1a: 4883c110 add rcx, 0x10
140a97b1e: 4c89742430 mov qword ptr [rsp + 0x30], r14
140a97b23: 48897c2428 mov qword ptr [rsp + 0x28], rdi
140a97b28: 48c744242013000000 mov qword ptr [rsp + 0x20], 0x13
140a97b31: 488d1506370402 lea rdx, [rip + 0x2043706]
140a97b38: 4c8d0d28370402 lea r9, [rip + 0x2043728]
140a97b3f: 41b829000000 mov r8d, 0x29
140a97b45: e8f6820a00 call 0x140b3fe40
140a97b4a: 41892f mov dword ptr [r15], ebp
140a97b4d: 48c7462001000000 mov qword ptr [rsi + 0x20], 1
140a97b55: 48894628 mov qword ptr [rsi + 0x28], rax
140a97b59: 4883c438 add rsp, 0x38
140a97b5d: 5b pop rbx
140a97b5e: 5d pop rbp
140a97b5f: 5f pop rdi
140a97b60: 5e pop rsi
140a97b61: 415e pop r14
140a97b63: 415f pop r15
140a97b65: c3 ret 
