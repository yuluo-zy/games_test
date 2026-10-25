140976ca0: 4157                         push r15
140976ca2: 4156                         push r14
140976ca4: 4155                         push r13
140976ca6: 4154                         push r12
140976ca8: 56                           push rsi
140976ca9: 57                           push rdi
140976caa: 55                           push rbp
140976cab: 53                           push rbx
140976cac: 4881ec88000000               sub rsp, 0x88
140976cb3: 0f297c2470                   movaps xmmword ptr [rsp + 0x70], xmm7
140976cb8: 0f29742460                   movaps xmmword ptr [rsp + 0x60], xmm6
140976cbd: 4889d7                       mov rdi, rdx
140976cc0: 4889ce                       mov rsi, rcx
140976cc3: 0f2801                       movaps xmm0, xmmword ptr [rcx]
140976cc6: 837a1800                     cmp dword ptr [rdx + 0x18], 0
140976cca: 0f85c9020000                 jne 0x140976f99
140976cd0: 488b5f28                     mov rbx, qword ptr [rdi + 0x28]
140976cd4: 48895c2430                   mov qword ptr [rsp + 0x30], rbx
140976cd9: 4983784800                   cmp qword ptr [r8 + 0x48], 0
140976cde: 0f843f020000                 je 0x140976f23
140976ce4: 498d4850                     lea rcx, [r8 + 0x50]
140976ce8: 488d542430                   lea rdx, [rsp + 0x30]
140976ced: 0f28f0                       movaps xmm6, xmm0
140976cf0: 4d89ce                       mov r14, r9
140976cf3: 4d89c7                       mov r15, r8
140976cf6: e875a0c3ff                   call 0x1405b0d70
140976cfb: 4c89f1                       mov rcx, r14
140976cfe: 0f28c6                       movaps xmm0, xmm6
140976d01: 498b5730                     mov rdx, qword ptr [r15 + 0x30]
140976d05: 4d8b4738                     mov r8, qword ptr [r15 + 0x38]
140976d09: 4d89c1                       mov r9, r8
140976d0c: 4921c1                       and r9, rax
140976d0f: 48c1e839                     shr rax, 0x39
140976d13: 660f6ec8                     movd xmm1, eax
140976d17: 660f60c9                     punpcklbw xmm1, xmm1
140976d1b: f20f70c900                   pshuflw xmm1, xmm1, 0
140976d20: 660f70c900                   pshufd xmm1, xmm1, 0
140976d25: 488d42f0                     lea rax, [rdx - 0x10]
140976d29: 4531d2                       xor r10d, r10d
140976d2c: 660f76d2                     pcmpeqd xmm2, xmm2
140976d30: f3420f6f1c0a                 movdqu xmm3, xmmword ptr [rdx + r9]
140976d36: 660f6fe3                     movdqa xmm4, xmm3
140976d3a: 660f74e1                     pcmpeqb xmm4, xmm1
140976d3e: 66440fd7dc                   pmovmskb r11d, xmm4
140976d43: 4585db                       test r11d, r11d
140976d46: 742b                         je 0x140976d73
140976d48: f3450fbcf3                   tzcnt r14d, r11d
140976d4d: 4d01ce                       add r14, r9
140976d50: 4d21c6                       and r14, r8
140976d53: 4d89f7                       mov r15, r14
140976d56: 49c1e704                     shl r15, 4
140976d5a: 4989c4                       mov r12, rax
140976d5d: 4d29fc                       sub r12, r15
140976d60: 49391c24                     cmp qword ptr [r12], rbx
140976d64: 742f                         je 0x140976d95
140976d66: 418d6bff                     lea ebp, [r11 - 1]
140976d6a: 664421dd                     and bp, r11w
140976d6e: 4189eb                       mov r11d, ebp
140976d71: 75d5                         jne 0x140976d48
140976d73: 660f74da                     pcmpeqb xmm3, xmm2
140976d77: 66440fd7db                   pmovmskb r11d, xmm3
140976d7c: 4585db                       test r11d, r11d
140976d7f: 0f859e010000                 jne 0x140976f23
140976d85: 4d01d1                       add r9, r10
140976d88: 4983c110                     add r9, 0x10
140976d8c: 4983c210                     add r10, 0x10
140976d90: 4d21c1                       and r9, r8
140976d93: eb9b                         jmp 0x140976d30
140976d95: 49f7de                       neg r14
140976d98: 49c1e604                     shl r14, 4
140976d9c: 4a8b4432f8                   mov rax, qword ptr [rdx + r14 - 8]
140976da1: 4989c0                       mov r8, rax
140976da4: 49c1e820                     shr r8, 0x20
140976da8: 0f8475010000                 je 0x140976f23
140976dae: 488b5108                     mov rdx, qword ptr [rcx + 8]
140976db2: 4189c2                       mov r10d, eax
140976db5: 4c3b5210                     cmp r10, qword ptr [rdx + 0x10]
140976db9: 0f83e9030000                 jae 0x1409771a8
140976dbf: 4c8b4a08                     mov r9, qword ptr [rdx + 8]
140976dc3: 4989c0                       mov r8, rax
140976dc6: 49c1e820                     shr r8, 0x20
140976dca: 4f8d1c92                     lea r11, [r10 + r10*4]
140976dce: 47390499                     cmp dword ptr [r9 + r11*4], r8d
140976dd2: 0f85d0030000                 jne 0x1409771a8
140976dd8: 4f8d1c99                     lea r11, [r9 + r11*4]
140976ddc: 458b4b04                     mov r9d, dword ptr [r11 + 4]
140976de0: bbffffffff                   mov ebx, 0xffffffff
140976de5: 4939d9                       cmp r9, rbx
140976de8: 0f84ba030000                 je 0x1409771a8
140976dee: 4c8b01                       mov r8, qword ptr [rcx]
140976df1: 31c9                         xor ecx, ecx
140976df3: 4d394838                     cmp qword ptr [r8 + 0x38], r9
140976df7: 0f86ba030000                 jbe 0x1409771b7
140976dfd: 4d8b5028                     mov r10, qword ptr [r8 + 0x28]
140976e01: 4489cb                       mov ebx, r9d
140976e04: c1eb06                       shr ebx, 6
140976e07: 4d8b14da                     mov r10, qword ptr [r10 + rbx*8]
140976e0b: 4d0fa3ca                     bt r10, r9
140976e0f: 0f83a2030000                 jae 0x1409771b7
140976e15: 418b4b0c                     mov ecx, dword ptr [r11 + 0xc]
140976e19: 418b4310                     mov eax, dword ptr [r11 + 0x10]
140976e1d: 488b92a8010000               mov rdx, qword ptr [rdx + 0x1a8]
140976e24: 4c8d0cc9                     lea r9, [rcx + rcx*8]
140976e28: 498b8810010000               mov rcx, qword ptr [r8 + 0x110]
140976e2f: 4a394cca40                   cmp qword ptr [rdx + r9*8 + 0x40], rcx
140976e34: 7627                         jbe 0x140976e5d
140976e36: 4a8d14ca                     lea rdx, [rdx + r9*8]
140976e3a: 4c8b4238                     mov r8, qword ptr [rdx + 0x38]
140976e3e: 498b0cc8                     mov rcx, qword ptr [r8 + rcx*8]
140976e42: 4885c9                       test rcx, rcx
140976e45: 7416                         je 0x140976e5d
140976e47: 488b5218                     mov rdx, qword ptr [rdx + 0x18]
140976e4b: 48f7d1                       not rcx
140976e4e: 488d0c49                     lea rcx, [rcx + rcx*2]
140976e52: 48c1e104                     shl rcx, 4
140976e56: 488b5c0a10                   mov rbx, qword ptr [rdx + rcx + 0x10]
140976e5b: eb02                         jmp 0x140976e5f
140976e5d: 31db                         xor ebx, ebx
140976e5f: 4c6bf858                     imul r15, rax, 0x58
140976e63: 4e8d343b                     lea r14, [rbx + r15]
140976e67: 0f28d0                       movaps xmm2, xmm0
140976e6a: 0f59d0                       mulps xmm2, xmm0
140976e6d: f30f16ca                     movshdup xmm1, xmm2
140976e71: f30f58ca                     addss xmm1, xmm2
140976e75: 0f28da                       movaps xmm3, xmm2
140976e78: 660f15da                     unpckhpd xmm3, xmm2
140976e7c: f30f58d9                     addss xmm3, xmm1
140976e80: 0fc6db00                     shufps xmm3, xmm3, 0
140976e84: 0fc6d2ff                     shufps xmm2, xmm2, 0xff
140976e88: 0f5cd3                       subps xmm2, xmm3
140976e8b: 0f59154eb51102               mulps xmm2, xmmword ptr [rip + 0x211b54e]
140976e92: 0f280de7210302               movaps xmm1, xmmword ptr [rip + 0x20321e7]
140976e99: 0f59c8                       mulps xmm1, xmm0
140976e9c: f30f16d9                     movshdup xmm3, xmm1
140976ea0: f30f58d9                     addss xmm3, xmm1
140976ea4: 0f12c9                       movhlps xmm1, xmm1
140976ea7: f30f58cb                     addss xmm1, xmm3
140976eab: f30f58c9                     addss xmm1, xmm1
140976eaf: 0fc6c900                     shufps xmm1, xmm1, 0
140976eb3: 0f59c8                       mulps xmm1, xmm0
140976eb6: 0f58ca                       addps xmm1, xmm2
140976eb9: 0f28d0                       movaps xmm2, xmm0
140976ebc: 0fc6d0d6                     shufps xmm2, xmm0, 0xd6
140976ec0: 0f591539620502               mulps xmm2, xmmword ptr [rip + 0x2056239]
140976ec7: 0f28d8                       movaps xmm3, xmm0
140976eca: 0fc6d8c5                     shufps xmm3, xmm0, 0xc5
140976ece: 0f591d1bb51102               mulps xmm3, xmmword ptr [rip + 0x211b51b]
140976ed5: 0f5cda                       subps xmm3, xmm2
140976ed8: 0f58c0                       addps xmm0, xmm0
140976edb: 0fc6c0ff                     shufps xmm0, xmm0, 0xff
140976edf: 0f59c3                       mulps xmm0, xmm3
140976ee2: 0f58c1                       addps xmm0, xmm1
140976ee5: 0f28c8                       movaps xmm1, xmm0
140976ee8: 660f15c8                     unpckhpd xmm1, xmm0
140976eec: 0f2815fde9fa01               movaps xmm2, xmmword ptr [rip + 0x1fae9fd]
140976ef3: 0f57c2                       xorps xmm0, xmm2
140976ef6: 0f57ca                       xorps xmm1, xmm2
140976ef9: e822cffa01                   call 0x142923e20
140976efe: 0f28f0                       movaps xmm6, xmm0
140976f01: 42f6443b0801                 test byte ptr [rbx + r15 + 8], 1
140976f07: 7426                         je 0x140976f2f
140976f09: f3410f104e0c                 movss xmm1, dword ptr [r14 + 0xc]
140976f0f: f3410f104610                 movss xmm0, dword ptr [r14 + 0x10]
140976f15: 0f5705d4e9fa01               xorps xmm0, xmmword ptr [rip + 0x1fae9d4]
140976f1c: e8ffcefa01                   call 0x142923e20
140976f21: eb2f                         jmp 0x140976f52
140976f23: 488d0d26fa1302               lea rcx, [rip + 0x213fa26]
140976f2a: e8e123f601                   call 0x1428d9310
140976f2f: 410f10460c                   movups xmm0, xmmword ptr [r14 + 0xc]
140976f34: 0f29442430                   movaps xmmword ptr [rsp + 0x30], xmm0
140976f39: 488d4c243c                   lea rcx, [rsp + 0x3c]
140976f3e: e8cdc2d700                   call 0x1416f3210
140976f43: 0f28c8                       movaps xmm1, xmm0
140976f46: f30f1005a2a00902             movss xmm0, dword ptr [rip + 0x209a0a2]
140976f4e: f30f5cc1                     subss xmm0, xmm1
140976f52: f30f5cf0                     subss xmm6, xmm0
140976f56: f30f100d16d7fb01             movss xmm1, dword ptr [rip + 0x1fbd716]
140976f5e: 0f28c6                       movaps xmm0, xmm6
140976f61: e8aacffa01                   call 0x142923f10
140976f66: 0f28f0                       movaps xmm6, xmm0
140976f69: f30f5935ffe8fa01             mulss xmm6, dword ptr [rip + 0x1fae8ff]
140976f71: 0f28c6                       movaps xmm0, xmm6
140976f74: e857d0fa01                   call 0x142923fd0
140976f79: 0f28f8                       movaps xmm7, xmm0
140976f7c: 0f28c6                       movaps xmm0, xmm6
140976f7f: e8fccefa01                   call 0x142923e80
140976f84: 0f28c8                       movaps xmm1, xmm0
140976f87: 0f57c0                       xorps xmm0, xmm0
140976f8a: 0f57d2                       xorps xmm2, xmm2
140976f8d: f30f10d1                     movss xmm2, xmm1
140976f91: f30f10c7                     movss xmm0, xmm7
140976f95: 0fc6c211                     shufps xmm0, xmm2, 0x11
140976f99: 4c8bb42410010000             mov r14, qword ptr [rsp + 0x110]
140976fa1: 488b9c2408010000             mov rbx, qword ptr [rsp + 0x108]
140976fa9: 4c8bac2400010000             mov r13, qword ptr [rsp + 0x100]
140976fb1: 4c8bbc24f8000000             mov r15, qword ptr [rsp + 0xf8]
140976fb9: 4c8ba424f0000000             mov r12, qword ptr [rsp + 0xf0]
140976fc1: 4883c610                     add rsi, 0x10
140976fc5: 488d6f18                     lea rbp, [rdi + 0x18]
140976fc9: 0f28d0                       movaps xmm2, xmm0
140976fcc: 0f59d0                       mulps xmm2, xmm0
140976fcf: f30f16ca                     movshdup xmm1, xmm2
140976fd3: f30f58ca                     addss xmm1, xmm2
140976fd7: 0f28da                       movaps xmm3, xmm2
140976fda: 660f15da                     unpckhpd xmm3, xmm2
140976fde: f30f58d9                     addss xmm3, xmm1
140976fe2: 0fc6db00                     shufps xmm3, xmm3, 0
140976fe6: 0fc6d2ff                     shufps xmm2, xmm2, 0xff
140976fea: 0f5cd3                       subps xmm2, xmm3
140976fed: 0f5915ecb31102               mulps xmm2, xmmword ptr [rip + 0x211b3ec]
140976ff4: 0f280d85200302               movaps xmm1, xmmword ptr [rip + 0x2032085]
140976ffb: 0f59c8                       mulps xmm1, xmm0
140976ffe: f30f16d9                     movshdup xmm3, xmm1
140977002: f30f58d9                     addss xmm3, xmm1
140977006: 0f12c9                       movhlps xmm1, xmm1
140977009: f30f58cb                     addss xmm1, xmm3
14097700d: f30f58c9                     addss xmm1, xmm1
140977011: 0fc6c900                     shufps xmm1, xmm1, 0
140977015: 0f59c8                       mulps xmm1, xmm0
140977018: 0f58ca                       addps xmm1, xmm2
14097701b: 0f28d0                       movaps xmm2, xmm0
14097701e: 0fc6d0d6                     shufps xmm2, xmm0, 0xd6
140977022: 0f5915d7600502               mulps xmm2, xmmword ptr [rip + 0x20560d7]
140977029: 0f28d8                       movaps xmm3, xmm0
14097702c: 0fc6d8c5                     shufps xmm3, xmm0, 0xc5
140977030: 0f591db9b31102               mulps xmm3, xmmword ptr [rip + 0x211b3b9]
140977037: 0f5cda                       subps xmm3, xmm2
14097703a: 0f58c0                       addps xmm0, xmm0
14097703d: 0fc6c0ff                     shufps xmm0, xmm0, 0xff
140977041: 0f59c3                       mulps xmm0, xmm3
140977044: 0f58c1                       addps xmm0, xmm1
140977047: 0f28c8                       movaps xmm1, xmm0
14097704a: 660f15c8                     unpckhpd xmm1, xmm0
14097704e: 0f28159be8fa01               movaps xmm2, xmmword ptr [rip + 0x1fae89b]
140977055: 0f57c2                       xorps xmm0, xmm2
140977058: 0f57ca                       xorps xmm1, xmm2
14097705b: e8c0cdfa01                   call 0x142923e20
140977060: 0f28f0                       movaps xmm6, xmm0
140977063: 4d8b3f                       mov r15, qword ptr [r15]
140977066: 4c89e1                       mov rcx, r12
140977069: 4c89fa                       mov rdx, r15
14097706c: e87f3295ff                   call 0x1402ca2f0
140977071: 488b4f30                     mov rcx, qword ptr [rdi + 0x30]
140977075: 488b5738                     mov rdx, qword ptr [rdi + 0x38]
140977079: e85213f4ff                   call 0x1408b83d0
14097707e: 89c7                         mov edi, eax
140977080: 0f104500                     movups xmm0, xmmword ptr [rbp]
140977084: 0f29442430                   movaps xmmword ptr [rsp + 0x30], xmm0
140977089: 488b4510                     mov rax, qword ptr [rbp + 0x10]
14097708d: 4889442440                   mov qword ptr [rsp + 0x40], rax
140977092: 418b451c                     mov eax, dword ptr [r13 + 0x1c]
140977096: 498b4d10                     mov rcx, qword ptr [r13 + 0x10]
14097709a: 8901                         mov dword ptr [rcx], eax
14097709c: 4d8b6500                     mov r12, qword ptr [r13]
1409770a0: 4d8b6c2430                   mov r13, qword ptr [r12 + 0x30]
1409770a5: 498b6c2440                   mov rbp, qword ptr [r12 + 0x40]
1409770aa: 4d3b6c2420                   cmp r13, qword ptr [r12 + 0x20]
1409770af: 7511                         jne 0x1409770c2
1409770b1: 498d4c2420                   lea rcx, [r12 + 0x20]
1409770b6: 488d158b302c02               lea rdx, [rip + 0x22c308b]
1409770bd: e82e891e00                   call 0x140b5f9f0
1409770c2: 498b442428                   mov rax, qword ptr [r12 + 0x28]
1409770c7: 4a8d0ced00000000             lea rcx, [r13*8]
1409770cf: 4c01e9                       add rcx, r13
1409770d2: 0f10442449                   movups xmm0, xmmword ptr [rsp + 0x49]
1409770d7: 0f1144c819                   movups xmmword ptr [rax + rcx*8 + 0x19], xmm0
1409770dc: 0f28442430                   movaps xmm0, xmmword ptr [rsp + 0x30]
1409770e1: 0f284c2440                   movaps xmm1, xmmword ptr [rsp + 0x40]
1409770e6: 0f114cc810                   movups xmmword ptr [rax + rcx*8 + 0x10], xmm1
1409770eb: 0f1104c8                     movups xmmword ptr [rax + rcx*8], xmm0
1409770ef: c644c82903                   mov byte ptr [rax + rcx*8 + 0x29], 3
1409770f4: f30f1174c82c                 movss dword ptr [rax + rcx*8 + 0x2c], xmm6
1409770fa: 488b16                       mov rdx, qword ptr [rsi]
1409770fd: 488954c830                   mov qword ptr [rax + rcx*8 + 0x30], rdx
140977102: 8b5608                       mov edx, dword ptr [rsi + 8]
140977105: 8954c838                     mov dword ptr [rax + rcx*8 + 0x38], edx
140977109: 40887cc83c                   mov byte ptr [rax + rcx*8 + 0x3c], dil
14097710e: 48896cc840                   mov qword ptr [rax + rcx*8 + 0x40], rbp
140977113: 49ffc5                       inc r13
140977116: 4d896c2430                   mov qword ptr [r12 + 0x30], r13
14097711b: 49ff442440                   inc qword ptr [r12 + 0x40]
140977120: 4889d9                       mov rcx, rbx
140977123: 4c89f2                       mov rdx, r14
140977126: e845f11000                   call 0x140a86270
14097712b: 0fb605d7dacb02               movzx eax, byte ptr [rip + 0x2cbdad7]
140977132: b918000000                   mov ecx, 0x18
140977137: ba08000000                   mov edx, 8
14097713c: e8cfcac9ff                   call 0x140613c10
140977141: 4885c0                       test rax, rax
140977144: 7453                         je 0x140977199
140977146: 48c70001000000               mov qword ptr [rax], 1
14097714d: 48c7400801000000             mov qword ptr [rax + 8], 1
140977155: 4c897810                     mov qword ptr [rax + 0x10], r15
140977159: 4889442438                   mov qword ptr [rsp + 0x38], rax
14097715e: 48c74424300d000000           mov qword ptr [rsp + 0x30], 0xd
140977167: 4c8d0512f81302               lea r8, [rip + 0x213f812]
14097716e: 488d542430                   lea rdx, [rsp + 0x30]
140977173: 4889d9                       mov rcx, rbx
140977176: e845dd1000                   call 0x140a84ec0
14097717b: 0f28742460                   movaps xmm6, xmmword ptr [rsp + 0x60]
140977180: 0f287c2470                   movaps xmm7, xmmword ptr [rsp + 0x70]
140977185: 4881c488000000               add rsp, 0x88
14097718c: 5b                           pop rbx
14097718d: 5d                           pop rbp
14097718e: 5f                           pop rdi
14097718f: 5e                           pop rsi
140977190: 415c                         pop r12
140977192: 415d                         pop r13
140977194: 415e                         pop r14
140977196: 415f                         pop r15
140977198: c3                           ret 
140977199: b908000000                   mov ecx, 8
14097719e: ba18000000                   mov edx, 0x18
1409771a3: e8581ef601                   call 0x1428d9000
1409771a8: 49c1e020                     shl r8, 0x20
1409771ac: 4d09d0                       or r8, r10
1409771af: b901000000                   mov ecx, 1
1409771b4: 4c89c0                       mov rax, r8
1409771b7: 894c2430                     mov dword ptr [rsp + 0x30], ecx
1409771bb: 44894c2434                   mov dword ptr [rsp + 0x34], r9d
1409771c0: 4889442438                   mov qword ptr [rsp + 0x38], rax
1409771c5: 488d059cf71302               lea rax, [rip + 0x213f79c]
1409771cc: 4889442420                   mov qword ptr [rsp + 0x20], rax
1409771d1: 488d0d18f51302               lea rcx, [rip + 0x213f518]
1409771d8: 4c8d0df1f41302               lea r9, [rip + 0x213f4f1]
1409771df: 4c8d442430                   lea r8, [rsp + 0x30]
1409771e4: ba2b000000                   mov edx, 0x2b
1409771e9: e87225f601                   call 0x1428d9760
1409771ee: cc                           int3 
