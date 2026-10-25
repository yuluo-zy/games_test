0x140a47200 push     rbp
0x140a47201 push     rsi
0x140a47202 push     rdi
0x140a47203 sub      rsp, 0xd0
0x140a4720a lea      rbp, [rsp + 0x80]
0x140a47212 mov      qword ptr [rbp + 0x48], 0xfffffffffffffffe
0x140a4721a mov      rsi, rcx
0x140a4721d movzx    eax, byte ptr [rdx]
0x140a47220 add      al, 0xc8
0x140a47222 cmp      al, 8
0x140a47224 movzx    eax, al
0x140a47227 mov      ecx, 6
0x140a4722c cmovb    ecx, eax
0x140a4722f movzx    eax, cl
0x140a47232 lea      rcx, [rip + 0x20884cf]
0x140a47239 movsxd   rax, dword ptr [rcx + rax*4]
0x140a4723d add      rax, rcx
0x140a47240 jmp      rax
0x140a47242 movzx    eax, byte ptr [rip + 0x2bed9c0]
0x140a47249 mov      ecx, 0xd
0x140a4724e mov      edx, 1
0x140a47253 call     0x140613c10 ; __rust_alloc
0x140a47258 test     rax, rax
0x140a4725b je       0x140a475ae
0x140a47261 movabs   rcx, 0x776f646e69576567
0x140a4726b mov      qword ptr [rax + 5], rcx
0x140a4726f movabs   rcx, 0x5765676174746f43
0x140a47279 mov      qword ptr [rax], rcx
0x140a4727c mov      byte ptr [rsi], 3
0x140a4727f mov      qword ptr [rsi + 8], 0xd
0x140a47287 mov      qword ptr [rsi + 0x10], rax
0x140a4728b mov      qword ptr [rsi + 0x18], 0xd
0x140a47293 jmp      0x140a4755e ; 
0x140a47298 movzx    eax, byte ptr [rip + 0x2bed96a]
0x140a4729f mov      ecx, 7
0x140a472a4 mov      edx, 1
0x140a472a9 call     0x140613c10 ; __rust_alloc
0x140a472ae test     rax, rax
0x140a472b1 je       0x140a4756c
0x140a472b7 mov      dword ptr [rax + 3], 0x6e726574
0x140a472be mov      dword ptr [rax], 0x746e614c
0x140a472c4 jmp      0x140a4733e ; 
0x140a472c6 movzx    eax, byte ptr [rip + 0x2bed93c]
0x140a472cd mov      ecx, 9
0x140a472d2 mov      edx, 1
0x140a472d7 call     0x140613c10 ; __rust_alloc
0x140a472dc test     rax, rax
0x140a472df je       0x140a47582
0x140a472e5 movabs   rcx, 0x696c53776f727241
0x140a472ef mov      qword ptr [rax], rcx
0x140a472f2 mov      byte ptr [rax + 8], 0x74
0x140a472f6 mov      byte ptr [rsi], 3
0x140a472f9 mov      qword ptr [rsi + 8], 9
0x140a47301 mov      qword ptr [rsi + 0x10], rax
0x140a47305 mov      qword ptr [rsi + 0x18], 9
0x140a4730d jmp      0x140a4755e ; 
0x140a47312 movzx    eax, byte ptr [rip + 0x2bed8f0]
0x140a47319 mov      ecx, 7
0x140a4731e mov      edx, 1
0x140a47323 call     0x140613c10 ; __rust_alloc
0x140a47328 test     rax, rax
0x140a4732b je       0x140a4756c
0x140a47331 mov      dword ptr [rax + 3], 0x79656e6d
0x140a47338 mov      dword ptr [rax], 0x6d696843
0x140a4733e mov      byte ptr [rsi], 3
0x140a47341 mov      qword ptr [rsi + 8], 7
0x140a47349 mov      qword ptr [rsi + 0x10], rax
0x140a4734d mov      qword ptr [rsi + 0x18], 7
0x140a47355 jmp      0x140a4755e ; 
0x140a4735a movzx    eax, byte ptr [rip + 0x2bed8a8]
0x140a47361 mov      ecx, 6
0x140a47366 mov      edx, 1
0x140a4736b call     0x140613c10 ; __rust_alloc
0x140a47370 test     rax, rax
0x140a47373 je       0x140a47598
0x140a47379 mov      word ptr [rax + 4], 0x7372
0x140a4737f mov      dword ptr [rax], 0x69617453
0x140a47385 mov      byte ptr [rsi], 3
0x140a47388 mov      qword ptr [rsi + 8], 6
0x140a47390 mov      qword ptr [rsi + 0x10], rax
0x140a47394 mov      qword ptr [rsi + 0x18], 6
0x140a4739c jmp      0x140a4755e ; 
0x140a473a1 movzx    eax, byte ptr [rip + 0x2bed861]
0x140a473a8 mov      ecx, 0xc
0x140a473ad mov      edx, 1
0x140a473b2 call     0x140613c10 ; __rust_alloc
0x140a473b7 test     rax, rax
0x140a473ba je       0x140a475c4
0x140a473c0 movabs   rcx, 0x6957636968746f47
0x140a473ca mov      qword ptr [rax], rcx
0x140a473cd mov      dword ptr [rax + 8], 0x776f646e
0x140a473d4 mov      byte ptr [rsi], 3
0x140a473d7 mov      qword ptr [rsi + 8], 0xc
0x140a473df mov      qword ptr [rsi + 0x10], rax
0x140a473e3 mov      qword ptr [rsi + 0x18], 0xc
0x140a473eb jmp      0x140a4755e ; 
0x140a473f0 movzx    eax, byte ptr [rip + 0x2bed812]
0x140a473f7 mov      ecx, 4
0x140a473fc mov      edx, 1
0x140a47401 call     0x140613c10 ; __rust_alloc
0x140a47406 test     rax, rax
0x140a47409 je       0x140a475da
0x140a4740f mov      dword ptr [rax], 0x67616c46
0x140a47415 mov      byte ptr [rsi], 3
0x140a47418 mov      qword ptr [rsi + 8], 4
0x140a47420 mov      qword ptr [rsi + 0x10], rax
0x140a47424 mov      qword ptr [rsi + 0x18], 4
0x140a4742c jmp      0x140a4755e ; 
0x140a47431 mov      rdi, rdx
0x140a47434 lea      rax, [rip + 0x2087945] ; @0x142aced80 'CluttertyStairsDecoratorType'
0x140a4743b mov      qword ptr [rsp + 0x20], rax
0x140a47440 mov      qword ptr [rsp + 0x30], 1
0x140a47449 mov      qword ptr [rsp + 0x28], 7
0x140a47452 lea      rdx, [rip + 0x2087936] ; @0x142aced8f 'DecoratorType'
0x140a47459 lea      rcx, [rbp - 0x40]
0x140a4745d mov      r8d, 0xd
0x140a47463 mov      r9d, 6
0x140a47469 call     0x140b86430 ; _$LT$serde_json..value..ser..Serializer$u20$as$u20$serde..ser..Serializer$GT$::serialize_struct_variant::hee65720d0920d233
0x140a4746e mov      rcx, qword ptr [rbp - 0x40]
0x140a47472 mov      rax, qword ptr [rbp - 0x38]
0x140a47476 mov      rdx, rcx
0x140a47479 neg      rdx
0x140a4747c jno      0x140a4748a
0x140a4747e mov      qword ptr [rsi + 8], rax
0x140a47482 mov      byte ptr [rsi], 6
0x140a47485 jmp      0x140a4755e ; 
0x140a4748a movups   xmm0, xmmword ptr [rbp - 0x30]
0x140a4748e movups   xmm1, xmmword ptr [rbp - 0x20]
0x140a47492 movups   xmmword ptr [rbp + 0x30], xmm1
0x140a47496 movups   xmmword ptr [rbp + 0x20], xmm0
0x140a4749a mov      qword ptr [rbp + 0x10], rcx
0x140a4749e mov      qword ptr [rbp + 0x18], rax
0x140a474a2 mov      byte ptr [rbp + 0x47], 1
0x140a474a6 lea      rdx, [rip + 0x20878da] ; @0x142aced87 'tyStairsDecoratorType'
0x140a474ad lea      rcx, [rbp + 0x10]
0x140a474b1 mov      r8d, 2
0x140a474b7 mov      r9, rdi
0x140a474ba call     0x14158e3c0 ; _$LT$serde_json..value..ser..SerializeStructVariant$u20$as$u20$serde..ser..SerializeStructVariant$GT$::serialize_field::h9e497d2814f09596
0x140a474bf test     rax, rax
0x140a474c2 je       0x140a4751b
0x140a474c4 mov      qword ptr [rsi + 8], rax
0x140a474c8 mov      byte ptr [rsi], 6
0x140a474cb mov      rdx, qword ptr [rbp + 0x10]
0x140a474cf test     rdx, rdx
0x140a474d2 je       0x140a474e3
0x140a474d4 mov      rcx, qword ptr [rbp + 0x18]
0x140a474d8 mov      r8d, 1
0x140a474de call     0x140613c20 ; __rust_dealloc
0x140a474e3 mov      rcx, qword ptr [rbp + 0x28]
0x140a474e7 test     rcx, rcx
0x140a474ea je       0x140a47545
0x140a474ec mov      rdx, qword ptr [rbp + 0x30]
0x140a474f0 mov      rax, qword ptr [rbp + 0x38]
0x140a474f4 mov      qword ptr [rbp - 0x38], 0
0x140a474fc mov      qword ptr [rbp - 0x30], rcx
0x140a47500 mov      qword ptr [rbp - 0x28], rdx
0x140a47504 mov      qword ptr [rbp - 0x18], 0
0x140a4750c mov      qword ptr [rbp - 0x10], rcx
0x140a47510 mov      qword ptr [rbp - 8], rdx
0x140a47514 mov      ecx, 1
0x140a47519 jmp      0x140a47549 ; 
0x140a4751b movups   xmm0, xmmword ptr [rbp + 0x10]
0x140a4751f movups   xmm1, xmmword ptr [rbp + 0x20]
0x140a47523 movups   xmm2, xmmword ptr [rbp + 0x30]
0x140a47527 movaps   xmmword ptr [rbp - 0x20], xmm2
0x140a4752b movaps   xmmword ptr [rbp - 0x30], xmm1
0x140a4752f movaps   xmmword ptr [rbp - 0x40], xmm0
0x140a47533 mov      byte ptr [rbp + 0x47], 0
0x140a47537 lea      rdx, [rbp - 0x40]
0x140a4753b mov      rcx, rsi
0x140a4753e call     0x140b86640 ; _$LT$serde_json..value..ser..SerializeStructVariant$u20$as$u20$serde..ser..SerializeStructVariant$GT$::end::h41c7b7ccbcbe64b4
0x140a47543 jmp      0x140a4755e ; 
0x140a47545 xor      ecx, ecx
0x140a47547 xor      eax, eax
0x140a47549 mov      qword ptr [rbp - 0x40], rcx
0x140a4754d mov      qword ptr [rbp - 0x20], rcx
0x140a47551 mov      qword ptr [rbp], rax
0x140a47555 lea      rcx, [rbp - 0x40]
0x140a47559 call     0x14087a9c0 ; core::ptr::drop_in_place$LT$alloc..collections..btree..map..IntoIter$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$::h3259f2c60ea86df4 (.llvm.12095432060108768980)
0x140a4755e mov      rax, rsi
0x140a47561 add      rsp, 0xd0
0x140a47568 pop      rdi
0x140a47569 pop      rsi
0x140a4756a pop      rbp
0x140a4756b ret      
0x140a4756c lea      r8, [rip + 0x2087115]
0x140a47573 mov      ecx, 1
0x140a47578 mov      edx, 7
0x140a4757d call     0x1428d8fe3 ; alloc::raw_vec::handle_error::hb99beac7784c4464
0x140a47582 lea      r8, [rip + 0x20870ff]
0x140a47589 mov      ecx, 1
0x140a4758e mov      edx, 9
0x140a47593 call     0x1428d8fe3 ; alloc::raw_vec::handle_error::hb99beac7784c4464
0x140a47598 lea      r8, [rip + 0x20870e9]
0x140a4759f mov      ecx, 1
0x140a475a4 mov      edx, 6
0x140a475a9 call     0x1428d8fe3 ; alloc::raw_vec::handle_error::hb99beac7784c4464
0x140a475ae lea      r8, [rip + 0x20870d3]
0x140a475b5 mov      ecx, 1
0x140a475ba mov      edx, 0xd
0x140a475bf call     0x1428d8fe3 ; alloc::raw_vec::handle_error::hb99beac7784c4464
0x140a475c4 lea      r8, [rip + 0x20870bd]
0x140a475cb mov      ecx, 1
0x140a475d0 mov      edx, 0xc
0x140a475d5 call     0x1428d8fe3 ; alloc::raw_vec::handle_error::hb99beac7784c4464
0x140a475da lea      r8, [rip + 0x20870a7]
0x140a475e1 mov      ecx, 1
0x140a475e6 mov      edx, 4
0x140a475eb call     0x1428d8fe3 ; alloc::raw_vec::handle_error::hb99beac7784c4464
0x140a475f0 int3     
0x140a475f1 nop      word ptr cs:[rax + rax]
0x140a47600 mov      qword ptr [rsp + 0x10], rdx
0x140a47605 push     rbp
0x140a47606 push     rsi
0x140a47607 push     rdi
0x140a47608 sub      rsp, 0x40
0x140a4760c lea      rbp, [rdx + 0x80]
0x140a47613 cmp      byte ptr [rbp + 0x47], 0
0x140a47617 jne      0x140a47621
0x140a47619 add      rsp, 0x40
0x140a4761d pop      rdi
0x140a4761e pop      rsi
0x140a4761f pop      rbp
0x140a47620 ret      
0x140a47621 lea      rcx, [rbp + 0x10]
0x140a47625 call     0x14088c930 ; core::ptr::drop_in_place$LT$serde_json..value..ser..SerializeStructVariant$GT$::hdb8294ca16185cc6
0x140a4762a jmp      0x140a47619 ; 