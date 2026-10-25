0x1415a9230 55 push rbp
0x1415a9231 4157 push r15
0x1415a9233 4156 push r14
0x1415a9235 4155 push r13
0x1415a9237 4154 push r12
0x1415a9239 56 push rsi
0x1415a923a 57 push rdi
0x1415a923b 53 push rbx
0x1415a923c 4881ec38010000 sub rsp, 0x138
0x1415a9243 488dac2480000000 lea rbp, [rsp + 0x80]
0x1415a924b 0f29b5a0000000 movaps xmmword ptr [rbp + 0xa0], xmm6
0x1415a9252 48c78598000000feffffff mov qword ptr [rbp + 0x98], 0xfffffffffffffffe
0x1415a925d 4889ce mov rsi, rcx
0x1415a9260 4c8b7a10 mov r15, qword ptr [rdx + 0x10]
0x1415a9264 488d4d48 lea rcx, [rbp + 0x48]
0x1415a9268 e813ee5dff call 0x140b88080
0x1415a926d 488b5d50 mov rbx, qword ptr [rbp + 0x50]
0x1415a9271 488b7d60 mov rdi, qword ptr [rbp + 0x60]
0x1415a9275 4839fb cmp rbx, rdi
0x1415a9278 7469 je 0x1415a92e3
0x1415a927a 4c8d6b20 lea r13, [rbx + 0x20]
0x1415a927e 4c896d50 mov qword ptr [rbp + 0x50], r13
0x1415a9282 0fb603 movzx eax, byte ptr [rbx]
0x1415a9285 3c06 cmp al, 6
0x1415a9287 745d je 0x1415a92e6
0x1415a9289 884570 mov byte ptr [rbp + 0x70], al
0x1415a928c 0f104301 movups xmm0, xmmword ptr [rbx + 1]
0x1415a9290 0f104b10 movups xmm1, xmmword ptr [rbx + 0x10]
0x1415a9294 0f114571 movups xmmword ptr [rbp + 0x71], xmm0
0x1415a9298 0f118d80000000 movups xmmword ptr [rbp + 0x80], xmm1
0x1415a929f 488d4d70 lea rcx, [rbp + 0x70]
0x1415a92a3 e8e83effff call 0x14159d190
0x1415a92a8 4989d6 mov r14, rdx
0x1415a92ab 4885c0 test rax, rax
0x1415a92ae 754e jne 0x1415a92fe
0x1415a92b0 4939fd cmp r13, rdi
0x1415a92b3 7414 je 0x1415a92c9
0x1415a92b5 4c8d6b40 lea r13, [rbx + 0x40]
0x1415a92b9 4c896d50 mov qword ptr [rbp + 0x50], r13
0x1415a92bd 0fb64320 movzx eax, byte ptr [rbx + 0x20]
0x1415a92c1 3c06 cmp al, 6
0x1415a92c3 0f858b000000 jne 0x1415a9354
0x1415a92c9 488d1538196d01 lea rdx, [rip + 0x16d1938]
0x1415a92d0 4c8d05890d6d01 lea r8, [rip + 0x16d0d89]
0x1415a92d7 b901000000 mov ecx, 1
0x1415a92dc e83f0a3501 call 0x1428f9d20
0x1415a92e1 eb18 jmp 0x1415a92fb
0x1415a92e3 4989dd mov r13, rbx
0x1415a92e6 488d151b196d01 lea rdx, [rip + 0x16d191b]
0x1415a92ed 4c8d056c0d6d01 lea r8, [rip + 0x16d0d6c]
0x1415a92f4 31c9 xor ecx, ecx
0x1415a92f6 e8250a3501 call 0x1428f9d20
0x1415a92fb 4989c6 mov r14, rax
0x1415a92fe 4c8936 mov qword ptr [rsi], r14
0x1415a9301 c7460802000000 mov dword ptr [rsi + 8], 2
0x1415a9308 4c29ef sub rdi, r13
0x1415a930b 48c1ef05 shr rdi, 5
0x1415a930f 4c89e9 mov rcx, r13
0x1415a9312 4889fa mov rdx, rdi
0x1415a9315 e856fa2dff call 0x140888d70
0x1415a931a 488b5558 mov rdx, qword ptr [rbp + 0x58]
0x1415a931e 4885d2 test rdx, rdx
0x1415a9321 7413 je 0x1415a9336
0x1415a9323 488b4d48 mov rcx, qword ptr [rbp + 0x48]
0x1415a9327 48c1e205 shl rdx, 5
0x1415a932b 41b808000000 mov r8d, 8
0x1415a9331 e8eaa806ff call 0x140613c20
0x1415a9336 4889f0 mov rax, rsi
0x1415a9339 0f28b5a0000000 movaps xmm6, xmmword ptr [rbp + 0xa0]
0x1415a9340 4881c438010000 add rsp, 0x138
0x1415a9347 5b pop rbx
0x1415a9348 5f pop rdi
0x1415a9349 5e pop rsi
0x1415a934a 415c pop r12
0x1415a934c 415d pop r13
0x1415a934e 415e pop r14
0x1415a9350 415f pop r15
0x1415a9352 5d pop rbp
0x1415a9353 c3 ret 
0x1415a9354 884528 mov byte ptr [rbp + 0x28], al
0x1415a9357 0f104321 movups xmm0, xmmword ptr [rbx + 0x21]
0x1415a935b 0f104b30 movups xmm1, xmmword ptr [rbx + 0x30]
0x1415a935f 0f114529 movups xmmword ptr [rbp + 0x29], xmm0
0x1415a9363 0f114d38 movups xmmword ptr [rbp + 0x38], xmm1
0x1415a9367 488d05ba666e01 lea rax, [rip + 0x16e66ba]
0x1415a936e 4889442420 mov qword ptr [rsp + 0x20], rax
0x1415a9373 48c74424280a000000 mov qword ptr [rsp + 0x28], 0xa
0x1415a937c 4c8d0525666e01 lea r8, [rip + 0x16e6625]
0x1415a9383 488d4d70 lea rcx, [rbp + 0x70]
0x1415a9387 488d5528 lea rdx, [rbp + 0x28]
0x1415a938b 41b90f000000 mov r9d, 0xf
0x1415a9391 e89a87ffff call 0x1415a1b30
0x1415a9396 f6457001 test byte ptr [rbp + 0x70], 1
0x1415a939a 7409 je 0x1415a93a5
0x1415a939c 4c8b7578 mov r14, qword ptr [rbp + 0x78]
0x1415a93a0 e959ffffff jmp 0x1415a92fe
0x1415a93a5 8b5574 mov edx, dword ptr [rbp + 0x74]
0x1415a93a8 4c8b4578 mov r8, qword ptr [rbp + 0x78]
0x1415a93ac 488b8580000000 mov rax, qword ptr [rbp + 0x80]
0x1415a93b3 488945f8 mov qword ptr [rbp - 8], rax
0x1415a93b7 488b8585000000 mov rax, qword ptr [rbp + 0x85]
0x1415a93be 488945fd mov qword ptr [rbp - 3], rax
0x1415a93c2 440fb6a58d000000 movzx r12d, byte ptr [rbp + 0x8d]
0x1415a93ca 4180fc02 cmp r12b, 2
0x1415a93ce 0f84f5feffff je 0x1415a92c9
0x1415a93d4 440fb78d8e000000 movzx r9d, word ptr [rbp + 0x8e]
0x1415a93dc 488b45f8 mov rax, qword ptr [rbp - 8]
0x1415a93e0 488b4dfd mov rcx, qword ptr [rbp - 3]
0x1415a93e4 48894ded mov qword ptr [rbp - 0x13], rcx
0x1415a93e8 488945e8 mov qword ptr [rbp - 0x18], rax
0x1415a93ec 4939fd cmp r13, rdi
0x1415a93ef 7410 je 0x1415a9401
0x1415a93f1 4c8d6b60 lea r13, [rbx + 0x60]
0x1415a93f5 4c896d50 mov qword ptr [rbp + 0x50], r13
0x1415a93f9 0fb64340 movzx eax, byte ptr [rbx + 0x40]
0x1415a93fd 3c06 cmp al, 6
0x1415a93ff 751d jne 0x1415a941e
0x1415a9401 488d1500186d01 lea rdx, [rip + 0x16d1800]
0x1415a9408 4c8d05510c6d01 lea r8, [rip + 0x16d0c51]
0x1415a940f b902000000 mov ecx, 2
0x1415a9414 e807093501 call 0x1428f9d20
0x1415a9419 e9ddfeffff jmp 0x1415a92fb
0x1415a941e 6644894d6e mov word ptr [rbp + 0x6e], r9w
0x1415a9423 4c8945d8 mov qword ptr [rbp - 0x28], r8
0x1415a9427 89551c mov dword ptr [rbp + 0x1c], edx
0x1415a942a 884528 mov byte ptr [rbp + 0x28], al
0x1415a942d 0f104341 movups xmm0, xmmword ptr [rbx + 0x41]
0x1415a9431 0f104b50 movups xmm1, xmmword ptr [rbx + 0x50]
0x1415a9435 0f114529 movups xmmword ptr [rbp + 0x29], xmm0
0x1415a9439 0f114d38 movups xmmword ptr [rbp + 0x38], xmm1
0x1415a943d 488d4d70 lea rcx, [rbp + 0x70]
0x1415a9441 488d5528 lea rdx, [rbp + 0x28]
0x1415a9445 e8d6311400 call 0x1416ec620
0x1415a944a f6457001 test byte ptr [rbp + 0x70], 1
0x1415a944e 0f8548ffffff jne 0x1415a939c
0x1415a9454 8b4d74 mov ecx, dword ptr [rbp + 0x74]
0x1415a9457 488b5578 mov rdx, qword ptr [rbp + 0x78]
0x1415a945b 0f108580000000 movups xmm0, xmmword ptr [rbp + 0x80]
0x1415a9462 0f2945b0 movaps xmmword ptr [rbp - 0x50], xmm0
0x1415a9466 83f902 cmp ecx, 2
0x1415a9469 7496 je 0x1415a9401
0x1415a946b 4939fd cmp r13, rdi
0x1415a946e 7413 je 0x1415a9483
0x1415a9470 4c8dab80000000 lea r13, [rbx + 0x80]
0x1415a9477 4c896d50 mov qword ptr [rbp + 0x50], r13
0x1415a947b 0fb64360 movzx eax, byte ptr [rbx + 0x60]
0x1415a947f 3c06 cmp al, 6
0x1415a9481 751d jne 0x1415a94a0
0x1415a9483 488d157e176d01 lea rdx, [rip + 0x16d177e]
0x1415a948a 4c8d05cf0b6d01 lea r8, [rip + 0x16d0bcf]
0x1415a9491 b903000000 mov ecx, 3
0x1415a9496 e885083501 call 0x1428f9d20
0x1415a949b e95bfeffff jmp 0x1415a92fb
0x1415a94a0 488955e0 mov qword ptr [rbp - 0x20], rdx
0x1415a94a4 894d20 mov dword ptr [rbp + 0x20], ecx
0x1415a94a7 884570 mov byte ptr [rbp + 0x70], al
0x1415a94aa 0f104361 movups xmm0, xmmword ptr [rbx + 0x61]
0x1415a94ae 0f104b70 movups xmm1, xmmword ptr [rbx + 0x70]
0x1415a94b2 0f114571 movups xmmword ptr [rbp + 0x71], xmm0
0x1415a94b6 0f118d80000000 movups xmmword ptr [rbp + 0x80], xmm1
0x1415a94bd 488d4d28 lea rcx, [rbp + 0x28]
0x1415a94c1 488d5570 lea rdx, [rbp + 0x70]
0x1415a94c5 e8a60effff call 0x14159a370
0x1415a94ca 837d2801 cmp dword ptr [rbp + 0x28], 1
0x1415a94ce 7509 jne 0x1415a94d9
0x1415a94d0 4c8b7530 mov r14, qword ptr [rbp + 0x30]
0x1415a94d4 e925feffff jmp 0x1415a92fe
0x1415a94d9 4939fd cmp r13, rdi
0x1415a94dc 741b je 0x1415a94f9
0x1415a94de f30f10752c movss xmm6, dword ptr [rbp + 0x2c]
0x1415a94e3 4c8daba0000000 lea r13, [rbx + 0xa0]
0x1415a94ea 4c896d50 mov qword ptr [rbp + 0x50], r13
0x1415a94ee 0fb68380000000 movzx eax, byte ptr [rbx + 0x80]
0x1415a94f5 3c06 cmp al, 6
0x1415a94f7 751d jne 0x1415a9516
0x1415a94f9 488d1508176d01 lea rdx, [rip + 0x16d1708]
0x1415a9500 4c8d05590b6d01 lea r8, [rip + 0x16d0b59]
0x1415a9507 b904000000 mov ecx, 4
0x1415a950c e80f083501 call 0x1428f9d20
0x1415a9511 e9e5fdffff jmp 0x1415a92fb
0x1415a9516 884570 mov byte ptr [rbp + 0x70], al
0x1415a9519 0f108381000000 movups xmm0, xmmword ptr [rbx + 0x81]
0x1415a9520 0f108b90000000 movups xmm1, xmmword ptr [rbx + 0x90]
0x1415a9527 0f114571 movups xmmword ptr [rbp + 0x71], xmm0
0x1415a952b 0f118d80000000 movups xmmword ptr [rbp + 0x80], xmm1
0x1415a9532 488d4d28 lea rcx, [rbp + 0x28]
0x1415a9536 488d5570 lea rdx, [rbp + 0x70]
0x1415a953a e8613affff call 0x14159cfa0
0x1415a953f 837d2801 cmp dword ptr [rbp + 0x28], 1
0x1415a9543 748b je 0x1415a94d0
0x1415a9545 8b452c mov eax, dword ptr [rbp + 0x2c]
0x1415a9548 894524 mov dword ptr [rbp + 0x24], eax
0x1415a954b 4939fd cmp r13, rdi
0x1415a954e 741a je 0x1415a956a
0x1415a9550 4c8dabc0000000 lea r13, [rbx + 0xc0]
0x1415a9557 4c896d50 mov qword ptr [rbp + 0x50], r13
0x1415a955b 0fb683a0000000 movzx eax, byte ptr [rbx + 0xa0]
0x1415a9562 3c06 cmp al, 6
0x1415a9564 0f8509010000 jne 0x1415a9673
0x1415a956a 31ff xor edi, edi
0x1415a956c 488d4d70 lea rcx, [rbp + 0x70]
0x1415a9570 488d5548 lea rdx, [rbp + 0x48]
0x1415a9574 e887750200 call 0x1415d0b00
0x1415a9579 f6457001 test byte ptr [rbp + 0x70], 1
0x1415a957d 7411 je 0x1415a9590
0x1415a957f 4c8b7578 mov r14, qword ptr [rbp + 0x78]
0x1415a9583 4c8b6d50 mov r13, qword ptr [rbp + 0x50]
0x1415a9587 488b7d60 mov rdi, qword ptr [rbp + 0x60]
0x1415a958b e96efdffff jmp 0x1415a92fe
0x1415a9590 0fb64d74 movzx ecx, byte ptr [rbp + 0x74]
0x1415a9594 0fb74575 movzx eax, word ptr [rbp + 0x75]
0x1415a9598 66898590000000 mov word ptr [rbp + 0x90], ax
0x1415a959f 0fb64577 movzx eax, byte ptr [rbp + 0x77]
0x1415a95a3 888592000000 mov byte ptr [rbp + 0x92], al
0x1415a95a9 b001 mov al, 1
0x1415a95ab 80f903 cmp cl, 3
0x1415a95ae 741d je 0x1415a95cd
0x1415a95b0 0fb68592000000 movzx eax, byte ptr [rbp + 0x92]
0x1415a95b7 888596000000 mov byte ptr [rbp + 0x96], al
0x1415a95bd 0fb78590000000 movzx eax, word ptr [rbp + 0x90]
0x1415a95c4 66898594000000 mov word ptr [rbp + 0x94], ax
0x1415a95cb 89c8 mov eax, ecx
0x1415a95cd 488b4d78 mov rcx, qword ptr [rbp + 0x78]
0x1415a95d1 488b55e8 mov rdx, qword ptr [rbp - 0x18]
0x1415a95d5 4c8b45ed mov r8, qword ptr [rbp - 0x13]
0x1415a95d9 4c89450d mov qword ptr [rbp + 0xd], r8
0x1415a95dd 48895508 mov qword ptr [rbp + 8], rdx
0x1415a95e1 0f2845b0 movaps xmm0, xmmword ptr [rbp - 0x50]
0x1415a95e5 0f2945c0 movaps xmmword ptr [rbp - 0x40], xmm0
0x1415a95e9 488b5560 mov rdx, qword ptr [rbp + 0x60]
0x1415a95ed 483b5550 cmp rdx, qword ptr [rbp + 0x50]
0x1415a95f1 0f85b8000000 jne 0x1415a96af
0x1415a95f7 0f2845c0 movaps xmm0, xmmword ptr [rbp - 0x40]
0x1415a95fb 0f114614 movups xmmword ptr [rsi + 0x14], xmm0
0x1415a95ff 488b5508 mov rdx, qword ptr [rbp + 8]
0x1415a9603 4c8b450d mov r8, qword ptr [rbp + 0xd]
0x1415a9607 48895630 mov qword ptr [rsi + 0x30], rdx
0x1415a960b 4c894635 mov qword ptr [rsi + 0x35], r8
0x1415a960f 884640 mov byte ptr [rsi + 0x40], al
0x1415a9612 0fb78594000000 movzx eax, word ptr [rbp + 0x94]
0x1415a9619 66894641 mov word ptr [rsi + 0x41], ax
0x1415a961d 0fb68596000000 movzx eax, byte ptr [rbp + 0x96]
0x1415a9624 884643 mov byte ptr [rsi + 0x43], al
0x1415a9627 48894e44 mov qword ptr [rsi + 0x44], rcx
0x1415a962b 4c8936 mov qword ptr [rsi], r14
0x1415a962e 8b4520 mov eax, dword ptr [rbp + 0x20]
0x1415a9631 894608 mov dword ptr [rsi + 8], eax
0x1415a9634 488b45e0 mov rax, qword ptr [rbp - 0x20]
0x1415a9638 4889460c mov qword ptr [rsi + 0xc], rax
0x1415a963c 8b451c mov eax, dword ptr [rbp + 0x1c]
0x1415a963f 894624 mov dword ptr [rsi + 0x24], eax
0x1415a9642 488b45d8 mov rax, qword ptr [rbp - 0x28]
0x1415a9646 48894628 mov qword ptr [rsi + 0x28], rax
0x1415a964a 4488663d mov byte ptr [rsi + 0x3d], r12b
0x1415a964e 0fb7456e movzx eax, word ptr [rbp + 0x6e]
0x1415a9652 6689463e mov word ptr [rsi + 0x3e], ax
0x1415a9656 f30f11764c movss dword ptr [rsi + 0x4c], xmm6
0x1415a965b 8b4524 mov eax, dword ptr [rbp + 0x24]
0x1415a965e 894650 mov dword ptr [rsi + 0x50], eax
0x1415a9661 40887e54 mov byte ptr [rsi + 0x54], dil
0x1415a9665 488d4d48 lea rcx, [rbp + 0x48]
0x1415a9669 e8f20e2eff call 0x14088a560
0x1415a966e e9c3fcffff jmp 0x1415a9336
0x1415a9673 884570 mov byte ptr [rbp + 0x70], al
0x1415a9676 0f1083a1000000 movups xmm0, xmmword ptr [rbx + 0xa1]
0x1415a967d 0f108bb0000000 movups xmm1, xmmword ptr [rbx + 0xb0]
0x1415a9684 0f114571 movups xmmword ptr [rbp + 0x71], xmm0
0x1415a9688 0f118d80000000 movups xmmword ptr [rbp + 0x80], xmm1
0x1415a968f 488d4d28 lea rcx, [rbp + 0x28]
0x1415a9693 488d5570 lea rdx, [rbp + 0x70]
0x1415a9697 e8e4ef1300 call 0x1416e8680
0x1415a969c 807d2801 cmp byte ptr [rbp + 0x28], 1
0x1415a96a0 0f842afeffff je 0x1415a94d0
0x1415a96a6 0fb67d29 movzx edi, byte ptr [rbp + 0x29]
0x1415a96aa e9bdfeffff jmp 0x1415a956c
0x1415a96af 488d159a096d01 lea rdx, [rip + 0x16d099a]
0x1415a96b6 4c8d05a3096d01 lea r8, [rip + 0x16d09a3]
0x1415a96bd 4c89f9 mov rcx, r15
0x1415a96c0 e85b063501 call 0x1428f9d20
0x1415a96c5 488906 mov qword ptr [rsi], rax
0x1415a96c8 c7460802000000 mov dword ptr [rsi + 8], 2
0x1415a96cf eb94 jmp 0x1415a9665
0x1415a96d1 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1415a96e0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415a96e5 55 push rbp
0x1415a96e6 4157 push r15
0x1415a96e8 4156 push r14
0x1415a96ea 4155 push r13
0x1415a96ec 4154 push r12
0x1415a96ee 56 push rsi
0x1415a96ef 57 push rdi
0x1415a96f0 53 push rbx
0x1415a96f1 4883ec48 sub rsp, 0x48
0x1415a96f5 488daa80000000 lea rbp, [rdx + 0x80]
0x1415a96fc 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
0x1415a9701 488b5558 mov rdx, qword ptr [rbp + 0x58]
0x1415a9705 4885d2 test rdx, rdx
0x1415a9708 7413 je 0x1415a971d
0x1415a970a 488b4d48 mov rcx, qword ptr [rbp + 0x48]
0x1415a970e 48c1e205 shl rdx, 5
0x1415a9712 41b808000000 mov r8d, 8
0x1415a9718 e803a506ff call 0x140613c20
0x1415a971d 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
0x1415a9722 4883c448 add rsp, 0x48
0x1415a9726 5b pop rbx
0x1415a9727 5f pop rdi
0x1415a9728 5e pop rsi
0x1415a9729 415c pop r12
0x1415a972b 415d pop r13
0x1415a972d 415e pop r14
0x1415a972f 415f pop r15
0x1415a9731 5d pop rbp
0x1415a9732 c3 ret 
0x1415a9733 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1415a9740 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415a9745 55 push rbp
0x1415a9746 4157 push r15
0x1415a9748 4156 push r14
0x1415a974a 4155 push r13
0x1415a974c 4154 push r12
0x1415a974e 56 push rsi
0x1415a974f 57 push rdi
0x1415a9750 53 push rbx
0x1415a9751 4883ec48 sub rsp, 0x48
0x1415a9755 488daa80000000 lea rbp, [rdx + 0x80]
0x1415a975c 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
0x1415a9761 488d4d48 lea rcx, [rbp + 0x48]
0x1415a9765 e8f60d2eff call 0x14088a560
0x1415a976a 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
0x1415a976f 4883c448 add rsp, 0x48
0x1415a9773 5b pop rbx
0x1415a9774 5f pop rdi
0x1415a9775 5e pop rsi
0x1415a9776 415c pop r12
0x1415a9778 415d pop r13
0x1415a977a 415e pop r14
0x1415a977c 415f pop r15
0x1415a977e 5d pop rbp
0x1415a977f c3 ret 
