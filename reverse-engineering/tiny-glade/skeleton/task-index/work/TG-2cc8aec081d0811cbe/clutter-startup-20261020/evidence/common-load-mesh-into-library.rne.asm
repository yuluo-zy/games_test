140b1aca0: 55 push rbp
140b1aca1: 4156 push r14
140b1aca3: 56 push rsi
140b1aca4: 57 push rdi
140b1aca5: 53 push rbx
140b1aca6: 4881ec60010000 sub rsp, 0x160
140b1acad: 488dac2480000000 lea rbp, [rsp + 0x80]
140b1acb5: 48c785d8000000feffffff mov qword ptr [rbp + 0xd8], 0xfffffffffffffffe
140b1acc0: 4c89c6 mov rsi, r8
140b1acc3: 4889d7 mov rdi, rdx
140b1acc6: 4889ca mov rdx, rcx
140b1acc9: c685d700000001 mov byte ptr [rbp + 0xd7], 1
140b1acd0: 4c89c9 mov rcx, r9
140b1acd3: 488995c8000000 mov qword ptr [rbp + 0xc8], rdx
140b1acda: e801fad9ff call 0x1408ba6e0
140b1acdf: 488b8dc8000000 mov rcx, qword ptr [rbp + 0xc8]
140b1ace6: 488b4170 mov rax, qword ptr [rcx + 0x70]
140b1acea: 488985a0000000 mov qword ptr [rbp + 0xa0], rax
140b1acf1: 0f104160 movups xmm0, xmmword ptr [rcx + 0x60]
140b1acf5: 0f298590000000 movaps xmmword ptr [rbp + 0x90], xmm0
140b1acfc: 0f104150 movups xmm0, xmmword ptr [rcx + 0x50]
140b1ad00: 0f298580000000 movaps xmmword ptr [rbp + 0x80], xmm0
140b1ad07: 0f104140 movups xmm0, xmmword ptr [rcx + 0x40]
140b1ad0b: 0f294570 movaps xmmword ptr [rbp + 0x70], xmm0
140b1ad0f: 0f1001 movups xmm0, xmmword ptr [rcx]
140b1ad12: 0f104910 movups xmm1, xmmword ptr [rcx + 0x10]
140b1ad16: 0f105120 movups xmm2, xmmword ptr [rcx + 0x20]
140b1ad1a: 0f105930 movups xmm3, xmmword ptr [rcx + 0x30]
140b1ad1e: 0f295d60 movaps xmmword ptr [rbp + 0x60], xmm3
140b1ad22: 0f295550 movaps xmmword ptr [rbp + 0x50], xmm2
140b1ad26: 0f294d40 movaps xmmword ptr [rbp + 0x40], xmm1
140b1ad2a: 0f294530 movaps xmmword ptr [rbp + 0x30], xmm0
140b1ad2e: 48b80000000000000080 movabs rax, 0x8000000000000000
140b1ad38: 488985a8000000 mov qword ptr [rbp + 0xa8], rax
140b1ad3f: 4885f6 test rsi, rsi
140b1ad42: 0f888c000000 js 0x140b1add4
140b1ad48: 7424 je 0x140b1ad6e
140b1ad4a: 0fb605b89eb102 movzx eax, byte ptr [rip + 0x2b19eb8]
140b1ad51: 41be01000000 mov r14d, 1
140b1ad57: ba01000000 mov edx, 1
140b1ad5c: 4889f1 mov rcx, rsi
140b1ad5f: e8ac8eafff call 0x140613c10
140b1ad64: 4885c0 test rax, rax
140b1ad67: 746e je 0x140b1add7
140b1ad69: 4889c3 mov rbx, rax
140b1ad6c: eb05 jmp 0x140b1ad73
140b1ad6e: bb01000000 mov ebx, 1
140b1ad73: 4c8bb530010000 mov r14, qword ptr [rbp + 0x130]
140b1ad7a: 4889d9 mov rcx, rbx
140b1ad7d: 4889fa mov rdx, rdi
140b1ad80: 4989f0 mov r8, rsi
140b1ad83: e8788ee001 call 0x142923c00
140b1ad88: 4889b5a8000000 mov qword ptr [rbp + 0xa8], rsi
140b1ad8f: 48899db0000000 mov qword ptr [rbp + 0xb0], rbx
140b1ad96: 4889b5b8000000 mov qword ptr [rbp + 0xb8], rsi
140b1ad9d: 488d75a0 lea rsi, [rbp - 0x60]
140b1ada1: 488d5530 lea rdx, [rbp + 0x30]
140b1ada5: 41b890000000 mov r8d, 0x90
140b1adab: 4889f1 mov rcx, rsi
140b1adae: e84d8ee001 call 0x142923c00
140b1adb3: c685d700000000 mov byte ptr [rbp + 0xd7], 0
140b1adba: 4c89f1 mov rcx, r14
140b1adbd: 4889f2 mov rdx, rsi
140b1adc0: e89b1ee1ff call 0x14092cc60
140b1adc5: 90 nop 
140b1adc6: 4881c460010000 add rsp, 0x160
140b1adcd: 5b pop rbx
140b1adce: 5f pop rdi
140b1adcf: 5e pop rsi
140b1add0: 415e pop r14
140b1add2: 5d pop rbp
140b1add3: c3 ret 
140b1add4: 4531f6 xor r14d, r14d
140b1add7: 4c8d05da411102 lea r8, [rip + 0x21141da]
140b1adde: 4c89f1 mov rcx, r14
140b1ade1: 4889f2 mov rdx, rsi
140b1ade4: e8fae1db01 call 0x1428d8fe3
140b1ade9: 0f0b ud2 
140b1adeb: 0f1f440000 nop dword ptr [rax + rax]
140b1adf0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140b1adf5: 55 push rbp
140b1adf6: 4156 push r14
140b1adf8: 56 push rsi
140b1adf9: 57 push rdi
140b1adfa: 53 push rbx
140b1adfb: 4883ec20 sub rsp, 0x20
140b1adff: 488daa80000000 lea rbp, [rdx + 0x80]
140b1ae06: 80bdd700000000 cmp byte ptr [rbp + 0xd7], 0
140b1ae0d: 750b jne 0x140b1ae1a
140b1ae0f: 4883c420 add rsp, 0x20
140b1ae13: 5b pop rbx
140b1ae14: 5f pop rdi
140b1ae15: 5e pop rsi
140b1ae16: 415e pop r14
140b1ae18: 5d pop rbp
140b1ae19: c3 ret 
140b1ae1a: 488b8dc8000000 mov rcx, qword ptr [rbp + 0xc8]
140b1ae21: e80addd6ff call 0x140888b30
140b1ae26: ebe7 jmp 0x140b1ae0f
140b1ae28: 0f1f840000000000 nop dword ptr [rax + rax]
140b1ae30: 4889542410 mov qword ptr [rsp + 0x10], rdx
140b1ae35: 55 push rbp
140b1ae36: 4156 push r14
140b1ae38: 56 push rsi
140b1ae39: 57 push rdi
140b1ae3a: 53 push rbx
140b1ae3b: 4883ec20 sub rsp, 0x20
140b1ae3f: 488daa80000000 lea rbp, [rdx + 0x80]
140b1ae46: c685d700000000 mov byte ptr [rbp + 0xd7], 0
140b1ae4d: 488d4d30 lea rcx, [rbp + 0x30]
140b1ae51: e8eaded7ff call 0x140898d40
140b1ae56: c685d700000000 mov byte ptr [rbp + 0xd7], 0
140b1ae5d: 4883c420 add rsp, 0x20
140b1ae61: 5b pop rbx
140b1ae62: 5f pop rdi
140b1ae63: 5e pop rsi
140b1ae64: 415e pop r14
140b1ae66: 5d pop rbp
140b1ae67: c3 ret 
