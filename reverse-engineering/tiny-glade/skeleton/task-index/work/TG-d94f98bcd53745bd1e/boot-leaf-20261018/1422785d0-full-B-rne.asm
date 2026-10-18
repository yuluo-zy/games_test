1422785d0: 4157 push r15
1422785d2: 4156 push r14
1422785d4: 4155 push r13
1422785d6: 4154 push r12
1422785d8: 56 push rsi
1422785d9: 57 push rdi
1422785da: 55 push rbp
1422785db: 53 push rbx
1422785dc: 4881ecd8000000 sub rsp, 0xd8
1422785e3: 440f299424c0000000 movaps xmmword ptr [rsp + 0xc0], xmm10
1422785ec: 440f298c24b0000000 movaps xmmword ptr [rsp + 0xb0], xmm9
1422785f5: 440f298424a0000000 movaps xmmword ptr [rsp + 0xa0], xmm8
1422785fe: 0f29bc2490000000 movaps xmmword ptr [rsp + 0x90], xmm7
142278606: 0f29b42480000000 movaps xmmword ptr [rsp + 0x80], xmm6
14227860e: 4c89c6 mov rsi, r8
142278611: 4889d7 mov rdi, rdx
142278614: 4889cb mov rbx, rcx
142278617: 488d4c2438 lea rcx, [rsp + 0x38]
14227861c: 4889da mov rdx, rbx
14227861f: e8dcdf82fe call 0x140aa6600 ; country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::bounds_ws
142278624: f2440f10542438 movsd xmm10, qword ptr [rsp + 0x38]
14227862b: f2440f104c2440 movsd xmm9, qword ptr [rsp + 0x40]
142278632: 488d4c2470 lea rcx, [rsp + 0x70]
142278637: 48897c2468 mov qword ptr [rsp + 0x68], rdi
14227863c: 4889fa mov rdx, rdi
14227863f: e8fcfaa1fe call 0x140c98140 ; utils::geometry::circle::Circle2d::aabb
142278644: 4889d9 mov rcx, rbx
142278647: e894df82fe call 0x140aa65e0 ; country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::cell_size
14227864c: 0f28f0 movaps xmm6, xmm0
14227864f: f2440f10442470 movsd xmm8, qword ptr [rsp + 0x70]
142278656: f20f107c2478 movsd xmm7, qword ptr [rsp + 0x78]
14227865c: 410f28c2 movaps xmm0, xmm10
142278660: 410f5fc0 maxps xmm0, xmm8
142278664: 450fc2c003 cmpunordps xmm8, xmm8
142278669: 450f54d0 andps xmm10, xmm8
14227866d: 440f55c0 andnps xmm8, xmm0
142278671: 450f56c2 orps xmm8, xmm10
142278675: 488d4c2438 lea rcx, [rsp + 0x38]
14227867a: 4889da mov rdx, rbx
14227867d: e87edf82fe call 0x140aa6600 ; country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::bounds_ws
142278682: f20f10442438 movsd xmm0, qword ptr [rsp + 0x38]
142278688: 410fc2c002 cmpleps xmm0, xmm8
14227868d: 0f14c0 unpcklps xmm0, xmm0
142278690: 660f50c0 movmskpd eax, xmm0
142278694: 83f803 cmp eax, 3
142278697: 0f851b020000 jne 0x1422788b8
14227869d: 410f28c1 movaps xmm0, xmm9
1422786a1: 0f5dc7 minps xmm0, xmm7
1422786a4: 0fc2ff03 cmpunordps xmm7, xmm7
1422786a8: 440f54cf andps xmm9, xmm7
1422786ac: 0f55f8 andnps xmm7, xmm0
1422786af: 410f56f9 orps xmm7, xmm9
1422786b3: f20f10442440 movsd xmm0, qword ptr [rsp + 0x40]
1422786b9: 0f28cf movaps xmm1, xmm7
1422786bc: 0fc2c802 cmpleps xmm1, xmm0
1422786c0: 0f14c9 unpcklps xmm1, xmm1
1422786c3: 660f50c1 movmskpd eax, xmm1
1422786c7: 83f803 cmp eax, 3
1422786ca: 0f8500020000 jne 0x1422788d0
1422786d0: 4889d9 mov rcx, rbx
1422786d3: e808df82fe call 0x140aa65e0 ; country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::cell_size
1422786d8: 0f2e0591d16a00 ucomiss xmm0, dword ptr [rip + 0x6ad191]
1422786df: 0f8603020000 jbe 0x1422788e8
1422786e5: 4889742450 mov qword ptr [rsp + 0x50], rsi
1422786ea: f3410f16d0 movshdup xmm2, xmm8
1422786ef: 4889d9 mov rcx, rbx
1422786f2: 410f28c8 movaps xmm1, xmm8
1422786f6: e825de82fe call 0x140aa6520 ; country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::world_space_to_grid_coord_unclamped
1422786fb: 4189c5 mov r13d, eax
1422786fe: 4531e4 xor r12d, r12d
142278701: 85c0 test eax, eax
142278703: 450f4eec cmovle r13d, r12d
142278707: 85d2 test edx, edx
142278709: 440f4fe2 cmovg r12d, edx
14227870d: 0f57c0 xorps xmm0, xmm0
142278710: f30f16d7 movshdup xmm2, xmm7
142278714: f30f58f8 addss xmm7, xmm0
142278718: f30f58d0 addss xmm2, xmm0
14227871c: 4889d9 mov rcx, rbx
14227871f: 0f28cf movaps xmm1, xmm7
142278722: e8f9dd82fe call 0x140aa6520 ; country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::world_space_to_grid_coord_unclamped
142278727: 4189c6 mov r14d, eax
14227872a: 4c8b7b48 mov r15, qword ptr [rbx + 0x48]
14227872e: 418d47ff lea eax, [r15 - 1]
142278732: 4d89f8 mov r8, r15
142278735: 49c1e820 shr r8, 0x20
142278739: 41ffc8 dec r8d
14227873c: 4439f0 cmp eax, r14d
14227873f: 440f4cf0 cmovl r14d, eax
142278743: 4139d0 cmp r8d, edx
142278746: 440f4dc2 cmovge r8d, edx
14227874a: 4539c4 cmp r12d, r8d
14227874d: 0f8f26010000 jg 0x142278879
142278753: 4539f5 cmp r13d, r14d
142278756: 7e25 jle 0x14227877d
142278758: 0f1f840000000000 nop dword ptr [rax + rax]
142278760: 31c0 xor eax, eax
142278762: 4539c4 cmp r12d, r8d
142278765: 0f9cc1 setl cl
142278768: 0f8d0b010000 jge 0x142278879
14227876e: 88c8 mov al, cl
142278770: 4101c4 add r12d, eax
142278773: 4539c4 cmp r12d, r8d
142278776: 7ee8 jle 0x142278760
142278778: e9fc000000 jmp 0x142278879
14227877d: f30f5935ebd06a00 mulss xmm6, dword ptr [rip + 0x6ad0eb]
142278785: f30f5935675f6b00 mulss xmm6, dword ptr [rip + 0x6b5f67]
14227878d: 488b4308 mov rax, qword ptr [rbx + 8]
142278791: 4889442458 mov qword ptr [rsp + 0x58], rax
142278796: 488b4310 mov rax, qword ptr [rbx + 0x10]
14227879a: 4889442430 mov qword ptr [rsp + 0x30], rax
14227879f: 48895c2460 mov qword ptr [rsp + 0x60], rbx
1422787a4: 44896c242c mov dword ptr [rsp + 0x2c], r13d
1422787a9: 4c89442448 mov qword ptr [rsp + 0x48], r8
1422787ae: 488b6c2460 mov rbp, qword ptr [rsp + 0x60]
1422787b3: 4589e7 mov r15d, r12d
1422787b6: 4531e4 xor r12d, r12d
1422787b9: 4539c7 cmp r15d, r8d
1422787bc: 410f9cc4 setl r12b
1422787c0: 4501fc add r12d, r15d
1422787c3: 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
1422787d0: 31ff xor edi, edi
1422787d2: 4539f5 cmp r13d, r14d
1422787d5: 0f9cc3 setl bl
1422787d8: 4889e9 mov rcx, rbp
1422787db: 4489ea mov edx, r13d
1422787de: 4589f8 mov r8d, r15d
1422787e1: e8eadd82fe call 0x140aa65d0 ; country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::grid_coord_to_cell_index
1422787e6: 4889c6 mov rsi, rax
1422787e9: 4889e9 mov rcx, rbp
1422787ec: 4889c2 mov rdx, rax
1422787ef: e87cd482fe call 0x140aa5c70 ; country_core::systems::wall::wall_state::wall_spatial_hash::WallSpatialHash::cell_center
1422787f4: 488b442468 mov rax, qword ptr [rsp + 0x68]
1422787f9: f20f1010 movsd xmm2, qword ptr [rax]
1422787fd: 0f14c1 unpcklps xmm0, xmm1
142278800: 0f5cc2 subps xmm0, xmm2
142278803: 0f59c0 mulps xmm0, xmm0
142278806: f30f16c8 movshdup xmm1, xmm0
14227880a: f30f58c8 addss xmm1, xmm0
14227880e: 0f57c0 xorps xmm0, xmm0
142278811: f30f51c1 sqrtss xmm0, xmm1
142278815: f30f104808 movss xmm1, dword ptr [rax + 8]
14227881a: f30f58ce addss xmm1, xmm6
14227881e: 0f2ec8 ucomiss xmm1, xmm0
142278821: 7620 jbe 0x142278843
142278823: 4839742430 cmp qword ptr [rsp + 0x30], rsi
142278828: 0f86d2000000 jbe 0x142278900
14227882e: 488d0476 lea rax, [rsi + rsi*2]
142278832: 48c1e004 shl rax, 4
142278836: 488b4c2458 mov rcx, qword ptr [rsp + 0x58]
14227883b: 48837c011000 cmp qword ptr [rcx + rax + 0x10], 0
142278841: 752e jne 0x142278871
142278843: 4539f5 cmp r13d, r14d
142278846: 7d0f jge 0x142278857
142278848: 4088df mov dil, bl
14227884b: 4101fd add r13d, edi
14227884e: 4539f5 cmp r13d, r14d
142278851: 0f8e79ffffff jle 0x1422787d0
142278857: 4c8b442448 mov r8, qword ptr [rsp + 0x48]
14227885c: 4539c7 cmp r15d, r8d
14227885f: 448b6c242c mov r13d, dword ptr [rsp + 0x2c]
142278864: 7d13 jge 0x142278879
142278866: 4539c4 cmp r12d, r8d
142278869: 0f8e44ffffff jle 0x1422787b3
14227886f: eb08 jmp 0x142278879
142278871: 488b442450 mov rax, qword ptr [rsp + 0x50]
142278876: c60000 mov byte ptr [rax], 0
142278879: 0f28b42480000000 movaps xmm6, xmmword ptr [rsp + 0x80]
142278881: 0f28bc2490000000 movaps xmm7, xmmword ptr [rsp + 0x90]
142278889: 440f288424a0000000 movaps xmm8, xmmword ptr [rsp + 0xa0]
142278892: 440f288c24b0000000 movaps xmm9, xmmword ptr [rsp + 0xb0]
14227889b: 440f289424c0000000 movaps xmm10, xmmword ptr [rsp + 0xc0]
1422788a4: 4881c4d8000000 add rsp, 0xd8
1422788ab: 5b pop rbx
1422788ac: 5d pop rbp
1422788ad: 5f pop rdi
1422788ae: 5e pop rsi
1422788af: 415c pop r12
1422788b1: 415d pop r13
1422788b3: 415e pop r14
1422788b5: 415f pop r15
1422788b7: c3 ret 
1422788b8: 488d0d01ecdb00 lea rcx, [rip + 0xdbec01] ; @1430374c0 'assertion failed: bbx_min.x >= grid_min.x && bbx_min.y >= grid_min.y/home/h3/code/pouncelight/co'
1422788bf: 4c8d05b2ecdb00 lea r8, [rip + 0xdbecb2]
1422788c6: ba44000000 mov edx, 0x44
1422788cb: e8600b6600 call 0x1428d9430 ; core::panicking::panic
1422788d0: 488d0db9ecdb00 lea rcx, [rip + 0xdbecb9] ; @143037590 'assertion failed: bbx_max.x <= grid_max.x && bbx_max.y <= grid_max.y'
1422788d7: 4c8d05faecdb00 lea r8, [rip + 0xdbecfa]
1422788de: ba44000000 mov edx, 0x44
1422788e3: e8480b6600 call 0x1428d9430 ; core::panicking::panic
1422788e8: 488d0d01eddb00 lea rcx, [rip + 0xdbed01] ; @1430375f0 'assertion failed: RESAMPLING < self.cell_size().x'
1422788ef: 4c8d0532eddb00 lea r8, [rip + 0xdbed32]
1422788f6: ba31000000 mov edx, 0x31
1422788fb: e8300b6600 call 0x1428d9430 ; core::panicking::panic
142278900: 4c8d0539eddb00 lea r8, [rip + 0xdbed39]
142278907: 4889f1 mov rcx, rsi
14227890a: 488b542430 mov rdx, qword ptr [rsp + 0x30]
14227890f: e8040c6600 call 0x1428d9518 ; core::panicking::panic_bounds_check
142278914: cc int3 
