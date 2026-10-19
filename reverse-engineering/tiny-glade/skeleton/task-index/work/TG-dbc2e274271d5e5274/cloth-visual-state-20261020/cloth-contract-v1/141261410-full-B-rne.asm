141261410: 55 push rbp
141261411: 4157 push r15
141261413: 4156 push r14
141261415: 4154 push r12
141261417: 56 push rsi
141261418: 57 push rdi
141261419: 53 push rbx
14126141a: 4883ec60 sub rsp, 0x60
14126141e: 488d6c2460 lea rbp, [rsp + 0x60]
141261423: 48c745f8feffffff mov qword ptr [rbp - 8], 0xfffffffffffffffe
14126142b: 4d89c6 mov r14, r8
14126142e: 4929d6 sub r14, rdx
141261431: 48b8f0ffffffffffff7f movabs rax, 0x7ffffffffffffff0
14126143b: 4939c6 cmp r14, rax
14126143e: 0f879f000000 ja 0x1412614e3
141261444: 4c89c7 mov rdi, r8
141261447: 4889d3 mov rbx, rdx
14126144a: 4889ce mov rsi, rcx
14126144d: 4939d0 cmp r8, rdx
141261450: 7436 je 0x141261488
141261452: 4d89cf mov r15, r9
141261455: 0fb605ad373d02 movzx eax, byte ptr [rip + 0x23d37ad]
14126145c: 41bc10000000 mov r12d, 0x10
141261462: ba10000000 mov edx, 0x10
141261467: 4c89f1 mov rcx, r14
14126146a: e8a1273bff call 0x140613c10 ; unknown
14126146f: 4885c0 test rax, rax
141261472: 7475 je 0x1412614e9
141261474: 49c1ee04 shr r14, 4
141261478: 48b9cdcccccccccccccc movabs rcx, 0xcccccccccccccccd
141261482: 490fafce imul rcx, r14
141261486: eb07 jmp 0x14126148f ; unknown
141261488: b810000000 mov eax, 0x10
14126148d: 31c9 xor ecx, ecx
14126148f: 48894de0 mov qword ptr [rbp - 0x20], rcx
141261493: 488945e8 mov qword ptr [rbp - 0x18], rax
141261497: 488d4df0 lea rcx, [rbp - 0x10]
14126149b: 48c745f000000000 mov qword ptr [rbp - 0x10], 0
1412614a3: 48894dc8 mov qword ptr [rbp - 0x38], rcx
1412614a7: 48c745d000000000 mov qword ptr [rbp - 0x30], 0
1412614af: 488945d8 mov qword ptr [rbp - 0x28], rax
1412614b3: 4c8d45c8 lea r8, [rbp - 0x38]
1412614b7: 4889d9 mov rcx, rbx
1412614ba: 4889fa mov rdx, rdi
1412614bd: e86e220300 call 0x141293730 ; core::iter::adapters::map::impl$2::fold<system_decorator::decorator_visual::flag_connections::CandidateInfo,core::slice::iter::Iter<system_decorator::decorator_visual_state::ClothInfo>,system_decorator::decorator_visual:
1412614c2: 488b45f0 mov rax, qword ptr [rbp - 0x10]
1412614c6: 48894610 mov qword ptr [rsi + 0x10], rax
1412614ca: 0f1045e0 movups xmm0, xmmword ptr [rbp - 0x20]
1412614ce: 0f1106 movups xmmword ptr [rsi], xmm0
1412614d1: 4889f0 mov rax, rsi
1412614d4: 4883c460 add rsp, 0x60
1412614d8: 5b pop rbx
1412614d9: 5f pop rdi
1412614da: 5e pop rsi
1412614db: 415c pop r12
1412614dd: 415e pop r14
1412614df: 415f pop r15
1412614e1: 5d pop rbp
1412614e2: c3 ret 
1412614e3: 4d89cf mov r15, r9
1412614e6: 4531e4 xor r12d, r12d
1412614e9: 4c89e1 mov rcx, r12
1412614ec: 4c89f2 mov rdx, r14
1412614ef: 4d89f8 mov r8, r15
1412614f2: e8ec7a6701 call 0x1428d8fe3 ; alloc::raw_vec::handle_error
1412614f7: cc int3 
1412614f8: 0f1f840000000000 nop dword ptr [rax + rax]
141261500: 4889542410 mov qword ptr [rsp + 0x10], rdx
141261505: 55 push rbp
141261506: 4157 push r15
141261508: 4156 push r14
14126150a: 4154 push r12
14126150c: 56 push rsi
14126150d: 57 push rdi
14126150e: 53 push rbx
14126150f: 4883ec20 sub rsp, 0x20
141261513: 488d6a60 lea rbp, [rdx + 0x60]
141261517: 488b45e0 mov rax, qword ptr [rbp - 0x20]
14126151b: 4885c0 test rax, rax
14126151e: 7417 je 0x141261537
141261520: 488b4de8 mov rcx, qword ptr [rbp - 0x18]
141261524: 48c1e004 shl rax, 4
141261528: 488d1480 lea rdx, [rax + rax*4]
14126152c: 41b810000000 mov r8d, 0x10
141261532: e8e9263bff call 0x140613c20 ; unknown
141261537: 90 nop 
141261538: 4883c420 add rsp, 0x20
14126153c: 5b pop rbx
14126153d: 5f pop rdi
14126153e: 5e pop rsi
14126153f: 415c pop r12
141261541: 415e pop r14
141261543: 415f pop r15
141261545: 5d pop rbp
141261546: c3 ret 
