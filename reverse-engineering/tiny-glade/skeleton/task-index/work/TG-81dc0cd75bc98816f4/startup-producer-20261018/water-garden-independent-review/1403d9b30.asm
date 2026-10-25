1403d9b30: 55 push rbp
1403d9b31: 4157 push r15
1403d9b33: 4156 push r14
1403d9b35: 4155 push r13
1403d9b37: 4154 push r12
1403d9b39: 56 push rsi
1403d9b3a: 57 push rdi
1403d9b3b: 53 push rbx
1403d9b3c: 4881ecb8030000 sub rsp, 0x3b8
1403d9b43: 488dac2480000000 lea rbp, [rsp + 0x80]
1403d9b4b: 48c78530030000feffffff mov qword ptr [rbp + 0x330], 0xfffffffffffffffe
1403d9b56: 4489c3 mov ebx, r8d
1403d9b59: 4889d6 mov rsi, rdx
1403d9b5c: 4889cf mov rdi, rcx
1403d9b5f: 488d4a38 lea rcx, [rdx + 0x38]
1403d9b63: 48ba9a25aa8862fa9f97 movabs rdx, 0x979ffa6288aa259a
1403d9b6d: 49b89d44ed465b1d7909 movabs r8, 0x9791d5b46ed449d
1403d9b77: e8743bd0ff call 0x1400dd6f0
1403d9b7c: 4885c0 test rax, rax
1403d9b7f: 0f84c4020000 je 0x1403d9e49
1403d9b85: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
1403d9b8c: 0f86b7020000 jbe 0x1403d9e49
1403d9b92: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
1403d9b99: 488b04d0 mov rax, qword ptr [rax + rdx*8]
1403d9b9d: 4885c0 test rax, rax
1403d9ba0: 0f84a3020000 je 0x1403d9e49
1403d9ba6: 4c8bbee0010000 mov r15, qword ptr [rsi + 0x1e0]
1403d9bad: 48f7d0 not rax
1403d9bb0: 488d0440 lea rax, [rax + rax*2]
1403d9bb4: 48c1e005 shl rax, 5
1403d9bb8: 49837c072000 cmp qword ptr [r15 + rax + 0x20], 0
1403d9bbe: 0f8485020000 je 0x1403d9e49
1403d9bc4: 4901c7 add r15, rax
1403d9bc7: 4d8b7710 mov r14, qword ptr [r15 + 0x10]
1403d9bcb: 448ba6b0050000 mov r12d, dword ptr [rsi + 0x5b0]
1403d9bd2: 4c89f1 mov rcx, r14
1403d9bd5: e8a6412a00 call 0x14067dd80
1403d9bda: 80b89a02000000 cmp byte ptr [rax + 0x29a], 0
1403d9be1: 0f844d020000 je 0x1403d9e34
1403d9be7: 488d8d48010000 lea rcx, [rbp + 0x148]
1403d9bee: 89da mov edx, ebx
1403d9bf0: e89bfc6700 call 0x140a59890
1403d9bf5: 8b8550010000 mov eax, dword ptr [rbp + 0x150]
1403d9bfb: 89851c030000 mov dword ptr [rbp + 0x31c], eax
1403d9c01: 83f84c cmp eax, 0x4c
1403d9c04: 7417 je 0x1403d9c1d
1403d9c06: 83f861 cmp eax, 0x61
1403d9c09: 0f8546020000 jne 0x1403d9e55
1403d9c0f: b807000000 mov eax, 7
1403d9c14: 488d0dfd995f02 lea rcx, [rip + 0x25f99fd]
1403d9c1b: eb0c jmp 0x1403d9c29
1403d9c1d: b804000000 mov eax, 4
1403d9c22: 488d0df6995f02 lea rcx, [rip + 0x25f99f6]
1403d9c29: 48898df0020000 mov qword ptr [rbp + 0x2f0], rcx
1403d9c30: 488985f8020000 mov qword ptr [rbp + 0x2f8], rax
1403d9c37: 4589675c mov dword ptr [r15 + 0x5c], r12d
1403d9c3b: 4c89f1 mov rcx, r14
1403d9c3e: e83d412a00 call 0x14067dd80
1403d9c43: 66c7809a0200000001 mov word ptr [rax + 0x29a], 0x100
1403d9c4c: 4c8dbdf0020000 lea r15, [rbp + 0x2f0]
1403d9c53: 4c89bd08030000 mov qword ptr [rbp + 0x308], r15
1403d9c5a: 4c8d252f02d3ff lea r12, [rip - 0x2cfdd1]
1403d9c61: 4c89a510030000 mov qword ptr [rbp + 0x310], r12
1403d9c68: 488d05d1995f02 lea rax, [rip + 0x25f99d1]
1403d9c6f: 48898548010000 mov qword ptr [rbp + 0x148], rax
1403d9c76: 48c7855001000001000000 mov qword ptr [rbp + 0x150], 1
1403d9c81: 4c8dad08030000 lea r13, [rbp + 0x308]
1403d9c88: 4c89ad58010000 mov qword ptr [rbp + 0x158], r13
1403d9c8f: 48c7856001000001000000 mov qword ptr [rbp + 0x160], 1
1403d9c9a: 48c7856801000000000000 mov qword ptr [rbp + 0x168], 0
1403d9ca5: 4c8d75b8 lea r14, [rbp - 0x48]
1403d9ca9: 488d9548010000 lea rdx, [rbp + 0x148]
1403d9cb0: 4c89f1 mov rcx, r14
1403d9cb3: e8d8029a00 call 0x140d79f90
1403d9cb8: 48897c2428 mov qword ptr [rsp + 0x28], rdi
1403d9cbd: 4889742420 mov qword ptr [rsp + 0x20], rsi
1403d9cc2: 488d1587995f02 lea rdx, [rip + 0x25f9987]
1403d9cc9: 488db548010000 lea rsi, [rbp + 0x148]
1403d9cd0: 41b811000000 mov r8d, 0x11
1403d9cd6: 4889f1 mov rcx, rsi
1403d9cd9: 4d89f1 mov r9, r14
1403d9cdc: e8bfd91a00 call 0x1405876a0
1403d9ce1: 488d4db8 lea rcx, [rbp - 0x48]
1403d9ce5: 49b8bbd5b0367191522f movabs r8, 0x2f52917136b0d5bb
1403d9cef: 4889f2 mov rdx, rsi
1403d9cf2: e869f21a00 call 0x140588f60
1403d9cf7: 4c89bd08030000 mov qword ptr [rbp + 0x308], r15
1403d9cfe: 4c89a510030000 mov qword ptr [rbp + 0x310], r12
1403d9d05: 488d0564995f02 lea rax, [rip + 0x25f9964]
1403d9d0c: 48898548010000 mov qword ptr [rbp + 0x148], rax
1403d9d13: 48c7855001000001000000 mov qword ptr [rbp + 0x150], 1
1403d9d1e: 4c89ad58010000 mov qword ptr [rbp + 0x158], r13
1403d9d25: 48c7856001000001000000 mov qword ptr [rbp + 0x160], 1
1403d9d30: 48c7856801000000000000 mov qword ptr [rbp + 0x168], 0
1403d9d3b: 488d8dd8020000 lea rcx, [rbp + 0x2d8]
1403d9d42: 4889f2 mov rdx, rsi
1403d9d45: e846029a00 call 0x140d79f90
1403d9d4a: 488b85d8020000 mov rax, qword ptr [rbp + 0x2d8]
1403d9d51: 48898520030000 mov qword ptr [rbp + 0x320], rax
1403d9d58: 488b8de0020000 mov rcx, qword ptr [rbp + 0x2e0]
1403d9d5f: 488b95e8020000 mov rdx, qword ptr [rbp + 0x2e8]
1403d9d66: c6852f03000001 mov byte ptr [rbp + 0x32f], 1
1403d9d6d: 48898d00030000 mov qword ptr [rbp + 0x300], rcx
1403d9d74: e8e7d88a00 call 0x140c87660
1403d9d79: 889d08030000 mov byte ptr [rbp + 0x308], bl
1403d9d7f: c7854801000002000000 mov dword ptr [rbp + 0x148], 2
1403d9d89: 4889742420 mov qword ptr [rsp + 0x20], rsi
1403d9d8e: 488d8d08030000 lea rcx, [rbp + 0x308]
1403d9d95: 488d55b8 lea rdx, [rbp - 0x48]
1403d9d99: 4989c0 mov r8, rax
1403d9d9c: 41b132 mov r9b, 0x32
1403d9d9f: e80ca0f8ff call 0x140363db0
1403d9da4: 488d8d48010000 lea rcx, [rbp + 0x148]
1403d9dab: 488d55b8 lea rdx, [rbp - 0x48]
1403d9daf: 41b890010000 mov r8d, 0x190
1403d9db5: e8469e5402 call 0x142923c00
1403d9dba: 80bd7e02000008 cmp byte ptr [rbp + 0x27e], 8
1403d9dc1: 7428 je 0x1403d9deb
1403d9dc3: 488d8d58020000 lea rcx, [rbp + 0x258]
1403d9dca: bae8000000 mov edx, 0xe8
1403d9dcf: 48039580020000 add rdx, qword ptr [rbp + 0x280]
1403d9dd6: 49b8b9dc726d287561b6 movabs r8, 0xb66175286d72dcb9
1403d9de0: 41b900020000 mov r9d, 0x200
1403d9de6: e885570b00 call 0x14048f570
1403d9deb: 488d75b8 lea rsi, [rbp - 0x48]
1403d9def: 488d9548010000 lea rdx, [rbp + 0x148]
1403d9df6: 41b890010000 mov r8d, 0x190
1403d9dfc: 4889f1 mov rcx, rsi
1403d9dff: e8fc9d5402 call 0x142923c00
1403d9e04: 48ba8000000000020000 movabs rdx, 0x20000000080
1403d9e0e: 4889f1 mov rcx, rsi
1403d9e11: e86a4e5900 call 0x14096ec80
1403d9e16: 488b9520030000 mov rdx, qword ptr [rbp + 0x320]
1403d9e1d: 4885d2 test rdx, rdx
1403d9e20: 7412 je 0x1403d9e34
1403d9e22: 41b801000000 mov r8d, 1
1403d9e28: 488b8d00030000 mov rcx, qword ptr [rbp + 0x300]
1403d9e2f: e8ec9d2300 call 0x140613c20
1403d9e34: 90 nop 
1403d9e35: 4881c4b8030000 add rsp, 0x3b8
1403d9e3c: 5b pop rbx
1403d9e3d: 5f pop rdi
1403d9e3e: 5e pop rsi
1403d9e3f: 415c pop r12
1403d9e41: 415d pop r13
1403d9e43: 415e pop r14
1403d9e45: 415f pop r15
1403d9e47: 5d pop rbp
1403d9e48: c3 ret 
1403d9e49: 488d0db0975f02 lea rcx, [rip + 0x25f97b0]
1403d9e50: e8bbf44f02 call 0x1428d9310
1403d9e55: 488d851c030000 lea rax, [rbp + 0x31c]
1403d9e5c: 488945b8 mov qword ptr [rbp - 0x48], rax
1403d9e60: 488d0519df8a00 lea rax, [rip + 0x8adf19]
1403d9e67: 488945c0 mov qword ptr [rbp - 0x40], rax
1403d9e6b: 488d0526985f02 lea rax, [rip + 0x25f9826]
1403d9e72: 48898548010000 mov qword ptr [rbp + 0x148], rax
1403d9e79: 48c7855001000001000000 mov qword ptr [rbp + 0x150], 1
1403d9e84: 48c7856801000000000000 mov qword ptr [rbp + 0x168], 0
1403d9e8f: 488d45b8 lea rax, [rbp - 0x48]
1403d9e93: 48898558010000 mov qword ptr [rbp + 0x158], rax
1403d9e9a: 48c7856001000001000000 mov qword ptr [rbp + 0x160], 1
1403d9ea5: 488d15fc975f02 lea rdx, [rip + 0x25f97fc]
1403d9eac: 488d8d48010000 lea rcx, [rbp + 0x148]
1403d9eb3: e8d8f44f02 call 0x1428d9390
1403d9eb8: cc int3 
1403d9eb9: 0f1f8000000000 nop dword ptr [rax]
1403d9ec0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1403d9ec5: 55 push rbp
1403d9ec6: 4157 push r15
1403d9ec8: 4156 push r14
1403d9eca: 4155 push r13
1403d9ecc: 4154 push r12
1403d9ece: 56 push rsi
1403d9ecf: 57 push rdi
1403d9ed0: 53 push rbx
1403d9ed1: 4883ec38 sub rsp, 0x38
1403d9ed5: 488daa80000000 lea rbp, [rdx + 0x80]
1403d9edc: 488d4db8 lea rcx, [rbp - 0x48]
1403d9ee0: e83bf62200 call 0x140609520
1403d9ee5: 90 nop 
1403d9ee6: 4883c438 add rsp, 0x38
1403d9eea: 5b pop rbx
1403d9eeb: 5f pop rdi
1403d9eec: 5e pop rsi
1403d9eed: 415c pop r12
1403d9eef: 415d pop r13
1403d9ef1: 415e pop r14
1403d9ef3: 415f pop r15
1403d9ef5: 5d pop rbp
1403d9ef6: c3 ret 
1403d9ef7: 660f1f840000000000 nop word ptr [rax + rax]
1403d9f00: 4889542410 mov qword ptr [rsp + 0x10], rdx
1403d9f05: 55 push rbp
1403d9f06: 4157 push r15
1403d9f08: 4156 push r14
1403d9f0a: 4155 push r13
1403d9f0c: 4154 push r12
1403d9f0e: 56 push rsi
1403d9f0f: 57 push rdi
1403d9f10: 53 push rbx
1403d9f11: 4883ec38 sub rsp, 0x38
1403d9f15: 488daa80000000 lea rbp, [rdx + 0x80]
1403d9f1c: 4883bd2003000000 cmp qword ptr [rbp + 0x320], 0
1403d9f24: 7419 je 0x1403d9f3f
1403d9f26: 41b801000000 mov r8d, 1
1403d9f2c: 488b8d00030000 mov rcx, qword ptr [rbp + 0x300]
1403d9f33: 488b9520030000 mov rdx, qword ptr [rbp + 0x320]
1403d9f3a: e8e19c2300 call 0x140613c20
1403d9f3f: 90 nop 
1403d9f40: 4883c438 add rsp, 0x38
1403d9f44: 5b pop rbx
1403d9f45: 5f pop rdi
1403d9f46: 5e pop rsi
1403d9f47: 415c pop r12
1403d9f49: 415d pop r13
1403d9f4b: 415e pop r14
1403d9f4d: 415f pop r15
1403d9f4f: 5d pop rbp
1403d9f50: c3 ret 
1403d9f51: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1403d9f60: 4889542410 mov qword ptr [rsp + 0x10], rdx
1403d9f65: 55 push rbp
1403d9f66: 4157 push r15
1403d9f68: 4156 push r14
1403d9f6a: 4155 push r13
1403d9f6c: 4154 push r12
1403d9f6e: 56 push rsi
1403d9f6f: 57 push rdi
1403d9f70: 53 push rbx
1403d9f71: 4883ec38 sub rsp, 0x38
1403d9f75: 488daa80000000 lea rbp, [rdx + 0x80]
1403d9f7c: 80bd2f03000000 cmp byte ptr [rbp + 0x32f], 0
1403d9f83: 741b je 0x1403d9fa0
1403d9f85: 488d4db8 lea rcx, [rbp - 0x48]
1403d9f89: e892f52200 call 0x140609520
1403d9f8e: 90 nop 
1403d9f8f: 4883c438 add rsp, 0x38
1403d9f93: 5b pop rbx
1403d9f94: 5f pop rdi
1403d9f95: 5e pop rsi
1403d9f96: 415c pop r12
1403d9f98: 415d pop r13
1403d9f9a: 415e pop r14
1403d9f9c: 415f pop r15
1403d9f9e: 5d pop rbp
1403d9f9f: c3 ret 
1403d9fa0: 4883c438 add rsp, 0x38
1403d9fa4: 5b pop rbx
1403d9fa5: 5f pop rdi
1403d9fa6: 5e pop rsi
1403d9fa7: 415c pop r12
1403d9fa9: 415d pop r13
1403d9fab: 415e pop r14
1403d9fad: 415f pop r15
1403d9faf: 5d pop rbp
1403d9fb0: c3 ret 
1403d9fb1: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1403d9fc0: 4889542410 mov qword ptr [rsp + 0x10], rdx
1403d9fc5: 55 push rbp
1403d9fc6: 4157 push r15
1403d9fc8: 4156 push r14
1403d9fca: 4155 push r13
1403d9fcc: 4154 push r12
1403d9fce: 56 push rsi
1403d9fcf: 57 push rdi
1403d9fd0: 53 push rbx
1403d9fd1: 4883ec38 sub rsp, 0x38
1403d9fd5: 488daa80000000 lea rbp, [rdx + 0x80]
1403d9fdc: c6852f03000000 mov byte ptr [rbp + 0x32f], 0
1403d9fe3: 488d4db8 lea rcx, [rbp - 0x48]
1403d9fe7: e834f52200 call 0x140609520
1403d9fec: c6852f03000000 mov byte ptr [rbp + 0x32f], 0
1403d9ff3: 4883c438 add rsp, 0x38
1403d9ff7: 5b pop rbx
1403d9ff8: 5f pop rdi
1403d9ff9: 5e pop rsi
1403d9ffa: 415c pop r12
1403d9ffc: 415d pop r13
1403d9ffe: 415e pop r14
1403da000: 415f pop r15
1403da002: 5d pop rbp
1403da003: c3 ret 
1403da004: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1403da010: 4889542410 mov qword ptr [rsp + 0x10], rdx
1403da015: 55 push rbp
1403da016: 4157 push r15
1403da018: 4156 push r14
1403da01a: 4155 push r13
1403da01c: 4154 push r12
1403da01e: 56 push rsi
1403da01f: 57 push rdi
1403da020: 53 push rbx
1403da021: 4883ec38 sub rsp, 0x38
1403da025: 488daa80000000 lea rbp, [rdx + 0x80]
1403da02c: 488d8d48010000 lea rcx, [rbp + 0x148]
1403da033: e8e8f42200 call 0x140609520
1403da038: 90 nop 
1403da039: 4883c438 add rsp, 0x38
1403da03d: 5b pop rbx
1403da03e: 5f pop rdi
1403da03f: 5e pop rsi
1403da040: 415c pop r12
1403da042: 415d pop r13
1403da044: 415e pop r14
1403da046: 415f pop r15
1403da048: 5d pop rbp
1403da049: c3 ret 
