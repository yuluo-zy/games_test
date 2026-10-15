0x140144ec0 push     rbp
0x140144ec1 push     r14
0x140144ec3 push     rsi
0x140144ec4 push     rdi
0x140144ec5 push     rbx
0x140144ec6 sub      rsp, 0x350
0x140144ecd lea      rbp, [rsp + 0x80]
0x140144ed5 mov      qword ptr [rbp + 0x2c8], 0xfffffffffffffffe
0x140144ee0 mov      rdi, rdx
0x140144ee3 mov      rsi, rcx
0x140144ee6 mov      rbx, qword ptr [rcx]
0x140144ee9 mov      r14, qword ptr [rcx + 0x18]
0x140144eed movzx    eax, byte ptr [rip + 0x34efd15]
0x140144ef4 mov      ecx, 0x10
0x140144ef9 mov      edx, 8
0x140144efe call     0x140613c10
0x140144f03 test     rax, rax
0x140144f06 je       0x140145337
0x140144f0c mov      qword ptr [rax], rbx
0x140144f0f mov      qword ptr [rax + 8], r14
0x140144f13 lea      rcx, [rip + 0x280c0fe]
0x140144f1a mov      qword ptr [rsp + 0x20], rcx
0x140144f1f lea      rcx, [rbp + 0xb8]
0x140144f26 mov      rdx, rdi
0x140144f29 xor      r8d, r8d
0x140144f2c mov      r9, rax
0x140144f2f call     0x1400c4230
0x140144f34 mov      rax, qword ptr [rbp + 0xc8]
0x140144f3b lock dec qword ptr [rax]
0x140144f3f jne      0x140144f4d
0x140144f41 lea      rcx, [rbp + 0xc8]
0x140144f48 call     0x140c41170
0x140144f4d mov      rax, qword ptr [rsi + 0x20]
0x140144f51 cmp      byte ptr [rax], 1
0x140144f54 jne      0x140145319
0x140144f5a mov      rbx, qword ptr [rsi + 8]
0x140144f5e movzx    eax, byte ptr [rip + 0x34efca4]
0x140144f65 mov      ecx, 8
0x140144f6a mov      edx, 8
0x140144f6f call     0x140613c10
0x140144f74 test     rax, rax
0x140144f77 je       0x140145328
0x140144f7d mov      qword ptr [rax], rbx
0x140144f80 lea      rcx, [rip + 0x280c071]
0x140144f87 mov      qword ptr [rsp + 0x20], rcx
0x140144f8c lea      rcx, [rbp + 0xb8]
0x140144f93 mov      rdx, rdi
0x140144f96 xor      r8d, r8d
0x140144f99 mov      r9, rax
0x140144f9c call     0x1400c4230
0x140144fa1 mov      rax, qword ptr [rbp + 0xc8]
0x140144fa8 lock dec qword ptr [rax]
0x140144fac jne      0x140144fba
0x140144fae lea      rcx, [rbp + 0xc8]
0x140144fb5 call     0x140c41170
0x140144fba movzx    eax, byte ptr [rip + 0x34efc48]
0x140144fc1 mov      ecx, 8
0x140144fc6 mov      edx, 8
0x140144fcb call     0x140613c10
0x140144fd0 test     rax, rax
0x140144fd3 je       0x140145328
0x140144fd9 lea      rcx, [rbx + 0x24]
0x140144fdd mov      qword ptr [rax], rcx
0x140144fe0 lea      rcx, [rip + 0x280c051]
0x140144fe7 mov      qword ptr [rsp + 0x20], rcx
0x140144fec lea      rcx, [rbp + 0xb8]
0x140144ff3 mov      rdx, rdi
0x140144ff6 xor      r8d, r8d
0x140144ff9 mov      r9, rax
0x140144ffc call     0x1400c4230
0x140145001 mov      rax, qword ptr [rbp + 0xc8]
0x140145008 lock dec qword ptr [rax]
0x14014500c jne      0x14014501a
0x14014500e lea      rcx, [rbp + 0xc8]
0x140145015 call     0x140c41170
0x14014501a movzx    eax, byte ptr [rip + 0x34efbe8]
0x140145021 mov      ecx, 8
0x140145026 mov      edx, 8
0x14014502b call     0x140613c10
0x140145030 test     rax, rax
0x140145033 je       0x140145328
0x140145039 lea      rcx, [rbx + 0x18]
0x14014503d mov      qword ptr [rax], rcx
0x140145040 lea      rcx, [rip + 0x280bf31]
0x140145047 mov      qword ptr [rsp + 0x20], rcx
0x14014504c lea      rcx, [rbp + 0xb8]
0x140145053 mov      rdx, rdi
0x140145056 xor      r8d, r8d
0x140145059 mov      r9, rax
0x14014505c call     0x1400c4230
0x140145061 mov      rax, qword ptr [rbp + 0xc8]
0x140145068 lock dec qword ptr [rax]
0x14014506c jne      0x14014507a
0x14014506e lea      rcx, [rbp + 0xc8]
0x140145075 call     0x140c41170
0x14014507a movzx    eax, byte ptr [rip + 0x34efb88]
0x140145081 mov      ecx, 8
0x140145086 mov      edx, 8
0x14014508b call     0x140613c10
0x140145090 test     rax, rax
0x140145093 je       0x140145328
0x140145099 lea      rcx, [rbx + 0xc]
0x14014509d mov      qword ptr [rax], rcx
0x1401450a0 lea      rcx, [rip + 0x280bfb1]
0x1401450a7 mov      qword ptr [rsp + 0x20], rcx
0x1401450ac lea      rcx, [rbp + 0xb8]
0x1401450b3 mov      rdx, rdi
0x1401450b6 xor      r8d, r8d
0x1401450b9 mov      r9, rax
0x1401450bc call     0x1400c4230
0x1401450c1 mov      rax, qword ptr [rbp + 0xc8]
0x1401450c8 lock dec qword ptr [rax]
0x1401450cc jne      0x1401450da
0x1401450ce lea      rcx, [rbp + 0xc8]
0x1401450d5 call     0x140c41170
0x1401450da movzx    eax, byte ptr [rip + 0x34efb28]
0x1401450e1 mov      ecx, 8
0x1401450e6 mov      edx, 8
0x1401450eb call     0x140613c10
0x1401450f0 test     rax, rax
0x1401450f3 je       0x140145328
0x1401450f9 lea      rcx, [rbx + 0x30]
0x1401450fd mov      qword ptr [rax], rcx
0x140145100 lea      rcx, [rip + 0x280be11]
0x140145107 mov      qword ptr [rsp + 0x20], rcx
0x14014510c lea      rcx, [rbp + 0xb8]
0x140145113 mov      rdx, rdi
0x140145116 xor      r8d, r8d
0x140145119 mov      r9, rax
0x14014511c call     0x1400c4230
0x140145121 mov      rax, qword ptr [rbp + 0xc8]
0x140145128 lock dec qword ptr [rax]
0x14014512c jne      0x14014513a
0x14014512e lea      rcx, [rbp + 0xc8]
0x140145135 call     0x140c41170
0x14014513a movzx    eax, byte ptr [rip + 0x34efac8]
0x140145141 mov      ecx, 8
0x140145146 mov      edx, 8
0x14014514b call     0x140613c10
0x140145150 test     rax, rax
0x140145153 je       0x140145328
0x140145159 lea      rcx, [rbx + 0x34]
0x14014515d mov      qword ptr [rax], rcx
0x140145160 lea      rcx, [rip + 0x280bdd1]
0x140145167 mov      qword ptr [rsp + 0x20], rcx
0x14014516c lea      rcx, [rbp + 0xb8]
0x140145173 mov      rdx, rdi
0x140145176 xor      r8d, r8d
0x140145179 mov      r9, rax
0x14014517c call     0x1400c4230
0x140145181 mov      rax, qword ptr [rbp + 0xc8]
0x140145188 lock dec qword ptr [rax]
0x14014518c jne      0x14014519a
0x14014518e lea      rcx, [rbp + 0xc8]
0x140145195 call     0x140c41170
0x14014519a movzx    eax, byte ptr [rip + 0x34efa68]
0x1401451a1 mov      ecx, 8
0x1401451a6 mov      edx, 8
0x1401451ab call     0x140613c10
0x1401451b0 test     rax, rax
0x1401451b3 je       0x140145328
0x1401451b9 add      rbx, 0x40
0x1401451bd mov      qword ptr [rax], rbx
0x1401451c0 lea      rcx, [rip + 0x280bd91]
0x1401451c7 mov      qword ptr [rsp + 0x20], rcx
0x1401451cc lea      rcx, [rbp + 0xb8]
0x1401451d3 mov      rdx, rdi
0x1401451d6 xor      r8d, r8d
0x1401451d9 mov      r9, rax
0x1401451dc call     0x1400c4230
0x1401451e1 mov      rax, qword ptr [rbp + 0xc8]
0x1401451e8 lock dec qword ptr [rax]
0x1401451ec jne      0x1401451fa
0x1401451ee lea      rcx, [rbp + 0xc8]
0x1401451f5 call     0x140c41170
0x1401451fa mov      qword ptr [rbp - 0x50], 4
0x140145202 mov      byte ptr [rbp + 0x2c7], 0
0x140145209 lea      rdx, [rip + 0x280c19e] ; @0x1429513ae 'Delete (2-click)Time ()'
0x140145210 lea      rcx, [rbp + 0xb8]
0x140145217 mov      r8d, 0x10
0x14014521d call     0x1400568d0
0x140145222 movups   xmm0, xmmword ptr [rbp + 0x118]
0x140145229 movaps   xmmword ptr [rbp + 0xa0], xmm0
0x140145230 movups   xmm0, xmmword ptr [rbp + 0x108]
0x140145237 movaps   xmmword ptr [rbp + 0x90], xmm0
0x14014523e movups   xmm0, xmmword ptr [rbp + 0xf8]
0x140145245 movaps   xmmword ptr [rbp + 0x80], xmm0
0x14014524c movups   xmm0, xmmword ptr [rbp + 0xb8]
0x140145253 movups   xmm1, xmmword ptr [rbp + 0xc8]
0x14014525a movups   xmm2, xmmword ptr [rbp + 0xd8]
0x140145261 movups   xmm3, xmmword ptr [rbp + 0xe8]
0x140145268 movaps   xmmword ptr [rbp + 0x70], xmm3
0x14014526c movaps   xmmword ptr [rbp + 0x60], xmm2
0x140145270 movaps   xmmword ptr [rbp + 0x50], xmm1
0x140145274 movaps   xmmword ptr [rbp + 0x40], xmm0
0x140145278 mov      byte ptr [rbp + 0x2c7], 1
0x14014527f lea      rcx, [rbp + 0xb8]
0x140145286 lea      rdx, [rbp - 0x50]
0x14014528a lea      r8, [rbp + 0x40]
0x14014528e call     0x140c46bd0
0x140145293 lea      rcx, [rbp + 0x260]
0x14014529a lea      rdx, [rbp + 0xb8]
0x1401452a1 mov      r8, rdi
0x1401452a4 call     0x140c46d20
0x1401452a9 mov      byte ptr [rbp + 0xb8], 0
0x1401452b0 cmp      byte ptr [rbp + 0x2b3], 0
0x1401452b7 je       0x140145300
0x1401452b9 lea      rcx, [rbp + 0x270]
0x1401452c0 lea      rdx, [rbp + 0xb8]
0x1401452c7 call     0x140042360
0x1401452cc test     al, al
0x1401452ce je       0x140145300
0x1401452d0 mov      rax, qword ptr [rbp + 0x270]
0x1401452d7 lock dec qword ptr [rax]
0x1401452db jne      0x1401452e9
0x1401452dd lea      rcx, [rbp + 0x270]
0x1401452e4 call     0x140c41170
0x1401452e9 mov      rax, qword ptr [rsi + 0x10]
0x1401452ed mov      rcx, qword ptr [rsi + 0x28]
0x1401452f1 mov      rax, qword ptr [rax]
0x1401452f4 mov      dword ptr [rcx], 1
0x1401452fa mov      qword ptr [rcx + 8], rax
0x1401452fe jmp      0x140145319
0x140145300 mov      rax, qword ptr [rbp + 0x270]
0x140145307 lock dec qword ptr [rax]
0x14014530b jne      0x140145319
0x14014530d lea      rcx, [rbp + 0x270]
0x140145314 call     0x140c41170
0x140145319 nop      
0x14014531a add      rsp, 0x350
0x140145321 pop      rbx
0x140145322 pop      rdi
0x140145323 pop      rsi
0x140145324 pop      r14
0x140145326 pop      rbp
0x140145327 ret      
0x140145328 mov      ecx, 8
0x14014532d mov      edx, 8
0x140145332 call     0x1428d9000
0x140145337 mov      ecx, 8
0x14014533c mov      edx, 0x10
0x140145341 call     0x1428d9000