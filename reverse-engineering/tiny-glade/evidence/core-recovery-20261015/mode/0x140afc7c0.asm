0x140afc7c0 push     r15
0x140afc7c2 push     r14
0x140afc7c4 push     r13
0x140afc7c6 push     r12
0x140afc7c8 push     rsi
0x140afc7c9 push     rdi
0x140afc7ca push     rbp
0x140afc7cb push     rbx
0x140afc7cc sub      rsp, 0x68
0x140afc7d0 mov      ebp, dword ptr [rcx]
0x140afc7d2 lea      eax, [rbp - 6]
0x140afc7d5 mov      r8, rbp
0x140afc7d8 add      r8, -5
0x140afc7dc xor      r9d, r9d
0x140afc7df cmp      eax, 0xf
0x140afc7e2 cmovb    r9, r8
0x140afc7e6 mov      rsi, rdx
0x140afc7e9 lea      rax, [rip + 0x1fe99fc]
0x140afc7f0 movsxd   rdx, dword ptr [rax + r9*4]
0x140afc7f4 add      rdx, rax
0x140afc7f7 jmp      rdx
0x140afc7f9 mov      rbx, qword ptr [rsi]
0x140afc7fc mov      r13, qword ptr [rsi + 0x18]
0x140afc800 inc      r13
0x140afc803 mov      qword ptr [rsi + 0x18], r13
0x140afc807 mov      byte ptr [rsi + 0x20], 0
0x140afc80b mov      rdx, qword ptr [rbx + 0x10]
0x140afc80f cmp      qword ptr [rbx], rdx
0x140afc812 je       0x140afd647
0x140afc818 mov      rax, qword ptr [rbx + 8]
0x140afc81c mov      byte ptr [rax + rdx], 0x7b
0x140afc820 inc      rdx
0x140afc823 mov      qword ptr [rbx + 0x10], rdx
0x140afc827 cmp      qword ptr [rbx], rdx
0x140afc82a je       0x140afd673
0x140afc830 mov      qword ptr [rsp + 0x28], rcx
0x140afc835 lea      rdi, [rsi + 8]
0x140afc839 mov      rax, qword ptr [rbx + 8]
0x140afc83d mov      byte ptr [rax + rdx], 0xa
0x140afc841 inc      rdx
0x140afc844 mov      qword ptr [rbx + 0x10], rdx
0x140afc848 test     r13, r13
0x140afc84b je       0x140afccf1
0x140afc851 mov      rbx, qword ptr [rsi + 8]
0x140afc855 mov      r14, qword ptr [rsi + 0x10]
0x140afc859 mov      r15, qword ptr [rsi]
0x140afc85c mov      r12, qword ptr [r15 + 0x10]
0x140afc860 mov      rax, qword ptr [r15]
0x140afc863 sub      rax, r12
0x140afc866 cmp      rax, r14
0x140afc869 jb       0x140afc88e
0x140afc86b mov      rcx, qword ptr [r15 + 8]
0x140afc86f add      rcx, r12
0x140afc872 mov      rdx, rbx
0x140afc875 mov      r8, r14
0x140afc878 call     0x142923c00 ; memcpy
0x140afc87d add      r12, r14
0x140afc880 mov      qword ptr [r15 + 0x10], r12
0x140afc884 dec      r13
0x140afc887 jne      0x140afc860
0x140afc889 jmp      0x140afccf1 ; 
0x140afc88e mov      qword ptr [rsp + 0x20], 1
0x140afc897 mov      r9d, 1
0x140afc89d mov      rcx, r15
0x140afc8a0 mov      rdx, r12
0x140afc8a3 mov      r8, r14
0x140afc8a6 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afc8ab mov      r12, qword ptr [r15 + 0x10]
0x140afc8af jmp      0x140afc86b ; 
0x140afc8b1 lea      rdx, [rsi + 8]
0x140afc8b5 lea      r8, [rip + 0x1fe8f27] ; @0x142ae57e3 'PhotomodeTimeOfDayPhotomodeFolderPhotomodeCameraPhotomodeFovPhotomodeFocusPhotomodeDofPhotomodeTakeP'
0x140afc8bc mov      r9d, 9
0x140afc8c2 jmp      0x140afcfe5 ; 
0x140afc8c7 lea      rdx, [rsi + 8]
0x140afc8cb lea      r8, [rip + 0x1fe8ef4] ; @0x142ae57c6 'TreeTerrainEditingWaterGardenPhotomodeTimeOfDayPhotomodeFolderPhotomodeCameraPhotomodeFovPhotomodeFo'
0x140afc8d2 jmp      0x140afca65 ; 
0x140afc8d7 movzx    ebp, byte ptr [rcx + 4]
0x140afc8db mov      rbx, qword ptr [rsi]
0x140afc8de mov      r13, qword ptr [rsi + 0x18]
0x140afc8e2 inc      r13
0x140afc8e5 mov      qword ptr [rsi + 0x18], r13
0x140afc8e9 mov      byte ptr [rsi + 0x20], 0
0x140afc8ed mov      rdx, qword ptr [rbx + 0x10]
0x140afc8f1 cmp      qword ptr [rbx], rdx
0x140afc8f4 je       0x140afd5af
0x140afc8fa mov      rax, qword ptr [rbx + 8]
0x140afc8fe mov      byte ptr [rax + rdx], 0x7b
0x140afc902 inc      rdx
0x140afc905 mov      qword ptr [rbx + 0x10], rdx
0x140afc909 cmp      qword ptr [rbx], rdx
0x140afc90c je       0x140afd5d5
0x140afc912 lea      rdi, [rsi + 8]
0x140afc916 mov      rax, qword ptr [rbx + 8]
0x140afc91a mov      byte ptr [rax + rdx], 0xa
0x140afc91e inc      rdx
0x140afc921 mov      qword ptr [rbx + 0x10], rdx
0x140afc925 test     r13, r13
0x140afc928 je       0x140afc98e
0x140afc92a mov      rbx, qword ptr [rsi + 8]
0x140afc92e mov      r14, qword ptr [rsi + 0x10]
0x140afc932 mov      r15, qword ptr [rsi]
0x140afc935 mov      r12, qword ptr [r15 + 0x10]
0x140afc939 nop      dword ptr [rax]
0x140afc940 mov      rax, qword ptr [r15]
0x140afc943 sub      rax, r12
0x140afc946 cmp      rax, r14
0x140afc949 jb       0x140afc96b
0x140afc94b mov      rcx, qword ptr [r15 + 8]
0x140afc94f add      rcx, r12
0x140afc952 mov      rdx, rbx
0x140afc955 mov      r8, r14
0x140afc958 call     0x142923c00 ; memcpy
0x140afc95d add      r12, r14
0x140afc960 mov      qword ptr [r15 + 0x10], r12
0x140afc964 dec      r13
0x140afc967 jne      0x140afc940
0x140afc969 jmp      0x140afc98e ; 
0x140afc96b mov      qword ptr [rsp + 0x20], 1
0x140afc974 mov      r9d, 1
0x140afc97a mov      rcx, r15
0x140afc97d mov      rdx, r12
0x140afc980 mov      r8, r14
0x140afc983 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afc988 mov      r12, qword ptr [r15 + 0x10]
0x140afc98c jmp      0x140afc94b ; 
0x140afc98e lea      r8, [rip + 0x1fe8e2b] ; @0x142ae57c0 'EraserTreeTerrainEditingWaterGardenPhotomodeTimeOfDayPhotomodeFolderPhotomodeCameraPhotomodeFovPhoto'
0x140afc995 mov      r9d, 6
0x140afc99b mov      rcx, rsi
0x140afc99e mov      rdx, rdi
0x140afc9a1 call     0x141580910 ; serde_json::ser::format_escaped_str::h775c3d29342bbbda
0x140afc9a6 test     rax, rax
0x140afc9a9 jne      0x140afcff6
0x140afc9af mov      rbx, qword ptr [rsi]
0x140afc9b2 mov      rax, qword ptr [rbx]
0x140afc9b5 mov      rdx, qword ptr [rbx + 0x10]
0x140afc9b9 sub      rax, rdx
0x140afc9bc cmp      rax, 1
0x140afc9c0 jbe      0x140afd743
0x140afc9c6 mov      rax, qword ptr [rbx + 8]
0x140afc9ca mov      word ptr [rax + rdx], 0x203a
0x140afc9d0 add      rdx, 2
0x140afc9d4 mov      qword ptr [rbx + 0x10], rdx
0x140afc9d8 mov      eax, ebp
0x140afc9da lea      rcx, [rip + 0x1fe984b]
0x140afc9e1 movsxd   rax, dword ptr [rcx + rax*4]
0x140afc9e5 add      rax, rcx
0x140afc9e8 jmp      rax
0x140afc9ea lea      r8, [rip + 0x1fe9021] ; @0x142ae5a12 'AllWallBulldoze'
0x140afc9f1 mov      r9d, 3
0x140afc9f7 jmp      0x140afd4ae ; 
0x140afc9fc lea      rdx, [rsi + 8]
0x140afca00 lea      r8, [rip + 0x1fe8dd6] ; @0x142ae57dd 'GardenPhotomodeTimeOfDayPhotomodeFolderPhotomodeCameraPhotomodeFovPhotomodeFocusPhotomodeDofPhotomod'
0x140afca07 jmp      0x140afcfdf ; 
0x140afca0c mov      qword ptr [rsp + 0x20], 0
0x140afca15 lea      r8, [rip + 0x1fe9290] ; @0x142ae5cac 'LoadGameFencesCloneClutter'
0x140afca1c lea      rcx, [rsp + 0x40]
0x140afca21 mov      r9d, 8
0x140afca27 mov      rdx, rsi
0x140afca2a call     0x140af8490 ; 
0x140afca2f mov      rax, qword ptr [rsp + 0x40]
0x140afca34 movzx    edx, byte ptr [rsp + 0x48]
0x140afca39 cmp      dl, 3
0x140afca3c je       0x140afd57b
0x140afca42 mov      rcx, rax
0x140afca45 add      rsp, 0x68
0x140afca49 pop      rbx
0x140afca4a pop      rbp
0x140afca4b pop      rdi
0x140afca4c pop      rsi
0x140afca4d pop      r12
0x140afca4f pop      r13
0x140afca51 pop      r14
0x140afca53 pop      r15
0x140afca55 jmp      0x140af8260 ; _$LT$serde_json..ser..Compound$LT$W$C$F$GT$$u20$as$u20$serde..ser..SerializeStructVariant$GT$::end::h44cbd3ab698f69d5 (.llvm.16848830187404718073)
0x140afca5a lea      rdx, [rsi + 8]
0x140afca5e lea      r8, [rip + 0x1fe8d40] ; @0x142ae57a5 'PathDecoratorsCustomizationEraserTreeTerrainEditingWaterGardenPhotomodeTimeOfDayPhotomodeFolderPhoto'
0x140afca65 mov      r9d, 4
0x140afca6b jmp      0x140afcfe5 ; 
0x140afca70 lea      rdx, [rsi + 8]
0x140afca74 lea      r8, [rip + 0x1fe923f] ; @0x142ae5cba 'CloneClutter'
0x140afca7b mov      r9d, 0xc
0x140afca81 jmp      0x140afcfe5 ; 
0x140afca86 mov      rdi, qword ptr [rcx + 8]
0x140afca8a mov      r14, qword ptr [rsi]
0x140afca8d mov      rbp, qword ptr [rsi + 0x18]
0x140afca91 inc      rbp
0x140afca94 mov      qword ptr [rsi + 0x18], rbp
0x140afca98 mov      byte ptr [rsi + 0x20], 0
0x140afca9c mov      rdx, qword ptr [r14 + 0x10]
0x140afcaa0 cmp      qword ptr [r14], rdx
0x140afcaa3 je       0x140afd5fb
0x140afcaa9 mov      rax, qword ptr [r14 + 8]
0x140afcaad mov      byte ptr [rax + rdx], 0x7b
0x140afcab1 inc      rdx
0x140afcab4 mov      qword ptr [r14 + 0x10], rdx
0x140afcab8 cmp      qword ptr [r14], rdx
0x140afcabb je       0x140afd621
0x140afcac1 lea      rbx, [rsi + 8]
0x140afcac5 mov      rax, qword ptr [r14 + 8]
0x140afcac9 mov      byte ptr [rax + rdx], 0xa
0x140afcacd inc      rdx
0x140afcad0 mov      qword ptr [r14 + 0x10], rdx
0x140afcad4 test     rbp, rbp
0x140afcad7 je       0x140afcb42
0x140afcad9 mov      r14, qword ptr [rsi + 8]
0x140afcadd mov      r15, qword ptr [rsi + 0x10]
0x140afcae1 mov      r12, qword ptr [rsi]
0x140afcae4 mov      r13, qword ptr [r12 + 0x10]
0x140afcae9 nop      dword ptr [rax]
0x140afcaf0 mov      rax, qword ptr [r12]
0x140afcaf4 sub      rax, r13
0x140afcaf7 cmp      rax, r15
0x140afcafa jb       0x140afcb1e
0x140afcafc mov      rcx, qword ptr [r12 + 8]
0x140afcb01 add      rcx, r13
0x140afcb04 mov      rdx, r14
0x140afcb07 mov      r8, r15
0x140afcb0a call     0x142923c00 ; memcpy
0x140afcb0f add      r13, r15
0x140afcb12 mov      qword ptr [r12 + 0x10], r13
0x140afcb17 dec      rbp
0x140afcb1a jne      0x140afcaf0
0x140afcb1c jmp      0x140afcb42 ; 
0x140afcb1e mov      qword ptr [rsp + 0x20], 1
0x140afcb27 mov      r9d, 1
0x140afcb2d mov      rcx, r12
0x140afcb30 mov      rdx, r13
0x140afcb33 mov      r8, r15
0x140afcb36 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afcb3b mov      r13, qword ptr [r12 + 0x10]
0x140afcb40 jmp      0x140afcafc ; 
0x140afcb42 lea      r8, [rip + 0x1fe90ff] ; @0x142ae5c48 'Cobblestone'
0x140afcb49 mov      r9d, 0xb
0x140afcb4f mov      rcx, rsi
0x140afcb52 mov      rdx, rbx
0x140afcb55 call     0x141580910 ; serde_json::ser::format_escaped_str::h775c3d29342bbbda
0x140afcb5a test     rax, rax
0x140afcb5d jne      0x140afcff6
0x140afcb63 mov      rbx, qword ptr [rsi]
0x140afcb66 mov      rax, qword ptr [rbx]
0x140afcb69 mov      rdx, qword ptr [rbx + 0x10]
0x140afcb6d sub      rax, rdx
0x140afcb70 cmp      rax, 1
0x140afcb74 jbe      0x140afd769
0x140afcb7a mov      rax, qword ptr [rbx + 8]
0x140afcb7e mov      word ptr [rax + rdx], 0x203a
0x140afcb84 add      rdx, 2
0x140afcb88 mov      qword ptr [rbx + 0x10], rdx
0x140afcb8c mov      eax, 0x14
0x140afcb91 lea      rcx, [rip + 0x1fe89a0] ; @0x142ae5538 '0001020304050607080910111213141516171819202122232425262728293031323334353637383940414243444546474849'
0x140afcb98 cmp      rdi, 0x2710
0x140afcb9f jb       0x140afd00e
0x140afcba5 mov      r9d, 0x14
0x140afcbab movabs   r8, 0x346dc5d63886594b
0x140afcbb5 nop      word ptr cs:[rax + rax]
0x140afcbc0 mov      rax, rdi
0x140afcbc3 mul      r8
0x140afcbc6 shr      rdx, 0xb
0x140afcbca imul     eax, edx, 0x2710
0x140afcbd0 mov      r10d, edi
0x140afcbd3 sub      r10d, eax
0x140afcbd6 imul     r11d, r10d, 0x147b
0x140afcbdd shr      r11d, 0x13
0x140afcbe1 imul     eax, r11d, 0x64
0x140afcbe5 sub      r10d, eax
0x140afcbe8 movzx    r10d, r10w
0x140afcbec lea      rax, [r9 - 4]
0x140afcbf0 movzx    r11d, word ptr [rcx + r11*2]
0x140afcbf5 mov      word ptr [rsp + r9 + 0x3c], r11w
0x140afcbfb movzx    r10d, word ptr [rcx + r10*2]
0x140afcc00 mov      word ptr [rsp + r9 + 0x3e], r10w
0x140afcc06 mov      r9, rax
0x140afcc09 cmp      rdi, 0x5f5e0ff
0x140afcc10 mov      rdi, rdx
0x140afcc13 ja       0x140afcbc0
0x140afcc15 jmp      0x140afd011 ; 
0x140afcc1a lea      rdx, [rsi + 8]
0x140afcc1e lea      r8, [rip + 0x1fe907c] ; @0x142ae5ca1 'ColorReplayLoadGameFencesCloneClutter'
0x140afcc25 mov      r9d, 5
0x140afcc2b jmp      0x140afcfe5 ; 
0x140afcc30 movzx    ebp, byte ptr [rcx + 4]
0x140afcc34 mov      rbx, qword ptr [rsi]
0x140afcc37 mov      r13, qword ptr [rsi + 0x18]
0x140afcc3b inc      r13
0x140afcc3e mov      qword ptr [rsi + 0x18], r13
0x140afcc42 mov      byte ptr [rsi + 0x20], 0
0x140afcc46 mov      rdx, qword ptr [rbx + 0x10]
0x140afcc4a cmp      qword ptr [rbx], rdx
0x140afcc4d je       0x140afd69f
0x140afcc53 mov      rax, qword ptr [rbx + 8]
0x140afcc57 mov      byte ptr [rax + rdx], 0x7b
0x140afcc5b inc      rdx
0x140afcc5e mov      qword ptr [rbx + 0x10], rdx
0x140afcc62 cmp      qword ptr [rbx], rdx
0x140afcc65 je       0x140afd6c5
0x140afcc6b lea      rdi, [rsi + 8]
0x140afcc6f mov      rax, qword ptr [rbx + 8]
0x140afcc73 mov      byte ptr [rax + rdx], 0xa
0x140afcc77 inc      rdx
0x140afcc7a mov      qword ptr [rbx + 0x10], rdx
0x140afcc7e test     r13, r13
0x140afcc81 je       0x140afcd78
0x140afcc87 mov      rbx, qword ptr [rsi + 8]
0x140afcc8b mov      r14, qword ptr [rsi + 0x10]
0x140afcc8f mov      r15, qword ptr [rsi]
0x140afcc92 mov      r12, qword ptr [r15 + 0x10]
0x140afcc96 nop      word ptr cs:[rax + rax]
0x140afcca0 mov      rax, qword ptr [r15]
0x140afcca3 sub      rax, r12
0x140afcca6 cmp      rax, r14
0x140afcca9 jb       0x140afccce
0x140afccab mov      rcx, qword ptr [r15 + 8]
0x140afccaf add      rcx, r12
0x140afccb2 mov      rdx, rbx
0x140afccb5 mov      r8, r14
0x140afccb8 call     0x142923c00 ; memcpy
0x140afccbd add      r12, r14
0x140afccc0 mov      qword ptr [r15 + 0x10], r12
0x140afccc4 dec      r13
0x140afccc7 jne      0x140afcca0
0x140afccc9 jmp      0x140afcd78 ; 
0x140afccce mov      qword ptr [rsp + 0x20], 1
0x140afccd7 mov      r9d, 1
0x140afccdd mov      rcx, r15
0x140afcce0 mov      rdx, r12
0x140afcce3 mov      r8, r14
0x140afcce6 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afcceb mov      r12, qword ptr [r15 + 0x10]
0x140afccef jmp      0x140afccab ; 
0x140afccf1 lea      r8, [rip + 0x1fe8ad9] ; @0x142ae57d1 'EditingWaterGardenPhotomodeTimeOfDayPhotomodeFolderPhotomodeCameraPhotomodeFovPhotomodeFocusPhotomod'
0x140afccf8 mov      r9d, 7
0x140afccfe mov      rcx, rsi
0x140afcd01 mov      rdx, rdi
0x140afcd04 call     0x141580910 ; serde_json::ser::format_escaped_str::h775c3d29342bbbda
0x140afcd09 test     rax, rax
0x140afcd0c jne      0x140afcff6
0x140afcd12 mov      rbx, qword ptr [rsi]
0x140afcd15 mov      rax, qword ptr [rbx]
0x140afcd18 mov      rdx, qword ptr [rbx + 0x10]
0x140afcd1c sub      rax, rdx
0x140afcd1f cmp      rax, 1
0x140afcd23 jbe      0x140afd7e1
0x140afcd29 mov      r8, qword ptr [rsp + 0x28]
0x140afcd2e mov      rax, qword ptr [rbx + 8]
0x140afcd32 mov      word ptr [rax + rdx], 0x203a
0x140afcd38 add      rdx, 2
0x140afcd3c mov      qword ptr [rbx + 0x10], rdx
0x140afcd40 mov      eax, ebp
0x140afcd42 lea      rcx, [rip + 0x1fe94ff]
0x140afcd49 movsxd   rax, dword ptr [rcx + rax*4]
0x140afcd4d add      rax, rcx
0x140afcd50 jmp      rax
0x140afcd52 lea      r8, [rip + 0x1fe8d87] ; @0x142ae5ae0 'None'
0x140afcd59 mov      r9d, 4
0x140afcd5f mov      rcx, rsi
0x140afcd62 mov      rdx, rdi
0x140afcd65 call     0x141580910 ; serde_json::ser::format_escaped_str::h775c3d29342bbbda
0x140afcd6a test     rax, rax
0x140afcd6d je       0x140afd3fa
0x140afcd73 jmp      0x140afcff6 ; 
0x140afcd78 lea      r8, [rip + 0x1fe8c96] ; @0x142ae5a15 'WallBulldoze'
0x140afcd7f mov      r9d, 4
0x140afcd85 mov      rcx, rsi
0x140afcd88 mov      rdx, rdi
0x140afcd8b call     0x141580910 ; serde_json::ser::format_escaped_str::h775c3d29342bbbda
0x140afcd90 test     rax, rax
0x140afcd93 jne      0x140afcff6
0x140afcd99 mov      rbx, qword ptr [rsi]
0x140afcd9c mov      rax, qword ptr [rbx]
0x140afcd9f mov      rdx, qword ptr [rbx + 0x10]
0x140afcda3 sub      rax, rdx
0x140afcda6 cmp      rax, 1
0x140afcdaa jbe      0x140afd807
0x140afcdb0 mov      rax, qword ptr [rbx + 8]
0x140afcdb4 mov      word ptr [rax + rdx], 0x203a
0x140afcdba add      rdx, 2
0x140afcdbe mov      qword ptr [rbx + 0x10], rdx
0x140afcdc2 test     ebp, ebp
0x140afcdc4 je       0x140afd141
0x140afcdca cmp      ebp, 1
0x140afcdcd jne      0x140afd150
0x140afcdd3 lea      r8, [rip + 0x1fe8cc6] ; @0x142ae5aa0 'CircleRectangle'
0x140afcdda mov      r9d, 6
0x140afcde0 jmp      0x140afd15d ; 
0x140afcde5 lea      rdx, [rsi + 8]
0x140afcde9 lea      r8, [rip + 0x1fe89da] ; @0x142ae57ca 'TerrainEditingWaterGardenPhotomodeTimeOfDayPhotomodeFolderPhotomodeCameraPhotomodeFovPhotomodeFocusP'
0x140afcdf0 mov      r9d, 7
0x140afcdf6 jmp      0x140afcfe5 ; 
0x140afcdfb lea      rdx, [rsi + 8]
0x140afcdff lea      r8, [rip + 0x1fe8eae] ; @0x142ae5cb4 'FencesCloneClutter'
0x140afce06 jmp      0x140afcfdf ; 
0x140afce0b mov      rbx, qword ptr [rsi]
0x140afce0e mov      r13, qword ptr [rsi + 0x18]
0x140afce12 inc      r13
0x140afce15 mov      qword ptr [rsi + 0x18], r13
0x140afce19 mov      byte ptr [rsi + 0x20], 0
0x140afce1d mov      rdx, qword ptr [rbx + 0x10]
0x140afce21 cmp      qword ptr [rbx], rdx
0x140afce24 je       0x140afd6eb
0x140afce2a mov      rax, qword ptr [rbx + 8]
0x140afce2e mov      byte ptr [rax + rdx], 0x7b
0x140afce32 inc      rdx
0x140afce35 mov      qword ptr [rbx + 0x10], rdx
0x140afce39 cmp      qword ptr [rbx], rdx
0x140afce3c je       0x140afd717
0x140afce42 mov      rbp, rcx
0x140afce45 lea      rdi, [rsi + 8]
0x140afce49 mov      rax, qword ptr [rbx + 8]
0x140afce4d mov      byte ptr [rax + rdx], 0xa
0x140afce51 inc      rdx
0x140afce54 mov      qword ptr [rbx + 0x10], rdx
0x140afce58 test     r13, r13
0x140afce5b je       0x140afcebe
0x140afce5d mov      rbx, qword ptr [rsi + 8]
0x140afce61 mov      r14, qword ptr [rsi + 0x10]
0x140afce65 mov      r15, qword ptr [rsi]
0x140afce68 mov      r12, qword ptr [r15 + 0x10]
0x140afce6c nop      dword ptr [rax]
0x140afce70 mov      rax, qword ptr [r15]
0x140afce73 sub      rax, r12
0x140afce76 cmp      rax, r14
0x140afce79 jb       0x140afce9b
0x140afce7b mov      rcx, qword ptr [r15 + 8]
0x140afce7f add      rcx, r12
0x140afce82 mov      rdx, rbx
0x140afce85 mov      r8, r14
0x140afce88 call     0x142923c00 ; memcpy
0x140afce8d add      r12, r14
0x140afce90 mov      qword ptr [r15 + 0x10], r12
0x140afce94 dec      r13
0x140afce97 jne      0x140afce70
0x140afce99 jmp      0x140afcebe ; 
0x140afce9b mov      qword ptr [rsp + 0x20], 1
0x140afcea4 mov      r9d, 1
0x140afceaa mov      rcx, r15
0x140afcead mov      rdx, r12
0x140afceb0 mov      r8, r14
0x140afceb3 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afceb8 mov      r12, qword ptr [r15 + 0x10]
0x140afcebc jmp      0x140afce7b ; 
0x140afcebe lea      r8, [rip + 0x1fe8dd3] ; @0x142ae5c98 'DecoratorColorReplayLoadGameFencesCloneClutter'
0x140afcec5 mov      r9d, 9
0x140afcecb mov      rcx, rsi
0x140afcece mov      rdx, rdi
0x140afced1 call     0x141580910 ; serde_json::ser::format_escaped_str::h775c3d29342bbbda
0x140afced6 test     rax, rax
0x140afced9 jne      0x140afcff6
0x140afcedf mov      rcx, rbp
0x140afcee2 add      rcx, 4
0x140afcee6 mov      rdi, qword ptr [rsi]
0x140afcee9 mov      rax, qword ptr [rdi]
0x140afceec mov      rdx, qword ptr [rdi + 0x10]
0x140afcef0 sub      rax, rdx
0x140afcef3 cmp      rax, 1
0x140afcef7 jbe      0x140afd82d
0x140afcefd mov      rax, qword ptr [rdi + 8]
0x140afcf01 mov      word ptr [rax + rdx], 0x203a
0x140afcf07 add      rdx, 2
0x140afcf0b mov      qword ptr [rdi + 0x10], rdx
0x140afcf0f mov      rdx, rsi
0x140afcf12 call     0x140a478f0 ; country_core::resources::walls::decorator::_::_$LT$impl$u20$serde..ser..Serialize$u20$for$u20$country_core..resources..walls..decorator..DecoratorType$GT$::serialize::hd7a22c6bfa763284
0x140afcf17 test     rax, rax
0x140afcf1a jne      0x140afd57b
0x140afcf20 mov      byte ptr [rsi + 0x20], 1
0x140afcf24 mov      rdi, qword ptr [rsi]
0x140afcf27 mov      r12, qword ptr [rsi + 0x18]
0x140afcf2b dec      r12
0x140afcf2e mov      qword ptr [rsi + 0x18], r12
0x140afcf32 mov      rdx, qword ptr [rdi + 0x10]
0x140afcf36 cmp      qword ptr [rdi], rdx
0x140afcf39 je       0x140afd8a5
0x140afcf3f mov      rax, qword ptr [rdi + 8]
0x140afcf43 mov      byte ptr [rax + rdx], 0xa
0x140afcf47 inc      rdx
0x140afcf4a mov      qword ptr [rdi + 0x10], rdx
0x140afcf4e test     r12, r12
0x140afcf51 je       0x140afd441
0x140afcf57 mov      rdi, qword ptr [rsi + 8]
0x140afcf5b mov      rbx, qword ptr [rsi + 0x10]
0x140afcf5f mov      r14, qword ptr [rsi]
0x140afcf62 mov      r15, qword ptr [r14 + 0x10]
0x140afcf66 nop      word ptr cs:[rax + rax]
0x140afcf70 mov      rax, qword ptr [r14]
0x140afcf73 sub      rax, r15
0x140afcf76 cmp      rax, rbx
0x140afcf79 jb       0x140afcf9e
0x140afcf7b mov      rcx, qword ptr [r14 + 8]
0x140afcf7f add      rcx, r15
0x140afcf82 mov      rdx, rdi
0x140afcf85 mov      r8, rbx
0x140afcf88 call     0x142923c00 ; memcpy
0x140afcf8d add      r15, rbx
0x140afcf90 mov      qword ptr [r14 + 0x10], r15
0x140afcf94 dec      r12
0x140afcf97 jne      0x140afcf70
0x140afcf99 jmp      0x140afd441 ; 
0x140afcf9e mov      qword ptr [rsp + 0x20], 1
0x140afcfa7 mov      r9d, 1
0x140afcfad mov      rcx, r14
0x140afcfb0 mov      rdx, r15
0x140afcfb3 mov      r8, rbx
0x140afcfb6 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afcfbb mov      r15, qword ptr [r14 + 0x10]
0x140afcfbf jmp      0x140afcf7b ; 
0x140afcfc1 lea      rdx, [rsi + 8]
0x140afcfc5 lea      r8, [rip + 0x1fe880c] ; @0x142ae57d8 'WaterGardenPhotomodeTimeOfDayPhotomodeFolderPhotomodeCameraPhotomodeFovPhotomodeFocusPhotomodeDofPho'
0x140afcfcc mov      r9d, 5
0x140afcfd2 jmp      0x140afcfe5 ; 
0x140afcfd4 lea      rdx, [rsi + 8]
0x140afcfd8 lea      r8, [rip + 0x1fe8cc7] ; @0x142ae5ca6 'ReplayLoadGameFencesCloneClutter'
0x140afcfdf mov      r9d, 6
0x140afcfe5 mov      rcx, rsi
0x140afcfe8 call     0x141580910 ; serde_json::ser::format_escaped_str::h775c3d29342bbbda
0x140afcfed test     rax, rax
0x140afcff0 je       0x140afd579
0x140afcff6 mov      rcx, rax
0x140afcff9 add      rsp, 0x68
0x140afcffd pop      rbx
0x140afcffe pop      rbp
0x140afcfff pop      rdi
0x140afd000 pop      rsi
0x140afd001 pop      r12
0x140afd003 pop      r13
0x140afd005 pop      r14
0x140afd007 pop      r15
0x140afd009 jmp      0x1428c98a0 ; serde_json::error::Error::io::hfcf27584d0d6aa9f
0x140afd00e mov      rdx, rdi
0x140afd011 cmp      rdx, 0x63
0x140afd015 jbe      0x140afd044
0x140afd017 movzx    r8d, dx
0x140afd01b shr      r8d, 2
0x140afd01f imul     r8d, r8d, 0x147b
0x140afd026 shr      r8d, 0x11
0x140afd02a imul     r9d, r8d, 0x64
0x140afd02e sub      edx, r9d
0x140afd031 movzx    edx, dx
0x140afd034 movzx    edx, word ptr [rcx + rdx*2]
0x140afd038 mov      word ptr [rsp + rax + 0x3e], dx
0x140afd03d add      rax, -2
0x140afd041 mov      rdx, r8
0x140afd044 mov      rdi, qword ptr [rsi]
0x140afd047 cmp      rdx, 0xa
0x140afd04b jae      0x140afd059
0x140afd04d or       dl, 0x30
0x140afd050 mov      byte ptr [rsp + rax + 0x3f], dl
0x140afd054 dec      rax
0x140afd057 jmp      0x140afd066 ; 
0x140afd059 movzx    ecx, word ptr [rcx + rdx*2]
0x140afd05d mov      word ptr [rsp + rax + 0x3e], cx
0x140afd062 add      rax, -2
0x140afd066 mov      ebx, 0x14
0x140afd06b sub      rbx, rax
0x140afd06e lea      rdx, [rsp + rax]
0x140afd072 add      rdx, 0x40
0x140afd076 mov      rax, qword ptr [rdi]
0x140afd079 mov      r14, qword ptr [rdi + 0x10]
0x140afd07d sub      rax, r14
0x140afd080 cmp      rax, rbx
0x140afd083 jb       0x140afd78f
0x140afd089 mov      rcx, qword ptr [rdi + 8]
0x140afd08d add      rcx, r14
0x140afd090 mov      r8, rbx
0x140afd093 call     0x142923c00 ; memcpy
0x140afd098 add      r14, rbx
0x140afd09b mov      qword ptr [rdi + 0x10], r14
0x140afd09f mov      byte ptr [rsi + 0x20], 1
0x140afd0a3 mov      rdi, qword ptr [rsi]
0x140afd0a6 mov      r12, qword ptr [rsi + 0x18]
0x140afd0aa dec      r12
0x140afd0ad mov      qword ptr [rsi + 0x18], r12
0x140afd0b1 mov      rdx, qword ptr [rdi + 0x10]
0x140afd0b5 cmp      qword ptr [rdi], rdx
0x140afd0b8 je       0x140afd7bb
0x140afd0be mov      rax, qword ptr [rdi + 8]
0x140afd0c2 mov      byte ptr [rax + rdx], 0xa
0x140afd0c6 inc      rdx
0x140afd0c9 mov      qword ptr [rdi + 0x10], rdx
0x140afd0cd test     r12, r12
0x140afd0d0 je       0x140afd55e
0x140afd0d6 mov      rdi, qword ptr [rsi + 8]
0x140afd0da mov      rbx, qword ptr [rsi + 0x10]
0x140afd0de mov      r14, qword ptr [rsi]
0x140afd0e1 mov      r15, qword ptr [r14 + 0x10]
0x140afd0e5 nop      word ptr cs:[rax + rax]
0x140afd0f0 mov      rax, qword ptr [r14]
0x140afd0f3 sub      rax, r15
0x140afd0f6 cmp      rax, rbx
0x140afd0f9 jb       0x140afd11e
0x140afd0fb mov      rcx, qword ptr [r14 + 8]
0x140afd0ff add      rcx, r15
0x140afd102 mov      rdx, rdi
0x140afd105 mov      r8, rbx
0x140afd108 call     0x142923c00 ; memcpy
0x140afd10d add      r15, rbx
0x140afd110 mov      qword ptr [r14 + 0x10], r15
0x140afd114 dec      r12
0x140afd117 jne      0x140afd0f0
0x140afd119 jmp      0x140afd55e ; 
0x140afd11e mov      qword ptr [rsp + 0x20], 1
0x140afd127 mov      r9d, 1
0x140afd12d mov      rcx, r14
0x140afd130 mov      rdx, r15
0x140afd133 mov      r8, rbx
0x140afd136 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afd13b mov      r15, qword ptr [r14 + 0x10]
0x140afd13f jmp      0x140afd0fb ; 
0x140afd141 lea      r8, [rip + 0x1fe8950] ; @0x142ae5a98 'FreeformCircleRectangle'
0x140afd148 mov      r9d, 8
0x140afd14e jmp      0x140afd15d ; 
0x140afd150 lea      r8, [rip + 0x1fe894f] ; @0x142ae5aa6 'Rectangle'
0x140afd157 mov      r9d, 9
0x140afd15d mov      rcx, rsi
0x140afd160 mov      rdx, rdi
0x140afd163 call     0x141580910 ; serde_json::ser::format_escaped_str::h775c3d29342bbbda
0x140afd168 test     rax, rax
0x140afd16b jne      0x140afcff6
0x140afd171 mov      byte ptr [rsi + 0x20], 1
0x140afd175 mov      rdi, qword ptr [rsi]
0x140afd178 mov      r12, qword ptr [rsi + 0x18]
0x140afd17c dec      r12
0x140afd17f mov      qword ptr [rsi + 0x18], r12
0x140afd183 mov      rdx, qword ptr [rdi + 0x10]
0x140afd187 cmp      qword ptr [rdi], rdx
0x140afd18a je       0x140afd87f
0x140afd190 mov      rax, qword ptr [rdi + 8]
0x140afd194 mov      byte ptr [rax + rdx], 0xa
0x140afd198 inc      rdx
0x140afd19b mov      qword ptr [rdi + 0x10], rdx
0x140afd19f test     r12, r12
0x140afd1a2 je       0x140afd55e
0x140afd1a8 mov      rdi, qword ptr [rsi + 8]
0x140afd1ac mov      rbx, qword ptr [rsi + 0x10]
0x140afd1b0 mov      r14, qword ptr [rsi]
0x140afd1b3 mov      r15, qword ptr [r14 + 0x10]
0x140afd1b7 nop      word ptr [rax + rax]
0x140afd1c0 mov      rax, qword ptr [r14]
0x140afd1c3 sub      rax, r15
0x140afd1c6 cmp      rax, rbx
0x140afd1c9 jb       0x140afd1ee
0x140afd1cb mov      rcx, qword ptr [r14 + 8]
0x140afd1cf add      rcx, r15
0x140afd1d2 mov      rdx, rdi
0x140afd1d5 mov      r8, rbx
0x140afd1d8 call     0x142923c00 ; memcpy
0x140afd1dd add      r15, rbx
0x140afd1e0 mov      qword ptr [r14 + 0x10], r15
0x140afd1e4 dec      r12
0x140afd1e7 jne      0x140afd1c0
0x140afd1e9 jmp      0x140afd55e ; 
0x140afd1ee mov      qword ptr [rsp + 0x20], 1
0x140afd1f7 mov      r9d, 1
0x140afd1fd mov      rcx, r14
0x140afd200 mov      rdx, r15
0x140afd203 mov      r8, rbx
0x140afd206 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afd20b mov      r15, qword ptr [r14 + 0x10]
0x140afd20f jmp      0x140afd1cb ; 
0x140afd211 mov      qword ptr [rsp + 0x20], 1
0x140afd21a lea      r8, [rip + 0x1fe8926] ; @0x142ae5b47 'StairPoint'
0x140afd221 lea      rcx, [rsp + 0x40]
0x140afd226 mov      r9d, 0xa
0x140afd22c jmp      0x140afd259 ; 
0x140afd22e add      r8, 8
0x140afd232 lea      rdx, [rip + 0x1fe88cf] ; @0x142ae5b08 'Roof'
0x140afd239 jmp      0x140afd2c2 ; 
0x140afd23e mov      qword ptr [rsp + 0x20], 1
0x140afd247 lea      r8, [rip + 0x1fe88e2] ; @0x142ae5b30 'DecoClutterdecorator_idStairPoint'
0x140afd24e lea      rcx, [rsp + 0x40]
0x140afd253 mov      r9d, 0xb
0x140afd259 mov      rdx, rsi
0x140afd25c call     0x140af8490 ; 
0x140afd261 mov      rax, qword ptr [rsp + 0x40]
0x140afd266 movzx    ecx, byte ptr [rsp + 0x48]
0x140afd26b cmp      cl, 3
0x140afd26e je       0x140afd57b
0x140afd274 mov      r9, qword ptr [rsp + 0x28]
0x140afd279 add      r9, 4
0x140afd27d mov      qword ptr [rsp + 0x30], rax
0x140afd282 mov      byte ptr [rsp + 0x38], cl
0x140afd286 lea      rdx, [rip + 0x1fe88ae] ; @0x142ae5b3b 'decorator_idStairPoint'
0x140afd28d lea      rcx, [rsp + 0x30]
0x140afd292 mov      r8d, 0xc
0x140afd298 call     0x141584710 ; serde::ser::SerializeMap::serialize_entry::h33951798e80c1197
0x140afd29d test     rax, rax
0x140afd2a0 jne      0x140afd57b
0x140afd2a6 mov      rcx, qword ptr [rsp + 0x30]
0x140afd2ab movzx    edx, byte ptr [rsp + 0x38]
0x140afd2b0 call     0x140af8260 ; _$LT$serde_json..ser..Compound$LT$W$C$F$GT$$u20$as$u20$serde..ser..SerializeStructVariant$GT$::end::h44cbd3ab698f69d5 (.llvm.16848830187404718073)
0x140afd2b5 jmp      0x140afd2ca ; 
0x140afd2b7 add      r8, 8
0x140afd2bb lea      rdx, [rip + 0x1fe8753] ; @0x142ae5a15 'WallBulldoze'
0x140afd2c2 mov      rcx, rsi
0x140afd2c5 call     0x140af8700 ; 
0x140afd2ca test     rax, rax
0x140afd2cd jne      0x140afd57b
0x140afd2d3 jmp      0x140afd3fa ; 
0x140afd2d8 mov      rbx, qword ptr [rsi]
0x140afd2db mov      r14, qword ptr [rsi + 0x18]
0x140afd2df inc      r14
0x140afd2e2 mov      qword ptr [rsi + 0x18], r14
0x140afd2e6 mov      byte ptr [rsi + 0x20], 0
0x140afd2ea lea      rdx, [rip + 0x1fe8311] ; @0x142ae5602 '{: crates/country-core/src/resources/toolbar.rs'
0x140afd2f1 lea      r15, [rip + 0x1fe8228]
0x140afd2f8 mov      r8d, 1
0x140afd2fe mov      rcx, rbx
0x140afd301 mov      r9, r15
0x140afd304 call     0x1412f4520 ; alloc::vec::Vec$LT$T$C$A$GT$::extend_from_slice::h70c862da947354be
0x140afd309 lea      rdx, [rip + 0x1fe82f0]
0x140afd310 mov      r8d, 1
0x140afd316 mov      rcx, rbx
0x140afd319 mov      r9, r15
0x140afd31c call     0x1412f4520 ; alloc::vec::Vec$LT$T$C$A$GT$::extend_from_slice::h70c862da947354be
0x140afd321 mov      r8, qword ptr [rsi + 8]
0x140afd325 mov      r9, qword ptr [rsi + 0x10]
0x140afd329 mov      rcx, rsi
0x140afd32c mov      rdx, r14
0x140afd32f call     0x141581130 ; serde_json::ser::indent::ha2990fb709079cf8
0x140afd334 test     rax, rax
0x140afd337 jne      0x140afcff6
0x140afd33d lea      r8, [rip + 0x1fe8486] ; @0x142ae57ca 'TerrainEditingWaterGardenPhotomodeTimeOfDayPhotomodeFolderPhotomodeCameraPhotomodeFovPhotomodeFocusP'
0x140afd344 mov      r9d, 7
0x140afd34a mov      rcx, rsi
0x140afd34d mov      rdx, rdi
0x140afd350 call     0x141580910 ; serde_json::ser::format_escaped_str::h775c3d29342bbbda
0x140afd355 test     rax, rax
0x140afd358 jne      0x140afcff6
0x140afd35e mov      rdi, qword ptr [rsp + 0x28]
0x140afd363 add      rdi, 4
0x140afd367 mov      rcx, qword ptr [rsi]
0x140afd36a lea      rdx, [rip + 0x1fe8292] ; @0x142ae5603 ': crates/country-core/src/resources/toolbar.rs'
0x140afd371 lea      r9, [rip + 0x1fe81a8]
0x140afd378 mov      r8d, 2
0x140afd37e call     0x1412f4520 ; alloc::vec::Vec$LT$T$C$A$GT$::extend_from_slice::h70c862da947354be
0x140afd383 mov      rcx, rdi
0x140afd386 mov      rdx, rsi
0x140afd389 call     0x1408e5bc0 ; country_core::resources::color_id::_::_$LT$impl$u20$serde..ser..Serialize$u20$for$u20$country_core..resources..color_id..ColorId$GT$::serialize::h0e1c7843949cbbab / country_core::resources::terrain_editing::_::_$LT$impl$u20$serde..ser..Serialize$u20$for$u20$country_core..resources..terrain_editing..TerrainStrokeId$GT$::serialize::h1d8b62e6ff9e4989
0x140afd38e test     rax, rax
0x140afd391 jne      0x140afd57b
0x140afd397 mov      byte ptr [rsi + 0x20], 1
0x140afd39b mov      rcx, qword ptr [rsi]
0x140afd39e mov      rdi, qword ptr [rsi + 0x18]
0x140afd3a2 dec      rdi
0x140afd3a5 mov      qword ptr [rsi + 0x18], rdi
0x140afd3a9 lea      rdx, [rip + 0x1fe8250]
0x140afd3b0 lea      r9, [rip + 0x1fe8169]
0x140afd3b7 mov      r8d, 1
0x140afd3bd call     0x1412f4520 ; alloc::vec::Vec$LT$T$C$A$GT$::extend_from_slice::h70c862da947354be
0x140afd3c2 mov      r8, qword ptr [rsi + 8]
0x140afd3c6 mov      r9, qword ptr [rsi + 0x10]
0x140afd3ca mov      rcx, rsi
0x140afd3cd mov      rdx, rdi
0x140afd3d0 call     0x141581130 ; serde_json::ser::indent::ha2990fb709079cf8
0x140afd3d5 test     rax, rax
0x140afd3d8 jne      0x140afcff6
0x140afd3de mov      rcx, qword ptr [rsi]
0x140afd3e1 lea      rdx, [rip + 0x1fe8219] ; @0x142ae5601 '}{: crates/country-core/src/resources/toolbar.rs'
0x140afd3e8 lea      r9, [rip + 0x1fe8131]
0x140afd3ef mov      r8d, 1
0x140afd3f5 call     0x1412f4520 ; alloc::vec::Vec$LT$T$C$A$GT$::extend_from_slice::h70c862da947354be
0x140afd3fa mov      byte ptr [rsi + 0x20], 1
0x140afd3fe mov      rcx, qword ptr [rsi]
0x140afd401 mov      rdi, qword ptr [rsi + 0x18]
0x140afd405 dec      rdi
0x140afd408 mov      qword ptr [rsi + 0x18], rdi
0x140afd40c lea      rdx, [rip + 0x1fe81ed]
0x140afd413 lea      r9, [rip + 0x1fe8106]
0x140afd41a mov      r8d, 1
0x140afd420 call     0x1412f4520 ; alloc::vec::Vec$LT$T$C$A$GT$::extend_from_slice::h70c862da947354be
0x140afd425 mov      r8, qword ptr [rsi + 8]
0x140afd429 mov      r9, qword ptr [rsi + 0x10]
0x140afd42d mov      rcx, rsi
0x140afd430 mov      rdx, rdi
0x140afd433 call     0x141581130 ; serde_json::ser::indent::ha2990fb709079cf8
0x140afd438 test     rax, rax
0x140afd43b jne      0x140afcff6
0x140afd441 mov      rcx, qword ptr [rsi]
0x140afd444 lea      rdx, [rip + 0x1fe81b6] ; @0x142ae5601 '}{: crates/country-core/src/resources/toolbar.rs'
0x140afd44b lea      r9, [rip + 0x1fe80ce]
0x140afd452 mov      r8d, 1
0x140afd458 call     0x1412f4520 ; alloc::vec::Vec$LT$T$C$A$GT$::extend_from_slice::h70c862da947354be
0x140afd45d jmp      0x140afd579 ; 
0x140afd462 lea      r8, [rip + 0x1fe836f] ; @0x142ae57d8 'WaterGardenPhotomodeTimeOfDayPhotomodeFolderPhotomodeCameraPhotomodeFovPhotomodeFocusPhotomodeDofPho'
0x140afd469 mov      r9d, 5
0x140afd46f jmp      0x140afd4ae ; 
0x140afd471 lea      r8, [rip + 0x1fe832d] ; @0x142ae57a5 'PathDecoratorsCustomizationEraserTreeTerrainEditingWaterGardenPhotomodeTimeOfDayPhotomodeFolderPhoto'
0x140afd478 jmp      0x140afd4a8 ; 
0x140afd47a lea      r8, [rip + 0x1fe8598] ; @0x142ae5a19 'Bulldoze'
0x140afd481 mov      r9d, 8
0x140afd487 jmp      0x140afd4ae ; 
0x140afd489 lea      r8, [rip + 0x1fe8585] ; @0x142ae5a15 'WallBulldoze'
0x140afd490 jmp      0x140afd4a8 ; 
0x140afd492 lea      r8, [rip + 0x1fe8344] ; @0x142ae57dd 'GardenPhotomodeTimeOfDayPhotomodeFolderPhotomodeCameraPhotomodeFovPhotomodeFocusPhotomodeDofPhotomod'
0x140afd499 mov      r9d, 6
0x140afd49f jmp      0x140afd4ae ; 
0x140afd4a1 lea      r8, [rip + 0x1fe831e] ; @0x142ae57c6 'TreeTerrainEditingWaterGardenPhotomodeTimeOfDayPhotomodeFolderPhotomodeCameraPhotomodeFovPhotomodeFo'
0x140afd4a8 mov      r9d, 4
0x140afd4ae mov      rcx, rsi
0x140afd4b1 mov      rdx, rdi
0x140afd4b4 call     0x141580910 ; serde_json::ser::format_escaped_str::h775c3d29342bbbda
0x140afd4b9 test     rax, rax
0x140afd4bc jne      0x140afcff6
0x140afd4c2 mov      byte ptr [rsi + 0x20], 1
0x140afd4c6 mov      rdi, qword ptr [rsi]
0x140afd4c9 mov      r12, qword ptr [rsi + 0x18]
0x140afd4cd dec      r12
0x140afd4d0 mov      qword ptr [rsi + 0x18], r12
0x140afd4d4 mov      rdx, qword ptr [rdi + 0x10]
0x140afd4d8 cmp      qword ptr [rdi], rdx
0x140afd4db je       0x140afd859
0x140afd4e1 mov      rax, qword ptr [rdi + 8]
0x140afd4e5 mov      byte ptr [rax + rdx], 0xa
0x140afd4e9 inc      rdx
0x140afd4ec mov      qword ptr [rdi + 0x10], rdx
0x140afd4f0 test     r12, r12
0x140afd4f3 je       0x140afd55e
0x140afd4f5 mov      rdi, qword ptr [rsi + 8]
0x140afd4f9 mov      rbx, qword ptr [rsi + 0x10]
0x140afd4fd mov      r14, qword ptr [rsi]
0x140afd500 mov      r15, qword ptr [r14 + 0x10]
0x140afd504 nop      word ptr cs:[rax + rax]
0x140afd510 mov      rax, qword ptr [r14]
0x140afd513 sub      rax, r15
0x140afd516 cmp      rax, rbx
0x140afd519 jb       0x140afd53b
0x140afd51b mov      rcx, qword ptr [r14 + 8]
0x140afd51f add      rcx, r15
0x140afd522 mov      rdx, rdi
0x140afd525 mov      r8, rbx
0x140afd528 call     0x142923c00 ; memcpy
0x140afd52d add      r15, rbx
0x140afd530 mov      qword ptr [r14 + 0x10], r15
0x140afd534 dec      r12
0x140afd537 jne      0x140afd510
0x140afd539 jmp      0x140afd55e ; 
0x140afd53b mov      qword ptr [rsp + 0x20], 1
0x140afd544 mov      r9d, 1
0x140afd54a mov      rcx, r14
0x140afd54d mov      rdx, r15
0x140afd550 mov      r8, rbx
0x140afd553 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afd558 mov      r15, qword ptr [r14 + 0x10]
0x140afd55c jmp      0x140afd51b ; 
0x140afd55e mov      rsi, qword ptr [rsi]
0x140afd561 mov      rdx, qword ptr [rsi + 0x10]
0x140afd565 cmp      qword ptr [rsi], rdx
0x140afd568 je       0x140afd58c
0x140afd56a mov      rax, qword ptr [rsi + 8]
0x140afd56e mov      byte ptr [rax + rdx], 0x7d
0x140afd572 inc      rdx
0x140afd575 mov      qword ptr [rsi + 0x10], rdx
0x140afd579 xor      eax, eax
0x140afd57b add      rsp, 0x68
0x140afd57f pop      rbx
0x140afd580 pop      rbp
0x140afd581 pop      rdi
0x140afd582 pop      rsi
0x140afd583 pop      r12
0x140afd585 pop      r13
0x140afd587 pop      r14
0x140afd589 pop      r15
0x140afd58b ret      
0x140afd58c mov      qword ptr [rsp + 0x20], 1
0x140afd595 mov      r8d, 1
0x140afd59b mov      r9d, 1
0x140afd5a1 mov      rcx, rsi
0x140afd5a4 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afd5a9 mov      rdx, qword ptr [rsi + 0x10]
0x140afd5ad jmp      0x140afd56a ; 
0x140afd5af mov      qword ptr [rsp + 0x20], 1
0x140afd5b8 mov      r8d, 1
0x140afd5be mov      r9d, 1
0x140afd5c4 mov      rcx, rbx
0x140afd5c7 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afd5cc mov      rdx, qword ptr [rbx + 0x10]
0x140afd5d0 jmp      0x140afc8fa ; 
0x140afd5d5 mov      qword ptr [rsp + 0x20], 1
0x140afd5de mov      r8d, 1
0x140afd5e4 mov      r9d, 1
0x140afd5ea mov      rcx, rbx
0x140afd5ed call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afd5f2 mov      rdx, qword ptr [rbx + 0x10]
0x140afd5f6 jmp      0x140afc912 ; 
0x140afd5fb mov      qword ptr [rsp + 0x20], 1
0x140afd604 mov      r8d, 1
0x140afd60a mov      r9d, 1
0x140afd610 mov      rcx, r14
0x140afd613 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afd618 mov      rdx, qword ptr [r14 + 0x10]
0x140afd61c jmp      0x140afcaa9 ; 
0x140afd621 mov      qword ptr [rsp + 0x20], 1
0x140afd62a mov      r8d, 1
0x140afd630 mov      r9d, 1
0x140afd636 mov      rcx, r14
0x140afd639 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afd63e mov      rdx, qword ptr [r14 + 0x10]
0x140afd642 jmp      0x140afcac1 ; 
0x140afd647 mov      qword ptr [rsp + 0x20], 1
0x140afd650 mov      r8d, 1
0x140afd656 mov      r9d, 1
0x140afd65c mov      rdi, rcx
0x140afd65f mov      rcx, rbx
0x140afd662 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afd667 mov      rcx, rdi
0x140afd66a mov      rdx, qword ptr [rbx + 0x10]
0x140afd66e jmp      0x140afc818 ; 
0x140afd673 mov      qword ptr [rsp + 0x20], 1
0x140afd67c mov      r8d, 1
0x140afd682 mov      r9d, 1
0x140afd688 mov      rdi, rcx
0x140afd68b mov      rcx, rbx
0x140afd68e call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afd693 mov      rcx, rdi
0x140afd696 mov      rdx, qword ptr [rbx + 0x10]
0x140afd69a jmp      0x140afc830 ; 
0x140afd69f mov      qword ptr [rsp + 0x20], 1
0x140afd6a8 mov      r8d, 1
0x140afd6ae mov      r9d, 1
0x140afd6b4 mov      rcx, rbx
0x140afd6b7 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afd6bc mov      rdx, qword ptr [rbx + 0x10]
0x140afd6c0 jmp      0x140afcc53 ; 
0x140afd6c5 mov      qword ptr [rsp + 0x20], 1
0x140afd6ce mov      r8d, 1
0x140afd6d4 mov      r9d, 1
0x140afd6da mov      rcx, rbx
0x140afd6dd call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afd6e2 mov      rdx, qword ptr [rbx + 0x10]
0x140afd6e6 jmp      0x140afcc6b ; 
0x140afd6eb mov      qword ptr [rsp + 0x20], 1
0x140afd6f4 mov      r8d, 1
0x140afd6fa mov      r9d, 1
0x140afd700 mov      rdi, rcx
0x140afd703 mov      rcx, rbx
0x140afd706 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afd70b mov      rcx, rdi
0x140afd70e mov      rdx, qword ptr [rbx + 0x10]
0x140afd712 jmp      0x140afce2a ; 
0x140afd717 mov      qword ptr [rsp + 0x20], 1
0x140afd720 mov      r8d, 1
0x140afd726 mov      r9d, 1
0x140afd72c mov      rdi, rcx
0x140afd72f mov      rcx, rbx
0x140afd732 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afd737 mov      rcx, rdi
0x140afd73a mov      rdx, qword ptr [rbx + 0x10]
0x140afd73e jmp      0x140afce42 ; 
0x140afd743 mov      qword ptr [rsp + 0x20], 1
0x140afd74c mov      r8d, 2
0x140afd752 mov      r9d, 1
0x140afd758 mov      rcx, rbx
0x140afd75b call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afd760 mov      rdx, qword ptr [rbx + 0x10]
0x140afd764 jmp      0x140afc9c6 ; 
0x140afd769 mov      qword ptr [rsp + 0x20], 1
0x140afd772 mov      r8d, 2
0x140afd778 mov      r9d, 1
0x140afd77e mov      rcx, rbx
0x140afd781 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afd786 mov      rdx, qword ptr [rbx + 0x10]
0x140afd78a jmp      0x140afcb7a ; 
0x140afd78f mov      qword ptr [rsp + 0x20], 1
0x140afd798 mov      r9d, 1
0x140afd79e mov      rcx, rdi
0x140afd7a1 mov      r15, rdx
0x140afd7a4 mov      rdx, r14
0x140afd7a7 mov      r8, rbx
0x140afd7aa call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afd7af mov      rdx, r15
0x140afd7b2 mov      r14, qword ptr [rdi + 0x10]
0x140afd7b6 jmp      0x140afd089 ; 
0x140afd7bb mov      qword ptr [rsp + 0x20], 1
0x140afd7c4 mov      r8d, 1
0x140afd7ca mov      r9d, 1
0x140afd7d0 mov      rcx, rdi
0x140afd7d3 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afd7d8 mov      rdx, qword ptr [rdi + 0x10]
0x140afd7dc jmp      0x140afd0be ; 
0x140afd7e1 mov      qword ptr [rsp + 0x20], 1
0x140afd7ea mov      r8d, 2
0x140afd7f0 mov      r9d, 1
0x140afd7f6 mov      rcx, rbx
0x140afd7f9 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afd7fe mov      rdx, qword ptr [rbx + 0x10]
0x140afd802 jmp      0x140afcd29 ; 
0x140afd807 mov      qword ptr [rsp + 0x20], 1
0x140afd810 mov      r8d, 2
0x140afd816 mov      r9d, 1
0x140afd81c mov      rcx, rbx
0x140afd81f call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afd824 mov      rdx, qword ptr [rbx + 0x10]
0x140afd828 jmp      0x140afcdb0 ; 
0x140afd82d mov      qword ptr [rsp + 0x20], 1
0x140afd836 mov      r8d, 2
0x140afd83c mov      r9d, 1
0x140afd842 mov      rbx, rcx
0x140afd845 mov      rcx, rdi
0x140afd848 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afd84d mov      rcx, rbx
0x140afd850 mov      rdx, qword ptr [rdi + 0x10]
0x140afd854 jmp      0x140afcefd ; 
0x140afd859 mov      qword ptr [rsp + 0x20], 1
0x140afd862 mov      r8d, 1
0x140afd868 mov      r9d, 1
0x140afd86e mov      rcx, rdi
0x140afd871 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afd876 mov      rdx, qword ptr [rdi + 0x10]
0x140afd87a jmp      0x140afd4e1 ; 
0x140afd87f mov      qword ptr [rsp + 0x20], 1
0x140afd888 mov      r8d, 1
0x140afd88e mov      r9d, 1
0x140afd894 mov      rcx, rdi
0x140afd897 call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afd89c mov      rdx, qword ptr [rdi + 0x10]
0x140afd8a0 jmp      0x140afd190 ; 
0x140afd8a5 mov      qword ptr [rsp + 0x20], 1
0x140afd8ae mov      r8d, 1
0x140afd8b4 mov      r9d, 1
0x140afd8ba mov      rcx, rdi
0x140afd8bd call     0x1428a9a60 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hdf5333d5fb0f60c2
0x140afd8c2 mov      rdx, qword ptr [rdi + 0x10]
0x140afd8c6 jmp      0x140afcf3f ; 