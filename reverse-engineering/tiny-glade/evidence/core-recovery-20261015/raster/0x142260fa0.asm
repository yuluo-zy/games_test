0x142260fa0 movups   xmmword ptr [rdi + 0x260], xmm3
0x142260fa7 mov      rcx, rsi
0x142260faa call     0x142262680
0x142260faf mov      rcx, r15
0x142260fb2 call     0x140a97860
0x142260fb7 mov      r9, rdx
0x142260fba movss    xmm0, dword ptr [rdi + 0x60]
0x142260fbf movzx    ecx, byte ptr [rdi + 0x64]
0x142260fc3 mov      edx, dword ptr [rdi + 0x68]
0x142260fc6 movss    xmm1, dword ptr [rdi + 0x6c]
0x142260fcb movss    dword ptr [rsp + 0x38], xmm1
0x142260fd1 mov      dword ptr [rsp + 0x30], edx
0x142260fd5 mov      byte ptr [rsp + 0x28], cl
0x142260fd9 movss    dword ptr [rsp + 0x20], xmm0
0x142260fdf mov      rcx, rsi
0x142260fe2 mov      dl, 1
0x142260fe4 mov      r8, rax
0x142260fe7 call     0x142261d10
0x142260fec mov      rcx, qword ptr [r14 + 0x60]
0x142260ff0 lea      rdx, [rip + 0xed3279]
0x142260ff7 call     0x1425dad90
0x142260ffc test     rax, rax
0x142260fff je       0x14226136e
0x142261005 mov      r14, rax
0x142261008 lea      rax, [rip + 0xed3399]
0x14226100f mov      qword ptr [rsp + 0x48], rax
0x142261014 mov      qword ptr [rsp + 0x50], 0x42
0x14226101d cmp      qword ptr [r14 + 0x18], 0x42
0x142261022 jne      0x142261314
0x142261028 mov      rdx, qword ptr [r14 + 0x10]
0x14226102c lea      rcx, [rip + 0xed3375]
0x142261033 mov      r8d, 0x42
0x142261039 call     0x142923bf0
0x14226103e test     eax, eax
0x142261040 jne      0x142261314
0x142261046 mov      rcx, qword ptr [r14]
0x142261049 mov      rax, qword ptr [r14 + 8]
0x14226104d call     qword ptr [rax + 0x18]
0x142261050 test     rax, rax
0x142261053 je       0x14226136e
0x142261059 mov      r8, qword ptr [rax]
0x14226105c mov      r9, qword ptr [rax + 8]
0x142261060 mov      rcx, rsi
0x142261063 mov      dl, 1
0x142261065 call     0x142261760
0x14226106a movss    xmm6, dword ptr [rdi + 0x60]
0x14226106f cmp      byte ptr [rdi + 0x64], 0
0x142261073 je       0x142261295
0x142261079 mov      eax, dword ptr [rbp + 0x1c]
0x14226107c mov      rcx, qword ptr [rbp + 0x10]
0x142261080 mov      dword ptr [rcx], eax
0x142261082 mov      rsi, qword ptr [rbp]
0x142261086 mov      rdi, qword ptr [rsi + 0x30]
0x14226108a mov      r14, qword ptr [rsi + 0x40]
0x14226108e cmp      rdi, qword ptr [rsi + 0x20]
0x142261092 jne      0x1422612c4
0x142261098 lea      rcx, [rsi + 0x20]
0x14226109c lea      rdx, [rip + 0xdd2cf5]
0x1422610a3 call     0x140562c30
0x1422610a8 jmp      0x1422612c4
0x1422610ad mov      eax, dword ptr [r14 + 0x3c]
0x1422610b1 mov      rcx, qword ptr [r14 + 0x30]
0x1422610b5 mov      dword ptr [rcx], eax
0x1422610b7 mov      rcx, qword ptr [r14 + 0x20]
0x1422610bb call     0x140b2bbf0
0x1422610c0 mov      rdx, qword ptr [r14 + 0x40]
0x1422610c4 lea      rsi, [rsp + 0x60]
0x1422610c9 mov      rcx, rsi
0x1422610cc call     0x140a12650
0x1422610d1 movzx    r8d, byte ptr [r15 + 0xc]
0x1422610d6 movss    xmm0, dword ptr [r15 + 4]
0x1422610dc movss    xmm3, dword ptr [r15 + 8]
0x1422610e2 movss    dword ptr [rsp + 0x28], xmm0
0x1422610e8 mov      dword ptr [rsp + 0x20], r13d
0x1422610ed mov      rcx, rdi
0x1422610f0 mov      rdx, rsi
0x1422610f3 call     0x140868d00
0x1422610f8 jmp      0x1422612f8
0x1422610fd cmp      qword ptr [rdi + 0x58], 1
0x142261102 jbe      0x1422612f8
0x142261108 mov      eax, dword ptr [r14 + 0xbc]
0x14226110f mov      rcx, qword ptr [r14 + 0xb0]
0x142261116 mov      dword ptr [rcx], eax
0x142261118 mov      rbx, qword ptr [r14 + 0xa0]
0x14226111f mov      r15, qword ptr [rbx + 0x30]
0x142261123 mov      r13, qword ptr [rbx + 0x40]
0x142261127 cmp      r15, qword ptr [rbx + 0x20]
0x14226112b jne      0x14226113d
0x14226112d lea      rcx, [rbx + 0x20]
0x142261131 lea      rdx, [rip + 0xed3070]
0x142261138 call     0x140562870
0x14226113d mov      rax, qword ptr [rbx + 0x28]
0x142261141 mov      qword ptr [rax + r15*8], r13
0x142261145 inc      r15
0x142261148 mov      qword ptr [rbx + 0x30], r15
0x14226114c inc      qword ptr [rbx + 0x40]
0x142261150 mov      eax, dword ptr [r12 + 0x1c]
0x142261155 mov      rcx, qword ptr [r12 + 0x10]
0x14226115a mov      dword ptr [rcx], eax
0x14226115c mov      rbx, qword ptr [r12]
0x142261160 mov      r15, qword ptr [rbx + 0x30]
0x142261164 mov      r12, qword ptr [rbx + 0x40]
0x142261168 cmp      r15, qword ptr [rbx + 0x20]
0x14226116c jne      0x14226117e
0x14226116e lea      rcx, [rbx + 0x20]
0x142261172 lea      rdx, [rip + 0xed302f]
0x142261179 call     0x140562870
0x14226117e mov      rax, qword ptr [rbx + 0x28]
0x142261182 mov      qword ptr [rax + r15*8], r12
0x142261186 inc      r15
0x142261189 mov      qword ptr [rbx + 0x30], r15
0x14226118d inc      qword ptr [rbx + 0x40]
0x142261191 mov      eax, dword ptr [r14 + 0x9c]
0x142261198 mov      rcx, qword ptr [r14 + 0x90]
0x14226119f mov      dword ptr [rcx], eax
0x1422611a1 mov      rbx, qword ptr [r14 + 0x80]
0x1422611a8 mov      r15, qword ptr [rbx + 0x30]
0x1422611ac mov      r12, qword ptr [rbx + 0x40]
0x1422611b0 cmp      r15, qword ptr [rbx + 0x20]
0x1422611b4 jne      0x1422611c6
0x1422611b6 lea      rcx, [rbx + 0x20]
0x1422611ba lea      rdx, [rip + 0xdd2bd7]
0x1422611c1 call     0x140562870
0x1422611c6 mov      rax, qword ptr [rbx + 0x28]
0x1422611ca mov      qword ptr [rax + r15*8], r12
0x1422611ce inc      r15
0x1422611d1 mov      qword ptr [rbx + 0x30], r15
0x1422611d5 inc      qword ptr [rbx + 0x40]
0x1422611d9 add      rdi, 0x70
0x1422611dd lea      rcx, [rsi + 0x98]
0x1422611e4 mov      rdx, rdi
0x1422611e7 call     0x140960c30
0x1422611ec mov      rcx, rdi
0x1422611ef call     0x140960c20
0x1422611f4 mov      rcx, rsi
0x1422611f7 call     0x142262a60
0x1422611fc mov      rcx, qword ptr [r14 + 0x60]
0x142261200 lea      rdx, [rip + 0xed3069]
0x142261207 call     0x1425dad90
0x14226120c test     rax, rax
0x14226120f je       0x14226137a
0x142261215 mov      rdi, rax
0x142261218 lea      rax, [rip + 0xed3189]
0x14226121f mov      qword ptr [rsp + 0x48], rax
0x142261224 mov      qword ptr [rsp + 0x50], 0x42
0x14226122d cmp      qword ptr [rdi + 0x18], 0x42
0x142261232 jne      0x142261341
0x142261238 mov      rdx, qword ptr [rdi + 0x10]
0x14226123c lea      rcx, [rip + 0xed3165]
0x142261243 mov      r8d, 0x42
0x142261249 call     0x142923bf0
0x14226124e test     eax, eax
0x142261250 jne      0x142261341
0x142261256 mov      rcx, qword ptr [rdi]
0x142261259 mov      rax, qword ptr [rdi + 8]
0x14226125d call     qword ptr [rax + 0x18]
0x142261260 test     rax, rax
0x142261263 je       0x14226137a
0x142261269 mov      r8, qword ptr [rax]
0x14226126c mov      r9, qword ptr [rax + 8]
0x142261270 mov      rcx, rsi
0x142261273 mov      dl, 1
0x142261275 movaps   xmm6, xmmword ptr [rsp + 0x90]
0x14226127d add      rsp, 0xa8
0x142261284 pop      rbx
0x142261285 pop      rbp
0x142261286 pop      rdi
0x142261287 pop      rsi
0x142261288 pop      r12
0x14226128a pop      r13
0x14226128c pop      r14
0x14226128e pop      r15
0x142261290 jmp      0x142261760
0x142261295 mov      rdx, qword ptr [rsp + 0x58]
0x14226129a mov      eax, dword ptr [rdx + 0x1c]
0x14226129d mov      rcx, qword ptr [rdx + 0x10]
0x1422612a1 mov      dword ptr [rcx], eax
0x1422612a3 mov      rsi, qword ptr [rdx]
0x1422612a6 mov      rdi, qword ptr [rsi + 0x30]
0x1422612aa mov      r14, qword ptr [rsi + 0x40]
0x1422612ae cmp      rdi, qword ptr [rsi + 0x20]
0x1422612b2 jne      0x1422612c4
0x1422612b4 lea      rcx, [rsi + 0x20]
0x1422612b8 lea      rdx, [rip + 0xdd2ad9]
0x1422612bf call     0x140562c30
0x1422612c4 mov      rax, qword ptr [rsi + 0x28]
0x1422612c8 mov      rcx, rdi
0x1422612cb shl      rcx, 5
0x1422612cf mov      qword ptr [rax + rcx], r14
0x1422612d3 mov      rdx, qword ptr [rbx]
0x1422612d6 mov      qword ptr [rax + rcx + 8], rdx
0x1422612db mov      edx, dword ptr [rbx + 8]
0x1422612de mov      dword ptr [rax + rcx + 0x10], edx
0x1422612e2 movss    dword ptr [rax + rcx + 0x14], xmm6
0x1422612e8 mov      byte ptr [rax + rcx + 0x18], 0
0x1422612ed inc      rdi
0x1422612f0 mov      qword ptr [rsi + 0x30], rdi
0x1422612f4 inc      qword ptr [rsi + 0x40]
0x1422612f8 movaps   xmm6, xmmword ptr [rsp + 0x90]
0x142261300 add      rsp, 0xa8
0x142261307 pop      rbx
0x142261308 pop      rbp
0x142261309 pop      rdi
0x14226130a pop      rsi
0x14226130b pop      r12
0x14226130d pop      r13
0x14226130f pop      r14
0x142261311 pop      r15
0x142261313 ret      
0x142261314 mov      qword ptr [rsp + 0x60], 0
0x14226131d add      r14, 0x10
0x142261321 lea      rax, [rip + 0xed2fb8]
0x142261328 mov      qword ptr [rsp + 0x20], rax
0x14226132d lea      rdx, [rsp + 0x48]
0x142261332 lea      r9, [rsp + 0x60]
0x142261337 xor      ecx, ecx
0x142261339 mov      r8, r14
0x14226133c call     0x1428cab41
0x142261341 mov      qword ptr [rsp + 0x60], 0
0x14226134a add      rdi, 0x10
0x14226134e lea      rax, [rip + 0xed2f8b]
0x142261355 mov      qword ptr [rsp + 0x20], rax
0x14226135a lea      rdx, [rsp + 0x48]
0x14226135f lea      r9, [rsp + 0x60]
0x142261364 xor      ecx, ecx
0x142261366 mov      r8, rdi
0x142261369 call     0x1428cab41
0x14226136e lea      rcx, [rip + 0xdd2ab3]
0x142261375 call     0x1428d9310
0x14226137a lea      rcx, [rip + 0xdd2abf]
0x142261381 call     0x1428d9310
0x142261386 int3     
0x142261387 int3     
0x142261388 int3     
0x142261389 int3     
0x14226138a int3     
0x14226138b int3     
0x14226138c int3     
0x14226138d int3     
0x14226138e int3     
0x14226138f int3     
0x142261390 push     r15
0x142261392 push     r14
0x142261394 push     r13
0x142261396 push     r12
0x142261398 push     rsi
0x142261399 push     rdi
0x14226139a push     rbp
0x14226139b push     rbx
0x14226139c sub      rsp, 0x78