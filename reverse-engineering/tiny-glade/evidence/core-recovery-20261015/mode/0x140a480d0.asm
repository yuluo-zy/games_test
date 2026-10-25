0x140a480d0 push     rbp
0x140a480d1 push     r15
0x140a480d3 push     r14
0x140a480d5 push     r12
0x140a480d7 push     rsi
0x140a480d8 push     rdi
0x140a480d9 push     rbx
0x140a480da sub      rsp, 0x140
0x140a480e1 lea      rbp, [rsp + 0x80]
0x140a480e9 mov      qword ptr [rbp + 0xb8], 0xfffffffffffffffe
0x140a480f4 mov      rsi, rcx
0x140a480f7 movzx    eax, byte ptr [rdx]
0x140a480fa cmp      eax, 3
0x140a480fd mov      dword ptr [rbp + 0xa0], eax
0x140a48103 mov      qword ptr [rbp + 0x78], rdx
0x140a48107 je       0x140a481fa
0x140a4810d cmp      eax, 5
0x140a48110 jne      0x140a482b0
0x140a48116 mov      r15, qword ptr [rdx + 8]
0x140a4811a mov      r12, qword ptr [rdx + 0x10]
0x140a4811e mov      rdi, qword ptr [rdx + 0x18]
0x140a48122 xor      eax, eax
0x140a48124 test     r15, r15
0x140a48127 setne    al
0x140a4812a mov      rcx, rdi
0x140a4812d cmove    rcx, r15
0x140a48131 mov      qword ptr [rbp + 0x20], rax
0x140a48135 mov      qword ptr [rbp + 0x28], 0
0x140a4813d mov      qword ptr [rbp + 0x30], r15
0x140a48141 mov      qword ptr [rbp + 0x38], r12
0x140a48145 mov      qword ptr [rbp + 0x40], rax
0x140a48149 mov      qword ptr [rbp + 0x48], 0
0x140a48151 mov      qword ptr [rbp + 0x50], r15
0x140a48155 mov      qword ptr [rbp + 0x58], r12
0x140a48159 mov      qword ptr [rbp + 0x60], rcx
0x140a4815d lea      rcx, [rbp + 0x80]
0x140a48164 lea      rdx, [rbp + 0x20]
0x140a48168 call     0x14143c450 ; alloc::collections::btree::map::IntoIter$LT$K$C$V$C$A$GT$::dying_next::hbfabc728ec1fadfe
0x140a4816d mov      rax, qword ptr [rbp + 0x80]
0x140a48174 test     rax, rax
0x140a48177 je       0x140a481bd
0x140a48179 mov      rcx, qword ptr [rbp + 0x90]
0x140a48180 lea      rdx, [rcx + rcx*2]
0x140a48184 mov      r8, qword ptr [rax + rdx*8 + 0x168]
0x140a4818c mov      r9, qword ptr [rax + rdx*8 + 0x170]
0x140a48194 mov      rbx, qword ptr [rax + rdx*8 + 0x178]
0x140a4819c shl      rcx, 5
0x140a481a0 movups   xmm0, xmmword ptr [rax + rcx]
0x140a481a4 movups   xmm1, xmmword ptr [rax + rcx + 0x10]
0x140a481a9 movaps   xmmword ptr [rbp - 0x20], xmm0
0x140a481ad movaps   xmmword ptr [rbp - 0x10], xmm1
0x140a481b1 mov      rax, r8
0x140a481b4 neg      rax
0x140a481b7 jno      0x140a48391
0x140a481bd mov      byte ptr [rbp + 0x80], 0xb
0x140a481c4 lea      rdx, [rip + 0x208627d]
0x140a481cb lea      r8, [rip + 0x2086286]
0x140a481d2 lea      rcx, [rbp + 0x80]
0x140a481d9 call     0x1428f9b40 ; serde::de::Error::invalid_value::h663c1282ccb5042a
0x140a481de mov      qword ptr [rsi + 8], rax
0x140a481e2 mov      byte ptr [rsi], 1
0x140a481e5 mov      byte ptr [rbp + 0xb7], 1
0x140a481ec lea      rcx, [rbp + 0x20]
0x140a481f0 call     0x140887df0 ; core::ptr::drop_in_place$LT$serde_json..map..IntoIter$GT$::hfe4344ed9e7b917b
0x140a481f5 jmp      0x140a48470 ; 
0x140a481fa mov      r15, qword ptr [rdx + 8]
0x140a481fe mov      r12, qword ptr [rdx + 0x10]
0x140a48202 mov      rdi, qword ptr [rdx + 0x18]
0x140a48206 mov      r14b, 6
0x140a48209 mov      rax, r15
0x140a4820c mov      rcx, r12
0x140a4820f mov      rbx, rdi
0x140a48212 mov      qword ptr [rbp + 0x20], rax
0x140a48216 mov      qword ptr [rbp + 0x28], rcx
0x140a4821a mov      qword ptr [rbp + 0x30], rbx
0x140a4821e movaps   xmm0, xmmword ptr [rbp - 0x40]
0x140a48222 movups   xmmword ptr [rbp + 0x39], xmm0
0x140a48226 movups   xmm0, xmmword ptr [rbp - 0x31]
0x140a4822a movups   xmmword ptr [rbp + 0x48], xmm0
0x140a4822e mov      byte ptr [rbp + 0x38], r14b
0x140a48232 mov      byte ptr [rbp + 0xb7], 1
0x140a48239 lea      rcx, [rbp - 0x20]
0x140a4823d lea      rdx, [rbp + 0x20]
0x140a48241 call     0x1415d2590 ; _$LT$serde_json..value..de..EnumDeserializer$u20$as$u20$serde..de..EnumAccess$GT$::variant_seed::h45a8aec73e152f51
0x140a48246 movzx    ecx, byte ptr [rbp - 0x18]
0x140a4824a mov      rax, qword ptr [rbp - 0x20]
0x140a4824e cmp      cl, 7
0x140a48251 jne      0x140a4833d
0x140a48257 mov      qword ptr [rsi + 8], rax
0x140a4825b mov      al, 1
0x140a4825d mov      byte ptr [rsi], al
0x140a4825f mov      eax, dword ptr [rbp + 0xa0]
0x140a48265 cmp      eax, 3
0x140a48268 je       0x140a48470
0x140a4826e cmp      eax, 5
0x140a48271 je       0x140a48470
0x140a48277 mov      qword ptr [rbp + 0xa0], r15
0x140a4827e mov      qword ptr [rbp + 0x78], r12
0x140a48282 mov      rcx, r12
0x140a48285 mov      rdx, rdi
0x140a48288 call     0x140888d70 ; core::ptr::drop_in_place$LT$$u5b$serde_json..value..Value$u5d$$GT$::h955217e1f536f36b (.llvm.12095432060108768980)
0x140a4828d mov      rdx, qword ptr [rbp + 0xa0]
0x140a48294 test     rdx, rdx
0x140a48297 mov      rcx, qword ptr [rbp + 0x78]
0x140a4829b je       0x140a48470
0x140a482a1 shl      rdx, 5
0x140a482a5 mov      r8d, 8
0x140a482ab jmp      0x140a48459 ; 
0x140a482b0 movups   xmm0, xmmword ptr [rdx]
0x140a482b3 movups   xmm1, xmmword ptr [rdx + 0x10]
0x140a482b7 movaps   xmmword ptr [rbp + 0x30], xmm1
0x140a482bb movaps   xmmword ptr [rbp + 0x20], xmm0
0x140a482bf lea      rcx, [rbp - 0x20]
0x140a482c3 lea      rdx, [rbp + 0x20]
0x140a482c7 call     0x1428c9c10 ; serde_json::value::de::_$LT$impl$u20$serde_json..value..Value$GT$::unexpected::h4c624156aff5fb4a
0x140a482cc lea      rdx, [rip + 0x20861b5]
0x140a482d3 lea      r8, [rip + 0x208617e]
0x140a482da lea      rcx, [rbp - 0x20]
0x140a482de call     0x1428c9a00 ; _$LT$serde_json..error..Error$u20$as$u20$serde..de..Error$GT$::invalid_type::h76868c415965851f
0x140a482e3 mov      qword ptr [rsi + 8], rax
0x140a482e7 mov      byte ptr [rsi], 1
0x140a482ea movzx    eax, byte ptr [rbp + 0x20]
0x140a482ee cmp      eax, 3
0x140a482f1 jb       0x140a48470
0x140a482f7 je       0x140a48446
0x140a482fd cmp      eax, 4
0x140a48300 jne      0x140a48460
0x140a48306 mov      rcx, qword ptr [rbp + 0x30]
0x140a4830a mov      rdx, qword ptr [rbp + 0x38]
0x140a4830e mov      qword ptr [rbp + 0xa8], rcx
0x140a48315 call     0x140888d70 ; core::ptr::drop_in_place$LT$$u5b$serde_json..value..Value$u5d$$GT$::h955217e1f536f36b (.llvm.12095432060108768980)
0x140a4831a mov      rdx, qword ptr [rbp + 0x28]
0x140a4831e test     rdx, rdx
0x140a48321 je       0x140a48470
0x140a48327 shl      rdx, 5
0x140a4832b mov      r8d, 8
0x140a48331 mov      rcx, qword ptr [rbp + 0xa8]
0x140a48338 jmp      0x140a48459 ; 
0x140a4833d movups   xmm0, xmmword ptr [rbp - 0x17]
0x140a48341 movups   xmm1, xmmword ptr [rbp - 8]
0x140a48345 movups   xmmword ptr [rbp + 0x90], xmm1
0x140a4834c movups   xmmword ptr [rbp + 0x81], xmm0
0x140a48353 mov      byte ptr [rbp + 0x80], cl
0x140a48359 movzx    eax, al
0x140a4835c lea      rcx, [rip + 0x2087445]
0x140a48363 movsxd   rax, dword ptr [rcx + rax*4]
0x140a48367 add      rax, rcx
0x140a4836a jmp      rax
0x140a4836c mov      byte ptr [rbp + 0xb7], 1
0x140a48373 lea      rcx, [rbp + 0x80]
0x140a4837a call     0x140b88000 ; _$LT$serde_json..value..de..VariantDeserializer$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant::h6d02fa9789367749
0x140a4837f test     rax, rax
0x140a48382 jne      0x140a48257
0x140a48388 mov      byte ptr [rsi + 1], 0x38
0x140a4838c jmp      0x140a48590 ; 
0x140a48391 mov      qword ptr [rbp + 0x70], r9
0x140a48395 mov      qword ptr [rbp + 0xa8], r8
0x140a4839c movaps   xmm0, xmmword ptr [rbp - 0x20]
0x140a483a0 movaps   xmm1, xmmword ptr [rbp - 0x10]
0x140a483a4 movaps   xmmword ptr [rbp + 0x90], xmm1
0x140a483ab movaps   xmmword ptr [rbp + 0x80], xmm0
0x140a483b2 lea      rcx, [rbp - 0x58]
0x140a483b6 lea      rdx, [rbp + 0x20]
0x140a483ba call     0x14143c450 ; alloc::collections::btree::map::IntoIter$LT$K$C$V$C$A$GT$::dying_next::hbfabc728ec1fadfe
0x140a483bf mov      rax, qword ptr [rbp - 0x58]
0x140a483c3 test     rax, rax
0x140a483c6 je       0x140a48409
0x140a483c8 mov      rcx, qword ptr [rbp - 0x48]
0x140a483cc lea      rdx, [rcx + rcx*2]
0x140a483d0 mov      r8, qword ptr [rax + rdx*8 + 0x178]
0x140a483d8 mov      qword ptr [rbp - 0x10], r8
0x140a483dc movups   xmm0, xmmword ptr [rax + rdx*8 + 0x168]
0x140a483e4 movaps   xmmword ptr [rbp - 0x20], xmm0
0x140a483e8 shl      rcx, 5
0x140a483ec movups   xmm0, xmmword ptr [rax + rcx]
0x140a483f0 movups   xmm1, xmmword ptr [rax + rcx + 0x10]
0x140a483f5 movups   xmmword ptr [rbp - 8], xmm0
0x140a483f9 movups   xmmword ptr [rbp + 8], xmm1
0x140a483fd mov      rdx, qword ptr [rbp - 0x20]
0x140a48401 mov      rax, rdx
0x140a48404 neg      rax
0x140a48407 jno      0x140a48485
0x140a48409 mov      r14b, byte ptr [rbp + 0x80]
0x140a48410 movups   xmm0, xmmword ptr [rbp + 0x81]
0x140a48417 movaps   xmmword ptr [rbp - 0x40], xmm0
0x140a4841b movups   xmm0, xmmword ptr [rbp + 0x90]
0x140a48422 movups   xmmword ptr [rbp - 0x31], xmm0
0x140a48426 mov      byte ptr [rbp + 0xb7], 1
0x140a4842d lea      rcx, [rbp + 0x20]
0x140a48431 call     0x140887df0 ; core::ptr::drop_in_place$LT$serde_json..map..IntoIter$GT$::hfe4344ed9e7b917b
0x140a48436 mov      rax, qword ptr [rbp + 0xa8]
0x140a4843d mov      rcx, qword ptr [rbp + 0x70]
0x140a48441 jmp      0x140a48212 ; 
0x140a48446 mov      rdx, qword ptr [rbp + 0x28]
0x140a4844a test     rdx, rdx
0x140a4844d je       0x140a48470
0x140a4844f mov      rcx, qword ptr [rbp + 0x30]
0x140a48453 mov      r8d, 1
0x140a48459 call     0x140613c20 ; __rust_dealloc
0x140a4845e jmp      0x140a48470 ; 
0x140a48460 lea      rcx, [rbp + 0x28]
0x140a48464 mov      byte ptr [rbp + 0xb7], 0
0x140a4846b call     0x140898f40 ; core::ptr::drop_in_place$LT$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$::h768a51823091e6f3
0x140a48470 mov      rax, rsi
0x140a48473 add      rsp, 0x140
0x140a4847a pop      rbx
0x140a4847b pop      rdi
0x140a4847c pop      rsi
0x140a4847d pop      r12
0x140a4847f pop      r14
0x140a48481 pop      r15
0x140a48483 pop      rbp
0x140a48484 ret      
0x140a48485 jae      0x140a48496
0x140a48487 mov      rcx, qword ptr [rbp - 0x18]
0x140a4848b mov      r8d, 1
0x140a48491 call     0x140613c20 ; __rust_dealloc
0x140a48496 movzx    eax, byte ptr [rbp - 8]
0x140a4849a cmp      eax, 3
0x140a4849d jb       0x140a485de
0x140a484a3 je       0x140a485bf
0x140a484a9 lea      rcx, [rbp]
0x140a484ad cmp      eax, 4
0x140a484b0 jne      0x140a485d9
0x140a484b6 call     0x140889390 ; core::ptr::drop_in_place$LT$serde_json..value..ser..SerializeVec$GT$::h0b4a276b7f9ff0e5 / core::ptr::drop_in_place$LT$alloc..vec..Vec$LT$serde_json..value..Value$GT$$GT$::h9b2a9e508e63fbd8
0x140a484bb jmp      0x140a485de ; 
0x140a484c0 mov      byte ptr [rbp + 0xb7], 1
0x140a484c7 lea      rcx, [rbp + 0x80]
0x140a484ce call     0x140b88000 ; _$LT$serde_json..value..de..VariantDeserializer$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant::h6d02fa9789367749
0x140a484d3 test     rax, rax
0x140a484d6 jne      0x140a48257
0x140a484dc mov      byte ptr [rsi + 1], 0x3c
0x140a484e0 jmp      0x140a48590 ; 
0x140a484e5 mov      byte ptr [rbp + 0xb7], 1
0x140a484ec lea      rcx, [rbp + 0x80]
0x140a484f3 call     0x140b88000 ; _$LT$serde_json..value..de..VariantDeserializer$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant::h6d02fa9789367749
0x140a484f8 test     rax, rax
0x140a484fb jne      0x140a48257
0x140a48501 mov      byte ptr [rsi + 1], 0x3a
0x140a48505 jmp      0x140a48590 ; 
0x140a4850a mov      byte ptr [rbp + 0xb7], 1
0x140a48511 lea      rcx, [rbp + 0x80]
0x140a48518 call     0x140b88000 ; _$LT$serde_json..value..de..VariantDeserializer$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant::h6d02fa9789367749
0x140a4851d test     rax, rax
0x140a48520 jne      0x140a48257
0x140a48526 mov      byte ptr [rsi + 1], 0x3b
0x140a4852a jmp      0x140a48590 ; 
0x140a4852c mov      byte ptr [rbp + 0xb7], 1
0x140a48533 lea      rcx, [rbp + 0x80]
0x140a4853a call     0x140b88000 ; _$LT$serde_json..value..de..VariantDeserializer$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant::h6d02fa9789367749
0x140a4853f test     rax, rax
0x140a48542 jne      0x140a48257
0x140a48548 mov      byte ptr [rsi + 1], 0x3f
0x140a4854c jmp      0x140a48590 ; 
0x140a4854e mov      byte ptr [rbp + 0xb7], 1
0x140a48555 lea      rcx, [rbp + 0x80]
0x140a4855c call     0x140b88000 ; _$LT$serde_json..value..de..VariantDeserializer$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant::h6d02fa9789367749
0x140a48561 test     rax, rax
0x140a48564 jne      0x140a48257
0x140a4856a mov      byte ptr [rsi + 1], 0x39
0x140a4856e jmp      0x140a48590 ; 
0x140a48570 mov      byte ptr [rbp + 0xb7], 1
0x140a48577 lea      rcx, [rbp + 0x80]
0x140a4857e call     0x140b88000 ; _$LT$serde_json..value..de..VariantDeserializer$u20$as$u20$serde..de..VariantAccess$GT$::unit_variant::h6d02fa9789367749
0x140a48583 test     rax, rax
0x140a48586 jne      0x140a48257
0x140a4858c mov      byte ptr [rsi + 1], 0x3d
0x140a48590 xor      eax, eax
0x140a48592 jmp      0x140a4825d ; 
0x140a48597 mov      byte ptr [rbp + 0xb7], 1
0x140a4859e lea      r8, [rip + 0x208687b]
0x140a485a5 lea      rdx, [rbp + 0x80]
0x140a485ac mov      r9d, 1
0x140a485b2 mov      rcx, rsi
0x140a485b5 call     0x1415de990 ; _$LT$serde_json..value..de..VariantDeserializer$u20$as$u20$serde..de..VariantAccess$GT$::struct_variant::h6dc5d3f85cca1e76
0x140a485ba jmp      0x140a4825f ; 
0x140a485bf mov      rdx, qword ptr [rbp]
0x140a485c3 test     rdx, rdx
0x140a485c6 je       0x140a485de
0x140a485c8 mov      rcx, qword ptr [rbp + 8]
0x140a485cc mov      r8d, 1
0x140a485d2 call     0x140613c20 ; __rust_dealloc
0x140a485d7 jmp      0x140a485de ; 
0x140a485d9 call     0x140898f40 ; core::ptr::drop_in_place$LT$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$::h768a51823091e6f3
0x140a485de mov      byte ptr [rbp - 0x20], 0xb
0x140a485e2 lea      rdx, [rip + 0x2085e5f]
0x140a485e9 lea      r8, [rip + 0x2085e68]
0x140a485f0 lea      rcx, [rbp - 0x20]
0x140a485f4 call     0x1428f9b40 ; serde::de::Error::invalid_value::h663c1282ccb5042a
0x140a485f9 mov      qword ptr [rsi + 8], rax
0x140a485fd mov      byte ptr [rsi], 1
0x140a48600 movzx    eax, byte ptr [rbp + 0x80]
0x140a48607 cmp      eax, 3
0x140a4860a jb       0x140a48646
0x140a4860c je       0x140a48621
0x140a4860e lea      rcx, [rbp + 0x88]
0x140a48615 cmp      eax, 4
0x140a48618 jne      0x140a48641
0x140a4861a call     0x140889390 ; core::ptr::drop_in_place$LT$serde_json..value..ser..SerializeVec$GT$::h0b4a276b7f9ff0e5 / core::ptr::drop_in_place$LT$alloc..vec..Vec$LT$serde_json..value..Value$GT$$GT$::h9b2a9e508e63fbd8
0x140a4861f jmp      0x140a48646 ; 
0x140a48621 mov      rdx, qword ptr [rbp + 0x88]
0x140a48628 test     rdx, rdx
0x140a4862b je       0x140a48646
0x140a4862d mov      rcx, qword ptr [rbp + 0x90]
0x140a48634 mov      r8d, 1
0x140a4863a call     0x140613c20 ; __rust_dealloc
0x140a4863f jmp      0x140a48646 ; 
0x140a48641 call     0x140898f40 ; core::ptr::drop_in_place$LT$serde_json..map..Map$LT$alloc..string..String$C$serde_json..value..Value$GT$$GT$::h768a51823091e6f3
0x140a48646 mov      rdx, qword ptr [rbp + 0xa8]
0x140a4864d test     rdx, rdx
0x140a48650 je       0x140a481e5
0x140a48656 mov      r8d, 1
0x140a4865c mov      rcx, qword ptr [rbp + 0x70]
0x140a48660 call     0x140613c20 ; __rust_dealloc
0x140a48665 jmp      0x140a481e5 ; 
0x140a4866a nop      word ptr [rax + rax]
0x140a48670 mov      qword ptr [rsp + 0x10], rdx
0x140a48675 push     rbp
0x140a48676 push     r15
0x140a48678 push     r14
0x140a4867a push     r12
0x140a4867c push     rsi
0x140a4867d push     rdi
0x140a4867e push     rbx
0x140a4867f sub      rsp, 0x20
0x140a48683 lea      rbp, [rdx + 0x80]
0x140a4868a lea      rcx, [rbp + 0x20]
0x140a4868e call     0x140887df0 ; core::ptr::drop_in_place$LT$serde_json..map..IntoIter$GT$::hfe4344ed9e7b917b
0x140a48693 mov      byte ptr [rbp + 0xb7], 1
0x140a4869a add      rsp, 0x20
0x140a4869e pop      rbx
0x140a4869f pop      rdi
0x140a486a0 pop      rsi
0x140a486a1 pop      r12
0x140a486a3 pop      r14
0x140a486a5 pop      r15
0x140a486a7 pop      rbp
0x140a486a8 ret      
0x140a486a9 nop      dword ptr [rax]
0x140a486b0 mov      qword ptr [rsp + 0x10], rdx
0x140a486b5 push     rbp
0x140a486b6 push     r15
0x140a486b8 push     r14
0x140a486ba push     r12
0x140a486bc push     rsi
0x140a486bd push     rdi
0x140a486be push     rbx
0x140a486bf sub      rsp, 0x20
0x140a486c3 lea      rbp, [rdx + 0x80]
0x140a486ca cmp      qword ptr [rbp + 0xa8], 0
0x140a486d2 je       0x140a486ea
0x140a486d4 mov      r8d, 1
0x140a486da mov      rcx, qword ptr [rbp + 0x70]
0x140a486de mov      rdx, qword ptr [rbp + 0xa8]
0x140a486e5 call     0x140613c20 ; __rust_dealloc
0x140a486ea nop      
0x140a486eb add      rsp, 0x20
0x140a486ef pop      rbx
0x140a486f0 pop      rdi
0x140a486f1 pop      rsi
0x140a486f2 pop      r12
0x140a486f4 pop      r14
0x140a486f6 pop      r15
0x140a486f8 pop      rbp
0x140a486f9 ret      
0x140a486fa nop      word ptr [rax + rax]
0x140a48700 mov      qword ptr [rsp + 0x10], rdx
0x140a48705 push     rbp
0x140a48706 push     r15
0x140a48708 push     r14
0x140a4870a push     r12
0x140a4870c push     rsi
0x140a4870d push     rdi
0x140a4870e push     rbx
0x140a4870f sub      rsp, 0x20
0x140a48713 lea      rbp, [rdx + 0x80]
0x140a4871a cmp      qword ptr [rbp + 0xa0], 0
0x140a48722 je       0x140a4873e
0x140a48724 mov      rdx, qword ptr [rbp + 0xa0]
0x140a4872b shl      rdx, 5
0x140a4872f mov      r8d, 8
0x140a48735 mov      rcx, qword ptr [rbp + 0x78]
0x140a48739 call     0x140613c20 ; __rust_dealloc
0x140a4873e nop      
0x140a4873f add      rsp, 0x20
0x140a48743 pop      rbx
0x140a48744 pop      rdi
0x140a48745 pop      rsi
0x140a48746 pop      r12
0x140a48748 pop      r14
0x140a4874a pop      r15
0x140a4874c pop      rbp
0x140a4874d ret      
0x140a4874e nop      
0x140a48750 mov      qword ptr [rsp + 0x10], rdx
0x140a48755 push     rbp
0x140a48756 push     r15
0x140a48758 push     r14
0x140a4875a push     r12
0x140a4875c push     rsi
0x140a4875d push     rdi
0x140a4875e push     rbx
0x140a4875f sub      rsp, 0x20
0x140a48763 lea      rbp, [rdx + 0x80]
0x140a4876a lea      rcx, [rbp + 0x20]
0x140a4876e call     0x14085da90 ; core::ptr::drop_in_place$LT$serde_json..value..Value$GT$::hc273c085b0e9cf2f (.llvm.7127459029268306789) / core::ptr::drop_in_place$LT$serde_json..value..Value$GT$::hc273c085b0e9cf2f (.llvm.5480732374369995240)
0x140a48773 mov      byte ptr [rbp + 0xb7], 0
0x140a4877a add      rsp, 0x20
0x140a4877e pop      rbx
0x140a4877f pop      rdi
0x140a48780 pop      rsi
0x140a48781 pop      r12
0x140a48783 pop      r14
0x140a48785 pop      r15
0x140a48787 pop      rbp
0x140a48788 ret      
0x140a48789 nop      dword ptr [rax]
0x140a48790 mov      qword ptr [rsp + 0x10], rdx
0x140a48795 push     rbp
0x140a48796 push     r15
0x140a48798 push     r14
0x140a4879a push     r12
0x140a4879c push     rsi
0x140a4879d push     rdi
0x140a4879e push     rbx
0x140a4879f sub      rsp, 0x20
0x140a487a3 lea      rbp, [rdx + 0x80]
0x140a487aa mov      rdx, qword ptr [rbp + 0x28]
0x140a487ae test     rdx, rdx
0x140a487b1 je       0x140a487c9
0x140a487b3 shl      rdx, 5
0x140a487b7 mov      r8d, 8
0x140a487bd mov      rcx, qword ptr [rbp + 0xa8]
0x140a487c4 call     0x140613c20 ; __rust_dealloc
0x140a487c9 mov      byte ptr [rbp + 0xb7], 0
0x140a487d0 add      rsp, 0x20
0x140a487d4 pop      rbx
0x140a487d5 pop      rdi
0x140a487d6 pop      rsi
0x140a487d7 pop      r12
0x140a487d9 pop      r14
0x140a487db pop      r15
0x140a487dd pop      rbp
0x140a487de ret      
0x140a487df nop      
0x140a487e0 mov      qword ptr [rsp + 0x10], rdx
0x140a487e5 push     rbp
0x140a487e6 push     r15
0x140a487e8 push     r14
0x140a487ea push     r12
0x140a487ec push     rsi
0x140a487ed push     rdi
0x140a487ee push     rbx
0x140a487ef sub      rsp, 0x20
0x140a487f3 lea      rbp, [rdx + 0x80]
0x140a487fa cmp      byte ptr [rbp + 0xb7], 0
0x140a48801 je       0x140a48818
0x140a48803 cmp      byte ptr [rbp + 0xa0], 3
0x140a4880a je       0x140a48818
0x140a4880c movzx    eax, byte ptr [rbp + 0xa0]
0x140a48813 cmp      eax, 5
0x140a48816 jne      0x140a48827
0x140a48818 add      rsp, 0x20
0x140a4881c pop      rbx
0x140a4881d pop      rdi
0x140a4881e pop      rsi
0x140a4881f pop      r12
0x140a48821 pop      r14
0x140a48823 pop      r15
0x140a48825 pop      rbp
0x140a48826 ret      
0x140a48827 mov      rcx, qword ptr [rbp + 0x78]
0x140a4882b call     0x14085da90 ; core::ptr::drop_in_place$LT$serde_json..value..Value$GT$::hc273c085b0e9cf2f (.llvm.7127459029268306789) / core::ptr::drop_in_place$LT$serde_json..value..Value$GT$::hc273c085b0e9cf2f (.llvm.5480732374369995240)
0x140a48830 jmp      0x140a48818 ; 
0x140a48832 nop      word ptr cs:[rax + rax]
0x140a48840 mov      qword ptr [rsp + 0x10], rdx
0x140a48845 push     rbp
0x140a48846 push     r15
0x140a48848 push     r14
0x140a4884a push     r12
0x140a4884c push     rsi
0x140a4884d push     rdi
0x140a4884e push     rbx
0x140a4884f sub      rsp, 0x20
0x140a48853 lea      rbp, [rdx + 0x80]
0x140a4885a lea      rcx, [rbp + 0x80]
0x140a48861 call     0x14085da90 ; core::ptr::drop_in_place$LT$serde_json..value..Value$GT$::hc273c085b0e9cf2f (.llvm.7127459029268306789) / core::ptr::drop_in_place$LT$serde_json..value..Value$GT$::hc273c085b0e9cf2f (.llvm.5480732374369995240)
0x140a48866 nop      
0x140a48867 add      rsp, 0x20
0x140a4886b pop      rbx
0x140a4886c pop      rdi
0x140a4886d pop      rsi
0x140a4886e pop      r12
0x140a48870 pop      r14
0x140a48872 pop      r15
0x140a48874 pop      rbp
0x140a48875 ret      