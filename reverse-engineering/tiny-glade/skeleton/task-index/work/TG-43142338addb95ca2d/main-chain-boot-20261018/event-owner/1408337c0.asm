1408337c0 55 push rbp
1408337c1 4157 push r15
1408337c3 4156 push r14
1408337c5 4155 push r13
1408337c7 4154 push r12
1408337c9 56 push rsi
1408337ca 57 push rdi
1408337cb 53 push rbx
1408337cc 4881ece8000000 sub rsp, 0xe8
1408337d3 488dac2480000000 lea rbp, [rsp + 0x80]
1408337db 48c74560feffffff mov qword ptr [rbp + 0x60], 0xfffffffffffffffe
1408337e3 8b411c mov eax, dword ptr [rcx + 0x1c]
1408337e6 4c8b4110 mov r8, qword ptr [rcx + 0x10]
1408337ea 418900 mov dword ptr [r8], eax
1408337ed 488b01 mov rax, qword ptr [rcx]
1408337f0 488d4808 lea rcx, [rax + 8]
1408337f4 4c8b4810 mov r9, qword ptr [rax + 0x10]
1408337f8 4c8b4018 mov r8, qword ptr [rax + 0x18]
1408337fc 48c7401800000000 mov qword ptr [rax + 0x18], 0
140833804 4c89c0 mov rax, r8
140833807 48c1e007 shl rax, 7
14083380b 4a8d1cc0 lea rbx, [rax + r8*8]
14083380f 4c01cb add rbx, r9
140833812 4c894538 mov qword ptr [rbp + 0x38], r8
140833816 48c7454000000000 mov qword ptr [rbp + 0x40], 0
14083381e 48895d28 mov qword ptr [rbp + 0x28], rbx
140833822 48894d30 mov qword ptr [rbp + 0x30], rcx
140833826 4d85c0 test r8, r8
140833829 0f8439010000 je 0x140833968
14083382f 8b421c mov eax, dword ptr [rdx + 0x1c]
140833832 89455c mov dword ptr [rbp + 0x5c], eax
140833835 4c8b3a mov r15, qword ptr [rdx]
140833838 4c8b6210 mov r12, qword ptr [rdx + 0x10]
14083383c 498d4720 lea rax, [r15 + 0x20]
140833840 48894548 mov qword ptr [rbp + 0x48], rax
140833844 4981c188000000 add r9, 0x88
14083384b e986000000 jmp 0x1408338d6
140833850 498b4728 mov rax, qword ptr [r15 + 0x28]
140833854 488d0cf6 lea rcx, [rsi + rsi*8]
140833858 48c1e104 shl rcx, 4
14083385c 48893c08 mov qword ptr [rax + rcx], rdi
140833860 0f2845a0 movaps xmm0, xmmword ptr [rbp - 0x60]
140833864 0f284db0 movaps xmm1, xmmword ptr [rbp - 0x50]
140833868 0f2855c0 movaps xmm2, xmmword ptr [rbp - 0x40]
14083386c 0f285dd0 movaps xmm3, xmmword ptr [rbp - 0x30]
140833870 0f11440808 movups xmmword ptr [rax + rcx + 8], xmm0
140833875 0f114c0818 movups xmmword ptr [rax + rcx + 0x18], xmm1
14083387a 0f11540828 movups xmmword ptr [rax + rcx + 0x28], xmm2
14083387f 0f115c0838 movups xmmword ptr [rax + rcx + 0x38], xmm3
140833884 0f2845e0 movaps xmm0, xmmword ptr [rbp - 0x20]
140833888 0f11440848 movups xmmword ptr [rax + rcx + 0x48], xmm0
14083388d 0f2845f0 movaps xmm0, xmmword ptr [rbp - 0x10]
140833891 0f11440858 movups xmmword ptr [rax + rcx + 0x58], xmm0
140833896 0f284500 movaps xmm0, xmmword ptr [rbp]
14083389a 0f11440868 movups xmmword ptr [rax + rcx + 0x68], xmm0
14083389f 0f284510 movaps xmm0, xmmword ptr [rbp + 0x10]
1408338a3 0f11440878 movups xmmword ptr [rax + rcx + 0x78], xmm0
1408338a8 4c89b40888000000 mov qword ptr [rax + rcx + 0x88], r14
1408338b0 48ffc6 inc rsi
1408338b3 49897730 mov qword ptr [r15 + 0x30], rsi
1408338b7 49ff4740 inc qword ptr [r15 + 0x40]
1408338bb 4c8b4d50 mov r9, qword ptr [rbp + 0x50]
1408338bf 4981c188000000 add r9, 0x88
1408338c6 4981c588000000 add r13, 0x88
1408338cd 4939dd cmp r13, rbx
1408338d0 0f8492000000 je 0x140833968
1408338d6 498bb978ffffff mov rdi, qword ptr [r9 - 0x88]
1408338dd 4883ff05 cmp rdi, 5
1408338e1 0f8481000000 je 0x140833968
1408338e7 4c894d50 mov qword ptr [rbp + 0x50], r9
1408338eb 4d8da978ffffff lea r13, [r9 - 0x88]
1408338f2 410f104578 movups xmm0, xmmword ptr [r13 + 0x78]
1408338f7 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
1408338fb 410f104568 movups xmm0, xmmword ptr [r13 + 0x68]
140833900 0f294500 movaps xmmword ptr [rbp], xmm0
140833904 410f104558 movups xmm0, xmmword ptr [r13 + 0x58]
140833909 0f2945f0 movaps xmmword ptr [rbp - 0x10], xmm0
14083390d 410f104548 movups xmm0, xmmword ptr [r13 + 0x48]
140833912 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
140833916 410f104508 movups xmm0, xmmword ptr [r13 + 8]
14083391b 410f104d18 movups xmm1, xmmword ptr [r13 + 0x18]
140833920 410f105528 movups xmm2, xmmword ptr [r13 + 0x28]
140833925 410f105d38 movups xmm3, xmmword ptr [r13 + 0x38]
14083392a 0f295dd0 movaps xmmword ptr [rbp - 0x30], xmm3
14083392e 0f2955c0 movaps xmmword ptr [rbp - 0x40], xmm2
140833932 0f294db0 movaps xmmword ptr [rbp - 0x50], xmm1
140833936 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
14083393a 8b455c mov eax, dword ptr [rbp + 0x5c]
14083393d 41890424 mov dword ptr [r12], eax
140833941 498b7730 mov rsi, qword ptr [r15 + 0x30]
140833945 4d8b7740 mov r14, qword ptr [r15 + 0x40]
140833949 493b7720 cmp rsi, qword ptr [r15 + 0x20]
14083394d 0f85fdfeffff jne 0x140833850
140833953 488b4d48 mov rcx, qword ptr [rbp + 0x48]
140833957 488d15ea674002 lea rdx, [rip + 0x24067ea]
14083395e e82de1dfff call 0x140631a90
140833963 e9e8feffff jmp 0x140833850
140833968 4881c4e8000000 add rsp, 0xe8
14083396f 5b pop rbx
140833970 5f pop rdi
140833971 5e pop rsi
140833972 415c pop r12
140833974 415d pop r13
140833976 415e pop r14
140833978 415f pop r15
14083397a 5d pop rbp
14083397b c3 ret 
14083397c 0f1f4000 nop dword ptr [rax]
140833980 4889542410 mov qword ptr [rsp + 0x10], rdx
140833985 55 push rbp
140833986 4157 push r15
140833988 4156 push r14
14083398a 4155 push r13
14083398c 4154 push r12
14083398e 56 push rsi
14083398f 57 push rdi
140833990 53 push rbx
140833991 4883ec28 sub rsp, 0x28
140833995 488daa80000000 lea rbp, [rdx + 0x80]
14083399c 488b4550 mov rax, qword ptr [rbp + 0x50]
1408339a0 48894520 mov qword ptr [rbp + 0x20], rax
1408339a4 488d4d20 lea rcx, [rbp + 0x20]
1408339a8 e803540400 call 0x140878db0
1408339ad 90 nop 
1408339ae 4883c428 add rsp, 0x28
1408339b2 5b pop rbx
1408339b3 5f pop rdi
1408339b4 5e pop rsi
1408339b5 415c pop r12
1408339b7 415d pop r13
1408339b9 415e pop r14
1408339bb 415f pop r15
1408339bd 5d pop rbp
1408339be c3 ret 
