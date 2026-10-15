0x1408b6250 push     rbp
0x1408b6251 push     rsi
0x1408b6252 sub      rsp, 0xa8
0x1408b6259 lea      rbp, [rsp + 0x80]
0x1408b6261 mov      qword ptr [rbp + 0x20], 0xfffffffffffffffe
0x1408b6269 mov      rsi, rcx
0x1408b626c cmp      dword ptr [rcx + 0x70], 1
0x1408b6270 jne      0x1408b6292
0x1408b6272 lea      rdx, [rsi + 0x78]
0x1408b6276 lea      rcx, [rbp - 0x58]
0x1408b627a xor      r8d, r8d
0x1408b627d call     0x141437440 ; libfmod::EventInstance::stop::h6aa257fdd68e9145
0x1408b6282 movabs   rax, 0x8000000000000004
0x1408b628c cmp      qword ptr [rbp - 0x58], rax
0x1408b6290 jne      0x1408b62cb
0x1408b6292 cmp      dword ptr [rsi + 0xa0], 1
0x1408b6299 jne      0x1408b62c1
0x1408b629b add      rsi, 0xa8
0x1408b62a2 lea      rcx, [rbp - 0x58]
0x1408b62a6 mov      rdx, rsi
0x1408b62a9 xor      r8d, r8d
0x1408b62ac call     0x141437440 ; libfmod::EventInstance::stop::h6aa257fdd68e9145
0x1408b62b1 movabs   rax, 0x8000000000000004
0x1408b62bb cmp      qword ptr [rbp - 0x58], rax
0x1408b62bf jne      0x1408b6315
0x1408b62c1 add      rsp, 0xa8
0x1408b62c8 pop      rsi
0x1408b62c9 pop      rbp
0x1408b62ca ret      
0x1408b62cb mov      rax, qword ptr [rbp - 0x28]
0x1408b62cf mov      qword ptr [rbp + 0x10], rax
0x1408b62d3 movups   xmm0, xmmword ptr [rbp - 0x58]
0x1408b62d7 movups   xmm1, xmmword ptr [rbp - 0x48]
0x1408b62db movups   xmm2, xmmword ptr [rbp - 0x38]
0x1408b62df movaps   xmmword ptr [rbp], xmm2
0x1408b62e3 movaps   xmmword ptr [rbp - 0x10], xmm1
0x1408b62e7 movaps   xmmword ptr [rbp - 0x20], xmm0
0x1408b62eb lea      rax, [rip + 0x221c916]
0x1408b62f2 mov      qword ptr [rsp + 0x20], rax
0x1408b62f7 lea      rcx, [rip + 0x221a302] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x1408b62fe lea      r9, [rip + 0x221a2bb]
0x1408b6305 lea      r8, [rbp - 0x20]
0x1408b6309 mov      edx, 0x2b
0x1408b630e call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x1408b6313 jmp      0x1408b635d ; 
0x1408b6315 mov      rax, qword ptr [rbp - 0x28]
0x1408b6319 mov      qword ptr [rbp + 0x10], rax
0x1408b631d movups   xmm0, xmmword ptr [rbp - 0x58]
0x1408b6321 movups   xmm1, xmmword ptr [rbp - 0x48]
0x1408b6325 movups   xmm2, xmmword ptr [rbp - 0x38]
0x1408b6329 movaps   xmmword ptr [rbp], xmm2
0x1408b632d movaps   xmmword ptr [rbp - 0x10], xmm1
0x1408b6331 movaps   xmmword ptr [rbp - 0x20], xmm0
0x1408b6335 lea      rax, [rip + 0x221c8cc]
0x1408b633c mov      qword ptr [rsp + 0x20], rax
0x1408b6341 lea      rcx, [rip + 0x221a2b8] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x1408b6348 lea      r9, [rip + 0x221a271]
0x1408b634f lea      r8, [rbp - 0x20]
0x1408b6353 mov      edx, 0x2b
0x1408b6358 call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x1408b635d ud2      
0x1408b635f nop      
0x1408b6360 mov      qword ptr [rsp + 0x10], rdx
0x1408b6365 push     rbp
0x1408b6366 push     rsi
0x1408b6367 sub      rsp, 0x28
0x1408b636b lea      rbp, [rdx + 0x80]
0x1408b6372 lea      rcx, [rbp - 0x20]
0x1408b6376 call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x1408b637b nop      
0x1408b637c add      rsp, 0x28
0x1408b6380 pop      rsi
0x1408b6381 pop      rbp
0x1408b6382 ret      
0x1408b6383 nop      word ptr cs:[rax + rax]
0x1408b6390 mov      qword ptr [rsp + 0x10], rdx
0x1408b6395 push     rbp
0x1408b6396 push     rsi
0x1408b6397 sub      rsp, 0x28
0x1408b639b lea      rbp, [rdx + 0x80]
0x1408b63a2 lea      rcx, [rbp - 0x20]
0x1408b63a6 call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x1408b63ab nop      
0x1408b63ac add      rsp, 0x28
0x1408b63b0 pop      rsi
0x1408b63b1 pop      rbp
0x1408b63b2 ret      