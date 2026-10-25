140a55f40: 4157 push r15
140a55f42: 4156 push r14
140a55f44: 56 push rsi
140a55f45: 57 push rdi
140a55f46: 53 push rbx
140a55f47: 4883ec20 sub rsp, 0x20
140a55f4b: 4889ce mov rsi, rcx
140a55f4e: 488b12 mov rdx, qword ptr [rdx]
140a55f51: 4885d2 test rdx, rdx
140a55f54: 740e je 0x140a55f64
140a55f56: 4883c210 add rdx, 0x10
140a55f5a: 4889f1 mov rcx, rsi
140a55f5d: e8dee4ffff call 0x140a54440 ; country_core::resources::audio_resources::AudioCoreInner::instantiate_audio
140a55f62: eb64 jmp 0x140a55fc8
140a55f64: 4d85c9 test r9, r9
140a55f67: 786e js 0x140a55fd7
140a55f69: 7430 je 0x140a55f9b
140a55f6b: 4d89c7 mov r15, r8
140a55f6e: 0fb60594ecbd02 movzx eax, byte ptr [rip + 0x2bdec94]
140a55f75: 41be01000000 mov r14d, 1
140a55f7b: ba01000000 mov edx, 1
140a55f80: 4c89cb mov rbx, r9
140a55f83: 4c89c9 mov rcx, r9
140a55f86: e885dcbbff call 0x140613c10 ; 
140a55f8b: 4885c0 test rax, rax
140a55f8e: 744d je 0x140a55fdd
140a55f90: 4889c7 mov rdi, rax
140a55f93: 4989d9 mov r9, rbx
140a55f96: 4d89f8 mov r8, r15
140a55f99: eb05 jmp 0x140a55fa0
140a55f9b: bf01000000 mov edi, 1
140a55fa0: 4889f9 mov rcx, rdi
140a55fa3: 4c89c2 mov rdx, r8
140a55fa6: 4d89c8 mov r8, r9
140a55fa9: 4c89cb mov rbx, r9
140a55fac: e84fdcec01 call 0x142923c00 ; 
140a55fb1: 48c70600000000 mov qword ptr [rsi], 0
140a55fb8: 48895e10 mov qword ptr [rsi + 0x10], rbx
140a55fbc: 48897e18 mov qword ptr [rsi + 0x18], rdi
140a55fc0: 48895e20 mov qword ptr [rsi + 0x20], rbx
140a55fc4: c6462802 mov byte ptr [rsi + 0x28], 2
140a55fc8: 4889f0 mov rax, rsi
140a55fcb: 4883c420 add rsp, 0x20
140a55fcf: 5b pop rbx
140a55fd0: 5f pop rdi
140a55fd1: 5e pop rsi
140a55fd2: 415e pop r14
140a55fd4: 415f pop r15
140a55fd6: c3 ret 
140a55fd7: 4c89cb mov rbx, r9
140a55fda: 4531f6 xor r14d, r14d
140a55fdd: 4c8d05cca70702 lea r8, [rip + 0x207a7cc]
140a55fe4: 4c89f1 mov rcx, r14
140a55fe7: 4889da mov rdx, rbx
140a55fea: e8f42fe801 call 0x1428d8fe3 ; alloc::raw_vec::handle_error
140a55fef: cc int3 
