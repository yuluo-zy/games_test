0x1422c92c0 55 push rbp
0x1422c92c1 4157 push r15
0x1422c92c3 4156 push r14
0x1422c92c5 4154 push r12
0x1422c92c7 56 push rsi
0x1422c92c8 57 push rdi
0x1422c92c9 53 push rbx
0x1422c92ca 4881ecb0010000 sub rsp, 0x1b0
0x1422c92d1 488dac2480000000 lea rbp, [rsp + 0x80]
0x1422c92d9 0f29b520010000 movaps xmmword ptr [rbp + 0x120], xmm6
0x1422c92e0 48c78518010000feffffff mov qword ptr [rbp + 0x118], 0xfffffffffffffffe
0x1422c92eb 4889ce mov rsi, rcx
0x1422c92ee 4c8b6118 mov r12, qword ptr [rcx + 0x18]
0x1422c92f2 899504010000 mov dword ptr [rbp + 0x104], edx
0x1422c92f8 488dbd04010000 lea rdi, [rbp + 0x104]
0x1422c92ff 48897d00 mov qword ptr [rbp], rdi
0x1422c9303 488d1d7642acfe lea rbx, [rip - 0x153bd8a]
0x1422c930a 48895d08 mov qword ptr [rbp + 8], rbx
0x1422c930e 488d05e389d700 lea rax, [rip + 0xd789e3]
0x1422c9315 48898580000000 mov qword ptr [rbp + 0x80], rax
0x1422c931c 48c7858800000002000000 mov qword ptr [rbp + 0x88], 2
0x1422c9327 4989ef mov r15, rbp
0x1422c932a 4c89bd90000000 mov qword ptr [rbp + 0x90], r15
0x1422c9331 48c7859800000001000000 mov qword ptr [rbp + 0x98], 1
0x1422c933c 48c785a000000000000000 mov qword ptr [rbp + 0xa0], 0
0x1422c9347 488d4de8 lea rcx, [rbp - 0x18]
0x1422c934b 488d9580000000 lea rdx, [rbp + 0x80]
0x1422c9352 e8390cabfe call 0x140d79f90
0x1422c9357 488b45f0 mov rax, qword ptr [rbp - 0x10]
0x1422c935b 48898508010000 mov qword ptr [rbp + 0x108], rax
0x1422c9362 4c8b75f8 mov r14, qword ptr [rbp - 8]
0x1422c9366 48897d00 mov qword ptr [rbp], rdi
0x1422c936a 48895d08 mov qword ptr [rbp + 8], rbx
0x1422c936e 488d05c389d700 lea rax, [rip + 0xd789c3]
0x1422c9375 48898580000000 mov qword ptr [rbp + 0x80], rax
0x1422c937c 48c7858800000001000000 mov qword ptr [rbp + 0x88], 1
0x1422c9387 4c89bd90000000 mov qword ptr [rbp + 0x90], r15
0x1422c938e 48c7859800000001000000 mov qword ptr [rbp + 0x98], 1
0x1422c9399 48c785a000000000000000 mov qword ptr [rbp + 0xa0], 0
0x1422c93a4 488d4dd0 lea rcx, [rbp - 0x30]
0x1422c93a8 488d9580000000 lea rdx, [rbp + 0x80]
0x1422c93af e8dc0babfe call 0x140d79f90
0x1422c93b4 488b7dd8 mov rdi, qword ptr [rbp - 0x28]
0x1422c93b8 488b5de0 mov rbx, qword ptr [rbp - 0x20]
0x1422c93bc 0fb60546b83601 movzx eax, byte ptr [rip + 0x136b846]
0x1422c93c3 4d8b3c24 mov r15, qword ptr [r12]
0x1422c93c7 4d8b642408 mov r12, qword ptr [r12 + 8]
0x1422c93cc 488d8d80000000 lea rcx, [rbp + 0x80]
0x1422c93d3 488b9508010000 mov rdx, qword ptr [rbp + 0x108]
0x1422c93da 4d89f0 mov r8, r14
0x1422c93dd e84eeb3300 call 0x142607f30
0x1422c93e2 31c0 xor eax, eax
0x1422c93e4 483b8580000000 cmp rax, qword ptr [rbp + 0x80]
0x1422c93eb 0f80a6010000 jo 0x1422c9597
0x1422c93f1 488b85f0000000 mov rax, qword ptr [rbp + 0xf0]
0x1422c93f8 48894570 mov qword ptr [rbp + 0x70], rax
0x1422c93fc 0f1085e0000000 movups xmm0, xmmword ptr [rbp + 0xe0]
0x1422c9403 0f294560 movaps xmmword ptr [rbp + 0x60], xmm0
0x1422c9407 0f1085d0000000 movups xmm0, xmmword ptr [rbp + 0xd0]
0x1422c940e 0f294550 movaps xmmword ptr [rbp + 0x50], xmm0
0x1422c9412 0f1085c0000000 movups xmm0, xmmword ptr [rbp + 0xc0]
0x1422c9419 0f294540 movaps xmmword ptr [rbp + 0x40], xmm0
0x1422c941d 0f108580000000 movups xmm0, xmmword ptr [rbp + 0x80]
0x1422c9424 0f108d90000000 movups xmm1, xmmword ptr [rbp + 0x90]
0x1422c942b 0f1095a0000000 movups xmm2, xmmword ptr [rbp + 0xa0]
0x1422c9432 0f109db0000000 movups xmm3, xmmword ptr [rbp + 0xb0]
0x1422c9439 0f295d30 movaps xmmword ptr [rbp + 0x30], xmm3
0x1422c943d 0f295520 movaps xmmword ptr [rbp + 0x20], xmm2
0x1422c9441 0f294d10 movaps xmmword ptr [rbp + 0x10], xmm1
0x1422c9445 0f294500 movaps xmmword ptr [rbp], xmm0
0x1422c9449 c6851701000001 mov byte ptr [rbp + 0x117], 1
0x1422c9450 4889e9 mov rcx, rbp
0x1422c9453 e888ed73fe call 0x140a081e0
0x1422c9458 0f28f0 movaps xmm6, xmm0
0x1422c945b c6851701000001 mov byte ptr [rbp + 0x117], 1
0x1422c9462 488d4db0 lea rcx, [rbp - 0x50]
0x1422c9466 4889ea mov rdx, rbp
0x1422c9469 e802e673fe call 0x140a07a70
0x1422c946e 488b4570 mov rax, qword ptr [rbp + 0x70]
0x1422c9472 488985f0000000 mov qword ptr [rbp + 0xf0], rax
0x1422c9479 0f284560 movaps xmm0, xmmword ptr [rbp + 0x60]
0x1422c947d 0f2985e0000000 movaps xmmword ptr [rbp + 0xe0], xmm0
0x1422c9484 0f284550 movaps xmm0, xmmword ptr [rbp + 0x50]
0x1422c9488 0f2985d0000000 movaps xmmword ptr [rbp + 0xd0], xmm0
0x1422c948f 0f284540 movaps xmm0, xmmword ptr [rbp + 0x40]
0x1422c9493 0f2985c0000000 movaps xmmword ptr [rbp + 0xc0], xmm0
0x1422c949a 0f284500 movaps xmm0, xmmword ptr [rbp]
0x1422c949e 0f284d10 movaps xmm1, xmmword ptr [rbp + 0x10]
0x1422c94a2 0f285520 movaps xmm2, xmmword ptr [rbp + 0x20]
0x1422c94a6 0f285d30 movaps xmm3, xmmword ptr [rbp + 0x30]
0x1422c94aa 0f299db0000000 movaps xmmword ptr [rbp + 0xb0], xmm3
0x1422c94b1 0f2995a0000000 movaps xmmword ptr [rbp + 0xa0], xmm2
0x1422c94b8 0f298d90000000 movaps xmmword ptr [rbp + 0x90], xmm1
0x1422c94bf 0f298580000000 movaps xmmword ptr [rbp + 0x80], xmm0
0x1422c94c6 4d8b0f mov r9, qword ptr [r15]
0x1422c94c9 498b0424 mov rax, qword ptr [r12]
0x1422c94cd c6851701000000 mov byte ptr [rbp + 0x117], 0
0x1422c94d4 4889442420 mov qword ptr [rsp + 0x20], rax
0x1422c94d9 488d8d80000000 lea rcx, [rbp + 0x80]
0x1422c94e0 4889fa mov rdx, rdi
0x1422c94e3 4989d8 mov r8, rbx
0x1422c94e6 e8b51785fe call 0x140b1aca0
0x1422c94eb 4989c6 mov r14, rax
0x1422c94ee c6851701000000 mov byte ptr [rbp + 0x117], 0
0x1422c94f5 4889f9 mov rcx, rdi
0x1422c94f8 4889da mov rdx, rbx
0x1422c94fb e880389dfe call 0x140c9cd80
0x1422c9500 4989c4 mov r12, rax
0x1422c9503 0fb605ffb63601 movzx eax, byte ptr [rip + 0x136b6ff]
0x1422c950a b948000000 mov ecx, 0x48
0x1422c950f ba08000000 mov edx, 8
0x1422c9514 e8f7a634fe call 0x140613c10
0x1422c9519 4885c0 test rax, rax
0x1422c951c 0f84a9000000 je 0x1422c95cb
0x1422c9522 4989c7 mov r15, rax
0x1422c9525 0f1045b0 movups xmm0, xmmword ptr [rbp - 0x50]
0x1422c9529 0f104dc0 movups xmm1, xmmword ptr [rbp - 0x40]
0x1422c952d 0f114810 movups xmmword ptr [rax + 0x10], xmm1
0x1422c9531 0f1100 movups xmmword ptr [rax], xmm0
0x1422c9534 48897820 mov qword ptr [rax + 0x20], rdi
0x1422c9538 48895828 mov qword ptr [rax + 0x28], rbx
0x1422c953c 4c897030 mov qword ptr [rax + 0x30], r14
0x1422c9540 4c896038 mov qword ptr [rax + 0x38], r12
0x1422c9544 f30f117040 movss dword ptr [rax + 0x40], xmm6
0x1422c9549 c740441f090000 mov dword ptr [rax + 0x44], 0x91f
0x1422c9550 488b55e8 mov rdx, qword ptr [rbp - 0x18]
0x1422c9554 4885d2 test rdx, rdx
0x1422c9557 488b8d08010000 mov rcx, qword ptr [rbp + 0x108]
0x1422c955e 740b je 0x1422c956b
0x1422c9560 41b801000000 mov r8d, 1
0x1422c9566 e8b5a634fe call 0x140613c20
0x1422c956b 488b4608 mov rax, qword ptr [rsi + 8]
0x1422c956f 488b4e10 mov rcx, qword ptr [rsi + 0x10]
0x1422c9573 4c893cc1 mov qword ptr [rcx + rax*8], r15
0x1422c9577 48ffc0 inc rax
0x1422c957a 48894608 mov qword ptr [rsi + 8], rax
0x1422c957e 0f28b520010000 movaps xmm6, xmmword ptr [rbp + 0x120]
0x1422c9585 4881c4b0010000 add rsp, 0x1b0
0x1422c958c 5b pop rbx
0x1422c958d 5f pop rdi
0x1422c958e 5e pop rsi
0x1422c958f 415c pop r12
0x1422c9591 415e pop r14
0x1422c9593 415f pop r15
0x1422c9595 5d pop rbp
0x1422c9596 c3 ret 
0x1422c9597 488b8588000000 mov rax, qword ptr [rbp + 0x88]
0x1422c959e 48894500 mov qword ptr [rbp], rax
0x1422c95a2 488d050787d700 lea rax, [rip + 0xd78707]
0x1422c95a9 4889442420 mov qword ptr [rsp + 0x20], rax
0x1422c95ae 488d0d9386d700 lea rcx, [rip + 0xd78693]
0x1422c95b5 4c8d0d6c86d700 lea r9, [rip + 0xd7866c]
0x1422c95bc 4989e8 mov r8, rbp
0x1422c95bf ba2b000000 mov edx, 0x2b
0x1422c95c4 e897016100 call 0x1428d9760
0x1422c95c9 eb16 jmp 0x1422c95e1
0x1422c95cb c6851701000000 mov byte ptr [rbp + 0x117], 0
0x1422c95d2 b908000000 mov ecx, 8
0x1422c95d7 ba48000000 mov edx, 0x48
0x1422c95dc e81ffa6000 call 0x1428d9000
0x1422c95e1 0f0b ud2 
0x1422c95e3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1422c95f0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1422c95f5 55 push rbp
0x1422c95f6 4157 push r15
0x1422c95f8 4156 push r14
0x1422c95fa 4154 push r12
0x1422c95fc 56 push rsi
0x1422c95fd 57 push rdi
0x1422c95fe 53 push rbx
0x1422c95ff 4883ec40 sub rsp, 0x40
0x1422c9603 488daa80000000 lea rbp, [rdx + 0x80]
0x1422c960a 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
0x1422c960f 488b8d08010000 mov rcx, qword ptr [rbp + 0x108]
0x1422c9616 488b55e8 mov rdx, qword ptr [rbp - 0x18]
0x1422c961a 4885d2 test rdx, rdx
0x1422c961d 740b je 0x1422c962a
0x1422c961f 41b801000000 mov r8d, 1
0x1422c9625 e8f6a534fe call 0x140613c20
0x1422c962a 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
0x1422c962f 4883c440 add rsp, 0x40
0x1422c9633 5b pop rbx
0x1422c9634 5f pop rdi
0x1422c9635 5e pop rsi
0x1422c9636 415c pop r12
0x1422c9638 415e pop r14
0x1422c963a 415f pop r15
0x1422c963c 5d pop rbp
0x1422c963d c3 ret 
0x1422c963e 6690 nop 
0x1422c9640 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1422c9645 55 push rbp
0x1422c9646 4157 push r15
0x1422c9648 4156 push r14
0x1422c964a 4154 push r12
0x1422c964c 56 push rsi
0x1422c964d 57 push rdi
0x1422c964e 53 push rbx
0x1422c964f 4883ec40 sub rsp, 0x40
0x1422c9653 488daa80000000 lea rbp, [rdx + 0x80]
0x1422c965a 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
0x1422c965f 4889e9 mov rcx, rbp
0x1422c9662 e8c935a8fe call 0x140d4cc30
0x1422c9667 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
0x1422c966c 4883c440 add rsp, 0x40
0x1422c9670 5b pop rbx
0x1422c9671 5f pop rdi
0x1422c9672 5e pop rsi
0x1422c9673 415c pop r12
0x1422c9675 415e pop r14
0x1422c9677 415f pop r15
0x1422c9679 5d pop rbp
0x1422c967a c3 ret 
0x1422c967b 0f1f440000 nop dword ptr [rax + rax]
0x1422c9680 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1422c9685 55 push rbp
0x1422c9686 4157 push r15
0x1422c9688 4156 push r14
0x1422c968a 4154 push r12
0x1422c968c 56 push rsi
0x1422c968d 57 push rdi
0x1422c968e 53 push rbx
0x1422c968f 4883ec40 sub rsp, 0x40
0x1422c9693 488daa80000000 lea rbp, [rdx + 0x80]
0x1422c969a 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
0x1422c969f 80bd1701000000 cmp byte ptr [rbp + 0x117], 0
0x1422c96a6 7408 je 0x1422c96b0
0x1422c96a8 4889e9 mov rcx, rbp
0x1422c96ab e870b4c0fe call 0x140ed4b20
0x1422c96b0 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
0x1422c96b5 4883c440 add rsp, 0x40
0x1422c96b9 5b pop rbx
0x1422c96ba 5f pop rdi
0x1422c96bb 5e pop rsi
0x1422c96bc 415c pop r12
0x1422c96be 415e pop r14
0x1422c96c0 415f pop r15
0x1422c96c2 5d pop rbp
0x1422c96c3 c3 ret 
