0x1408b5500 push     rbp
0x1408b5501 push     rsi
0x1408b5502 push     rdi
0x1408b5503 sub      rsp, 0x100
0x1408b550a lea      rbp, [rsp + 0x80]
0x1408b5512 movaps   xmmword ptr [rbp + 0x70], xmm6
0x1408b5516 mov      qword ptr [rbp + 0x68], 0xfffffffffffffffe
0x1408b551e mov      rsi, rcx
0x1408b5521 xor      edi, edi
0x1408b5523 cmp      qword ptr [rcx], 0
0x1408b5527 je       0x1408b5595
0x1408b5529 mov      rax, qword ptr [rsi + 8]
0x1408b552d cmp      rax, qword ptr [rsi + 0xf8]
0x1408b5534 jae      0x1408b5595
0x1408b5536 mov      rcx, qword ptr [rsi + 0xf0]
0x1408b553d lea      rdx, [rax + rax*2]
0x1408b5541 shl      rdx, 4
0x1408b5545 xor      edi, edi
0x1408b5547 cmp      dword ptr [rcx + rdx], 1
0x1408b554b mov      eax, 0
0x1408b5550 jne      0x1408b5590
0x1408b5552 movaps   xmm6, xmm1
0x1408b5555 add      rcx, rdx
0x1408b5558 mov      rax, qword ptr [rcx + 8]
0x1408b555c mov      qword ptr [rbp + 0x30], rax
0x1408b5560 lea      rcx, [rbp - 0x50]
0x1408b5564 lea      rdx, [rbp + 0x30]
0x1408b5568 call     0x1414375b0 ; libfmod::EventInstance::get_timeline_position::hfe280d0cf1cb588a
0x1408b556d movabs   rax, 0x8000000000000004
0x1408b5577 cmp      qword ptr [rbp - 0x50], rax
0x1408b557b jne      0x1408b5582
0x1408b557d mov      eax, dword ptr [rbp - 0x48]
0x1408b5580 jmp      0x1408b558d ; 
0x1408b5582 lea      rcx, [rbp - 0x50]
0x1408b5586 call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x1408b558b xor      eax, eax
0x1408b558d movaps   xmm1, xmm6
0x1408b5590 test     eax, eax
0x1408b5592 cmovg    edi, eax
0x1408b5595 mov      eax, dword ptr [rsi + 0x118]
0x1408b559b test     eax, eax
0x1408b559d je       0x1408b5630
0x1408b55a3 cmp      edi, dword ptr [rsi + 0x128]
0x1408b55a9 jge      0x1408b56d7
0x1408b55af mov      qword ptr [rip + 0x2d7f6d6], 0
0x1408b55ba mov      dword ptr [rsi + 0x128], edi
0x1408b55c0 dec      eax
0x1408b55c2 lea      rcx, [rip + 0x21ea46b]
0x1408b55c9 movsxd   rax, dword ptr [rcx + rax*4]
0x1408b55cd add      rax, rcx
0x1408b55d0 jmp      rax
0x1408b55d2 addss    xmm1, dword ptr [rsi + 0x11c]
0x1408b55da movss    dword ptr [rsi + 0x11c], xmm1
0x1408b55e2 ucomiss  xmm1, dword ptr [rip + 0x20702e7]
0x1408b55e9 jbe      0x1408b5615
0x1408b55eb cmp      dword ptr [rsi + 0x40], 1
0x1408b55ef jne      0x1408b5615
0x1408b55f1 lea      rdx, [rsi + 0x48]
0x1408b55f5 lea      rcx, [rbp + 0x30]
0x1408b55f9 xor      r8d, r8d
0x1408b55fc call     0x141437440 ; libfmod::EventInstance::stop::h6aa257fdd68e9145
0x1408b5601 movabs   rax, 0x8000000000000004
0x1408b560b cmp      qword ptr [rbp + 0x30], rax
0x1408b560f jne      0x1408b5f60
0x1408b5615 mov      qword ptr [rsi + 0x118], 2
0x1408b5620 mov      qword ptr [rsi + 0x120], 0
0x1408b562b jmp      0x1408b5bee ; 
0x1408b5630 mov      dword ptr [rsi + 0x128], 0
0x1408b563a mov      qword ptr [rip + 0x2d7f64b], 0
0x1408b5645 mov      rcx, rsi
0x1408b5648 call     0x1408b70d0 ; country_core::systems::audio::music::music_player::MusicPlayer::switch_track::he3bf1734a414967b
0x1408b564d cmp      qword ptr [rsi], 0
0x1408b5651 je       0x1408b5787
0x1408b5657 mov      rax, qword ptr [rsi + 8]
0x1408b565b cmp      rax, qword ptr [rsi + 0xf8]
0x1408b5662 jae      0x1408b5787
0x1408b5668 lea      rdi, [rax + rax*2]
0x1408b566c shl      rdi, 4
0x1408b5670 add      rdi, qword ptr [rsi + 0xf0]
0x1408b5677 cmp      dword ptr [rsi + 0x40], 1
0x1408b567b jne      0x1408b569e
0x1408b567d lea      rdx, [rsi + 0x48]
0x1408b5681 lea      rcx, [rbp + 0x30]
0x1408b5685 call     0x1414372d0 ; libfmod::EventInstance::start::hb8dc190a69906390
0x1408b568a movabs   rax, 0x8000000000000004
0x1408b5694 cmp      qword ptr [rbp + 0x30], rax
0x1408b5698 jne      0x1408b5de2
0x1408b569e cmp      dword ptr [rdi], 1
0x1408b56a1 jne      0x1408b56c7
0x1408b56a3 add      rdi, 8
0x1408b56a7 lea      rcx, [rbp + 0x30]
0x1408b56ab mov      rdx, rdi
0x1408b56ae call     0x1414372d0 ; libfmod::EventInstance::start::hb8dc190a69906390
0x1408b56b3 movabs   rax, 0x8000000000000004
0x1408b56bd cmp      qword ptr [rbp + 0x30], rax
0x1408b56c1 jne      0x1408b5e7c
0x1408b56c7 mov      qword ptr [rsi + 0x118], 1
0x1408b56d2 jmp      0x1408b5bee ; 
0x1408b56d7 mov      dword ptr [rsi + 0x128], edi
0x1408b56dd dec      eax
0x1408b56df lea      rcx, [rip + 0x21ea33e]
0x1408b56e6 movsxd   rax, dword ptr [rcx + rax*4]
0x1408b56ea add      rax, rcx
0x1408b56ed jmp      rax
0x1408b56ef addss    xmm1, dword ptr [rsi + 0x11c]
0x1408b56f7 movss    dword ptr [rsi + 0x11c], xmm1
0x1408b56ff ucomiss  xmm1, dword ptr [rip + 0x20701ca]
0x1408b5706 jbe      0x1408b5bee
0x1408b570c cmp      dword ptr [rsi + 0x40], 1
0x1408b5710 jne      0x1408b5bee
0x1408b5716 lea      rdx, [rsi + 0x48]
0x1408b571a lea      rcx, [rbp + 0x30]
0x1408b571e xor      r8d, r8d
0x1408b5721 call     0x141437440 ; libfmod::EventInstance::stop::h6aa257fdd68e9145
0x1408b5726 movabs   rax, 0x8000000000000004
0x1408b5730 cmp      qword ptr [rbp + 0x30], rax
0x1408b5734 je       0x1408b5bee
0x1408b573a mov      rax, qword ptr [rbp + 0x60]
0x1408b573e mov      qword ptr [rbp - 0x20], rax
0x1408b5742 movups   xmm0, xmmword ptr [rbp + 0x30]
0x1408b5746 movups   xmm1, xmmword ptr [rbp + 0x40]
0x1408b574a movups   xmm2, xmmword ptr [rbp + 0x50]
0x1408b574e movaps   xmmword ptr [rbp - 0x30], xmm2
0x1408b5752 movaps   xmmword ptr [rbp - 0x40], xmm1
0x1408b5756 movaps   xmmword ptr [rbp - 0x50], xmm0
0x1408b575a lea      rax, [rip + 0x221d4a7]
0x1408b5761 mov      qword ptr [rsp + 0x20], rax
0x1408b5766 lea      rcx, [rip + 0x221ae93] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x1408b576d lea      r9, [rip + 0x221ae4c]
0x1408b5774 lea      r8, [rbp - 0x50]
0x1408b5778 mov      edx, 0x2b
0x1408b577d call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x1408b5782 jmp      0x1408b5fa8 ; 
0x1408b5787 mov      rax, qword ptr [rip + 0x2d76c1a]
0x1408b578e mov      rax, qword ptr [rax]
0x1408b5791 cmp      rax, 5
0x1408b5795 jb       0x1408b5bee
0x1408b579b lea      rcx, [rip + 0x21e9e16]
0x1408b57a2 call     0x14067dd80 ; _$LT$country_core..render..ssbo..TypedStorageBuffer$LT$T$GT$$u20$as$u20$country_core..asset_libraries..ssbo_library..StorageBufferDyn$GT$::buffer::h55d27067a9534614 / _$LT$country_core..render..ssbo..TypedStorageBuffer$LT$T$GT$$u20$as$u20$country_core..asset_libraries..ssbo_library..StorageBufferDyn$GT$::buffer::h781648298f14b5a6
0x1408b57a7 movups   xmm0, xmmword ptr [rax]
0x1408b57aa mov      eax, dword ptr [rax + 0x10]
0x1408b57ad mov      qword ptr [rbp - 0x20], 5
0x1408b57b5 lea      rcx, [rip + 0x21e9e14] ; @0x142a9f5d0 'country_core::systems::audio::music::music_playersnapshot:/Track Switching'
0x1408b57bc mov      qword ptr [rbp - 0x18], rcx
0x1408b57c0 mov      qword ptr [rbp - 0x10], 0x31
0x1408b57c8 lea      rdx, [rip + 0x21e9d99]
0x1408b57cf mov      qword ptr [rbp], rdx
0x1408b57d3 mov      qword ptr [rbp + 8], 1
0x1408b57db mov      qword ptr [rbp + 0x10], 8
0x1408b57e3 mov      qword ptr [rbp - 0x50], 0
0x1408b57eb xorps    xmm1, xmm1
0x1408b57ee movups   xmmword ptr [rbp + 0x18], xmm1
0x1408b57f2 mov      qword ptr [rbp - 0x48], rcx
0x1408b57f6 mov      qword ptr [rbp - 0x40], 0x31
0x1408b57fe mov      qword ptr [rbp - 0x38], 0
0x1408b5806 movups   xmmword ptr [rbp - 0x30], xmm0
0x1408b580a mov      dword ptr [rbp - 8], 1
0x1408b5811 mov      dword ptr [rbp - 4], eax
0x1408b5814 lea      rcx, [rbp + 0x30]
0x1408b5818 lea      rdx, [rbp - 0x50]
0x1408b581c call     0x140d4a330 ; _$LT$log..__private_api..GlobalLogger$u20$as$u20$log..Log$GT$::log::h234cc9ac9918612c
0x1408b5821 jmp      0x1408b5bee ; 
0x1408b5826 addss    xmm1, dword ptr [rsi + 0x11c]
0x1408b582e movss    dword ptr [rsi + 0x11c], xmm1
0x1408b5836 ucomiss  xmm1, dword ptr [rip + 0x21e9bc3]
0x1408b583d movaps   xmm0, xmm1
0x1408b5840 jbe      0x1408b587d
0x1408b5842 cmp      dword ptr [rsi + 0x10], 1
0x1408b5846 movaps   xmm0, xmm1
0x1408b5849 jne      0x1408b587d
0x1408b584b movaps   xmm6, xmm1
0x1408b584e lea      rdx, [rsi + 0x18]
0x1408b5852 lea      rcx, [rbp + 0x30]
0x1408b5856 mov      r8b, 1
0x1408b5859 call     0x141437440 ; libfmod::EventInstance::stop::h6aa257fdd68e9145
0x1408b585e movabs   rax, 0x8000000000000004
0x1408b5868 cmp      qword ptr [rbp + 0x30], rax
0x1408b586c jne      0x1408b5f16
0x1408b5872 movss    xmm0, dword ptr [rsi + 0x11c]
0x1408b587a movaps   xmm1, xmm6
0x1408b587d ucomiss  xmm0, dword ptr [rip + 0x21e9b80]
0x1408b5884 jbe      0x1408b5890
0x1408b5886 mov      dword ptr [rsi + 0x118], 0
0x1408b5890 cmp      dword ptr [rsi + 0x40], 1
0x1408b5894 jne      0x1408b58c0
0x1408b5896 movaps   xmm6, xmm1
0x1408b5899 lea      rdx, [rsi + 0x48]
0x1408b589d lea      rcx, [rbp + 0x30]
0x1408b58a1 xor      r8d, r8d
0x1408b58a4 call     0x141437440 ; libfmod::EventInstance::stop::h6aa257fdd68e9145
0x1408b58a9 movabs   rax, 0x8000000000000004
0x1408b58b3 cmp      qword ptr [rbp + 0x30], rax
0x1408b58b7 jne      0x1408b5d95
0x1408b58bd movaps   xmm1, xmm6
0x1408b58c0 ucomiss  xmm1, dword ptr [rip + 0x21c61b9]
0x1408b58c7 jbe      0x1408b5bee
0x1408b58cd cmp      qword ptr [rsi], 0
0x1408b58d1 je       0x1408b5c9f
0x1408b58d7 mov      rax, qword ptr [rsi + 8]
0x1408b58db cmp      rax, qword ptr [rsi + 0xf8]
0x1408b58e2 jae      0x1408b5bee
0x1408b58e8 mov      rdx, qword ptr [rsi + 0xf0]
0x1408b58ef lea      rax, [rax + rax*2]
0x1408b58f3 shl      rax, 4
0x1408b58f7 cmp      dword ptr [rdx + rax], 1
0x1408b58fb jne      0x1408b5bee
0x1408b5901 add      rdx, rax
0x1408b5904 add      rdx, 8
0x1408b5908 lea      rcx, [rbp + 0x30]
0x1408b590c mov      r8b, 1
0x1408b590f call     0x141437440 ; libfmod::EventInstance::stop::h6aa257fdd68e9145
0x1408b5914 movabs   rax, 0x8000000000000004
0x1408b591e cmp      qword ptr [rbp + 0x30], rax
0x1408b5922 je       0x1408b5bee
0x1408b5928 mov      rax, qword ptr [rbp + 0x60]
0x1408b592c mov      qword ptr [rbp - 0x20], rax
0x1408b5930 movups   xmm0, xmmword ptr [rbp + 0x30]
0x1408b5934 movups   xmm1, xmmword ptr [rbp + 0x40]
0x1408b5938 movups   xmm2, xmmword ptr [rbp + 0x50]
0x1408b593c movaps   xmmword ptr [rbp - 0x30], xmm2
0x1408b5940 movaps   xmmword ptr [rbp - 0x40], xmm1
0x1408b5944 movaps   xmmword ptr [rbp - 0x50], xmm0
0x1408b5948 lea      rax, [rip + 0x221d2d1]
0x1408b594f mov      qword ptr [rsp + 0x20], rax
0x1408b5954 lea      rcx, [rip + 0x221aca5] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x1408b595b lea      r9, [rip + 0x221ac5e]
0x1408b5962 lea      r8, [rbp - 0x50]
0x1408b5966 mov      edx, 0x2b
0x1408b596b call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x1408b5970 jmp      0x1408b5fa8 ; 
0x1408b5975 mov      al, 1
0x1408b5977 test     al, al
0x1408b5979 jne      0x1408b59be
0x1408b597b jmp      0x1408b59fa ; 
0x1408b597d addss    xmm1, dword ptr [rsi + 0x11c]
0x1408b5985 movss    dword ptr [rsi + 0x11c], xmm1
0x1408b598d mov      rax, qword ptr [rsi + 0x120]
0x1408b5994 test     rax, rax
0x1408b5997 js       0x1408b5a9f
0x1408b599d cvtsi2ss xmm0, rax
0x1408b59a2 ucomiss  xmm1, xmm0
0x1408b59a5 jbe      0x1408b5abd
0x1408b59ab jmp      0x1408b5b52 ; 
0x1408b59b0 test     byte ptr [rip + 0x2d7f2d9], 3
0x1408b59b7 sete     al
0x1408b59ba test     al, al
0x1408b59bc je       0x1408b59fa
0x1408b59be cmp      qword ptr [rip + 0x2d7f2a2], 1
0x1408b59c6 jne      0x1408b59fa
0x1408b59c8 mov      qword ptr [rsi + 0x118], 4
0x1408b59d3 cmp      dword ptr [rsi + 0x10], 1
0x1408b59d7 jne      0x1408b59fa
0x1408b59d9 lea      rdx, [rsi + 0x18]
0x1408b59dd lea      rcx, [rbp + 0x30]
0x1408b59e1 call     0x1414372d0 ; libfmod::EventInstance::start::hb8dc190a69906390
0x1408b59e6 movabs   rax, 0x8000000000000004
0x1408b59f0 cmp      qword ptr [rbp + 0x30], rax
0x1408b59f4 jne      0x1408b5ec9
0x1408b59fa cmp      dword ptr [rsi + 0x40], 1
0x1408b59fe jne      0x1408b5bee
0x1408b5a04 lea      rdx, [rsi + 0x48]
0x1408b5a08 lea      rcx, [rbp + 0x30]
0x1408b5a0c xor      r8d, r8d
0x1408b5a0f call     0x141437440 ; libfmod::EventInstance::stop::h6aa257fdd68e9145
0x1408b5a14 movabs   rax, 0x8000000000000004
0x1408b5a1e cmp      qword ptr [rbp + 0x30], rax
0x1408b5a22 je       0x1408b5bee
0x1408b5a28 mov      rax, qword ptr [rbp + 0x60]
0x1408b5a2c mov      qword ptr [rbp - 0x20], rax
0x1408b5a30 movups   xmm0, xmmword ptr [rbp + 0x30]
0x1408b5a34 movups   xmm1, xmmword ptr [rbp + 0x40]
0x1408b5a38 movups   xmm2, xmmword ptr [rbp + 0x50]
0x1408b5a3c movaps   xmmword ptr [rbp - 0x30], xmm2
0x1408b5a40 movaps   xmmword ptr [rbp - 0x40], xmm1
0x1408b5a44 movaps   xmmword ptr [rbp - 0x50], xmm0
0x1408b5a48 lea      rax, [rip + 0x221d1b9]
0x1408b5a4f mov      qword ptr [rsp + 0x20], rax
0x1408b5a54 lea      rcx, [rip + 0x221aba5] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x1408b5a5b lea      r9, [rip + 0x221ab5e]
0x1408b5a62 lea      r8, [rbp - 0x50]
0x1408b5a66 mov      edx, 0x2b
0x1408b5a6b call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x1408b5a70 jmp      0x1408b5fa8 ; 
0x1408b5a75 addss    xmm1, dword ptr [rsi + 0x11c]
0x1408b5a7d movss    dword ptr [rsi + 0x11c], xmm1
0x1408b5a85 mov      rax, qword ptr [rsi + 0x120]
0x1408b5a8c test     rax, rax
0x1408b5a8f js       0x1408b5b38
0x1408b5a95 cvtsi2ss xmm0, rax
0x1408b5a9a jmp      0x1408b5b4d ; 
0x1408b5a9f mov      rcx, rax
0x1408b5aa2 shr      rcx, 1
0x1408b5aa5 and      eax, 1
0x1408b5aa8 or       rax, rcx
0x1408b5aab cvtsi2ss xmm0, rax
0x1408b5ab0 addss    xmm0, xmm0
0x1408b5ab4 ucomiss  xmm1, xmm0
0x1408b5ab7 ja       0x1408b5b52
0x1408b5abd cmp      dword ptr [rsi + 0x40], 1
0x1408b5ac1 jne      0x1408b5bee
0x1408b5ac7 lea      rdx, [rsi + 0x48]
0x1408b5acb lea      rcx, [rbp + 0x30]
0x1408b5acf xor      r8d, r8d
0x1408b5ad2 call     0x141437440 ; libfmod::EventInstance::stop::h6aa257fdd68e9145
0x1408b5ad7 movabs   rax, 0x8000000000000004
0x1408b5ae1 cmp      qword ptr [rbp + 0x30], rax
0x1408b5ae5 je       0x1408b5bee
0x1408b5aeb mov      rax, qword ptr [rbp + 0x60]
0x1408b5aef mov      qword ptr [rbp - 0x20], rax
0x1408b5af3 movups   xmm0, xmmword ptr [rbp + 0x30]
0x1408b5af7 movups   xmm1, xmmword ptr [rbp + 0x40]
0x1408b5afb movups   xmm2, xmmword ptr [rbp + 0x50]
0x1408b5aff movaps   xmmword ptr [rbp - 0x30], xmm2
0x1408b5b03 movaps   xmmword ptr [rbp - 0x40], xmm1
0x1408b5b07 movaps   xmmword ptr [rbp - 0x50], xmm0
0x1408b5b0b lea      rax, [rip + 0x221d0f6]
0x1408b5b12 mov      qword ptr [rsp + 0x20], rax
0x1408b5b17 lea      rcx, [rip + 0x221aae2] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x1408b5b1e lea      r9, [rip + 0x221aa9b]
0x1408b5b25 lea      r8, [rbp - 0x50]
0x1408b5b29 mov      edx, 0x2b
0x1408b5b2e call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x1408b5b33 jmp      0x1408b5fa8 ; 
0x1408b5b38 mov      rcx, rax
0x1408b5b3b shr      rcx, 1
0x1408b5b3e and      eax, 1
0x1408b5b41 or       rax, rcx
0x1408b5b44 cvtsi2ss xmm0, rax
0x1408b5b49 addss    xmm0, xmm0
0x1408b5b4d ucomiss  xmm1, xmm0
0x1408b5b50 jbe      0x1408b5bc4
0x1408b5b52 movaps   xmm0, xmm1
0x1408b5b55 movaps   xmm6, xmm1
0x1408b5b58 call     0x142923e60 ; ceilf
0x1408b5b5d cvttss2si rax, xmm0
0x1408b5b62 mov      rcx, rax
0x1408b5b65 sar      rcx, 0x3f
0x1408b5b69 movaps   xmm1, xmm0
0x1408b5b6c subss    xmm1, dword ptr [rip + 0x20a4568]
0x1408b5b74 cvttss2si rdx, xmm1
0x1408b5b79 and      rdx, rcx
0x1408b5b7c or       rdx, rax
0x1408b5b7f xor      eax, eax
0x1408b5b81 xorps    xmm1, xmm1
0x1408b5b84 ucomiss  xmm0, xmm1
0x1408b5b87 cmovae   rax, rdx
0x1408b5b8b ucomiss  xmm0, dword ptr [rip + 0x20a454e]
0x1408b5b92 mov      rcx, 0xffffffffffffffff
0x1408b5b99 cmovbe   rcx, rax
0x1408b5b9d mov      qword ptr [rsi + 0x120], rcx
0x1408b5ba4 divss    xmm6, dword ptr [rip + 0x211754c]
0x1408b5bac call     0x140caf3c0 ; fastrand::f32::hbb5da0caede33dce
0x1408b5bb1 mulss    xmm6, xmm6
0x1408b5bb5 ucomiss  xmm6, xmm0
0x1408b5bb8 jbe      0x1408b5bc4
0x1408b5bba mov      dword ptr [rsi + 0x118], 3
0x1408b5bc4 cmp      dword ptr [rsi + 0x40], 1
0x1408b5bc8 jne      0x1408b5bee
0x1408b5bca lea      rdx, [rsi + 0x48]
0x1408b5bce lea      rcx, [rbp + 0x30]
0x1408b5bd2 xor      r8d, r8d
0x1408b5bd5 call     0x141437440 ; libfmod::EventInstance::stop::h6aa257fdd68e9145
0x1408b5bda movabs   rax, 0x8000000000000004
0x1408b5be4 cmp      qword ptr [rbp + 0x30], rax
0x1408b5be8 jne      0x1408b5e2f
0x1408b5bee cmp      qword ptr [rsi], 0
0x1408b5bf2 je       0x1408b5c9f
0x1408b5bf8 mov      rax, qword ptr [rsi + 8]
0x1408b5bfc cmp      rax, qword ptr [rsi + 0xf8]
0x1408b5c03 jae      0x1408b5c9f
0x1408b5c09 lea      rdi, [rax + rax*2]
0x1408b5c0d shl      rdi, 4
0x1408b5c11 add      rdi, qword ptr [rsi + 0xf0]
0x1408b5c18 mov      rax, qword ptr [rdi]
0x1408b5c1b cmp      rax, 1
0x1408b5c1f jne      0x1408b5c47
0x1408b5c21 lea      rdx, [rdi + 8]
0x1408b5c25 lea      rcx, [rbp + 0x30]
0x1408b5c29 call     0x141436fe0 ; libfmod::EventInstance::get_paused::h000fc06dc32eddbe
0x1408b5c2e movabs   rax, 0x8000000000000004
0x1408b5c38 cmp      qword ptr [rbp + 0x30], rax
0x1408b5c3c jne      0x1408b5cae
0x1408b5c3e cmp      byte ptr [rbp + 0x38], 1
0x1408b5c42 jne      0x1408b5c9f
0x1408b5c44 mov      rax, qword ptr [rdi]
0x1408b5c47 test     al, 1
0x1408b5c49 je       0x1408b5c72
0x1408b5c4b add      rdi, 8
0x1408b5c4f lea      rcx, [rbp + 0x30]
0x1408b5c53 mov      rdx, rdi
0x1408b5c56 xor      r8d, r8d
0x1408b5c59 call     0x141437160 ; libfmod::EventInstance::set_paused::h32fbd681052381c4
0x1408b5c5e movabs   rax, 0x8000000000000004
0x1408b5c68 cmp      qword ptr [rbp + 0x30], rax
0x1408b5c6c jne      0x1408b5cfb
0x1408b5c72 cmp      dword ptr [rsi + 0x10], 1
0x1408b5c76 jne      0x1408b5c9f
0x1408b5c78 add      rsi, 0x18
0x1408b5c7c lea      rcx, [rbp + 0x30]
0x1408b5c80 mov      rdx, rsi
0x1408b5c83 xor      r8d, r8d
0x1408b5c86 call     0x141437160 ; libfmod::EventInstance::set_paused::h32fbd681052381c4
0x1408b5c8b movabs   rax, 0x8000000000000004
0x1408b5c95 cmp      qword ptr [rbp + 0x30], rax
0x1408b5c99 jne      0x1408b5d48
0x1408b5c9f movaps   xmm6, xmmword ptr [rbp + 0x70]
0x1408b5ca3 add      rsp, 0x100
0x1408b5caa pop      rdi
0x1408b5cab pop      rsi
0x1408b5cac pop      rbp
0x1408b5cad ret      
0x1408b5cae mov      rax, qword ptr [rbp + 0x60]
0x1408b5cb2 mov      qword ptr [rbp - 0x20], rax
0x1408b5cb6 movups   xmm0, xmmword ptr [rbp + 0x30]
0x1408b5cba movups   xmm1, xmmword ptr [rbp + 0x40]
0x1408b5cbe movups   xmm2, xmmword ptr [rbp + 0x50]
0x1408b5cc2 movaps   xmmword ptr [rbp - 0x30], xmm2
0x1408b5cc6 movaps   xmmword ptr [rbp - 0x40], xmm1
0x1408b5cca movaps   xmmword ptr [rbp - 0x50], xmm0
0x1408b5cce lea      rax, [rip + 0x221cf7b]
0x1408b5cd5 mov      qword ptr [rsp + 0x20], rax
0x1408b5cda lea      rcx, [rip + 0x221a91f] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x1408b5ce1 lea      r9, [rip + 0x221a8d8]
0x1408b5ce8 lea      r8, [rbp - 0x50]
0x1408b5cec mov      edx, 0x2b
0x1408b5cf1 call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x1408b5cf6 jmp      0x1408b5fa8 ; 
0x1408b5cfb mov      rax, qword ptr [rbp + 0x60]
0x1408b5cff mov      qword ptr [rbp - 0x20], rax
0x1408b5d03 movups   xmm0, xmmword ptr [rbp + 0x30]
0x1408b5d07 movups   xmm1, xmmword ptr [rbp + 0x40]
0x1408b5d0b movups   xmm2, xmmword ptr [rbp + 0x50]
0x1408b5d0f movaps   xmmword ptr [rbp - 0x30], xmm2
0x1408b5d13 movaps   xmmword ptr [rbp - 0x40], xmm1
0x1408b5d17 movaps   xmmword ptr [rbp - 0x50], xmm0
0x1408b5d1b lea      rax, [rip + 0x221cf46]
0x1408b5d22 mov      qword ptr [rsp + 0x20], rax
0x1408b5d27 lea      rcx, [rip + 0x221a8d2] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x1408b5d2e lea      r9, [rip + 0x221a88b]
0x1408b5d35 lea      r8, [rbp - 0x50]
0x1408b5d39 mov      edx, 0x2b
0x1408b5d3e call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x1408b5d43 jmp      0x1408b5fa8 ; 
0x1408b5d48 mov      rax, qword ptr [rbp + 0x60]
0x1408b5d4c mov      qword ptr [rbp - 0x20], rax
0x1408b5d50 movups   xmm0, xmmword ptr [rbp + 0x30]
0x1408b5d54 movups   xmm1, xmmword ptr [rbp + 0x40]
0x1408b5d58 movups   xmm2, xmmword ptr [rbp + 0x50]
0x1408b5d5c movaps   xmmword ptr [rbp - 0x30], xmm2
0x1408b5d60 movaps   xmmword ptr [rbp - 0x40], xmm1
0x1408b5d64 movaps   xmmword ptr [rbp - 0x50], xmm0
0x1408b5d68 lea      rax, [rip + 0x221cef9]
0x1408b5d6f mov      qword ptr [rsp + 0x20], rax
0x1408b5d74 lea      rcx, [rip + 0x221a885] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x1408b5d7b lea      r9, [rip + 0x221a83e]
0x1408b5d82 lea      r8, [rbp - 0x50]
0x1408b5d86 mov      edx, 0x2b
0x1408b5d8b call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x1408b5d90 jmp      0x1408b5fa8 ; 
0x1408b5d95 mov      rax, qword ptr [rbp + 0x60]
0x1408b5d99 mov      qword ptr [rbp - 0x20], rax
0x1408b5d9d movups   xmm0, xmmword ptr [rbp + 0x30]
0x1408b5da1 movups   xmm1, xmmword ptr [rbp + 0x40]
0x1408b5da5 movups   xmm2, xmmword ptr [rbp + 0x50]
0x1408b5da9 movaps   xmmword ptr [rbp - 0x30], xmm2
0x1408b5dad movaps   xmmword ptr [rbp - 0x40], xmm1
0x1408b5db1 movaps   xmmword ptr [rbp - 0x50], xmm0
0x1408b5db5 lea      rax, [rip + 0x221ce4c]
0x1408b5dbc mov      qword ptr [rsp + 0x20], rax
0x1408b5dc1 lea      rcx, [rip + 0x221a838] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x1408b5dc8 lea      r9, [rip + 0x221a7f1]
0x1408b5dcf lea      r8, [rbp - 0x50]
0x1408b5dd3 mov      edx, 0x2b
0x1408b5dd8 call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x1408b5ddd jmp      0x1408b5fa8 ; 
0x1408b5de2 mov      rax, qword ptr [rbp + 0x60]
0x1408b5de6 mov      qword ptr [rbp - 0x20], rax
0x1408b5dea movups   xmm0, xmmword ptr [rbp + 0x30]
0x1408b5dee movups   xmm1, xmmword ptr [rbp + 0x40]
0x1408b5df2 movups   xmm2, xmmword ptr [rbp + 0x50]
0x1408b5df6 movaps   xmmword ptr [rbp - 0x30], xmm2
0x1408b5dfa movaps   xmmword ptr [rbp - 0x40], xmm1
0x1408b5dfe movaps   xmmword ptr [rbp - 0x50], xmm0
0x1408b5e02 lea      rax, [rip + 0x221cdb7]
0x1408b5e09 mov      qword ptr [rsp + 0x20], rax
0x1408b5e0e lea      rcx, [rip + 0x221a7eb] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x1408b5e15 lea      r9, [rip + 0x221a7a4]
0x1408b5e1c lea      r8, [rbp - 0x50]
0x1408b5e20 mov      edx, 0x2b
0x1408b5e25 call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x1408b5e2a jmp      0x1408b5fa8 ; 
0x1408b5e2f mov      rax, qword ptr [rbp + 0x60]
0x1408b5e33 mov      qword ptr [rbp - 0x20], rax
0x1408b5e37 movups   xmm0, xmmword ptr [rbp + 0x30]
0x1408b5e3b movups   xmm1, xmmword ptr [rbp + 0x40]
0x1408b5e3f movups   xmm2, xmmword ptr [rbp + 0x50]
0x1408b5e43 movaps   xmmword ptr [rbp - 0x30], xmm2
0x1408b5e47 movaps   xmmword ptr [rbp - 0x40], xmm1
0x1408b5e4b movaps   xmmword ptr [rbp - 0x50], xmm0
0x1408b5e4f lea      rax, [rip + 0x221cdb2]
0x1408b5e56 mov      qword ptr [rsp + 0x20], rax
0x1408b5e5b lea      rcx, [rip + 0x221a79e] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x1408b5e62 lea      r9, [rip + 0x221a757]
0x1408b5e69 lea      r8, [rbp - 0x50]
0x1408b5e6d mov      edx, 0x2b
0x1408b5e72 call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x1408b5e77 jmp      0x1408b5fa8 ; 
0x1408b5e7c mov      rax, qword ptr [rbp + 0x60]
0x1408b5e80 mov      qword ptr [rbp - 0x20], rax
0x1408b5e84 movups   xmm0, xmmword ptr [rbp + 0x30]
0x1408b5e88 movups   xmm1, xmmword ptr [rbp + 0x40]
0x1408b5e8c movups   xmm2, xmmword ptr [rbp + 0x50]
0x1408b5e90 movaps   xmmword ptr [rbp - 0x30], xmm2
0x1408b5e94 movaps   xmmword ptr [rbp - 0x40], xmm1
0x1408b5e98 movaps   xmmword ptr [rbp - 0x50], xmm0
0x1408b5e9c lea      rax, [rip + 0x221cd1d]
0x1408b5ea3 mov      qword ptr [rsp + 0x20], rax
0x1408b5ea8 lea      rcx, [rip + 0x221a751] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x1408b5eaf lea      r9, [rip + 0x221a70a]
0x1408b5eb6 lea      r8, [rbp - 0x50]
0x1408b5eba mov      edx, 0x2b
0x1408b5ebf call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x1408b5ec4 jmp      0x1408b5fa8 ; 
0x1408b5ec9 mov      rax, qword ptr [rbp + 0x60]
0x1408b5ecd mov      qword ptr [rbp - 0x20], rax
0x1408b5ed1 movups   xmm0, xmmword ptr [rbp + 0x30]
0x1408b5ed5 movups   xmm1, xmmword ptr [rbp + 0x40]
0x1408b5ed9 movups   xmm2, xmmword ptr [rbp + 0x50]
0x1408b5edd movaps   xmmword ptr [rbp - 0x30], xmm2
0x1408b5ee1 movaps   xmmword ptr [rbp - 0x40], xmm1
0x1408b5ee5 movaps   xmmword ptr [rbp - 0x50], xmm0
0x1408b5ee9 lea      rax, [rip + 0x221ccd0]
0x1408b5ef0 mov      qword ptr [rsp + 0x20], rax
0x1408b5ef5 lea      rcx, [rip + 0x221a704] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x1408b5efc lea      r9, [rip + 0x221a6bd]
0x1408b5f03 lea      r8, [rbp - 0x50]
0x1408b5f07 mov      edx, 0x2b
0x1408b5f0c call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x1408b5f11 jmp      0x1408b5fa8 ; 
0x1408b5f16 mov      rax, qword ptr [rbp + 0x60]
0x1408b5f1a mov      qword ptr [rbp - 0x20], rax
0x1408b5f1e movups   xmm0, xmmword ptr [rbp + 0x30]
0x1408b5f22 movups   xmm1, xmmword ptr [rbp + 0x40]
0x1408b5f26 movups   xmm2, xmmword ptr [rbp + 0x50]
0x1408b5f2a movaps   xmmword ptr [rbp - 0x30], xmm2
0x1408b5f2e movaps   xmmword ptr [rbp - 0x40], xmm1
0x1408b5f32 movaps   xmmword ptr [rbp - 0x50], xmm0
0x1408b5f36 lea      rax, [rip + 0x221cce3]
0x1408b5f3d mov      qword ptr [rsp + 0x20], rax
0x1408b5f42 lea      rcx, [rip + 0x221a6b7] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x1408b5f49 lea      r9, [rip + 0x221a670]
0x1408b5f50 lea      r8, [rbp - 0x50]
0x1408b5f54 mov      edx, 0x2b
0x1408b5f59 call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x1408b5f5e jmp      0x1408b5fa8 ; 
0x1408b5f60 mov      rax, qword ptr [rbp + 0x60]
0x1408b5f64 mov      qword ptr [rbp - 0x20], rax
0x1408b5f68 movups   xmm0, xmmword ptr [rbp + 0x30]
0x1408b5f6c movups   xmm1, xmmword ptr [rbp + 0x40]
0x1408b5f70 movups   xmm2, xmmword ptr [rbp + 0x50]
0x1408b5f74 movaps   xmmword ptr [rbp - 0x30], xmm2
0x1408b5f78 movaps   xmmword ptr [rbp - 0x40], xmm1
0x1408b5f7c movaps   xmmword ptr [rbp - 0x50], xmm0
0x1408b5f80 lea      rax, [rip + 0x221cc81]
0x1408b5f87 mov      qword ptr [rsp + 0x20], rax
0x1408b5f8c lea      rcx, [rip + 0x221a66d] ; @0x142ad0600 'called `Result::unwrap()` on an `Err` value'
0x1408b5f93 lea      r9, [rip + 0x221a626]
0x1408b5f9a lea      r8, [rbp - 0x50]
0x1408b5f9e mov      edx, 0x2b
0x1408b5fa3 call     0x1428d9760 ; core::result::unwrap_failed::h96d74ae09566f4b3
0x1408b5fa8 ud2      
0x1408b5faa nop      word ptr [rax + rax]
0x1408b5fb0 mov      qword ptr [rsp + 0x10], rdx
0x1408b5fb5 push     rbp
0x1408b5fb6 push     rsi
0x1408b5fb7 push     rdi
0x1408b5fb8 sub      rsp, 0x40
0x1408b5fbc lea      rbp, [rdx + 0x80]
0x1408b5fc3 movaps   xmmword ptr [rsp + 0x20], xmm6
0x1408b5fc8 lea      rcx, [rbp - 0x50]
0x1408b5fcc call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x1408b5fd1 movaps   xmm6, xmmword ptr [rsp + 0x20]
0x1408b5fd6 add      rsp, 0x40
0x1408b5fda pop      rdi
0x1408b5fdb pop      rsi
0x1408b5fdc pop      rbp
0x1408b5fdd ret      
0x1408b5fde nop      
0x1408b5fe0 mov      qword ptr [rsp + 0x10], rdx
0x1408b5fe5 push     rbp
0x1408b5fe6 push     rsi
0x1408b5fe7 push     rdi
0x1408b5fe8 sub      rsp, 0x40
0x1408b5fec lea      rbp, [rdx + 0x80]
0x1408b5ff3 movaps   xmmword ptr [rsp + 0x20], xmm6
0x1408b5ff8 lea      rcx, [rbp - 0x50]
0x1408b5ffc call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x1408b6001 movaps   xmm6, xmmword ptr [rsp + 0x20]
0x1408b6006 add      rsp, 0x40
0x1408b600a pop      rdi
0x1408b600b pop      rsi
0x1408b600c pop      rbp
0x1408b600d ret      
0x1408b600e nop      
0x1408b6010 mov      qword ptr [rsp + 0x10], rdx
0x1408b6015 push     rbp
0x1408b6016 push     rsi
0x1408b6017 push     rdi
0x1408b6018 sub      rsp, 0x40
0x1408b601c lea      rbp, [rdx + 0x80]
0x1408b6023 movaps   xmmword ptr [rsp + 0x20], xmm6
0x1408b6028 lea      rcx, [rbp - 0x50]
0x1408b602c call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x1408b6031 movaps   xmm6, xmmword ptr [rsp + 0x20]
0x1408b6036 add      rsp, 0x40
0x1408b603a pop      rdi
0x1408b603b pop      rsi
0x1408b603c pop      rbp
0x1408b603d ret      
0x1408b603e nop      
0x1408b6040 mov      qword ptr [rsp + 0x10], rdx
0x1408b6045 push     rbp
0x1408b6046 push     rsi
0x1408b6047 push     rdi
0x1408b6048 sub      rsp, 0x40
0x1408b604c lea      rbp, [rdx + 0x80]
0x1408b6053 movaps   xmmword ptr [rsp + 0x20], xmm6
0x1408b6058 lea      rcx, [rbp - 0x50]
0x1408b605c call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x1408b6061 movaps   xmm6, xmmword ptr [rsp + 0x20]
0x1408b6066 add      rsp, 0x40
0x1408b606a pop      rdi
0x1408b606b pop      rsi
0x1408b606c pop      rbp
0x1408b606d ret      
0x1408b606e nop      
0x1408b6070 mov      qword ptr [rsp + 0x10], rdx
0x1408b6075 push     rbp
0x1408b6076 push     rsi
0x1408b6077 push     rdi
0x1408b6078 sub      rsp, 0x40
0x1408b607c lea      rbp, [rdx + 0x80]
0x1408b6083 movaps   xmmword ptr [rsp + 0x20], xmm6
0x1408b6088 lea      rcx, [rbp - 0x50]
0x1408b608c call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x1408b6091 movaps   xmm6, xmmword ptr [rsp + 0x20]
0x1408b6096 add      rsp, 0x40
0x1408b609a pop      rdi
0x1408b609b pop      rsi
0x1408b609c pop      rbp
0x1408b609d ret      
0x1408b609e nop      
0x1408b60a0 mov      qword ptr [rsp + 0x10], rdx
0x1408b60a5 push     rbp
0x1408b60a6 push     rsi
0x1408b60a7 push     rdi
0x1408b60a8 sub      rsp, 0x40
0x1408b60ac lea      rbp, [rdx + 0x80]
0x1408b60b3 movaps   xmmword ptr [rsp + 0x20], xmm6
0x1408b60b8 lea      rcx, [rbp - 0x50]
0x1408b60bc call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x1408b60c1 movaps   xmm6, xmmword ptr [rsp + 0x20]
0x1408b60c6 add      rsp, 0x40
0x1408b60ca pop      rdi
0x1408b60cb pop      rsi
0x1408b60cc pop      rbp
0x1408b60cd ret      
0x1408b60ce nop      
0x1408b60d0 mov      qword ptr [rsp + 0x10], rdx
0x1408b60d5 push     rbp
0x1408b60d6 push     rsi
0x1408b60d7 push     rdi
0x1408b60d8 sub      rsp, 0x40
0x1408b60dc lea      rbp, [rdx + 0x80]
0x1408b60e3 movaps   xmmword ptr [rsp + 0x20], xmm6
0x1408b60e8 lea      rcx, [rbp - 0x50]
0x1408b60ec call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x1408b60f1 movaps   xmm6, xmmword ptr [rsp + 0x20]
0x1408b60f6 add      rsp, 0x40
0x1408b60fa pop      rdi
0x1408b60fb pop      rsi
0x1408b60fc pop      rbp
0x1408b60fd ret      
0x1408b60fe nop      
0x1408b6100 mov      qword ptr [rsp + 0x10], rdx
0x1408b6105 push     rbp
0x1408b6106 push     rsi
0x1408b6107 push     rdi
0x1408b6108 sub      rsp, 0x40
0x1408b610c lea      rbp, [rdx + 0x80]
0x1408b6113 movaps   xmmword ptr [rsp + 0x20], xmm6
0x1408b6118 lea      rcx, [rbp - 0x50]
0x1408b611c call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x1408b6121 movaps   xmm6, xmmword ptr [rsp + 0x20]
0x1408b6126 add      rsp, 0x40
0x1408b612a pop      rdi
0x1408b612b pop      rsi
0x1408b612c pop      rbp
0x1408b612d ret      
0x1408b612e nop      
0x1408b6130 mov      qword ptr [rsp + 0x10], rdx
0x1408b6135 push     rbp
0x1408b6136 push     rsi
0x1408b6137 push     rdi
0x1408b6138 sub      rsp, 0x40
0x1408b613c lea      rbp, [rdx + 0x80]
0x1408b6143 movaps   xmmword ptr [rsp + 0x20], xmm6
0x1408b6148 lea      rcx, [rbp - 0x50]
0x1408b614c call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x1408b6151 movaps   xmm6, xmmword ptr [rsp + 0x20]
0x1408b6156 add      rsp, 0x40
0x1408b615a pop      rdi
0x1408b615b pop      rsi
0x1408b615c pop      rbp
0x1408b615d ret      
0x1408b615e nop      
0x1408b6160 mov      qword ptr [rsp + 0x10], rdx
0x1408b6165 push     rbp
0x1408b6166 push     rsi
0x1408b6167 push     rdi
0x1408b6168 sub      rsp, 0x40
0x1408b616c lea      rbp, [rdx + 0x80]
0x1408b6173 movaps   xmmword ptr [rsp + 0x20], xmm6
0x1408b6178 lea      rcx, [rbp - 0x50]
0x1408b617c call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x1408b6181 movaps   xmm6, xmmword ptr [rsp + 0x20]
0x1408b6186 add      rsp, 0x40
0x1408b618a pop      rdi
0x1408b618b pop      rsi
0x1408b618c pop      rbp
0x1408b618d ret      
0x1408b618e nop      
0x1408b6190 mov      qword ptr [rsp + 0x10], rdx
0x1408b6195 push     rbp
0x1408b6196 push     rsi
0x1408b6197 push     rdi
0x1408b6198 sub      rsp, 0x40
0x1408b619c lea      rbp, [rdx + 0x80]
0x1408b61a3 movaps   xmmword ptr [rsp + 0x20], xmm6
0x1408b61a8 lea      rcx, [rbp - 0x50]
0x1408b61ac call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x1408b61b1 movaps   xmm6, xmmword ptr [rsp + 0x20]
0x1408b61b6 add      rsp, 0x40
0x1408b61ba pop      rdi
0x1408b61bb pop      rsi
0x1408b61bc pop      rbp
0x1408b61bd ret      
0x1408b61be nop      
0x1408b61c0 mov      qword ptr [rsp + 0x10], rdx
0x1408b61c5 push     rbp
0x1408b61c6 push     rsi
0x1408b61c7 push     rdi
0x1408b61c8 sub      rsp, 0x40
0x1408b61cc lea      rbp, [rdx + 0x80]
0x1408b61d3 movaps   xmmword ptr [rsp + 0x20], xmm6
0x1408b61d8 lea      rcx, [rbp - 0x50]
0x1408b61dc call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x1408b61e1 movaps   xmm6, xmmword ptr [rsp + 0x20]
0x1408b61e6 add      rsp, 0x40
0x1408b61ea pop      rdi
0x1408b61eb pop      rsi
0x1408b61ec pop      rbp
0x1408b61ed ret      
0x1408b61ee nop      
0x1408b61f0 mov      qword ptr [rsp + 0x10], rdx
0x1408b61f5 push     rbp
0x1408b61f6 push     rsi
0x1408b61f7 push     rdi
0x1408b61f8 sub      rsp, 0x40
0x1408b61fc lea      rbp, [rdx + 0x80]
0x1408b6203 movaps   xmmword ptr [rsp + 0x20], xmm6
0x1408b6208 lea      rcx, [rbp - 0x50]
0x1408b620c call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x1408b6211 movaps   xmm6, xmmword ptr [rsp + 0x20]
0x1408b6216 add      rsp, 0x40
0x1408b621a pop      rdi
0x1408b621b pop      rsi
0x1408b621c pop      rbp
0x1408b621d ret      
0x1408b621e nop      
0x1408b6220 mov      qword ptr [rsp + 0x10], rdx
0x1408b6225 push     rbp
0x1408b6226 push     rsi
0x1408b6227 push     rdi
0x1408b6228 sub      rsp, 0x40
0x1408b622c lea      rbp, [rdx + 0x80]
0x1408b6233 movaps   xmmword ptr [rsp + 0x20], xmm6
0x1408b6238 lea      rcx, [rbp - 0x50]
0x1408b623c call     0x140a526b0 ; core::ptr::drop_in_place$LT$libfmod..Error$GT$::h60b34b2c39a0f0f1 (.llvm.4456468861159116502)
0x1408b6241 movaps   xmm6, xmmword ptr [rsp + 0x20]
0x1408b6246 add      rsp, 0x40
0x1408b624a pop      rdi
0x1408b624b pop      rsi
0x1408b624c pop      rbp
0x1408b624d ret      