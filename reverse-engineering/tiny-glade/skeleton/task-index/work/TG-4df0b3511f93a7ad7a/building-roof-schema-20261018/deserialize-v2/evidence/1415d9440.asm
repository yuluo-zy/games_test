0x1415d9440 55 push rbp
0x1415d9441 4157 push r15
0x1415d9443 4156 push r14
0x1415d9445 4155 push r13
0x1415d9447 4154 push r12
0x1415d9449 56 push rsi
0x1415d944a 57 push rdi
0x1415d944b 53 push rbx
0x1415d944c 4881ec48010000 sub rsp, 0x148
0x1415d9453 488dac2480000000 lea rbp, [rsp + 0x80]
0x1415d945b 48c785c0000000feffffff mov qword ptr [rbp + 0xc0], 0xfffffffffffffffe
0x1415d9466 48898db0000000 mov qword ptr [rbp + 0xb0], rcx
0x1415d946d 0fb602 movzx eax, byte ptr [rdx]
0x1415d9470 83f806 cmp eax, 6
0x1415d9473 488955c0 mov qword ptr [rbp - 0x40], rdx
0x1415d9477 488945b8 mov qword ptr [rbp - 0x48], rax
0x1415d947b 753b jne 0x1415d94b8
0x1415d947d c645e00d mov byte ptr [rbp - 0x20], 0xd
0x1415d9481 c685bf00000000 mov byte ptr [rbp + 0xbf], 0
0x1415d9488 488d15f90d6a01 lea rdx, [rip + 0x16a0df9]
0x1415d948f 4c8d05ca0b6a01 lea r8, [rip + 0x16a0bca]
0x1415d9496 488d4de0 lea rcx, [rbp - 0x20]
0x1415d949a e861052f01 call 0x1428c9a00
0x1415d949f 488b8db0000000 mov rcx, qword ptr [rbp + 0xb0]
0x1415d94a6 48894108 mov qword ptr [rcx + 8], rax
0x1415d94aa 4889c8 mov rax, rcx
0x1415d94ad c70101000000 mov dword ptr [rcx], 1
0x1415d94b3 e9ff030000 jmp 0x1415d98b7
0x1415d94b8 83f805 cmp eax, 5
0x1415d94bb 0f85b2040000 jne 0x1415d9973
0x1415d94c1 0f104208 movups xmm0, xmmword ptr [rdx + 8]
0x1415d94c5 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
0x1415d94c9 488b4218 mov rax, qword ptr [rdx + 0x18]
0x1415d94cd 488945c8 mov qword ptr [rbp - 0x38], rax
0x1415d94d1 488945b0 mov qword ptr [rbp - 0x50], rax
0x1415d94d5 c685bf00000000 mov byte ptr [rbp + 0xbf], 0
0x1415d94dc 488d4de0 lea rcx, [rbp - 0x20]
0x1415d94e0 488d55a0 lea rdx, [rbp - 0x60]
0x1415d94e4 e8e7eb5aff call 0x140b880d0
0x1415d94e9 488d5d51 lea rbx, [rbp + 0x51]
0x1415d94ed c785a800000000000000 mov dword ptr [rbp + 0xa8], 0
0x1415d94f7 c685be00000002 mov byte ptr [rbp + 0xbe], 2
0x1415d94fe b001 mov al, 1
0x1415d9500 8985a4000000 mov dword ptr [rbp + 0xa4], eax
0x1415d9506 4c8d7578 lea r14, [rbp + 0x78]
0x1415d950a 4c8d7de0 lea r15, [rbp - 0x20]
0x1415d950e 4c8d6550 lea r12, [rbp + 0x50]
0x1415d9512 488d3dcb266a01 lea rdi, [rip + 0x16a26cb]
0x1415d9519 488945d8 mov qword ptr [rbp - 0x28], rax
0x1415d951d c785ac00000000000000 mov dword ptr [rbp + 0xac], 0
0x1415d9527 660f1f840000000000 nop word ptr [rax + rax]
0x1415d9530 4c89f1 mov rcx, r14
0x1415d9533 4c89fa mov rdx, r15
0x1415d9536 e8152fe6ff call 0x14143c450
0x1415d953b 488b4578 mov rax, qword ptr [rbp + 0x78]
0x1415d953f 4885c0 test rax, rax
0x1415d9542 0f84f0020000 je 0x1415d9838
0x1415d9548 488b8d88000000 mov rcx, qword ptr [rbp + 0x88]
0x1415d954f 488d1449 lea rdx, [rcx + rcx*2]
0x1415d9553 4c8b84d068010000 mov r8, qword ptr [rax + rdx*8 + 0x168]
0x1415d955b 4c8b8cd070010000 mov r9, qword ptr [rax + rdx*8 + 0x170]
0x1415d9563 4c898d90000000 mov qword ptr [rbp + 0x90], r9
0x1415d956a 4c8bacd078010000 mov r13, qword ptr [rax + rdx*8 + 0x178]
0x1415d9572 48c1e105 shl rcx, 5
0x1415d9576 0f100408 movups xmm0, xmmword ptr [rax + rcx]
0x1415d957a 0f104c0810 movups xmm1, xmmword ptr [rax + rcx + 0x10]
0x1415d957f 0f294550 movaps xmmword ptr [rbp + 0x50], xmm0
0x1415d9583 0f294d60 movaps xmmword ptr [rbp + 0x60], xmm1
0x1415d9587 4c89c0 mov rax, r8
0x1415d958a 48f7d8 neg rax
0x1415d958d 0f80a5020000 jo 0x1415d9838
0x1415d9593 4c898598000000 mov qword ptr [rbp + 0x98], r8
0x1415d959a 0fb64528 movzx eax, byte ptr [rbp + 0x28]
0x1415d959e 4883f806 cmp rax, 6
0x1415d95a2 0f8746010000 ja 0x1415d96ee
0x1415d95a8 48630487 movsxd rax, dword ptr [rdi + rax*4]
0x1415d95ac 4801f8 add rax, rdi
0x1415d95af ffe0 jmp rax
0x1415d95b1 0f284550 movaps xmm0, xmmword ptr [rbp + 0x50]
0x1415d95b5 0f284d60 movaps xmm1, xmmword ptr [rbp + 0x60]
0x1415d95b9 488d4528 lea rax, [rbp + 0x28]
0x1415d95bd 0f114810 movups xmmword ptr [rax + 0x10], xmm1
0x1415d95c1 0f1100 movups xmmword ptr [rax], xmm0
0x1415d95c4 4983fd0a cmp r13, 0xa
0x1415d95c8 7446 je 0x1415d9610
0x1415d95ca 40b603 mov sil, 3
0x1415d95cd 4983fd08 cmp r13, 8
0x1415d95d1 488b9598000000 mov rdx, qword ptr [rbp + 0x98]
0x1415d95d8 488b8d90000000 mov rcx, qword ptr [rbp + 0x90]
0x1415d95df 7560 jne 0x1415d9641
0x1415d95e1 488b01 mov rax, qword ptr [rcx]
0x1415d95e4 483b0535626b01 cmp rax, qword ptr [rip + 0x16b6235]
0x1415d95eb 0f840b010000 je 0x1415d96fc
0x1415d95f1 488b01 mov rax, qword ptr [rcx]
0x1415d95f4 483b052d626b01 cmp rax, qword ptr [rip + 0x16b622d]
0x1415d95fb 400f95c6 setne sil
0x1415d95ff 4000f6 add sil, sil
0x1415d9602 40fec6 inc sil
0x1415d9605 4885d2 test rdx, rdx
0x1415d9608 753c jne 0x1415d9646
0x1415d960a eb45 jmp 0x1415d9651
0x1415d960c 0f1f4000 nop dword ptr [rax]
0x1415d9610 488b8d90000000 mov rcx, qword ptr [rbp + 0x90]
0x1415d9617 488b01 mov rax, qword ptr [rcx]
0x1415d961a 4833050f626b01 xor rax, qword ptr [rip + 0x16b620f]
0x1415d9621 0fb75108 movzx edx, word ptr [rcx + 8]
0x1415d9625 6633150c626b01 xor dx, word ptr [rip + 0x16b620c]
0x1415d962c 0fb7d2 movzx edx, dx
0x1415d962f 4809c2 or rdx, rax
0x1415d9632 400f94c6 sete sil
0x1415d9636 4080f603 xor sil, 3
0x1415d963a 488b9598000000 mov rdx, qword ptr [rbp + 0x98]
0x1415d9641 4885d2 test rdx, rdx
0x1415d9644 740b je 0x1415d9651
0x1415d9646 41b801000000 mov r8d, 1
0x1415d964c e8cfa503ff call 0x140613c20
0x1415d9651 4080fe03 cmp sil, 3
0x1415d9655 0f85b1000000 jne 0x1415d970c
0x1415d965b 0fb64528 movzx eax, byte ptr [rbp + 0x28]
0x1415d965f c6452806 mov byte ptr [rbp + 0x28], 6
0x1415d9663 3c06 cmp al, 6
0x1415d9665 0f840a020000 je 0x1415d9875
0x1415d966b 884550 mov byte ptr [rbp + 0x50], al
0x1415d966e 488d4528 lea rax, [rbp + 0x28]
0x1415d9672 0f104001 movups xmm0, xmmword ptr [rax + 1]
0x1415d9676 0f104810 movups xmm1, xmmword ptr [rax + 0x10]
0x1415d967a 0f114b0f movups xmmword ptr [rbx + 0xf], xmm1
0x1415d967e 0f1103 movups xmmword ptr [rbx], xmm0
0x1415d9681 4c89e1 mov rcx, r12
0x1415d9684 e8b7a2fcff call 0x1415a3940
0x1415d9689 4885c0 test rax, rax
0x1415d968c 0f849efeffff je 0x1415d9530
0x1415d9692 e9ef010000 jmp 0x1415d9886
0x1415d9697 488b5530 mov rdx, qword ptr [rbp + 0x30]
0x1415d969b 4885d2 test rdx, rdx
0x1415d969e 0f840dffffff je 0x1415d95b1
0x1415d96a4 488b4d38 mov rcx, qword ptr [rbp + 0x38]
0x1415d96a8 41b801000000 mov r8d, 1
0x1415d96ae e86da503ff call 0x140613c20
0x1415d96b3 e9f9feffff jmp 0x1415d95b1
0x1415d96b8 488b4d38 mov rcx, qword ptr [rbp + 0x38]
0x1415d96bc 488b5540 mov rdx, qword ptr [rbp + 0x40]
0x1415d96c0 48894dd0 mov qword ptr [rbp - 0x30], rcx
0x1415d96c4 e8a7f62aff call 0x140888d70
0x1415d96c9 488b5530 mov rdx, qword ptr [rbp + 0x30]
0x1415d96cd 4885d2 test rdx, rdx
0x1415d96d0 0f84dbfeffff je 0x1415d95b1
0x1415d96d6 48c1e205 shl rdx, 5
0x1415d96da 41b808000000 mov r8d, 8
0x1415d96e0 488b4dd0 mov rcx, qword ptr [rbp - 0x30]
0x1415d96e4 e837a503ff call 0x140613c20
0x1415d96e9 e9c3feffff jmp 0x1415d95b1
0x1415d96ee 488d4d30 lea rcx, [rbp + 0x30]
0x1415d96f2 e849f82bff call 0x140898f40
0x1415d96f7 e9b5feffff jmp 0x1415d95b1
0x1415d96fc 31f6 xor esi, esi
0x1415d96fe 4885d2 test rdx, rdx
0x1415d9701 0f853fffffff jne 0x1415d9646
0x1415d9707 e945ffffff jmp 0x1415d9651
0x1415d970c 400fb6c6 movzx eax, sil
0x1415d9710 83f802 cmp eax, 2
0x1415d9713 7460 je 0x1415d9775
0x1415d9715 83f801 cmp eax, 1
0x1415d9718 0f85bd000000 jne 0x1415d97db
0x1415d971e 80bdbe00000002 cmp byte ptr [rbp + 0xbe], 2
0x1415d9725 0f85b7020000 jne 0x1415d99e2
0x1415d972b 0fb64528 movzx eax, byte ptr [rbp + 0x28]
0x1415d972f c6452806 mov byte ptr [rbp + 0x28], 6
0x1415d9733 3c06 cmp al, 6
0x1415d9735 0f840c020000 je 0x1415d9947
0x1415d973b 884550 mov byte ptr [rbp + 0x50], al
0x1415d973e 488d4528 lea rax, [rbp + 0x28]
0x1415d9742 0f104001 movups xmm0, xmmword ptr [rax + 1]
0x1415d9746 0f104810 movups xmm1, xmmword ptr [rax + 0x10]
0x1415d974a 0f114b0f movups xmmword ptr [rbx + 0xf], xmm1
0x1415d974e 0f1103 movups xmmword ptr [rbx], xmm0
0x1415d9751 4c89f1 mov rcx, r14
0x1415d9754 4c89e2 mov rdx, r12
0x1415d9757 e864021100 call 0x1416e99c0
0x1415d975c f6457801 test byte ptr [rbp + 0x78], 1
0x1415d9760 0f85bf010000 jne 0x1415d9925
0x1415d9766 0fb64579 movzx eax, byte ptr [rbp + 0x79]
0x1415d976a 8885be000000 mov byte ptr [rbp + 0xbe], al
0x1415d9770 e9bbfdffff jmp 0x1415d9530
0x1415d9775 f685a400000001 test byte ptr [rbp + 0xa4], 1
0x1415d977c 0f844a020000 je 0x1415d99cc
0x1415d9782 0fb64528 movzx eax, byte ptr [rbp + 0x28]
0x1415d9786 c6452806 mov byte ptr [rbp + 0x28], 6
0x1415d978a 3c06 cmp al, 6
0x1415d978c 0f849f010000 je 0x1415d9931
0x1415d9792 884550 mov byte ptr [rbp + 0x50], al
0x1415d9795 488d4528 lea rax, [rbp + 0x28]
0x1415d9799 0f104001 movups xmm0, xmmword ptr [rax + 1]
0x1415d979d 0f104810 movups xmm1, xmmword ptr [rax + 0x10]
0x1415d97a1 0f114b0f movups xmmword ptr [rbx + 0xf], xmm1
0x1415d97a5 0f1103 movups xmmword ptr [rbx], xmm0
0x1415d97a8 4c89f1 mov rcx, r14
0x1415d97ab 4c89e2 mov rdx, r12
0x1415d97ae e8ed37fcff call 0x14159cfa0
0x1415d97b3 f6457801 test byte ptr [rbp + 0x78], 1
0x1415d97b7 0f8568010000 jne 0x1415d9925
0x1415d97bd 8b457c mov eax, dword ptr [rbp + 0x7c]
0x1415d97c0 488945d8 mov qword ptr [rbp - 0x28], rax
0x1415d97c4 b001 mov al, 1
0x1415d97c6 8985a8000000 mov dword ptr [rbp + 0xa8], eax
0x1415d97cc c785a400000000000000 mov dword ptr [rbp + 0xa4], 0
0x1415d97d6 e955fdffff jmp 0x1415d9530
0x1415d97db 83bdac00000000 cmp dword ptr [rbp + 0xac], 0
0x1415d97e2 0f8510020000 jne 0x1415d99f8
0x1415d97e8 0fb64528 movzx eax, byte ptr [rbp + 0x28]
0x1415d97ec c6452806 mov byte ptr [rbp + 0x28], 6
0x1415d97f0 3c06 cmp al, 6
0x1415d97f2 0f8465010000 je 0x1415d995d
0x1415d97f8 884550 mov byte ptr [rbp + 0x50], al
0x1415d97fb 488d4528 lea rax, [rbp + 0x28]
0x1415d97ff 0f104001 movups xmm0, xmmword ptr [rax + 1]
0x1415d9803 0f104810 movups xmm1, xmmword ptr [rax + 0x10]
0x1415d9807 0f114b0f movups xmmword ptr [rbx + 0xf], xmm1
0x1415d980b 0f1103 movups xmmword ptr [rbx], xmm0
0x1415d980e 4c89f1 mov rcx, r14
0x1415d9811 4c89e2 mov rdx, r12
0x1415d9814 e88737fcff call 0x14159cfa0
0x1415d9819 f6457801 test byte ptr [rbp + 0x78], 1
0x1415d981d 0f8502010000 jne 0x1415d9925
0x1415d9823 8b457c mov eax, dword ptr [rbp + 0x7c]
0x1415d9826 894574 mov dword ptr [rbp + 0x74], eax
0x1415d9829 c785ac00000001000000 mov dword ptr [rbp + 0xac], 1
0x1415d9833 e9f8fcffff jmp 0x1415d9530
0x1415d9838 f685ac00000001 test byte ptr [rbp + 0xac], 1
0x1415d983f 7421 je 0x1415d9862
0x1415d9841 440fb685be000000 movzx r8d, byte ptr [rbp + 0xbe]
0x1415d9849 4180f802 cmp r8b, 2
0x1415d984d 757c jne 0x1415d98cb
0x1415d984f 488d0d84136a01 lea rcx, [rip + 0x16a1384]
0x1415d9856 ba08000000 mov edx, 8
0x1415d985b e860033201 call 0x1428f9bc0
0x1415d9860 eb24 jmp 0x1415d9886
0x1415d9862 488d0dc00d6a01 lea rcx, [rip + 0x16a0dc0]
0x1415d9869 ba08000000 mov edx, 8
0x1415d986e e84d033201 call 0x1428f9bc0
0x1415d9873 eb11 jmp 0x1415d9886
0x1415d9875 488d0dec096a01 lea rcx, [rip + 0x16a09ec]
0x1415d987c ba10000000 mov edx, 0x10
0x1415d9881 e87a073201 call 0x1428fa000
0x1415d9886 488b8db0000000 mov rcx, qword ptr [rbp + 0xb0]
0x1415d988d 48894108 mov qword ptr [rcx + 8], rax
0x1415d9891 c70101000000 mov dword ptr [rcx], 1
0x1415d9897 488d4de0 lea rcx, [rbp - 0x20]
0x1415d989b e850e52aff call 0x140887df0
0x1415d98a0 c685bf00000000 mov byte ptr [rbp + 0xbf], 0
0x1415d98a7 488d4d28 lea rcx, [rbp + 0x28]
0x1415d98ab e8804b2bff call 0x14088e430
0x1415d98b0 488b85b0000000 mov rax, qword ptr [rbp + 0xb0]
0x1415d98b7 4881c448010000 add rsp, 0x148
0x1415d98be 5b pop rbx
0x1415d98bf 5f pop rdi
0x1415d98c0 5e pop rsi
0x1415d98c1 415c pop r12
0x1415d98c3 415d pop r13
0x1415d98c5 415e pop r14
0x1415d98c7 415f pop r15
0x1415d98c9 5d pop rbp
0x1415d98ca c3 ret 
0x1415d98cb 48837d2000 cmp qword ptr [rbp + 0x20], 0
0x1415d98d0 0f8538010000 jne 0x1415d9a0e
0x1415d98d6 4180e001 and r8b, 1
0x1415d98da 488b55d8 mov rdx, qword ptr [rbp - 0x28]
0x1415d98de 48c1e220 shl rdx, 0x20
0x1415d98e2 31c9 xor ecx, ecx
0x1415d98e4 f685a800000001 test byte ptr [rbp + 0xa8], 1
0x1415d98eb 480f44d1 cmove rdx, rcx
0x1415d98ef 8b4574 mov eax, dword ptr [rbp + 0x74]
0x1415d98f2 4809d0 or rax, rdx
0x1415d98f5 488b95b0000000 mov rdx, qword ptr [rbp + 0xb0]
0x1415d98fc c6420402 mov byte ptr [rdx + 4], 2
0x1415d9900 44884205 mov byte ptr [rdx + 5], r8b
0x1415d9904 48894208 mov qword ptr [rdx + 8], rax
0x1415d9908 890a mov dword ptr [rdx], ecx
0x1415d990a 488d4de0 lea rcx, [rbp - 0x20]
0x1415d990e e8dde42aff call 0x140887df0
0x1415d9913 c685bf00000000 mov byte ptr [rbp + 0xbf], 0
0x1415d991a 488d4d28 lea rcx, [rbp + 0x28]
0x1415d991e e80d4b2bff call 0x14088e430
0x1415d9923 eb8b jmp 0x1415d98b0
0x1415d9925 488b8580000000 mov rax, qword ptr [rbp + 0x80]
0x1415d992c e955ffffff jmp 0x1415d9886
0x1415d9931 488d0d30096a01 lea rcx, [rip + 0x16a0930]
0x1415d9938 ba10000000 mov edx, 0x10
0x1415d993d e8be063201 call 0x1428fa000
0x1415d9942 e93fffffff jmp 0x1415d9886
0x1415d9947 488d0d1a096a01 lea rcx, [rip + 0x16a091a]
0x1415d994e ba10000000 mov edx, 0x10
0x1415d9953 e8a8063201 call 0x1428fa000
0x1415d9958 e929ffffff jmp 0x1415d9886
0x1415d995d 488d0d04096a01 lea rcx, [rip + 0x16a0904]
0x1415d9964 ba10000000 mov edx, 0x10
0x1415d9969 e892063201 call 0x1428fa000
0x1415d996e e913ffffff jmp 0x1415d9886
0x1415d9973 0f1002 movups xmm0, xmmword ptr [rdx]
0x1415d9976 0f104a10 movups xmm1, xmmword ptr [rdx + 0x10]
0x1415d997a 0f294df0 movaps xmmword ptr [rbp - 0x10], xmm1
0x1415d997e 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
0x1415d9982 488d4d50 lea rcx, [rbp + 0x50]
0x1415d9986 488d55e0 lea rdx, [rbp - 0x20]
0x1415d998a e881022f01 call 0x1428c9c10
0x1415d998f 488d15f2086a01 lea rdx, [rip + 0x16a08f2]
0x1415d9996 4c8d05c3066a01 lea r8, [rip + 0x16a06c3]
0x1415d999d 488d4d50 lea rcx, [rbp + 0x50]
0x1415d99a1 e85a002f01 call 0x1428c9a00
0x1415d99a6 488b8db0000000 mov rcx, qword ptr [rbp + 0xb0]
0x1415d99ad 48894108 mov qword ptr [rcx + 8], rax
0x1415d99b1 c70101000000 mov dword ptr [rcx], 1
0x1415d99b7 c685bf00000001 mov byte ptr [rbp + 0xbf], 1
0x1415d99be 488d4de0 lea rcx, [rbp - 0x20]
0x1415d99c2 e8c94028ff call 0x14085da90
0x1415d99c7 e9e4feffff jmp 0x1415d98b0
0x1415d99cc 488d0d0f126a01 lea rcx, [rip + 0x16a120f]
0x1415d99d3 ba0a000000 mov edx, 0xa
0x1415d99d8 e8d3033201 call 0x1428f9db0
0x1415d99dd e9a4feffff jmp 0x1415d9886
0x1415d99e2 488d0df1116a01 lea rcx, [rip + 0x16a11f1]
0x1415d99e9 ba08000000 mov edx, 8
0x1415d99ee e8bd033201 call 0x1428f9db0
0x1415d99f3 e98efeffff jmp 0x1415d9886
0x1415d99f8 488d0d2a0c6a01 lea rcx, [rip + 0x16a0c2a]
0x1415d99ff ba08000000 mov edx, 8
0x1415d9a04 e8a7033201 call 0x1428f9db0
0x1415d9a09 e978feffff jmp 0x1415d9886
0x1415d9a0e 488d1583066a01 lea rdx, [rip + 0x16a0683]
0x1415d9a15 4c8d0544066a01 lea r8, [rip + 0x16a0644]
0x1415d9a1c 488b4dc8 mov rcx, qword ptr [rbp - 0x38]
0x1415d9a20 e8fb023201 call 0x1428f9d20
0x1415d9a25 b901000000 mov ecx, 1
0x1415d9a2a 488b95b0000000 mov rdx, qword ptr [rbp + 0xb0]
0x1415d9a31 e9cefeffff jmp 0x1415d9904
0x1415d9a36 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1415d9a40 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415d9a45 55 push rbp
0x1415d9a46 4157 push r15
0x1415d9a48 4156 push r14
0x1415d9a4a 4155 push r13
0x1415d9a4c 4154 push r12
0x1415d9a4e 56 push rsi
0x1415d9a4f 57 push rdi
0x1415d9a50 53 push rbx
0x1415d9a51 4883ec28 sub rsp, 0x28
0x1415d9a55 488daa80000000 lea rbp, [rdx + 0x80]
0x1415d9a5c 488b45b8 mov rax, qword ptr [rbp - 0x48]
0x1415d9a60 83c0fb add eax, -5
0x1415d9a63 3c02 cmp al, 2
0x1415d9a65 7209 jb 0x1415d9a70
0x1415d9a67 f685bf00000001 test byte ptr [rbp + 0xbf], 1
0x1415d9a6e 7411 je 0x1415d9a81
0x1415d9a70 4883c428 add rsp, 0x28
0x1415d9a74 5b pop rbx
0x1415d9a75 5f pop rdi
0x1415d9a76 5e pop rsi
0x1415d9a77 415c pop r12
0x1415d9a79 415d pop r13
0x1415d9a7b 415e pop r14
0x1415d9a7d 415f pop r15
0x1415d9a7f 5d pop rbp
0x1415d9a80 c3 ret 
0x1415d9a81 488b4dc0 mov rcx, qword ptr [rbp - 0x40]
0x1415d9a85 e8064028ff call 0x14085da90
0x1415d9a8a ebe4 jmp 0x1415d9a70
0x1415d9a8c 0f1f4000 nop dword ptr [rax]
0x1415d9a90 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415d9a95 55 push rbp
0x1415d9a96 4157 push r15
0x1415d9a98 4156 push r14
0x1415d9a9a 4155 push r13
0x1415d9a9c 4154 push r12
0x1415d9a9e 56 push rsi
0x1415d9a9f 57 push rdi
0x1415d9aa0 53 push rbx
0x1415d9aa1 4883ec28 sub rsp, 0x28
0x1415d9aa5 488daa80000000 lea rbp, [rdx + 0x80]
0x1415d9aac 488b5530 mov rdx, qword ptr [rbp + 0x30]
0x1415d9ab0 4885d2 test rdx, rdx
0x1415d9ab3 7413 je 0x1415d9ac8
0x1415d9ab5 48c1e205 shl rdx, 5
0x1415d9ab9 41b808000000 mov r8d, 8
0x1415d9abf 488b4dd0 mov rcx, qword ptr [rbp - 0x30]
0x1415d9ac3 e858a103ff call 0x140613c20
0x1415d9ac8 90 nop 
0x1415d9ac9 4883c428 add rsp, 0x28
0x1415d9acd 5b pop rbx
0x1415d9ace 5f pop rdi
0x1415d9acf 5e pop rsi
0x1415d9ad0 415c pop r12
0x1415d9ad2 415d pop r13
0x1415d9ad4 415e pop r14
0x1415d9ad6 415f pop r15
0x1415d9ad8 5d pop rbp
0x1415d9ad9 c3 ret 
0x1415d9ada 660f1f440000 nop word ptr [rax + rax]
0x1415d9ae0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415d9ae5 55 push rbp
0x1415d9ae6 4157 push r15
0x1415d9ae8 4156 push r14
0x1415d9aea 4155 push r13
0x1415d9aec 4154 push r12
0x1415d9aee 56 push rsi
0x1415d9aef 57 push rdi
0x1415d9af0 53 push rbx
0x1415d9af1 4883ec28 sub rsp, 0x28
0x1415d9af5 488daa80000000 lea rbp, [rdx + 0x80]
0x1415d9afc 0f284550 movaps xmm0, xmmword ptr [rbp + 0x50]
0x1415d9b00 0f284d60 movaps xmm1, xmmword ptr [rbp + 0x60]
0x1415d9b04 488d4528 lea rax, [rbp + 0x28]
0x1415d9b08 0f114810 movups xmmword ptr [rax + 0x10], xmm1
0x1415d9b0c 0f1100 movups xmmword ptr [rax], xmm0
0x1415d9b0f 4883bd9800000000 cmp qword ptr [rbp + 0x98], 0
0x1415d9b17 7419 je 0x1415d9b32
0x1415d9b19 41b801000000 mov r8d, 1
0x1415d9b1f 488b8d90000000 mov rcx, qword ptr [rbp + 0x90]
0x1415d9b26 488b9598000000 mov rdx, qword ptr [rbp + 0x98]
0x1415d9b2d e8eea003ff call 0x140613c20
0x1415d9b32 90 nop 
0x1415d9b33 4883c428 add rsp, 0x28
0x1415d9b37 5b pop rbx
0x1415d9b38 5f pop rdi
0x1415d9b39 5e pop rsi
0x1415d9b3a 415c pop r12
0x1415d9b3c 415d pop r13
0x1415d9b3e 415e pop r14
0x1415d9b40 415f pop r15
0x1415d9b42 5d pop rbp
0x1415d9b43 c3 ret 
0x1415d9b44 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1415d9b50 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415d9b55 55 push rbp
0x1415d9b56 4157 push r15
0x1415d9b58 4156 push r14
0x1415d9b5a 4155 push r13
0x1415d9b5c 4154 push r12
0x1415d9b5e 56 push rsi
0x1415d9b5f 57 push rdi
0x1415d9b60 53 push rbx
0x1415d9b61 4883ec28 sub rsp, 0x28
0x1415d9b65 488daa80000000 lea rbp, [rdx + 0x80]
0x1415d9b6c c685bf00000000 mov byte ptr [rbp + 0xbf], 0
0x1415d9b73 488d4de0 lea rcx, [rbp - 0x20]
0x1415d9b77 e884092bff call 0x14088a500
0x1415d9b7c c685bf00000000 mov byte ptr [rbp + 0xbf], 0
0x1415d9b83 4883c428 add rsp, 0x28
0x1415d9b87 5b pop rbx
0x1415d9b88 5f pop rdi
0x1415d9b89 5e pop rsi
0x1415d9b8a 415c pop r12
0x1415d9b8c 415d pop r13
0x1415d9b8e 415e pop r14
0x1415d9b90 415f pop r15
0x1415d9b92 5d pop rbp
0x1415d9b93 c3 ret 
0x1415d9b94 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1415d9ba0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415d9ba5 55 push rbp
0x1415d9ba6 4157 push r15
0x1415d9ba8 4156 push r14
0x1415d9baa 4155 push r13
0x1415d9bac 4154 push r12
0x1415d9bae 56 push rsi
0x1415d9baf 57 push rdi
0x1415d9bb0 53 push rbx
0x1415d9bb1 4883ec28 sub rsp, 0x28
0x1415d9bb5 488daa80000000 lea rbp, [rdx + 0x80]
0x1415d9bbc 488d4de0 lea rcx, [rbp - 0x20]
0x1415d9bc0 e8cb3e28ff call 0x14085da90
0x1415d9bc5 c685bf00000001 mov byte ptr [rbp + 0xbf], 1
0x1415d9bcc 4883c428 add rsp, 0x28
0x1415d9bd0 5b pop rbx
0x1415d9bd1 5f pop rdi
0x1415d9bd2 5e pop rsi
0x1415d9bd3 415c pop r12
0x1415d9bd5 415d pop r13
0x1415d9bd7 415e pop r14
0x1415d9bd9 415f pop r15
0x1415d9bdb 5d pop rbp
0x1415d9bdc c3 ret 
0x1415d9bdd 0f1f00 nop dword ptr [rax]
0x1415d9be0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415d9be5 55 push rbp
0x1415d9be6 4157 push r15
0x1415d9be8 4156 push r14
0x1415d9bea 4155 push r13
0x1415d9bec 4154 push r12
0x1415d9bee 56 push rsi
0x1415d9bef 57 push rdi
0x1415d9bf0 53 push rbx
0x1415d9bf1 4883ec28 sub rsp, 0x28
0x1415d9bf5 488daa80000000 lea rbp, [rdx + 0x80]
0x1415d9bfc c685bf00000000 mov byte ptr [rbp + 0xbf], 0
0x1415d9c03 488d4d28 lea rcx, [rbp + 0x28]
0x1415d9c07 e824482bff call 0x14088e430
0x1415d9c0c c685bf00000000 mov byte ptr [rbp + 0xbf], 0
0x1415d9c13 4883c428 add rsp, 0x28
0x1415d9c17 5b pop rbx
0x1415d9c18 5f pop rdi
0x1415d9c19 5e pop rsi
0x1415d9c1a 415c pop r12
0x1415d9c1c 415d pop r13
0x1415d9c1e 415e pop r14
0x1415d9c20 415f pop r15
0x1415d9c22 5d pop rbp
0x1415d9c23 c3 ret 
0x1415d9c24 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1415d9c30 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1415d9c35 55 push rbp
0x1415d9c36 4157 push r15
0x1415d9c38 4156 push r14
0x1415d9c3a 4155 push r13
0x1415d9c3c 4154 push r12
0x1415d9c3e 56 push rsi
0x1415d9c3f 57 push rdi
0x1415d9c40 53 push rbx
0x1415d9c41 4883ec28 sub rsp, 0x28
0x1415d9c45 488daa80000000 lea rbp, [rdx + 0x80]
0x1415d9c4c c685bf00000000 mov byte ptr [rbp + 0xbf], 0
0x1415d9c53 488d4d28 lea rcx, [rbp + 0x28]
0x1415d9c57 e8d4472bff call 0x14088e430
0x1415d9c5c c685bf00000000 mov byte ptr [rbp + 0xbf], 0
0x1415d9c63 4883c428 add rsp, 0x28
0x1415d9c67 5b pop rbx
0x1415d9c68 5f pop rdi
0x1415d9c69 5e pop rsi
0x1415d9c6a 415c pop r12
0x1415d9c6c 415d pop r13
0x1415d9c6e 415e pop r14
0x1415d9c70 415f pop r15
0x1415d9c72 5d pop rbp
0x1415d9c73 c3 ret 
