000000014130d1b0  55                       push       rbp
000000014130d1b1  4157                     push       r15
000000014130d1b3  4156                     push       r14
000000014130d1b5  4155                     push       r13
000000014130d1b7  4154                     push       r12
000000014130d1b9  56                       push       rsi
000000014130d1ba  57                       push       rdi
000000014130d1bb  53                       push       rbx
000000014130d1bc  4881ec48010000           sub        rsp, 0x148
000000014130d1c3  488dac2480000000         lea        rbp, [rsp + 0x80]
000000014130d1cb  48c785c0000000feffffff   mov        qword ptr [rbp + 0xc0], 0xfffffffffffffffe
000000014130d1d6  4c89c7                   mov        rdi, r8
000000014130d1d9  4889d3                   mov        rbx, rdx
000000014130d1dc  4889ce                   mov        rsi, rcx
000000014130d1df  488d4db0                 lea        rcx, [rbp - 0x50]
000000014130d1e3  e87857feff               call       0x1412f2960
000000014130d1e8  f645b001                 test       byte ptr [rbp - 0x50], 1
000000014130d1ec  0f845d010000             je         0x14130d34f
000000014130d1f2  48833b00                 cmp        qword ptr [rbx], 0
000000014130d1f6  740b                     je         0x14130d203
000000014130d1f8  48837b2000               cmp        qword ptr [rbx + 0x20], 0
000000014130d1fd  0f849f010000             je         0x14130d3a2
000000014130d203  48837b5000               cmp        qword ptr [rbx + 0x50], 0
000000014130d208  740b                     je         0x14130d215
000000014130d20a  48837b7000               cmp        qword ptr [rbx + 0x70], 0
000000014130d20f  0f848d010000             je         0x14130d3a2
000000014130d215  0fb605ed793202           movzx      eax, byte ptr [rip + 0x23279ed]
000000014130d21c  b9a0000000               mov        ecx, 0xa0
000000014130d221  ba04000000               mov        edx, 4
000000014130d226  e8e56930ff               call       0x140613c10
000000014130d22b  4885c0                   test       rax, rax
000000014130d22e  0f847a010000             je         0x14130d3ae
000000014130d234  4989c4                   mov        r12, rax
000000014130d237  488d45b4                 lea        rax, [rbp - 0x4c]
000000014130d23b  488b4820                 mov        rcx, qword ptr [rax + 0x20]
000000014130d23f  49894c2420               mov        qword ptr [r12 + 0x20], rcx
000000014130d244  0f1000                   movups     xmm0, xmmword ptr [rax]
000000014130d247  0f104810                 movups     xmm1, xmmword ptr [rax + 0x10]
000000014130d24b  410f114c2410             movups     xmmword ptr [r12 + 0x10], xmm1
000000014130d251  410f110424               movups     xmmword ptr [r12], xmm0
000000014130d256  48c785a800000004000000   mov        qword ptr [rbp + 0xa8], 4
000000014130d261  4c89a5b0000000           mov        qword ptr [rbp + 0xb0], r12
000000014130d268  48c785b800000001000000   mov        qword ptr [rbp + 0xb8], 1
000000014130d273  488d7db0                 lea        rdi, [rbp - 0x50]
000000014130d277  41b8c8000000             mov        r8d, 0xc8
000000014130d27d  4889f9                   mov        rcx, rdi
000000014130d280  4889da                   mov        rdx, rbx
000000014130d283  e878696101               call       0x142923c00
000000014130d288  bb01000000               mov        ebx, 1
000000014130d28d  4c8dad80000000           lea        r13, [rbp + 0x80]
000000014130d294  4c8d757c                 lea        r14, [rbp + 0x7c]
000000014130d298  4c8dbda8000000           lea        r15, [rbp + 0xa8]
000000014130d29f  eb62                     jmp        0x14130d303
000000014130d2a1  6666666666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
000000014130d2b0  48c744242028000000       mov        qword ptr [rsp + 0x20], 0x28
000000014130d2b9  41b801000000             mov        r8d, 1
000000014130d2bf  41b904000000             mov        r9d, 4
000000014130d2c5  4c89f9                   mov        rcx, r15
000000014130d2c8  4889da                   mov        rdx, rbx
000000014130d2cb  e890c75901               call       0x1428a9a60
000000014130d2d0  4c8ba5b0000000           mov        r12, qword ptr [rbp + 0xb0]
000000014130d2d7  488d049b                 lea        rax, [rbx + rbx*4]
000000014130d2db  498b4d20                 mov        rcx, qword ptr [r13 + 0x20]
000000014130d2df  49894cc420               mov        qword ptr [r12 + rax*8 + 0x20], rcx
000000014130d2e4  410f104500               movups     xmm0, xmmword ptr [r13]
000000014130d2e9  410f104d10               movups     xmm1, xmmword ptr [r13 + 0x10]
000000014130d2ee  410f114cc410             movups     xmmword ptr [r12 + rax*8 + 0x10], xmm1
000000014130d2f4  410f1104c4               movups     xmmword ptr [r12 + rax*8], xmm0
000000014130d2f9  48ffc3                   inc        rbx
000000014130d2fc  48899db8000000           mov        qword ptr [rbp + 0xb8], rbx
000000014130d303  4c89f1                   mov        rcx, r14
000000014130d306  4889fa                   mov        rdx, rdi
000000014130d309  e85256feff               call       0x1412f2960
000000014130d30e  837d7c01                 cmp        dword ptr [rbp + 0x7c], 1
000000014130d312  7554                     jne        0x14130d368
000000014130d314  483b9da8000000           cmp        rbx, qword ptr [rbp + 0xa8]
000000014130d31b  75ba                     jne        0x14130d2d7
000000014130d31d  48837db000               cmp        qword ptr [rbp - 0x50], 0
000000014130d322  7407                     je         0x14130d32b
000000014130d324  48837dd000               cmp        qword ptr [rbp - 0x30], 0
000000014130d329  7469                     je         0x14130d394
000000014130d32b  48837d0000               cmp        qword ptr [rbp], 0
000000014130d330  0f847affffff             je         0x14130d2b0
000000014130d336  48837d2000               cmp        qword ptr [rbp + 0x20], 0
000000014130d33b  0f856fffffff             jne        0x14130d2b0
000000014130d341  488d0da0ba9201           lea        rcx, [rip + 0x192baa0]
000000014130d348  e893c65c01               call       0x1428d99e0
000000014130d34d  eb51                     jmp        0x14130d3a0
000000014130d34f  48c70600000000           mov        qword ptr [rsi], 0
000000014130d356  48c7460804000000         mov        qword ptr [rsi + 8], 4
000000014130d35e  48c7461000000000         mov        qword ptr [rsi + 0x10], 0
000000014130d366  eb15                     jmp        0x14130d37d
000000014130d368  488b85b8000000           mov        rax, qword ptr [rbp + 0xb8]
000000014130d36f  48894610                 mov        qword ptr [rsi + 0x10], rax
000000014130d373  0f1085a8000000           movups     xmm0, xmmword ptr [rbp + 0xa8]
000000014130d37a  0f1106                   movups     xmmword ptr [rsi], xmm0
000000014130d37d  4889f0                   mov        rax, rsi
000000014130d380  4881c448010000           add        rsp, 0x148
000000014130d387  5b                       pop        rbx
000000014130d388  5f                       pop        rdi
000000014130d389  5e                       pop        rsi
000000014130d38a  415c                     pop        r12
000000014130d38c  415d                     pop        r13
000000014130d38e  415e                     pop        r14
000000014130d390  415f                     pop        r15
000000014130d392  5d                       pop        rbp
000000014130d393  c3                       ret        
000000014130d394  488d0d4dba9201           lea        rcx, [rip + 0x192ba4d]
000000014130d39b  e840c65c01               call       0x1428d99e0
000000014130d3a0  0f0b                     ud2        
000000014130d3a2  488d0d3fba9201           lea        rcx, [rip + 0x192ba3f]
000000014130d3a9  e832c65c01               call       0x1428d99e0
000000014130d3ae  b904000000               mov        ecx, 4
000000014130d3b3  baa0000000               mov        edx, 0xa0
000000014130d3b8  4989f8                   mov        r8, rdi
000000014130d3bb  e823bc5c01               call       0x1428d8fe3
000000014130d3c0  cc                       int3       
000000014130d3c1  6666666666662e0f1f840000000000 nop        word ptr cs:[rax + rax]
000000014130d3d0  4889542410               mov        qword ptr [rsp + 0x10], rdx
000000014130d3d5  55                       push       rbp
000000014130d3d6  4157                     push       r15
000000014130d3d8  4156                     push       r14
000000014130d3da  4155                     push       r13
000000014130d3dc  4154                     push       r12
000000014130d3de  56                       push       rsi
000000014130d3df  57                       push       rdi
000000014130d3e0  53                       push       rbx
000000014130d3e1  4883ec28                 sub        rsp, 0x28
000000014130d3e5  488daa80000000           lea        rbp, [rdx + 0x80]
000000014130d3ec  488b85a8000000           mov        rax, qword ptr [rbp + 0xa8]
000000014130d3f3  4885c0                   test       rax, rax
000000014130d3f6  741a                     je         0x14130d412
000000014130d3f8  488b8db0000000           mov        rcx, qword ptr [rbp + 0xb0]
000000014130d3ff  48c1e003                 shl        rax, 3
000000014130d403  488d1480                 lea        rdx, [rax + rax*4]
000000014130d407  41b804000000             mov        r8d, 4
000000014130d40d  e80e6830ff               call       0x140613c20
000000014130d412  90                       nop        
000000014130d413  4883c428                 add        rsp, 0x28
000000014130d417  5b                       pop        rbx
000000014130d418  5f                       pop        rdi
000000014130d419  5e                       pop        rsi
000000014130d41a  415c                     pop        r12
000000014130d41c  415d                     pop        r13
000000014130d41e  415e                     pop        r14
000000014130d420  415f                     pop        r15
000000014130d422  5d                       pop        rbp
000000014130d423  c3                       ret        
