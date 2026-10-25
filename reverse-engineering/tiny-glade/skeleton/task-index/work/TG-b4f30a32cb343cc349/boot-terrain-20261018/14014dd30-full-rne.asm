14014dd30: push rbp
14014dd31: push rsi
14014dd32: push rdi
14014dd33: sub rsp, 0xa0
14014dd3a: lea rbp, [rsp + 0x80]
14014dd42: mov qword ptr [rbp + 0x18], 0xfffffffffffffffe
14014dd4a: mov rsi, qword ptr [rcx + 8]
14014dd4e: test rsi, rsi
14014dd51: je 0x14014dd9c
14014dd53: movups xmm0, xmmword ptr [rdx]
14014dd56: movups xmm1, xmmword ptr [rdx + 0x10]
14014dd5a: movups xmm2, xmmword ptr [rdx + 0x20]
14014dd5e: movaps xmmword ptr [rbp], xmm2
14014dd62: movaps xmmword ptr [rbp - 0x10], xmm1
14014dd66: movaps xmmword ptr [rbp - 0x20], xmm0
14014dd6a: mov rax, qword ptr [rsi]
14014dd6d: mov rdi, qword ptr [rsi + 0x10]
14014dd71: sub rax, rdi
14014dd74: cmp rax, 0x37
14014dd78: ja 0x14014ddc7
14014dd7a: mov qword ptr [rsp + 0x20], 1
14014dd83: mov r8d, 0x38
14014dd89: mov r9d, 1
14014dd8f: mov rcx, rsi
14014dd92: mov rdx, rdi
14014dd95: call 0x1428a29e0 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
14014dd9a: jmp 0x14014ddc7 ; bevy_ecs::system::commands::Commands::insert_resource<country_core::resources::stairs::stairs_top_down::StairsTopDownMesh>
14014dd9c: mov rsi, qword ptr [rcx + 0x10]
14014dda0: movups xmm0, xmmword ptr [rdx]
14014dda3: movups xmm1, xmmword ptr [rdx + 0x10]
14014dda7: movups xmm2, xmmword ptr [rdx + 0x20]
14014ddab: movaps xmmword ptr [rbp], xmm2
14014ddaf: movaps xmmword ptr [rbp - 0x10], xmm1
14014ddb3: movaps xmmword ptr [rbp - 0x20], xmm0
14014ddb7: mov rax, qword ptr [rsi]
14014ddba: mov rdi, qword ptr [rsi + 0x10]
14014ddbe: sub rax, rdi
14014ddc1: cmp rax, 0x37
14014ddc5: jbe 0x14014de1c
14014ddc7: mov rax, qword ptr [rsi + 8]
14014ddcb: movaps xmm0, xmmword ptr [rbp - 0x20]
14014ddcf: movaps xmm1, xmmword ptr [rbp - 0x10]
14014ddd3: movaps xmm2, xmmword ptr [rbp]
14014ddd7: movaps xmmword ptr [rbp - 0x30], xmm2
14014dddb: movaps xmmword ptr [rbp - 0x40], xmm1
14014dddf: movaps xmmword ptr [rbp - 0x50], xmm0
14014dde3: lea rcx, [rip - 0x9da]
14014ddea: mov qword ptr [rax + rdi], rcx
14014ddee: movaps xmm0, xmmword ptr [rbp - 0x50]
14014ddf2: movaps xmm1, xmmword ptr [rbp - 0x40]
14014ddf6: movaps xmm2, xmmword ptr [rbp - 0x30]
14014ddfa: movups xmmword ptr [rax + rdi + 8], xmm0
14014ddff: movups xmmword ptr [rax + rdi + 0x18], xmm1
14014de04: movups xmmword ptr [rax + rdi + 0x28], xmm2
14014de09: add rdi, 0x38
14014de0d: mov qword ptr [rsi + 0x10], rdi
14014de11: add rsp, 0xa0
14014de18: pop rdi
14014de19: pop rsi
14014de1a: pop rbp
14014de1b: ret 
14014de1c: mov qword ptr [rsp + 0x20], 1
14014de25: mov r8d, 0x38
14014de2b: mov r9d, 1
14014de31: mov rcx, rsi
14014de34: mov rdx, rdi
14014de37: call 0x1428a29e0 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
14014de3c: jmp 0x14014ddc7 ; bevy_ecs::system::commands::Commands::insert_resource<country_core::resources::stairs::stairs_top_down::StairsTopDownMesh>
14014de3e: nop 
14014de40: mov qword ptr [rsp + 0x10], rdx
14014de45: push rbp
14014de46: push rsi
14014de47: push rdi
14014de48: sub rsp, 0x30
14014de4c: lea rbp, [rdx + 0x80]
14014de53: lea rcx, [rbp - 0x20]
14014de57: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
14014de5c: nop 
14014de5d: add rsp, 0x30
14014de61: pop rdi
14014de62: pop rsi
14014de63: pop rbp
14014de64: ret 
14014de65: nop word ptr cs:[rax + rax]
14014de70: mov qword ptr [rsp + 0x10], rdx
14014de75: push rbp
14014de76: push rsi
14014de77: push rdi
14014de78: sub rsp, 0x30
14014de7c: lea rbp, [rdx + 0x80]
14014de83: lea rcx, [rbp - 0x20]
14014de87: call 0x140be24b0 ; rhapsody::raw::managed::handle::impl$6::drop
14014de8c: nop 
14014de8d: add rsp, 0x30
14014de91: pop rdi
14014de92: pop rsi
14014de93: pop rbp
14014de94: ret 
