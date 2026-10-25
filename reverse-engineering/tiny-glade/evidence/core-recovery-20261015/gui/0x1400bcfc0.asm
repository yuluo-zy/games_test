0x1400bcfc0 push     rbp
0x1400bcfc1 push     r15
0x1400bcfc3 push     r14
0x1400bcfc5 push     r12
0x1400bcfc7 push     rsi
0x1400bcfc8 push     rdi
0x1400bcfc9 push     rbx
0x1400bcfca sub      rsp, 0x3b0
0x1400bcfd1 lea      rbp, [rsp + 0x80]
0x1400bcfd9 mov      qword ptr [rbp + 0x328], 0xfffffffffffffffe
0x1400bcfe4 mov      rsi, rdx
0x1400bcfe7 mov      rdi, rcx
0x1400bcfea movzx    eax, byte ptr [rip + 0x3577c18]
0x1400bcff1 mov      ecx, 0xb
0x1400bcff6 mov      edx, 1
0x1400bcffb call     0x140613c10
0x1400bd000 test     rax, rax
0x1400bd003 je       0x1400bd90d
0x1400bd009 movabs   rcx, 0x61747320797a614c
0x1400bd013 mov      qword ptr [rax], rcx
0x1400bd016 mov      dword ptr [rax + 7], 0x73726961
0x1400bd01d mov      qword ptr [rbp + 0x78], 0xb
0x1400bd025 mov      qword ptr [rbp + 0x80], rax
0x1400bd02c mov      qword ptr [rbp + 0x88], 0xb
0x1400bd037 mov      dword ptr [rbp + 0x90], 0
0x1400bd041 mov      qword ptr [rbp + 0xa0], 0
0x1400bd04c mov      dword ptr [rbp + 0x98], 0
0x1400bd056 mov      qword ptr [rbp + 0x48], 3
0x1400bd05e mov      qword ptr [rbp + 0x60], 6
0x1400bd066 mov      byte ptr [rbp + 0xa8], 0
0x1400bd06d mov      dword ptr [rbp + 0xad], 0
0x1400bd077 mov      dword ptr [rbp + 0xb0], 0
0x1400bd081 mov      rax, qword ptr [rip + 0x356ef90]
0x1400bd088 mov      qword ptr [rbp + 0xb8], rax
0x1400bd08f mov      byte ptr [rbp + 0xc0], 0
0x1400bd096 lea      rcx, [rbp + 0x250]
0x1400bd09d lea      rdx, [rbp + 0x48]
0x1400bd0a1 mov      r8, rsi
0x1400bd0a4 call     0x140c48930
0x1400bd0a9 mov      rax, qword ptr [rbp + 0x260]
0x1400bd0b0 lock dec qword ptr [rax]
0x1400bd0b4 jne      0x1400bd0c2
0x1400bd0b6 lea      rcx, [rbp + 0x260]
0x1400bd0bd call     0x140c41170
0x1400bd0c2 mov      r14, qword ptr [rdi]
0x1400bd0c5 mov      eax, dword ptr [r14 + 0x1c]
0x1400bd0c9 mov      rcx, qword ptr [r14 + 0x10]
0x1400bd0cd mov      dword ptr [rcx], eax
0x1400bd0cf mov      rbx, qword ptr [r14]
0x1400bd0d2 movzx    eax, byte ptr [rip + 0x3577b30]
0x1400bd0d9 mov      ecx, 0xa
0x1400bd0de mov      edx, 1
0x1400bd0e3 call     0x140613c10
0x1400bd0e8 test     rax, rax
0x1400bd0eb je       0x1400bd923
0x1400bd0f1 add      rbx, 0xf7
0x1400bd0f8 movabs   rcx, 0x20443220776f6853
0x1400bd102 mov      qword ptr [rax], rcx
0x1400bd105 mov      word ptr [rax + 8], 0x4955
0x1400bd10b mov      qword ptr [rbp + 0x78], 0xa
0x1400bd113 mov      qword ptr [rbp + 0x80], rax
0x1400bd11a mov      qword ptr [rbp + 0x88], 0xa
0x1400bd125 mov      dword ptr [rbp + 0x90], 0
0x1400bd12f mov      qword ptr [rbp + 0xa0], 0
0x1400bd13a mov      dword ptr [rbp + 0x98], 0
0x1400bd144 mov      qword ptr [rbp + 0x48], 3
0x1400bd14c mov      qword ptr [rbp + 0x60], 6
0x1400bd154 mov      byte ptr [rbp + 0xa8], 0
0x1400bd15b mov      dword ptr [rbp + 0xad], 0
0x1400bd165 mov      dword ptr [rbp + 0xb0], 0
0x1400bd16f mov      qword ptr [rbp + 0xb8], rbx
0x1400bd176 mov      byte ptr [rbp + 0xc0], 0
0x1400bd17d lea      rcx, [rbp + 0x250]
0x1400bd184 lea      rdx, [rbp + 0x48]
0x1400bd188 mov      r8, rsi
0x1400bd18b call     0x140c48930
0x1400bd190 mov      rax, qword ptr [rbp + 0x260]
0x1400bd197 lock dec qword ptr [rax]
0x1400bd19b jne      0x1400bd1a9
0x1400bd19d lea      rcx, [rbp + 0x260]
0x1400bd1a4 call     0x140c41170
0x1400bd1a9 mov      eax, dword ptr [r14 + 0x1c]
0x1400bd1ad mov      rcx, qword ptr [r14 + 0x10]
0x1400bd1b1 mov      dword ptr [rcx], eax
0x1400bd1b3 mov      rbx, qword ptr [r14]
0x1400bd1b6 movzx    eax, byte ptr [rip + 0x3577a4c]
0x1400bd1bd mov      ecx, 0x13
0x1400bd1c2 mov      edx, 1
0x1400bd1c7 call     0x140613c10
0x1400bd1cc test     rax, rax
0x1400bd1cf je       0x1400bd939
0x1400bd1d5 add      rbx, 0xf5
0x1400bd1dc movups   xmm0, xmmword ptr [rip + 0x28781b0] ; @0x142935393 'Show sheep controlsBirdsProxiesFinish shader refactoringhttps://github.com/anopara/country-slice'
0x1400bd1e3 movups   xmmword ptr [rax], xmm0
0x1400bd1e6 mov      dword ptr [rax + 0xf], 0x736c6f72
0x1400bd1ed mov      qword ptr [rbp + 0x78], 0x13
0x1400bd1f5 mov      qword ptr [rbp + 0x80], rax
0x1400bd1fc mov      qword ptr [rbp + 0x88], 0x13
0x1400bd207 mov      dword ptr [rbp + 0x90], 0
0x1400bd211 mov      qword ptr [rbp + 0xa0], 0
0x1400bd21c mov      dword ptr [rbp + 0x98], 0
0x1400bd226 mov      qword ptr [rbp + 0x48], 3
0x1400bd22e mov      qword ptr [rbp + 0x60], 6
0x1400bd236 mov      byte ptr [rbp + 0xa8], 0
0x1400bd23d mov      dword ptr [rbp + 0xad], 0
0x1400bd247 mov      dword ptr [rbp + 0xb0], 0
0x1400bd251 mov      qword ptr [rbp + 0xb8], rbx
0x1400bd258 mov      byte ptr [rbp + 0xc0], 0
0x1400bd25f lea      rcx, [rbp + 0x250]
0x1400bd266 lea      rdx, [rbp + 0x48]
0x1400bd26a mov      r8, rsi
0x1400bd26d call     0x140c48930
0x1400bd272 mov      rax, qword ptr [rbp + 0x260]
0x1400bd279 lock dec qword ptr [rax]
0x1400bd27d jne      0x1400bd28b
0x1400bd27f lea      rcx, [rbp + 0x260]
0x1400bd286 call     0x140c41170
0x1400bd28b mov      rbx, qword ptr [rdi + 8]
0x1400bd28f lea      rdx, [rip + 0x2878110] ; @0x1429353a6 'BirdsProxiesFinish shader refactoringhttps://github.com/anopara/country-slice-private/'
0x1400bd296 lea      rcx, [rbp - 0x50]
0x1400bd29a mov      r8d, 5
0x1400bd2a0 call     0x14015eea0
0x1400bd2a5 movzx    eax, byte ptr [rip + 0x357795d]
0x1400bd2ac mov      ecx, 8
0x1400bd2b1 mov      edx, 8
0x1400bd2b6 call     0x140613c10
0x1400bd2bb test     rax, rax
0x1400bd2be mov      qword ptr [rbp + 0x318], rax
0x1400bd2c5 je       0x1400bd94f
0x1400bd2cb mov      r12, rax
0x1400bd2ce mov      qword ptr [rax], rbx
0x1400bd2d1 mov      byte ptr [rbp + 0x1f0], 1
0x1400bd2d8 lea      rcx, [rbp + 0x250]
0x1400bd2df lea      rdx, [rbp - 0x50]
0x1400bd2e3 mov      r8d, 0x90
0x1400bd2e9 call     0x142923c00
0x1400bd2ee lea      r15, [rbp + 0x1f0]
0x1400bd2f5 mov      qword ptr [rbp + 0x2e0], r15
0x1400bd2fc mov      qword ptr [rbp + 0x2e8], r12
0x1400bd303 lea      rax, [rip + 0x28775c6]
0x1400bd30a mov      qword ptr [rbp + 0x2f0], rax
0x1400bd311 movzx    eax, byte ptr [rip + 0x35778f1]
0x1400bd318 mov      ecx, 0xa8
0x1400bd31d mov      edx, 8
0x1400bd322 call     0x140613c10
0x1400bd327 test     rax, rax
0x1400bd32a je       0x1400bd960
0x1400bd330 mov      rbx, rax
0x1400bd333 lea      rdx, [rbp + 0x250]
0x1400bd33a mov      r8d, 0xa8
0x1400bd340 mov      rcx, rax
0x1400bd343 call     0x142923c00
0x1400bd348 lea      r12, [rip + 0x289f4b1]
0x1400bd34f mov      qword ptr [rsp + 0x20], r12
0x1400bd354 lea      rcx, [rbp + 0x48]
0x1400bd358 movabs   r8, 0x20001000000
0x1400bd362 mov      rdx, rsi
0x1400bd365 mov      r9, rbx
0x1400bd368 call     0x1400c2f70
0x1400bd36d lea      rcx, [rbp + 0x250]
0x1400bd374 lea      rdx, [rbp + 0x48]
0x1400bd378 mov      r8d, 0xc8
0x1400bd37e call     0x142923c00
0x1400bd383 mov      rax, qword ptr [rbp + 0x120]
0x1400bd38a lock dec qword ptr [rax]
0x1400bd38e jne      0x1400bd39c
0x1400bd390 lea      rcx, [rbp + 0x120]
0x1400bd397 call     0x140c41170
0x1400bd39c mov      rax, qword ptr [rbp + 0x2c0]
0x1400bd3a3 lock dec qword ptr [rax]
0x1400bd3a7 jne      0x1400bd3b5
0x1400bd3a9 lea      rcx, [rbp + 0x2c0]
0x1400bd3b0 call     0x140c41170
0x1400bd3b5 cmp      dword ptr [rbp + 0x290], 2
0x1400bd3bc je       0x1400bd3d7
0x1400bd3be mov      rax, qword ptr [rbp + 0x260]
0x1400bd3c5 lock dec qword ptr [rax]
0x1400bd3c9 jne      0x1400bd3d7
0x1400bd3cb lea      rcx, [rbp + 0x260]
0x1400bd3d2 call     0x140c41170
0x1400bd3d7 lea      rdx, [rip + 0x2877fcd] ; @0x1429353ab 'ProxiesFinish shader refactoringhttps://github.com/anopara/country-slice-private/'
0x1400bd3de lea      rcx, [rbp - 0x50]
0x1400bd3e2 mov      r8d, 7
0x1400bd3e8 call     0x14015eea0
0x1400bd3ed movzx    eax, byte ptr [rip + 0x3577815]
0x1400bd3f4 mov      ecx, 8
0x1400bd3f9 mov      edx, 8
0x1400bd3fe call     0x140613c10
0x1400bd403 test     rax, rax
0x1400bd406 mov      qword ptr [rbp + 0x318], rax
0x1400bd40d je       0x1400bd971
0x1400bd413 mov      rbx, rax
0x1400bd416 mov      qword ptr [rax], r14
0x1400bd419 mov      byte ptr [rbp + 0x1f0], 1
0x1400bd420 lea      rcx, [rbp + 0x250]
0x1400bd427 lea      rdx, [rbp - 0x50]
0x1400bd42b mov      r8d, 0x90
0x1400bd431 call     0x142923c00
0x1400bd436 mov      qword ptr [rbp + 0x2e0], r15
0x1400bd43d mov      qword ptr [rbp + 0x2e8], rbx
0x1400bd444 lea      rax, [rip + 0x2877445]
0x1400bd44b mov      qword ptr [rbp + 0x2f0], rax
0x1400bd452 movzx    eax, byte ptr [rip + 0x35777b0]
0x1400bd459 mov      ecx, 0xa8
0x1400bd45e mov      edx, 8
0x1400bd463 call     0x140613c10
0x1400bd468 test     rax, rax
0x1400bd46b je       0x1400bd982
0x1400bd471 mov      rbx, rax
0x1400bd474 lea      rdx, [rbp + 0x250]
0x1400bd47b mov      r8d, 0xa8
0x1400bd481 mov      rcx, rax
0x1400bd484 call     0x142923c00
0x1400bd489 mov      qword ptr [rsp + 0x20], r12
0x1400bd48e lea      rcx, [rbp + 0x48]
0x1400bd492 movabs   r8, 0x20001000000
0x1400bd49c mov      rdx, rsi
0x1400bd49f mov      r9, rbx
0x1400bd4a2 call     0x1400c2f70
0x1400bd4a7 lea      rcx, [rbp + 0x250]
0x1400bd4ae lea      rdx, [rbp + 0x48]
0x1400bd4b2 mov      r8d, 0xc8
0x1400bd4b8 call     0x142923c00
0x1400bd4bd mov      rax, qword ptr [rbp + 0x120]
0x1400bd4c4 lock dec qword ptr [rax]
0x1400bd4c8 jne      0x1400bd4d6
0x1400bd4ca lea      rcx, [rbp + 0x120]
0x1400bd4d1 call     0x140c41170
0x1400bd4d6 mov      rax, qword ptr [rbp + 0x2c0]
0x1400bd4dd lock dec qword ptr [rax]
0x1400bd4e1 jne      0x1400bd4ef
0x1400bd4e3 lea      rcx, [rbp + 0x2c0]
0x1400bd4ea call     0x140c41170
0x1400bd4ef cmp      dword ptr [rbp + 0x290], 2
0x1400bd4f6 je       0x1400bd511
0x1400bd4f8 mov      rax, qword ptr [rbp + 0x260]
0x1400bd4ff lock dec qword ptr [rax]
0x1400bd503 jne      0x1400bd511
0x1400bd505 lea      rcx, [rbp + 0x260]
0x1400bd50c call     0x140c41170
0x1400bd511 mov      r15, qword ptr [rdi + 0x10]
0x1400bd515 mov      rcx, qword ptr [r15]
0x1400bd518 call     0x140b1f210
0x1400bd51d test     al, al
0x1400bd51f je       0x1400bd5fa
0x1400bd525 mov      qword ptr [rbp + 0x250], 4
0x1400bd530 mov      byte ptr [rbp + 0x326], 0
0x1400bd537 lea      rdx, [rip + 0x2877f62] ; @0x1429354a0 'Start shader refactoringShow haven populationAirborne: '
0x1400bd53e lea      rcx, [rbp + 0x48]
0x1400bd542 mov      r8d, 0x18
0x1400bd548 call     0x1400568d0
0x1400bd54d movups   xmm0, xmmword ptr [rbp + 0xa8]
0x1400bd554 movaps   xmmword ptr [rbp + 0x10], xmm0
0x1400bd558 movups   xmm0, xmmword ptr [rbp + 0x98]
0x1400bd55f movaps   xmmword ptr [rbp], xmm0
0x1400bd563 movups   xmm0, xmmword ptr [rbp + 0x88]
0x1400bd56a movaps   xmmword ptr [rbp - 0x10], xmm0
0x1400bd56e movups   xmm0, xmmword ptr [rbp + 0x48]
0x1400bd572 movups   xmm1, xmmword ptr [rbp + 0x58]
0x1400bd576 movups   xmm2, xmmword ptr [rbp + 0x68]
0x1400bd57a movups   xmm3, xmmword ptr [rbp + 0x78]
0x1400bd57e movaps   xmmword ptr [rbp - 0x20], xmm3
0x1400bd582 movaps   xmmword ptr [rbp - 0x30], xmm2
0x1400bd586 movaps   xmmword ptr [rbp - 0x40], xmm1
0x1400bd58a movaps   xmmword ptr [rbp - 0x50], xmm0
0x1400bd58e mov      byte ptr [rbp + 0x326], 1
0x1400bd595 lea      rcx, [rbp + 0x48]
0x1400bd599 lea      rdx, [rbp + 0x250]
0x1400bd5a0 lea      r8, [rbp - 0x50]
0x1400bd5a4 call     0x140c46bd0
0x1400bd5a9 lea      rcx, [rbp + 0x1f0]
0x1400bd5b0 lea      rdx, [rbp + 0x48]
0x1400bd5b4 mov      r8, rsi
0x1400bd5b7 call     0x140c46d20
0x1400bd5bc cmp      byte ptr [rbp + 0x244], 0
0x1400bd5c3 je       0x1400bd6d1
0x1400bd5c9 mov      rax, qword ptr [rbp + 0x200]
0x1400bd5d0 lock dec qword ptr [rax]
0x1400bd5d4 jne      0x1400bd5e2
0x1400bd5d6 lea      rcx, [rbp + 0x200]
0x1400bd5dd call     0x140c41170
0x1400bd5e2 mov      eax, dword ptr [r15 + 0x1c]
0x1400bd5e6 mov      rcx, qword ptr [r15 + 0x10]
0x1400bd5ea mov      dword ptr [rcx], eax
0x1400bd5ec mov      rax, qword ptr [r15]
0x1400bd5ef mov      dword ptr [rax], 1
0x1400bd5f5 jmp      0x1400bd8fa
0x1400bd5fa mov      qword ptr [rbp + 0x250], 4
0x1400bd605 mov      byte ptr [rbp + 0x327], 0
0x1400bd60c lea      rdx, [rip + 0x2877d9f] ; @0x1429353b2 'Finish shader refactoringhttps://github.com/anopara/country-slice-private/'
0x1400bd613 lea      rcx, [rbp + 0x48]
0x1400bd617 mov      r8d, 0x19
0x1400bd61d call     0x1400568d0
0x1400bd622 movups   xmm0, xmmword ptr [rbp + 0xa8]
0x1400bd629 movaps   xmmword ptr [rbp + 0x10], xmm0
0x1400bd62d movups   xmm0, xmmword ptr [rbp + 0x98]
0x1400bd634 movaps   xmmword ptr [rbp], xmm0
0x1400bd638 movups   xmm0, xmmword ptr [rbp + 0x88]
0x1400bd63f movaps   xmmword ptr [rbp - 0x10], xmm0
0x1400bd643 movups   xmm0, xmmword ptr [rbp + 0x48]
0x1400bd647 movups   xmm1, xmmword ptr [rbp + 0x58]
0x1400bd64b movups   xmm2, xmmword ptr [rbp + 0x68]
0x1400bd64f movups   xmm3, xmmword ptr [rbp + 0x78]
0x1400bd653 movaps   xmmword ptr [rbp - 0x20], xmm3
0x1400bd657 movaps   xmmword ptr [rbp - 0x30], xmm2
0x1400bd65b movaps   xmmword ptr [rbp - 0x40], xmm1
0x1400bd65f movaps   xmmword ptr [rbp - 0x50], xmm0
0x1400bd663 mov      byte ptr [rbp + 0x327], 1
0x1400bd66a lea      rcx, [rbp + 0x48]
0x1400bd66e lea      rbx, [rbp + 0x250]
0x1400bd675 lea      r8, [rbp - 0x50]
0x1400bd679 mov      rdx, rbx
0x1400bd67c call     0x140c46bd0
0x1400bd681 lea      rdi, [rbp + 0x1f0]
0x1400bd688 lea      rdx, [rbp + 0x48]
0x1400bd68c mov      rcx, rdi
0x1400bd68f mov      r8, rsi
0x1400bd692 call     0x140c46d20
0x1400bd697 cmp      byte ptr [rbp + 0x244], 0
0x1400bd69e je       0x1400bd713
0x1400bd6a0 mov      rax, qword ptr [rbp + 0x200]
0x1400bd6a7 lock dec qword ptr [rax]
0x1400bd6ab jne      0x1400bd6b9
0x1400bd6ad lea      rcx, [rbp + 0x200]
0x1400bd6b4 call     0x140c41170
0x1400bd6b9 mov      eax, dword ptr [r15 + 0x1c]
0x1400bd6bd mov      rcx, qword ptr [r15 + 0x10]
0x1400bd6c1 mov      dword ptr [rcx], eax
0x1400bd6c3 mov      rax, qword ptr [r15]
0x1400bd6c6 mov      dword ptr [rax], 0
0x1400bd6cc jmp      0x1400bd751
0x1400bd6d1 mov      byte ptr [rbp + 0x48], 0
0x1400bd6d5 cmp      byte ptr [rbp + 0x243], 0
0x1400bd6dc je       0x1400bd6f6
0x1400bd6de lea      rcx, [rbp + 0x200]
0x1400bd6e5 lea      rdx, [rbp + 0x48]
0x1400bd6e9 call     0x140042870
0x1400bd6ee test     al, al
0x1400bd6f0 jne      0x1400bd5c9
0x1400bd6f6 mov      rax, qword ptr [rbp + 0x200]
0x1400bd6fd lock dec qword ptr [rax]
0x1400bd701 jne      0x1400bd8fa
0x1400bd707 lea      rcx, [rbp + 0x200]
0x1400bd70e jmp      0x1400bd8f5
0x1400bd713 mov      byte ptr [rbp + 0x48], 0
0x1400bd717 cmp      byte ptr [rbp + 0x243], 0
0x1400bd71e je       0x1400bd738
0x1400bd720 lea      rcx, [rbp + 0x200]
0x1400bd727 lea      rdx, [rbp + 0x48]
0x1400bd72b call     0x140042870
0x1400bd730 test     al, al
0x1400bd732 jne      0x1400bd6a0
0x1400bd738 mov      rax, qword ptr [rbp + 0x200]
0x1400bd73f lock dec qword ptr [rax]
0x1400bd743 jne      0x1400bd751
0x1400bd745 lea      rcx, [rbp + 0x200]
0x1400bd74c call     0x140c41170
0x1400bd751 lea      rax, [rip + 0x2877ca8]
0x1400bd758 mov      qword ptr [rbp + 0x250], rax
0x1400bd75f lea      rax, [rip + 0x4c72a]
0x1400bd766 mov      qword ptr [rbp + 0x258], rax
0x1400bd76d lea      rax, [rip + 0x2877cbc]
0x1400bd774 mov      qword ptr [rbp + 0x48], rax
0x1400bd778 mov      qword ptr [rbp + 0x50], 2
0x1400bd780 mov      qword ptr [rbp + 0x58], rbx
0x1400bd784 mov      qword ptr [rbp + 0x60], 1
0x1400bd78c mov      qword ptr [rbp + 0x68], 0
0x1400bd794 lea      rbx, [rbp - 0x50]
0x1400bd798 lea      rdx, [rbp + 0x48]
0x1400bd79c mov      rcx, rbx
0x1400bd79f call     0x140d79f90
0x1400bd7a4 lea      rdx, [rip + 0x2877ca5] ; @0x142935450 'Open profiling tips'
0x1400bd7ab lea      r14, [rbp + 0x48]
0x1400bd7af mov      r8d, 0x13
0x1400bd7b5 mov      rcx, r14
0x1400bd7b8 mov      r9, rbx
0x1400bd7bb call     0x140056530
0x1400bd7c0 lea      rcx, [rbp + 0x250]
0x1400bd7c7 mov      rdx, r14
0x1400bd7ca mov      r8, rsi
0x1400bd7cd call     0x140c73730
0x1400bd7d2 mov      rax, qword ptr [rbp + 0x260]
0x1400bd7d9 lock dec qword ptr [rax]
0x1400bd7dd jne      0x1400bd7eb
0x1400bd7df lea      rcx, [rbp + 0x260]
0x1400bd7e6 call     0x140c41170
0x1400bd7eb mov      rax, qword ptr [r15]
0x1400bd7ee cmp      dword ptr [rax], 3
0x1400bd7f1 jne      0x1400bd8fa
0x1400bd7f7 add      rax, 4
0x1400bd7fb mov      qword ptr [rbp + 0x1f0], rax
0x1400bd802 mov      qword ptr [rbp - 0x50], rdi
0x1400bd806 lea      rax, [rip + 0x476143]
0x1400bd80d mov      qword ptr [rbp - 0x48], rax
0x1400bd811 lea      rax, [rip + 0x2877c50]
0x1400bd818 mov      qword ptr [rbp - 0x40], rax
0x1400bd81c lea      rax, [rip + 0xccfe9d]
0x1400bd823 mov      qword ptr [rbp - 0x38], rax
0x1400bd827 lea      rax, [rip + 0x2877c52]
0x1400bd82e mov      qword ptr [rbp + 0x250], rax
0x1400bd835 mov      qword ptr [rbp + 0x258], 2
0x1400bd840 mov      qword ptr [rbp + 0x260], rbx
0x1400bd847 mov      qword ptr [rbp + 0x268], 2
0x1400bd852 mov      qword ptr [rbp + 0x270], 0
0x1400bd85d lea      rcx, [rbp + 0x78]
0x1400bd861 lea      rdx, [rbp + 0x250]
0x1400bd868 call     0x140d79f90
0x1400bd86d mov      byte ptr [rbp + 0xa8], 0
0x1400bd874 mov      dword ptr [rbp + 0xad], 0
0x1400bd87e mov      dword ptr [rbp + 0xb0], 0
0x1400bd888 mov      qword ptr [rbp + 0x48], 3
0x1400bd890 mov      qword ptr [rbp + 0x60], 6
0x1400bd898 mov      dword ptr [rbp + 0x90], 0
0x1400bd8a2 mov      dword ptr [rbp + 0x98], 0
0x1400bd8ac mov      qword ptr [rbp + 0xa0], 0
0x1400bd8b7 mov      word ptr [rbp + 0xb8], 0x200
0x1400bd8c0 mov      byte ptr [rbp + 0xba], 2
0x1400bd8c7 mov      byte ptr [rbp + 0xbd], 2
0x1400bd8ce lea      rcx, [rbp + 0x250]
0x1400bd8d5 lea      rdx, [rbp + 0x48]
0x1400bd8d9 mov      r8, rsi
0x1400bd8dc call     0x140c28d40
0x1400bd8e1 mov      rax, qword ptr [rbp + 0x260]
0x1400bd8e8 lock dec qword ptr [rax]
0x1400bd8ec jne      0x1400bd8fa
0x1400bd8ee lea      rcx, [rbp + 0x260]
0x1400bd8f5 call     0x140c41170
0x1400bd8fa nop      
0x1400bd8fb add      rsp, 0x3b0
0x1400bd902 pop      rbx
0x1400bd903 pop      rdi
0x1400bd904 pop      rsi
0x1400bd905 pop      r12
0x1400bd907 pop      r14
0x1400bd909 pop      r15
0x1400bd90b pop      rbp
0x1400bd90c ret      
0x1400bd90d lea      r8, [rip + 0x2877434]
0x1400bd914 mov      ecx, 1
0x1400bd919 mov      edx, 0xb
0x1400bd91e call     0x1428d8fe3
0x1400bd923 lea      r8, [rip + 0x287741e]
0x1400bd92a mov      ecx, 1
0x1400bd92f mov      edx, 0xa
0x1400bd934 call     0x1428d8fe3
0x1400bd939 lea      r8, [rip + 0x2877408]
0x1400bd940 mov      ecx, 1
0x1400bd945 mov      edx, 0x13
0x1400bd94a call     0x1428d8fe3
0x1400bd94f mov      ecx, 8
0x1400bd954 mov      edx, 8
0x1400bd959 call     0x1428d9000
0x1400bd95e jmp      0x1400bd991
0x1400bd960 mov      ecx, 8
0x1400bd965 mov      edx, 0xa8
0x1400bd96a call     0x1428d9000
0x1400bd96f jmp      0x1400bd991
0x1400bd971 mov      ecx, 8
0x1400bd976 mov      edx, 8
0x1400bd97b call     0x1428d9000
0x1400bd980 jmp      0x1400bd991
0x1400bd982 mov      ecx, 8