0x140a478f0 push     r15
0x140a478f2 push     r14
0x140a478f4 push     r13
0x140a478f6 push     r12
0x140a478f8 push     rsi
0x140a478f9 push     rdi
0x140a478fa push     rbp
0x140a478fb push     rbx
0x140a478fc sub      rsp, 0x38
0x140a47900 mov      rsi, rdx
0x140a47903 movzx    eax, byte ptr [rcx]
0x140a47906 add      al, 0xc8
0x140a47908 cmp      al, 8
0x140a4790a movzx    eax, al
0x140a4790d mov      edx, 6
0x140a47912 cmovb    edx, eax
0x140a47915 movzx    eax, dl
0x140a47918 lea      rdx, [rip + 0x2087e29]
0x140a4791f movsxd   rax, dword ptr [rdx + rax*4]
0x140a47923 add      rax, rdx
0x140a47926 jmp      rax
0x140a47928 lea      rdx, [rsi + 8]
0x140a4792c lea      r8, [rip + 0x20873f5] ; @0x142aced28 'CottageWindowGothicWindowArrowSlitChimneyLanternFlag'
0x140a47933 mov      r9d, 0xd
0x140a47939 jmp      0x140a479ab ; 
0x140a4793b lea      rdx, [rsi + 8]
0x140a4793f lea      r8, [rip + 0x208740b] ; @0x142aced51 'LanternFlag'
0x140a47946 mov      r9d, 7
0x140a4794c jmp      0x140a479ab ; 
0x140a4794e lea      rdx, [rsi + 8]
0x140a47952 lea      r8, [rip + 0x20873e8] ; @0x142aced41 'ArrowSlitChimneyLanternFlag'
0x140a47959 mov      r9d, 9
0x140a4795f jmp      0x140a479ab ; 
0x140a47961 lea      rdx, [rsi + 8]
0x140a47965 lea      r8, [rip + 0x20873de] ; @0x142aced4a 'ChimneyLanternFlag'
0x140a4796c mov      r9d, 7
0x140a47972 jmp      0x140a479ab ; 
0x140a47974 lea      rdx, [rsi + 8]
0x140a47978 lea      r8, [rip + 0x208740a] ; @0x142aced89 'StairsDecoratorType'
0x140a4797f mov      r9d, 6
0x140a47985 jmp      0x140a479ab ; 
0x140a47987 lea      rdx, [rsi + 8]
0x140a4798b lea      r8, [rip + 0x20873a3] ; @0x142aced35 'GothicWindowArrowSlitChimneyLanternFlag'
0x140a47992 mov      r9d, 0xc
0x140a47998 jmp      0x140a479ab ; 
0x140a4799a lea      rdx, [rsi + 8]
0x140a4799e lea      r8, [rip + 0x20873b3] ; @0x142aced58 'Flag'
0x140a479a5 mov      r9d, 4
0x140a479ab mov      rcx, rsi
0x140a479ae call     0x141580910 ; serde_json::ser::format_escaped_str::h775c3d29342bbbda
0x140a479b3 test     rax, rax
0x140a479b6 jne      0x140a47c3e
0x140a479bc xor      eax, eax
0x140a479be add      rsp, 0x38
0x140a479c2 pop      rbx
0x140a479c3 pop      rbp
0x140a479c4 pop      rdi
0x140a479c5 pop      rsi
0x140a479c6 pop      r12
0x140a479c8 pop      r13
0x140a479ca pop      r14
0x140a479cc pop      r15
0x140a479ce ret      
0x140a479cf mov      r14, qword ptr [rsi]
0x140a479d2 mov      rbp, qword ptr [rsi + 0x18]
0x140a479d6 inc      rbp
0x140a479d9 mov      qword ptr [rsi + 0x18], rbp
0x140a479dd mov      byte ptr [rsi + 0x20], 0
0x140a479e1 mov      rdx, qword ptr [r14 + 0x10]
0x140a479e5 cmp      qword ptr [r14], rdx
0x140a479e8 mov      rdi, rcx
0x140a479eb je       0x140a47c87
0x140a479f1 mov      rax, qword ptr [r14 + 8]
0x140a479f5 mov      byte ptr [rax + rdx], 0x7b
0x140a479f9 inc      rdx
0x140a479fc mov      qword ptr [r14 + 0x10], rdx
0x140a47a00 cmp      qword ptr [r14], rdx
0x140a47a03 je       0x140a47cad
0x140a47a09 lea      rbx, [rsi + 8]
0x140a47a0d mov      rax, qword ptr [r14 + 8]
0x140a47a11 mov      byte ptr [rax + rdx], 0xa
0x140a47a15 inc      rdx
0x140a47a18 mov      qword ptr [r14 + 0x10], rdx
0x140a47a1c test     rbp, rbp
0x140a47a1f je       0x140a47a92
0x140a47a21 mov      r14, qword ptr [rsi + 8]
0x140a47a25 mov      r15, qword ptr [rsi + 0x10]
0x140a47a29 mov      r12, qword ptr [rsi]
0x140a47a2c mov      r13, qword ptr [r12 + 0x10]
0x140a47a31 nop      word ptr cs:[rax + rax]
0x140a47a40 mov      rax, qword ptr [r12]
0x140a47a44 sub      rax, r13
0x140a47a47 cmp      rax, r15
0x140a47a4a jb       0x140a47a6e
0x140a47a4c mov      rcx, qword ptr [r12 + 8]
0x140a47a51 add      rcx, r13
0x140a47a54 mov      rdx, r14
0x140a47a57 mov      r8, r15
0x140a47a5a call     0x142923c00 ; memcpy
0x140a47a5f add      r13, r15
0x140a47a62 mov      qword ptr [r12 + 0x10], r13
0x140a47a67 dec      rbp
0x140a47a6a jne      0x140a47a40
0x140a47a6c jmp      0x140a47a92 ; 
0x140a47a6e mov      qword ptr [rsp + 0x20], 1
0x140a47a77 mov      r9d, 1
0x140a47a7d mov      rcx, r12
0x140a47a80 mov      rdx, r13
0x140a47a83 mov      r8, r15
0x140a47a86 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140a47a8b mov      r13, qword ptr [r12 + 0x10]
0x140a47a90 jmp      0x140a47a4c ; 
0x140a47a92 lea      r8, [rip + 0x20872e7] ; @0x142aced80 'CluttertyStairsDecoratorType'
0x140a47a99 mov      r9d, 7
0x140a47a9f mov      rcx, rsi
0x140a47aa2 mov      rdx, rbx
0x140a47aa5 call     0x141580910 ; serde_json::ser::format_escaped_str::h775c3d29342bbbda
0x140a47aaa test     rax, rax
0x140a47aad jne      0x140a47c7a
0x140a47ab3 mov      rbx, qword ptr [rsi]
0x140a47ab6 mov      rax, qword ptr [rbx]
0x140a47ab9 mov      rdx, qword ptr [rbx + 0x10]
0x140a47abd sub      rax, rdx
0x140a47ac0 cmp      rax, 1
0x140a47ac4 jbe      0x140a47cd3
0x140a47aca mov      rax, qword ptr [rbx + 8]
0x140a47ace mov      word ptr [rax + rdx], 0x203a
0x140a47ad4 add      rdx, 2
0x140a47ad8 mov      qword ptr [rbx + 0x10], rdx
0x140a47adc mov      rbx, qword ptr [rsi]
0x140a47adf inc      qword ptr [rsi + 0x18]
0x140a47ae3 mov      byte ptr [rsi + 0x20], 0
0x140a47ae7 mov      rdx, qword ptr [rbx + 0x10]
0x140a47aeb cmp      qword ptr [rbx], rdx
0x140a47aee je       0x140a47cf9
0x140a47af4 mov      rax, qword ptr [rbx + 8]
0x140a47af8 mov      byte ptr [rax + rdx], 0x7b
0x140a47afc inc      rdx
0x140a47aff mov      qword ptr [rbx + 0x10], rdx
0x140a47b03 mov      qword ptr [rsp + 0x28], rsi
0x140a47b08 mov      byte ptr [rsp + 0x30], 1
0x140a47b0d lea      rdx, [rip + 0x2087273] ; @0x142aced87 'tyStairsDecoratorType'
0x140a47b14 lea      rcx, [rsp + 0x28]
0x140a47b19 mov      r8d, 2
0x140a47b1f mov      r9, rdi
0x140a47b22 call     0x1415882d0 ; serde::ser::SerializeMap::serialize_entry::h8dc024a0c2c21910
0x140a47b27 test     rax, rax
0x140a47b2a jne      0x140a479be
0x140a47b30 mov      rsi, qword ptr [rsp + 0x28]
0x140a47b35 cmp      byte ptr [rsp + 0x30], 0
0x140a47b3a je       0x140a47ba4
0x140a47b3c mov      rdi, qword ptr [rsi + 0x18]
0x140a47b40 dec      rdi
0x140a47b43 mov      qword ptr [rsi + 0x18], rdi
0x140a47b47 cmp      byte ptr [rsi + 0x20], 0
0x140a47b4b je       0x140a47b85
0x140a47b4d mov      rcx, qword ptr [rsi]
0x140a47b50 lea      rdx, [rip + 0x2086dd1]
0x140a47b57 lea      r9, [rip + 0x208698a]
0x140a47b5e mov      r8d, 1
0x140a47b64 call     0x1412f4520 ; alloc::vec::Vec$LT$T$C$A$GT$::extend_from_slice::h70c862da947354be
0x140a47b69 mov      r8, qword ptr [rsi + 8]
0x140a47b6d mov      r9, qword ptr [rsi + 0x10]
0x140a47b71 mov      rcx, rsi
0x140a47b74 mov      rdx, rdi
0x140a47b77 call     0x141581130 ; serde_json::ser::indent::ha2990fb709079cf8
0x140a47b7c test     rax, rax
0x140a47b7f jne      0x140a47c7a
0x140a47b85 mov      rdi, qword ptr [rsi]
0x140a47b88 mov      rdx, qword ptr [rdi + 0x10]
0x140a47b8c cmp      qword ptr [rdi], rdx
0x140a47b8f je       0x140a47d6b
0x140a47b95 mov      rax, qword ptr [rdi + 8]
0x140a47b99 mov      byte ptr [rax + rdx], 0x7d
0x140a47b9d inc      rdx
0x140a47ba0 mov      qword ptr [rdi + 0x10], rdx
0x140a47ba4 mov      byte ptr [rsi + 0x20], 1
0x140a47ba8 mov      rdi, qword ptr [rsi]
0x140a47bab mov      r12, qword ptr [rsi + 0x18]
0x140a47baf dec      r12
0x140a47bb2 mov      qword ptr [rsi + 0x18], r12
0x140a47bb6 mov      rdx, qword ptr [rdi + 0x10]
0x140a47bba cmp      qword ptr [rdi], rdx
0x140a47bbd je       0x140a47d1f
0x140a47bc3 mov      rax, qword ptr [rdi + 8]
0x140a47bc7 mov      byte ptr [rax + rdx], 0xa
0x140a47bcb inc      rdx
0x140a47bce mov      qword ptr [rdi + 0x10], rdx
0x140a47bd2 test     r12, r12
0x140a47bd5 je       0x140a47c56
0x140a47bd7 mov      rdi, qword ptr [rsi + 8]
0x140a47bdb mov      rbx, qword ptr [rsi + 0x10]
0x140a47bdf mov      r14, qword ptr [rsi]
0x140a47be2 mov      r15, qword ptr [r14 + 0x10]
0x140a47be6 nop      word ptr cs:[rax + rax]
0x140a47bf0 mov      rax, qword ptr [r14]
0x140a47bf3 sub      rax, r15
0x140a47bf6 cmp      rax, rbx
0x140a47bf9 jb       0x140a47c1b
0x140a47bfb mov      rcx, qword ptr [r14 + 8]
0x140a47bff add      rcx, r15
0x140a47c02 mov      rdx, rdi
0x140a47c05 mov      r8, rbx
0x140a47c08 call     0x142923c00 ; memcpy
0x140a47c0d add      r15, rbx
0x140a47c10 mov      qword ptr [r14 + 0x10], r15
0x140a47c14 dec      r12
0x140a47c17 jne      0x140a47bf0
0x140a47c19 jmp      0x140a47c56 ; 
0x140a47c1b mov      qword ptr [rsp + 0x20], 1
0x140a47c24 mov      r9d, 1
0x140a47c2a mov      rcx, r14
0x140a47c2d mov      rdx, r15
0x140a47c30 mov      r8, rbx
0x140a47c33 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140a47c38 mov      r15, qword ptr [r14 + 0x10]
0x140a47c3c jmp      0x140a47bfb ; 
0x140a47c3e mov      rcx, rax
0x140a47c41 add      rsp, 0x38
0x140a47c45 pop      rbx
0x140a47c46 pop      rbp
0x140a47c47 pop      rdi
0x140a47c48 pop      rsi
0x140a47c49 pop      r12
0x140a47c4b pop      r13
0x140a47c4d pop      r14
0x140a47c4f pop      r15
0x140a47c51 jmp      0x1428c98a0 ; serde_json::error::Error::io::hfcf27584d0d6aa9f
0x140a47c56 mov      rsi, qword ptr [rsi]
0x140a47c59 mov      rdx, qword ptr [rsi + 0x10]
0x140a47c5d cmp      qword ptr [rsi], rdx
0x140a47c60 je       0x140a47d45
0x140a47c66 mov      rax, qword ptr [rsi + 8]
0x140a47c6a mov      byte ptr [rax + rdx], 0x7d
0x140a47c6e inc      rdx
0x140a47c71 mov      qword ptr [rsi + 0x10], rdx
0x140a47c75 jmp      0x140a479bc ; 
0x140a47c7a mov      rcx, rax
0x140a47c7d call     0x1428c98a0 ; serde_json::error::Error::io::hfcf27584d0d6aa9f
0x140a47c82 jmp      0x140a479be ; 
0x140a47c87 mov      qword ptr [rsp + 0x20], 1
0x140a47c90 mov      r8d, 1
0x140a47c96 mov      r9d, 1
0x140a47c9c mov      rcx, r14
0x140a47c9f call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140a47ca4 mov      rdx, qword ptr [r14 + 0x10]
0x140a47ca8 jmp      0x140a479f1 ; 
0x140a47cad mov      qword ptr [rsp + 0x20], 1
0x140a47cb6 mov      r8d, 1
0x140a47cbc mov      r9d, 1
0x140a47cc2 mov      rcx, r14
0x140a47cc5 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140a47cca mov      rdx, qword ptr [r14 + 0x10]
0x140a47cce jmp      0x140a47a09 ; 
0x140a47cd3 mov      qword ptr [rsp + 0x20], 1
0x140a47cdc mov      r8d, 2
0x140a47ce2 mov      r9d, 1
0x140a47ce8 mov      rcx, rbx
0x140a47ceb call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140a47cf0 mov      rdx, qword ptr [rbx + 0x10]
0x140a47cf4 jmp      0x140a47aca ; 
0x140a47cf9 mov      qword ptr [rsp + 0x20], 1
0x140a47d02 mov      r8d, 1
0x140a47d08 mov      r9d, 1
0x140a47d0e mov      rcx, rbx
0x140a47d11 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140a47d16 mov      rdx, qword ptr [rbx + 0x10]
0x140a47d1a jmp      0x140a47af4 ; 
0x140a47d1f mov      qword ptr [rsp + 0x20], 1
0x140a47d28 mov      r8d, 1
0x140a47d2e mov      r9d, 1
0x140a47d34 mov      rcx, rdi
0x140a47d37 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140a47d3c mov      rdx, qword ptr [rdi + 0x10]
0x140a47d40 jmp      0x140a47bc3 ; 
0x140a47d45 mov      qword ptr [rsp + 0x20], 1
0x140a47d4e mov      r8d, 1
0x140a47d54 mov      r9d, 1
0x140a47d5a mov      rcx, rsi
0x140a47d5d call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140a47d62 mov      rdx, qword ptr [rsi + 0x10]
0x140a47d66 jmp      0x140a47c66 ; 
0x140a47d6b mov      qword ptr [rsp + 0x20], 1
0x140a47d74 mov      r8d, 1
0x140a47d7a mov      r9d, 1
0x140a47d80 mov      rcx, rdi
0x140a47d83 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140a47d88 mov      rdx, qword ptr [rdi + 0x10]
0x140a47d8c jmp      0x140a47b95 ; 