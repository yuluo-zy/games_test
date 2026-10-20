1403d6ce0 push rbp
1403d6ce1 push rsi
1403d6ce2 push rdi
1403d6ce3 push rbx
1403d6ce4 sub rsp, 0x3e8
1403d6ceb lea rbp, [rsp + 0x80]
1403d6cf3 mov qword ptr [rbp + 0x360], 0xfffffffffffffffe
1403d6cfe mov rsi, rcx
1403d6d01 movups xmm0, xmmword ptr [rdx + 0x60]
1403d6d05 movaps xmmword ptr [rbp + 0x150], xmm0
1403d6d0c movups xmm0, xmmword ptr [rdx + 0x50]
1403d6d10 movaps xmmword ptr [rbp + 0x140], xmm0
1403d6d17 movups xmm0, xmmword ptr [rdx + 0x40]
1403d6d1b movaps xmmword ptr [rbp + 0x130], xmm0
1403d6d22 movups xmm0, xmmword ptr [rdx]
1403d6d25 movups xmm1, xmmword ptr [rdx + 0x10]
1403d6d29 movups xmm2, xmmword ptr [rdx + 0x20]
1403d6d2d movups xmm3, xmmword ptr [rdx + 0x30]
1403d6d31 movaps xmmword ptr [rbp + 0x120], xmm3
1403d6d38 movaps xmmword ptr [rbp + 0x110], xmm2
1403d6d3f movaps xmmword ptr [rbp + 0x100], xmm1
1403d6d46 movaps xmmword ptr [rbp + 0xf0], xmm0
1403d6d4d movzx eax, byte ptr [rip + 0x325deb5]
1403d6d54 mov ecx, 0x150
1403d6d59 mov edx, 8
1403d6d5e call 0x140613c10
1403d6d63 test rax, rax
1403d6d66 je 0x1403d7286
1403d6d6c mov qword ptr [rbp + 0x350], rax
1403d6d73 xorps xmm0, xmm0
1403d6d76 movups xmmword ptr [rbp + 0x180], xmm0
1403d6d7d movups xmmword ptr [rbp + 0x198], xmm0
1403d6d84 movups xmmword ptr [rbp + 0x1b0], xmm0
1403d6d8b movups xmmword ptr [rbp + 0x1c8], xmm0
1403d6d92 movups xmmword ptr [rbp + 0x1e0], xmm0
1403d6d99 mov dword ptr [rbp + 0x1f0], 0
1403d6da3 movups xmmword ptr [rbp + 0x218], xmm0
1403d6daa movups xmmword ptr [rbp + 0x230], xmm0
1403d6db1 movups xmmword ptr [rbp + 0x248], xmm0
1403d6db8 movups xmmword ptr [rbp + 0x260], xmm0
1403d6dbf movups xmmword ptr [rbp + 0x278], xmm0
1403d6dc6 mov dword ptr [rbp + 0x288], 0
1403d6dd0 mov qword ptr [rbp + 0x160], 0
1403d6ddb mov qword ptr [rbp + 0x168], 8
1403d6de6 mov qword ptr [rbp + 0x170], 0
1403d6df1 mov qword ptr [rbp + 0x178], 0x10
1403d6dfc mov qword ptr [rbp + 0x190], 0x10
1403d6e07 mov qword ptr [rbp + 0x1a8], 0x10
1403d6e12 mov qword ptr [rbp + 0x1c0], 0x10
1403d6e1d mov qword ptr [rbp + 0x1d8], 0x10
1403d6e28 movabs rbx, 0x8000000000000000
1403d6e32 mov qword ptr [rbp + 0x1f8], rbx
1403d6e39 lea rax, [rip + 0x25fc04a]
1403d6e40 mov qword ptr [rbp + 0x200], rax
1403d6e47 mov qword ptr [rbp + 0x208], 0x33
1403d6e52 mov qword ptr [rbp + 0x210], 0x10
1403d6e5d mov qword ptr [rbp + 0x228], 0x10
1403d6e68 mov qword ptr [rbp + 0x240], 0x10
1403d6e73 mov qword ptr [rbp + 0x258], 0x10
1403d6e7e mov qword ptr [rbp + 0x270], 0x10
1403d6e89 mov dword ptr [rbp + 0x290], 0
1403d6e93 mov word ptr [rbp + 0x294], 1
1403d6e9c mov qword ptr [rbp + 0x298], rbx
1403d6ea3 mov dword ptr [rbp + 0x348], 0
1403d6ead movzx eax, byte ptr [rip + 0x325dd55]
1403d6eb4 mov ecx, 0x1f0
1403d6eb9 mov edx, 8
1403d6ebe call 0x140613c10
1403d6ec3 test rax, rax
1403d6ec6 je 0x1403d729e
1403d6ecc mov rdi, rax
1403d6ecf lea rdx, [rbp + 0x160]
1403d6ed6 mov r8d, 0x1f0
1403d6edc mov rcx, rax
1403d6edf call 0x142923c00
1403d6ee4 mov byte ptr [rbp + 0x35f], 1
1403d6eeb lea r8, [rip + 0x25fbdf6]
1403d6ef2 lea rcx, [rbp + 0x80]
1403d6ef9 mov rdx, rdi
1403d6efc call 0x140ce4b10
1403d6f01 movaps xmm0, xmmword ptr [rbp + 0x150]
1403d6f08 movaps xmmword ptr [rbp + 0x1c0], xmm0
1403d6f0f movaps xmm0, xmmword ptr [rbp + 0x140]
1403d6f16 movaps xmmword ptr [rbp + 0x1b0], xmm0
1403d6f1d movaps xmm0, xmmword ptr [rbp + 0x130]
1403d6f24 movaps xmmword ptr [rbp + 0x1a0], xmm0
1403d6f2b movaps xmm0, xmmword ptr [rbp + 0xf0]
1403d6f32 movaps xmm1, xmmword ptr [rbp + 0x100]
1403d6f39 movaps xmm2, xmmword ptr [rbp + 0x110]
1403d6f40 movaps xmm3, xmmword ptr [rbp + 0x120]
1403d6f47 movaps xmmword ptr [rbp + 0x190], xmm3
1403d6f4e movaps xmmword ptr [rbp + 0x180], xmm2
1403d6f55 movaps xmmword ptr [rbp + 0x170], xmm1
1403d6f5c movaps xmmword ptr [rbp + 0x160], xmm0
1403d6f63 lea rcx, [rbp + 0x10]
1403d6f67 lea rdx, [rbp + 0x160]
1403d6f6e call 0x1400d4290
1403d6f73 xorps xmm0, xmm0
1403d6f76 movups xmmword ptr [rbp + 0x1a0], xmm0
1403d6f7d movups xmmword ptr [rbp + 0x1b8], xmm0
1403d6f84 movups xmmword ptr [rbp + 0x1d0], xmm0
1403d6f8b movups xmmword ptr [rbp + 0x1e8], xmm0
1403d6f92 movups xmmword ptr [rbp + 0x200], xmm0
1403d6f99 mov dword ptr [rbp + 0x210], 0
1403d6fa3 movups xmmword ptr [rbp + 0x238], xmm0
1403d6faa movups xmmword ptr [rbp + 0x250], xmm0
1403d6fb1 movups xmmword ptr [rbp + 0x268], xmm0
1403d6fb8 movups xmmword ptr [rbp + 0x280], xmm0
1403d6fbf movups xmmword ptr [rbp + 0x298], xmm0
1403d6fc6 mov dword ptr [rbp + 0x2a8], 0
1403d6fd0 mov qword ptr [rbp + 0x160], 0
1403d6fdb mov qword ptr [rbp + 0x180], 0
1403d6fe6 mov qword ptr [rbp + 0x188], 8
1403d6ff1 mov qword ptr [rbp + 0x190], 0
1403d6ffc mov qword ptr [rbp + 0x198], 0x10
1403d7007 mov qword ptr [rbp + 0x1b0], 0x10
1403d7012 mov qword ptr [rbp + 0x1c8], 0x10
1403d701d mov qword ptr [rbp + 0x1e0], 0x10
1403d7028 mov qword ptr [rbp + 0x1f8], 0x10
1403d7033 mov qword ptr [rbp + 0x218], rbx
1403d703a lea rax, [rip + 0x25fbdac]
1403d7041 mov qword ptr [rbp + 0x220], rax
1403d7048 mov qword ptr [rbp + 0x228], 0x34
1403d7053 mov qword ptr [rbp + 0x230], 0x10
1403d705e mov qword ptr [rbp + 0x248], 0x10
1403d7069 mov qword ptr [rbp + 0x260], 0x10
1403d7074 mov qword ptr [rbp + 0x278], 0x10
1403d707f mov qword ptr [rbp + 0x290], 0x10
1403d708a mov dword ptr [rbp + 0x2b0], 0
1403d7094 mov word ptr [rbp + 0x2b4], 1
1403d709d mov dword ptr [rbp + 0x2b8], 0
1403d70a7 movzx eax, byte ptr [rip + 0x325db5b]
1403d70ae mov ecx, 0x160
1403d70b3 mov edx, 8
1403d70b8 call 0x140613c10
1403d70bd test rax, rax
1403d70c0 je 0x1403d72af
1403d70c6 mov rdi, rax
1403d70c9 lea rdx, [rbp + 0x160]
1403d70d0 mov r8d, 0x160
1403d70d6 mov rcx, rax
1403d70d9 call 0x142923c00
1403d70de lea r8, [rip + 0x25fba93]
1403d70e5 lea rcx, [rbp - 0x60]
1403d70e9 mov rdx, rdi
1403d70ec call 0x140ce4b10
1403d70f1 movups xmm0, xmmword ptr [rbp + 0xe0]
1403d70f8 mov rax, qword ptr [rbp + 0x350]
1403d70ff movups xmmword ptr [rax + 0x60], xmm0
1403d7103 movups xmm0, xmmword ptr [rbp + 0xd0]
1403d710a movups xmmword ptr [rax + 0x50], xmm0
1403d710e movups xmm0, xmmword ptr [rbp + 0xc0]
1403d7115 movups xmmword ptr [rax + 0x40], xmm0
1403d7119 movups xmm0, xmmword ptr [rbp + 0x80]
1403d7120 movups xmm1, xmmword ptr [rbp + 0x90]
1403d7127 movups xmm2, xmmword ptr [rbp + 0xa0]
1403d712e movups xmm3, xmmword ptr [rbp + 0xb0]
1403d7135 movups xmmword ptr [rax + 0x30], xmm3
1403d7139 movups xmmword ptr [rax + 0x20], xmm2
1403d713d movups xmmword ptr [rax + 0x10], xmm1
1403d7141 movups xmmword ptr [rax], xmm0
1403d7144 movups xmm0, xmmword ptr [rbp + 0x10]
1403d7148 movups xmm1, xmmword ptr [rbp + 0x20]
1403d714c movups xmm2, xmmword ptr [rbp + 0x30]
1403d7150 movups xmm3, xmmword ptr [rbp + 0x40]
1403d7154 movups xmmword ptr [rax + 0x70], xmm0
1403d7158 movups xmmword ptr [rax + 0x80], xmm1
1403d715f movups xmmword ptr [rax + 0x90], xmm2
1403d7166 movups xmmword ptr [rax + 0xa0], xmm3
1403d716d movups xmm0, xmmword ptr [rbp + 0x50]
1403d7171 movups xmmword ptr [rax + 0xb0], xmm0
1403d7178 movups xmm0, xmmword ptr [rbp + 0x60]
1403d717c movups xmmword ptr [rax + 0xc0], xmm0
1403d7183 movups xmm0, xmmword ptr [rbp + 0x70]
1403d7187 movups xmmword ptr [rax + 0xd0], xmm0
1403d718e movups xmm0, xmmword ptr [rbp - 0x60]
1403d7192 movups xmm1, xmmword ptr [rbp - 0x50]
1403d7196 movups xmm2, xmmword ptr [rbp - 0x40]
1403d719a movups xmm3, xmmword ptr [rbp - 0x30]
1403d719e movups xmmword ptr [rax + 0xe0], xmm0
1403d71a5 movups xmmword ptr [rax + 0xf0], xmm1
1403d71ac movups xmmword ptr [rax + 0x100], xmm2
1403d71b3 movups xmmword ptr [rax + 0x110], xmm3
1403d71ba movups xmm0, xmmword ptr [rbp - 0x20]
1403d71be movups xmmword ptr [rax + 0x120], xmm0
1403d71c5 movups xmm0, xmmword ptr [rbp - 0x10]
1403d71c9 movups xmmword ptr [rax + 0x130], xmm0
1403d71d0 movups xmm0, xmmword ptr [rbp]
1403d71d4 movups xmmword ptr [rax + 0x140], xmm0
1403d71db mov qword ptr [rbp + 0x168], 3
1403d71e6 mov qword ptr [rbp + 0x170], rax
1403d71ed mov qword ptr [rbp + 0x178], 3
1403d71f8 mov qword ptr [rbp + 0x180], 0
1403d7203 mov qword ptr [rbp + 0x188], 8
1403d720e xorps xmm0, xmm0
1403d7211 movups xmmword ptr [rbp + 0x190], xmm0
1403d7218 mov qword ptr [rbp + 0x160], rbx
1403d721f lea rcx, [rbp + 0x198]
1403d7226 call 0x1401705d0
1403d722b movups xmm0, xmmword ptr [rbp + 0x1c0]
1403d7232 movups xmmword ptr [rsi + 0x60], xmm0
1403d7236 movups xmm0, xmmword ptr [rbp + 0x1b0]
1403d723d movups xmmword ptr [rsi + 0x50], xmm0
1403d7241 movups xmm0, xmmword ptr [rbp + 0x1a0]
1403d7248 movups xmmword ptr [rsi + 0x40], xmm0
1403d724c movups xmm0, xmmword ptr [rbp + 0x160]
1403d7253 movups xmm1, xmmword ptr [rbp + 0x170]
1403d725a movups xmm2, xmmword ptr [rbp + 0x180]
1403d7261 movups xmm3, xmmword ptr [rbp + 0x190]
1403d7268 movups xmmword ptr [rsi + 0x30], xmm3
1403d726c movups xmmword ptr [rsi + 0x20], xmm2
1403d7270 movups xmmword ptr [rsi + 0x10], xmm1
1403d7274 movups xmmword ptr [rsi], xmm0
1403d7277 mov rax, rsi
1403d727a add rsp, 0x3e8
1403d7281 pop rbx
1403d7282 pop rdi
1403d7283 pop rsi
1403d7284 pop rbp
1403d7285 ret 
1403d7286 mov byte ptr [rbp + 0x35e], 1
1403d728d mov ecx, 8
1403d7292 mov edx, 0x150
1403d7297 call 0x1428d9000
1403d729c jmp 0x1403d72be
1403d729e mov ecx, 8
1403d72a3 mov edx, 0x1f0
1403d72a8 call 0x1428d9000
1403d72ad jmp 0x1403d72be
1403d72af mov ecx, 8
1403d72b4 mov edx, 0x160
1403d72b9 call 0x1428d9000
1403d72be ud2 
1403d72c0 mov qword ptr [rsp + 0x10], rdx
1403d72c5 push rbp
1403d72c6 push rsi
1403d72c7 push rdi
1403d72c8 push rbx
1403d72c9 sub rsp, 0x28
1403d72cd lea rbp, [rdx + 0x80]
1403d72d4 mov byte ptr [rbp + 0x35f], 1
1403d72db lea rcx, [rbp + 0x160]
1403d72e2 call 0x1405d53e0
1403d72e7 mov byte ptr [rbp + 0x35f], 1
1403d72ee add rsp, 0x28
1403d72f2 pop rbx
1403d72f3 pop rdi
1403d72f4 pop rsi
1403d72f5 pop rbp
1403d72f6 ret 
1403d72f7 nop word ptr [rax + rax]
1403d7300 mov qword ptr [rsp + 0x10], rdx
1403d7305 push rbp
1403d7306 push rsi
1403d7307 push rdi
1403d7308 push rbx
1403d7309 sub rsp, 0x28
1403d730d lea rbp, [rdx + 0x80]
1403d7314 movzx ebx, byte ptr [rbp + 0x35f]
1403d731b mov edx, 0x150
1403d7320 mov r8d, 8
1403d7326 mov rcx, qword ptr [rbp + 0x350]
1403d732d call 0x140613c20
1403d7332 mov byte ptr [rbp + 0x35e], bl
1403d7338 add rsp, 0x28
1403d733c pop rbx
1403d733d pop rdi
1403d733e pop rsi
1403d733f pop rbp
1403d7340 ret 
1403d7341 nop word ptr cs:[rax + rax]
1403d7350 mov qword ptr [rsp + 0x10], rdx
1403d7355 push rbp
1403d7356 push rsi
1403d7357 push rdi
1403d7358 push rbx
1403d7359 sub rsp, 0x28
1403d735d lea rbp, [rdx + 0x80]
1403d7364 lea rcx, [rbp + 0x160]
1403d736b call 0x1405e5e70
1403d7370 nop 
1403d7371 add rsp, 0x28
1403d7375 pop rbx
1403d7376 pop rdi
1403d7377 pop rsi
1403d7378 pop rbp
1403d7379 ret 
1403d737a nop word ptr [rax + rax]
1403d7380 mov qword ptr [rsp + 0x10], rdx
1403d7385 push rbp
1403d7386 push rsi
1403d7387 push rdi
1403d7388 push rbx
1403d7389 sub rsp, 0x28
1403d738d lea rbp, [rdx + 0x80]
1403d7394 lea rcx, [rbp + 0x10]
1403d7398 call 0x140029880
1403d739d nop 
1403d739e add rsp, 0x28
1403d73a2 pop rbx
1403d73a3 pop rdi
1403d73a4 pop rsi
1403d73a5 pop rbp
1403d73a6 ret 
1403d73a7 nop word ptr [rax + rax]
1403d73b0 mov qword ptr [rsp + 0x10], rdx
1403d73b5 push rbp
1403d73b6 push rsi
1403d73b7 push rdi
1403d73b8 push rbx
1403d73b9 sub rsp, 0x28
1403d73bd lea rbp, [rdx + 0x80]
1403d73c4 cmp byte ptr [rbp + 0x35e], 0
1403d73cb jne 0x1403d73d6
1403d73cd add rsp, 0x28
1403d73d1 pop rbx
1403d73d2 pop rdi
1403d73d3 pop rsi
1403d73d4 pop rbp
1403d73d5 ret 
1403d73d6 lea rcx, [rbp + 0xf0]
1403d73dd call 0x140029880
1403d73e2 jmp 0x1403d73cd
1403d73e4 nop word ptr cs:[rax + rax]
1403d73f0 mov qword ptr [rsp + 0x10], rdx
1403d73f5 push rbp
1403d73f6 push rsi
1403d73f7 push rdi
1403d73f8 push rbx
1403d73f9 sub rsp, 0x28
1403d73fd lea rbp, [rdx + 0x80]
1403d7404 lea rcx, [rbp + 0x80]
1403d740b call 0x140029880
1403d7410 mov byte ptr [rbp + 0x35f], 0
1403d7417 add rsp, 0x28
1403d741b pop rbx
1403d741c pop rdi
1403d741d pop rsi
1403d741e pop rbp
1403d741f ret 
1403d7420 mov qword ptr [rsp + 0x10], rdx
1403d7425 push rbp
1403d7426 push rsi
1403d7427 push rdi
1403d7428 push rbx
1403d7429 sub rsp, 0x28
1403d742d lea rbp, [rdx + 0x80]
1403d7434 lea rcx, [rbp + 0x160]
1403d743b call 0x140029880
1403d7440 nop 
1403d7441 add rsp, 0x28
1403d7445 pop rbx
1403d7446 pop rdi
1403d7447 pop rsi
1403d7448 pop rbp
1403d7449 ret 