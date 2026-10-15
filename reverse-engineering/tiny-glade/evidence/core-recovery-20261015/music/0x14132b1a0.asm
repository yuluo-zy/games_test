0x14132b1a0 push     r15
0x14132b1a2 push     r14
0x14132b1a4 push     r13
0x14132b1a6 push     r12
0x14132b1a8 push     rsi
0x14132b1a9 push     rdi
0x14132b1aa push     rbp
0x14132b1ab push     rbx
0x14132b1ac sub      rsp, 0x28
0x14132b1b0 mov      rax, rcx
0x14132b1b3 mov      rdi, qword ptr [rdx]
0x14132b1b6 mov      rsi, qword ptr [rdx + 8]
0x14132b1ba xor      ecx, ecx
0x14132b1bc call     qword ptr [rax]
0x14132b1be test     rax, rax
0x14132b1c1 je       0x14132b2c0
0x14132b1c7 cmp      rsi, 2
0x14132b1cb jb       0x14132b29a
0x14132b1d1 mov      rcx, rax
0x14132b1d4 mov      r15, qword ptr [rax]
0x14132b1d7 mov      ebx, 1
0x14132b1dc movabs   r10, 0xa0761d6478bd642f
0x14132b1e6 movabs   r11, 0xe7037ed1a0b428db
0x14132b1f0 movabs   r14, 0x40ec3ac8f17ac85e
0x14132b1fa nop      word ptr [rax + rax]
0x14132b200 lea      r12, [rbx + 1]
0x14132b204 lea      r13, [r15 + r10]
0x14132b208 mov      rax, r13
0x14132b20b xor      rax, r11
0x14132b20e mul      r13
0x14132b211 xor      rax, rdx
0x14132b214 mul      r12
0x14132b217 cmp      rax, rbx
0x14132b21a ja       0x14132b264
0x14132b21c mov      r9, rax
0x14132b21f mov      rbp, rdx
0x14132b222 mov      rax, rbx
0x14132b225 not      rax
0x14132b228 xor      edx, edx
0x14132b22a div      r12
0x14132b22d cmp      r9, rdx
0x14132b230 jae      0x14132b270
0x14132b232 mov      r8, rdx
0x14132b235 add      r15, r14
0x14132b238 nop      dword ptr [rax + rax]
0x14132b240 mov      rax, r15
0x14132b243 xor      rax, r11
0x14132b246 mul      r15
0x14132b249 add      r13, r10
0x14132b24c xor      rdx, rax
0x14132b24f mov      rax, r12
0x14132b252 imul     rax, rdx
0x14132b256 add      r15, r10
0x14132b259 cmp      rax, r8
0x14132b25c jb       0x14132b240
0x14132b25e mov      rax, rdx
0x14132b261 mul      r12
0x14132b264 mov      r15, r13
0x14132b267 cmp      rdx, rsi
0x14132b26a jb       0x14132b27b
0x14132b26c jmp      0x14132b2ab ; 
0x14132b26e nop      
0x14132b270 mov      r15, r13
0x14132b273 mov      rdx, rbp
0x14132b276 cmp      rdx, rsi
0x14132b279 jae      0x14132b2ab
0x14132b27b mov      rax, qword ptr [rdi + rbx*8]
0x14132b27f mov      r8, qword ptr [rdi + rdx*8]
0x14132b283 mov      qword ptr [rdi + rbx*8], r8
0x14132b287 mov      qword ptr [rdi + rdx*8], rax
0x14132b28b mov      rbx, r12
0x14132b28e cmp      r12, rsi
0x14132b291 jne      0x14132b200
0x14132b297 mov      qword ptr [rcx], r15
0x14132b29a add      rsp, 0x28
0x14132b29e pop      rbx
0x14132b29f pop      rbp
0x14132b2a0 pop      rdi
0x14132b2a1 pop      rsi
0x14132b2a2 pop      r12
0x14132b2a4 pop      r13
0x14132b2a6 pop      r14
0x14132b2a8 pop      r15
0x14132b2aa ret      
0x14132b2ab mov      qword ptr [rcx], r15
0x14132b2ae lea      r8, [rip + 0x190f9a3]
0x14132b2b5 mov      rcx, rdx
0x14132b2b8 mov      rdx, rsi
0x14132b2bb call     0x1428d9518 ; core::panicking::panic_bounds_check::h213dd6812833a70b
0x14132b2c0 lea      rcx, [rip + 0x190f7d9]
0x14132b2c7 call     0x1428d80d0 ; std::thread::local::panic_access_error::hcff639665d708376
0x14132b2cc int3     