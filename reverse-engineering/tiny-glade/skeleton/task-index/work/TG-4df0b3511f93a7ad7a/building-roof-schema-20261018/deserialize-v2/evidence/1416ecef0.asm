0x1416ecef0 55 push rbp
0x1416ecef1 56 push rsi
0x1416ecef2 4883ec48 sub rsp, 0x48
0x1416ecef6 488d6c2440 lea rbp, [rsp + 0x40]
0x1416ecefb 48c74500feffffff mov qword ptr [rbp], 0xfffffffffffffffe
0x1416ecf03 4889ce mov rsi, rcx
0x1416ecf06 4983f809 cmp r8, 9
0x1416ecf0a 7433 je 0x1416ecf3f
0x1416ecf0c 4983f806 cmp r8, 6
0x1416ecf10 7569 jne 0x1416ecf7b
0x1416ecf12 803a43 cmp byte ptr [rdx], 0x43
0x1416ecf15 7564 jne 0x1416ecf7b
0x1416ecf17 807a0169 cmp byte ptr [rdx + 1], 0x69
0x1416ecf1b 755e jne 0x1416ecf7b
0x1416ecf1d 807a0272 cmp byte ptr [rdx + 2], 0x72
0x1416ecf21 7558 jne 0x1416ecf7b
0x1416ecf23 807a0363 cmp byte ptr [rdx + 3], 0x63
0x1416ecf27 7552 jne 0x1416ecf7b
0x1416ecf29 807a046c cmp byte ptr [rdx + 4], 0x6c
0x1416ecf2d 754c jne 0x1416ecf7b
0x1416ecf2f 807a0565 cmp byte ptr [rdx + 5], 0x65
0x1416ecf33 7546 jne 0x1416ecf7b
0x1416ecf35 66c7060000 mov word ptr [rsi], 0
0x1416ecf3a e98a000000 jmp 0x1416ecfc9
0x1416ecf3f 803a52 cmp byte ptr [rdx], 0x52
0x1416ecf42 7537 jne 0x1416ecf7b
0x1416ecf44 807a0165 cmp byte ptr [rdx + 1], 0x65
0x1416ecf48 7531 jne 0x1416ecf7b
0x1416ecf4a 807a0263 cmp byte ptr [rdx + 2], 0x63
0x1416ecf4e 752b jne 0x1416ecf7b
0x1416ecf50 807a0374 cmp byte ptr [rdx + 3], 0x74
0x1416ecf54 7525 jne 0x1416ecf7b
0x1416ecf56 807a0461 cmp byte ptr [rdx + 4], 0x61
0x1416ecf5a 751f jne 0x1416ecf7b
0x1416ecf5c 807a056e cmp byte ptr [rdx + 5], 0x6e
0x1416ecf60 7519 jne 0x1416ecf7b
0x1416ecf62 807a0667 cmp byte ptr [rdx + 6], 0x67
0x1416ecf66 7513 jne 0x1416ecf7b
0x1416ecf68 807a076c cmp byte ptr [rdx + 7], 0x6c
0x1416ecf6c 750d jne 0x1416ecf7b
0x1416ecf6e 807a0865 cmp byte ptr [rdx + 8], 0x65
0x1416ecf72 7507 jne 0x1416ecf7b
0x1416ecf74 66c7060001 mov word ptr [rsi], 0x100
0x1416ecf79 eb4e jmp 0x1416ecfc9
0x1416ecf7b 488d4de0 lea rcx, [rbp - 0x20]
0x1416ecf7f e86cf665ff call 0x140d4c5f0
0x1416ecf84 488b4de8 mov rcx, qword ptr [rbp - 0x18]
0x1416ecf88 488b55f0 mov rdx, qword ptr [rbp - 0x10]
0x1416ecf8c 4c8d05052d5a01 lea r8, [rip + 0x15a2d05]
0x1416ecf93 41b902000000 mov r9d, 2
0x1416ecf99 48894df8 mov qword ptr [rbp - 8], rcx
0x1416ecf9d e87ece2001 call 0x1428f9e20
0x1416ecfa2 48894608 mov qword ptr [rsi + 8], rax
0x1416ecfa6 c60601 mov byte ptr [rsi], 1
0x1416ecfa9 488b55e0 mov rdx, qword ptr [rbp - 0x20]
0x1416ecfad 488d045500000000 lea rax, [rdx*2]
0x1416ecfb5 4885c0 test rax, rax
0x1416ecfb8 740f je 0x1416ecfc9
0x1416ecfba 41b801000000 mov r8d, 1
0x1416ecfc0 488b4df8 mov rcx, qword ptr [rbp - 8]
0x1416ecfc4 e8576cf2fe call 0x140613c20
0x1416ecfc9 4889f0 mov rax, rsi
0x1416ecfcc 4883c448 add rsp, 0x48
0x1416ecfd0 5e pop rsi
0x1416ecfd1 5d pop rbp
0x1416ecfd2 c3 ret 
0x1416ecfd3 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1416ecfe0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1416ecfe5 55 push rbp
0x1416ecfe6 56 push rsi
0x1416ecfe7 4883ec28 sub rsp, 0x28
0x1416ecfeb 488d6a40 lea rbp, [rdx + 0x40]
0x1416ecfef 488b55e0 mov rdx, qword ptr [rbp - 0x20]
0x1416ecff3 488d045500000000 lea rax, [rdx*2]
0x1416ecffb 4885c0 test rax, rax
0x1416ecffe 740f je 0x1416ed00f
0x1416ed000 41b801000000 mov r8d, 1
0x1416ed006 488b4df8 mov rcx, qword ptr [rbp - 8]
0x1416ed00a e8116cf2fe call 0x140613c20
0x1416ed00f 90 nop 
0x1416ed010 4883c428 add rsp, 0x28
0x1416ed014 5e pop rsi
0x1416ed015 5d pop rbp
0x1416ed016 c3 ret 
