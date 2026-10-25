140928350: push r15
140928352: push r14
140928354: push r13
140928356: push r12
140928358: push rsi
140928359: push rdi
14092835a: push rbp
14092835b: push rbx
14092835c: sub rsp, 0x58
140928360: cmp r8d, edx
140928363: jb 0x140928586
140928369: mov edi, r9d
14092836c: mov r14d, r8d
14092836f: mov rbx, rcx
140928372: mov r13d, dword ptr [rsp + 0xc0]
14092837a: lea esi, [r14 + 1]
14092837e: mov eax, esi
140928380: sub eax, edx
140928382: cmp eax, 6
140928385: jae 0x140928390
140928387: mov r15d, edx
14092838a: jmp 0x1409283bb
14092838c: nop dword ptr [rax]
140928390: lea r15d, [rdx + 4]
140928394: cmp r14d, r15d
140928397: cmovb r15d, r14d
14092839b: mov dword ptr [rsp + 0x20], r13d
1409283a0: mov rcx, rbx
1409283a3: mov r8d, r15d
1409283a6: mov r9d, edi
1409283a9: call 0x140928350 ; country_core::startup::startup_terrain::grid_gen_ccw
1409283ae: mov eax, esi
1409283b0: sub eax, r15d
1409283b3: mov edx, r15d
1409283b6: cmp eax, 6
1409283b9: jae 0x140928390
1409283bb: mov qword ptr [rsp + 0x28], r15
1409283c0: test edi, edi
1409283c2: je 0x140928575
1409283c8: cmp dword ptr [rsp + 0x28], r14d
1409283cd: jae 0x140928575
1409283d3: lea eax, [r13 + 1]
1409283d7: mov r15, qword ptr [rbx + 0x10]
1409283db: add r13d, 2
1409283df: xor ecx, ecx
1409283e1: mov dword ptr [rsp + 0x30], eax
1409283e5: xor edx, edx
1409283e7: mov dword ptr [rsp + 0x34], edi
1409283eb: mov qword ptr [rsp + 0x48], r14
1409283f0: jmp 0x140928436
1409283f2: nop word ptr cs:[rax + rax]
140928400: add r12, 6
140928404: mov eax, dword ptr [rsp + 0x30]
140928408: mov edx, dword ptr [rsp + 0x3c]
14092840c: add edx, eax
14092840e: mov ecx, dword ptr [rsp + 0x40]
140928412: add ecx, eax
140928414: mov r13, qword ptr [rsp + 0x50]
140928419: add r13d, eax
14092841c: mov eax, edx
14092841e: mov r15, r12
140928421: mov edi, dword ptr [rsp + 0x34]
140928425: mov edx, dword ptr [rsp + 0x38]
140928429: cmp edx, edi
14092842b: mov r14, qword ptr [rsp + 0x48]
140928430: je 0x140928575
140928436: inc edx
140928438: mov dword ptr [rsp + 0x38], edx
14092843c: mov qword ptr [rsp + 0x50], r13
140928441: mov ebp, r14d
140928444: mov dword ptr [rsp + 0x40], ecx
140928448: mov esi, ecx
14092844a: mov dword ptr [rsp + 0x3c], eax
14092844e: mov edi, eax
140928450: mov r14, qword ptr [rsp + 0x28]
140928455: jmp 0x140928486
140928457: nop word ptr [rax + rax]
140928460: lea eax, [r14 + r13]
140928464: mov rcx, qword ptr [rbx + 8]
140928468: mov dword ptr [rcx + r12*4 + 0x14], eax
14092846d: inc r15
140928470: mov qword ptr [rbx + 0x10], r15
140928474: inc edi
140928476: inc esi
140928478: dec ebp
14092847a: inc r13d
14092847d: cmp r14d, ebp
140928480: je 0x140928400
140928486: mov r12, r15
140928489: cmp r15, qword ptr [rbx]
14092848c: jne 0x14092849d
14092848e: mov rcx, rbx
140928491: lea rdx, [rip + 0x2186f28]
140928498: call 0x1416a83a0 ; alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>
14092849d: lea eax, [r14 + rsi]
1409284a1: mov rcx, qword ptr [rbx + 8]
1409284a5: mov dword ptr [rcx + r12*4], eax
1409284a9: lea r15, [r12 + 1]
1409284ae: mov qword ptr [rbx + 0x10], r15
1409284b2: cmp r15, qword ptr [rbx]
1409284b5: jne 0x1409284c6
1409284b7: mov rcx, rbx
1409284ba: lea rdx, [rip + 0x2186f17]
1409284c1: call 0x1416a83a0 ; alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>
1409284c6: lea ecx, [r14 + rsi]
1409284ca: inc ecx
1409284cc: mov rax, qword ptr [rbx + 8]
1409284d0: mov dword ptr [rsp + 0x44], ecx
1409284d4: mov dword ptr [rax + r12*4 + 4], ecx
1409284d9: inc r15
1409284dc: mov qword ptr [rbx + 0x10], r15
1409284e0: cmp r15, qword ptr [rbx]
1409284e3: jne 0x1409284f4
1409284e5: mov rcx, rbx
1409284e8: lea rdx, [rip + 0x2186f01]
1409284ef: call 0x1416a83a0 ; alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>
1409284f4: add r14d, edi
1409284f7: mov rax, qword ptr [rbx + 8]
1409284fb: mov dword ptr [rax + r12*4 + 8], r14d
140928500: inc r15
140928503: mov qword ptr [rbx + 0x10], r15
140928507: cmp r15, qword ptr [rbx]
14092850a: jne 0x14092851b
14092850c: mov rcx, rbx
14092850f: lea rdx, [rip + 0x2186ef2]
140928516: call 0x1416a83a0 ; alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>
14092851b: mov rax, qword ptr [rbx + 8]
14092851f: mov dword ptr [rax + r12*4 + 0xc], r14d
140928524: inc r15
140928527: mov qword ptr [rbx + 0x10], r15
14092852b: cmp r15, qword ptr [rbx]
14092852e: jne 0x14092853f
140928530: mov rcx, rbx
140928533: lea rdx, [rip + 0x2186ee6]
14092853a: call 0x1416a83a0 ; alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>
14092853f: mov rax, qword ptr [rbx + 8]
140928543: mov ecx, dword ptr [rsp + 0x44]
140928547: mov dword ptr [rax + r12*4 + 0x10], ecx
14092854c: inc r15
14092854f: mov qword ptr [rbx + 0x10], r15
140928553: cmp r15, qword ptr [rbx]
140928556: mov r14, qword ptr [rsp + 0x28]
14092855b: jne 0x140928460
140928561: mov rcx, rbx
140928564: lea rdx, [rip + 0x2186ecd]
14092856b: call 0x1416a83a0 ; alloc::raw_vec::RawVec::grow_one<u32,alloc::alloc::Global>
140928570: jmp 0x140928460
140928575: add rsp, 0x58
140928579: pop rbx
14092857a: pop rbp
14092857b: pop rdi
14092857c: pop rsi
14092857d: pop r12
14092857f: pop r13
140928581: pop r14
140928583: pop r15
140928585: ret 
140928586: lea rcx, [rip + 0x2186e1b]
14092858d: call 0x1428d9310 ; core::option::unwrap_failed
140928592: int3 
