1407f75c7: 488b8508020000 mov rax, qword ptr [rbp + 0x208]
1407f75ce: 8078da02 cmp byte ptr [rax - 0x26], 2
1407f75d2: 0f83750f0000 jae 0x1407f854d
1407f75d8: f20f1032 movsd xmm6, qword ptr [rdx]
1407f75dc: f20f11b510010000 movsd qword ptr [rbp + 0x110], xmm6
1407f75e4: f20f11b578010000 movsd qword ptr [rbp + 0x178], xmm6
1407f75ec: 488b8db8010000 mov rcx, qword ptr [rbp + 0x1b8]
1407f75f3: e848313300 call 0x140b2a740 ; country_core::resources::walls::public_wall_state::PublicWallState::wall_space
1407f75f8: 8985c0010000 mov dword ptr [rbp + 0x1c0], eax
1407f75fe: 660f7e85c4010000 movd dword ptr [rbp + 0x1c4], xmm0
1407f7606: f30f108d78010000 movss xmm1, dword ptr [rbp + 0x178]
1407f760e: f30f580de2401f02 addss xmm1, dword ptr [rip + 0x21f40e2]
1407f7616: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1407f761d: e88ea84900 call 0x140c91eb0 ; utils::wall_space::impl$1::project_clamp
1407f7622: 660f7e8578010000 movd dword ptr [rbp + 0x178], xmm0
1407f762a: 0f13b590010000 movlps qword ptr [rbp + 0x190], xmm6
1407f7631: 488b8db8010000 mov rcx, qword ptr [rbp + 0x1b8]
1407f7638: e803313300 call 0x140b2a740 ; country_core::resources::walls::public_wall_state::PublicWallState::wall_space
1407f763d: 8985c0010000 mov dword ptr [rbp + 0x1c0], eax
1407f7643: 660f7e85c4010000 movd dword ptr [rbp + 0x1c4], xmm0
1407f764b: f30f108d90010000 movss xmm1, dword ptr [rbp + 0x190]
1407f7653: f30f580d29e31202 addss xmm1, dword ptr [rip + 0x212e329]
1407f765b: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1407f7662: e849a84900 call 0x140c91eb0 ; utils::wall_space::impl$1::project_clamp
1407f7667: 660f7e8590010000 movd dword ptr [rbp + 0x190], xmm0
1407f766f: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1407f7676: 488d9510010000 lea rdx, [rbp + 0x110]
1407f767d: 4c8b85b8010000 mov r8, qword ptr [rbp + 0x1b8]
1407f7684: e807aa4900 call 0x140c92090 ; utils::wall_space::WallSpaceCoord::to_world_space
1407f7689: f30f10b5c0010000 movss xmm6, dword ptr [rbp + 0x1c0]
1407f7691: f30f10bdc8010000 movss xmm7, dword ptr [rbp + 0x1c8]
1407f7699: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1407f76a0: 488d9578010000 lea rdx, [rbp + 0x178]
1407f76a7: 4c8b85b8010000 mov r8, qword ptr [rbp + 0x1b8]
1407f76ae: e8dda94900 call 0x140c92090 ; utils::wall_space::WallSpaceCoord::to_world_space
1407f76b3: f3440f10bdc0010000 movss xmm15, dword ptr [rbp + 0x1c0]
1407f76bc: f3440f1085c8010000 movss xmm8, dword ptr [rbp + 0x1c8]
1407f76c5: 488d8dc0010000 lea rcx, [rbp + 0x1c0]
1407f76cc: 488d9590010000 lea rdx, [rbp + 0x190]
1407f76d3: 4c8b85b8010000 mov r8, qword ptr [rbp + 0x1b8]
1407f76da: e8b1a94900 call 0x140c92090 ; utils::wall_space::WallSpaceCoord::to_world_space
1407f76df: 450f14f8 unpcklps xmm15, xmm8
1407f76e3: 0f14f7 unpcklps xmm6, xmm7
1407f76e6: 440f5cfe subps xmm15, xmm6
1407f76ea: 410f28c7 movaps xmm0, xmm15
1407f76ee: 410f59c7 mulps xmm0, xmm15
1407f76f2: f30f16c8 movshdup xmm1, xmm0
1407f76f6: f30f58c8 addss xmm1, xmm0
1407f76fa: f30f51c9 sqrtss xmm1, xmm1
1407f76fe: f30f101dfee11202 movss xmm3, dword ptr [rip + 0x212e1fe]
1407f7706: 0f28c3 movaps xmm0, xmm3
1407f7709: f30f5ec1 divss xmm0, xmm1
1407f770d: 660f7ec0 movd eax, xmm0
1407f7711: 85c0 test eax, eax
1407f7713: 0f99c1 setns cl
1407f7716: 89c2 mov edx, eax
1407f7718: 81e2ffffff7f and edx, 0x7fffffff
1407f771e: 81c2000080ff add edx, 0xff800000
1407f7724: 81fa0000007f cmp edx, 0x7f000000
1407f772a: 0f92c2 setb dl
1407f772d: 20ca and dl, cl
1407f772f: ffc8 dec eax
1407f7731: 3dffff7f00 cmp eax, 0x7fffff
1407f7736: 0f92c0 setb al
1407f7739: 08d0 or al, dl
1407f773b: 0f845d0b0000 je 0x1407f829e
1407f7741: f3440f109dc0010000 movss xmm11, dword ptr [rbp + 0x1c0]
1407f774a: f30f108dc8010000 movss xmm1, dword ptr [rbp + 0x1c8]
1407f7752: 440f14d9 unpcklps xmm11, xmm1
1407f7756: 440f5cde subps xmm11, xmm6
1407f775a: 410f28cb movaps xmm1, xmm11
1407f775e: 410f59cb mulps xmm1, xmm11
1407f7762: f30f16d1 movshdup xmm2, xmm1
1407f7766: f30f58d1 addss xmm2, xmm1
1407f776a: f30f51d2 sqrtss xmm2, xmm2
1407f776e: 0f28cb movaps xmm1, xmm3
1407f7771: f30f5eca divss xmm1, xmm2
1407f7775: 660f7ec8 movd eax, xmm1
1407f7779: 85c0 test eax, eax
1407f777b: 0f98c1 sets cl
1407f777e: 89c2 mov edx, eax
1407f7780: 81e2ffffff7f and edx, 0x7fffffff
1407f7786: 81c2000080ff add edx, 0xff800000
1407f778c: 81fa0000007f cmp edx, 0x7f000000
1407f7792: 0f93c2 setae dl
1407f7795: 08ca or dl, cl
1407f7797: ffc8 dec eax
1407f7799: 3dffff7f00 cmp eax, 0x7fffff
1407f779e: 0f93c0 setae al
1407f77a1: 84d0 test al, dl
1407f77a3: 0f85f50a0000 jne 0x1407f829e
1407f77a9: f30f12c0 movsldup xmm0, xmm0
1407f77ad: 440f59f8 mulps xmm15, xmm0
1407f77b1: f30f12c1 movsldup xmm0, xmm1
1407f77b5: 440f59d8 mulps xmm11, xmm0
1407f77b9: 410f28f3 movaps xmm6, xmm11
1407f77bd: 410f5cf7 subps xmm6, xmm15
1407f77c1: 0f28c6 movaps xmm0, xmm6
1407f77c4: 0f59c6 mulps xmm0, xmm6
1407f77c7: f30f16c8 movshdup xmm1, xmm0
1407f77cb: f30f58c8 addss xmm1, xmm0
1407f77cf: f30f51c9 sqrtss xmm1, xmm1
1407f77d3: 0f28c3 movaps xmm0, xmm3
1407f77d6: f30f5ec1 divss xmm0, xmm1
1407f77da: 660f7ec0 movd eax, xmm0
1407f77de: 85c0 test eax, eax
1407f77e0: 0f98c1 sets cl
1407f77e3: 89c2 mov edx, eax
1407f77e5: 81e2ffffff7f and edx, 0x7fffffff
1407f77eb: 81c2000080ff add edx, 0xff800000
1407f77f1: 81fa0000007f cmp edx, 0x7f000000
1407f77f7: 0f93c2 setae dl
1407f77fa: 08ca or dl, cl
1407f77fc: ffc8 dec eax
1407f77fe: 3dffff7f00 cmp eax, 0x7fffff
1407f7803: 0f93c0 setae al
1407f7806: 84d0 test al, dl
1407f7808: 0f85900a0000 jne 0x1407f829e
1407f780e: f30f16fe movshdup xmm7, xmm6
1407f7812: 0f573dd7e01202 xorps xmm7, xmmword ptr [rip + 0x212e0d7]
1407f7819: f30f59f8 mulss xmm7, xmm0
1407f781d: f30f59f0 mulss xmm6, xmm0
1407f7821: e962f5ffff jmp 0x1407f6d88 ; unknown
