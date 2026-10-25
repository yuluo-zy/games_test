0x142008640 55 push rbp
0x142008641 4157 push r15
0x142008643 4156 push r14
0x142008645 56 push rsi
0x142008646 57 push rdi
0x142008647 53 push rbx
0x142008648 4881eca8000000 sub rsp, 0xa8
0x14200864f 488dac2480000000 lea rbp, [rsp + 0x80]
0x142008657 48c74520feffffff mov qword ptr [rbp + 0x20], 0xfffffffffffffffe
0x14200865f 0f1002 movups xmm0, xmmword ptr [rdx]
0x142008662 0f104a10 movups xmm1, xmmword ptr [rdx + 0x10]
0x142008666 0f294db0 movaps xmmword ptr [rbp - 0x50], xmm1
0x14200866a 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
0x14200866e 8b411c mov eax, dword ptr [rcx + 0x1c]
0x142008671 ffc0 inc eax
0x142008673 83f8ff cmp eax, -1
0x142008676 89451c mov dword ptr [rbp + 0x1c], eax
0x142008679 0f8411010000 je 0x142008790
0x14200867f 4889ce mov rsi, rcx
0x142008682 488b7910 mov rdi, qword ptr [rcx + 0x10]
0x142008686 8b5918 mov ebx, dword ptr [rcx + 0x18]
0x142008689 4839df cmp rdi, rbx
0x14200868c 7667 jbe 0x1420086f5
0x14200868e 4c8b7608 mov r14, qword ptr [rsi + 8]
0x142008692 4c8d3c9b lea r15, [rbx + rbx*4]
0x142008696 438b7cfe20 mov edi, dword ptr [r14 + r15*8 + 0x20]
0x14200869b 83cf01 or edi, 1
0x14200869e 0f1002 movups xmm0, xmmword ptr [rdx]
0x1420086a1 0f104a10 movups xmm1, xmmword ptr [rdx + 0x10]
0x1420086a5 0f294d00 movaps xmmword ptr [rbp], xmm1
0x1420086a9 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
0x1420086ad 89f9 mov ecx, edi
0x1420086af 89da mov edx, ebx
0x1420086b1 e89aab82fe call 0x140833250
0x1420086b6 4b8d04fe lea rax, [r14 + r15*8]
0x1420086ba 0f2845f0 movaps xmm0, xmmword ptr [rbp - 0x10]
0x1420086be 0f284d00 movaps xmm1, xmmword ptr [rbp]
0x1420086c2 0f294dd0 movaps xmmword ptr [rbp - 0x30], xmm1
0x1420086c6 0f2945c0 movaps xmmword ptr [rbp - 0x40], xmm0
0x1420086ca 8b08 mov ecx, dword ptr [rax]
0x1420086cc 894e18 mov dword ptr [rsi + 0x18], ecx
0x1420086cf 0f2845c0 movaps xmm0, xmmword ptr [rbp - 0x40]
0x1420086d3 0f284dd0 movaps xmm1, xmmword ptr [rbp - 0x30]
0x1420086d7 0f114810 movups xmmword ptr [rax + 0x10], xmm1
0x1420086db 0f1100 movups xmmword ptr [rax], xmm0
0x1420086de 897820 mov dword ptr [rax + 0x20], edi
0x1420086e1 8b451c mov eax, dword ptr [rbp + 0x1c]
0x1420086e4 89461c mov dword ptr [rsi + 0x1c], eax
0x1420086e7 89f9 mov ecx, edi
0x1420086e9 89da mov edx, ebx
0x1420086eb e860ab82fe call 0x140833250
0x1420086f0 e98a000000 jmp 0x14200877f
0x1420086f5 0f1002 movups xmm0, xmmword ptr [rdx]
0x1420086f8 0f104a10 movups xmm1, xmmword ptr [rdx + 0x10]
0x1420086fc 0f294d00 movaps xmmword ptr [rbp], xmm1
0x142008700 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
0x142008704 b901000000 mov ecx, 1
0x142008709 89fa mov edx, edi
0x14200870b e840ab82fe call 0x140833250
0x142008710 0f2845f0 movaps xmm0, xmmword ptr [rbp - 0x10]
0x142008714 0f284d00 movaps xmm1, xmmword ptr [rbp]
0x142008718 0f294dd0 movaps xmmword ptr [rbp - 0x30], xmm1
0x14200871c 0f2945c0 movaps xmmword ptr [rbp - 0x40], xmm0
0x142008720 c745e001000000 mov dword ptr [rbp - 0x20], 1
0x142008727 483b3e cmp rdi, qword ptr [rsi]
0x14200872a 750f jne 0x14200873b
0x14200872c 488d15055cfc00 lea rdx, [rip + 0xfc5c05]
0x142008733 4889f1 mov rcx, rsi
0x142008736 e875a355fe call 0x140562ab0
0x14200873b 488b4608 mov rax, qword ptr [rsi + 8]
0x14200873f 488d0cbf lea rcx, [rdi + rdi*4]
0x142008743 0f2845c0 movaps xmm0, xmmword ptr [rbp - 0x40]
0x142008747 0f284dd0 movaps xmm1, xmmword ptr [rbp - 0x30]
0x14200874b 488b55e0 mov rdx, qword ptr [rbp - 0x20]
0x14200874f 488954c820 mov qword ptr [rax + rcx*8 + 0x20], rdx
0x142008754 0f114cc810 movups xmmword ptr [rax + rcx*8 + 0x10], xmm1
0x142008759 0f1104c8 movups xmmword ptr [rax + rcx*8], xmm0
0x14200875d 4889f8 mov rax, rdi
0x142008760 48ffc0 inc rax
0x142008763 48894610 mov qword ptr [rsi + 0x10], rax
0x142008767 8d4701 lea eax, [rdi + 1]
0x14200876a 894618 mov dword ptr [rsi + 0x18], eax
0x14200876d 8b451c mov eax, dword ptr [rbp + 0x1c]
0x142008770 89461c mov dword ptr [rsi + 0x1c], eax
0x142008773 b901000000 mov ecx, 1
0x142008778 89fa mov edx, edi
0x14200877a e8d1aa82fe call 0x140833250
0x14200877f 90 nop
0x142008780 4881c4a8000000 add rsp, 0xa8
0x142008787 5b pop rbx
0x142008788 5f pop rdi
0x142008789 5e pop rsi
0x14200878a 415e pop r14
0x14200878c 415f pop r15
0x14200878e 5d pop rbp
0x14200878f c3 ret
0x142008790 488d0db95bfc00 lea rcx, [rip + 0xfc5bb9]
0x142008797 4c8d05da5bfc00 lea r8, [rip + 0xfc5bda]
0x14200879e ba23000000 mov edx, 0x23
0x1420087a3 e87dff8c00 call 0x1428d8725
0x1420087a8 0f0b ud2
0x1420087aa 660f1f440000 nop word ptr [rax + rax]
0x1420087b0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1420087b5 55 push rbp
0x1420087b6 4157 push r15
0x1420087b8 4156 push r14
0x1420087ba 56 push rsi
0x1420087bb 57 push rdi
0x1420087bc 53 push rbx
0x1420087bd 4883ec28 sub rsp, 0x28
0x1420087c1 488daa80000000 lea rbp, [rdx + 0x80]
0x1420087c8 488d4dc0 lea rcx, [rbp - 0x40]
0x1420087cc e8bf7e5cfe call 0x1405d0690
0x1420087d1 90 nop
0x1420087d2 4883c428 add rsp, 0x28
0x1420087d6 5b pop rbx
0x1420087d7 5f pop rdi
0x1420087d8 5e pop rsi
0x1420087d9 415e pop r14
0x1420087db 415f pop r15
0x1420087dd 5d pop rbp
0x1420087de c3 ret
0x1420087df 90 nop
0x1420087e0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1420087e5 55 push rbp
0x1420087e6 4157 push r15
0x1420087e8 4156 push r14
0x1420087ea 56 push rsi
0x1420087eb 57 push rdi
0x1420087ec 53 push rbx
0x1420087ed 4883ec28 sub rsp, 0x28
0x1420087f1 488daa80000000 lea rbp, [rdx + 0x80]
0x1420087f8 488b4508 mov rax, qword ptr [rbp + 8]
0x1420087fc f048ff08 lock dec qword ptr [rax]
0x142008800 7509 jne 0x14200880b
0x142008802 488d4d08 lea rcx, [rbp + 8]
0x142008806 e8153eb5fe call 0x140b5c620
0x14200880b 90 nop
0x14200880c 4883c428 add rsp, 0x28
0x142008810 5b pop rbx
0x142008811 5f pop rdi
0x142008812 5e pop rsi
0x142008813 415e pop r14
0x142008815 415f pop r15
0x142008817 5d pop rbp
0x142008818 c3 ret
0x142008819 0f1f8000000000 nop dword ptr [rax]
0x142008820 4889542410 mov qword ptr [rsp + 0x10], rdx
0x142008825 55 push rbp
0x142008826 4157 push r15
0x142008828 4156 push r14
0x14200882a 56 push rsi
0x14200882b 57 push rdi
0x14200882c 53 push rbx
0x14200882d 4883ec28 sub rsp, 0x28
0x142008831 488daa80000000 lea rbp, [rdx + 0x80]
0x142008838 488b4508 mov rax, qword ptr [rbp + 8]
0x14200883c f048ff08 lock dec qword ptr [rax]
0x142008840 7509 jne 0x14200884b
0x142008842 488d4d08 lea rcx, [rbp + 8]
0x142008846 e8d53db5fe call 0x140b5c620
0x14200884b 90 nop
0x14200884c 4883c428 add rsp, 0x28
0x142008850 5b pop rbx
0x142008851 5f pop rdi
0x142008852 5e pop rsi
0x142008853 415e pop r14
0x142008855 415f pop r15
0x142008857 5d pop rbp
0x142008858 c3 ret
0x142008859 0f1f8000000000 nop dword ptr [rax]
0x142008860 4889542410 mov qword ptr [rsp + 0x10], rdx
0x142008865 55 push rbp
0x142008866 4157 push r15
0x142008868 4156 push r14
0x14200886a 56 push rsi
0x14200886b 57 push rdi
0x14200886c 53 push rbx
0x14200886d 4883ec28 sub rsp, 0x28
0x142008871 488daa80000000 lea rbp, [rdx + 0x80]
0x142008878 837d1cff cmp dword ptr [rbp + 0x1c], -1
0x14200887c 7513 jne 0x142008891
0x14200887e 488b45b8 mov rax, qword ptr [rbp - 0x48]
0x142008882 f048ff08 lock dec qword ptr [rax]
0x142008886 7509 jne 0x142008891
0x142008888 488d4db8 lea rcx, [rbp - 0x48]
0x14200888c e88f3db5fe call 0x140b5c620
0x142008891 90 nop
0x142008892 4883c428 add rsp, 0x28
0x142008896 5b pop rbx
0x142008897 5f pop rdi
0x142008898 5e pop rsi
0x142008899 415e pop r14
0x14200889b 415f pop r15
0x14200889d 5d pop rbp
0x14200889e c3 ret
