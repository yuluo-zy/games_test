140a68430 55 push rbp
140a68431 4157 push r15
140a68433 4156 push r14
140a68435 4155 push r13
140a68437 4154 push r12
140a68439 56 push rsi
140a6843a 57 push rdi
140a6843b 53 push rbx
140a6843c 4881ec08010000 sub rsp, 0x108
140a68443 488dac2480000000 lea rbp, [rsp + 0x80]
140a6844b 48c78580000000feffffff mov qword ptr [rbp + 0x80], 0xfffffffffffffffe
140a68456 4c894538 mov qword ptr [rbp + 0x38], r8
140a6845a 4889d6 mov rsi, rdx
140a6845d 4989ce mov r14, rcx
140a68460 e89be51a00 call 0x140c16a00
140a68465 84c0 test al, al
140a68467 88457f mov byte ptr [rbp + 0x7f], al
140a6846a 745f je 0x140a684cb
140a6846c 4c8d3df7c60602 lea r15, [rip + 0x206c6f7]
140a68473 bf3f000000 mov edi, 0x3f
140a68478 ba3f000000 mov edx, 0x3f
140a6847d 4c89f9 mov rcx, r15
140a68480 e8fbeeffff call 0x140a67380
140a68485 4883f801 cmp rax, 1
140a68489 0f85b1000000 jne 0x140a68540
140a6848f 4989d1 mov r9, rdx
140a68492 4885d2 test rdx, rdx
140a68495 743d je 0x140a684d4
140a68497 4983f93f cmp r9, 0x3f
140a6849b 7335 jae 0x140a684d2
140a6849d 488d05c6c60602 lea rax, [rip + 0x206c6c6]
140a684a4 41803c01bf cmp byte ptr [r9 + rax], 0xbf
140a684a9 7f29 jg 0x140a684d4
140a684ab 488d05c6c70602 lea rax, [rip + 0x206c7c6]
140a684b2 4889442420 mov qword ptr [rsp + 0x20], rax
140a684b7 488d0dacc60602 lea rcx, [rip + 0x206c6ac]
140a684be ba3f000000 mov edx, 0x3f
140a684c3 4531c0 xor r8d, r8d
140a684c6 e87514e701 call 0x1428d9940
140a684cb 31c0 xor eax, eax
140a684cd e9c6010000 jmp 0x140a68698
140a684d2 75d7 jne 0x140a684ab
140a684d4 4c8d3d8fc60602 lea r15, [rip + 0x206c68f]
140a684db 4c89f9 mov rcx, r15
140a684de 4c89ca mov rdx, r9
140a684e1 e89aeeffff call 0x140a67380
140a684e6 4883f801 cmp rax, 1
140a684ea 7554 jne 0x140a68540
140a684ec 4989d0 mov r8, rdx
140a684ef 4983c002 add r8, 2
140a684f3 7439 je 0x140a6852e
140a684f5 4983f83f cmp r8, 0x3f
140a684f9 7331 jae 0x140a6852c
140a684fb 488d0568c60602 lea rax, [rip + 0x206c668]
140a68502 41803c00bf cmp byte ptr [r8 + rax], 0xbf
140a68507 7f25 jg 0x140a6852e
140a68509 488d0580c70602 lea rax, [rip + 0x206c780]
140a68510 4889442420 mov qword ptr [rsp + 0x20], rax
140a68515 488d0d4ec60602 lea rcx, [rip + 0x206c64e]
140a6851c ba3f000000 mov edx, 0x3f
140a68521 41b93f000000 mov r9d, 0x3f
140a68527 e81414e701 call 0x1428d9940
140a6852c 75db jne 0x140a68509
140a6852e bf3d000000 mov edi, 0x3d
140a68533 4829d7 sub rdi, rdx
140a68536 4c8d3d2dc60602 lea r15, [rip + 0x206c62d]
140a6853d 4d01c7 add r15, r8
140a68540 488d0591cb0602 lea rax, [rip + 0x206cb91]
140a68547 4c8d2554cb0602 lea r12, [rip + 0x206cb54]
140a6854e 6690 nop 
140a68550 0fbe48ff movsx ecx, byte ptr [rax - 1]
140a68554 85c9 test ecx, ecx
140a68556 7818 js 0x140a68570
140a68558 48ffc8 dec rax
140a6855b 83f95c cmp ecx, 0x5c
140a6855e 7569 jne 0x140a685c9
140a68560 eb7c jmp 0x140a685de
140a68562 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a68570 0fb650fe movzx edx, byte ptr [rax - 2]
140a68574 80fac0 cmp dl, 0xc0
140a68577 7d25 jge 0x140a6859e
140a68579 440fb640fd movzx r8d, byte ptr [rax - 3]
140a6857e 4180f8c0 cmp r8b, 0xc0
140a68582 7d23 jge 0x140a685a7
140a68584 440fb648fc movzx r9d, byte ptr [rax - 4]
140a68589 4883c0fc add rax, -4
140a6858d 4183e107 and r9d, 7
140a68591 41c1e106 shl r9d, 6
140a68595 4183e03f and r8d, 0x3f
140a68599 4509c8 or r8d, r9d
140a6859c eb11 jmp 0x140a685af
140a6859e 4883c0fe add rax, -2
140a685a2 83e21f and edx, 0x1f
140a685a5 eb12 jmp 0x140a685b9
140a685a7 4883c0fd add rax, -3
140a685ab 4183e00f and r8d, 0xf
140a685af 41c1e006 shl r8d, 6
140a685b3 83e23f and edx, 0x3f
140a685b6 4409c2 or edx, r8d
140a685b9 c1e206 shl edx, 6
140a685bc 80e13f and cl, 0x3f
140a685bf 0fb6c9 movzx ecx, cl
140a685c2 09d1 or ecx, edx
140a685c4 83f95c cmp ecx, 0x5c
140a685c7 7415 je 0x140a685de
140a685c9 83f92f cmp ecx, 0x2f
140a685cc 7410 je 0x140a685de
140a685ce 4c39e0 cmp rax, r12
140a685d1 0f8579ffffff jne 0x140a68550
140a685d7 bb36000000 mov ebx, 0x36
140a685dc eb4f jmp 0x140a6862d
140a685de 4c8d25bdca0602 lea r12, [rip + 0x206cabd]
140a685e5 4c29e0 sub rax, r12
140a685e8 4989c0 mov r8, rax
140a685eb 49ffc0 inc r8
140a685ee 7432 je 0x140a68622
140a685f0 4983f836 cmp r8, 0x36
140a685f4 732a jae 0x140a68620
140a685f6 43803c20bf cmp byte ptr [r8 + r12], 0xbf
140a685fb 7f25 jg 0x140a68622
140a685fd 488d0554c60602 lea rax, [rip + 0x206c654]
140a68604 4889442420 mov qword ptr [rsp + 0x20], rax
140a68609 488d0d92ca0602 lea rcx, [rip + 0x206ca92]
140a68610 ba36000000 mov edx, 0x36
140a68615 41b936000000 mov r9d, 0x36
140a6861b e82013e701 call 0x1428d9940
140a68620 75db jne 0x140a685fd
140a68622 bb35000000 mov ebx, 0x35
140a68627 4829c3 sub rbx, rax
140a6862a 4d01c4 add r12, r8
140a6862d e8fefa1a00 call 0x140c18130
140a68632 4889c1 mov rcx, rax
140a68635 488b00 mov rax, qword ptr [rax]
140a68638 4883f801 cmp rax, 1
140a6863c 0f85b9020000 jne 0x140a688fb
140a68642 4883c108 add rcx, 8
140a68646 48833900 cmp qword ptr [rcx], 0
140a6864a 0f85ef020000 jne 0x140a6893f
140a68650 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
140a68657 48894d70 mov qword ptr [rbp + 0x70], rcx
140a6865b 4883c108 add rcx, 8
140a6865f 48895c2420 mov qword ptr [rsp + 0x20], rbx
140a68664 48c744243000000000 mov qword ptr [rsp + 0x30], 0
140a6866d 48c744242801000000 mov qword ptr [rsp + 0x28], 1
140a68676 4c89fa mov rdx, r15
140a68679 4989f8 mov r8, rdi
140a6867c 4d89e1 mov r9, r12
140a6867f e8bce61a00 call 0x140c16d40
140a68684 488b4d70 mov rcx, qword ptr [rbp + 0x70]
140a68688 48ff01 inc qword ptr [rcx]
140a6868b 48894560 mov qword ptr [rbp + 0x60], rax
140a6868f 48894520 mov qword ptr [rbp + 0x20], rax
140a68693 b801000000 mov eax, 1
140a68698 48894518 mov qword ptr [rbp + 0x18], rax
140a6869c 498b0e mov rcx, qword ptr [r14]
140a6869f 4d8b6e08 mov r13, qword ptr [r14 + 8]
140a686a3 488bb9f8000000 mov rdi, qword ptr [rcx + 0xf8]
140a686aa 488b8100010000 mov rax, qword ptr [rcx + 0x100]
140a686b1 4c8d2487 lea r12, [rdi + rax*4]
140a686b5 48894d58 mov qword ptr [rbp + 0x58], rcx
140a686b9 0fb68108010000 movzx eax, byte ptr [rcx + 0x108]
140a686c0 884570 mov byte ptr [rbp + 0x70], al
140a686c3 8b461c mov eax, dword ptr [rsi + 0x1c]
140a686c6 89456c mov dword ptr [rbp + 0x6c], eax
140a686c9 488b06 mov rax, qword ptr [rsi]
140a686cc 48894540 mov qword ptr [rbp + 0x40], rax
140a686d0 488b4610 mov rax, qword ptr [rsi + 0x10]
140a686d4 48894548 mov qword ptr [rbp + 0x48], rax
140a686d8 b808000000 mov eax, 8
140a686dd 48894550 mov qword ptr [rbp + 0x50], rax
140a686e1 4531ff xor r15d, r15d
140a686e4 4531f6 xor r14d, r14d
140a686e7 31db xor ebx, ebx
140a686e9 807d7000 cmp byte ptr [rbp + 0x70], 0
140a686ed 0f847d000000 je 0x140a68770
140a686f3 4c39f3 cmp rbx, r14
140a686f6 0f8504010000 jne 0x140a68800
140a686fc 0f1f4000 nop dword ptr [rax]
140a68700 4c39e7 cmp rdi, r12
140a68703 0f849e010000 je 0x140a688a7
140a68709 8b0f mov ecx, dword ptr [rdi]
140a6870b 4883c704 add rdi, 4
140a6870f 498b85a8010000 mov rax, qword ptr [r13 + 0x1a8]
140a68716 488d0cc9 lea rcx, [rcx + rcx*8]
140a6871a 4c8b74c810 mov r14, qword ptr [rax + rcx*8 + 0x10]
140a6871f 4d85f6 test r14, r14
140a68722 74dc je 0x140a68700
140a68724 488d04c8 lea rax, [rax + rcx*8]
140a68728 488b4d58 mov rcx, qword ptr [rbp + 0x58]
140a6872c 488b8910010000 mov rcx, qword ptr [rcx + 0x110]
140a68733 48394840 cmp qword ptr [rax + 0x40], rcx
140a68737 0f86c7000000 jbe 0x140a68804
140a6873d 488b5038 mov rdx, qword ptr [rax + 0x38]
140a68741 488b0cca mov rcx, qword ptr [rdx + rcx*8]
140a68745 4885c9 test rcx, rcx
140a68748 0f84b6000000 je 0x140a68804
140a6874e 488b4018 mov rax, qword ptr [rax + 0x18]
140a68752 48f7d1 not rcx
140a68755 488d0c49 lea rcx, [rcx + rcx*2]
140a68759 48c1e104 shl rcx, 4
140a6875d 4c8b7c0810 mov r15, qword ptr [rax + rcx + 0x10]
140a68762 31db xor ebx, ebx
140a68764 89d8 mov eax, ebx
140a68766 e9be000000 jmp 0x140a68829
140a6876b 0f1f440000 nop dword ptr [rax + rax]
140a68770 4c39f3 cmp rbx, r14
140a68773 0f85a1000000 jne 0x140a6881a
140a68779 0f1f8000000000 nop dword ptr [rax]
140a68780 4c39e7 cmp rdi, r12
140a68783 0f841e010000 je 0x140a688a7
140a68789 8b0f mov ecx, dword ptr [rdi]
140a6878b 4883c704 add rdi, 4
140a6878f 498b8500010000 mov rax, qword ptr [r13 + 0x100]
140a68796 488d0c89 lea rcx, [rcx + rcx*4]
140a6879a 48c1e105 shl rcx, 5
140a6879e 4c8b740858 mov r14, qword ptr [rax + rcx + 0x58]
140a687a3 4d85f6 test r14, r14
140a687a6 74d8 je 0x140a68780
140a687a8 4801c8 add rax, rcx
140a687ab 8b8894000000 mov ecx, dword ptr [rax + 0x94]
140a687b1 498b95a8010000 mov rdx, qword ptr [r13 + 0x1a8]
140a687b8 4c8d04c9 lea r8, [rcx + rcx*8]
140a687bc 488b4d58 mov rcx, qword ptr [rbp + 0x58]
140a687c0 488b8910010000 mov rcx, qword ptr [rcx + 0x110]
140a687c7 4a394cc240 cmp qword ptr [rdx + r8*8 + 0x40], rcx
140a687cc 763f jbe 0x140a6880d
140a687ce 4a8d14c2 lea rdx, [rdx + r8*8]
140a687d2 4c8b4238 mov r8, qword ptr [rdx + 0x38]
140a687d6 498b0cc8 mov rcx, qword ptr [r8 + rcx*8]
140a687da 4885c9 test rcx, rcx
140a687dd 742e je 0x140a6880d
140a687df 488b5218 mov rdx, qword ptr [rdx + 0x18]
140a687e3 48f7d1 not rcx
140a687e6 488d0c49 lea rcx, [rcx + rcx*2]
140a687ea 48c1e104 shl rcx, 4
140a687ee 4c8b7c0a10 mov r15, qword ptr [rdx + rcx + 0x10]
140a687f3 eb1b jmp 0x140a68810
140a687f5 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a68800 89d8 mov eax, ebx
140a68802 eb25 jmp 0x140a68829
140a68804 4531ff xor r15d, r15d
140a68807 31db xor ebx, ebx
140a68809 89d8 mov eax, ebx
140a6880b eb1c jmp 0x140a68829
140a6880d 4531ff xor r15d, r15d
140a68810 488b4050 mov rax, qword ptr [rax + 0x50]
140a68814 48894550 mov qword ptr [rbp + 0x50], rax
140a68818 31db xor ebx, ebx
140a6881a 4889d8 mov rax, rbx
140a6881d 48c1e004 shl rax, 4
140a68821 488b4d50 mov rcx, qword ptr [rbp + 0x50]
140a68825 8b440108 mov eax, dword ptr [rcx + rax + 8]
140a68829 498d04c7 lea rax, [r15 + rax*8]
140a6882d 8b4d6c mov ecx, dword ptr [rbp + 0x6c]
140a68830 488b5548 mov rdx, qword ptr [rbp + 0x48]
140a68834 890a mov dword ptr [rdx], ecx
140a68836 488b10 mov rdx, qword ptr [rax]
140a68839 488b4d40 mov rcx, qword ptr [rbp + 0x40]
140a6883d e8be4aecff call 0x14092d300
140a68842 4889c6 mov rsi, rax
140a68845 4885c0 test rax, rax
140a68848 0f84fd000000 je 0x140a6894b
140a6884e 488d4dd0 lea rcx, [rbp - 0x30]
140a68852 4889f2 mov rdx, rsi
140a68855 e8d6f2f9ff call 0x140a07b30
140a6885a 0f1045d0 movups xmm0, xmmword ptr [rbp - 0x30]
140a6885e 0f104de0 movups xmm1, xmmword ptr [rbp - 0x20]
140a68862 0f294d00 movaps xmmword ptr [rbp], xmm1
140a68866 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
140a6886a 488b4538 mov rax, qword ptr [rbp + 0x38]
140a6886e 48894510 mov qword ptr [rbp + 0x10], rax
140a68872 488d4db8 lea rcx, [rbp - 0x48]
140a68876 488d55f0 lea rdx, [rbp - 0x10]
140a6887a 4c8d0517c20602 lea r8, [rip + 0x206c217]
140a68881 e8ca4c8900 call 0x1412fd550
140a68886 48ffc3 inc rbx
140a68889 41b80f000000 mov r8d, 0xf
140a6888f 4889f1 mov rcx, rsi
140a68892 488d1587c80602 lea rdx, [rip + 0x206c887]
140a68899 4c8d4db8 lea r9, [rbp - 0x48]
140a6889d e8eec3fcff call 0x140a34c90
140a688a2 e942feffff jmp 0x140a686e9
140a688a7 807d7f00 cmp byte ptr [rbp + 0x7f], 0
140a688ab 743a je 0x140a688e7
140a688ad e87ef81a00 call 0x140c18130
140a688b2 4889c1 mov rcx, rax
140a688b5 488b00 mov rax, qword ptr [rax]
140a688b8 4883f801 cmp rax, 1
140a688bc 7557 jne 0x140a68915
140a688be 4883c108 add rcx, 8
140a688c2 488b5560 mov rdx, qword ptr [rbp + 0x60]
140a688c6 48833900 cmp qword ptr [rcx], 0
140a688ca 7573 jne 0x140a6893f
140a688cc 48c701ffffffff mov qword ptr [rcx], 0xffffffffffffffff
140a688d3 48894d70 mov qword ptr [rbp + 0x70], rcx
140a688d7 4883c108 add rcx, 8
140a688db e810e81a00 call 0x140c170f0
140a688e0 488b4570 mov rax, qword ptr [rbp + 0x70]
140a688e4 48ff00 inc qword ptr [rax]
140a688e7 4881c408010000 add rsp, 0x108
140a688ee 5b pop rbx
140a688ef 5f pop rdi
140a688f0 5e pop rsi
140a688f1 415c pop r12
140a688f3 415d pop r13
140a688f5 415e pop r14
140a688f7 415f pop r15
140a688f9 5d pop rbp
140a688fa c3 ret 
140a688fb 4885c0 test rax, rax
140a688fe 7533 jne 0x140a68933
140a68900 31d2 xor edx, edx
140a68902 e8c9fee801 call 0x1428f87d0
140a68907 4889c1 mov rcx, rax
140a6890a 4885c0 test rax, rax
140a6890d 0f8533fdffff jne 0x140a68646
140a68913 eb1e jmp 0x140a68933
140a68915 4885c0 test rax, rax
140a68918 488b4560 mov rax, qword ptr [rbp + 0x60]
140a6891c 7515 jne 0x140a68933
140a6891e 4889c6 mov rsi, rax
140a68921 31d2 xor edx, edx
140a68923 e8a8fee801 call 0x1428f87d0
140a68928 4889f2 mov rdx, rsi
140a6892b 4889c1 mov rcx, rax
140a6892e 4885c0 test rax, rax
140a68931 7593 jne 0x140a688c6
140a68933 488d0d66211d02 lea rcx, [rip + 0x21d2166]
140a6893a e891f7e601 call 0x1428d80d0
140a6893f 488d0d2a221d02 lea rcx, [rip + 0x21d222a]
140a68946 e80509e701 call 0x1428d9250
140a6894b 488d0d86c70602 lea rcx, [rip + 0x206c786]
140a68952 4c8d05afc70602 lea r8, [rip + 0x206c7af]
140a68959 ba2c000000 mov edx, 0x2c
140a6895e e8cd09e701 call 0x1428d9330
140a68963 0f0b ud2 
140a68965 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a68970 4889542410 mov qword ptr [rsp + 0x10], rdx
140a68975 55 push rbp
140a68976 4157 push r15
140a68978 4156 push r14
140a6897a 4155 push r13
140a6897c 4154 push r12
140a6897e 56 push rsi
140a6897f 57 push rdi
140a68980 53 push rbx
140a68981 4883ec38 sub rsp, 0x38
140a68985 488daa80000000 lea rbp, [rdx + 0x80]
140a6898c 488b4570 mov rax, qword ptr [rbp + 0x70]
140a68990 48ff00 inc qword ptr [rax]
140a68993 4883c438 add rsp, 0x38
140a68997 5b pop rbx
140a68998 5f pop rdi
140a68999 5e pop rsi
140a6899a 415c pop r12
140a6899c 415d pop r13
140a6899e 415e pop r14
140a689a0 415f pop r15
140a689a2 5d pop rbp
140a689a3 c3 ret 
140a689a4 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a689b0 4889542410 mov qword ptr [rsp + 0x10], rdx
140a689b5 55 push rbp
140a689b6 4157 push r15
140a689b8 4156 push r14
140a689ba 4155 push r13
140a689bc 4154 push r12
140a689be 56 push rsi
140a689bf 57 push rdi
140a689c0 53 push rbx
140a689c1 4883ec38 sub rsp, 0x38
140a689c5 488daa80000000 lea rbp, [rdx + 0x80]
140a689cc 488b4570 mov rax, qword ptr [rbp + 0x70]
140a689d0 48ff00 inc qword ptr [rax]
140a689d3 4883c438 add rsp, 0x38
140a689d7 5b pop rbx
140a689d8 5f pop rdi
140a689d9 5e pop rsi
140a689da 415c pop r12
140a689dc 415d pop r13
140a689de 415e pop r14
140a689e0 415f pop r15
140a689e2 5d pop rbp
140a689e3 c3 ret 
140a689e4 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a689f0 4889542410 mov qword ptr [rsp + 0x10], rdx
140a689f5 55 push rbp
140a689f6 4157 push r15
140a689f8 4156 push r14
140a689fa 4155 push r13
140a689fc 4154 push r12
140a689fe 56 push rsi
140a689ff 57 push rdi
140a68a00 53 push rbx
140a68a01 4883ec38 sub rsp, 0x38
140a68a05 488daa80000000 lea rbp, [rdx + 0x80]
140a68a0c 807d7f00 cmp byte ptr [rbp + 0x7f], 0
140a68a10 7420 je 0x140a68a32
140a68a12 488d4520 lea rax, [rbp + 0x20]
140a68a16 48894530 mov qword ptr [rbp + 0x30], rax
140a68a1a 488d4530 lea rax, [rbp + 0x30]
140a68a1e 48894528 mov qword ptr [rbp + 0x28], rax
140a68a22 488d0dcfc10602 lea rcx, [rip + 0x206c1cf]
140a68a29 488d5528 lea rdx, [rbp + 0x28]
140a68a2d e89e688c00 call 0x14132f2d0
140a68a32 90 nop 
140a68a33 4883c438 add rsp, 0x38
140a68a37 5b pop rbx
140a68a38 5f pop rdi
140a68a39 5e pop rsi
140a68a3a 415c pop r12
140a68a3c 415d pop r13
140a68a3e 415e pop r14
140a68a40 415f pop r15
140a68a42 5d pop rbp
140a68a43 c3 ret 
