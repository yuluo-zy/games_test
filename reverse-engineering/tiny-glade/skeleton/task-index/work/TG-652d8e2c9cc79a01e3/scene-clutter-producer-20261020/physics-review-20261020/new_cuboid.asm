0x1408cfff0 55 push rbp
0x1408cfff1 4156 push r14
0x1408cfff3 56 push rsi
0x1408cfff4 57 push rdi
0x1408cfff5 53 push rbx
0x1408cfff6 4883ec60 sub rsp, 0x60
0x1408cfffa 488d6c2460 lea rbp, [rsp + 0x60]
0x1408cffff 48c745f8feffffff mov qword ptr [rbp - 8], 0xfffffffffffffffe
0x1408d0007 4c89c3 mov rbx, r8
0x1408d000a 4989d6 mov r14, rdx
0x1408d000d 4889ce mov rsi, rcx
0x1408d0010 418b4008 mov eax, dword ptr [r8 + 8]
0x1408d0014 8945e4 mov dword ptr [rbp - 0x1c], eax
0x1408d0017 498b00 mov rax, qword ptr [r8]
0x1408d001a 488945dc mov qword ptr [rbp - 0x24], rax
0x1408d001e 488b02 mov rax, qword ptr [rdx]
0x1408d0021 488945d0 mov qword ptr [rbp - 0x30], rax
0x1408d0025 8b4208 mov eax, dword ptr [rdx + 8]
0x1408d0028 8945d8 mov dword ptr [rbp - 0x28], eax
0x1408d002b 488d4dd0 lea rcx, [rbp - 0x30]
0x1408d002f e80ca5b800 call 0x14145a540
0x1408d0034 4889c7 mov rdi, rax
0x1408d0037 f30f101531580502 movss xmm2, dword ptr [rip + 0x2055831]
0x1408d003f f3410f1006 movss xmm0, dword ptr [r14]
0x1408d0044 f30f59c2 mulss xmm0, xmm2
0x1408d0048 f3410f104e04 movss xmm1, dword ptr [r14 + 4]
0x1408d004e f30f59ca mulss xmm1, xmm2
0x1408d0052 f3410f595608 mulss xmm2, dword ptr [r14 + 8]
0x1408d0058 e823b9de00 call 0x1416bb980
0x1408d005d 4989d6 mov r14, rdx
0x1408d0060 488945f0 mov qword ptr [rbp - 0x10], rax
0x1408d0064 488945c0 mov qword ptr [rbp - 0x40], rax
0x1408d0068 488955c8 mov qword ptr [rbp - 0x38], rdx
0x1408d006c 488d4dd0 lea rcx, [rbp - 0x30]
0x1408d0070 4889da mov rdx, rbx
0x1408d0073 e8b81a3c00 call 0x140c91b30
0x1408d0078 48c70604000000 mov qword ptr [rsi], 4
0x1408d007f 48c7460800000000 mov qword ptr [rsi + 8], 0
0x1408d0087 48c7461004000000 mov qword ptr [rsi + 0x10], 4
0x1408d008f 48c7461800000000 mov qword ptr [rsi + 0x18], 0
0x1408d0097 48c7463000000000 mov qword ptr [rsi + 0x30], 0
0x1408d009f 48c7464000000000 mov qword ptr [rsi + 0x40], 0
0x1408d00a7 488b45f0 mov rax, qword ptr [rbp - 0x10]
0x1408d00ab 48894620 mov qword ptr [rsi + 0x20], rax
0x1408d00af 4c897628 mov qword ptr [rsi + 0x28], r14
0x1408d00b3 0f2845d0 movaps xmm0, xmmword ptr [rbp - 0x30]
0x1408d00b7 0f284de0 movaps xmm1, xmmword ptr [rbp - 0x20]
0x1408d00bb 0f294650 movaps xmmword ptr [rsi + 0x50], xmm0
0x1408d00bf 0f294e60 movaps xmmword ptr [rsi + 0x60], xmm1
0x1408d00c3 48897e70 mov qword ptr [rsi + 0x70], rdi
0x1408d00c7 4889f0 mov rax, rsi
0x1408d00ca 4883c460 add rsp, 0x60
0x1408d00ce 5b pop rbx
0x1408d00cf 5f pop rdi
0x1408d00d0 5e pop rsi
0x1408d00d1 415e pop r14
0x1408d00d3 5d pop rbp
0x1408d00d4 c3 ret 
0x1408d00d5 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1408d00e0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1408d00e5 55 push rbp
0x1408d00e6 4156 push r14
0x1408d00e8 56 push rsi
0x1408d00e9 57 push rdi
0x1408d00ea 53 push rbx
0x1408d00eb 4883ec20 sub rsp, 0x20
0x1408d00ef 488d6a60 lea rbp, [rdx + 0x60]
0x1408d00f3 488b45f0 mov rax, qword ptr [rbp - 0x10]
0x1408d00f7 f048ff08 lock dec qword ptr [rax]
0x1408d00fb 7509 jne 0x1408d0106
0x1408d00fd 488d4dc0 lea rcx, [rbp - 0x40]
0x1408d0101 e8caf8d7ff call 0x14064f9d0
0x1408d0106 90 nop 
0x1408d0107 4883c420 add rsp, 0x20
0x1408d010b 5b pop rbx
0x1408d010c 5f pop rdi
0x1408d010d 5e pop rsi
0x1408d010e 415e pop r14
0x1408d0110 5d pop rbp
0x1408d0111 c3 ret 
