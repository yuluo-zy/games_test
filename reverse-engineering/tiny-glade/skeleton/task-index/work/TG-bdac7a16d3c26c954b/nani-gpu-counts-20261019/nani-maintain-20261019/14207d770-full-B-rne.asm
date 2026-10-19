14207d770: 55 push rbp
14207d771: 4157 push r15
14207d773: 4156 push r14
14207d775: 4155 push r13
14207d777: 4154 push r12
14207d779: 56 push rsi
14207d77a: 57 push rdi
14207d77b: 53 push rbx
14207d77c: 4881ec48010000 sub rsp, 0x148
14207d783: 488dac2480000000 lea rbp, [rsp + 0x80]
14207d78b: 440f2995b0000000 movaps xmmword ptr [rbp + 0xb0], xmm10
14207d793: 440f298da0000000 movaps xmmword ptr [rbp + 0xa0], xmm9
14207d79b: 440f298590000000 movaps xmmword ptr [rbp + 0x90], xmm8
14207d7a3: 0f29bd80000000 movaps xmmword ptr [rbp + 0x80], xmm7
14207d7aa: 0f297570 movaps xmmword ptr [rbp + 0x70], xmm6
14207d7ae: 48c74568feffffff mov qword ptr [rbp + 0x68], 0xfffffffffffffffe
14207d7b6: 4889d6 mov rsi, rdx
14207d7b9: 4889cf mov rdi, rcx
14207d7bc: e83f92b9fe call 0x140c16a00 ; puffin::are_scopes_on
14207d7c1: 84c0 test al, al
14207d7c3: 745f je 0x14207d824
14207d7c5: 4c8d35015bf500 lea r14, [rip + 0xf55b01]
14207d7cc: bb3d000000 mov ebx, 0x3d
14207d7d1: ba3d000000 mov edx, 0x3d
14207d7d6: 4c89f1 mov rcx, r14
14207d7d9: e89216ffff call 0x14206ee70 ; core::str::impl$0::rfind<ref$<str$> >
14207d7de: 4883f801 cmp rax, 1
14207d7e2: 0f85b1000000 jne 0x14207d899
14207d7e8: 4989d1 mov r9, rdx
14207d7eb: 4885d2 test rdx, rdx
14207d7ee: 743d je 0x14207d82d
14207d7f0: 4983f93d cmp r9, 0x3d
14207d7f4: 7335 jae 0x14207d82b
14207d7f6: 488d05d05af500 lea rax, [rip + 0xf55ad0]
14207d7fd: 41803c01bf cmp byte ptr [r9 + rax], 0xbf
14207d802: 7f29 jg 0x14207d82d
14207d804: 488d05cd5bf500 lea rax, [rip + 0xf55bcd]
14207d80b: 4889442420 mov qword ptr [rsp + 0x20], rax
14207d810: 488d0db65af500 lea rcx, [rip + 0xf55ab6]
14207d817: ba3d000000 mov edx, 0x3d
14207d81c: 4531c0 xor r8d, r8d
14207d81f: e81cc18500 call 0x1428d9940 ; core::str::slice_error_fail
14207d824: 31c0 xor eax, eax
14207d826: e9bf010000 jmp 0x14207d9ea ; unknown
14207d82b: 75d7 jne 0x14207d804
14207d82d: 4c8d35995af500 lea r14, [rip + 0xf55a99]
14207d834: 4c89f1 mov rcx, r14
14207d837: 4c89ca mov rdx, r9
14207d83a: e83116ffff call 0x14206ee70 ; core::str::impl$0::rfind<ref$<str$> >
14207d83f: 4883f801 cmp rax, 1
14207d843: 7554 jne 0x14207d899
14207d845: 4989d0 mov r8, rdx
14207d848: 4983c002 add r8, 2
14207d84c: 7439 je 0x14207d887
14207d84e: 4983f83d cmp r8, 0x3d
14207d852: 7331 jae 0x14207d885
14207d854: 488d05725af500 lea rax, [rip + 0xf55a72]
14207d85b: 41803c00bf cmp byte ptr [r8 + rax], 0xbf
14207d860: 7f25 jg 0x14207d887
14207d862: 488d05875bf500 lea rax, [rip + 0xf55b87]
14207d869: 4889442420 mov qword ptr [rsp + 0x20], rax
14207d86e: 488d0d585af500 lea rcx, [rip + 0xf55a58]
14207d875: ba3d000000 mov edx, 0x3d
14207d87a: 41b93d000000 mov r9d, 0x3d
14207d880: e8bbc08500 call 0x1428d9940 ; core::str::slice_error_fail
14207d885: 75db jne 0x14207d862
14207d887: bb3b000000 mov ebx, 0x3b
14207d88c: 4829d3 sub rbx, rdx
14207d88f: 4c8d35375af500 lea r14, [rip + 0xf55a37]
14207d896: 4d01c6 add r14, r8
14207d899: 4c8d05be5bf500 lea r8, [rip + 0xf55bbe]
14207d8a0: 4c8d3d895bf500 lea r15, [rip + 0xf55b89]
14207d8a7: 660f1f840000000000 nop word ptr [rax + rax]
14207d8b0: 410fbe40ff movsx eax, byte ptr [r8 - 1]
14207d8b5: 85c0 test eax, eax
14207d8b7: 7817 js 0x14207d8d0
14207d8b9: 49ffc8 dec r8
14207d8bc: 83f85c cmp eax, 0x5c
14207d8bf: 7563 jne 0x14207d924
14207d8c1: eb73 jmp 0x14207d936 ; unknown
14207d8c3: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14207d8d0: 410fb648fe movzx ecx, byte ptr [r8 - 2]
14207d8d5: 80f9c0 cmp cl, 0xc0
14207d8d8: 7d23 jge 0x14207d8fd
14207d8da: 410fb650fd movzx edx, byte ptr [r8 - 3]
14207d8df: 80fac0 cmp dl, 0xc0
14207d8e2: 7d22 jge 0x14207d906
14207d8e4: 450fb648fc movzx r9d, byte ptr [r8 - 4]
14207d8e9: 4983c0fc add r8, -4
14207d8ed: 4183e107 and r9d, 7
14207d8f1: 41c1e106 shl r9d, 6
14207d8f5: 83e23f and edx, 0x3f
14207d8f8: 4409ca or edx, r9d
14207d8fb: eb10 jmp 0x14207d90d ; unknown
14207d8fd: 4983c0fe add r8, -2
14207d901: 83e11f and ecx, 0x1f
14207d904: eb0f jmp 0x14207d915 ; unknown
14207d906: 4983c0fd add r8, -3
14207d90a: 83e20f and edx, 0xf
14207d90d: c1e206 shl edx, 6
14207d910: 83e13f and ecx, 0x3f
14207d913: 09d1 or ecx, edx
14207d915: c1e106 shl ecx, 6
14207d918: 243f and al, 0x3f
14207d91a: 0fb6c0 movzx eax, al
14207d91d: 09c8 or eax, ecx
14207d91f: 83f85c cmp eax, 0x5c
14207d922: 7412 je 0x14207d936
14207d924: 83f82f cmp eax, 0x2f
14207d927: 740d je 0x14207d936
14207d929: 4d39f8 cmp r8, r15
14207d92c: 7582 jne 0x14207d8b0
14207d92e: 41bc2e000000 mov r12d, 0x2e
14207d934: eb4d jmp 0x14207d983 ; unknown
14207d936: 4c8d3df35af500 lea r15, [rip + 0xf55af3]
14207d93d: 4d29f8 sub r8, r15
14207d940: 49ffc0 inc r8
14207d943: 7432 je 0x14207d977
14207d945: 4983f82e cmp r8, 0x2e
14207d949: 732a jae 0x14207d975
14207d94b: 43803c38bf cmp byte ptr [r8 + r15], 0xbf
14207d950: 7f25 jg 0x14207d977
14207d952: 488d055f5af500 lea rax, [rip + 0xf55a5f]
14207d959: 4889442420 mov qword ptr [rsp + 0x20], rax
14207d95e: 488d0dcb5af500 lea rcx, [rip + 0xf55acb]
14207d965: ba2e000000 mov edx, 0x2e
14207d96a: 41b92e000000 mov r9d, 0x2e
14207d970: e8cbbf8500 call 0x1428d9940 ; core::str::slice_error_fail
14207d975: 75db jne 0x14207d952
14207d977: 41bc2e000000 mov r12d, 0x2e
14207d97d: 4d29c4 sub r12, r8
14207d980: 4d01c7 add r15, r8
14207d983: e8a8a7b9fe call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
14207d988: 4889c1 mov rcx, rax
14207d98b: 488b00 mov rax, qword ptr [rax]
14207d98e: 4883f801 cmp rax, 1
14207d992: 0f85f8050000 jne 0x14207df90
14207d998: 4883c108 add rcx, 8
14207d99c: 48833900 cmp qword ptr [rcx], 0
14207d9a0: 0f850e060000 jne 0x14207dfb4
14207d9a6: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
14207d9ad: 48894d48 mov qword ptr [rbp + 0x48], rcx
14207d9b1: 4883c108 add rcx, 8
14207d9b5: 4c89642420 mov qword ptr [rsp + 0x20], r12
14207d9ba: 48c744243000000000 mov qword ptr [rsp + 0x30], 0
14207d9c3: 48c744242801000000 mov qword ptr [rsp + 0x28], 1
14207d9cc: 4c89f2 mov rdx, r14
14207d9cf: 4989d8 mov r8, rbx
14207d9d2: 4d89f9 mov r9, r15
14207d9d5: e86693b9fe call 0x140c16d40 ; puffin::ThreadProfiler::begin_scope
14207d9da: 488b4d48 mov rcx, qword ptr [rbp + 0x48]
14207d9de: 48ff01 inc qword ptr [rcx]
14207d9e1: 48894530 mov qword ptr [rbp + 0x30], rax
14207d9e5: b801000000 mov eax, 1
14207d9ea: 48894528 mov qword ptr [rbp + 0x28], rax
14207d9ee: 488b0e mov rcx, qword ptr [rsi]
14207d9f1: b216 mov dl, 0x16
14207d9f3: e8584291fe call 0x140991c50 ; country_core::render::nani::gpu_world::GpuWorld::owned_subset_mut
14207d9f8: 48895520 mov qword ptr [rbp + 0x20], rdx
14207d9fc: 48894540 mov qword ptr [rbp + 0x40], rax
14207da00: 488b1f mov rbx, qword ptr [rdi]
14207da03: 4c8b7708 mov r14, qword ptr [rdi + 8]
14207da07: 4c8bbbf8000000 mov r15, qword ptr [rbx + 0xf8]
14207da0e: 488b8300010000 mov rax, qword ptr [rbx + 0x100]
14207da15: 4d8d2487 lea r12, [r15 + rax*4]
14207da19: 0fb68308010000 movzx eax, byte ptr [rbx + 0x108]
14207da20: 884548 mov byte ptr [rbp + 0x48], al
14207da23: b808000000 mov eax, 8
14207da28: 48894538 mov qword ptr [rbp + 0x38], rax
14207da2c: 31ff xor edi, edi
14207da2e: f30f1035ce7e8a00 movss xmm6, dword ptr [rip + 0x8a7ece]
14207da36: 0f283d430f8b00 movaps xmm7, xmmword ptr [rip + 0x8b0f43]
14207da3d: 31f6 xor esi, esi
14207da3f: 48c7455000000000 mov qword ptr [rbp + 0x50], 0
14207da47: 48c7455800000000 mov qword ptr [rbp + 0x58], 0
14207da4f: 48c7456000000000 mov qword ptr [rbp + 0x60], 0
14207da57: 4531ed xor r13d, r13d
14207da5a: 807d4800 cmp byte ptr [rbp + 0x48], 0
14207da5e: 0f857c010000 jne 0x14207dbe0
14207da64: 4839fe cmp rsi, rdi
14207da67: 0f8552010000 jne 0x14207dbbf
14207da6d: 0f1f00 nop dword ptr [rax]
14207da70: 4d39e7 cmp r15, r12
14207da73: 0f845b040000 je 0x14207ded4
14207da79: 418b0f mov ecx, dword ptr [r15]
14207da7c: 4983c704 add r15, 4
14207da80: 498b8600010000 mov rax, qword ptr [r14 + 0x100]
14207da87: 488d0c89 lea rcx, [rcx + rcx*4]
14207da8b: 48c1e105 shl rcx, 5
14207da8f: 488b7c0858 mov rdi, qword ptr [rax + rcx + 0x58]
14207da94: 4885ff test rdi, rdi
14207da97: 74d7 je 0x14207da70
14207da99: 4801c8 add rax, rcx
14207da9c: 8b8894000000 mov ecx, dword ptr [rax + 0x94]
14207daa2: 498b96a8010000 mov rdx, qword ptr [r14 + 0x1a8]
14207daa9: 4c8d0cc9 lea r9, [rcx + rcx*8]
14207daad: 4a8d0cca lea rcx, [rdx + r9*8]
14207dab1: 4c8b8310010000 mov r8, qword ptr [rbx + 0x110]
14207dab8: 4a8b54ca40 mov rdx, qword ptr [rdx + r9*8 + 0x40]
14207dabd: 4c39c2 cmp rdx, r8
14207dac0: 7633 jbe 0x14207daf5
14207dac2: 4c8b4938 mov r9, qword ptr [rcx + 0x38]
14207dac6: 4f8b04c1 mov r8, qword ptr [r9 + r8*8]
14207daca: 4d85c0 test r8, r8
14207dacd: 7426 je 0x14207daf5
14207dacf: 4c8b4918 mov r9, qword ptr [rcx + 0x18]
14207dad3: 49f7d0 not r8
14207dad6: 4f8d0440 lea r8, [r8 + r8*2]
14207dada: 49c1e004 shl r8, 4
14207dade: 4f8b440110 mov r8, qword ptr [r9 + r8 + 0x10]
14207dae3: 4c894550 mov qword ptr [rbp + 0x50], r8
14207dae7: 4c8b8318010000 mov r8, qword ptr [rbx + 0x118]
14207daee: 4c39c2 cmp rdx, r8
14207daf1: 7716 ja 0x14207db09
14207daf3: eb47 jmp 0x14207db3c ; unknown
14207daf5: 48c7455000000000 mov qword ptr [rbp + 0x50], 0
14207dafd: 4c8b8318010000 mov r8, qword ptr [rbx + 0x118]
14207db04: 4c39c2 cmp rdx, r8
14207db07: 7633 jbe 0x14207db3c
14207db09: 4c8b4938 mov r9, qword ptr [rcx + 0x38]
14207db0d: 4f8b04c1 mov r8, qword ptr [r9 + r8*8]
14207db11: 4d85c0 test r8, r8
14207db14: 7426 je 0x14207db3c
14207db16: 4c8b4918 mov r9, qword ptr [rcx + 0x18]
14207db1a: 49f7d0 not r8
14207db1d: 4f8d0440 lea r8, [r8 + r8*2]
14207db21: 49c1e004 shl r8, 4
14207db25: 4f8b440110 mov r8, qword ptr [r9 + r8 + 0x10]
14207db2a: 4c894558 mov qword ptr [rbp + 0x58], r8
14207db2e: 4c8b8320010000 mov r8, qword ptr [rbx + 0x120]
14207db35: 4c39c2 cmp rdx, r8
14207db38: 764d jbe 0x14207db87
14207db3a: eb14 jmp 0x14207db50 ; unknown
14207db3c: 48c7455800000000 mov qword ptr [rbp + 0x58], 0
14207db44: 4c8b8320010000 mov r8, qword ptr [rbx + 0x120]
14207db4b: 4c39c2 cmp rdx, r8
14207db4e: 7637 jbe 0x14207db87
14207db50: 488b5138 mov rdx, qword ptr [rcx + 0x38]
14207db54: 4a8b14c2 mov rdx, qword ptr [rdx + r8*8]
14207db58: 4885d2 test rdx, rdx
14207db5b: 742a je 0x14207db87
14207db5d: 488b4918 mov rcx, qword ptr [rcx + 0x18]
14207db61: 48f7d2 not rdx
14207db64: 488d1452 lea rdx, [rdx + rdx*2]
14207db68: 48c1e204 shl rdx, 4
14207db6c: 488b4c1110 mov rcx, qword ptr [rcx + rdx + 0x10]
14207db71: 48894d60 mov qword ptr [rbp + 0x60], rcx
14207db75: 488b8b28010000 mov rcx, qword ptr [rbx + 0x128]
14207db7c: 48398888000000 cmp qword ptr [rax + 0x88], rcx
14207db83: 762d jbe 0x14207dbb2
14207db85: eb18 jmp 0x14207db9f ; unknown
14207db87: 48c7456000000000 mov qword ptr [rbp + 0x60], 0
14207db8f: 488b8b28010000 mov rcx, qword ptr [rbx + 0x128]
14207db96: 48398888000000 cmp qword ptr [rax + 0x88], rcx
14207db9d: 7613 jbe 0x14207dbb2
14207db9f: 488b9080000000 mov rdx, qword ptr [rax + 0x80]
14207dba6: 48833cca00 cmp qword ptr [rdx + rcx*8], 0
14207dbab: 7405 je 0x14207dbb2
14207dbad: 41b501 mov r13b, 1
14207dbb0: eb03 jmp 0x14207dbb5 ; unknown
14207dbb2: 4531ed xor r13d, r13d
14207dbb5: 488b4050 mov rax, qword ptr [rax + 0x50]
14207dbb9: 48894538 mov qword ptr [rbp + 0x38], rax
14207dbbd: 31f6 xor esi, esi
14207dbbf: 4889f0 mov rax, rsi
14207dbc2: 48c1e004 shl rax, 4
14207dbc6: 488b4d38 mov rcx, qword ptr [rbp + 0x38]
14207dbca: 8b4c0108 mov ecx, dword ptr [rcx + rax + 8]
14207dbce: e94f010000 jmp 0x14207dd22 ; unknown
14207dbd3: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14207dbe0: 4839fe cmp rsi, rdi
14207dbe3: 0f8537010000 jne 0x14207dd20
14207dbe9: 0f1f8000000000 nop dword ptr [rax]
14207dbf0: 4d39e7 cmp r15, r12
14207dbf3: 0f84db020000 je 0x14207ded4
14207dbf9: 418b0f mov ecx, dword ptr [r15]
14207dbfc: 4983c704 add r15, 4
14207dc00: 498b86a8010000 mov rax, qword ptr [r14 + 0x1a8]
14207dc07: 488d0cc9 lea rcx, [rcx + rcx*8]
14207dc0b: 488b7cc810 mov rdi, qword ptr [rax + rcx*8 + 0x10]
14207dc10: 4885ff test rdi, rdi
14207dc13: 74db je 0x14207dbf0
14207dc15: 488d04c8 lea rax, [rax + rcx*8]
14207dc19: 488b9310010000 mov rdx, qword ptr [rbx + 0x110]
14207dc20: 488b4840 mov rcx, qword ptr [rax + 0x40]
14207dc24: 4839d1 cmp rcx, rdx
14207dc27: 7633 jbe 0x14207dc5c
14207dc29: 4c8b4038 mov r8, qword ptr [rax + 0x38]
14207dc2d: 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
14207dc31: 4885d2 test rdx, rdx
14207dc34: 7426 je 0x14207dc5c
14207dc36: 4c8b4018 mov r8, qword ptr [rax + 0x18]
14207dc3a: 48f7d2 not rdx
14207dc3d: 488d1452 lea rdx, [rdx + rdx*2]
14207dc41: 48c1e204 shl rdx, 4
14207dc45: 498b541010 mov rdx, qword ptr [r8 + rdx + 0x10]
14207dc4a: 48895550 mov qword ptr [rbp + 0x50], rdx
14207dc4e: 488b9318010000 mov rdx, qword ptr [rbx + 0x118]
14207dc55: 4839d1 cmp rcx, rdx
14207dc58: 7716 ja 0x14207dc70
14207dc5a: eb47 jmp 0x14207dca3 ; unknown
14207dc5c: 48c7455000000000 mov qword ptr [rbp + 0x50], 0
14207dc64: 488b9318010000 mov rdx, qword ptr [rbx + 0x118]
14207dc6b: 4839d1 cmp rcx, rdx
14207dc6e: 7633 jbe 0x14207dca3
14207dc70: 4c8b4038 mov r8, qword ptr [rax + 0x38]
14207dc74: 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
14207dc78: 4885d2 test rdx, rdx
14207dc7b: 7426 je 0x14207dca3
14207dc7d: 4c8b4018 mov r8, qword ptr [rax + 0x18]
14207dc81: 48f7d2 not rdx
14207dc84: 488d1452 lea rdx, [rdx + rdx*2]
14207dc88: 48c1e204 shl rdx, 4
14207dc8c: 498b541010 mov rdx, qword ptr [r8 + rdx + 0x10]
14207dc91: 48895558 mov qword ptr [rbp + 0x58], rdx
14207dc95: 488b9320010000 mov rdx, qword ptr [rbx + 0x120]
14207dc9c: 4839d1 cmp rcx, rdx
14207dc9f: 764e jbe 0x14207dcef
14207dca1: eb14 jmp 0x14207dcb7 ; unknown
14207dca3: 48c7455800000000 mov qword ptr [rbp + 0x58], 0
14207dcab: 488b9320010000 mov rdx, qword ptr [rbx + 0x120]
14207dcb2: 4839d1 cmp rcx, rdx
14207dcb5: 7638 jbe 0x14207dcef
14207dcb7: 4c8b4038 mov r8, qword ptr [rax + 0x38]
14207dcbb: 498b14d0 mov rdx, qword ptr [r8 + rdx*8]
14207dcbf: 4885d2 test rdx, rdx
14207dcc2: 742b je 0x14207dcef
14207dcc4: 4c8b4018 mov r8, qword ptr [rax + 0x18]
14207dcc8: 48f7d2 not rdx
14207dccb: 488d1452 lea rdx, [rdx + rdx*2]
14207dccf: 48c1e204 shl rdx, 4
14207dcd3: 498b541010 mov rdx, qword ptr [r8 + rdx + 0x10]
14207dcd8: 48895560 mov qword ptr [rbp + 0x60], rdx
14207dcdc: 488b9328010000 mov rdx, qword ptr [rbx + 0x128]
14207dce3: 4839d1 cmp rcx, rdx
14207dce6: 771b ja 0x14207dd03
14207dce8: 31f6 xor esi, esi
14207dcea: 4531ed xor r13d, r13d
14207dced: eb31 jmp 0x14207dd20 ; unknown
14207dcef: 48c7456000000000 mov qword ptr [rbp + 0x60], 0
14207dcf7: 488b9328010000 mov rdx, qword ptr [rbx + 0x128]
14207dcfe: 4839d1 cmp rcx, rdx
14207dd01: 76e5 jbe 0x14207dce8
14207dd03: 488b4038 mov rax, qword ptr [rax + 0x38]
14207dd07: 48833cd000 cmp qword ptr [rax + rdx*8], 0
14207dd0c: 410f95c5 setne r13b
14207dd10: 31f6 xor esi, esi
14207dd12: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14207dd20: 89f1 mov ecx, esi
14207dd22: 488d0449 lea rax, [rcx + rcx*2]
14207dd26: 488b5550 mov rdx, qword ptr [rbp + 0x50]
14207dd2a: 488d1482 lea rdx, [rdx + rax*4]
14207dd2e: 48c1e004 shl rax, 4
14207dd32: 48034558 add rax, qword ptr [rbp + 0x58]
14207dd36: 4c8b4560 mov r8, qword ptr [rbp + 0x60]
14207dd3a: 498d0c88 lea rcx, [r8 + rcx*4]
14207dd3e: 448b4018 mov r8d, dword ptr [rax + 0x18]
14207dd42: 44894518 mov dword ptr [rbp + 0x18], r8d
14207dd46: 4c8b4010 mov r8, qword ptr [rax + 0x10]
14207dd4a: 4c894510 mov qword ptr [rbp + 0x10], r8
14207dd4e: 0f2820 movaps xmm4, xmmword ptr [rax]
14207dd51: f30f105808 movss xmm3, dword ptr [rax + 8]
14207dd56: 440f28c3 movaps xmm8, xmm3
14207dd5a: f3440f58c3 addss xmm8, xmm3
14207dd5f: f3410f59d8 mulss xmm3, xmm8
14207dd64: 0f28cc movaps xmm1, xmm4
14207dd67: 0f58cc addps xmm1, xmm4
14207dd6a: f30f16c1 movshdup xmm0, xmm1
14207dd6e: f30f59c4 mulss xmm0, xmm4
14207dd72: 0f28d4 movaps xmm2, xmm4
14207dd75: f3410f12e8 movsldup xmm5, xmm8
14207dd7a: 0f59ec mulps xmm5, xmm4
14207dd7d: 0fc6e4ff shufps xmm4, xmm4, 0xff
14207dd81: 0f59d1 mulps xmm2, xmm1
14207dd84: 0f59cc mulps xmm1, xmm4
14207dd87: 440f28cc movaps xmm9, xmm4
14207dd8b: f3450f59c8 mulss xmm9, xmm8
14207dd90: f3440f16d2 movshdup xmm10, xmm2
14207dd95: 450f28c2 movaps xmm8, xmm10
14207dd99: f3440f58c3 addss xmm8, xmm3
14207dd9e: 0f28e6 movaps xmm4, xmm6
14207dda1: f3410f5ce0 subss xmm4, xmm8
14207dda6: 440f28c0 movaps xmm8, xmm0
14207ddaa: f3450f58c1 addss xmm8, xmm9
14207ddaf: 410f14e0 unpcklps xmm4, xmm8
14207ddb3: f3410f5cc1 subss xmm0, xmm9
14207ddb8: f30f58da addss xmm3, xmm2
14207ddbc: 440f28c6 movaps xmm8, xmm6
14207ddc0: f3440f5cc3 subss xmm8, xmm3
14207ddc5: 0fc6c9e1 shufps xmm1, xmm1, 0xe1
14207ddc9: 410f14c0 unpcklps xmm0, xmm8
14207ddcd: 440f28c5 movaps xmm8, xmm5
14207ddd1: 440f5cc1 subps xmm8, xmm1
14207ddd5: 0f58cd addps xmm1, xmm5
14207ddd8: f30f105820 movss xmm3, dword ptr [rax + 0x20]
14207dddd: f3410f58d2 addss xmm2, xmm10
14207dde2: 440f28ce movaps xmm9, xmm6
14207dde6: f3440f5cca subss xmm9, xmm2
14207ddeb: f30f106824 movss xmm5, dword ptr [rax + 0x24]
14207ddf0: 410f16e0 movlhps xmm4, xmm8
14207ddf4: 0fc6c1d4 shufps xmm0, xmm1, 0xd4
14207ddf8: 440fc6c101 shufps xmm8, xmm1, 1
14207ddfd: 440fc6c930 shufps xmm9, xmm1, 0x30
14207de02: 0f10501c movups xmm2, xmmword ptr [rax + 0x1c]
14207de06: 450fc6c182 shufps xmm8, xmm9, 0x82
14207de0b: 0f104820 movups xmm1, xmmword ptr [rax + 0x20]
14207de0f: 0fc6d200 shufps xmm2, xmm2, 0
14207de13: 0f59d4 mulps xmm2, xmm4
14207de16: 0f28e1 movaps xmm4, xmm1
14207de19: 0fc6e100 shufps xmm4, xmm1, 0
14207de1d: 0f59e0 mulps xmm4, xmm0
14207de20: 0fc6c955 shufps xmm1, xmm1, 0x55
14207de24: 410f59c8 mulps xmm1, xmm8
14207de28: 0f54df andps xmm3, xmm7
14207de2b: 0f54ef andps xmm5, xmm7
14207de2e: 0f28c3 movaps xmm0, xmm3
14207de31: f30fc2c303 cmpunordss xmm0, xmm3
14207de36: 440f28c0 movaps xmm8, xmm0
14207de3a: 440f54c5 andps xmm8, xmm5
14207de3e: f30f5feb maxss xmm5, xmm3
14207de42: f30f10581c movss xmm3, dword ptr [rax + 0x1c]
14207de47: 0f54df andps xmm3, xmm7
14207de4a: 0f55c5 andnps xmm0, xmm5
14207de4d: 410f56c0 orps xmm0, xmm8
14207de51: 0f28e8 movaps xmm5, xmm0
14207de54: f30f5feb maxss xmm5, xmm3
14207de58: f30fc2db03 cmpunordss xmm3, xmm3
14207de5d: 440f28c3 movaps xmm8, xmm3
14207de61: 440f54c0 andps xmm8, xmm0
14207de65: 0f55dd andnps xmm3, xmm5
14207de68: 410f56d8 orps xmm3, xmm8
14207de6c: 0f28c2 movaps xmm0, xmm2
14207de6f: f20f10c4 movsd xmm0, xmm4
14207de73: 0fc6d024 shufps xmm2, xmm0, 0x24
14207de77: 0f284010 movaps xmm0, xmmword ptr [rax + 0x10]
14207de7b: 8b01 mov eax, dword ptr [rcx]
14207de7d: 0f2955c0 movaps xmmword ptr [rbp - 0x40], xmm2
14207de81: 0fc6e149 shufps xmm4, xmm1, 0x49
14207de85: 0f2965d0 movaps xmmword ptr [rbp - 0x30], xmm4
14207de89: 0fc6c802 shufps xmm1, xmm0, 2
14207de8d: 0fc6c898 shufps xmm1, xmm0, 0x98
14207de91: 0f294de0 movaps xmmword ptr [rbp - 0x20], xmm1
14207de95: 4489e9 mov ecx, r13d
14207de98: 80e101 and cl, 1
14207de9b: 0fb6c9 movzx ecx, cl
14207de9e: 894df0 mov dword ptr [rbp - 0x10], ecx
14207dea1: 8945f4 mov dword ptr [rbp - 0xc], eax
14207dea4: 48c745f800000000 mov qword ptr [rbp - 8], 0
14207deac: 488d45c0 lea rax, [rbp - 0x40]
14207deb0: 4889442420 mov qword ptr [rsp + 0x20], rax
14207deb5: 488b4d20 mov rcx, qword ptr [rbp + 0x20]
14207deb9: 4c8d4510 lea r8, [rbp + 0x10]
14207debd: e87e104fff call 0x14156ef40 ; country_core::render::nani::gpu_world::OwnedGpuWorldSubset::update_instance_data<country_core::render::nani::instance_data::BalconyDoorInstanceData>
14207dec2: 48ffc6 inc rsi
14207dec5: 807d4800 cmp byte ptr [rbp + 0x48], 0
14207dec9: 0f8495fbffff je 0x14207da64
14207decf: e90cfdffff jmp 0x14207dbe0 ; unknown
14207ded4: 488b7540 mov rsi, qword ptr [rbp + 0x40]
14207ded8: 4889f1 mov rcx, rsi
14207dedb: e8303fc0fe call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
14207dee0: 4889f1 mov rcx, rsi
14207dee3: 4883c901 or rcx, 1
14207dee7: e8243fc0fe call 0x140c81e10 ; parking_lot_core::parking_lot::deadlock_impl::release_resource
14207deec: 31c9 xor ecx, ecx
14207deee: b808000000 mov eax, 8
14207def3: f0480fb10e lock cmpxchg qword ptr [rsi], rcx
14207def8: 0f85c2000000 jne 0x14207dfc0
14207defe: 48837d2800 cmp qword ptr [rbp + 0x28], 0
14207df03: 743e je 0x14207df43
14207df05: e826a2b9fe call 0x140c18130 ; puffin::impl$6::call::THREAD_PROFILER::constant$0::closure$0::VAL
14207df0a: 4889c1 mov rcx, rax
14207df0d: 488b00 mov rax, qword ptr [rax]
14207df10: 4883f801 cmp rax, 1
14207df14: 7564 jne 0x14207df7a
14207df16: 4883c108 add rcx, 8
14207df1a: 48833900 cmp qword ptr [rcx], 0
14207df1e: 0f8590000000 jne 0x14207dfb4
14207df24: 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
14207df2b: 48894d48 mov qword ptr [rbp + 0x48], rcx
14207df2f: 4883c108 add rcx, 8
14207df33: 488b5530 mov rdx, qword ptr [rbp + 0x30]
14207df37: e8b491b9fe call 0x140c170f0 ; puffin::ThreadProfiler::end_scope
14207df3c: 488b4548 mov rax, qword ptr [rbp + 0x48]
14207df40: 48ff00 inc qword ptr [rax]
14207df43: 0f287570 movaps xmm6, xmmword ptr [rbp + 0x70]
14207df47: 0f28bd80000000 movaps xmm7, xmmword ptr [rbp + 0x80]
14207df4e: 440f288590000000 movaps xmm8, xmmword ptr [rbp + 0x90]
14207df56: 440f288da0000000 movaps xmm9, xmmword ptr [rbp + 0xa0]
14207df5e: 440f2895b0000000 movaps xmm10, xmmword ptr [rbp + 0xb0]
14207df66: 4881c448010000 add rsp, 0x148
14207df6d: 5b pop rbx
14207df6e: 5f pop rdi
14207df6f: 5e pop rsi
14207df70: 415c pop r12
14207df72: 415d pop r13
14207df74: 415e pop r14
14207df76: 415f pop r15
14207df78: 5d pop rbp
14207df79: c3 ret 
14207df7a: 4885c0 test rax, rax
14207df7d: 7529 jne 0x14207dfa8
14207df7f: 31d2 xor edx, edx
14207df81: e84aa88700 call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
14207df86: 4889c1 mov rcx, rax
14207df89: 4885c0 test rax, rax
14207df8c: 758c jne 0x14207df1a
14207df8e: eb18 jmp 0x14207dfa8 ; unknown
14207df90: 4885c0 test rax, rax
14207df93: 7513 jne 0x14207dfa8
14207df95: 31d2 xor edx, edx
14207df97: e834a88700 call 0x1428f87d0 ; std::sys::thread_local::native::lazy::Storage::initialize<core::cell::RefCell<puffin::ThreadProfiler>,tuple$<>,core::cell::RefCell<puffin::ThreadProfiler> (*)()>
14207df9c: 4889c1 mov rcx, rax
14207df9f: 4885c0 test rax, rax
14207dfa2: 0f85f4f9ffff jne 0x14207d99c
14207dfa8: 488d0df1cabb00 lea rcx, [rip + 0xbbcaf1]
14207dfaf: e81ca18500 call 0x1428d80d0 ; std::thread::local::panic_access_error
14207dfb4: 488d0db5cbbb00 lea rcx, [rip + 0xbbcbb5]
14207dfbb: e890b28500 call 0x1428d9250 ; core::cell::panic_already_borrowed
14207dfc0: 4889f1 mov rcx, rsi
14207dfc3: 31d2 xor edx, edx
14207dfc5: e896df8400 call 0x1428cbf60 ; parking_lot::raw_rwlock::RawRwLock::unlock_exclusive_slow
14207dfca: e92fffffff jmp 0x14207defe ; unknown
14207dfcf: 90 nop 
14207dfd0: 4889542410 mov qword ptr [rsp + 0x10], rdx
14207dfd5: 55 push rbp
14207dfd6: 4157 push r15
14207dfd8: 4156 push r14
14207dfda: 4155 push r13
14207dfdc: 4154 push r12
14207dfde: 56 push rsi
14207dfdf: 57 push rdi
14207dfe0: 53 push rbx
14207dfe1: 4881ec88000000 sub rsp, 0x88
14207dfe8: 488daa80000000 lea rbp, [rdx + 0x80]
14207dfef: 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
14207dff5: 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
14207dffb: 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
14207e001: 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
14207e006: 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
14207e00b: 488b4548 mov rax, qword ptr [rbp + 0x48]
14207e00f: 48ff00 inc qword ptr [rax]
14207e012: 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
14207e017: 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
14207e01c: 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
14207e022: 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
14207e028: 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
14207e02e: 4881c488000000 add rsp, 0x88
14207e035: 5b pop rbx
14207e036: 5f pop rdi
14207e037: 5e pop rsi
14207e038: 415c pop r12
14207e03a: 415d pop r13
14207e03c: 415e pop r14
14207e03e: 415f pop r15
14207e040: 5d pop rbp
14207e041: c3 ret 
14207e042: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14207e050: 4889542410 mov qword ptr [rsp + 0x10], rdx
14207e055: 55 push rbp
14207e056: 4157 push r15
14207e058: 4156 push r14
14207e05a: 4155 push r13
14207e05c: 4154 push r12
14207e05e: 56 push rsi
14207e05f: 57 push rdi
14207e060: 53 push rbx
14207e061: 4881ec88000000 sub rsp, 0x88
14207e068: 488daa80000000 lea rbp, [rdx + 0x80]
14207e06f: 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
14207e075: 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
14207e07b: 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
14207e081: 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
14207e086: 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
14207e08b: 488b4548 mov rax, qword ptr [rbp + 0x48]
14207e08f: 48ff00 inc qword ptr [rax]
14207e092: 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
14207e097: 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
14207e09c: 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
14207e0a2: 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
14207e0a8: 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
14207e0ae: 4881c488000000 add rsp, 0x88
14207e0b5: 5b pop rbx
14207e0b6: 5f pop rdi
14207e0b7: 5e pop rsi
14207e0b8: 415c pop r12
14207e0ba: 415d pop r13
14207e0bc: 415e pop r14
14207e0be: 415f pop r15
14207e0c0: 5d pop rbp
14207e0c1: c3 ret 
14207e0c2: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14207e0d0: 4889542410 mov qword ptr [rsp + 0x10], rdx
14207e0d5: 55 push rbp
14207e0d6: 4157 push r15
14207e0d8: 4156 push r14
14207e0da: 4155 push r13
14207e0dc: 4154 push r12
14207e0de: 56 push rsi
14207e0df: 57 push rdi
14207e0e0: 53 push rbx
14207e0e1: 4881ec88000000 sub rsp, 0x88
14207e0e8: 488daa80000000 lea rbp, [rdx + 0x80]
14207e0ef: 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
14207e0f5: 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
14207e0fb: 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
14207e101: 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
14207e106: 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
14207e10b: 488b4d40 mov rcx, qword ptr [rbp + 0x40]
14207e10f: e8bc31fcfd call 0x1400412d0 ; core::ptr::drop_in_place<lock_api::rwlock::MappedRwLockWriteGuard<parking_lot::raw_rwlock::RawRwLock,egui::context::ContextImpl> >
14207e114: 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
14207e119: 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
14207e11e: 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
14207e124: 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
14207e12a: 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
14207e130: 4881c488000000 add rsp, 0x88
14207e137: 5b pop rbx
14207e138: 5f pop rdi
14207e139: 5e pop rsi
14207e13a: 415c pop r12
14207e13c: 415d pop r13
14207e13e: 415e pop r14
14207e140: 415f pop r15
14207e142: 5d pop rbp
14207e143: c3 ret 
14207e144: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14207e150: 4889542410 mov qword ptr [rsp + 0x10], rdx
14207e155: 55 push rbp
14207e156: 4157 push r15
14207e158: 4156 push r14
14207e15a: 4155 push r13
14207e15c: 4154 push r12
14207e15e: 56 push rsi
14207e15f: 57 push rdi
14207e160: 53 push rbx
14207e161: 4881ec88000000 sub rsp, 0x88
14207e168: 488daa80000000 lea rbp, [rdx + 0x80]
14207e16f: 440f29542430 movaps xmmword ptr [rsp + 0x30], xmm10
14207e175: 440f294c2440 movaps xmmword ptr [rsp + 0x40], xmm9
14207e17b: 440f29442450 movaps xmmword ptr [rsp + 0x50], xmm8
14207e181: 0f297c2460 movaps xmmword ptr [rsp + 0x60], xmm7
14207e186: 0f29742470 movaps xmmword ptr [rsp + 0x70], xmm6
14207e18b: 48837d2800 cmp qword ptr [rbp + 0x28], 0
14207e190: 741f je 0x14207e1b1
14207e192: 488d4530 lea rax, [rbp + 0x30]
14207e196: 48894508 mov qword ptr [rbp + 8], rax
14207e19a: 488d4508 lea rax, [rbp + 8]
14207e19e: 48894500 mov qword ptr [rbp], rax
14207e1a2: 488d0daf51f500 lea rcx, [rip + 0xf551af]
14207e1a9: 4889ea mov rdx, rbp
14207e1ac: e81f112bff call 0x14132f2d0 ; std::thread::local::LocalKey::with<core::cell::RefCell<puffin::ThreadProfiler>,puffin::impl$6::call::closure_env$0<tuple$<>,puffin::impl$10::drop::closure_env$0>,tuple$<> >
14207e1b1: 0f28742470 movaps xmm6, xmmword ptr [rsp + 0x70]
14207e1b6: 0f287c2460 movaps xmm7, xmmword ptr [rsp + 0x60]
14207e1bb: 440f28442450 movaps xmm8, xmmword ptr [rsp + 0x50]
14207e1c1: 440f284c2440 movaps xmm9, xmmword ptr [rsp + 0x40]
14207e1c7: 440f28542430 movaps xmm10, xmmword ptr [rsp + 0x30]
14207e1cd: 4881c488000000 add rsp, 0x88
14207e1d4: 5b pop rbx
14207e1d5: 5f pop rdi
14207e1d6: 5e pop rsi
14207e1d7: 415c pop r12
14207e1d9: 415d pop r13
14207e1db: 415e pop r14
14207e1dd: 415f pop r15
14207e1df: 5d pop rbp
14207e1e0: c3 ret 
