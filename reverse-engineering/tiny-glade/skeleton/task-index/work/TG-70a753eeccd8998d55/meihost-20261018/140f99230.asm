0x140f99230 push r14
0x140f99232 push rsi
0x140f99233 push rdi
0x140f99234 push rbp
0x140f99235 push rbx
0x140f99236 sub rsp, 0x30
0x140f9923a mov rdi, rdx
0x140f9923d mov rsi, rcx
0x140f99240 movzx ebp, byte ptr [rsp + 0x80]
0x140f99248 mov qword ptr [rsp + 0x20], r8
0x140f9924d mov qword ptr [rsp + 0x28], r9
0x140f99252 lea rcx, [rsp + 0x20]
0x140f99257 call 0x1405bdda0
0x140f9925c movabs rbx, 0xb1695320004fb443
0x140f99266 xor rbx, rax
0x140f99269 mov rcx, rdi
0x140f9926c mov rdx, rbx
0x140f9926f call 0x140683ec0
0x140f99274 mov byte ptr [rax + 0x5d], 0
0x140f99278 test bpl, bpl
0x140f9927b je 0x140f99284
0x140f9927d mov byte ptr [rax + 0xb7], 1
0x140f99284 test dl, 1
0x140f99287 jne 0x140f992bc
0x140f99289 mov r14, qword ptr [rdi + 0x10]
0x140f9928d cmp r14, qword ptr [rdi]
0x140f99290 jne 0x140f992a1
0x140f99292 lea rdx, [rip + 0x1c15d2f]
0x140f99299 mov rcx, rdi
0x140f9929c call 0x1406319d0
0x140f992a1 mov rax, qword ptr [rdi + 8]
0x140f992a5 imul rcx, r14, 0x38
0x140f992a9 mov dword ptr [rax + rcx], 1
0x140f992b0 mov qword ptr [rax + rcx + 0x30], rbx
0x140f992b5 inc r14
0x140f992b8 mov qword ptr [rdi + 0x10], r14
0x140f992bc mov qword ptr [rsi], rdi
0x140f992bf mov qword ptr [rsi + 8], rbx
0x140f992c3 mov qword ptr [rsi + 0x10], 0
0x140f992cb mov rax, rsi
0x140f992ce add rsp, 0x30
0x140f992d2 pop rbx
0x140f992d3 pop rbp
0x140f992d4 pop rdi
0x140f992d5 pop rsi
0x140f992d6 pop r14
0x140f992d8 ret 
0x140f992d9 int3 
0x140f992da int3 
0x140f992db int3 
0x140f992dc int3 
0x140f992dd int3 
0x140f992de int3 
0x140f992df int3 