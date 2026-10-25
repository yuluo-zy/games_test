0x14179b3e0 4156 push r14
0x14179b3e2 56 push rsi
0x14179b3e3 57 push rdi
0x14179b3e4 53 push rbx
0x14179b3e5 4883ec38 sub rsp, 0x38
0x14179b3e9 4889ce mov rsi, rcx
0x14179b3ec 488b4a20 mov rcx, qword ptr [rdx + 0x20]
0x14179b3f0 488b4228 mov rax, qword ptr [rdx + 0x28]
0x14179b3f4 4839c8 cmp rax, rcx
0x14179b3f7 733d jae 0x14179b436
0x14179b3f9 4c8b4218 mov r8, qword ptr [rdx + 0x18]
0x14179b3fd 48ffc0 inc rax
0x14179b400 48f7d9 neg rcx
0x14179b403 48bf0026000001000000 movabs rdi, 0x100002600
0x14179b40d 0f1f00 nop dword ptr [rax]
0x14179b410 450fb64c00ff movzx r9d, byte ptr [r8 + rax - 1]
0x14179b416 4983f922 cmp r9, 0x22
0x14179b41a 776f ja 0x14179b48b
0x14179b41c 4c0fa3cf bt rdi, r9
0x14179b420 7332 jae 0x14179b454
0x14179b422 48894228 mov qword ptr [rdx + 0x28], rax
0x14179b426 4c8d0c01 lea r9, [rcx + rax]
0x14179b42a 49ffc1 inc r9
0x14179b42d 48ffc0 inc rax
0x14179b430 4983f901 cmp r9, 1
0x14179b434 75da jne 0x14179b410
0x14179b436 48c744242005000000 mov qword ptr [rsp + 0x20], 5
0x14179b43f 488d442420 lea rax, [rsp + 0x20]
0x14179b444 4889d1 mov rcx, rdx
0x14179b447 4889c2 mov rdx, rax
0x14179b44a e8d1541001 call 0x1428a0920
0x14179b44f e9f9000000 jmp 0x14179b54d
0x14179b454 4983f922 cmp r9, 0x22
0x14179b458 7531 jne 0x14179b48b
0x14179b45a 488d4c2420 lea rcx, [rsp + 0x20]
0x14179b45f e8ec18c7ff call 0x14140cd50
0x14179b464 807c242001 cmp byte ptr [rsp + 0x20], 1
0x14179b469 744b je 0x14179b4b6
0x14179b46b 0fb6442421 movzx eax, byte ptr [rsp + 0x21]
0x14179b470 488d0d11fc5301 lea rcx, [rip + 0x153fc11]
0x14179b477 48630481 movsxd rax, dword ptr [rcx + rax*4]
0x14179b47b 4801c8 add rax, rcx
0x14179b47e ffe0 jmp rax
0x14179b480 c6460100 mov byte ptr [rsi + 1], 0
0x14179b484 31c0 xor eax, eax
0x14179b486 e9c8000000 jmp 0x14179b553
0x14179b48b 4183f97b cmp r9d, 0x7b
0x14179b48f 0f853b010000 jne 0x14179b5d0
0x14179b495 fe4a30 dec byte ptr [rdx + 0x30]
0x14179b498 0f849d030000 je 0x14179b83b
0x14179b49e 48894228 mov qword ptr [rdx + 0x28], rax
0x14179b4a2 488d4c2420 lea rcx, [rsp + 0x20]
0x14179b4a7 4989d6 mov r14, rdx
0x14179b4aa e8a118c7ff call 0x14140cd50
0x14179b4af 807c242000 cmp byte ptr [rsp + 0x20], 0
0x14179b4b4 740a je 0x14179b4c0
0x14179b4b6 488b442428 mov rax, qword ptr [rsp + 0x28]
0x14179b4bb e98d000000 jmp 0x14179b54d
0x14179b4c0 4c89f1 mov rcx, r14
0x14179b4c3 498b5620 mov rdx, qword ptr [r14 + 0x20]
0x14179b4c7 498b4628 mov rax, qword ptr [r14 + 0x28]
0x14179b4cb 4839d0 cmp rax, rdx
0x14179b4ce 733d jae 0x14179b50d
0x14179b4d0 0fb65c2421 movzx ebx, byte ptr [rsp + 0x21]
0x14179b4d5 440fb6c3 movzx r8d, bl
0x14179b4d9 4c8b4918 mov r9, qword ptr [rcx + 0x18]
0x14179b4dd 48ffc0 inc rax
0x14179b4e0 48f7da neg rdx
0x14179b4e3 450fb65401ff movzx r10d, byte ptr [r9 + rax - 1]
0x14179b4e9 4983fa3a cmp r10, 0x3a
0x14179b4ed 0f8756030000 ja 0x14179b849
0x14179b4f3 4c0fa3d7 bt rdi, r10
0x14179b4f7 7329 jae 0x14179b522
0x14179b4f9 48894128 mov qword ptr [rcx + 0x28], rax
0x14179b4fd 4c8d1402 lea r10, [rdx + rax]
0x14179b501 49ffc2 inc r10
0x14179b504 48ffc0 inc rax
0x14179b507 4983fa01 cmp r10, 1
0x14179b50b 75d6 jne 0x14179b4e3
0x14179b50d 48c744242003000000 mov qword ptr [rsp + 0x20], 3
0x14179b516 488d542420 lea rdx, [rsp + 0x20]
0x14179b51b e800541001 call 0x1428a0920
0x14179b520 eb2b jmp 0x14179b54d
0x14179b522 4983fa3a cmp r10, 0x3a
0x14179b526 0f851d030000 jne 0x14179b849
0x14179b52c 48894128 mov qword ptr [rcx + 0x28], rax
0x14179b530 4489c0 mov eax, r8d
0x14179b533 488d152efc5301 lea rdx, [rip + 0x153fc2e]
0x14179b53a 48630482 movsxd rax, dword ptr [rdx + rax*4]
0x14179b53e 4801d0 add rax, rdx
0x14179b541 ffe0 jmp rax
0x14179b543 e85824c7ff call 0x14140d9a0
0x14179b548 4885c0 test rax, rax
0x14179b54b 7415 je 0x14179b562
0x14179b54d 48894608 mov qword ptr [rsi + 8], rax
0x14179b551 b001 mov al, 1
0x14179b553 8806 mov byte ptr [rsi], al
0x14179b555 4889f0 mov rax, rsi
0x14179b558 4883c438 add rsp, 0x38
0x14179b55c 5b pop rbx
0x14179b55d 5f pop rdi
0x14179b55e 5e pop rsi
0x14179b55f 415e pop r14
0x14179b561 c3 ret 
0x14179b562 4c89f1 mov rcx, r14
0x14179b565 41fe4630 inc byte ptr [r14 + 0x30]
0x14179b569 498b5620 mov rdx, qword ptr [r14 + 0x20]
0x14179b56d 498b4628 mov rax, qword ptr [r14 + 0x28]
0x14179b571 4839d0 cmp rax, rdx
0x14179b574 7330 jae 0x14179b5a6
0x14179b576 4c8b4118 mov r8, qword ptr [rcx + 0x18]
0x14179b57a 48ffc0 inc rax
0x14179b57d 48f7da neg rdx
0x14179b580 450fb64c00ff movzx r9d, byte ptr [r8 + rax - 1]
0x14179b586 4983f920 cmp r9, 0x20
0x14179b58a 772f ja 0x14179b5bb
0x14179b58c 4c0fa3cf bt rdi, r9
0x14179b590 7329 jae 0x14179b5bb
0x14179b592 48894128 mov qword ptr [rcx + 0x28], rax
0x14179b596 4c8d0c02 lea r9, [rdx + rax]
0x14179b59a 49ffc1 inc r9
0x14179b59d 48ffc0 inc rax
0x14179b5a0 4983f901 cmp r9, 1
0x14179b5a4 75da jne 0x14179b580
0x14179b5a6 48c744242003000000 mov qword ptr [rsp + 0x20], 3
0x14179b5af 488d542420 lea rdx, [rsp + 0x20]
0x14179b5b4 e8e7591001 call 0x1428a0fa0
0x14179b5b9 eb92 jmp 0x14179b54d
0x14179b5bb 4183f97d cmp r9d, 0x7d
0x14179b5bf 0f8592020000 jne 0x14179b857
0x14179b5c5 48894128 mov qword ptr [rcx + 0x28], rax
0x14179b5c9 885e01 mov byte ptr [rsi + 1], bl
0x14179b5cc 31c0 xor eax, eax
0x14179b5ce eb83 jmp 0x14179b553
0x14179b5d0 48c74424200a000000 mov qword ptr [rsp + 0x20], 0xa
0x14179b5d9 e961feffff jmp 0x14179b43f
0x14179b5de c6460132 mov byte ptr [rsi + 1], 0x32
0x14179b5e2 31c0 xor eax, eax
0x14179b5e4 e96affffff jmp 0x14179b553
0x14179b5e9 c646012b mov byte ptr [rsi + 1], 0x2b
0x14179b5ed 31c0 xor eax, eax
0x14179b5ef e95fffffff jmp 0x14179b553
0x14179b5f4 c6460125 mov byte ptr [rsi + 1], 0x25
0x14179b5f8 31c0 xor eax, eax
0x14179b5fa e954ffffff jmp 0x14179b553
0x14179b5ff c6460135 mov byte ptr [rsi + 1], 0x35
0x14179b603 31c0 xor eax, eax
0x14179b605 e949ffffff jmp 0x14179b553
0x14179b60a c646012f mov byte ptr [rsi + 1], 0x2f
0x14179b60e 31c0 xor eax, eax
0x14179b610 e93effffff jmp 0x14179b553
0x14179b615 c6460123 mov byte ptr [rsi + 1], 0x23
0x14179b619 31c0 xor eax, eax
0x14179b61b e933ffffff jmp 0x14179b553
0x14179b620 c6460124 mov byte ptr [rsi + 1], 0x24
0x14179b624 31c0 xor eax, eax
0x14179b626 e928ffffff jmp 0x14179b553
0x14179b62b c646011b mov byte ptr [rsi + 1], 0x1b
0x14179b62f 31c0 xor eax, eax
0x14179b631 e91dffffff jmp 0x14179b553
0x14179b636 c6460122 mov byte ptr [rsi + 1], 0x22
0x14179b63a 31c0 xor eax, eax
0x14179b63c e912ffffff jmp 0x14179b553
0x14179b641 c6460110 mov byte ptr [rsi + 1], 0x10
0x14179b645 31c0 xor eax, eax
0x14179b647 e907ffffff jmp 0x14179b553
0x14179b64c c646012c mov byte ptr [rsi + 1], 0x2c
0x14179b650 31c0 xor eax, eax
0x14179b652 e9fcfeffff jmp 0x14179b553
0x14179b657 c646010e mov byte ptr [rsi + 1], 0xe
0x14179b65b 31c0 xor eax, eax
0x14179b65d e9f1feffff jmp 0x14179b553
0x14179b662 c6460121 mov byte ptr [rsi + 1], 0x21
0x14179b666 31c0 xor eax, eax
0x14179b668 e9e6feffff jmp 0x14179b553
0x14179b66d c6460130 mov byte ptr [rsi + 1], 0x30
0x14179b671 31c0 xor eax, eax
0x14179b673 e9dbfeffff jmp 0x14179b553
0x14179b678 c646010b mov byte ptr [rsi + 1], 0xb
0x14179b67c 31c0 xor eax, eax
0x14179b67e e9d0feffff jmp 0x14179b553
0x14179b683 c6460131 mov byte ptr [rsi + 1], 0x31
0x14179b687 31c0 xor eax, eax
0x14179b689 e9c5feffff jmp 0x14179b553
0x14179b68e c6460104 mov byte ptr [rsi + 1], 4
0x14179b692 31c0 xor eax, eax
0x14179b694 e9bafeffff jmp 0x14179b553
0x14179b699 c6460128 mov byte ptr [rsi + 1], 0x28
0x14179b69d 31c0 xor eax, eax
0x14179b69f e9affeffff jmp 0x14179b553
0x14179b6a4 c646010c mov byte ptr [rsi + 1], 0xc
0x14179b6a8 31c0 xor eax, eax
0x14179b6aa e9a4feffff jmp 0x14179b553
0x14179b6af c6460109 mov byte ptr [rsi + 1], 9
0x14179b6b3 31c0 xor eax, eax
0x14179b6b5 e999feffff jmp 0x14179b553
0x14179b6ba c6460119 mov byte ptr [rsi + 1], 0x19
0x14179b6be 31c0 xor eax, eax
0x14179b6c0 e98efeffff jmp 0x14179b553
0x14179b6c5 c6460118 mov byte ptr [rsi + 1], 0x18
0x14179b6c9 31c0 xor eax, eax
0x14179b6cb e983feffff jmp 0x14179b553
0x14179b6d0 c6460102 mov byte ptr [rsi + 1], 2
0x14179b6d4 31c0 xor eax, eax
0x14179b6d6 e978feffff jmp 0x14179b553
0x14179b6db c646010f mov byte ptr [rsi + 1], 0xf
0x14179b6df 31c0 xor eax, eax
0x14179b6e1 e96dfeffff jmp 0x14179b553
0x14179b6e6 c6460103 mov byte ptr [rsi + 1], 3
0x14179b6ea 31c0 xor eax, eax
0x14179b6ec e962feffff jmp 0x14179b553
0x14179b6f1 c6460107 mov byte ptr [rsi + 1], 7
0x14179b6f5 31c0 xor eax, eax
0x14179b6f7 e957feffff jmp 0x14179b553
0x14179b6fc c6460101 mov byte ptr [rsi + 1], 1
0x14179b700 31c0 xor eax, eax
0x14179b702 e94cfeffff jmp 0x14179b553
0x14179b707 c646011f mov byte ptr [rsi + 1], 0x1f
0x14179b70b 31c0 xor eax, eax
0x14179b70d e941feffff jmp 0x14179b553
0x14179b712 c6460111 mov byte ptr [rsi + 1], 0x11
0x14179b716 31c0 xor eax, eax
0x14179b718 e936feffff jmp 0x14179b553
0x14179b71d c6460129 mov byte ptr [rsi + 1], 0x29
0x14179b721 31c0 xor eax, eax
0x14179b723 e92bfeffff jmp 0x14179b553
0x14179b728 c6460114 mov byte ptr [rsi + 1], 0x14
0x14179b72c 31c0 xor eax, eax
0x14179b72e e920feffff jmp 0x14179b553
0x14179b733 c6460105 mov byte ptr [rsi + 1], 5
0x14179b737 31c0 xor eax, eax
0x14179b739 e915feffff jmp 0x14179b553
0x14179b73e c6460115 mov byte ptr [rsi + 1], 0x15
0x14179b742 31c0 xor eax, eax
0x14179b744 e90afeffff jmp 0x14179b553
0x14179b749 c646010d mov byte ptr [rsi + 1], 0xd
0x14179b74d 31c0 xor eax, eax
0x14179b74f e9fffdffff jmp 0x14179b553
0x14179b754 c6460106 mov byte ptr [rsi + 1], 6
0x14179b758 31c0 xor eax, eax
0x14179b75a e9f4fdffff jmp 0x14179b553
0x14179b75f c6460112 mov byte ptr [rsi + 1], 0x12
0x14179b763 31c0 xor eax, eax
0x14179b765 e9e9fdffff jmp 0x14179b553
0x14179b76a c646010a mov byte ptr [rsi + 1], 0xa
0x14179b76e 31c0 xor eax, eax
0x14179b770 e9defdffff jmp 0x14179b553
0x14179b775 c6460108 mov byte ptr [rsi + 1], 8
0x14179b779 31c0 xor eax, eax
0x14179b77b e9d3fdffff jmp 0x14179b553
0x14179b780 c646011e mov byte ptr [rsi + 1], 0x1e
0x14179b784 31c0 xor eax, eax
0x14179b786 e9c8fdffff jmp 0x14179b553
0x14179b78b c6460120 mov byte ptr [rsi + 1], 0x20
0x14179b78f 31c0 xor eax, eax
0x14179b791 e9bdfdffff jmp 0x14179b553
0x14179b796 c646012d mov byte ptr [rsi + 1], 0x2d
0x14179b79a 31c0 xor eax, eax
0x14179b79c e9b2fdffff jmp 0x14179b553
0x14179b7a1 c6460116 mov byte ptr [rsi + 1], 0x16
0x14179b7a5 31c0 xor eax, eax
0x14179b7a7 e9a7fdffff jmp 0x14179b553
0x14179b7ac c6460127 mov byte ptr [rsi + 1], 0x27
0x14179b7b0 31c0 xor eax, eax
0x14179b7b2 e99cfdffff jmp 0x14179b553
0x14179b7b7 c6460113 mov byte ptr [rsi + 1], 0x13
0x14179b7bb 31c0 xor eax, eax
0x14179b7bd e991fdffff jmp 0x14179b553
0x14179b7c2 c646012a mov byte ptr [rsi + 1], 0x2a
0x14179b7c6 31c0 xor eax, eax
0x14179b7c8 e986fdffff jmp 0x14179b553
0x14179b7cd c6460117 mov byte ptr [rsi + 1], 0x17
0x14179b7d1 31c0 xor eax, eax
0x14179b7d3 e97bfdffff jmp 0x14179b553
0x14179b7d8 c646011c mov byte ptr [rsi + 1], 0x1c
0x14179b7dc 31c0 xor eax, eax
0x14179b7de e970fdffff jmp 0x14179b553
0x14179b7e3 c646011d mov byte ptr [rsi + 1], 0x1d
0x14179b7e7 31c0 xor eax, eax
0x14179b7e9 e965fdffff jmp 0x14179b553
0x14179b7ee c6460126 mov byte ptr [rsi + 1], 0x26
0x14179b7f2 31c0 xor eax, eax
0x14179b7f4 e95afdffff jmp 0x14179b553
0x14179b7f9 c646011a mov byte ptr [rsi + 1], 0x1a
0x14179b7fd 31c0 xor eax, eax
0x14179b7ff e94ffdffff jmp 0x14179b553
0x14179b804 c6460136 mov byte ptr [rsi + 1], 0x36
0x14179b808 31c0 xor eax, eax
0x14179b80a e944fdffff jmp 0x14179b553
0x14179b80f c6460137 mov byte ptr [rsi + 1], 0x37
0x14179b813 31c0 xor eax, eax
0x14179b815 e939fdffff jmp 0x14179b553
0x14179b81a c6460134 mov byte ptr [rsi + 1], 0x34
0x14179b81e 31c0 xor eax, eax
0x14179b820 e92efdffff jmp 0x14179b553
0x14179b825 c646012e mov byte ptr [rsi + 1], 0x2e
0x14179b829 31c0 xor eax, eax
0x14179b82b e923fdffff jmp 0x14179b553
0x14179b830 c6460133 mov byte ptr [rsi + 1], 0x33
0x14179b834 31c0 xor eax, eax
0x14179b836 e918fdffff jmp 0x14179b553
0x14179b83b 48c744242015000000 mov qword ptr [rsp + 0x20], 0x15
0x14179b844 e9f6fbffff jmp 0x14179b43f
0x14179b849 48c744242006000000 mov qword ptr [rsp + 0x20], 6
0x14179b852 e9bffcffff jmp 0x14179b516
0x14179b857 48c74424200a000000 mov qword ptr [rsp + 0x20], 0xa
0x14179b860 e94afdffff jmp 0x14179b5af
