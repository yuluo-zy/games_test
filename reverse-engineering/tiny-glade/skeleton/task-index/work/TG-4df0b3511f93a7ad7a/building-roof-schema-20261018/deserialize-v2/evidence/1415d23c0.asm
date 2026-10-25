0x1415d23c0 55 push rbp
0x1415d23c1 4156 push r14
0x1415d23c3 56 push rsi
0x1415d23c4 57 push rdi
0x1415d23c5 53 push rbx
0x1415d23c6 4881ecb0000000 sub rsp, 0xb0
0x1415d23cd 488dac2480000000 lea rbp, [rsp + 0x80]
0x1415d23d5 48c74528feffffff mov qword ptr [rbp + 0x28], 0xfffffffffffffffe
0x1415d23dd 4889ce mov rsi, rcx
0x1415d23e0 0f104218 movups xmm0, xmmword ptr [rdx + 0x18]
0x1415d23e4 0f104a28 movups xmm1, xmmword ptr [rdx + 0x28]
0x1415d23e8 0f294d00 movaps xmmword ptr [rbp], xmm1
0x1415d23ec 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
0x1415d23f0 4c8b0a mov r9, qword ptr [rdx]
0x1415d23f3 488b4a08 mov rcx, qword ptr [rdx + 8]
0x1415d23f7 488b5210 mov rdx, qword ptr [rdx + 0x10]
0x1415d23fb 4883fa05 cmp rdx, 5
0x1415d23ff 752d jne 0x1415d242e
0x1415d2401 8b01 mov eax, dword ptr [rcx]
0x1415d2403 3305bfd66b01 xor eax, dword ptr [rip + 0x16bd6bf]
0x1415d2409 440fb64104 movzx r8d, byte ptr [rcx + 4]
0x1415d240e 443205b7d66b01 xor r8b, byte ptr [rip + 0x16bd6b7]
0x1415d2415 450fb6c0 movzx r8d, r8b
0x1415d2419 4109c0 or r8d, eax
0x1415d241c 0f85b2000000 jne 0x1415d24d4
0x1415d2422 31db xor ebx, ebx
0x1415d2424 4531f6 xor r14d, r14d
0x1415d2427 4d85c9 test r9, r9
0x1415d242a 7534 jne 0x1415d2460
0x1415d242c eb40 jmp 0x1415d246e
0x1415d242e 4883fa06 cmp rdx, 6
0x1415d2432 0f859c000000 jne 0x1415d24d4
0x1415d2438 8b01 mov eax, dword ptr [rcx]
0x1415d243a 33058dd66b01 xor eax, dword ptr [rip + 0x16bd68d]
0x1415d2440 440fb74104 movzx r8d, word ptr [rcx + 4]
0x1415d2445 6644330584d66b01 xor r8w, word ptr [rip + 0x16bd684]
0x1415d244d 450fb7c0 movzx r8d, r8w
0x1415d2451 4109c0 or r8d, eax
0x1415d2454 757e jne 0x1415d24d4
0x1415d2456 b301 mov bl, 1
0x1415d2458 4531f6 xor r14d, r14d
0x1415d245b 4d85c9 test r9, r9
0x1415d245e 740e je 0x1415d246e
0x1415d2460 41b801000000 mov r8d, 1
0x1415d2466 4c89ca mov rdx, r9
0x1415d2469 e8b21704ff call 0x140613c20
0x1415d246e 0f2845f0 movaps xmm0, xmmword ptr [rbp - 0x10]
0x1415d2472 0f284d00 movaps xmm1, xmmword ptr [rbp]
0x1415d2476 0f294db0 movaps xmmword ptr [rbp - 0x50], xmm1
0x1415d247a 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
0x1415d247e 4584f6 test r14b, r14b
0x1415d2481 7416 je 0x1415d2499
0x1415d2483 48897e08 mov qword ptr [rsi + 8], rdi
0x1415d2487 c60602 mov byte ptr [rsi], 2
0x1415d248a c6452700 mov byte ptr [rbp + 0x27], 0
0x1415d248e 488d4da0 lea rcx, [rbp - 0x60]
0x1415d2492 e899bf2bff call 0x14088e430
0x1415d2497 eb2a jmp 0x1415d24c3
0x1415d2499 0f2845f0 movaps xmm0, xmmword ptr [rbp - 0x10]
0x1415d249d 0f284d00 movaps xmm1, xmmword ptr [rbp]
0x1415d24a1 0f114de0 movups xmmword ptr [rbp - 0x20], xmm1
0x1415d24a5 0f1145d0 movups xmmword ptr [rbp - 0x30], xmm0
0x1415d24a9 881e mov byte ptr [rsi], bl
0x1415d24ab 0f1045c9 movups xmm0, xmmword ptr [rbp - 0x37]
0x1415d24af 0f104dd9 movups xmm1, xmmword ptr [rbp - 0x27]
0x1415d24b3 0f114601 movups xmmword ptr [rsi + 1], xmm0
0x1415d24b7 0f114e11 movups xmmword ptr [rsi + 0x11], xmm1
0x1415d24bb 488b45e8 mov rax, qword ptr [rbp - 0x18]
0x1415d24bf 48894620 mov qword ptr [rsi + 0x20], rax
0x1415d24c3 4889f0 mov rax, rsi
0x1415d24c6 4881c4b0000000 add rsp, 0xb0
0x1415d24cd 5b pop rbx
0x1415d24ce 5f pop rdi
0x1415d24cf 5e pop rsi
0x1415d24d0 415e pop r14
0x1415d24d2 5d pop rbp
0x1415d24d3 c3 ret 
0x1415d24d4 4c894d18 mov qword ptr [rbp + 0x18], r9
0x1415d24d8 4c8d05f9d56b01 lea r8, [rip + 0x16bd5f9]
0x1415d24df 41b902000000 mov r9d, 2
0x1415d24e5 48894d10 mov qword ptr [rbp + 0x10], rcx
0x1415d24e9 e832793201 call 0x1428f9e20
0x1415d24ee 4889c7 mov rdi, rax
0x1415d24f1 41b601 mov r14b, 1
0x1415d24f4 4c8b4d18 mov r9, qword ptr [rbp + 0x18]
0x1415d24f8 488b4d10 mov rcx, qword ptr [rbp + 0x10]
0x1415d24fc 4d85c9 test r9, r9
0x1415d24ff 0f855bffffff jne 0x1415d2460
0x1415d2505 e964ffffff jmp 0x1415d246e
0x1415d250a 660f1f440000 nop word ptr [rax + rax]
0x1415d2510 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415d2515 55 push rbp
0x1415d2516 4156 push r14
0x1415d2518 56 push rsi
0x1415d2519 57 push rdi
0x1415d251a 53 push rbx
0x1415d251b 4883ec20 sub rsp, 0x20
0x1415d251f 488daa80000000 lea rbp, [rdx + 0x80]
0x1415d2526 48837d1800 cmp qword ptr [rbp + 0x18], 0
0x1415d252b 7413 je 0x1415d2540
0x1415d252d 41b801000000 mov r8d, 1
0x1415d2533 488b4d10 mov rcx, qword ptr [rbp + 0x10]
0x1415d2537 488b5518 mov rdx, qword ptr [rbp + 0x18]
0x1415d253b e8e01604ff call 0x140613c20
0x1415d2540 c6452701 mov byte ptr [rbp + 0x27], 1
0x1415d2544 4883c420 add rsp, 0x20
0x1415d2548 5b pop rbx
0x1415d2549 5f pop rdi
0x1415d254a 5e pop rsi
0x1415d254b 415e pop r14
0x1415d254d 5d pop rbp
0x1415d254e c3 ret 
0x1415d254f 90 nop 
0x1415d2550 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415d2555 55 push rbp
0x1415d2556 4156 push r14
0x1415d2558 56 push rsi
0x1415d2559 57 push rdi
0x1415d255a 53 push rbx
0x1415d255b 4883ec20 sub rsp, 0x20
0x1415d255f 488daa80000000 lea rbp, [rdx + 0x80]
0x1415d2566 807d2700 cmp byte ptr [rbp + 0x27], 0
0x1415d256a 7409 je 0x1415d2575
0x1415d256c 488d4df0 lea rcx, [rbp - 0x10]
0x1415d2570 e8bbbe2bff call 0x14088e430
0x1415d2575 90 nop 
0x1415d2576 4883c420 add rsp, 0x20
0x1415d257a 5b pop rbx
0x1415d257b 5f pop rdi
0x1415d257c 5e pop rsi
0x1415d257d 415e pop r14
0x1415d257f 5d pop rbp
0x1415d2580 c3 ret 
