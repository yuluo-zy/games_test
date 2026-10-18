0x14142c2a0 55 push rbp
0x14142c2a1 4157 push r15
0x14142c2a3 4156 push r14
0x14142c2a5 4155 push r13
0x14142c2a7 4154 push r12
0x14142c2a9 56 push rsi
0x14142c2aa 57 push rdi
0x14142c2ab 53 push rbx
0x14142c2ac 4881ece8000000 sub rsp, 0xe8
0x14142c2b3 488dac2480000000 lea rbp, [rsp + 0x80]
0x14142c2bb 0f297550 movaps xmmword ptr [rbp + 0x50], xmm6
0x14142c2bf 48c74548feffffff mov qword ptr [rbp + 0x48], 0xfffffffffffffffe
0x14142c2c7 4889d7 mov rdi, rdx
0x14142c2ca 4889ce mov rsi, rcx
0x14142c2cd 4c8b5220 mov r10, qword ptr [rdx + 0x20]
0x14142c2d1 488b4228 mov rax, qword ptr [rdx + 0x28]
0x14142c2d5 4c39d0 cmp rax, r10
0x14142c2d8 734c jae 0x14142c326
0x14142c2da 4c8d7718 lea r14, [rdi + 0x18]
0x14142c2de 488b5718 mov rdx, qword ptr [rdi + 0x18]
0x14142c2e2 48ffc0 inc rax
0x14142c2e5 4d89d0 mov r8, r10
0x14142c2e8 49f7d8 neg r8
0x14142c2eb 49bf0026000001000000 movabs r15, 0x100002600
0x14142c2f5 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14142c300 440fb64c02ff movzx r9d, byte ptr [rdx + rax - 1]
0x14142c306 4983f920 cmp r9, 0x20
0x14142c30a 773d ja 0x14142c349
0x14142c30c 4d0fa3cf bt r15, r9
0x14142c310 7337 jae 0x14142c349
0x14142c312 48894728 mov qword ptr [rdi + 0x28], rax
0x14142c316 4d8d0c00 lea r9, [r8 + rax]
0x14142c31a 49ffc1 inc r9
0x14142c31d 48ffc0 inc rax
0x14142c320 4983f901 cmp r9, 1
0x14142c324 75da jne 0x14142c300
0x14142c326 48c7451805000000 mov qword ptr [rbp + 0x18], 5
0x14142c32e 488d5518 lea rdx, [rbp + 0x18]
0x14142c332 4889f9 mov rcx, rdi
0x14142c335 e8e6454701 call 0x1428a0920
0x14142c33a 48894608 mov qword ptr [rsi + 8], rax
0x14142c33e c70601000000 mov dword ptr [rsi], 1
0x14142c344 e9f8070000 jmp 0x14142cb41
0x14142c349 4183f95b cmp r9d, 0x5b
0x14142c34d 0f843e040000 je 0x14142c791
0x14142c353 4183f97b cmp r9d, 0x7b
0x14142c357 0f8557060000 jne 0x14142c9b4
0x14142c35d fe4f30 dec byte ptr [rdi + 0x30]
0x14142c360 0f84c8080000 je 0x14142cc2e
0x14142c366 48894728 mov qword ptr [rdi + 0x28], rax
0x14142c36a 4c39d0 cmp rax, r10
0x14142c36d 0f8311040000 jae 0x14142c784
0x14142c373 c7451000000000 mov dword ptr [rbp + 0x10], 0
0x14142c37a 41b401 mov r12b, 1
0x14142c37d 488d4d18 lea rcx, [rbp + 0x18]
0x14142c381 c7454000000000 mov dword ptr [rbp + 0x40], 0
0x14142c388 c745e400000000 mov dword ptr [rbp - 0x1c], 0
0x14142c38f c745e800000000 mov dword ptr [rbp - 0x18], 0
0x14142c396 c745ec00000000 mov dword ptr [rbp - 0x14], 0
0x14142c39d 498b16 mov rdx, qword ptr [r14]
0x14142c3a0 440fb60402 movzx r8d, byte ptr [rdx + rax]
0x14142c3a5 4983f82c cmp r8, 0x2c
0x14142c3a9 7775 ja 0x14142c420
0x14142c3ab 4d0fa3c7 bt r15, r8
0x14142c3af 731f jae 0x14142c3d0
0x14142c3b1 48ffc0 inc rax
0x14142c3b4 48894728 mov qword ptr [rdi + 0x28], rax
0x14142c3b8 4939c2 cmp r10, rax
0x14142c3bb 75e3 jne 0x14142c3a0
0x14142c3bd e9c2030000 jmp 0x14142c784
0x14142c3c2 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14142c3d0 4983f82c cmp r8, 0x2c
0x14142c3d4 754a jne 0x14142c420
0x14142c3d6 41f6c401 test r12b, 1
0x14142c3da 0f850c080000 jne 0x14142cbec
0x14142c3e0 48ffc0 inc rax
0x14142c3e3 48894728 mov qword ptr [rdi + 0x28], rax
0x14142c3e7 4c39d0 cmp rax, r10
0x14142c3ea 0f8383050000 jae 0x14142c973
0x14142c3f0 440fb60402 movzx r8d, byte ptr [rdx + rax]
0x14142c3f5 4983f820 cmp r8, 0x20
0x14142c3f9 773c ja 0x14142c437
0x14142c3fb 4d0fa3c7 bt r15, r8
0x14142c3ff 7336 jae 0x14142c437
0x14142c401 48ffc0 inc rax
0x14142c404 48894728 mov qword ptr [rdi + 0x28], rax
0x14142c408 4939c2 cmp r10, rax
0x14142c40b 75e3 jne 0x14142c3f0
0x14142c40d e961050000 jmp 0x14142c973
0x14142c412 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14142c420 4183f87d cmp r8d, 0x7d
0x14142c424 0f84d5040000 je 0x14142c8ff
0x14142c42a 41f6c401 test r12b, 1
0x14142c42e 0f8407080000 je 0x14142cc3b
0x14142c434 4531e4 xor r12d, r12d
0x14142c437 4180f822 cmp r8b, 0x22
0x14142c43b 0f8595070000 jne 0x14142cbd6
0x14142c441 48ffc0 inc rax
0x14142c444 48894728 mov qword ptr [rdi + 0x28], rax
0x14142c448 48c7471000000000 mov qword ptr [rdi + 0x10], 0
0x14142c450 4889cb mov rbx, rcx
0x14142c453 4c89f2 mov rdx, r14
0x14142c456 4989f8 mov r8, rdi
0x14142c459 e8b2e475ff call 0x140b8a910
0x14142c45e 837d1802 cmp dword ptr [rbp + 0x18], 2
0x14142c462 488b5520 mov rdx, qword ptr [rbp + 0x20]
0x14142c466 0f8480050000 je 0x14142c9ec
0x14142c46c 4c8b4528 mov r8, qword ptr [rbp + 0x28]
0x14142c470 488d4df0 lea rcx, [rbp - 0x10]
0x14142c474 e8d7713900 call 0x1417c3650
0x14142c479 f645f001 test byte ptr [rbp - 0x10], 1
0x14142c47d 0f8565050000 jne 0x14142c9e8
0x14142c483 0fb645f1 movzx eax, byte ptr [rbp - 0xf]
0x14142c487 488d0d5a178301 lea rcx, [rip + 0x183175a]
0x14142c48e 48630481 movsxd rax, dword ptr [rcx + rax*4]
0x14142c492 4801c8 add rax, rcx
0x14142c495 ffe0 jmp rax
0x14142c497 837d1000 cmp dword ptr [rbp + 0x10], 0
0x14142c49b 0f85ff070000 jne 0x14142cca0
0x14142c4a1 488b4f20 mov rcx, qword ptr [rdi + 0x20]
0x14142c4a5 488b4728 mov rax, qword ptr [rdi + 0x28]
0x14142c4a9 4839c8 cmp rax, rcx
0x14142c4ac 0f83d2020000 jae 0x14142c784
0x14142c4b2 498b16 mov rdx, qword ptr [r14]
0x14142c4b5 48ffc0 inc rax
0x14142c4b8 48f7d9 neg rcx
0x14142c4bb 440fb64402ff movzx r8d, byte ptr [rdx + rax - 1]
0x14142c4c1 4983f83a cmp r8, 0x3a
0x14142c4c5 0f877d070000 ja 0x14142cc48
0x14142c4cb 4d0fa3c7 bt r15, r8
0x14142c4cf 0f8332020000 jae 0x14142c707
0x14142c4d5 48894728 mov qword ptr [rdi + 0x28], rax
0x14142c4d9 4c8d0401 lea r8, [rcx + rax]
0x14142c4dd 49ffc0 inc r8
0x14142c4e0 48ffc0 inc rax
0x14142c4e3 4983f801 cmp r8, 1
0x14142c4e7 75d2 jne 0x14142c4bb
0x14142c4e9 e996020000 jmp 0x14142c784
0x14142c4ee 837dec00 cmp dword ptr [rbp - 0x14], 0
0x14142c4f2 0f8592070000 jne 0x14142cc8a
0x14142c4f8 488b4f20 mov rcx, qword ptr [rdi + 0x20]
0x14142c4fc 488b4728 mov rax, qword ptr [rdi + 0x28]
0x14142c500 4839c8 cmp rax, rcx
0x14142c503 0f837b020000 jae 0x14142c784
0x14142c509 498b16 mov rdx, qword ptr [r14]
0x14142c50c 48ffc0 inc rax
0x14142c50f 48f7d9 neg rcx
0x14142c512 440fb64402ff movzx r8d, byte ptr [rdx + rax - 1]
0x14142c518 4983f83a cmp r8, 0x3a
0x14142c51c 0f8726070000 ja 0x14142cc48
0x14142c522 4d0fa3c7 bt r15, r8
0x14142c526 0f8334010000 jae 0x14142c660
0x14142c52c 48894728 mov qword ptr [rdi + 0x28], rax
0x14142c530 4c8d0401 lea r8, [rcx + rax]
0x14142c534 49ffc0 inc r8
0x14142c537 48ffc0 inc rax
0x14142c53a 4983f801 cmp r8, 1
0x14142c53e 75d2 jne 0x14142c512
0x14142c540 e93f020000 jmp 0x14142c784
0x14142c545 837de400 cmp dword ptr [rbp - 0x1c], 0
0x14142c549 0f8525070000 jne 0x14142cc74
0x14142c54f 488b4f20 mov rcx, qword ptr [rdi + 0x20]
0x14142c553 488b4728 mov rax, qword ptr [rdi + 0x28]
0x14142c557 4839c8 cmp rax, rcx
0x14142c55a 0f8324020000 jae 0x14142c784
0x14142c560 498b16 mov rdx, qword ptr [r14]
0x14142c563 48ffc0 inc rax
0x14142c566 48f7d9 neg rcx
0x14142c569 440fb64402ff movzx r8d, byte ptr [rdx + rax - 1]
0x14142c56f 4983f83a cmp r8, 0x3a
0x14142c573 0f87cf060000 ja 0x14142cc48
0x14142c579 4d0fa3c7 bt r15, r8
0x14142c57d 0f8313010000 jae 0x14142c696
0x14142c583 48894728 mov qword ptr [rdi + 0x28], rax
0x14142c587 4c8d0401 lea r8, [rcx + rax]
0x14142c58b 49ffc0 inc r8
0x14142c58e 48ffc0 inc rax
0x14142c591 4983f801 cmp r8, 1
0x14142c595 75d2 jne 0x14142c569
0x14142c597 e9e8010000 jmp 0x14142c784
0x14142c59c 837de800 cmp dword ptr [rbp - 0x18], 0
0x14142c5a0 0f85af060000 jne 0x14142cc55
0x14142c5a6 488b4f20 mov rcx, qword ptr [rdi + 0x20]
0x14142c5aa 488b4728 mov rax, qword ptr [rdi + 0x28]
0x14142c5ae 4839c8 cmp rax, rcx
0x14142c5b1 0f83cd010000 jae 0x14142c784
0x14142c5b7 498b16 mov rdx, qword ptr [r14]
0x14142c5ba 48ffc0 inc rax
0x14142c5bd 48f7d9 neg rcx
0x14142c5c0 440fb64402ff movzx r8d, byte ptr [rdx + rax - 1]
0x14142c5c6 4983f83a cmp r8, 0x3a
0x14142c5ca 0f8778060000 ja 0x14142cc48
0x14142c5d0 4d0fa3c7 bt r15, r8
0x14142c5d4 0f83f2000000 jae 0x14142c6cc
0x14142c5da 48894728 mov qword ptr [rdi + 0x28], rax
0x14142c5de 4c8d0401 lea r8, [rcx + rax]
0x14142c5e2 49ffc0 inc r8
0x14142c5e5 48ffc0 inc rax
0x14142c5e8 4983f801 cmp r8, 1
0x14142c5ec 75d2 jne 0x14142c5c0
0x14142c5ee e991010000 jmp 0x14142c784
0x14142c5f3 837d4000 cmp dword ptr [rbp + 0x40], 0
0x14142c5f7 0f856e060000 jne 0x14142cc6b
0x14142c5fd 488b4f20 mov rcx, qword ptr [rdi + 0x20]
0x14142c601 488b4728 mov rax, qword ptr [rdi + 0x28]
0x14142c605 4839c8 cmp rax, rcx
0x14142c608 0f8376010000 jae 0x14142c784
0x14142c60e 498b16 mov rdx, qword ptr [r14]
0x14142c611 48ffc0 inc rax
0x14142c614 48f7d9 neg rcx
0x14142c617 440fb64402ff movzx r8d, byte ptr [rdx + rax - 1]
0x14142c61d 4983f83a cmp r8, 0x3a
0x14142c621 0f8721060000 ja 0x14142cc48
0x14142c627 4d0fa3c7 bt r15, r8
0x14142c62b 0f830f010000 jae 0x14142c740
0x14142c631 48894728 mov qword ptr [rdi + 0x28], rax
0x14142c635 4c8d0401 lea r8, [rcx + rax]
0x14142c639 49ffc0 inc r8
0x14142c63c 48ffc0 inc rax
0x14142c63f 4983f801 cmp r8, 1
0x14142c643 75d2 jne 0x14142c617
0x14142c645 e93a010000 jmp 0x14142c784
0x14142c64a 4889f9 mov rcx, rdi
0x14142c64d e8aecafdff call 0x141409100
0x14142c652 4885c0 test rax, rax
0x14142c655 0f8415010000 je 0x14142c770
0x14142c65b e962050000 jmp 0x14142cbc2
0x14142c660 4983f83a cmp r8, 0x3a
0x14142c664 0f85de050000 jne 0x14142cc48
0x14142c66a 48894728 mov qword ptr [rdi + 0x28], rax
0x14142c66e 488d4df0 lea rcx, [rbp - 0x10]
0x14142c672 4889fa mov rdx, rdi
0x14142c675 e8f60bfeff call 0x14140d270
0x14142c67a f645f001 test byte ptr [rbp - 0x10], 1
0x14142c67e 0f85ea040000 jne 0x14142cb6e
0x14142c684 8b45f4 mov eax, dword ptr [rbp - 0xc]
0x14142c687 8945dc mov dword ptr [rbp - 0x24], eax
0x14142c68a c745ec01000000 mov dword ptr [rbp - 0x14], 1
0x14142c691 e9da000000 jmp 0x14142c770
0x14142c696 4983f83a cmp r8, 0x3a
0x14142c69a 0f85a8050000 jne 0x14142cc48
0x14142c6a0 48894728 mov qword ptr [rdi + 0x28], rax
0x14142c6a4 488d4df0 lea rcx, [rbp - 0x10]
0x14142c6a8 4889fa mov rdx, rdi
0x14142c6ab e8c00bfeff call 0x14140d270
0x14142c6b0 f645f001 test byte ptr [rbp - 0x10], 1
0x14142c6b4 0f85b4040000 jne 0x14142cb6e
0x14142c6ba 8b45f4 mov eax, dword ptr [rbp - 0xc]
0x14142c6bd 8945d4 mov dword ptr [rbp - 0x2c], eax
0x14142c6c0 c745e401000000 mov dword ptr [rbp - 0x1c], 1
0x14142c6c7 e9a4000000 jmp 0x14142c770
0x14142c6cc 4983f83a cmp r8, 0x3a
0x14142c6d0 0f8572050000 jne 0x14142cc48
0x14142c6d6 48894728 mov qword ptr [rdi + 0x28], rax
0x14142c6da 488d4df0 lea rcx, [rbp - 0x10]
0x14142c6de 4889fa mov rdx, rdi
0x14142c6e1 e83a71feff call 0x141413820
0x14142c6e6 f645f001 test byte ptr [rbp - 0x10], 1
0x14142c6ea 0f857e040000 jne 0x14142cb6e
0x14142c6f0 8b45f4 mov eax, dword ptr [rbp - 0xc]
0x14142c6f3 8945d8 mov dword ptr [rbp - 0x28], eax
0x14142c6f6 488b45f8 mov rax, qword ptr [rbp - 8]
0x14142c6fa 488945a8 mov qword ptr [rbp - 0x58], rax
0x14142c6fe c745e801000000 mov dword ptr [rbp - 0x18], 1
0x14142c705 eb69 jmp 0x14142c770
0x14142c707 4983f83a cmp r8, 0x3a
0x14142c70b 0f8537050000 jne 0x14142cc48
0x14142c711 48894728 mov qword ptr [rdi + 0x28], rax
0x14142c715 488d4df0 lea rcx, [rbp - 0x10]
0x14142c719 4889fa mov rdx, rdi
0x14142c71c e8ff70feff call 0x141413820
0x14142c721 f645f001 test byte ptr [rbp - 0x10], 1
0x14142c725 0f8543040000 jne 0x14142cb6e
0x14142c72b 448b6df4 mov r13d, dword ptr [rbp - 0xc]
0x14142c72f 488b45f8 mov rax, qword ptr [rbp - 8]
0x14142c733 488945a0 mov qword ptr [rbp - 0x60], rax
0x14142c737 c7451001000000 mov dword ptr [rbp + 0x10], 1
0x14142c73e eb30 jmp 0x14142c770
0x14142c740 4983f83a cmp r8, 0x3a
0x14142c744 0f85fe040000 jne 0x14142cc48
0x14142c74a 48894728 mov qword ptr [rdi + 0x28], rax
0x14142c74e 488d4df0 lea rcx, [rbp - 0x10]
0x14142c752 4889fa mov rdx, rdi
0x14142c755 e87608feff call 0x14140cfd0
0x14142c75a f645f001 test byte ptr [rbp - 0x10], 1
0x14142c75e 0f850a040000 jne 0x14142cb6e
0x14142c764 f30f1075f4 movss xmm6, dword ptr [rbp - 0xc]
0x14142c769 c7454001000000 mov dword ptr [rbp + 0x40], 1
0x14142c770 4c8b5720 mov r10, qword ptr [rdi + 0x20]
0x14142c774 488b4728 mov rax, qword ptr [rdi + 0x28]
0x14142c778 4c39d0 cmp rax, r10
0x14142c77b 4889d9 mov rcx, rbx
0x14142c77e 0f8219fcffff jb 0x14142c39d
0x14142c784 48c7451803000000 mov qword ptr [rbp + 0x18], 3
0x14142c78c e9ea010000 jmp 0x14142c97b
0x14142c791 fe4f30 dec byte ptr [rdi + 0x30]
0x14142c794 0f8494040000 je 0x14142cc2e
0x14142c79a 48894728 mov qword ptr [rdi + 0x28], rax
0x14142c79e 48897df0 mov qword ptr [rbp - 0x10], rdi
0x14142c7a2 c645f801 mov byte ptr [rbp - 8], 1
0x14142c7a6 488d4d18 lea rcx, [rbp + 0x18]
0x14142c7aa 488d55f0 lea rdx, [rbp - 0x10]
0x14142c7ae e85dddfdff call 0x14140a510
0x14142c7b3 8b4518 mov eax, dword ptr [rbp + 0x18]
0x14142c7b6 83f802 cmp eax, 2
0x14142c7b9 0f8493000000 je 0x14142c852
0x14142c7bf 85c0 test eax, eax
0x14142c7c1 0f8444020000 je 0x14142ca0b
0x14142c7c7 448b751c mov r14d, dword ptr [rbp + 0x1c]
0x14142c7cb 488b5d20 mov rbx, qword ptr [rbp + 0x20]
0x14142c7cf 488d4d18 lea rcx, [rbp + 0x18]
0x14142c7d3 488d55f0 lea rdx, [rbp - 0x10]
0x14142c7d7 e874f8fdff call 0x14140c050
0x14142c7dc f6451801 test byte ptr [rbp + 0x18], 1
0x14142c7e0 7570 jne 0x14142c852
0x14142c7e2 f6451c01 test byte ptr [rbp + 0x1c], 1
0x14142c7e6 0f84ac030000 je 0x14142cb98
0x14142c7ec f30f107520 movss xmm6, dword ptr [rbp + 0x20]
0x14142c7f1 488d4d18 lea rcx, [rbp + 0x18]
0x14142c7f5 488d55f0 lea rdx, [rbp - 0x10]
0x14142c7f9 e842dafdff call 0x14140a240
0x14142c7fe f6451801 test byte ptr [rbp + 0x18], 1
0x14142c802 754e jne 0x14142c852
0x14142c804 f6451c01 test byte ptr [rbp + 0x1c], 1
0x14142c808 0f849f030000 je 0x14142cbad
0x14142c80e 448b6520 mov r12d, dword ptr [rbp + 0x20]
0x14142c812 488d4d18 lea rcx, [rbp + 0x18]
0x14142c816 488d55f0 lea rdx, [rbp - 0x10]
0x14142c81a e8f1dcfdff call 0x14140a510
0x14142c81f 8b4518 mov eax, dword ptr [rbp + 0x18]
0x14142c822 83f802 cmp eax, 2
0x14142c825 742b je 0x14142c852
0x14142c827 85c0 test eax, eax
0x14142c829 0f84ca030000 je 0x14142cbf9
0x14142c82f 44897510 mov dword ptr [rbp + 0x10], r14d
0x14142c833 448b7d1c mov r15d, dword ptr [rbp + 0x1c]
0x14142c837 4c8b6d20 mov r13, qword ptr [rbp + 0x20]
0x14142c83b 488d4d18 lea rcx, [rbp + 0x18]
0x14142c83f 488d55f0 lea rdx, [rbp - 0x10]
0x14142c843 e8f8d9fdff call 0x14140a240
0x14142c848 f6451801 test byte ptr [rbp + 0x18], 1
0x14142c84c 0f8425030000 je 0x14142cb77
0x14142c852 488b5d20 mov rbx, qword ptr [rbp + 0x20]
0x14142c856 41be01000000 mov r14d, 1
0x14142c85c 48c7454000000000 mov qword ptr [rbp + 0x40], 0
0x14142c864 fe4730 inc byte ptr [rdi + 0x30]
0x14142c867 44897518 mov dword ptr [rbp + 0x18], r14d
0x14142c86b 4189cd mov r13d, ecx
0x14142c86e 894d1c mov dword ptr [rbp + 0x1c], ecx
0x14142c871 48895d20 mov qword ptr [rbp + 0x20], rbx
0x14142c875 f30f117528 movss dword ptr [rbp + 0x28], xmm6
0x14142c87a 4489652c mov dword ptr [rbp + 0x2c], r12d
0x14142c87e 44897d30 mov dword ptr [rbp + 0x30], r15d
0x14142c882 48895534 mov qword ptr [rbp + 0x34], rdx
0x14142c886 89453c mov dword ptr [rbp + 0x3c], eax
0x14142c889 4889f9 mov rcx, rdi
0x14142c88c e85fbcc2fe call 0x1400584f0
0x14142c891 488d5528 lea rdx, [rbp + 0x28]
0x14142c895 488b4a10 mov rcx, qword ptr [rdx + 0x10]
0x14142c899 48894d00 mov qword ptr [rbp], rcx
0x14142c89d 0f1002 movups xmm0, xmmword ptr [rdx]
0x14142c8a0 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
0x14142c8a4 4585f6 test r14d, r14d
0x14142c8a7 0f84a2000000 je 0x14142c94f
0x14142c8ad 41be01000000 mov r14d, 1
0x14142c8b3 4885c0 test rax, rax
0x14142c8b6 0f84f0000000 je 0x14142c9ac
0x14142c8bc 488b08 mov rcx, qword ptr [rax]
0x14142c8bf 4883f901 cmp rcx, 1
0x14142c8c3 48894510 mov qword ptr [rbp + 0x10], rax
0x14142c8c7 0f84bf000000 je 0x14142c98c
0x14142c8cd 4885c9 test rcx, rcx
0x14142c8d0 0f85c2000000 jne 0x14142c998
0x14142c8d6 488b4510 mov rax, qword ptr [rbp + 0x10]
0x14142c8da 488b5010 mov rdx, qword ptr [rax + 0x10]
0x14142c8de 4885d2 test rdx, rdx
0x14142c8e1 0f84b1000000 je 0x14142c998
0x14142c8e7 488b4510 mov rax, qword ptr [rbp + 0x10]
0x14142c8eb 488b4808 mov rcx, qword ptr [rax + 8]
0x14142c8ef 41b801000000 mov r8d, 1
0x14142c8f5 e826731eff call 0x140613c20
0x14142c8fa e999000000 jmp 0x14142c998
0x14142c8ff f6451001 test byte ptr [rbp + 0x10], 1
0x14142c903 0f84c3000000 je 0x14142c9cc
0x14142c909 f6454001 test byte ptr [rbp + 0x40], 1
0x14142c90d 0f84c7000000 je 0x14142c9da
0x14142c913 f645e401 test byte ptr [rbp - 0x1c], 1
0x14142c917 0f84e0000000 je 0x14142c9fd
0x14142c91d f645e801 test byte ptr [rbp - 0x18], 1
0x14142c921 0f840f010000 je 0x14142ca36
0x14142c927 f645ec01 test byte ptr [rbp - 0x14], 1
0x14142c92b 0f8413010000 je 0x14142ca44
0x14142c931 31db xor ebx, ebx
0x14142c933 b001 mov al, 1
0x14142c935 894540 mov dword ptr [rbp + 0x40], eax
0x14142c938 4c8b7da0 mov r15, qword ptr [rbp - 0x60]
0x14142c93c 8b45d4 mov eax, dword ptr [rbp - 0x2c]
0x14142c93f 8b4dd8 mov ecx, dword ptr [rbp - 0x28]
0x14142c942 488b55a8 mov rdx, qword ptr [rbp - 0x58]
0x14142c946 448b45dc mov r8d, dword ptr [rbp - 0x24]
0x14142c94a e915010000 jmp 0x14142ca64
0x14142c94f 41be01000000 mov r14d, 1
0x14142c955 4885c0 test rax, rax
0x14142c958 0f85c2010000 jne 0x14142cb20
0x14142c95e 488b4500 mov rax, qword ptr [rbp]
0x14142c962 488945c0 mov qword ptr [rbp - 0x40], rax
0x14142c966 0f2845f0 movaps xmm0, xmmword ptr [rbp - 0x10]
0x14142c96a 0f2945b0 movaps xmmword ptr [rbp - 0x50], xmm0
0x14142c96e 4531f6 xor r14d, r14d
0x14142c971 eb39 jmp 0x14142c9ac
0x14142c973 48c7451805000000 mov qword ptr [rbp + 0x18], 5
0x14142c97b 488d5518 lea rdx, [rbp + 0x18]
0x14142c97f 4889f9 mov rcx, rdi
0x14142c982 e8993f4701 call 0x1428a0920
0x14142c987 e9c9000000 jmp 0x14142ca55
0x14142c98c 4889c1 mov rcx, rax
0x14142c98f 4883c108 add rcx, 8
0x14142c993 e868b01bff call 0x1405e7a00
0x14142c998 ba28000000 mov edx, 0x28
0x14142c99d 41b808000000 mov r8d, 8
0x14142c9a3 488b4d10 mov rcx, qword ptr [rbp + 0x10]
0x14142c9a7 e874721eff call 0x140613c20
0x14142c9ac 4889d8 mov rax, rbx
0x14142c9af e96c010000 jmp 0x14142cb20
0x14142c9b4 4c8d0555fc8201 lea r8, [rip + 0x182fc55]
0x14142c9bb 488d55e3 lea rdx, [rbp - 0x1d]
0x14142c9bf 4889f9 mov rcx, rdi
0x14142c9c2 e839c24c01 call 0x1428f8c00
0x14142c9c7 e990010000 jmp 0x14142cb5c
0x14142c9cc 488d0d75038301 lea rcx, [rip + 0x1830375]
0x14142c9d3 ba08000000 mov edx, 8
0x14142c9d8 eb76 jmp 0x14142ca50
0x14142c9da 488d0d07078301 lea rcx, [rip + 0x1830707]
0x14142c9e1 ba08000000 mov edx, 8
0x14142c9e6 eb68 jmp 0x14142ca50
0x14142c9e8 488b55f8 mov rdx, qword ptr [rbp - 8]
0x14142c9ec bb01000000 mov ebx, 1
0x14142c9f1 c7454000000000 mov dword ptr [rbp + 0x40], 0
0x14142c9f8 4989d7 mov r15, rdx
0x14142c9fb eb67 jmp 0x14142ca64
0x14142c9fd 488d0dec068301 lea rcx, [rip + 0x18306ec]
0x14142ca04 ba07000000 mov edx, 7
0x14142ca09 eb45 jmp 0x14142ca50
0x14142ca0b 488d15c6068301 lea rdx, [rip + 0x18306c6]
0x14142ca12 4c8d05d7f48201 lea r8, [rip + 0x182f4d7]
0x14142ca19 48c7454000000000 mov qword ptr [rbp + 0x40], 0
0x14142ca21 31c9 xor ecx, ecx
0x14142ca23 e8f8d24c01 call 0x1428f9d20
0x14142ca28 4889c3 mov rbx, rax
0x14142ca2b 41be01000000 mov r14d, 1
0x14142ca31 e92efeffff jmp 0x14142c864
0x14142ca36 488d0dba068301 lea rcx, [rip + 0x18306ba]
0x14142ca3d ba0f000000 mov edx, 0xf
0x14142ca42 eb0c jmp 0x14142ca50
0x14142ca44 488d0dbb068301 lea rcx, [rip + 0x18306bb]
0x14142ca4b ba09000000 mov edx, 9
0x14142ca50 e86bd14c01 call 0x1428f9bc0
0x14142ca55 4989c7 mov r15, rax
0x14142ca58 bb01000000 mov ebx, 1
0x14142ca5d c7454000000000 mov dword ptr [rbp + 0x40], 0
0x14142ca64 fe4730 inc byte ptr [rdi + 0x30]
0x14142ca67 895d18 mov dword ptr [rbp + 0x18], ebx
0x14142ca6a 44896d1c mov dword ptr [rbp + 0x1c], r13d
0x14142ca6e 4c897d20 mov qword ptr [rbp + 0x20], r15
0x14142ca72 f30f117528 movss dword ptr [rbp + 0x28], xmm6
0x14142ca77 89452c mov dword ptr [rbp + 0x2c], eax
0x14142ca7a 894d30 mov dword ptr [rbp + 0x30], ecx
0x14142ca7d 48895534 mov qword ptr [rbp + 0x34], rdx
0x14142ca81 4489453c mov dword ptr [rbp + 0x3c], r8d
0x14142ca85 4889f9 mov rcx, rdi
0x14142ca88 e8c3b9c2fe call 0x140058450
0x14142ca8d 488d5528 lea rdx, [rbp + 0x28]
0x14142ca91 488b4a10 mov rcx, qword ptr [rdx + 0x10]
0x14142ca95 48894d00 mov qword ptr [rbp], rcx
0x14142ca99 0f1002 movups xmm0, xmmword ptr [rdx]
0x14142ca9c 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
0x14142caa0 41be01000000 mov r14d, 1
0x14142caa6 85db test ebx, ebx
0x14142caa8 7439 je 0x14142cae3
0x14142caaa 4885c0 test rax, rax
0x14142caad 746e je 0x14142cb1d
0x14142caaf 488b08 mov rcx, qword ptr [rax]
0x14142cab2 4883f901 cmp rcx, 1
0x14142cab6 48894510 mov qword ptr [rbp + 0x10], rax
0x14142caba 7441 je 0x14142cafd
0x14142cabc 4885c9 test rcx, rcx
0x14142cabf 7548 jne 0x14142cb09
0x14142cac1 488b4510 mov rax, qword ptr [rbp + 0x10]
0x14142cac5 488b5010 mov rdx, qword ptr [rax + 0x10]
0x14142cac9 4885d2 test rdx, rdx
0x14142cacc 743b je 0x14142cb09
0x14142cace 488b4510 mov rax, qword ptr [rbp + 0x10]
0x14142cad2 488b4808 mov rcx, qword ptr [rax + 8]
0x14142cad6 41b801000000 mov r8d, 1
0x14142cadc e83f711eff call 0x140613c20
0x14142cae1 eb26 jmp 0x14142cb09
0x14142cae3 4885c0 test rax, rax
0x14142cae6 7538 jne 0x14142cb20
0x14142cae8 488b4500 mov rax, qword ptr [rbp]
0x14142caec 488945c0 mov qword ptr [rbp - 0x40], rax
0x14142caf0 0f2845f0 movaps xmm0, xmmword ptr [rbp - 0x10]
0x14142caf4 0f2945b0 movaps xmmword ptr [rbp - 0x50], xmm0
0x14142caf8 4531f6 xor r14d, r14d
0x14142cafb eb20 jmp 0x14142cb1d
0x14142cafd 4889c1 mov rcx, rax
0x14142cb00 4883c108 add rcx, 8
0x14142cb04 e8f7ae1bff call 0x1405e7a00
0x14142cb09 ba28000000 mov edx, 0x28
0x14142cb0e 41b808000000 mov r8d, 8
0x14142cb14 488b4d10 mov rcx, qword ptr [rbp + 0x10]
0x14142cb18 e803711eff call 0x140613c20
0x14142cb1d 4c89f8 mov rax, r15
0x14142cb20 4585f6 test r14d, r14d
0x14142cb23 7537 jne 0x14142cb5c
0x14142cb25 488b4dc0 mov rcx, qword ptr [rbp - 0x40]
0x14142cb29 48894e20 mov qword ptr [rsi + 0x20], rcx
0x14142cb2d 0f2845b0 movaps xmm0, xmmword ptr [rbp - 0x50]
0x14142cb31 0f114610 movups xmmword ptr [rsi + 0x10], xmm0
0x14142cb35 44896e04 mov dword ptr [rsi + 4], r13d
0x14142cb39 31c9 xor ecx, ecx
0x14142cb3b 48894608 mov qword ptr [rsi + 8], rax
0x14142cb3f 890e mov dword ptr [rsi], ecx
0x14142cb41 4889f0 mov rax, rsi
0x14142cb44 0f287550 movaps xmm6, xmmword ptr [rbp + 0x50]
0x14142cb48 4881c4e8000000 add rsp, 0xe8
0x14142cb4f 5b pop rbx
0x14142cb50 5f pop rdi
0x14142cb51 5e pop rsi
0x14142cb52 415c pop r12
0x14142cb54 415d pop r13
0x14142cb56 415e pop r14
0x14142cb58 415f pop r15
0x14142cb5a 5d pop rbp
0x14142cb5b c3 ret 
0x14142cb5c 4889c1 mov rcx, rax
0x14142cb5f 4889fa mov rdx, rdi
0x14142cb62 e899564701 call 0x1428a2200
0x14142cb67 b901000000 mov ecx, 1
0x14142cb6c ebcd jmp 0x14142cb3b
0x14142cb6e 4c8b7df8 mov r15, qword ptr [rbp - 8]
0x14142cb72 e9e1feffff jmp 0x14142ca58
0x14142cb77 f6451c01 test byte ptr [rbp + 0x1c], 1
0x14142cb7b 0f848d000000 je 0x14142cc0e
0x14142cb81 8b4520 mov eax, dword ptr [rbp + 0x20]
0x14142cb84 4531f6 xor r14d, r14d
0x14142cb87 b101 mov cl, 1
0x14142cb89 48894d40 mov qword ptr [rbp + 0x40], rcx
0x14142cb8d 8b4d10 mov ecx, dword ptr [rbp + 0x10]
0x14142cb90 4c89ea mov rdx, r13
0x14142cb93 e9ccfcffff jmp 0x14142c864
0x14142cb98 488d1539058301 lea rdx, [rip + 0x1830539]
0x14142cb9f 4c8d054af38201 lea r8, [rip + 0x182f34a]
0x14142cba6 b901000000 mov ecx, 1
0x14142cbab eb74 jmp 0x14142cc21
0x14142cbad 488d1524058301 lea rdx, [rip + 0x1830524]
0x14142cbb4 4c8d0535f38201 lea r8, [rip + 0x182f335]
0x14142cbbb b902000000 mov ecx, 2
0x14142cbc0 eb5f jmp 0x14142cc21
0x14142cbc2 bb01000000 mov ebx, 1
0x14142cbc7 c7454000000000 mov dword ptr [rbp + 0x40], 0
0x14142cbce 4989c7 mov r15, rax
0x14142cbd1 e98efeffff jmp 0x14142ca64
0x14142cbd6 410fb6c0 movzx eax, r8b
0x14142cbda 83f87d cmp eax, 0x7d
0x14142cbdd 750d jne 0x14142cbec
0x14142cbdf 48c7451812000000 mov qword ptr [rbp + 0x18], 0x12
0x14142cbe7 e98ffdffff jmp 0x14142c97b
0x14142cbec 48c7451810000000 mov qword ptr [rbp + 0x18], 0x10
0x14142cbf4 e982fdffff jmp 0x14142c97b
0x14142cbf9 488d15d8048301 lea rdx, [rip + 0x18304d8]
0x14142cc00 4c8d05e9f28201 lea r8, [rip + 0x182f2e9]
0x14142cc07 b903000000 mov ecx, 3
0x14142cc0c eb13 jmp 0x14142cc21
0x14142cc0e 488d15c3048301 lea rdx, [rip + 0x18304c3]
0x14142cc15 4c8d05d4f28201 lea r8, [rip + 0x182f2d4]
0x14142cc1c b904000000 mov ecx, 4
0x14142cc21 e8fad04c01 call 0x1428f9d20
0x14142cc26 4889c3 mov rbx, rax
0x14142cc29 e928fcffff jmp 0x14142c856
0x14142cc2e 48c7451815000000 mov qword ptr [rbp + 0x18], 0x15
0x14142cc36 e9f3f6ffff jmp 0x14142c32e
0x14142cc3b 48c7451808000000 mov qword ptr [rbp + 0x18], 8
0x14142cc43 e933fdffff jmp 0x14142c97b
0x14142cc48 48c7451806000000 mov qword ptr [rbp + 0x18], 6
0x14142cc50 e926fdffff jmp 0x14142c97b
0x14142cc55 488d0d9b048301 lea rcx, [rip + 0x183049b]
0x14142cc5c ba0f000000 mov edx, 0xf
0x14142cc61 e84ad14c01 call 0x1428f9db0
0x14142cc66 e9eafdffff jmp 0x14142ca55
0x14142cc6b 488d0d76048301 lea rcx, [rip + 0x1830476]
0x14142cc72 eb33 jmp 0x14142cca7
0x14142cc74 488d0d75048301 lea rcx, [rip + 0x1830475]
0x14142cc7b ba07000000 mov edx, 7
0x14142cc80 e82bd14c01 call 0x1428f9db0
0x14142cc85 e9cbfdffff jmp 0x14142ca55
0x14142cc8a 488d0d75048301 lea rcx, [rip + 0x1830475]
0x14142cc91 ba09000000 mov edx, 9
0x14142cc96 e815d14c01 call 0x1428f9db0
0x14142cc9b e9b5fdffff jmp 0x14142ca55
0x14142cca0 488d0da1008301 lea rcx, [rip + 0x18300a1]
0x14142cca7 ba08000000 mov edx, 8
0x14142ccac e8ffd04c01 call 0x1428f9db0
0x14142ccb1 e99ffdffff jmp 0x14142ca55
0x14142ccb6 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14142ccc0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14142ccc5 55 push rbp
0x14142ccc6 4157 push r15
0x14142ccc8 4156 push r14
0x14142ccca 4155 push r13
0x14142cccc 4154 push r12
0x14142ccce 56 push rsi
0x14142cccf 57 push rdi
0x14142ccd0 53 push rbx
0x14142ccd1 4883ec38 sub rsp, 0x38
0x14142ccd5 488daa80000000 lea rbp, [rdx + 0x80]
0x14142ccdc 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
0x14142cce1 ba28000000 mov edx, 0x28
0x14142cce6 41b808000000 mov r8d, 8
0x14142ccec 488b4d10 mov rcx, qword ptr [rbp + 0x10]
0x14142ccf0 e82b6f1eff call 0x140613c20
0x14142ccf5 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
0x14142ccfa 4883c438 add rsp, 0x38
0x14142ccfe 5b pop rbx
0x14142ccff 5f pop rdi
0x14142cd00 5e pop rsi
0x14142cd01 415c pop r12
0x14142cd03 415d pop r13
0x14142cd05 415e pop r14
0x14142cd07 415f pop r15
0x14142cd09 5d pop rbp
0x14142cd0a c3 ret 
0x14142cd0b 0f1f440000 nop dword ptr [rax + rax]
0x14142cd10 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14142cd15 55 push rbp
0x14142cd16 4157 push r15
0x14142cd18 4156 push r14
0x14142cd1a 4155 push r13
0x14142cd1c 4154 push r12
0x14142cd1e 56 push rsi
0x14142cd1f 57 push rdi
0x14142cd20 53 push rbx
0x14142cd21 4883ec38 sub rsp, 0x38
0x14142cd25 488daa80000000 lea rbp, [rdx + 0x80]
0x14142cd2c 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
0x14142cd31 ba28000000 mov edx, 0x28
0x14142cd36 41b808000000 mov r8d, 8
0x14142cd3c 488b4d10 mov rcx, qword ptr [rbp + 0x10]
0x14142cd40 e8db6e1eff call 0x140613c20
0x14142cd45 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
0x14142cd4a 4883c438 add rsp, 0x38
0x14142cd4e 5b pop rbx
0x14142cd4f 5f pop rdi
0x14142cd50 5e pop rsi
0x14142cd51 415c pop r12
0x14142cd53 415d pop r13
0x14142cd55 415e pop r14
0x14142cd57 415f pop r15
0x14142cd59 5d pop rbp
0x14142cd5a c3 ret 
0x14142cd5b 0f1f440000 nop dword ptr [rax + rax]
0x14142cd60 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14142cd65 55 push rbp
0x14142cd66 4157 push r15
0x14142cd68 4156 push r14
0x14142cd6a 4155 push r13
0x14142cd6c 4154 push r12
0x14142cd6e 56 push rsi
0x14142cd6f 57 push rdi
0x14142cd70 53 push rbx
0x14142cd71 4883ec38 sub rsp, 0x38
0x14142cd75 488daa80000000 lea rbp, [rdx + 0x80]
0x14142cd7c 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
0x14142cd81 488d4d20 lea rcx, [rbp + 0x20]
0x14142cd85 807d4000 cmp byte ptr [rbp + 0x40], 0
0x14142cd89 7505 jne 0x14142cd90
0x14142cd8b e810bb1bff call 0x1405e88a0
0x14142cd90 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
0x14142cd95 4883c438 add rsp, 0x38
0x14142cd99 5b pop rbx
0x14142cd9a 5f pop rdi
0x14142cd9b 5e pop rsi
0x14142cd9c 415c pop r12
0x14142cd9e 415d pop r13
0x14142cda0 415e pop r14
0x14142cda2 415f pop r15
0x14142cda4 5d pop rbp
0x14142cda5 c3 ret 
0x14142cda6 662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x14142cdb0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x14142cdb5 55 push rbp
0x14142cdb6 4157 push r15
0x14142cdb8 4156 push r14
0x14142cdba 4155 push r13
0x14142cdbc 4154 push r12
0x14142cdbe 56 push rsi
0x14142cdbf 57 push rdi
0x14142cdc0 53 push rbx
0x14142cdc1 4883ec38 sub rsp, 0x38
0x14142cdc5 488daa80000000 lea rbp, [rdx + 0x80]
0x14142cdcc 0f29742420 movaps xmmword ptr [rsp + 0x20], xmm6
0x14142cdd1 488d4d20 lea rcx, [rbp + 0x20]
0x14142cdd5 807d4000 cmp byte ptr [rbp + 0x40], 0
0x14142cdd9 7505 jne 0x14142cde0
0x14142cddb e8c0ba1bff call 0x1405e88a0
0x14142cde0 0f28742420 movaps xmm6, xmmword ptr [rsp + 0x20]
0x14142cde5 4883c438 add rsp, 0x38
0x14142cde9 5b pop rbx
0x14142cdea 5f pop rdi
0x14142cdeb 5e pop rsi
0x14142cdec 415c pop r12
0x14142cdee 415d pop r13
0x14142cdf0 415e pop r14
0x14142cdf2 415f pop r15
0x14142cdf4 5d pop rbp
0x14142cdf5 c3 ret 
