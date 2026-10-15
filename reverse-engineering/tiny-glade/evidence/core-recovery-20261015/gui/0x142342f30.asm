0x142342f30 push     r15
0x142342f32 push     r14
0x142342f34 push     r13
0x142342f36 push     r12
0x142342f38 push     rsi
0x142342f39 push     rdi
0x142342f3a push     rbp
0x142342f3b push     rbx
0x142342f3c sub      rsp, 0x88
0x142342f43 movaps   xmmword ptr [rsp + 0x70], xmm6
0x142342f48 mov      rbp, qword ptr [rdx]
0x142342f4b mov      r11, qword ptr [rdx + 8]
0x142342f4f mov      rdi, qword ptr [rbp]
0x142342f53 xor      esi, esi
0x142342f55 mov      rax, rdi
0x142342f58 sub      rax, qword ptr [r11 + 0x18]
0x142342f5c cmovb    rax, rsi
0x142342f60 sub      rdi, qword ptr [r11 + 0x38]
0x142342f64 mov      r10, qword ptr [rsp + 0xf8]
0x142342f6c cmovb    rdi, rsi
0x142342f70 mov      rdx, rax
0x142342f73 shl      rdx, 4
0x142342f77 add      rdx, qword ptr [r11 + 8]
0x142342f7b mov      r12, qword ptr [r11 + 0x10]
0x142342f7f mov      r13, qword ptr [r11 + 0x30]
0x142342f83 sub      r12, rax
0x142342f86 cmovb    r12, rsi
0x142342f8a mov      ebx, 8
0x142342f8f cmovb    rdx, rbx
0x142342f93 mov      rax, rdi
0x142342f96 shl      rax, 4
0x142342f9a add      rax, qword ptr [r11 + 0x28]
0x142342f9e sub      r13, rdi
0x142342fa1 cmovb    r13, rsi
0x142342fa5 cmovb    rax, rbx
0x142342fa9 mov      r11, qword ptr [r11 + 0x40]
0x142342fad sub      r11, r13
0x142342fb0 sub      r11, r12
0x142342fb3 mov      qword ptr [rbp], r11
0x142342fb7 shl      r12, 4
0x142342fbb shl      r13, 4
0x142342fbf mov      r11, qword ptr [r10 + 8]
0x142342fc3 mov      qword ptr [rsp + 0x30], r11
0x142342fc8 test     r11, r11
0x142342fcb jne      0x142342fd6
0x142342fcd mov      r10, qword ptr [r10 + 0x10]
0x142342fd1 mov      qword ptr [rsp + 0x30], r10
0x142342fd6 add      r12, rdx
0x142342fd9 add      r13, rax
0x142342fdc mov      r11, qword ptr [rcx]
0x142342fdf mov      r14, qword ptr [rcx + 8]
0x142342fe3 mov      ecx, dword ptr [rcx + 0x14]
0x142342fe6 mov      dword ptr [rsp + 0x48], ecx
0x142342fea mov      rsi, qword ptr [r8]
0x142342fed mov      rcx, qword ptr [r8 + 0x10]
0x142342ff1 mov      qword ptr [rsp + 0x50], rcx
0x142342ff6 mov      ecx, dword ptr [r8 + 0x1c]
0x142342ffa mov      dword ptr [rsp + 0x3c], ecx
0x142342ffe mov      ecx, dword ptr [r9 + 0x1c]
0x142343002 mov      dword ptr [rsp + 0x4c], ecx
0x142343006 mov      rcx, qword ptr [r9]
0x142343009 mov      qword ptr [rsp + 0x68], rcx
0x14234300e mov      rcx, qword ptr [r9 + 0x10]
0x142343012 mov      qword ptr [rsp + 0x60], rcx
0x142343017 mov      edi, 0xffffffff
0x14234301c movsd    xmm6, qword ptr [rip + 0xd0b08c]
0x142343024 mov      qword ptr [rsp + 0x40], r11
0x142343029 nop      dword ptr [rax]
0x142343030 test     rdx, rdx
0x142343033 je       0x142343050
0x142343035 cmp      rdx, r12
0x142343038 je       0x142343050
0x14234303a lea      r15, [rdx + 0x10]
0x14234303e mov      rbx, rax
0x142343041 mov      rcx, rdx
0x142343044 jmp      0x14234306f ; 
0x142343046 nop      word ptr cs:[rax + rax]
0x142343050 test     rax, rax
0x142343053 je       0x1423431b9
0x142343059 cmp      rax, r13
0x14234305c je       0x1423431b9
0x142343062 lea      rbx, [rax + 0x10]
0x142343066 xor      r15d, r15d
0x142343069 mov      rcx, rax
0x14234306c mov      rax, rbx
0x14234306f inc      qword ptr [rbp]
0x142343073 mov      r8, qword ptr [rcx + 8]
0x142343077 mov      r9d, r8d
0x14234307a mov      rdx, r15
0x14234307d cmp      r9, qword ptr [r14 + 0x10]
0x142343081 jae      0x142343030
0x142343083 mov      r10, qword ptr [r14 + 8]
0x142343087 shr      r8, 0x20
0x14234308b lea      r9, [r9 + r9*4]
0x14234308f mov      rax, rbx
0x142343092 mov      rdx, r15
0x142343095 cmp      dword ptr [r10 + r9*4], r8d
0x142343099 jne      0x142343030
0x14234309b lea      rax, [r10 + r9*4]
0x14234309f mov      r8d, dword ptr [rax + 4]
0x1423430a3 mov      rax, rbx
0x1423430a6 mov      rdx, r15
0x1423430a9 cmp      r8, rdi
0x1423430ac je       0x142343030
0x1423430ae mov      rax, rbx
0x1423430b1 mov      rdx, r15
0x1423430b4 cmp      qword ptr [r11 + 0x38], r8
0x1423430b8 jbe      0x142343030
0x1423430be mov      rax, qword ptr [r11 + 0x28]
0x1423430c2 mov      edx, r8d
0x1423430c5 shr      edx, 6
0x1423430c8 mov      rax, qword ptr [rax + rdx*8]
0x1423430cc bt       rax, r8
0x1423430d0 mov      rax, rbx
0x1423430d3 mov      rdx, r15
0x1423430d6 jae      0x142343030
0x1423430dc movzx    eax, byte ptr [rsi + 0xa]
0x1423430e0 mov      rdx, qword ptr [rsp + 0x50]
0x1423430e5 mov      r8d, dword ptr [rsp + 0x3c]
0x1423430ea mov      dword ptr [rdx], r8d
0x1423430ed xor      al, 1
0x1423430ef mov      byte ptr [rsi + 0xa], al
0x1423430f2 mov      r8, qword ptr [rcx + 8]
0x1423430f6 mov      rcx, qword ptr [rsp + 0x30]
0x1423430fb mov      rax, qword ptr [rcx]
0x1423430fe mov      rdi, qword ptr [rcx + 0x10]
0x142343102 sub      rax, rdi
0x142343105 cmp      rax, 0x27
0x142343109 jbe      0x14234317e
0x14234310b mov      rax, qword ptr [rcx + 8]
0x14234310f lea      rdx, [rip - 0x1363f16] ; @0x140fdf200 'VWH'
0x142343116 mov      qword ptr [rax + rdi], rdx
0x14234311a lea      rdx, [rip - 0x4f38c1]
0x142343121 mov      qword ptr [rax + rdi + 8], rdx
0x142343126 mov      qword ptr [rax + rdi + 0x10], r8
0x14234312b movsd    qword ptr [rax + rdi + 0x18], xmm6
0x142343131 mov      dword ptr [rax + rdi + 0x20], 0
0x142343139 mov      byte ptr [rax + rdi + 0x24], 0
0x14234313e add      rdi, 0x28
0x142343142 mov      qword ptr [rcx + 0x10], rdi
0x142343146 cmp      byte ptr [rsi + 0xa], 1
0x14234314a mov      rax, rbx
0x14234314d mov      rdx, r15
0x142343150 mov      edi, 0xffffffff
0x142343155 jne      0x142343030
0x14234315b mov      eax, dword ptr [rsp + 0x4c]
0x14234315f mov      rcx, qword ptr [rsp + 0x60]
0x142343164 mov      dword ptr [rcx], eax
0x142343166 mov      rax, qword ptr [rsp + 0x68]
0x14234316b mov      qword ptr [rax + 0x10], 0
0x142343173 mov      rax, rbx
0x142343176 mov      rdx, r15
0x142343179 jmp      0x142343030 ; 
0x14234317e mov      qword ptr [rsp + 0x20], 1
0x142343187 mov      qword ptr [rsp + 0x58], r8
0x14234318c mov      r8d, 0x28
0x142343192 mov      r9d, 1
0x142343198 mov      rcx, qword ptr [rsp + 0x30]
0x14234319d mov      rdx, rdi
0x1423431a0 call     0x1428a58d0 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hbdbbfc746df8f5db
0x1423431a5 mov      r8, qword ptr [rsp + 0x58]
0x1423431aa mov      rcx, qword ptr [rsp + 0x30]
0x1423431af mov      r11, qword ptr [rsp + 0x40]
0x1423431b4 jmp      0x14234310b ; 
0x1423431b9 mov      qword ptr [rsp + 0x30], rsi
0x1423431be mov      rax, qword ptr [rsp + 0xf0]
0x1423431c6 mov      r9, qword ptr [rax]
0x1423431c9 mov      r10, qword ptr [rax + 8]
0x1423431cd mov      rax, qword ptr [r9]
0x1423431d0 xor      ebx, ebx
0x1423431d2 mov      rdx, rax
0x1423431d5 sub      rdx, qword ptr [r10 + 0x18]
0x1423431d9 cmovb    rdx, rbx
0x1423431dd sub      rax, qword ptr [r10 + 0x38]
0x1423431e1 cmovb    rax, rbx
0x1423431e5 mov      r11, qword ptr [r10 + 0x10]
0x1423431e9 mov      r8, qword ptr [r10 + 0x30]
0x1423431ed mov      rcx, r8
0x1423431f0 sub      rcx, rax
0x1423431f3 cmovb    rcx, rbx
0x1423431f7 setbe    bpl
0x1423431fb mov      rsi, r11
0x1423431fe sub      rsi, rdx
0x142343201 cmovb    rsi, rbx
0x142343205 mov      rdi, qword ptr [r10 + 8]
0x142343209 mov      r15, qword ptr [r10 + 0x40]
0x14234320d setbe    r12b
0x142343211 lea      r13, [rcx + rsi]
0x142343215 test     r12b, bpl
0x142343218 cmove    r13, rbx
0x14234321c sub      r15, r13
0x14234321f mov      r10, qword ptr [r10 + 0x28]
0x142343223 mov      qword ptr [r9], r15
0x142343226 test     r12b, bpl
0x142343229 jne      0x142343290
0x14234322b xor      r9d, r9d
0x14234322e cmp      r11, rdx
0x142343231 lea      rdx, [rdx + rdx*4]
0x142343235 lea      rdx, [rdi + rdx*8]
0x142343239 mov      r11d, 8
0x14234323f cmovb    rdx, r11
0x142343243 lea      rsi, [rsi + rsi*4]
0x142343247 lea      rdx, [rdx + rsi*8 - 0x28]
0x14234324c cmova    r9, rdx
0x142343250 cmp      r8, rax
0x142343253 lea      rax, [rax + rax*4]
0x142343257 lea      rax, [r10 + rax*8]
0x14234325b cmovb    rax, r11
0x14234325f lea      rcx, [rcx + rcx*4]
0x142343263 lea      rax, [rax + rcx*8 - 0x28]
0x142343268 cmovbe   rax, r9
0x14234326c cmp      dword ptr [rax], 0xd
0x14234326f je       0x142343290
0x142343271 mov      rax, qword ptr [rsp + 0x30]
0x142343276 cmp      byte ptr [rax + 0xa], 1
0x14234327a jne      0x142343290
0x14234327c mov      rax, qword ptr [rsp + 0x50]
0x142343281 mov      ecx, dword ptr [rsp + 0x3c]
0x142343285 mov      dword ptr [rax], ecx
0x142343287 mov      rax, qword ptr [rsp + 0x30]
0x14234328c mov      byte ptr [rax + 0xa], 0
0x142343290 mov      rdx, qword ptr [rsp + 0x40]
0x142343295 mov      rax, qword ptr [rdx + 0xf8]
0x14234329c mov      rcx, qword ptr [rdx + 0x100]
0x1423432a3 lea      rcx, [rax + rcx*4]
0x1423432a7 cmp      byte ptr [rdx + 0x108], 0
0x1423432ae mov      edi, dword ptr [rsp + 0x48]
0x1423432b2 je       0x1423432c3
0x1423432b4 xor      r8d, r8d
0x1423432b7 xor      edx, edx
0x1423432b9 xor      r10d, r10d
0x1423432bc mov      rsi, qword ptr [rsp + 0x30]
0x1423432c1 jmp      0x14234332a ; 
0x1423432c3 mov      r9d, 8
0x1423432c9 xor      r8d, r8d
0x1423432cc xor      edx, edx
0x1423432ce xor      r11d, r11d
0x1423432d1 jmp      0x1423433df ; 
0x1423432d6 nop      word ptr cs:[rax + rax]
0x1423432e0 lea      r8, [r8 + r9*8]
0x1423432e4 mov      r9, qword ptr [r10 + 0x118]
0x1423432eb mov      r10, qword ptr [r8 + 0x18]
0x1423432ef mov      r8, qword ptr [r8 + 0x38]
0x1423432f3 mov      r8, qword ptr [r8 + r9*8]
0x1423432f7 not      r8
0x1423432fa lea      r9, [r8 + r8*2]
0x1423432fe shl      r9, 4
0x142343302 mov      r8, qword ptr [r10 + r9 + 0x10]
0x142343307 mov      r9, qword ptr [r10 + r9 + 0x28]
0x14234330c add      rax, 4
0x142343310 xor      r10d, r10d
0x142343313 mov      r11d, r10d
0x142343316 inc      r10
0x142343319 movzx    ebx, byte ptr [rsi + 0xa]
0x14234331d mov      dword ptr [r9 + r11*4], edi
0x142343321 lea      r11, [r11 + r11*2]
0x142343325 mov      byte ptr [r8 + r11 + 2], bl
0x14234332a cmp      r10, rdx
0x14234332d jne      0x142343313
0x14234332f cmp      rax, rcx
0x142343332 mov      r10, qword ptr [rsp + 0x40]
0x142343337 je       0x142343421
0x14234333d mov      r8, qword ptr [r14 + 0x1a8]
0x142343344 nop      word ptr cs:[rax + rax]
0x142343350 mov      edx, dword ptr [rax]
0x142343352 lea      r9, [rdx + rdx*8]
0x142343356 mov      rdx, qword ptr [r8 + r9*8 + 0x10]
0x14234335b test     rdx, rdx
0x14234335e jne      0x1423432e0
0x142343360 add      rax, 4
0x142343364 cmp      rax, rcx
0x142343367 jne      0x142343350
0x142343369 jmp      0x142343421 ; 
0x14234336e nop      
0x142343370 add      r9, r8
0x142343373 mov      r8d, dword ptr [r9 + 0x94]
0x14234337a mov      r10, qword ptr [r14 + 0x1a8]
0x142343381 lea      r8, [r8 + r8*8]
0x142343385 mov      r11, qword ptr [r11 + 0x118]
0x14234338c mov      rsi, qword ptr [r10 + r8*8 + 0x18]
0x142343391 mov      r8, qword ptr [r10 + r8*8 + 0x38]
0x142343396 mov      r8, qword ptr [r8 + r11*8]
0x14234339a not      r8
0x14234339d lea      r10, [r8 + r8*2]
0x1423433a1 shl      r10, 4
0x1423433a5 mov      r8, qword ptr [rsi + r10 + 0x10]
0x1423433aa mov      r10, qword ptr [rsi + r10 + 0x28]
0x1423433af mov      r9, qword ptr [r9 + 0x50]
0x1423433b3 add      rax, 4
0x1423433b7 xor      r11d, r11d
0x1423433ba mov      rsi, r11
0x1423433bd shl      rsi, 4
0x1423433c1 mov      esi, dword ptr [r9 + rsi + 8]
0x1423433c6 inc      r11
0x1423433c9 mov      rbx, qword ptr [rsp + 0x30]
0x1423433ce movzx    ebx, byte ptr [rbx + 0xa]
0x1423433d2 mov      dword ptr [r10 + rsi*4], edi
0x1423433d6 lea      rsi, [rsi + rsi*2]
0x1423433da mov      byte ptr [r8 + rsi + 2], bl
0x1423433df cmp      r11, rdx
0x1423433e2 jne      0x1423433ba
0x1423433e4 cmp      rax, rcx
0x1423433e7 mov      r11, qword ptr [rsp + 0x40]
0x1423433ec je       0x142343421
0x1423433ee mov      r9, qword ptr [r14 + 0x100]
0x1423433f5 nop      word ptr cs:[rax + rax]
0x142343400 mov      edx, dword ptr [rax]
0x142343402 lea      r8, [rdx + rdx*4]
0x142343406 shl      r8, 5
0x14234340a mov      rdx, qword ptr [r9 + r8 + 0x58]
0x14234340f test     rdx, rdx
0x142343412 jne      0x142343370
0x142343418 add      rax, 4
0x14234341c cmp      rax, rcx
0x14234341f jne      0x142343400
0x142343421 movaps   xmm6, xmmword ptr [rsp + 0x70]
0x142343426 add      rsp, 0x88
0x14234342d pop      rbx
0x14234342e pop      rbp
0x14234342f pop      rdi
0x142343430 pop      rsi
0x142343431 pop      r12
0x142343433 pop      r13
0x142343435 pop      r14
0x142343437 pop      r15
0x142343439 ret      