0x140147e00 push     rbp
0x140147e01 push     r15
0x140147e03 push     r14
0x140147e05 push     r13
0x140147e07 push     r12
0x140147e09 push     rsi
0x140147e0a push     rdi
0x140147e0b push     rbx
0x140147e0c sub      rsp, 0x5e8
0x140147e13 lea      rbp, [rsp + 0x80]
0x140147e1b mov      qword ptr [rbp + 0x560], 0xfffffffffffffffe
0x140147e26 mov      rbx, r8
0x140147e29 movups   xmm0, xmmword ptr [rcx]
0x140147e2c movups   xmm1, xmmword ptr [rcx + 0x10]
0x140147e30 movaps   xmmword ptr [rbp + 0x540], xmm1
0x140147e37 movaps   xmmword ptr [rbp + 0x530], xmm0
0x140147e3e movups   xmm0, xmmword ptr [rcx + 0x20]
0x140147e42 movups   xmm1, xmmword ptr [rcx + 0x30]
0x140147e46 movaps   xmmword ptr [rbp + 0x4d0], xmm1
0x140147e4d movaps   xmmword ptr [rbp + 0x4c0], xmm0
0x140147e54 movups   xmm0, xmmword ptr [rcx + 0x40]
0x140147e58 movups   xmm1, xmmword ptr [rcx + 0x50]
0x140147e5c movaps   xmmword ptr [rbp + 0x4b0], xmm1
0x140147e63 movaps   xmmword ptr [rbp + 0x4a0], xmm0
0x140147e6a mov      r12, qword ptr [rcx + 0x60]
0x140147e6e mov      rax, qword ptr [rcx + 0x70]
0x140147e72 mov      r8d, dword ptr [rcx + 0x7c]
0x140147e76 movups   xmm0, xmmword ptr [rcx + 0x80]
0x140147e7d movups   xmm1, xmmword ptr [rcx + 0x90]
0x140147e84 movaps   xmmword ptr [rbp + 0x490], xmm1
0x140147e8b movaps   xmmword ptr [rbp + 0x480], xmm0
0x140147e92 movups   xmm0, xmmword ptr [rcx + 0xa0]
0x140147e99 movups   xmm1, xmmword ptr [rcx + 0xb0]
0x140147ea0 movaps   xmmword ptr [rbp + 0x470], xmm1
0x140147ea7 movaps   xmmword ptr [rbp + 0x460], xmm0
0x140147eae movups   xmm0, xmmword ptr [rcx + 0xc0]
0x140147eb5 movups   xmm1, xmmword ptr [rcx + 0xd0]
0x140147ebc movaps   xmmword ptr [rbp + 0x510], xmm1
0x140147ec3 movaps   xmmword ptr [rbp + 0x500], xmm0
0x140147eca movups   xmm0, xmmword ptr [rcx + 0xe0]
0x140147ed1 movups   xmm1, xmmword ptr [rcx + 0xf0]
0x140147ed8 movaps   xmmword ptr [rbp + 0x450], xmm1
0x140147edf movaps   xmmword ptr [rbp + 0x440], xmm0
0x140147ee6 movups   xmm0, xmmword ptr [rcx + 0x100]
0x140147eed movups   xmm1, xmmword ptr [rcx + 0x110]
0x140147ef4 movaps   xmmword ptr [rbp + 0x430], xmm1
0x140147efb movaps   xmmword ptr [rbp + 0x420], xmm0
0x140147f02 movups   xmm0, xmmword ptr [rcx + 0x120]
0x140147f09 movups   xmm1, xmmword ptr [rcx + 0x130]
0x140147f10 movaps   xmmword ptr [rbp + 0x410], xmm1
0x140147f17 movaps   xmmword ptr [rbp + 0x400], xmm0
0x140147f1e movups   xmm0, xmmword ptr [rcx + 0x140]
0x140147f25 movups   xmm1, xmmword ptr [rcx + 0x150]
0x140147f2c movaps   xmmword ptr [rbp + 0x4f0], xmm1
0x140147f33 movaps   xmmword ptr [rbp + 0x4e0], xmm0
0x140147f3a movups   xmm0, xmmword ptr [rcx + 0x160]
0x140147f41 movups   xmm1, xmmword ptr [rcx + 0x170]
0x140147f48 movaps   xmmword ptr [rbp + 0x3f0], xmm1
0x140147f4f movaps   xmmword ptr [rbp + 0x3e0], xmm0
0x140147f56 movups   xmm0, xmmword ptr [rcx + 0x1a0]
0x140147f5d movups   xmm1, xmmword ptr [rcx + 0x1b0]
0x140147f64 movaps   xmmword ptr [rbp + 0x3d0], xmm1
0x140147f6b movaps   xmmword ptr [rbp + 0x3c0], xmm0
0x140147f72 movups   xmm0, xmmword ptr [rcx + 0x1c0]
0x140147f79 movups   xmm1, xmmword ptr [rcx + 0x1d0]
0x140147f80 movaps   xmmword ptr [rbp + 0x3b0], xmm1
0x140147f87 movaps   xmmword ptr [rbp + 0x3a0], xmm0
0x140147f8e mov      rdi, qword ptr [rdx + 0xa0]
0x140147f95 movups   xmm0, xmmword ptr [rdx]
0x140147f98 movups   xmm1, xmmword ptr [rdx + 0x10]
0x140147f9c movaps   xmmword ptr [rbp + 0x390], xmm1
0x140147fa3 movaps   xmmword ptr [rbp + 0x380], xmm0
0x140147faa movups   xmm0, xmmword ptr [rdx + 0x20]
0x140147fae movups   xmm1, xmmword ptr [rdx + 0x30]
0x140147fb2 movaps   xmmword ptr [rbp + 0x370], xmm1
0x140147fb9 movaps   xmmword ptr [rbp + 0x360], xmm0
0x140147fc0 mov      r15, qword ptr [rdx + 0x40]
0x140147fc4 movups   xmm0, xmmword ptr [rdx + 0x60]
0x140147fc8 movups   xmm1, xmmword ptr [rdx + 0x70]
0x140147fcc movaps   xmmword ptr [rbp + 0x350], xmm1
0x140147fd3 movaps   xmmword ptr [rbp + 0x340], xmm0
0x140147fda movdqu   xmm0, xmmword ptr [rdx + 0x80]
0x140147fe2 movdqu   xmm1, xmmword ptr [rdx + 0x90]
0x140147fea movdqa   xmmword ptr [rbp + 0x330], xmm1
0x140147ff2 movdqa   xmmword ptr [rbp + 0x320], xmm0
0x140147ffa mov      rcx, qword ptr [rdx + 0xc0]
0x140148001 mov      qword ptr [rbp + 0x528], rcx
0x140148008 test     rdi, rdi
0x14014800b je       0x140148421
0x140148011 mov      rsi, r9
0x140148014 mov      dword ptr [rax], r8d
0x140148017 mov      ecx, dword ptr [r15 + 0xe0]
0x14014801e movd     xmm1, dword ptr [r15 + 0xe4]
0x140148027 call     0x14093fa90
0x14014802c mov      rcx, qword ptr [rbp + 0x530]
0x140148033 test     al, al
0x140148035 je       0x14014805e
0x140148037 mov      eax, dword ptr [rbp + 0x54c]
0x14014803d mov      rdx, qword ptr [rbp + 0x540]
0x140148044 mov      dword ptr [rdx], eax
0x140148046 movzx    eax, byte ptr [rcx + 0xf0]
0x14014804d xor      al, 1
0x14014804f mov      byte ptr [rcx + 0xf0], al
0x140148055 test     al, 1
0x140148057 jne      0x14014806d
0x140148059 jmp      0x140148299
0x14014805e movzx    eax, byte ptr [rcx + 0xf0]
0x140148065 test     al, 1
0x140148067 je       0x140148299
0x14014806d lea      rdx, [rip + 0x28090f2] ; @0x142951166 'HAAAAX(Experimental)Note: it only works with build mode camera.Reset'
0x140148074 lea      rcx, [rbp - 0x58]
0x140148078 mov      r8d, 6
0x14014807e call     0x140053bb0
0x140148083 mov      byte ptr [rbp + 0x55f], 1
0x14014808a mov      rcx, rdi
0x14014808d call     0x14067dd80
0x140148092 mov      r14, rax
0x140148095 lea      rcx, [rbp + 0x130]
0x14014809c lea      rdx, [rbp - 0x58]
0x1401480a0 mov      r8d, 0x188
0x1401480a6 call     0x142923c00
0x1401480ab movzx    eax, byte ptr [rip + 0x34ecb57]
0x1401480b2 mov      ecx, 0x108
0x1401480b7 mov      edx, 8
0x1401480bc call     0x140613c10
0x1401480c1 test     rax, rax
0x1401480c4 mov      qword ptr [rbp + 0x550], rax
0x1401480cb je       0x140148436
0x1401480d1 mov      rdx, qword ptr [rbp + 0x5f8]
0x1401480d8 mov      r10, qword ptr [rbp + 0x5e8]
0x1401480df mov      rcx, qword ptr [rbp + 0x5d8]
0x1401480e6 mov      r13, qword ptr [rbp + 0x5d0]
0x1401480ed lea      r8, [r12 + 0x1c]
0x1401480f2 lea      r9, [r12 + 0x18]
0x1401480f7 movq     xmm0, r12
0x1401480fc pshufd   xmm1, xmm0, 0x44
0x140148101 movdqa   xmm0, xmmword ptr [rip + 0x2808c87]
0x140148109 paddq    xmm0, xmm1
0x14014810d paddq    xmm1, xmmword ptr [rip + 0x2808c8b]
0x140148115 add      r12, 0x14
0x140148119 lea      r11, [rbp + 0x3c0]
0x140148120 mov      qword ptr [rax], r11
0x140148123 lea      r11, [rbp + 0x3a0]
0x14014812a mov      qword ptr [rax + 8], r11
0x14014812e lea      r11, [rbp + 0x5f0]
0x140148135 mov      qword ptr [rax + 0x10], r11
0x140148139 lea      r11, [rbp + 0x500]
0x140148140 mov      qword ptr [rax + 0x18], r11
0x140148144 lea      r11, [rbp + 0x400]
0x14014814b mov      qword ptr [rax + 0x20], r11
0x14014814f lea      r11, [rbp + 0x530]
0x140148156 mov      qword ptr [rax + 0x28], r11
0x14014815a lea      r11, [rbp + 0x340]
0x140148161 mov      qword ptr [rax + 0x30], r11
0x140148165 lea      r11, [rbp + 0x528]
0x14014816c mov      qword ptr [rax + 0x38], r11
0x140148170 lea      r11, [rbp + 0x320]
0x140148177 mov      qword ptr [rax + 0x40], r11
0x14014817b mov      qword ptr [rax + 0x48], r10
0x14014817f lea      r10, [rbp + 0x5e0]
0x140148186 mov      qword ptr [rax + 0x50], r10
0x14014818a lea      r10, [rbp + 0x4e0]
0x140148191 mov      qword ptr [rax + 0x58], r10
0x140148195 lea      r10, [rbp + 0x3e0]
0x14014819c mov      qword ptr [rax + 0x60], r10
0x1401481a0 lea      r10, [rbp + 0x440]
0x1401481a7 mov      qword ptr [rax + 0x68], r10
0x1401481ab movdqu   xmmword ptr [rax + 0x80], xmm1
0x1401481b3 movdqu   xmmword ptr [rax + 0x70], xmm0
0x1401481b8 mov      qword ptr [rax + 0x90], r12
0x1401481bf mov      qword ptr [rax + 0x98], r9
0x1401481c6 mov      qword ptr [rax + 0xa0], r8
0x1401481cd mov      qword ptr [rax + 0xa8], rbx
0x1401481d4 mov      qword ptr [rax + 0xb0], rdx
0x1401481db lea      rdx, [rbp + 0x480]
0x1401481e2 mov      qword ptr [rax + 0xb8], rdx
0x1401481e9 lea      rdx, [rbp + 0x460]
0x1401481f0 mov      qword ptr [rax + 0xc0], rdx
0x1401481f7 mov      qword ptr [rax + 0xc8], rcx
0x1401481fe lea      rcx, [rbp + 0x4c0]
0x140148205 mov      qword ptr [rax + 0xd0], rcx
0x14014820c lea      rcx, [rbp + 0x360]
0x140148213 mov      qword ptr [rax + 0xd8], rcx
0x14014821a lea      rcx, [rbp + 0x380]
0x140148221 mov      qword ptr [rax + 0xe0], rcx
0x140148228 mov      qword ptr [rax + 0xe8], rsi
0x14014822f mov      qword ptr [rax + 0xf0], r13
0x140148236 lea      rcx, [rbp + 0x420]
0x14014823d mov      qword ptr [rax + 0xf8], rcx
0x140148244 lea      rcx, [rbp + 0x4a0]
0x14014824b mov      qword ptr [rax + 0x100], rcx
0x140148252 lea      rcx, [rip + 0x2808c7f]
0x140148259 mov      qword ptr [rsp + 0x20], rcx
0x14014825e lea      rcx, [rbp + 0x2b8]
0x140148265 lea      rdx, [rbp + 0x130]
0x14014826c mov      r8, r14
0x14014826f mov      r9, rax
0x140148272 call     0x140053ee0
0x140148277 cmp      dword ptr [rbp + 0x2f8], 2
0x14014827e je       0x140148299
0x140148280 mov      rax, qword ptr [rbp + 0x2c8]
0x140148287 lock dec qword ptr [rax]
0x14014828b jne      0x140148299
0x14014828d lea      rcx, [rbp + 0x2c8]
0x140148294 call     0x140c41170
0x140148299 mov      ecx, dword ptr [r15 + 0x108]
0x1401482a0 movd     xmm1, dword ptr [r15 + 0x10c]
0x1401482a9 call     0x14093fa90
0x1401482ae test     al, al
0x1401482b0 je       0x1401482da
0x1401482b2 mov      rax, qword ptr [rbp + 0x530]
0x1401482b9 mov      rcx, qword ptr [rbp + 0x540]
0x1401482c0 movzx    edx, byte ptr [rax + 0x101]
0x1401482c7 mov      r8d, dword ptr [rbp + 0x54c]
0x1401482ce mov      dword ptr [rcx], r8d
0x1401482d1 xor      dl, 1
0x1401482d4 mov      byte ptr [rax + 0x101], dl
0x1401482da mov      rax, qword ptr [rbp + 0x530]
0x1401482e1 movzx    ecx, byte ptr [rax + 0x101]
0x1401482e8 test     cl, cl
0x1401482ea je       0x1401483c2
0x1401482f0 lea      rdx, [rip + 0x2808e75] ; @0x14295116c '(Experimental)Note: it only works with build mode camera.Reset'
0x1401482f7 lea      rcx, [rbp - 0x58]
0x1401482fb mov      r8d, 0xe
0x140148301 call     0x140053bb0
0x140148306 mov      byte ptr [rbp + 0x55e], 1
0x14014830d mov      rcx, rdi
0x140148310 call     0x14067dd80
0x140148315 mov      rbx, rax
0x140148318 lea      rcx, [rbp + 0x130]
0x14014831f lea      rdx, [rbp - 0x58]
0x140148323 mov      r8d, 0x188
0x140148329 call     0x142923c00
0x14014832e movzx    eax, byte ptr [rip + 0x34ec8d4]
0x140148335 mov      ecx, 0x18
0x14014833a mov      edx, 8
0x14014833f call     0x140613c10
0x140148344 test     rax, rax
0x140148347 mov      qword ptr [rbp + 0x550], rax
0x14014834e je       0x140148447
0x140148354 mov      r9, rax
0x140148357 lea      rax, [rbp + 0x500]
0x14014835e mov      qword ptr [r9], rax
0x140148361 lea      rax, [rbp + 0x530]
0x140148368 mov      qword ptr [r9 + 8], rax
0x14014836c mov      qword ptr [r9 + 0x10], rsi
0x140148370 lea      rax, [rip + 0x2808b41]
0x140148377 mov      qword ptr [rsp + 0x20], rax
0x14014837c lea      rcx, [rbp + 0x2b8]
0x140148383 lea      rdx, [rbp + 0x130]
0x14014838a mov      r8, rbx
0x14014838d call     0x140053ee0
0x140148392 cmp      dword ptr [rbp + 0x2f8], 2
0x140148399 je       0x1401483b4
0x14014839b mov      rax, qword ptr [rbp + 0x2c8]
0x1401483a2 lock dec qword ptr [rax]
0x1401483a6 jne      0x1401483b4
0x1401483a8 lea      rcx, [rbp + 0x2c8]
0x1401483af call     0x140c41170
0x1401483b4 mov      rax, qword ptr [rbp + 0x530]
0x1401483bb movzx    ecx, byte ptr [rax + 0x101]
0x1401483c2 test     cl, 1
0x1401483c5 jne      0x1401483d0
0x1401483c7 cmp      byte ptr [rax + 0xf0], 0
0x1401483ce je       0x140148421
0x1401483d0 mov      rcx, qword ptr [rbp + 0x4e0]
0x1401483d7 call     0x140987ca0
0x1401483dc mov      rsi, rax
0x1401483df mov      rbx, qword ptr [rbp + 0x5e0]
0x1401483e6 mov      rcx, rdi
0x1401483e9 call     0x14067dd80
0x1401483ee lea      rcx, [rbx + 0x48]
0x1401483f2 mov      qword ptr [rbp + 0x130], rbx
0x1401483f9 add      rbx, 0x49
0x1401483fd mov      qword ptr [rbp + 0x138], rcx
0x140148404 mov      qword ptr [rbp + 0x140], rbx
0x14014840b mov      qword ptr [rbp + 0x148], rax
0x140148412 lea      rdx, [rbp + 0x130]
0x140148419 mov      rcx, rsi
0x14014841c call     0x1405b9810
0x140148421 nop      
0x140148422 add      rsp, 0x5e8
0x140148429 pop      rbx
0x14014842a pop      rdi
0x14014842b pop      rsi
0x14014842c pop      r12
0x14014842e pop      r13
0x140148430 pop      r14
0x140148432 pop      r15
0x140148434 pop      rbp
0x140148435 ret      
0x140148436 mov      ecx, 8
0x14014843b mov      edx, 0x108
0x140148440 call     0x1428d9000
0x140148445 jmp      0x140148456
0x140148447 mov      ecx, 8
0x14014844c mov      edx, 0x18