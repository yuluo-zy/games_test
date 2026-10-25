0x1412f6fd0 55 push rbp
0x1412f6fd1 4156 push r14
0x1412f6fd3 56 push rsi
0x1412f6fd4 57 push rdi
0x1412f6fd5 53 push rbx
0x1412f6fd6 4881ec90000000 sub rsp, 0x90
0x1412f6fdd 488dac2480000000 lea rbp, [rsp + 0x80]
0x1412f6fe5 48c74508feffffff mov qword ptr [rbp + 8], 0xfffffffffffffffe
0x1412f6fed 4989d6 mov r14, rdx
0x1412f6ff0 4889ce mov rsi, rcx
0x1412f6ff3 488b5a18 mov rbx, qword ptr [rdx + 0x18]
0x1412f6ff7 488b4208 mov rax, qword ptr [rdx + 8]
0x1412f6ffb 4829c3 sub rbx, rax
0x1412f6ffe 488955e0 mov qword ptr [rbp - 0x20], rdx
0x1412f7002 742a je 0x1412f702e
0x1412f7004 4889d8 mov rax, rbx
0x1412f7007 48c1e803 shr rax, 3
0x1412f700b 488d3c40 lea rdi, [rax + rax*2]
0x1412f700f 0fb605f3db3302 movzx eax, byte ptr [rip + 0x233dbf3]
0x1412f7016 ba08000000 mov edx, 8
0x1412f701b 4889f9 mov rcx, rdi
0x1412f701e e8edcb31ff call 0x140613c10
0x1412f7023 4885c0 test rax, rax
0x1412f7026 746f je 0x1412f7097
0x1412f7028 48c1eb07 shr rbx, 7
0x1412f702c eb07 jmp 0x1412f7035
0x1412f702e 31db xor ebx, ebx
0x1412f7030 b808000000 mov eax, 8
0x1412f7035 48895de8 mov qword ptr [rbp - 0x18], rbx
0x1412f7039 488945f0 mov qword ptr [rbp - 0x10], rax
0x1412f703d 488d4df8 lea rcx, [rbp - 8]
0x1412f7041 48c745f800000000 mov qword ptr [rbp - 8], 0
0x1412f7049 410f1006 movups xmm0, xmmword ptr [r14]
0x1412f704d 410f104e10 movups xmm1, xmmword ptr [r14 + 0x10]
0x1412f7052 0f294db0 movaps xmmword ptr [rbp - 0x50], xmm1
0x1412f7056 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
0x1412f705a 48894dc8 mov qword ptr [rbp - 0x38], rcx
0x1412f705e 48c745d000000000 mov qword ptr [rbp - 0x30], 0
0x1412f7066 488945d8 mov qword ptr [rbp - 0x28], rax
0x1412f706a 488d4da0 lea rcx, [rbp - 0x60]
0x1412f706e 488d55c8 lea rdx, [rbp - 0x38]
0x1412f7072 e8596d2800 call 0x14157ddd0
0x1412f7077 488b45f8 mov rax, qword ptr [rbp - 8]
0x1412f707b 48894610 mov qword ptr [rsi + 0x10], rax
0x1412f707f 0f1045e8 movups xmm0, xmmword ptr [rbp - 0x18]
0x1412f7083 0f1106 movups xmmword ptr [rsi], xmm0
0x1412f7086 4889f0 mov rax, rsi
0x1412f7089 4881c490000000 add rsp, 0x90
0x1412f7090 5b pop rbx
0x1412f7091 5f pop rdi
0x1412f7092 5e pop rsi
0x1412f7093 415e pop r14
0x1412f7095 5d pop rbp
0x1412f7096 c3 ret
0x1412f7097 c6450701 mov byte ptr [rbp + 7], 1
0x1412f709b 4c8d05d6199401 lea r8, [rip + 0x19419d6]
0x1412f70a2 b908000000 mov ecx, 8
0x1412f70a7 4889fa mov rdx, rdi
0x1412f70aa e8341f5e01 call 0x1428d8fe3
0x1412f70af 0f0b ud2
0x1412f70b1 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1412f70c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1412f70c5 55 push rbp
0x1412f70c6 4156 push r14
0x1412f70c8 56 push rsi
0x1412f70c9 57 push rdi
0x1412f70ca 53 push rbx
0x1412f70cb 4883ec20 sub rsp, 0x20
0x1412f70cf 488daa80000000 lea rbp, [rdx + 0x80]
0x1412f70d6 488d4de8 lea rcx, [rbp - 0x18]
0x1412f70da e811d658ff call 0x1408846f0
0x1412f70df c6450700 mov byte ptr [rbp + 7], 0
0x1412f70e3 4883c420 add rsp, 0x20
0x1412f70e7 5b pop rbx
0x1412f70e8 5f pop rdi
0x1412f70e9 5e pop rsi
0x1412f70ea 415e pop r14
0x1412f70ec 5d pop rbp
0x1412f70ed c3 ret
0x1412f70ee 6690 nop
0x1412f70f0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1412f70f5 55 push rbp
0x1412f70f6 4156 push r14
0x1412f70f8 56 push rsi
0x1412f70f9 57 push rdi
0x1412f70fa 53 push rbx
0x1412f70fb 4883ec20 sub rsp, 0x20
0x1412f70ff 488daa80000000 lea rbp, [rdx + 0x80]
0x1412f7106 807d0700 cmp byte ptr [rbp + 7], 0
0x1412f710a 7409 je 0x1412f7115
0x1412f710c 488b4de0 mov rcx, qword ptr [rbp - 0x20]
0x1412f7110 e87b2d58ff call 0x140879e90
0x1412f7115 90 nop
0x1412f7116 4883c420 add rsp, 0x20
0x1412f711a 5b pop rbx
0x1412f711b 5f pop rdi
0x1412f711c 5e pop rsi
0x1412f711d 415e pop r14
0x1412f711f 5d pop rbp
0x1412f7120 c3 ret
