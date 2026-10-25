0x1416e91c0 55 push rbp
0x1416e91c1 4157 push r15
0x1416e91c3 4156 push r14
0x1416e91c5 4154 push r12
0x1416e91c7 56 push rsi
0x1416e91c8 57 push rdi
0x1416e91c9 53 push rbx
0x1416e91ca 4881ec40010000 sub rsp, 0x140
0x1416e91d1 488dac2480000000 lea rbp, [rsp + 0x80]
0x1416e91d9 48c785b8000000feffffff mov qword ptr [rbp + 0xb8], 0xfffffffffffffffe
0x1416e91e4 4889ce mov rsi, rcx
0x1416e91e7 0fb602 movzx eax, byte ptr [rdx]
0x1416e91ea 83f803 cmp eax, 3
0x1416e91ed 8985a0000000 mov dword ptr [rbp + 0xa0], eax
0x1416e91f3 48895578 mov qword ptr [rbp + 0x78], rdx
0x1416e91f7 0f84f0000000 je 0x1416e92ed
0x1416e91fd 83f805 cmp eax, 5
0x1416e9200 0f85a0010000 jne 0x1416e93a6
0x1416e9206 4c8b7a08 mov r15, qword ptr [rdx + 8]
0x1416e920a 4c8b6210 mov r12, qword ptr [rdx + 0x10]
0x1416e920e 488b7a18 mov rdi, qword ptr [rdx + 0x18]
0x1416e9212 31c0 xor eax, eax
0x1416e9214 4d85ff test r15, r15
0x1416e9217 0f95c0 setne al
0x1416e921a 4889f9 mov rcx, rdi
0x1416e921d 490f44cf cmove rcx, r15
0x1416e9221 48894520 mov qword ptr [rbp + 0x20], rax
0x1416e9225 48c7452800000000 mov qword ptr [rbp + 0x28], 0
0x1416e922d 4c897d30 mov qword ptr [rbp + 0x30], r15
0x1416e9231 4c896538 mov qword ptr [rbp + 0x38], r12
0x1416e9235 48894540 mov qword ptr [rbp + 0x40], rax
0x1416e9239 48c7454800000000 mov qword ptr [rbp + 0x48], 0
0x1416e9241 4c897d50 mov qword ptr [rbp + 0x50], r15
0x1416e9245 4c896558 mov qword ptr [rbp + 0x58], r12
0x1416e9249 48894d60 mov qword ptr [rbp + 0x60], rcx
0x1416e924d 488d8d80000000 lea rcx, [rbp + 0x80]
0x1416e9254 488d5520 lea rdx, [rbp + 0x20]
0x1416e9258 e8f331d5ff call 0x14143c450
0x1416e925d 488b8580000000 mov rax, qword ptr [rbp + 0x80]
0x1416e9264 4885c0 test rax, rax
0x1416e9267 7444 je 0x1416e92ad
0x1416e9269 488b8d90000000 mov rcx, qword ptr [rbp + 0x90]
0x1416e9270 488d1449 lea rdx, [rcx + rcx*2]
0x1416e9274 4c8b84d068010000 mov r8, qword ptr [rax + rdx*8 + 0x168]
0x1416e927c 4c8b8cd070010000 mov r9, qword ptr [rax + rdx*8 + 0x170]
0x1416e9284 488b9cd078010000 mov rbx, qword ptr [rax + rdx*8 + 0x178]
0x1416e928c 48c1e105 shl rcx, 5
0x1416e9290 0f100408 movups xmm0, xmmword ptr [rax + rcx]
0x1416e9294 0f104c0810 movups xmm1, xmmword ptr [rax + rcx + 0x10]
0x1416e9299 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
0x1416e929d 0f294df0 movaps xmmword ptr [rbp - 0x10], xmm1
0x1416e92a1 4c89c0 mov rax, r8
0x1416e92a4 48f7d8 neg rax
0x1416e92a7 0f81dc010000 jno 0x1416e9489
0x1416e92ad c685800000000b mov byte ptr [rbp + 0x80], 0xb
0x1416e92b4 488d152d5f5a01 lea rdx, [rip + 0x15a5f2d]
0x1416e92bb 4c8d05365f5a01 lea r8, [rip + 0x15a5f36]
0x1416e92c2 488d8d80000000 lea rcx, [rbp + 0x80]
0x1416e92c9 e872082101 call 0x1428f9b40
0x1416e92ce 48894608 mov qword ptr [rsi + 8], rax
0x1416e92d2 c70601000000 mov dword ptr [rsi], 1
0x1416e92d8 c685b700000001 mov byte ptr [rbp + 0xb7], 1
0x1416e92df 488d4d20 lea rcx, [rbp + 0x20]
0x1416e92e3 e808eb19ff call 0x140887df0
0x1416e92e8 e9ce020000 jmp 0x1416e95bb
0x1416e92ed 4c8b7a08 mov r15, qword ptr [rdx + 8]
0x1416e92f1 4c8b6210 mov r12, qword ptr [rdx + 0x10]
0x1416e92f5 488b7a18 mov rdi, qword ptr [rdx + 0x18]
0x1416e92f9 41b606 mov r14b, 6
0x1416e92fc 4c89f8 mov rax, r15
0x1416e92ff 4c89e1 mov rcx, r12
0x1416e9302 4889fb mov rbx, rdi
0x1416e9305 48894520 mov qword ptr [rbp + 0x20], rax
0x1416e9309 48894d28 mov qword ptr [rbp + 0x28], rcx
0x1416e930d 48895d30 mov qword ptr [rbp + 0x30], rbx
0x1416e9311 0f2845c0 movaps xmm0, xmmword ptr [rbp - 0x40]
0x1416e9315 0f114539 movups xmmword ptr [rbp + 0x39], xmm0
0x1416e9319 0f1045cf movups xmm0, xmmword ptr [rbp - 0x31]
0x1416e931d 0f114548 movups xmmword ptr [rbp + 0x48], xmm0
0x1416e9321 44887538 mov byte ptr [rbp + 0x38], r14b
0x1416e9325 c685b700000001 mov byte ptr [rbp + 0xb7], 1
0x1416e932c 488d4de0 lea rcx, [rbp - 0x20]
0x1416e9330 488d5520 lea rdx, [rbp + 0x20]
0x1416e9334 e8779beeff call 0x1415d2eb0
0x1416e9339 0fb64de0 movzx ecx, byte ptr [rbp - 0x20]
0x1416e933d 488b45e8 mov rax, qword ptr [rbp - 0x18]
0x1416e9341 83f903 cmp ecx, 3
0x1416e9344 0f85ec000000 jne 0x1416e9436
0x1416e934a 48894608 mov qword ptr [rsi + 8], rax
0x1416e934e b801000000 mov eax, 1
0x1416e9353 8906 mov dword ptr [rsi], eax
0x1416e9355 8b85a0000000 mov eax, dword ptr [rbp + 0xa0]
0x1416e935b 83f803 cmp eax, 3
0x1416e935e 0f8457020000 je 0x1416e95bb
0x1416e9364 83f805 cmp eax, 5
0x1416e9367 0f844e020000 je 0x1416e95bb
0x1416e936d 4c89bda0000000 mov qword ptr [rbp + 0xa0], r15
0x1416e9374 4c896578 mov qword ptr [rbp + 0x78], r12
0x1416e9378 4c89e1 mov rcx, r12
0x1416e937b 4889fa mov rdx, rdi
0x1416e937e e8edf919ff call 0x140888d70
0x1416e9383 488b95a0000000 mov rdx, qword ptr [rbp + 0xa0]
0x1416e938a 4885d2 test rdx, rdx
0x1416e938d 488b4d78 mov rcx, qword ptr [rbp + 0x78]
0x1416e9391 0f8424020000 je 0x1416e95bb
0x1416e9397 48c1e205 shl rdx, 5
0x1416e939b 41b808000000 mov r8d, 8
0x1416e93a1 e9fe010000 jmp 0x1416e95a4
0x1416e93a6 0f1002 movups xmm0, xmmword ptr [rdx]
0x1416e93a9 0f104a10 movups xmm1, xmmword ptr [rdx + 0x10]
0x1416e93ad 0f294d30 movaps xmmword ptr [rbp + 0x30], xmm1
0x1416e93b1 0f294520 movaps xmmword ptr [rbp + 0x20], xmm0
0x1416e93b5 488d4de0 lea rcx, [rbp - 0x20]
0x1416e93b9 488d5520 lea rdx, [rbp + 0x20]
0x1416e93bd e84e081e01 call 0x1428c9c10
0x1416e93c2 488d155f5e5a01 lea rdx, [rip + 0x15a5e5f]
0x1416e93c9 4c8d05285e5a01 lea r8, [rip + 0x15a5e28]
0x1416e93d0 488d4de0 lea rcx, [rbp - 0x20]
0x1416e93d4 e827061e01 call 0x1428c9a00
0x1416e93d9 48894608 mov qword ptr [rsi + 8], rax
0x1416e93dd c70601000000 mov dword ptr [rsi], 1
0x1416e93e3 0fb64520 movzx eax, byte ptr [rbp + 0x20]
0x1416e93e7 83f803 cmp eax, 3
0x1416e93ea 0f82cb010000 jb 0x1416e95bb
0x1416e93f0 0f849b010000 je 0x1416e9591
0x1416e93f6 83f804 cmp eax, 4
0x1416e93f9 0f85ac010000 jne 0x1416e95ab
0x1416e93ff 488b4d30 mov rcx, qword ptr [rbp + 0x30]
0x1416e9403 488b5538 mov rdx, qword ptr [rbp + 0x38]
0x1416e9407 48898da8000000 mov qword ptr [rbp + 0xa8], rcx
0x1416e940e e85df919ff call 0x140888d70
0x1416e9413 488b5528 mov rdx, qword ptr [rbp + 0x28]
0x1416e9417 4885d2 test rdx, rdx
0x1416e941a 0f849b010000 je 0x1416e95bb
0x1416e9420 48c1e205 shl rdx, 5
0x1416e9424 41b808000000 mov r8d, 8
0x1416e942a 488b8da8000000 mov rcx, qword ptr [rbp + 0xa8]
0x1416e9431 e96e010000 jmp 0x1416e95a4
0x1416e9436 488b5500 mov rdx, qword ptr [rbp]
0x1416e943a 48899598000000 mov qword ptr [rbp + 0x98], rdx
0x1416e9441 0f1045f0 movups xmm0, xmmword ptr [rbp - 0x10]
0x1416e9445 0f118588000000 movups xmmword ptr [rbp + 0x88], xmm0
0x1416e944c 48898580000000 mov qword ptr [rbp + 0x80], rax
0x1416e9453 85c9 test ecx, ecx
0x1416e9455 0f84e7000000 je 0x1416e9542
0x1416e945b 83f901 cmp ecx, 1
0x1416e945e 0f8505010000 jne 0x1416e9569
0x1416e9464 c685b700000001 mov byte ptr [rbp + 0xb7], 1
0x1416e946b 488d8d80000000 lea rcx, [rbp + 0x80]
0x1416e9472 e889eb49ff call 0x140b88000
0x1416e9477 4885c0 test rax, rax
0x1416e947a 0f85cafeffff jne 0x1416e934a
0x1416e9480 c6460401 mov byte ptr [rsi + 4], 1
0x1416e9484 e9d9000000 jmp 0x1416e9562
0x1416e9489 4c894d70 mov qword ptr [rbp + 0x70], r9
0x1416e948d 4c8985a8000000 mov qword ptr [rbp + 0xa8], r8
0x1416e9494 0f2845e0 movaps xmm0, xmmword ptr [rbp - 0x20]
0x1416e9498 0f284df0 movaps xmm1, xmmword ptr [rbp - 0x10]
0x1416e949c 0f298d90000000 movaps xmmword ptr [rbp + 0x90], xmm1
0x1416e94a3 0f298580000000 movaps xmmword ptr [rbp + 0x80], xmm0
0x1416e94aa 488d4da8 lea rcx, [rbp - 0x58]
0x1416e94ae 488d5520 lea rdx, [rbp + 0x20]
0x1416e94b2 e8992fd5ff call 0x14143c450
0x1416e94b7 488b45a8 mov rax, qword ptr [rbp - 0x58]
0x1416e94bb 4885c0 test rax, rax
0x1416e94be 7445 je 0x1416e9505
0x1416e94c0 488b4db8 mov rcx, qword ptr [rbp - 0x48]
0x1416e94c4 488d1449 lea rdx, [rcx + rcx*2]
0x1416e94c8 4c8b84d078010000 mov r8, qword ptr [rax + rdx*8 + 0x178]
0x1416e94d0 4c8945f0 mov qword ptr [rbp - 0x10], r8
0x1416e94d4 0f1084d068010000 movups xmm0, xmmword ptr [rax + rdx*8 + 0x168]
0x1416e94dc 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
0x1416e94e0 48c1e105 shl rcx, 5
0x1416e94e4 0f100408 movups xmm0, xmmword ptr [rax + rcx]
0x1416e94e8 0f104c0810 movups xmm1, xmmword ptr [rax + rcx + 0x10]
0x1416e94ed 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
0x1416e94f1 0f114d08 movups xmmword ptr [rbp + 8], xmm1
0x1416e94f5 488b55e0 mov rdx, qword ptr [rbp - 0x20]
0x1416e94f9 4889d0 mov rax, rdx
0x1416e94fc 48f7d8 neg rax
0x1416e94ff 0f81cb000000 jno 0x1416e95d0
0x1416e9505 448ab580000000 mov r14b, byte ptr [rbp + 0x80]
0x1416e950c 0f108581000000 movups xmm0, xmmword ptr [rbp + 0x81]
0x1416e9513 0f2945c0 movaps xmmword ptr [rbp - 0x40], xmm0
0x1416e9517 0f108590000000 movups xmm0, xmmword ptr [rbp + 0x90]
0x1416e951e 0f1145cf movups xmmword ptr [rbp - 0x31], xmm0
0x1416e9522 c685b700000001 mov byte ptr [rbp + 0xb7], 1
0x1416e9529 488d4d20 lea rcx, [rbp + 0x20]
0x1416e952d e8bee819ff call 0x140887df0
0x1416e9532 488b85a8000000 mov rax, qword ptr [rbp + 0xa8]
0x1416e9539 488b4d70 mov rcx, qword ptr [rbp + 0x70]
0x1416e953d e9c3fdffff jmp 0x1416e9305
0x1416e9542 c685b700000001 mov byte ptr [rbp + 0xb7], 1
0x1416e9549 488d8d80000000 lea rcx, [rbp + 0x80]
0x1416e9550 e8abea49ff call 0x140b88000
0x1416e9555 4885c0 test rax, rax
0x1416e9558 0f85ecfdffff jne 0x1416e934a
0x1416e955e c6460400 mov byte ptr [rsi + 4], 0
0x1416e9562 31c0 xor eax, eax
0x1416e9564 e9eafdffff jmp 0x1416e9353
0x1416e9569 c685b700000001 mov byte ptr [rbp + 0xb7], 1
0x1416e9570 4c8d0501635a01 lea r8, [rip + 0x15a6301]
0x1416e9577 488d9580000000 lea rdx, [rbp + 0x80]
0x1416e957e 41b903000000 mov r9d, 3
0x1416e9584 4889f1 mov rcx, rsi
0x1416e9587 e8b4feeeff call 0x1415d9440
0x1416e958c e9c4fdffff jmp 0x1416e9355
0x1416e9591 488b5528 mov rdx, qword ptr [rbp + 0x28]
0x1416e9595 4885d2 test rdx, rdx
0x1416e9598 7421 je 0x1416e95bb
0x1416e959a 488b4d30 mov rcx, qword ptr [rbp + 0x30]
0x1416e959e 41b801000000 mov r8d, 1
0x1416e95a4 e877a6f2fe call 0x140613c20
0x1416e95a9 eb10 jmp 0x1416e95bb
0x1416e95ab 488d4d28 lea rcx, [rbp + 0x28]
0x1416e95af c685b700000000 mov byte ptr [rbp + 0xb7], 0
0x1416e95b6 e885f91aff call 0x140898f40
0x1416e95bb 4889f0 mov rax, rsi
0x1416e95be 4881c440010000 add rsp, 0x140
0x1416e95c5 5b pop rbx
0x1416e95c6 5f pop rdi
0x1416e95c7 5e pop rsi
0x1416e95c8 415c pop r12
0x1416e95ca 415e pop r14
0x1416e95cc 415f pop r15
0x1416e95ce 5d pop rbp
0x1416e95cf c3 ret 
0x1416e95d0 730f jae 0x1416e95e1
0x1416e95d2 488b4de8 mov rcx, qword ptr [rbp - 0x18]
0x1416e95d6 41b801000000 mov r8d, 1
0x1416e95dc e83fa6f2fe call 0x140613c20
0x1416e95e1 0fb645f8 movzx eax, byte ptr [rbp - 8]
0x1416e95e5 83f803 cmp eax, 3
0x1416e95e8 7231 jb 0x1416e961b
0x1416e95ea 7410 je 0x1416e95fc
0x1416e95ec 488d4d00 lea rcx, [rbp]
0x1416e95f0 83f804 cmp eax, 4
0x1416e95f3 7521 jne 0x1416e9616
0x1416e95f5 e896fd19ff call 0x140889390
0x1416e95fa eb1f jmp 0x1416e961b
0x1416e95fc 488b5500 mov rdx, qword ptr [rbp]
0x1416e9600 4885d2 test rdx, rdx
0x1416e9603 7416 je 0x1416e961b
0x1416e9605 488b4d08 mov rcx, qword ptr [rbp + 8]
0x1416e9609 41b801000000 mov r8d, 1
0x1416e960f e80ca6f2fe call 0x140613c20
0x1416e9614 eb05 jmp 0x1416e961b
0x1416e9616 e825f91aff call 0x140898f40
0x1416e961b c645e00b mov byte ptr [rbp - 0x20], 0xb
0x1416e961f 488d15c25b5a01 lea rdx, [rip + 0x15a5bc2]
0x1416e9626 4c8d05cb5b5a01 lea r8, [rip + 0x15a5bcb]
0x1416e962d 488d4de0 lea rcx, [rbp - 0x20]
0x1416e9631 e80a052101 call 0x1428f9b40
0x1416e9636 48894608 mov qword ptr [rsi + 8], rax
0x1416e963a c70601000000 mov dword ptr [rsi], 1
0x1416e9640 0fb68580000000 movzx eax, byte ptr [rbp + 0x80]
0x1416e9647 83f803 cmp eax, 3
0x1416e964a 723a jb 0x1416e9686
0x1416e964c 7413 je 0x1416e9661
0x1416e964e 488d8d88000000 lea rcx, [rbp + 0x88]
0x1416e9655 83f804 cmp eax, 4
0x1416e9658 7527 jne 0x1416e9681
0x1416e965a e831fd19ff call 0x140889390
0x1416e965f eb25 jmp 0x1416e9686
0x1416e9661 488b9588000000 mov rdx, qword ptr [rbp + 0x88]
0x1416e9668 4885d2 test rdx, rdx
0x1416e966b 7419 je 0x1416e9686
0x1416e966d 488b8d90000000 mov rcx, qword ptr [rbp + 0x90]
0x1416e9674 41b801000000 mov r8d, 1
0x1416e967a e8a1a5f2fe call 0x140613c20
0x1416e967f eb05 jmp 0x1416e9686
0x1416e9681 e8baf81aff call 0x140898f40
0x1416e9686 488b95a8000000 mov rdx, qword ptr [rbp + 0xa8]
0x1416e968d 4885d2 test rdx, rdx
0x1416e9690 0f8442fcffff je 0x1416e92d8
0x1416e9696 41b801000000 mov r8d, 1
0x1416e969c 488b4d70 mov rcx, qword ptr [rbp + 0x70]
0x1416e96a0 e87ba5f2fe call 0x140613c20
0x1416e96a5 e92efcffff jmp 0x1416e92d8
0x1416e96aa 660f1f440000 nop word ptr [rax + rax]
0x1416e96b0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416e96b5 55 push rbp
0x1416e96b6 4157 push r15
0x1416e96b8 4156 push r14
0x1416e96ba 4154 push r12
0x1416e96bc 56 push rsi
0x1416e96bd 57 push rdi
0x1416e96be 53 push rbx
0x1416e96bf 4883ec20 sub rsp, 0x20
0x1416e96c3 488daa80000000 lea rbp, [rdx + 0x80]
0x1416e96ca 488d4d20 lea rcx, [rbp + 0x20]
0x1416e96ce e81de719ff call 0x140887df0
0x1416e96d3 c685b700000001 mov byte ptr [rbp + 0xb7], 1
0x1416e96da 4883c420 add rsp, 0x20
0x1416e96de 5b pop rbx
0x1416e96df 5f pop rdi
0x1416e96e0 5e pop rsi
0x1416e96e1 415c pop r12
0x1416e96e3 415e pop r14
0x1416e96e5 415f pop r15
0x1416e96e7 5d pop rbp
0x1416e96e8 c3 ret 
0x1416e96e9 0f1f8000000000 nop dword ptr [rax]
0x1416e96f0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416e96f5 55 push rbp
0x1416e96f6 4157 push r15
0x1416e96f8 4156 push r14
0x1416e96fa 4154 push r12
0x1416e96fc 56 push rsi
0x1416e96fd 57 push rdi
0x1416e96fe 53 push rbx
0x1416e96ff 4883ec20 sub rsp, 0x20
0x1416e9703 488daa80000000 lea rbp, [rdx + 0x80]
0x1416e970a 4883bda800000000 cmp qword ptr [rbp + 0xa8], 0
0x1416e9712 7416 je 0x1416e972a
0x1416e9714 41b801000000 mov r8d, 1
0x1416e971a 488b4d70 mov rcx, qword ptr [rbp + 0x70]
0x1416e971e 488b95a8000000 mov rdx, qword ptr [rbp + 0xa8]
0x1416e9725 e8f6a4f2fe call 0x140613c20
0x1416e972a 90 nop 
0x1416e972b 4883c420 add rsp, 0x20
0x1416e972f 5b pop rbx
0x1416e9730 5f pop rdi
0x1416e9731 5e pop rsi
0x1416e9732 415c pop r12
0x1416e9734 415e pop r14
0x1416e9736 415f pop r15
0x1416e9738 5d pop rbp
0x1416e9739 c3 ret 
0x1416e973a 660f1f440000 nop word ptr [rax + rax]
0x1416e9740 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416e9745 55 push rbp
0x1416e9746 4157 push r15
0x1416e9748 4156 push r14
0x1416e974a 4154 push r12
0x1416e974c 56 push rsi
0x1416e974d 57 push rdi
0x1416e974e 53 push rbx
0x1416e974f 4883ec20 sub rsp, 0x20
0x1416e9753 488daa80000000 lea rbp, [rdx + 0x80]
0x1416e975a 4883bda000000000 cmp qword ptr [rbp + 0xa0], 0
0x1416e9762 741a je 0x1416e977e
0x1416e9764 488b95a0000000 mov rdx, qword ptr [rbp + 0xa0]
0x1416e976b 48c1e205 shl rdx, 5
0x1416e976f 41b808000000 mov r8d, 8
0x1416e9775 488b4d78 mov rcx, qword ptr [rbp + 0x78]
0x1416e9779 e8a2a4f2fe call 0x140613c20
0x1416e977e 90 nop 
0x1416e977f 4883c420 add rsp, 0x20
0x1416e9783 5b pop rbx
0x1416e9784 5f pop rdi
0x1416e9785 5e pop rsi
0x1416e9786 415c pop r12
0x1416e9788 415e pop r14
0x1416e978a 415f pop r15
0x1416e978c 5d pop rbp
0x1416e978d c3 ret 
0x1416e978e 6690 nop 
0x1416e9790 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416e9795 55 push rbp
0x1416e9796 4157 push r15
0x1416e9798 4156 push r14
0x1416e979a 4154 push r12
0x1416e979c 56 push rsi
0x1416e979d 57 push rdi
0x1416e979e 53 push rbx
0x1416e979f 4883ec20 sub rsp, 0x20
0x1416e97a3 488daa80000000 lea rbp, [rdx + 0x80]
0x1416e97aa 488d4d20 lea rcx, [rbp + 0x20]
0x1416e97ae e8dd4217ff call 0x14085da90
0x1416e97b3 c685b700000000 mov byte ptr [rbp + 0xb7], 0
0x1416e97ba 4883c420 add rsp, 0x20
0x1416e97be 5b pop rbx
0x1416e97bf 5f pop rdi
0x1416e97c0 5e pop rsi
0x1416e97c1 415c pop r12
0x1416e97c3 415e pop r14
0x1416e97c5 415f pop r15
0x1416e97c7 5d pop rbp
0x1416e97c8 c3 ret 
0x1416e97c9 0f1f8000000000 nop dword ptr [rax]
0x1416e97d0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416e97d5 55 push rbp
0x1416e97d6 4157 push r15
0x1416e97d8 4156 push r14
0x1416e97da 4154 push r12
0x1416e97dc 56 push rsi
0x1416e97dd 57 push rdi
0x1416e97de 53 push rbx
0x1416e97df 4883ec20 sub rsp, 0x20
0x1416e97e3 488daa80000000 lea rbp, [rdx + 0x80]
0x1416e97ea 488b5528 mov rdx, qword ptr [rbp + 0x28]
0x1416e97ee 4885d2 test rdx, rdx
0x1416e97f1 7416 je 0x1416e9809
0x1416e97f3 48c1e205 shl rdx, 5
0x1416e97f7 41b808000000 mov r8d, 8
0x1416e97fd 488b8da8000000 mov rcx, qword ptr [rbp + 0xa8]
0x1416e9804 e817a4f2fe call 0x140613c20
0x1416e9809 c685b700000000 mov byte ptr [rbp + 0xb7], 0
0x1416e9810 4883c420 add rsp, 0x20
0x1416e9814 5b pop rbx
0x1416e9815 5f pop rdi
0x1416e9816 5e pop rsi
0x1416e9817 415c pop r12
0x1416e9819 415e pop r14
0x1416e981b 415f pop r15
0x1416e981d 5d pop rbp
0x1416e981e c3 ret 
0x1416e981f 90 nop 
0x1416e9820 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416e9825 55 push rbp
0x1416e9826 4157 push r15
0x1416e9828 4156 push r14
0x1416e982a 4154 push r12
0x1416e982c 56 push rsi
0x1416e982d 57 push rdi
0x1416e982e 53 push rbx
0x1416e982f 4883ec20 sub rsp, 0x20
0x1416e9833 488daa80000000 lea rbp, [rdx + 0x80]
0x1416e983a 80bdb700000000 cmp byte ptr [rbp + 0xb7], 0
0x1416e9841 7415 je 0x1416e9858
0x1416e9843 80bda000000003 cmp byte ptr [rbp + 0xa0], 3
0x1416e984a 740c je 0x1416e9858
0x1416e984c 0fb685a0000000 movzx eax, byte ptr [rbp + 0xa0]
0x1416e9853 83f805 cmp eax, 5
0x1416e9856 750f jne 0x1416e9867
0x1416e9858 4883c420 add rsp, 0x20
0x1416e985c 5b pop rbx
0x1416e985d 5f pop rdi
0x1416e985e 5e pop rsi
0x1416e985f 415c pop r12
0x1416e9861 415e pop r14
0x1416e9863 415f pop r15
0x1416e9865 5d pop rbp
0x1416e9866 c3 ret 
0x1416e9867 488b4d78 mov rcx, qword ptr [rbp + 0x78]
0x1416e986b e8204217ff call 0x14085da90
0x1416e9870 ebe6 jmp 0x1416e9858
0x1416e9872 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1416e9880 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416e9885 55 push rbp
0x1416e9886 4157 push r15
0x1416e9888 4156 push r14
0x1416e988a 4154 push r12
0x1416e988c 56 push rsi
0x1416e988d 57 push rdi
0x1416e988e 53 push rbx
0x1416e988f 4883ec20 sub rsp, 0x20
0x1416e9893 488daa80000000 lea rbp, [rdx + 0x80]
0x1416e989a 488d8d80000000 lea rcx, [rbp + 0x80]
0x1416e98a1 e8ea4117ff call 0x14085da90
0x1416e98a6 90 nop 
0x1416e98a7 4883c420 add rsp, 0x20
0x1416e98ab 5b pop rbx
0x1416e98ac 5f pop rdi
0x1416e98ad 5e pop rsi
0x1416e98ae 415c pop r12
0x1416e98b0 415e pop r14
0x1416e98b2 415f pop r15
0x1416e98b4 5d pop rbp
0x1416e98b5 c3 ret 
