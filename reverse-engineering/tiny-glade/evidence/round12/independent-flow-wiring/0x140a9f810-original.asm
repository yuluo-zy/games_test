140a9f810: push rbp
140a9f811: push rsi
140a9f812: push rdi
140a9f813: push rbx
140a9f814: sub rsp, 0x88
140a9f81b: lea rbp, [rsp + 0x80]
140a9f823: mov qword ptr [rbp], 0xfffffffffffffffe
140a9f82b: mov rax, qword ptr [rdx]
140a9f82e: mov rdx, qword ptr [rdx + 8]
140a9f832: mov r10, qword ptr [rax]
140a9f835: xor r11d, r11d
140a9f838: mov rsi, r10
140a9f83b: sub rsi, qword ptr [rdx + 0x18]
140a9f83f: cmovb rsi, r11
140a9f843: sub r10, qword ptr [rdx + 0x38]
140a9f847: cmovb r10, r11
140a9f84b: mov rdi, qword ptr [rdx + 0x30]
140a9f84f: sub rdi, r10
140a9f852: cmovb rdi, r11
140a9f856: mov r10, qword ptr [rdx + 0x10]
140a9f85a: setbe bl
140a9f85d: sub r10, rsi
140a9f860: cmovb r10, r11
140a9f864: setbe sil
140a9f868: add r10, rdi
140a9f86b: test sil, bl
140a9f86e: cmove r10, r11
140a9f872: mov rdx, qword ptr [rdx + 0x40]
140a9f876: sub rdx, r10
140a9f879: mov qword ptr [rax], rdx
140a9f87c: test sil, bl
140a9f87f: je 0x140a9f88c
140a9f881: test byte ptr [r8], 1
140a9f885: jne 0x140a9f890
140a9f887: jmp 0x140a9f993
140a9f88c: mov byte ptr [r8], 1
140a9f890: mov rax, qword ptr [rcx]
140a9f893: mov eax, dword ptr [rax]
140a9f895: lea ecx, [rax - 6]
140a9f898: cmp ecx, 0xf
140a9f89b: setb cl
140a9f89e: mov eax, eax
140a9f8a0: add rax, -0xd
140a9f8a4: cmp rax, 2
140a9f8a8: setb al
140a9f8ab: and al, cl
140a9f8ad: cmp al, 1
140a9f8af: je 0x140a9f993
140a9f8b5: mov rdx, qword ptr [r9]
140a9f8b8: add rdx, 0x60
140a9f8bc: lea rsi, [rbp - 0x58]
140a9f8c0: mov rcx, rsi
140a9f8c3: call 0x140b46000 ; country_core::resources::session::SessionId::save_folder_path
140a9f8c8: mov rcx, rsi
140a9f8cb: call 0x140d76ed0 ; alloc::boxed::convert::impl$19::from::impl$0::description
140a9f8d0: mov r8, rdx
140a9f8d3: mov qword ptr [rsp + 0x20], 0xb
140a9f8dc: lea r9, [rip + 0x203c9ab]
140a9f8e3: lea rcx, [rbp - 0x38]
140a9f8e7: mov rdx, rax
140a9f8ea: call 0x140d62810 ; std::path::Path::_join
140a9f8ef: lea rcx, [rbp - 0x38]
140a9f8f3: call 0x140d76ed0 ; alloc::boxed::convert::impl$19::from::impl$0::description
140a9f8f8: mov rcx, rax
140a9f8fb: call 0x140d69590 ; std::sys::pal::windows::fs::unlink
140a9f900: mov rsi, rax
140a9f903: mov rdx, qword ptr [rbp - 0x38]
140a9f907: test rdx, rdx
140a9f90a: je 0x140a9f91b
140a9f90c: mov rcx, qword ptr [rbp - 0x30]
140a9f910: mov r8d, 1
140a9f916: call 0x140613c20 ; 
140a9f91b: mov eax, esi
140a9f91d: and eax, 3
140a9f920: cmp eax, 1
140a9f923: jne 0x140a9f97b
140a9f925: mov rax, rsi
140a9f928: dec rax
140a9f92b: mov qword ptr [rbp - 0x18], rax
140a9f92f: mov rax, qword ptr [rsi - 1]
140a9f933: mov qword ptr [rbp - 0x10], rax
140a9f937: mov rax, qword ptr [rsi + 7]
140a9f93b: mov qword ptr [rbp - 8], rax
140a9f93f: mov rax, qword ptr [rax]
140a9f942: test rax, rax
140a9f945: je 0x140a9f94d
140a9f947: mov rcx, qword ptr [rbp - 0x10]
140a9f94b: call rax
140a9f94d: mov rcx, qword ptr [rbp - 0x10]
140a9f951: mov rax, qword ptr [rbp - 8]
140a9f955: mov rdx, qword ptr [rax + 8]
140a9f959: test rdx, rdx
140a9f95c: je 0x140a9f967
140a9f95e: mov r8, qword ptr [rax + 0x10]
140a9f962: call 0x140613c20 ; 
140a9f967: mov edx, 0x18
140a9f96c: mov r8d, 8
140a9f972: mov rcx, qword ptr [rbp - 0x18]
140a9f976: call 0x140613c20 ; 
140a9f97b: mov rdx, qword ptr [rbp - 0x58]
140a9f97f: test rdx, rdx
140a9f982: je 0x140a9f993
140a9f984: mov rcx, qword ptr [rbp - 0x50]
140a9f988: mov r8d, 1
140a9f98e: call 0x140613c20 ; 
140a9f993: nop 
140a9f994: add rsp, 0x88
140a9f99b: pop rbx
140a9f99c: pop rdi
140a9f99d: pop rsi
140a9f99e: pop rbp
140a9f99f: ret 
140a9f9a0: mov qword ptr [rsp + 0x10], rdx
140a9f9a5: push rbp
140a9f9a6: push rsi
140a9f9a7: push rdi
140a9f9a8: push rbx
140a9f9a9: sub rsp, 0x28
140a9f9ad: lea rbp, [rdx + 0x80]
140a9f9b4: mov rdx, qword ptr [rbp - 0x58]
140a9f9b8: test rdx, rdx
140a9f9bb: je 0x140a9f9cc
140a9f9bd: mov rcx, qword ptr [rbp - 0x50]
140a9f9c1: mov r8d, 1
140a9f9c7: call 0x140613c20 ; 
140a9f9cc: nop 
140a9f9cd: add rsp, 0x28
140a9f9d1: pop rbx
140a9f9d2: pop rdi
140a9f9d3: pop rsi
140a9f9d4: pop rbp
140a9f9d5: ret 
140a9f9d6: nop word ptr cs:[rax + rax]
140a9f9e0: mov qword ptr [rsp + 0x10], rdx
140a9f9e5: push rbp
140a9f9e6: push rsi
140a9f9e7: push rdi
140a9f9e8: push rbx
140a9f9e9: sub rsp, 0x28
140a9f9ed: lea rbp, [rdx + 0x80]
140a9f9f4: mov rdx, qword ptr [rbp - 0x38]
140a9f9f8: test rdx, rdx
140a9f9fb: je 0x140a9fa0c
140a9f9fd: mov rcx, qword ptr [rbp - 0x30]
140a9fa01: mov r8d, 1
140a9fa07: call 0x140613c20 ; 
140a9fa0c: nop 
140a9fa0d: add rsp, 0x28
140a9fa11: pop rbx
140a9fa12: pop rdi
140a9fa13: pop rsi
140a9fa14: pop rbp
140a9fa15: ret 
140a9fa16: nop word ptr cs:[rax + rax]
140a9fa20: mov qword ptr [rsp + 0x10], rdx
140a9fa25: push rbp
140a9fa26: push rsi
140a9fa27: push rdi
140a9fa28: push rbx
140a9fa29: sub rsp, 0x28
140a9fa2d: lea rbp, [rdx + 0x80]
140a9fa34: mov rcx, qword ptr [rbp - 0x10]
140a9fa38: mov rax, qword ptr [rbp - 8]
140a9fa3c: mov rdx, qword ptr [rax + 8]
140a9fa40: test rdx, rdx
140a9fa43: je 0x140a9fa52
140a9fa45: mov rax, qword ptr [rbp - 8]
140a9fa49: mov r8, qword ptr [rax + 0x10]
140a9fa4d: call 0x140613c20 ; 
140a9fa52: mov edx, 0x18
140a9fa57: mov r8d, 8
140a9fa5d: mov rcx, qword ptr [rbp - 0x18]
140a9fa61: call 0x140613c20 ; 
140a9fa66: nop 
140a9fa67: add rsp, 0x28
140a9fa6b: pop rbx
140a9fa6c: pop rdi
140a9fa6d: pop rsi
140a9fa6e: pop rbp
140a9fa6f: ret 
