0x140b669c0 55 push rbp
0x140b669c1 56 push rsi
0x140b669c2 57 push rdi
0x140b669c3 53 push rbx
0x140b669c4 4881ec48040000 sub rsp, 0x448
0x140b669cb 488dac2480000000 lea rbp, [rsp + 0x80]
0x140b669d3 48c785c0030000feffffff mov qword ptr [rbp + 0x3c0], 0xfffffffffffffffe
0x140b669de 488995b0030000 mov qword ptr [rbp + 0x3b0], rdx
0x140b669e5 4889ce mov rsi, rcx
0x140b669e8 4883c138 add rcx, 0x38
0x140b669ec c685bf03000001 mov byte ptr [rbp + 0x3bf], 1
0x140b669f3 48ba84a72b9651d21b8e movabs rdx, 0x8e1bd251962ba784
0x140b669fd 49b89b52e6c07423786a movabs r8, 0x6a782374c0e6529b
0x140b66a07 e8e46c57ff call 0x1400dd6f0
0x140b66a0c 4885c0 test rax, rax
0x140b66a0f 0f84f7000000 je 0x140b66b0c
0x140b66a15 48399618020000 cmp qword ptr [rsi + 0x218], rdx
0x140b66a1c 0f86ea000000 jbe 0x140b66b0c
0x140b66a22 488b8610020000 mov rax, qword ptr [rsi + 0x210]
0x140b66a29 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
0x140b66a2d 4885c9 test rcx, rcx
0x140b66a30 0f84d6000000 je 0x140b66b0c
0x140b66a36 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
0x140b66a3d 48f7d1 not rcx
0x140b66a40 488d0c49 lea rcx, [rcx + rcx*2]
0x140b66a44 48c1e105 shl rcx, 5
0x140b66a48 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
0x140b66a4e 0f84b8000000 je 0x140b66b0c
0x140b66a54 4801c8 add rax, rcx
0x140b66a57 488b7810 mov rdi, qword ptr [rax + 0x10]
0x140b66a5b 8b8eb0050000 mov ecx, dword ptr [rsi + 0x5b0]
0x140b66a61 89485c mov dword ptr [rax + 0x5c], ecx
0x140b66a64 488d4da0 lea rcx, [rbp - 0x60]
0x140b66a68 41b8c0010000 mov r8d, 0x1c0
0x140b66a6e 488b95b0030000 mov rdx, qword ptr [rbp + 0x3b0]
0x140b66a75 e886d1db01 call 0x142923c00
0x140b66a7a c685be03000001 mov byte ptr [rbp + 0x3be], 1
0x140b66a81 4889f9 mov rcx, rdi
0x140b66a84 e857a5b300 call 0x1416a0fe0
0x140b66a89 4889c7 mov rdi, rax
0x140b66a8c 488d9d60010000 lea rbx, [rbp + 0x160]
0x140b66a93 488d55a0 lea rdx, [rbp - 0x60]
0x140b66a97 41b8c0010000 mov r8d, 0x1c0
0x140b66a9d 4889d9 mov rcx, rbx
0x140b66aa0 e85bd1db01 call 0x142923c00
0x140b66aa5 c685be03000000 mov byte ptr [rbp + 0x3be], 0
0x140b66aac 488d8d20030000 lea rcx, [rbp + 0x320]
0x140b66ab3 4889da mov rdx, rbx
0x140b66ab6 e815998e00 call 0x1414503d0
0x140b66abb c685be03000000 mov byte ptr [rbp + 0x3be], 0
0x140b66ac2 488d8d90030000 lea rcx, [rbp + 0x390]
0x140b66ac9 4c8d8520030000 lea r8, [rbp + 0x320]
0x140b66ad0 4889fa mov rdx, rdi
0x140b66ad3 4531c9 xor r9d, r9d
0x140b66ad6 e85582b300 call 0x14169ed30
0x140b66adb 488b9590030000 mov rdx, qword ptr [rbp + 0x390]
0x140b66ae2 4885d2 test rdx, rdx
0x140b66ae5 7416 je 0x140b66afd
0x140b66ae7 488b8d98030000 mov rcx, qword ptr [rbp + 0x398]
0x140b66aee 48c1e204 shl rdx, 4
0x140b66af2 41b808000000 mov r8d, 8
0x140b66af8 e823d1aaff call 0x140613c20
0x140b66afd 4889f0 mov rax, rsi
0x140b66b00 4881c448040000 add rsp, 0x448
0x140b66b07 5b pop rbx
0x140b66b08 5f pop rdi
0x140b66b09 5e pop rsi
0x140b66b0a 5d pop rbp
0x140b66b0b c3 ret 
0x140b66b0c 488d05adc1f801 lea rax, [rip + 0x1f8c1ad]
0x140b66b13 488945a0 mov qword ptr [rbp - 0x60], rax
0x140b66b17 48c745a827000000 mov qword ptr [rbp - 0x58], 0x27
0x140b66b1f 488d45a0 lea rax, [rbp - 0x60]
0x140b66b23 48898520030000 mov qword ptr [rbp + 0x320], rax
0x140b66b2a 488d055f335aff lea rax, [rip - 0xa5cca1]
0x140b66b31 48898528030000 mov qword ptr [rbp + 0x328], rax
0x140b66b38 488d05b1c2f801 lea rax, [rip + 0x1f8c2b1]
0x140b66b3f 48898560010000 mov qword ptr [rbp + 0x160], rax
0x140b66b46 48c7856801000002000000 mov qword ptr [rbp + 0x168], 2
0x140b66b51 48c7858001000000000000 mov qword ptr [rbp + 0x180], 0
0x140b66b5c 488d8520030000 lea rax, [rbp + 0x320]
0x140b66b63 48898570010000 mov qword ptr [rbp + 0x170], rax
0x140b66b6a 48c7857801000001000000 mov qword ptr [rbp + 0x178], 1
0x140b66b75 c685bf03000001 mov byte ptr [rbp + 0x3bf], 1
0x140b66b7c 488d1525c1f801 lea rdx, [rip + 0x1f8c125]
0x140b66b83 488d8d60010000 lea rcx, [rbp + 0x160]
0x140b66b8a e80128d701 call 0x1428d9390
0x140b66b8f 0f0b ud2 
0x140b66b91 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140b66ba0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140b66ba5 55 push rbp
0x140b66ba6 56 push rsi
0x140b66ba7 57 push rdi
0x140b66ba8 53 push rbx
0x140b66ba9 4883ec28 sub rsp, 0x28
0x140b66bad 488daa80000000 lea rbp, [rdx + 0x80]
0x140b66bb4 80bdbe03000000 cmp byte ptr [rbp + 0x3be], 0
0x140b66bbb 7410 je 0x140b66bcd
0x140b66bbd c685bf03000000 mov byte ptr [rbp + 0x3bf], 0
0x140b66bc4 488d4da0 lea rcx, [rbp - 0x60]
0x140b66bc8 e863e6ffff call 0x140b65230
0x140b66bcd c685bf03000000 mov byte ptr [rbp + 0x3bf], 0
0x140b66bd4 4883c428 add rsp, 0x28
0x140b66bd8 5b pop rbx
0x140b66bd9 5f pop rdi
0x140b66bda 5e pop rsi
0x140b66bdb 5d pop rbp
0x140b66bdc c3 ret 
0x140b66bdd 0f1f00 nop dword ptr [rax]
0x140b66be0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140b66be5 55 push rbp
0x140b66be6 56 push rsi
0x140b66be7 57 push rdi
0x140b66be8 53 push rbx
0x140b66be9 4883ec28 sub rsp, 0x28
0x140b66bed 488daa80000000 lea rbp, [rdx + 0x80]
0x140b66bf4 80bdbf03000000 cmp byte ptr [rbp + 0x3bf], 0
0x140b66bfb 7509 jne 0x140b66c06
0x140b66bfd 4883c428 add rsp, 0x28
0x140b66c01 5b pop rbx
0x140b66c02 5f pop rdi
0x140b66c03 5e pop rsi
0x140b66c04 5d pop rbp
0x140b66c05 c3 ret 
0x140b66c06 488b8db0030000 mov rcx, qword ptr [rbp + 0x3b0]
0x140b66c0d e81ee6ffff call 0x140b65230
0x140b66c12 ebe9 jmp 0x140b66bfd
