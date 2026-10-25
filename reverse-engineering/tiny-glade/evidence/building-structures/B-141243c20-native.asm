141243c20 push rbp
141243c21 push r15
141243c23 push r14
141243c25 push r13
141243c27 push r12
141243c29 push rsi
141243c2a push rdi
141243c2b push rbx
141243c2c sub rsp, 0x128
141243c33 lea rbp, [rsp + 0x80]
141243c3b movaps xmmword ptr [rbp + 0x90], xmm7
141243c42 movaps xmmword ptr [rbp + 0x80], xmm6
141243c49 mov qword ptr [rbp + 0x78], 0xfffffffffffffffe
141243c51 mov r10, qword ptr [r9]
141243c54 mov r11, qword ptr [r9 + 8]
141243c58 cmp r11, r10
141243c5b je 0x1412440f2
141243c61 cmp r11, -1
141243c65 je 0x141244114
141243c6b lea rax, [r11 + 1]
141243c6f cmp byte ptr [r9 + 0x10], 0
141243c74 mov r15, r10
141243c77 cmovne r15, rax
141243c7b mov rsi, rax
141243c7e sub rsi, r15
141243c81 jb 0x141244120
141243c87 mov r9, qword ptr [rcx + 8]
141243c8b cmp r11, r9
141243c8e jae 0x141244132
141243c94 mov r9, qword ptr [rcx + 0x18]
141243c98 cmp r11, r9
141243c9b jae 0x141244144
141243ca1 mov r9, qword ptr [rcx + 0x28]
141243ca5 cmp r11, r9
141243ca8 jae 0x141244156
141243cae mov r9d, dword ptr [rbp + 0x118]
141243cb5 mov r14, qword ptr [rcx]
141243cb8 lea rax, [r15 + r15*2]
141243cbc lea r12, [rax*4]
141243cc4 add r12, qword ptr [rcx + 0x10]
141243cc8 mov rcx, qword ptr [rcx + 0x20]
141243ccc mov qword ptr [rbp + 0x18], rdx
141243cd0 mov dword ptr [rbp + 0x48], r9d
141243cd4 mov dword ptr [rbp + 0x44], r10d
141243cd8 cmp rsi, 2
141243cdc jb 0x1412440f2
141243ce2 mov rdi, r8
141243ce5 lea rdx, [rcx + rax*4]
141243ce9 mov dword ptr [rsp + 0x20], 0x3f800000
141243cf1 lea rcx, [rbp - 0x28]
141243cf5 movss xmm3, dword ptr [rip + 0x178936b]
141243cfd mov r8, rsi
141243d00 call 0x141fe0b90
141243d05 mov rax, qword ptr [rbp - 0x28]
141243d09 mov qword ptr [rbp + 0x38], rax
141243d0d mov rax, qword ptr [rbp - 0x20]
141243d11 mov qword ptr [rbp + 0x60], rax
141243d15 mov rbx, qword ptr [rbp - 0x18]
141243d19 mov rax, qword ptr [rbp - 0x10]
141243d1d test rax, rax
141243d20 je 0x141243d39
141243d22 mov rcx, qword ptr [rbp - 8]
141243d26 shl rax, 2
141243d2a lea rdx, [rax + rax*2]
141243d2e mov r8d, 4
141243d34 call 0x140613c20
141243d39 mov byte ptr [rbp + 0x77], 0
141243d3d mov rcx, rdi
141243d40 call 0x140b2a770
141243d45 mov dword ptr [rbp + 0x4c], eax
141243d48 test rbx, rbx
141243d4b je 0x1412440d6
141243d51 cmp rbx, 1
141243d55 je 0x1412440d6
141243d5b mov rax, qword ptr [rbp + 0x60]
141243d5f mov rcx, qword ptr [rax + 8]
141243d63 cmp rcx, -1
141243d67 je 0x141244168
141243d6d mov rax, qword ptr [rbp + 0x60]
141243d71 mov rdi, qword ptr [rax]
141243d74 lea rdx, [rcx + 1]
141243d78 mov rax, rdx
141243d7b sub rax, rdi
141243d7e jb 0x14124418d
141243d84 cmp rcx, rsi
141243d87 jae 0x141244176
141243d8d movaps xmm6, xmm0
141243d90 mov r13, rcx
141243d93 sub r13, rdi
141243d96 mov qword ptr [rbp + 0x20], r12
141243d9a jae 0x141243da5
141243d9c mov qword ptr [rbp + 0x68], rax
141243da0 xor r13d, r13d
141243da3 jmp 0x141243dcf
141243da5 cmp r13, -1
141243da9 jne 0x141243dc8
141243dab lea rcx, [rip + 0x19dba7e]
141243db2 lea r8, [rip + 0x19dbadf]
141243db9 mov edx, 0x16
141243dbe call 0x1428d9330
141243dc3 jmp 0x1412441c4
141243dc8 mov qword ptr [rbp + 0x68], rax
141243dcc inc r13
141243dcf lea r12, [r13*4]
141243dd7 mov rax, r13
141243dda shr rax, 0x3e
141243dde jne 0x1412441a9
141243de4 movabs rax, 0x7ffffffffffffffc
141243dee cmp r12, rax
141243df1 ja 0x1412441a9
141243df7 test r12, r12
141243dfa mov qword ptr [rbp - 0x30], rcx
141243dfe je 0x141243e26
141243e00 movzx eax, byte ptr [rip + 0x23f0e02]
141243e07 mov edx, 4
141243e0c mov rcx, r12
141243e0f call 0x140613c10
141243e14 test rax, rax
141243e17 je 0x14124419e
141243e1d mov r9, rax
141243e20 mov qword ptr [rbp + 0x50], r13
141243e24 jmp 0x141243e34
141243e26 mov r9d, 4
141243e2c mov qword ptr [rbp + 0x50], 0
141243e34 lea rdx, [r14 + r15*4]
141243e38 lea rax, [rdi + rdi*2]
141243e3c mov rcx, qword ptr [rbp + 0x20]
141243e40 lea r15, [rcx + rax*4]
141243e44 mov qword ptr [rbp - 0x40], rdx
141243e48 lea rdi, [rdx + rdi*4]
141243e4c cmp r13, 1
141243e50 jbe 0x141243e6e
141243e52 lea r8, [r12 - 4]
141243e57 mov r14, r9
141243e5a mov rcx, r9
141243e5d xor edx, edx
141243e5f call 0x142923c20
141243e64 lea rax, [r12 + r14]
141243e68 add rax, -4
141243e6c jmp 0x141243e79
141243e6e mov r14, r9
141243e71 mov rax, r9
141243e74 test r13, r13
141243e77 je 0x141243e7f
141243e79 mov dword ptr [rax], 0
141243e7f movsd xmm7, qword ptr [rip + 0x19db999]
141243e87 movsd qword ptr [rbp + 0x30], xmm7
141243e8c mov eax, dword ptr [rbp + 0x4c]
141243e8f mov dword ptr [rbp + 0x28], eax
141243e92 movss dword ptr [rbp + 0x2c], xmm6
141243e97 mov rax, qword ptr [rbp + 0x110]
141243e9e mov qword ptr [rbp - 0x28], rax
141243ea2 lea rax, [rbp + 0x48]
141243ea6 mov qword ptr [rbp - 0x20], rax
141243eaa lea rax, [rbp + 0x44]
141243eae mov qword ptr [rbp - 0x18], rax
141243eb2 lea rax, [rbp + 0x18]
141243eb6 mov qword ptr [rbp - 0x10], rax
141243eba mov r12, qword ptr [rbp + 0x50]
141243ebe mov qword ptr [rbp + 0x10], r12
141243ec2 mov qword ptr [rbp + 8], r14
141243ec6 lea rax, [rbp - 0x28]
141243eca mov qword ptr [rsp + 0x38], rax
141243ecf mov qword ptr [rsp + 0x30], r13
141243ed4 mov qword ptr [rsp + 0x28], r14
141243ed9 mov r8, qword ptr [rbp + 0x68]
141243edd mov qword ptr [rsp + 0x20], r8
141243ee2 lea rcx, [rbp + 0x28]
141243ee6 mov rdx, rdi
141243ee9 mov r9, r15
141243eec call 0x141233380
141243ef1 mov rax, qword ptr [rbp + 0x60]
141243ef5 lea rdi, [rax + rbx*8]
141243ef9 lea rbx, [rax + 0x10]
141243efd test r12, r12
141243f00 je 0x141243f17
141243f02 shl r12, 2
141243f06 mov r8d, 4
141243f0c mov rcx, r14
141243f0f mov rdx, r12
141243f12 call 0x140613c20
141243f17 cmp rbx, rdi
141243f1a mov r8, qword ptr [rbp - 0x30]
141243f1e je 0x1412440d6
141243f24 mov qword ptr [rbp + 0x58], 0
141243f2c mov qword ptr [rbp - 0x38], rdi
141243f30 mov r15, qword ptr [rbx]
141243f33 cmp r15, -1
141243f37 je 0x141244168
141243f3d lea rdx, [r15 + 1]
141243f41 mov r13, rdx
141243f44 sub r13, r8
141243f47 jb 0x14124418a
141243f4d cmp r15, rsi
141243f50 jae 0x141244176
141243f56 mov r14, r15
141243f59 sub r14, r8
141243f5c jae 0x141243f7b
141243f5e mov qword ptr [rbp + 0x68], rbx
141243f62 xor r14d, r14d
141243f65 lea r12, [r14*4]
141243f6d mov rax, r14
141243f70 shr rax, 0x3e
141243f74 je 0x141243fa1
141243f76 jmp 0x1412441b1
141243f7b cmp r14, -1
141243f7f je 0x141243dab
141243f85 mov qword ptr [rbp + 0x68], rbx
141243f89 inc r14
141243f8c lea r12, [r14*4]
141243f94 mov rax, r14
141243f97 shr rax, 0x3e
141243f9b jne 0x1412441b1
141243fa1 movabs rax, 0x7ffffffffffffffc
141243fab cmp r12, rax
141243fae ja 0x1412441b1
141243fb4 test r12, r12
141243fb7 je 0x14124400c
141243fb9 mov rbx, r8
141243fbc movzx eax, byte ptr [rip + 0x23f0c46]
141243fc3 mov edx, 4
141243fc8 mov rcx, r12
141243fcb call 0x140613c10
141243fd0 test rax, rax
141243fd3 je 0x14124419e
141243fd9 mov rdi, rax
141243fdc mov qword ptr [rbp + 0x50], r14
141243fe0 mov r8, rbx
141243fe3 cmp r14, 2
141243fe7 jb 0x14124401f
141243fe9 mov rbx, r8
141243fec lea r8, [r12 - 4]
141243ff1 mov rcx, rdi
141243ff4 xor edx, edx
141243ff6 call 0x142923c20
141243ffb mov r8, rbx
141243ffe lea rax, [r12 + rdi]
141244002 add rax, -4
141244006 mov rbx, qword ptr [rbp + 0x68]
14124400a jmp 0x14124402b
14124400c mov edi, 4
141244011 mov qword ptr [rbp + 0x50], 0
141244019 cmp r14, 2
14124401d jae 0x141243fe9
14124401f mov rax, rdi
141244022 test r14, r14
141244025 mov rbx, qword ptr [rbp + 0x68]
141244029 je 0x141244031
14124402b mov dword ptr [rax], 0
141244031 lea rax, [r8 + r8*2]
141244035 mov rcx, qword ptr [rbp + 0x20]
141244039 lea r9, [rcx + rax*4]
14124403d mov rax, qword ptr [rbp - 0x40]
141244041 lea rdx, [rax + r8*4]
141244045 movsd qword ptr [rbp + 0x30], xmm7
14124404a mov eax, dword ptr [rbp + 0x4c]
14124404d mov dword ptr [rbp + 0x28], eax
141244050 movss dword ptr [rbp + 0x2c], xmm6
141244055 mov rax, qword ptr [rbp + 0x110]
14124405c mov qword ptr [rbp - 0x28], rax
141244060 lea rax, [rbp + 0x48]
141244064 mov qword ptr [rbp - 0x20], rax
141244068 lea rax, [rbp + 0x44]
14124406c mov qword ptr [rbp - 0x18], rax
141244070 lea rax, [rbp + 0x18]
141244074 mov qword ptr [rbp - 0x10], rax
141244078 mov r12, qword ptr [rbp + 0x50]
14124407c mov qword ptr [rbp + 0x10], r12
141244080 mov qword ptr [rbp + 8], rdi
141244084 lea rax, [rbp - 0x28]
141244088 mov qword ptr [rsp + 0x38], rax
14124408d mov qword ptr [rsp + 0x30], r14
141244092 mov qword ptr [rsp + 0x28], rdi
141244097 mov qword ptr [rsp + 0x20], r13
14124409c lea rcx, [rbp + 0x28]
1412440a0 mov r8, r13
1412440a3 call 0x141233380
1412440a8 test r12, r12
1412440ab je 0x1412440c2
1412440ad shl r12, 2
1412440b1 mov r8d, 4
1412440b7 mov rcx, rdi
1412440ba mov rdx, r12
1412440bd call 0x140613c20
1412440c2 add rbx, 8
1412440c6 mov r8, r15
1412440c9 mov rdi, qword ptr [rbp - 0x38]
1412440cd cmp rbx, rdi
1412440d0 jne 0x141243f30
1412440d6 mov rdx, qword ptr [rbp + 0x38]
1412440da test rdx, rdx
1412440dd je 0x1412440f2
1412440df shl rdx, 3
1412440e3 mov r8d, 8
1412440e9 mov rcx, qword ptr [rbp + 0x60]
1412440ed call 0x140613c20
1412440f2 movaps xmm6, xmmword ptr [rbp + 0x80]
1412440f9 movaps xmm7, xmmword ptr [rbp + 0x90]
141244100 add rsp, 0x128
141244107 pop rbx
141244108 pop rdi
141244109 pop rsi
14124410a pop r12
14124410c pop r13
14124410e pop r14
141244110 pop r15
141244112 pop rbp
141244113 ret 
141244114 lea rcx, [rip + 0x19db8ed]
14124411b call 0x1428d98a0
141244120 lea r8, [rip + 0x19db8e1]
141244127 mov rcx, r15
14124412a mov rdx, rax
14124412d call 0x1428d9850
141244132 lea r8, [rip + 0x19db8cf]
141244139 mov rcx, rax
14124413c mov rdx, r9
14124413f call 0x1428d9840
141244144 lea r8, [rip + 0x19db8d5]
14124414b mov rcx, rax
14124414e mov rdx, r9
141244151 call 0x1428d9840
141244156 lea r8, [rip + 0x19db8db]
14124415d mov rcx, rax
141244160 mov rdx, r9
141244163 call 0x1428d9840
141244168 lea rcx, [rip + 0x19db8e1]
14124416f call 0x1428d98a0
141244174 jmp 0x1412441c4
141244176 lea r8, [rip + 0x19db8d3]
14124417d mov rcx, rdx
141244180 mov rdx, rsi
141244183 call 0x1428d9840
141244188 jmp 0x1412441c4
14124418a mov rdi, r8
14124418d lea r8, [rip + 0x19db8bc]
141244194 mov rcx, rdi
141244197 call 0x1428d9850
14124419c jmp 0x1412441c4
14124419e mov eax, 4
1412441a3 mov qword ptr [rbp + 0x58], rax
1412441a7 jmp 0x1412441b1
1412441a9 mov qword ptr [rbp + 0x58], 0
1412441b1 lea r8, [rip + 0x19db8b0]
1412441b8 mov rcx, qword ptr [rbp + 0x58]
1412441bc mov rdx, r12
1412441bf call 0x1428d8fe3
1412441c4 ud2 
