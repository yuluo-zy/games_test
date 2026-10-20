14090e6f0: 55 push rbp
14090e6f1: 4157 push r15
14090e6f3: 4156 push r14
14090e6f5: 56 push rsi
14090e6f6: 57 push rdi
14090e6f7: 53 push rbx
14090e6f8: 4881ec18010000 sub rsp, 0x118
14090e6ff: 488dac2480000000 lea rbp, [rsp + 0x80]
14090e707: 0f29bd80000000 movaps xmmword ptr [rbp + 0x80], xmm7
14090e70e: 0f297570 movaps xmmword ptr [rbp + 0x70], xmm6
14090e712: 48c74568feffffff mov qword ptr [rbp + 0x68], 0xfffffffffffffffe
14090e71a: 4989cf mov r15, rcx
14090e71d: 48c745e000000000 mov qword ptr [rbp - 0x20], 0
14090e725: 48c745e808000000 mov qword ptr [rbp - 0x18], 8
14090e72d: 48c745f000000000 mov qword ptr [rbp - 0x10], 0
14090e735: f20f10b138010000 movsd xmm6, qword ptr [rcx + 0x138]
14090e73d: f30f10b940010000 movss xmm7, dword ptr [rcx + 0x140]
14090e745: 0fb605bd64d202 movzx eax, byte ptr [rip + 0x2d264bd]
14090e74c: b90c000000 mov ecx, 0xc
14090e751: ba04000000 mov edx, 4
14090e756: e8b554d0ff call 0x140613c10
14090e75b: 4885c0 test rax, rax
14090e75e: 0f8448020000 je 0x14090e9ac
14090e764: 0f1330 movlps qword ptr [rax], xmm6
14090e767: 48894560 mov qword ptr [rbp + 0x60], rax
14090e76b: f30f117808 movss dword ptr [rax + 8], xmm7
14090e770: 488d1551e71d02 lea rdx, [rip + 0x21de751]
14090e777: 488d4de0 lea rcx, [rbp - 0x20]
14090e77b: e8b03ec5ff call 0x140562630
14090e780: 488b45e8 mov rax, qword ptr [rbp - 0x18]
14090e784: 488b4d60 mov rcx, qword ptr [rbp + 0x60]
14090e788: 488908 mov qword ptr [rax], rcx
14090e78b: 488d0d3ee81d02 lea rcx, [rip + 0x21de83e]
14090e792: 48894808 mov qword ptr [rax + 8], rcx
14090e796: 0f1045e0 movups xmm0, xmmword ptr [rbp - 0x20]
14090e79a: 0f294520 movaps xmmword ptr [rbp + 0x20], xmm0
14090e79e: 48c7453001000000 mov qword ptr [rbp + 0x30], 1
14090e7a6: f2410f10b750010000 movsd xmm6, qword ptr [r15 + 0x150]
14090e7af: 0fb6055364d202 movzx eax, byte ptr [rip + 0x2d26453]
14090e7b6: b908000000 mov ecx, 8
14090e7bb: ba04000000 mov edx, 4
14090e7c0: e84b54d0ff call 0x140613c10
14090e7c5: 4885c0 test rax, rax
14090e7c8: 0f84ef010000 je 0x14090e9bd
14090e7ce: 0fc6f6e1 shufps xmm6, xmm6, 0xe1
14090e7d2: 48894560 mov qword ptr [rbp + 0x60], rax
14090e7d6: 0f1330 movlps qword ptr [rax], xmm6
14090e7d9: 488b7530 mov rsi, qword ptr [rbp + 0x30]
14090e7dd: 483b7520 cmp rsi, qword ptr [rbp + 0x20]
14090e7e1: 7510 jne 0x14090e7f3
14090e7e3: 488d15dee61d02 lea rdx, [rip + 0x21de6de]
14090e7ea: 488d4d20 lea rcx, [rbp + 0x20]
14090e7ee: e83d3ec5ff call 0x140562630
14090e7f3: 488b4528 mov rax, qword ptr [rbp + 0x28]
14090e7f7: 4889f1 mov rcx, rsi
14090e7fa: 48c1e104 shl rcx, 4
14090e7fe: 488b5560 mov rdx, qword ptr [rbp + 0x60]
14090e802: 48891408 mov qword ptr [rax + rcx], rdx
14090e806: 488d1593e81d02 lea rdx, [rip + 0x21de893]
14090e80d: 4889540808 mov qword ptr [rax + rcx + 8], rdx
14090e812: 48ffc6 inc rsi
14090e815: 48897550 mov qword ptr [rbp + 0x50], rsi
14090e819: 0f284520 movaps xmm0, xmmword ptr [rbp + 0x20]
14090e81d: 0f294540 movaps xmmword ptr [rbp + 0x40], xmm0
14090e821: 0fb605e163d202 movzx eax, byte ptr [rip + 0x2d263e1]
14090e828: b940000000 mov ecx, 0x40
14090e82d: ba10000000 mov edx, 0x10
14090e832: e8d953d0ff call 0x140613c10
14090e837: 4885c0 test rax, rax
14090e83a: 0f848e010000 je 0x14090e9ce
14090e840: 0f57c0 xorps xmm0, xmm0
14090e843: 0f2900 movaps xmmword ptr [rax], xmm0
14090e846: c7402000000000 mov dword ptr [rax + 0x20], 0
14090e84d: f20f10055bfe0102 movsd xmm0, qword ptr [rip + 0x201fe5b]
14090e855: f20f114030 movsd qword ptr [rax + 0x30], xmm0
14090e85a: 48894560 mov qword ptr [rbp + 0x60], rax
14090e85e: c740380000803f mov dword ptr [rax + 0x38], 0x3f800000
14090e865: 488b5d50 mov rbx, qword ptr [rbp + 0x50]
14090e869: 483b5d40 cmp rbx, qword ptr [rbp + 0x40]
14090e86d: 7510 jne 0x14090e87f
14090e86f: 488d1552e61d02 lea rdx, [rip + 0x21de652]
14090e876: 488d4d40 lea rcx, [rbp + 0x40]
14090e87a: e8b13dc5ff call 0x140562630
14090e87f: 488b4548 mov rax, qword ptr [rbp + 0x48]
14090e883: 4889d9 mov rcx, rbx
14090e886: 48c1e104 shl rcx, 4
14090e88a: 488b5560 mov rdx, qword ptr [rbp + 0x60]
14090e88e: 48891408 mov qword ptr [rax + rcx], rdx
14090e892: 488d159fe71d02 lea rdx, [rip + 0x21de79f]
14090e899: 4889540808 mov qword ptr [rax + rcx + 8], rdx
14090e89e: 48ffc3 inc rbx
14090e8a1: 48895d50 mov qword ptr [rbp + 0x50], rbx
14090e8a5: 48895df0 mov qword ptr [rbp - 0x10], rbx
14090e8a9: 0f284540 movaps xmm0, xmmword ptr [rbp + 0x40]
14090e8ad: 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
14090e8b1: 0f100588e51d02 movups xmm0, xmmword ptr [rip + 0x21de588]
14090e8b8: 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
14090e8bc: 0f100d89e51d02 movups xmm1, xmmword ptr [rip + 0x21de589]
14090e8c3: 0f114d04 movups xmmword ptr [rbp + 4], xmm1
14090e8c7: 0f114d4c movups xmmword ptr [rbp + 0x4c], xmm1
14090e8cb: 0f294540 movaps xmmword ptr [rbp + 0x40], xmm0
14090e8cf: 744b je 0x14090e91c
14090e8d1: 4c8b75e8 mov r14, qword ptr [rbp - 0x18]
14090e8d5: 48c1e304 shl rbx, 4
14090e8d9: 4c01f3 add rbx, r14
14090e8dc: 488d7520 lea rsi, [rbp + 0x20]
14090e8e0: 488d7d40 lea rdi, [rbp + 0x40]
14090e8e4: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14090e8f0: 498b16 mov rdx, qword ptr [r14]
14090e8f3: 498b4608 mov rax, qword ptr [r14 + 8]
14090e8f7: 0f57db xorps xmm3, xmm3
14090e8fa: 4889f1 mov rcx, rsi
14090e8fd: 4989f8 mov r8, rdi
14090e900: ff5030 call qword ptr [rax + 0x30]
14090e903: 4983c610 add r14, 0x10
14090e907: 0f104520 movups xmm0, xmmword ptr [rbp + 0x20]
14090e90b: 0f104d2c movups xmm1, xmmword ptr [rbp + 0x2c]
14090e90f: 0f114d4c movups xmmword ptr [rbp + 0x4c], xmm1
14090e913: 0f294540 movaps xmmword ptr [rbp + 0x40], xmm0
14090e917: 4939de cmp r14, rbx
14090e91a: 75d4 jne 0x14090e8f0
14090e91c: 488d45f8 lea rax, [rbp - 8]
14090e920: 8b4d4c mov ecx, dword ptr [rbp + 0x4c]
14090e923: 89480c mov dword ptr [rax + 0xc], ecx
14090e926: 488b4d50 mov rcx, qword ptr [rbp + 0x50]
14090e92a: 48894810 mov qword ptr [rax + 0x10], rcx
14090e92e: 8b4d58 mov ecx, dword ptr [rbp + 0x58]
14090e931: 894818 mov dword ptr [rax + 0x18], ecx
14090e934: 0f284540 movaps xmm0, xmmword ptr [rbp + 0x40]
14090e938: 0f1100 movups xmmword ptr [rax], xmm0
14090e93b: 0f2845e0 movaps xmm0, xmmword ptr [rbp - 0x20]
14090e93f: 0f284df0 movaps xmm1, xmmword ptr [rbp - 0x10]
14090e943: 0f285500 movaps xmm2, xmmword ptr [rbp]
14090e947: 0f2945a0 movaps xmmword ptr [rbp - 0x60], xmm0
14090e94b: 0f294db0 movaps xmmword ptr [rbp - 0x50], xmm1
14090e94f: 0f2955c0 movaps xmmword ptr [rbp - 0x40], xmm2
14090e953: 488b4510 mov rax, qword ptr [rbp + 0x10]
14090e957: 488945d0 mov qword ptr [rbp - 0x30], rax
14090e95b: 4981c770020000 add r15, 0x270
14090e962: 4c897d60 mov qword ptr [rbp + 0x60], r15
14090e966: 4c89f9 mov rcx, r15
14090e969: e82215ccff call 0x1405cfe90
14090e96e: 488b45d0 mov rax, qword ptr [rbp - 0x30]
14090e972: 488b4d60 mov rcx, qword ptr [rbp + 0x60]
14090e976: 48894130 mov qword ptr [rcx + 0x30], rax
14090e97a: 0f2845a0 movaps xmm0, xmmword ptr [rbp - 0x60]
14090e97e: 0f284db0 movaps xmm1, xmmword ptr [rbp - 0x50]
14090e982: 0f2855c0 movaps xmm2, xmmword ptr [rbp - 0x40]
14090e986: 0f115120 movups xmmword ptr [rcx + 0x20], xmm2
14090e98a: 0f114910 movups xmmword ptr [rcx + 0x10], xmm1
14090e98e: 0f1101 movups xmmword ptr [rcx], xmm0
14090e991: 0f287570 movaps xmm6, xmmword ptr [rbp + 0x70]
14090e995: 0f28bd80000000 movaps xmm7, xmmword ptr [rbp + 0x80]
14090e99c: 4881c418010000 add rsp, 0x118
14090e9a3: 5b pop rbx
14090e9a4: 5f pop rdi
14090e9a5: 5e pop rsi
14090e9a6: 415e pop r14
14090e9a8: 415f pop r15
14090e9aa: 5d pop rbp
14090e9ab: c3 ret 
14090e9ac: b904000000 mov ecx, 4
14090e9b1: ba0c000000 mov edx, 0xc
14090e9b6: e845a6fc01 call 0x1428d9000
14090e9bb: eb20 jmp 0x14090e9dd
14090e9bd: b904000000 mov ecx, 4
14090e9c2: ba08000000 mov edx, 8
14090e9c7: e834a6fc01 call 0x1428d9000
14090e9cc: eb0f jmp 0x14090e9dd
14090e9ce: b910000000 mov ecx, 0x10
14090e9d3: ba40000000 mov edx, 0x40
14090e9d8: e823a6fc01 call 0x1428d9000
14090e9dd: 0f0b ud2 
14090e9df: 90 nop 
14090e9e0: 4889542410 mov qword ptr [rsp + 0x10], rdx
14090e9e5: 55 push rbp
14090e9e6: 4157 push r15
14090e9e8: 4156 push r14
14090e9ea: 56 push rsi
14090e9eb: 57 push rdi
14090e9ec: 53 push rbx
14090e9ed: 4883ec48 sub rsp, 0x48
14090e9f1: 488daa80000000 lea rbp, [rdx + 0x80]
14090e9f8: 0f297c2420 movaps xmmword ptr [rsp + 0x20], xmm7
14090e9fd: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
14090ea02: 488d4de0 lea rcx, [rbp - 0x20]
14090ea06: e88514ccff call 0x1405cfe90
14090ea0b: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
14090ea10: 0f287c2420 movaps xmm7, xmmword ptr [rsp + 0x20]
14090ea15: 4883c448 add rsp, 0x48
14090ea19: 5b pop rbx
14090ea1a: 5f pop rdi
14090ea1b: 5e pop rsi
14090ea1c: 415e pop r14
14090ea1e: 415f pop r15
14090ea20: 5d pop rbp
14090ea21: c3 ret 
14090ea22: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14090ea30: 4889542410 mov qword ptr [rsp + 0x10], rdx
14090ea35: 55 push rbp
14090ea36: 4157 push r15
14090ea38: 4156 push r14
14090ea3a: 56 push rsi
14090ea3b: 57 push rdi
14090ea3c: 53 push rbx
14090ea3d: 4883ec48 sub rsp, 0x48
14090ea41: 488daa80000000 lea rbp, [rdx + 0x80]
14090ea48: 0f297c2420 movaps xmmword ptr [rsp + 0x20], xmm7
14090ea4d: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
14090ea52: 488d1577e51d02 lea rdx, [rip + 0x21de577]
14090ea59: 488b4d60 mov rcx, qword ptr [rbp + 0x60]
14090ea5d: e89efe71ff call 0x14002e900
14090ea62: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
14090ea67: 0f287c2420 movaps xmm7, xmmword ptr [rsp + 0x20]
14090ea6c: 4883c448 add rsp, 0x48
14090ea70: 5b pop rbx
14090ea71: 5f pop rdi
14090ea72: 5e pop rsi
14090ea73: 415e pop r14
14090ea75: 415f pop r15
14090ea77: 5d pop rbp
14090ea78: c3 ret 
14090ea79: 0f1f8000000000 nop dword ptr [rax]
14090ea80: 4889542410 mov qword ptr [rsp + 0x10], rdx
14090ea85: 55 push rbp
14090ea86: 4157 push r15
14090ea88: 4156 push r14
14090ea8a: 56 push rsi
14090ea8b: 57 push rdi
14090ea8c: 53 push rbx
14090ea8d: 4883ec48 sub rsp, 0x48
14090ea91: 488daa80000000 lea rbp, [rdx + 0x80]
14090ea98: 0f297c2420 movaps xmmword ptr [rsp + 0x20], xmm7
14090ea9d: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
14090eaa2: 488d4d20 lea rcx, [rbp + 0x20]
14090eaa6: e8e513ccff call 0x1405cfe90
14090eaab: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
14090eab0: 0f287c2420 movaps xmm7, xmmword ptr [rsp + 0x20]
14090eab5: 4883c448 add rsp, 0x48
14090eab9: 5b pop rbx
14090eaba: 5f pop rdi
14090eabb: 5e pop rsi
14090eabc: 415e pop r14
14090eabe: 415f pop r15
14090eac0: 5d pop rbp
14090eac1: c3 ret 
14090eac2: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14090ead0: 4889542410 mov qword ptr [rsp + 0x10], rdx
14090ead5: 55 push rbp
14090ead6: 4157 push r15
14090ead8: 4156 push r14
14090eada: 56 push rsi
14090eadb: 57 push rdi
14090eadc: 53 push rbx
14090eadd: 4883ec48 sub rsp, 0x48
14090eae1: 488daa80000000 lea rbp, [rdx + 0x80]
14090eae8: 0f297c2420 movaps xmmword ptr [rsp + 0x20], xmm7
14090eaed: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
14090eaf2: 488d4d40 lea rcx, [rbp + 0x40]
14090eaf6: e89513ccff call 0x1405cfe90
14090eafb: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
14090eb00: 0f287c2420 movaps xmm7, xmmword ptr [rsp + 0x20]
14090eb05: 4883c448 add rsp, 0x48
14090eb09: 5b pop rbx
14090eb0a: 5f pop rdi
14090eb0b: 5e pop rsi
14090eb0c: 415e pop r14
14090eb0e: 415f pop r15
14090eb10: 5d pop rbp
14090eb11: c3 ret 
14090eb12: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14090eb20: 4889542410 mov qword ptr [rsp + 0x10], rdx
14090eb25: 55 push rbp
14090eb26: 4157 push r15
14090eb28: 4156 push r14
14090eb2a: 56 push rsi
14090eb2b: 57 push rdi
14090eb2c: 53 push rbx
14090eb2d: 4883ec48 sub rsp, 0x48
14090eb31: 488daa80000000 lea rbp, [rdx + 0x80]
14090eb38: 0f297c2420 movaps xmmword ptr [rsp + 0x20], xmm7
14090eb3d: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
14090eb42: 488d1557e51d02 lea rdx, [rip + 0x21de557]
14090eb49: 488b4d60 mov rcx, qword ptr [rbp + 0x60]
14090eb4d: e8aefd71ff call 0x14002e900
14090eb52: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
14090eb57: 0f287c2420 movaps xmm7, xmmword ptr [rsp + 0x20]
14090eb5c: 4883c448 add rsp, 0x48
14090eb60: 5b pop rbx
14090eb61: 5f pop rdi
14090eb62: 5e pop rsi
14090eb63: 415e pop r14
14090eb65: 415f pop r15
14090eb67: 5d pop rbp
14090eb68: c3 ret 
14090eb69: 0f1f8000000000 nop dword ptr [rax]
14090eb70: 4889542410 mov qword ptr [rsp + 0x10], rdx
14090eb75: 55 push rbp
14090eb76: 4157 push r15
14090eb78: 4156 push r14
14090eb7a: 56 push rsi
14090eb7b: 57 push rdi
14090eb7c: 53 push rbx
14090eb7d: 4883ec48 sub rsp, 0x48
14090eb81: 488daa80000000 lea rbp, [rdx + 0x80]
14090eb88: 0f297c2420 movaps xmmword ptr [rsp + 0x20], xmm7
14090eb8d: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
14090eb92: 488d159fe41d02 lea rdx, [rip + 0x21de49f]
14090eb99: 488b4d60 mov rcx, qword ptr [rbp + 0x60]
14090eb9d: e85efd71ff call 0x14002e900
14090eba2: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
14090eba7: 0f287c2420 movaps xmm7, xmmword ptr [rsp + 0x20]
14090ebac: 4883c448 add rsp, 0x48
14090ebb0: 5b pop rbx
14090ebb1: 5f pop rdi
14090ebb2: 5e pop rsi
14090ebb3: 415e pop r14
14090ebb5: 415f pop r15
14090ebb7: 5d pop rbp
14090ebb8: c3 ret 
14090ebb9: 0f1f8000000000 nop dword ptr [rax]
14090ebc0: 4889542410 mov qword ptr [rsp + 0x10], rdx
14090ebc5: 55 push rbp
14090ebc6: 4157 push r15
14090ebc8: 4156 push r14
14090ebca: 56 push rsi
14090ebcb: 57 push rdi
14090ebcc: 53 push rbx
14090ebcd: 4883ec48 sub rsp, 0x48
14090ebd1: 488daa80000000 lea rbp, [rdx + 0x80]
14090ebd8: 0f297c2420 movaps xmmword ptr [rsp + 0x20], xmm7
14090ebdd: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
14090ebe2: 488d4de0 lea rcx, [rbp - 0x20]
14090ebe6: e8a512ccff call 0x1405cfe90
14090ebeb: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
14090ebf0: 0f287c2420 movaps xmm7, xmmword ptr [rsp + 0x20]
14090ebf5: 4883c448 add rsp, 0x48
14090ebf9: 5b pop rbx
14090ebfa: 5f pop rdi
14090ebfb: 5e pop rsi
14090ebfc: 415e pop r14
14090ebfe: 415f pop r15
14090ec00: 5d pop rbp
14090ec01: c3 ret 
14090ec02: 66666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
14090ec10: 4889542410 mov qword ptr [rsp + 0x10], rdx
14090ec15: 55 push rbp
14090ec16: 4157 push r15
14090ec18: 4156 push r14
14090ec1a: 56 push rsi
14090ec1b: 57 push rdi
14090ec1c: 53 push rbx
14090ec1d: 4883ec48 sub rsp, 0x48
14090ec21: 488daa80000000 lea rbp, [rdx + 0x80]
14090ec28: 0f297c2420 movaps xmmword ptr [rsp + 0x20], xmm7
14090ec2d: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
14090ec32: 488b45d0 mov rax, qword ptr [rbp - 0x30]
14090ec36: 488b4d60 mov rcx, qword ptr [rbp + 0x60]
14090ec3a: 48894130 mov qword ptr [rcx + 0x30], rax
14090ec3e: 0f2845a0 movaps xmm0, xmmword ptr [rbp - 0x60]
14090ec42: 0f284db0 movaps xmm1, xmmword ptr [rbp - 0x50]
14090ec46: 0f2855c0 movaps xmm2, xmmword ptr [rbp - 0x40]
14090ec4a: 0f115120 movups xmmword ptr [rcx + 0x20], xmm2
14090ec4e: 0f114910 movups xmmword ptr [rcx + 0x10], xmm1
14090ec52: 0f1101 movups xmmword ptr [rcx], xmm0
14090ec55: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
14090ec5a: 0f287c2420 movaps xmm7, xmmword ptr [rsp + 0x20]
14090ec5f: 4883c448 add rsp, 0x48
14090ec63: 5b pop rbx
14090ec64: 5f pop rdi
14090ec65: 5e pop rsi
14090ec66: 415e pop r14
14090ec68: 415f pop r15
14090ec6a: 5d pop rbp
14090ec6b: c3 ret 
