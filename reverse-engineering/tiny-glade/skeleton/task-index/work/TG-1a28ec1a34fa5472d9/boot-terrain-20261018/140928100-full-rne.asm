140928100: push r15
140928102: push r14
140928104: push r13
140928106: push r12
140928108: push rsi
140928109: push rdi
14092810a: push rbp
14092810b: push rbx
14092810c: sub rsp, 0x58
140928110: cmp r8d, edx
140928113: jb 0x140928337
140928119: mov edi, r9d
14092811c: mov r14d, r8d
14092811f: mov rbx, rcx
140928122: mov r13d, dword ptr [rsp + 0xc0]
14092812a: lea esi, [r14 + 1]
14092812e: mov eax, esi
140928130: sub eax, edx
140928132: cmp eax, 6
140928135: jae 0x140928140
140928137: mov r15d, edx
14092813a: jmp 0x14092816b
14092813c: nop dword ptr [rax]
140928140: lea r15d, [rdx + 4]
140928144: cmp r14d, r15d
140928147: cmovb r15d, r14d
14092814b: mov dword ptr [rsp + 0x20], r13d
140928150: mov rcx, rbx
140928153: mov r8d, r15d
140928156: mov r9d, edi
140928159: call 0x140928100 ; country_core::startup::startup_terrain::grid_gen_cw
14092815e: mov eax, esi
140928160: sub eax, r15d
140928163: mov edx, r15d
140928166: cmp eax, 6
140928169: jae 0x140928140
14092816b: mov qword ptr [rsp + 0x28], r15
140928170: test edi, edi
140928172: je 0x140928326
140928178: cmp dword ptr [rsp + 0x28], r14d
14092817d: jae 0x140928326
140928183: lea eax, [r13 + 1]
140928187: mov r15, qword ptr [rbx + 0x10]
14092818b: add r13d, 2
14092818f: xor ecx, ecx
140928191: mov dword ptr [rsp + 0x30], eax
140928195: xor edx, edx
140928197: mov dword ptr [rsp + 0x34], edi
14092819b: mov qword ptr [rsp + 0x48], r14
1409281a0: jmp 0x1409281e6
1409281a2: nop word ptr cs:[rax + rax]
1409281b0: add r12, 6
1409281b4: mov eax, dword ptr [rsp + 0x30]
1409281b8: mov edx, dword ptr [rsp + 0x3c]
1409281bc: add edx, eax
1409281be: mov ecx, dword ptr [rsp + 0x40]
1409281c2: add ecx, eax
1409281c4: mov r13, qword ptr [rsp + 0x50]
1409281c9: add r13d, eax
1409281cc: mov eax, edx
1409281ce: mov r15, r12
1409281d1: mov edi, dword ptr [rsp + 0x34]
1409281d5: mov edx, dword ptr [rsp + 0x38]
1409281d9: cmp edx, edi
1409281db: mov r14, qword ptr [rsp + 0x48]
1409281e0: je 0x140928326
1409281e6: inc edx
1409281e8: mov dword ptr [rsp + 0x38], edx
1409281ec: mov qword ptr [rsp + 0x50], r13
1409281f1: mov ebp, r14d
1409281f4: mov dword ptr [rsp + 0x40], ecx
1409281f8: mov esi, ecx
1409281fa: mov dword ptr [rsp + 0x3c], eax
1409281fe: mov edi, eax
140928200: mov r14, qword ptr [rsp + 0x28]
140928205: jmp 0x140928236
140928207: nop word ptr [rax + rax]
140928210: lea eax, [r14 + r13]
140928214: mov rcx, qword ptr [rbx + 8]
140928218: mov dword ptr [rcx + r12*4 + 0x14], eax
14092821d: inc r15
140928220: mov qword ptr [rbx + 0x10], r15
140928224: inc edi
140928226: inc esi
140928228: dec ebp
14092822a: inc r13d
14092822d: cmp r14d, ebp
140928230: je 0x1409281b0
140928236: mov r12, r15
140928239: cmp r15, qword ptr [rbx]
14092823c: jne 0x14092824d
14092823e: mov rcx, rbx
140928241: lea rdx, [rip + 0x21870c8]
140928248: call 0x1416a83a0 ; alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>
14092824d: lea eax, [r14 + rsi]
140928251: mov rcx, qword ptr [rbx + 8]
140928255: mov dword ptr [rcx + r12*4], eax
140928259: lea r15, [r12 + 1]
14092825e: mov qword ptr [rbx + 0x10], r15
140928262: cmp r15, qword ptr [rbx]
140928265: jne 0x140928276
140928267: mov rcx, rbx
14092826a: lea rdx, [rip + 0x21870b7]
140928271: call 0x1416a83a0 ; alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>
140928276: lea ecx, [r14 + rdi]
14092827a: mov rax, qword ptr [rbx + 8]
14092827e: mov dword ptr [rsp + 0x44], ecx
140928282: mov dword ptr [rax + r12*4 + 4], ecx
140928287: inc r15
14092828a: mov qword ptr [rbx + 0x10], r15
14092828e: cmp r15, qword ptr [rbx]
140928291: jne 0x1409282a2
140928293: mov rcx, rbx
140928296: lea rdx, [rip + 0x21870a3]
14092829d: call 0x1416a83a0 ; alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>
1409282a2: add r14d, esi
1409282a5: inc r14d
1409282a8: mov rax, qword ptr [rbx + 8]
1409282ac: mov dword ptr [rax + r12*4 + 8], r14d
1409282b1: inc r15
1409282b4: mov qword ptr [rbx + 0x10], r15
1409282b8: cmp r15, qword ptr [rbx]
1409282bb: jne 0x1409282cc
1409282bd: mov rcx, rbx
1409282c0: lea rdx, [rip + 0x2187091]
1409282c7: call 0x1416a83a0 ; alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>
1409282cc: mov rax, qword ptr [rbx + 8]
1409282d0: mov dword ptr [rax + r12*4 + 0xc], r14d
1409282d5: inc r15
1409282d8: mov qword ptr [rbx + 0x10], r15
1409282dc: cmp r15, qword ptr [rbx]
1409282df: jne 0x1409282f0
1409282e1: mov rcx, rbx
1409282e4: lea rdx, [rip + 0x2187085]
1409282eb: call 0x1416a83a0 ; alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>
1409282f0: mov rax, qword ptr [rbx + 8]
1409282f4: mov ecx, dword ptr [rsp + 0x44]
1409282f8: mov dword ptr [rax + r12*4 + 0x10], ecx
1409282fd: inc r15
140928300: mov qword ptr [rbx + 0x10], r15
140928304: cmp r15, qword ptr [rbx]
140928307: mov r14, qword ptr [rsp + 0x28]
14092830c: jne 0x140928210
140928312: mov rcx, rbx
140928315: lea rdx, [rip + 0x218706c]
14092831c: call 0x1416a83a0 ; alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>
140928321: jmp 0x140928210
140928326: add rsp, 0x58
14092832a: pop rbx
14092832b: pop rbp
14092832c: pop rdi
14092832d: pop rsi
14092832e: pop r12
140928330: pop r13
140928332: pop r14
140928334: pop r15
140928336: ret 
140928337: lea rcx, [rip + 0x2186fba]
14092833e: call 0x1428d9310 ; core::option::unwrap_failed
140928343: int3 
