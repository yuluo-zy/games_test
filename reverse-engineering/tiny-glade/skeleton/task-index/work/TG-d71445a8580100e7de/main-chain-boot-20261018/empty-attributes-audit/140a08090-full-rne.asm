140a08090: push r15
140a08092: push r14
140a08094: push r12
140a08096: push rsi
140a08097: push rdi
140a08098: push rbp
140a08099: push rbx
140a0809a: sub rsp, 0x20
140a0809e: mov rsi, r8
140a080a1: mov rdi, rdx
140a080a4: mov rbx, rcx
140a080a7: call 0x1412f64f0 ; alloc::vec::Vec::retain<u32,alloc::alloc::Global,country_core::render::mesh::impl$0::retain_vertices::closure_env$0>
140a080ac: mov r14, qword ptr [rbx + 0x50]
140a080b0: test r14, r14
140a080b3: je 0x140a0812b
140a080b5: mov r15, qword ptr [rbx + 0x38]
140a080b9: lea r12, [r15 + 0x10]
140a080bd: movdqa xmm0, xmmword ptr [r15]
140a080c2: pmovmskb eax, xmm0
140a080c6: not eax
140a080c8: jmp 0x140a080fa
140a080ca: nop word ptr [rax + rax]
140a080d0: lea ebp, [rax - 1]
140a080d3: and ebp, eax
140a080d5: tzcnt eax, eax
140a080d9: neg rax
140a080dc: imul rax, rax, 0x38
140a080e0: lea rcx, [r15 + rax]
140a080e4: add rcx, -0x20
140a080e8: mov rdx, rdi
140a080eb: mov r8, rsi
140a080ee: call 0x140a36110 ; country_core::render::mesh::VertexAttributeValues::retain_by_index<country_core::render::mesh::impl$0::retain_vertices::closure_env$1>
140a080f3: mov eax, ebp
140a080f5: dec r14
140a080f8: je 0x140a0812b
140a080fa: test ax, ax
140a080fd: jne 0x140a080d0
140a080ff: nop 
140a08100: movdqa xmm0, xmmword ptr [r12]
140a08106: pmovmskb eax, xmm0
140a0810a: add r15, -0x380
140a08111: add r12, 0x10
140a08115: cmp eax, 0xffff
140a0811a: je 0x140a08100
140a0811c: mov ecx, 0xfffffffe
140a08121: sub ecx, eax
140a08123: not eax
140a08125: mov ebp, eax
140a08127: and ebp, ecx
140a08129: jmp 0x140a080d5
140a0812b: test rsi, rsi
140a0812e: je 0x140a08165
140a08130: lea rax, [rsi*4]
140a08138: xor ecx, ecx
140a0813a: xor edx, edx
140a0813c: nop dword ptr [rax]
140a08140: xor r8d, r8d
140a08143: cmp dword ptr [rdi + rcx], 1
140a08147: sete r8b
140a0814b: mov r9d, 0xffffffff
140a08151: cmove r9d, edx
140a08155: add edx, r8d
140a08158: mov dword ptr [rdi + rcx], r9d
140a0815c: add rcx, 4
140a08160: cmp rax, rcx
140a08163: jne 0x140a08140
140a08165: mov rax, qword ptr [rbx + 0x10]
140a08169: test rax, rax
140a0816c: je 0x140a0819e
140a0816e: mov rdx, qword ptr [rbx + 8]
140a08172: shl rax, 2
140a08176: xor r8d, r8d
140a08179: nop dword ptr [rax]
140a08180: mov ecx, dword ptr [rdx + r8]
140a08184: cmp rcx, rsi
140a08187: jae 0x140a081c5
140a08189: mov ecx, dword ptr [rdi + rcx*4]
140a0818c: cmp ecx, -1
140a0818f: je 0x140a081ad
140a08191: mov dword ptr [rdx + r8], ecx
140a08195: add r8, 4
140a08199: cmp rax, r8
140a0819c: jne 0x140a08180
140a0819e: add rsp, 0x20
140a081a2: pop rbx
140a081a3: pop rbp
140a081a4: pop rdi
140a081a5: pop rsi
140a081a6: pop r12
140a081a8: pop r14
140a081aa: pop r15
140a081ac: ret 
140a081ad: lea rcx, [rip + 0x20be6d4]
140a081b4: lea r8, [rip + 0x20be6f5]
140a081bb: mov edx, 0x26
140a081c0: call 0x1428d9430 ; core::panicking::panic
140a081c5: lea r8, [rip + 0x20be6a4]
140a081cc: mov rdx, rsi
140a081cf: call 0x1428d9518 ; core::panicking::panic_bounds_check
140a081d4: int3 
