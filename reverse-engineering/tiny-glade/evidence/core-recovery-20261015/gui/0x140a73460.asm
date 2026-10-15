0x140a73460 push     rbp
0x140a73461 push     r15
0x140a73463 push     r14
0x140a73465 push     r13
0x140a73467 push     r12
0x140a73469 push     rsi
0x140a7346a push     rdi
0x140a7346b push     rbx
0x140a7346c sub      rsp, 0x1f8
0x140a73473 lea      rbp, [rsp + 0x80]
0x140a7347b movaps   xmmword ptr [rbp + 0x160], xmm15
0x140a73483 movapd   xmmword ptr [rbp + 0x150], xmm14
0x140a7348c movaps   xmmword ptr [rbp + 0x140], xmm13
0x140a73494 movapd   xmmword ptr [rbp + 0x130], xmm12
0x140a7349d movapd   xmmword ptr [rbp + 0x120], xmm11
0x140a734a6 movapd   xmmword ptr [rbp + 0x110], xmm10
0x140a734af movaps   xmmword ptr [rbp + 0x100], xmm9
0x140a734b7 movaps   xmmword ptr [rbp + 0xf0], xmm8
0x140a734bf movaps   xmmword ptr [rbp + 0xe0], xmm7
0x140a734c6 movaps   xmmword ptr [rbp + 0xd0], xmm6
0x140a734cd mov      qword ptr [rbp + 0xc8], 0xfffffffffffffffe
0x140a734d8 mov      qword ptr [rbp + 0xb8], r9
0x140a734df mov      qword ptr [rbp - 8], r8
0x140a734e3 mov      rdi, rdx
0x140a734e6 mov      r14, rcx
0x140a734e9 call     0x140c16a00
0x140a734ee test     al, al
0x140a734f0 mov      qword ptr [rbp], rdi
0x140a734f4 je       0x140a73556
0x140a734f6 lea      r12, [rip + 0x2062af3] ; @0x142ad5ff0 'country_core::systems::time_of_day::time_of_day::f'
0x140a734fd mov      r15d, 0x2f
0x140a73503 mov      edx, 0x2f
0x140a73508 mov      rcx, r12
0x140a7350b call     0x140a719d0
0x140a73510 cmp      rax, 1
0x140a73514 jne      0x140a735cc
0x140a7351a mov      r9, rdx
0x140a7351d test     rdx, rdx
0x140a73520 je       0x140a7355f
0x140a73522 cmp      r9, 0x2f
0x140a73526 jae      0x140a7355d
0x140a73528 lea      rax, [rip + 0x2062ac1] ; @0x142ad5ff0 'country_core::systems::time_of_day::time_of_day::f'
0x140a7352f cmp      byte ptr [r9 + rax], 0xbf
0x140a73534 jg       0x140a7355f
0x140a73536 lea      rax, [rip + 0x2062b6b]
0x140a7353d mov      qword ptr [rsp + 0x20], rax
0x140a73542 lea      rcx, [rip + 0x2062aa7] ; @0x142ad5ff0 'country_core::systems::time_of_day::time_of_day::f'
0x140a73549 mov      edx, 0x2f
0x140a7354e xor      r8d, r8d
0x140a73551 call     0x1428d9940
0x140a73556 xor      eax, eax
0x140a73558 jmp      0x140a7372a
0x140a7355d jne      0x140a73536
0x140a7355f lea      r12, [rip + 0x2062a8a] ; @0x142ad5ff0 'country_core::systems::time_of_day::time_of_day::f'
0x140a73566 mov      rcx, r12
0x140a73569 mov      rdx, r9
0x140a7356c call     0x140a719d0
0x140a73571 cmp      rax, 1
0x140a73575 jne      0x140a735cc
0x140a73577 mov      r8, rdx
0x140a7357a add      r8, 2
0x140a7357e je       0x140a735b9
0x140a73580 cmp      r8, 0x2f
0x140a73584 jae      0x140a735b7
0x140a73586 lea      rax, [rip + 0x2062a63] ; @0x142ad5ff0 'country_core::systems::time_of_day::time_of_day::f'
0x140a7358d cmp      byte ptr [r8 + rax], 0xbf
0x140a73592 jg       0x140a735b9
0x140a73594 lea      rax, [rip + 0x2062b25]
0x140a7359b mov      qword ptr [rsp + 0x20], rax
0x140a735a0 lea      rcx, [rip + 0x2062a49] ; @0x142ad5ff0 'country_core::systems::time_of_day::time_of_day::f'
0x140a735a7 mov      edx, 0x2f
0x140a735ac mov      r9d, 0x2f
0x140a735b2 call     0x1428d9940
0x140a735b7 jne      0x140a73594
0x140a735b9 mov      r15d, 0x2d
0x140a735bf sub      r15, rdx
0x140a735c2 lea      r12, [rip + 0x2062a27] ; @0x142ad5ff0 'country_core::systems::time_of_day::time_of_day::f'
0x140a735c9 add      r12, r8
0x140a735cc lea      rax, [rip + 0x2062e1c] ; @0x142ad63ef 'TOD-EnabledTimetravelDayNight'
0x140a735d3 lea      r13, [rip + 0x2062de7] ; @0x142ad63c1 'crates/country-core/src/systems/time_of_day.rsTOD-EnabledTimetravelDayNight'
0x140a735da nop      word ptr [rax + rax]
0x140a735e0 movsx    ecx, byte ptr [rax - 1]
0x140a735e4 test     ecx, ecx
0x140a735e6 js       0x140a73600
0x140a735e8 dec      rax
0x140a735eb cmp      ecx, 0x5c
0x140a735ee jne      0x140a73659
0x140a735f0 jmp      0x140a7366e
0x140a735f2 nop      word ptr cs:[rax + rax]
0x140a73600 movzx    edx, byte ptr [rax - 2]
0x140a73604 cmp      dl, 0xc0
0x140a73607 jge      0x140a7362e
0x140a73609 movzx    r8d, byte ptr [rax - 3]
0x140a7360e cmp      r8b, 0xc0
0x140a73612 jge      0x140a73637
0x140a73614 movzx    r9d, byte ptr [rax - 4]
0x140a73619 add      rax, -4
0x140a7361d and      r9d, 7
0x140a73621 shl      r9d, 6
0x140a73625 and      r8d, 0x3f
0x140a73629 or       r8d, r9d
0x140a7362c jmp      0x140a7363f
0x140a7362e add      rax, -2
0x140a73632 and      edx, 0x1f
0x140a73635 jmp      0x140a73649
0x140a73637 add      rax, -3
0x140a7363b and      r8d, 0xf
0x140a7363f shl      r8d, 6
0x140a73643 and      edx, 0x3f
0x140a73646 or       edx, r8d
0x140a73649 shl      edx, 6
0x140a7364c and      cl, 0x3f
0x140a7364f movzx    ecx, cl
0x140a73652 or       ecx, edx
0x140a73654 cmp      ecx, 0x5c
0x140a73657 je       0x140a7366e
0x140a73659 cmp      ecx, 0x2f
0x140a7365c je       0x140a7366e
0x140a7365e cmp      rax, r13
0x140a73661 jne      0x140a735e0
0x140a73667 mov      esi, 0x2e
0x140a7366c jmp      0x140a736bd
0x140a7366e lea      r13, [rip + 0x2062d4c] ; @0x142ad63c1 'crates/country-core/src/systems/time_of_day.rsTOD-EnabledTimetravelDayNight'
0x140a73675 sub      rax, r13
0x140a73678 mov      r8, rax
0x140a7367b inc      r8
0x140a7367e je       0x140a736b2
0x140a73680 cmp      r8, 0x2e
0x140a73684 jae      0x140a736b0
0x140a73686 cmp      byte ptr [r8 + r13], 0xbf
0x140a7368b jg       0x140a736b2
0x140a7368d lea      rax, [rip + 0x20629f4]
0x140a73694 mov      qword ptr [rsp + 0x20], rax
0x140a73699 lea      rcx, [rip + 0x2062d21] ; @0x142ad63c1 'crates/country-core/src/systems/time_of_day.rsTOD-EnabledTimetravelDayNight'
0x140a736a0 mov      edx, 0x2e
0x140a736a5 mov      r9d, 0x2e
0x140a736ab call     0x1428d9940
0x140a736b0 jne      0x140a7368d
0x140a736b2 mov      esi, 0x2d
0x140a736b7 sub      rsi, rax
0x140a736ba add      r13, r8
0x140a736bd call     0x140c18130
0x140a736c2 mov      rcx, rax
0x140a736c5 mov      rax, qword ptr [rax]
0x140a736c8 cmp      rax, 1
0x140a736cc jne      0x140a74466
0x140a736d2 add      rcx, 8
0x140a736d6 cmp      qword ptr [rcx], 0
0x140a736da jne      0x140a7448a
0x140a736e0 mov      qword ptr [rcx], 0xffffffffffffffff
0x140a736e7 mov      qword ptr [rbp + 0xa0], rcx
0x140a736ee add      rcx, 8
0x140a736f2 mov      qword ptr [rsp + 0x20], rsi
0x140a736f7 mov      qword ptr [rsp + 0x30], 0
0x140a73700 mov      qword ptr [rsp + 0x28], 1
0x140a73709 mov      rdx, r12
0x140a7370c mov      r8, r15
0x140a7370f mov      r9, r13
0x140a73712 call     0x140c16d40
0x140a73717 mov      rcx, qword ptr [rbp + 0xa0]
0x140a7371e inc      qword ptr [rcx]
0x140a73721 mov      qword ptr [rbp + 0x48], rax
0x140a73725 mov      eax, 1
0x140a7372a mov      rdi, qword ptr [rbp + 0x218]
0x140a73731 mov      rcx, qword ptr [rbp + 0x200]
0x140a73738 mov      qword ptr [rbp + 0x40], rax
0x140a7373c mov      r12, qword ptr [r14]
0x140a7373f mov      r14, qword ptr [rcx]
0x140a73742 mov      rcx, qword ptr [r12]
0x140a73746 test     rcx, rcx
0x140a73749 je       0x140a7375a
0x140a7374b cmp      byte ptr [r14 + 0xe3], 0
0x140a73753 jne      0x140a73763
0x140a73755 xorps    xmm3, xmm3
0x140a73758 jmp      0x140a7376b
0x140a7375a mov      rsi, qword ptr [rdi]
0x140a7375d lea      r15, [rsi + 0xb]
0x140a73761 jmp      0x140a737ba
0x140a73763 movss    xmm3, dword ptr [rip + 0x1eb2199]
0x140a7376b add      rcx, 0x10
0x140a7376f lea      rdx, [rip + 0x2062c79] ; @0x142ad63ef 'TOD-EnabledTimetravelDayNight'
0x140a73776 mov      r8d, 0xb
0x140a7377c call     0x140a53db0
0x140a73781 mov      rcx, qword ptr [r12]
0x140a73785 mov      rsi, qword ptr [rdi]
0x140a73788 lea      r15, [rsi + 0xb]
0x140a7378c test     rcx, rcx
0x140a7378f je       0x140a737ba
0x140a73791 cmp      byte ptr [r15], 0
0x140a73795 jne      0x140a7379c
0x140a73797 xorps    xmm3, xmm3
0x140a7379a jmp      0x140a737a4
0x140a7379c movss    xmm3, dword ptr [rip + 0x1eb2160]
0x140a737a4 add      rcx, 0x10
0x140a737a8 lea      rdx, [rip + 0x2062c4b] ; @0x142ad63fa 'TimetravelDayNight'
0x140a737af mov      r8d, 0xa
0x140a737b5 call     0x140a53db0
0x140a737ba mov      rbx, qword ptr [rbp + 0x1f0]
0x140a737c1 cmp      byte ptr [rsi + 0xc], 0
0x140a737c5 mov      qword ptr [rbp + 0xa0], r14
0x140a737cc mov      qword ptr [rbp + 0xb0], r15
0x140a737d3 je       0x140a7384e
0x140a737d5 movss    xmm6, dword ptr [rip + 0x1ec0e97]
0x140a737dd movaps   xmm7, xmm6
0x140a737e0 subss    xmm7, dword ptr [rsi + 4]
0x140a737e5 mov      r14, qword ptr [rbx]
0x140a737e8 movss    xmm0, dword ptr [r14 + 0xc]
0x140a737ee movaps   xmm1, xmm6
0x140a737f1 call     0x142923f10
0x140a737f6 addss    xmm6, xmm0
0x140a737fa xorps    xmm1, xmm1
0x140a737fd movaps   xmm8, xmm0
0x140a73801 cmpltss  xmm8, xmm1
0x140a73807 andps    xmm6, xmm8
0x140a7380b andnps   xmm8, xmm0
0x140a7380f orps     xmm8, xmm6
0x140a73813 mov      rax, qword ptr [rbp + 0xb8]
0x140a7381a mov      rax, qword ptr [rax]
0x140a7381d movss    xmm0, dword ptr [rax + 0x28]
0x140a73822 mulss    xmm0, dword ptr [rip + 0x20620be]
0x140a7382a call     0x142923ec0
0x140a7382f movaps   xmm1, xmm7
0x140a73832 subss    xmm1, xmm8
0x140a73837 movss    xmm2, dword ptr [rip + 0x1f7a375]
0x140a7383f ucomiss  xmm2, xmm1
0x140a73842 jbe      0x140a73874
0x140a73844 addss    xmm1, dword ptr [rip + 0x1ec0e28]
0x140a7384c jmp      0x140a73889
0x140a7384e cmp      byte ptr [rsi + 0xa], 0
0x140a73852 jne      0x140a738d2
0x140a73854 mov      rax, qword ptr [rbx]
0x140a73857 mov      ecx, dword ptr [rdi + 0x1c]
0x140a7385a mov      rdx, qword ptr [rdi + 0x10]
0x140a7385e movss    xmm0, dword ptr [rip + 0x1ec0e0e]
0x140a73866 subss    xmm0, dword ptr [rax + 0xc]
0x140a7386b mov      dword ptr [rdx], ecx
0x140a7386d movss    dword ptr [rsi + 4], xmm0
0x140a73872 jmp      0x140a738d2
0x140a73874 movss    xmm2, dword ptr [rip + 0x1f7a328]
0x140a7387c ucomiss  xmm2, xmm1
0x140a7387f ja       0x140a73889
0x140a73881 addss    xmm1, dword ptr [rip + 0x2062063]
0x140a73889 mov      eax, dword ptr [rbx + 0x1c]
0x140a7388c mov      rcx, qword ptr [rbx + 0x10]
0x140a73890 mov      dword ptr [rcx], eax
0x140a73892 movss    xmm2, dword ptr [rip + 0x1eb206a]
0x140a7389a subss    xmm2, xmm0
0x140a7389e mulss    xmm2, xmm1
0x140a738a2 addss    xmm2, xmm8
0x140a738a7 movss    dword ptr [r14 + 0xc], xmm2
0x140a738ad subss    xmm2, xmm7
0x140a738b1 andps    xmm2, xmmword ptr [rip + 0x1ebb0c8]
0x140a738b8 movss    xmm0, dword ptr [rip + 0x1eecdb4]
0x140a738c0 ucomiss  xmm0, xmm2
0x140a738c3 jbe      0x140a738d2
0x140a738c5 mov      eax, dword ptr [rdi + 0x1c]
0x140a738c8 mov      rcx, qword ptr [rdi + 0x10]
0x140a738cc mov      dword ptr [rcx], eax
0x140a738ce mov      byte ptr [rsi + 0xc], 0
0x140a738d2 mov      r15, qword ptr [rbp + 0x210]
0x140a738d9 mov      r13, qword ptr [rbp + 0x208]
0x140a738e0 cmp      byte ptr [rsi + 0xa], 0
0x140a738e4 je       0x140a73940
0x140a738e6 mov      rax, qword ptr [rbp + 0xb0]
0x140a738ed cmp      byte ptr [rax], 0
0x140a738f0 je       0x140a7395b
0x140a738f2 mov      r14, qword ptr [rbx]
0x140a738f5 movss    xmm6, dword ptr [r14 + 8]
0x140a738fb movss    xmm8, dword ptr [rsi]
0x140a73900 mov      rax, qword ptr [rbp + 0xb8]
0x140a73907 mov      rax, qword ptr [rax]
0x140a7390a movss    xmm0, dword ptr [rax + 0x28]
0x140a7390f xorps    xmm0, xmmword ptr [rip + 0x1eb1fda]
0x140a73916 call     0x142923ec0
0x140a7391b subss    xmm8, xmm6
0x140a73920 movss    xmm1, dword ptr [rip + 0x1ee67c0]
0x140a73928 ucomiss  xmm1, xmm8
0x140a7392c jbe      0x140a739eb
0x140a73932 addss    xmm8, dword ptr [rip + 0x1eb1fc9]
0x140a7393b jmp      0x140a73a02
0x140a73940 mov      r14, qword ptr [rbx]
0x140a73943 movss    xmm0, dword ptr [r14 + 8]
0x140a73949 mov      eax, dword ptr [rdi + 0x1c]
0x140a7394c mov      rcx, qword ptr [rdi + 0x10]
0x140a73950 mov      dword ptr [rcx], eax
0x140a73952 movss    dword ptr [rsi], xmm0
0x140a73956 jmp      0x140a73aaa
0x140a7395b movss    xmm0, dword ptr [rsi]
0x140a7395f movss    xmm6, dword ptr [rip + 0x1eb1f9d]
0x140a73967 movaps   xmm1, xmm6
0x140a7396a call     0x142923f10
0x140a7396f movaps   xmm1, xmm0
0x140a73972 addss    xmm1, xmm6
0x140a73976 xorps    xmm2, xmm2
0x140a73979 movaps   xmm7, xmm0
0x140a7397c cmpltss  xmm7, xmm2
0x140a73981 andps    xmm1, xmm7
0x140a73984 andnps   xmm7, xmm0
0x140a73987 orps     xmm7, xmm1
0x140a7398a mov      r14, qword ptr [rbx]
0x140a7398d mov      rax, qword ptr [rbp + 0xb8]
0x140a73994 mov      rax, qword ptr [rax]
0x140a73997 movss    xmm1, dword ptr [rax + 0x28]
0x140a7399c mulss    xmm1, dword ptr [rip + 0x2061f44]
0x140a739a4 movss    xmm8, dword ptr [r14 + 8]
0x140a739aa mov      rax, qword ptr [rbp + 0xa0]
0x140a739b1 movss    xmm0, dword ptr [rax + 0x118]
0x140a739b9 maxss    xmm0, xmm6
0x140a739bd mulss    xmm0, xmm1
0x140a739c1 call     0x142923ec0
0x140a739c6 subss    xmm8, xmm7
0x140a739cb movss    xmm1, dword ptr [rip + 0x1ee6715]
0x140a739d3 ucomiss  xmm1, xmm8
0x140a739d7 jbe      0x140a73a79
0x140a739dd addss    xmm8, dword ptr [rip + 0x1eb1f1e]
0x140a739e6 jmp      0x140a73a90
0x140a739eb movss    xmm1, dword ptr [rip + 0x1eb1e7d]
0x140a739f3 ucomiss  xmm1, xmm8
0x140a739f7 ja       0x140a73a02
0x140a739f9 addss    xmm8, dword ptr [rip + 0x1ee66e2]
0x140a73a02 movss    xmm7, dword ptr [rip + 0x1eb1efa]
0x140a73a0a movaps   xmm1, xmm7
0x140a73a0d subss    xmm1, xmm0
0x140a73a11 mulss    xmm1, xmm8
0x140a73a16 addss    xmm6, xmm1
0x140a73a1a movaps   xmm0, xmm6
0x140a73a1d movaps   xmm1, xmm7
0x140a73a20 call     0x142923f10
0x140a73a25 addss    xmm7, xmm0
0x140a73a29 xorps    xmm1, xmm1
0x140a73a2c movaps   xmm2, xmm0
0x140a73a2f cmpltss  xmm2, xmm1
0x140a73a34 andps    xmm7, xmm2
0x140a73a37 andnps   xmm2, xmm0
0x140a73a3a orps     xmm2, xmm7
0x140a73a3d mov      eax, dword ptr [rbx + 0x1c]
0x140a73a40 mov      rcx, qword ptr [rbx + 0x10]
0x140a73a44 mov      dword ptr [rcx], eax
0x140a73a46 movss    dword ptr [r14 + 8], xmm2
0x140a73a4c subss    xmm2, dword ptr [rsi]
0x140a73a50 andps    xmm2, xmmword ptr [rip + 0x1ebaf29]
0x140a73a57 movss    xmm0, dword ptr [rip + 0x1f59601]
0x140a73a5f ucomiss  xmm0, xmm2
0x140a73a62 jbe      0x140a73aaa
0x140a73a64 mov      eax, dword ptr [rdi + 0x1c]
0x140a73a67 mov      rcx, qword ptr [rdi + 0x10]
0x140a73a6b mov      dword ptr [rcx], eax
0x140a73a6d mov      rax, qword ptr [rbp + 0xb0]
0x140a73a74 mov      byte ptr [rax], 0
0x140a73a77 jmp      0x140a73aaa
0x140a73a79 movss    xmm1, dword ptr [rip + 0x1eb1def]
0x140a73a81 ucomiss  xmm1, xmm8
0x140a73a85 ja       0x140a73a90
0x140a73a87 addss    xmm8, dword ptr [rip + 0x1ee6654]
0x140a73a90 mov      eax, dword ptr [rdi + 0x1c]
0x140a73a93 mov      rcx, qword ptr [rdi + 0x10]
0x140a73a97 mov      dword ptr [rcx], eax
0x140a73a99 subss    xmm6, xmm0
0x140a73a9d mulss    xmm6, xmm8
0x140a73aa2 addss    xmm6, xmm7
0x140a73aa6 movss    dword ptr [rsi], xmm6
0x140a73aaa mov      rcx, qword ptr [r13]
0x140a73aae mov      rdx, qword ptr [r13 + 8]
0x140a73ab2 mov      r8, qword ptr [rcx]
0x140a73ab5 xor      eax, eax
0x140a73ab7 mov      r9, r8
0x140a73aba sub      r9, qword ptr [rdx + 0x18]
0x140a73abe cmovb    r9, rax
0x140a73ac2 sub      r8, qword ptr [rdx + 0x38]
0x140a73ac6 cmovb    r8, rax
0x140a73aca mov      r10, qword ptr [rdx + 0x30]
0x140a73ace sub      r10, r8
0x140a73ad1 cmovb    r10, rax
0x140a73ad5 setbe    r8b
0x140a73ad9 mov      r11, qword ptr [rdx + 0x10]
0x140a73add sub      r11, r9
0x140a73ae0 cmovb    r11, rax
0x140a73ae4 mov      rdx, qword ptr [rdx + 0x40]
0x140a73ae8 setbe    dil
0x140a73aec add      r11, r10
0x140a73aef and      dil, r8b
0x140a73af2 cmove    r11, rax
0x140a73af6 sub      rdx, r11
0x140a73af9 mov      qword ptr [rcx], rdx
0x140a73afc mov      rcx, qword ptr [r15]
0x140a73aff mov      rdx, qword ptr [r15 + 8]
0x140a73b03 mov      r8, qword ptr [rcx]
0x140a73b06 mov      r9, r8
0x140a73b09 sub      r9, qword ptr [rdx + 0x18]
0x140a73b0d cmovb    r9, rax
0x140a73b11 sub      r8, qword ptr [rdx + 0x38]
0x140a73b15 cmovb    r8, rax
0x140a73b19 mov      r10, qword ptr [rdx + 0x30]
0x140a73b1d sub      r10, r8
0x140a73b20 cmovb    r10, rax
0x140a73b24 setbe    r8b
0x140a73b28 mov      r11, qword ptr [rdx + 0x10]
0x140a73b2c sub      r11, r9
0x140a73b2f cmovb    r11, rax
0x140a73b33 mov      rdx, qword ptr [rdx + 0x40]
0x140a73b37 setbe    r9b
0x140a73b3b add      r11, r10
0x140a73b3e and      r9b, r8b
0x140a73b41 mov      byte ptr [rbp + 0xc7], r9b
0x140a73b48 cmove    r11, rax
0x140a73b4c sub      rdx, r11
0x140a73b4f mov      qword ptr [rcx], rdx
0x140a73b52 cmp      byte ptr [rip + 0x2bc117f], 1
0x140a73b59 mov      r15, qword ptr [rbp + 0xa0]
0x140a73b60 jne      0x140a73b9d
0x140a73b62 mov      rax, qword ptr [rbp + 0x1e0]
0x140a73b69 mov      rcx, qword ptr [rax]
0x140a73b6c mov      dword ptr [rbp + 0x50], 0xa9
0x140a73b73 add      rcx, 0x20
0x140a73b77 lea      rdx, [rbp + 0x50]
0x140a73b7b call     0x14089aef0
0x140a73b80 test     rax, rax
0x140a73b83 je       0x140a73b9d
0x140a73b85 mov      rcx, qword ptr [rbp + 0x200]
0x140a73b8c mov      eax, dword ptr [rcx + 0x1c]
0x140a73b8f mov      rcx, qword ptr [rcx + 0x10]
0x140a73b93 mov      dword ptr [rcx], eax
0x140a73b95 xor      byte ptr [r15 + 0xe3], 1
0x140a73b9d mov      ecx, dword ptr [r14]
0x140a73ba0 cmp      byte ptr [r15 + 0xe3], 0
0x140a73ba8 je       0x140a73bdf
0x140a73baa mov      eax, 1
0x140a73baf test     ecx, ecx
0x140a73bb1 je       0x140a73c13
0x140a73bb3 cmp      ecx, 1
0x140a73bb6 jne      0x140a73c18
0x140a73bb8 mov      rcx, qword ptr [rbp + 0xb8]
0x140a73bbf mov      rcx, qword ptr [rcx]
0x140a73bc2 movss    xmm6, dword ptr [rcx + 0x28]
0x140a73bc7 divss    xmm6, dword ptr [rip + 0x1f594a9]
0x140a73bcf addss    xmm6, dword ptr [r14 + 4]
0x140a73bd5 minss    xmm6, dword ptr [rip + 0x1eb1d27]
0x140a73bdd jmp      0x140a73c1e
0x140a73bdf mov      eax, 2
0x140a73be4 test     ecx, ecx
0x140a73be6 je       0x140a73c13
0x140a73be8 cmp      ecx, 2
0x140a73beb jne      0x140a73c18
0x140a73bed mov      rcx, qword ptr [rbp + 0xb8]
0x140a73bf4 mov      rcx, qword ptr [rcx]
0x140a73bf7 movss    xmm6, dword ptr [rcx + 0x28]
0x140a73bfc divss    xmm6, dword ptr [rip + 0x2007a4c]
0x140a73c04 addss    xmm6, dword ptr [r14 + 4]
0x140a73c0a xorps    xmm0, xmm0
0x140a73c0d maxss    xmm6, xmm0
0x140a73c11 jmp      0x140a73c1e
0x140a73c13 xorps    xmm6, xmm6
0x140a73c16 jmp      0x140a73c1e
0x140a73c18 movss    xmm6, dword ptr [r14 + 4]
0x140a73c1e mov      r13, qword ptr [rbp + 0x220]
0x140a73c25 mov      r15d, dword ptr [rbx + 0x1c]
0x140a73c29 mov      rbx, qword ptr [rbx + 0x10]
0x140a73c2d mov      dword ptr [rbx], r15d
0x140a73c30 mov      dword ptr [r14], eax
0x140a73c33 movss    dword ptr [r14 + 4], xmm6
0x140a73c39 cmp      byte ptr [rsi + 0xa], 1
0x140a73c3d jne      0x140a73c53
0x140a73c3f mov      rax, qword ptr [rbp + 0xb0]
0x140a73c46 cmp      byte ptr [rax], 1
0x140a73c49 jne      0x140a73c53
0x140a73c4b movss    xmm0, dword ptr [r14 + 8]
0x140a73c51 jmp      0x140a73cc9
0x140a73c53 mov      rax, qword ptr [rbp + 0xb8]
0x140a73c5a mov      rax, qword ptr [rax]
0x140a73c5d movss    xmm7, dword ptr [rax + 0x28]
0x140a73c62 movss    xmm0, dword ptr [rip + 0x2bc111a]
0x140a73c6a mulss    xmm7, dword ptr [rip + 0x2bb8446]
0x140a73c72 call     0x142923eb0
0x140a73c77 mulss    xmm6, xmm7
0x140a73c7b mulss    xmm0, xmm6
0x140a73c7f mov      rax, qword ptr [rbp + 0xa0]
0x140a73c86 mulss    xmm0, dword ptr [rax + 0x118]
0x140a73c8e addss    xmm0, dword ptr [r14 + 8]
0x140a73c94 movss    xmm6, dword ptr [rip + 0x1eb1c68]
0x140a73c9c addss    xmm0, xmm6
0x140a73ca0 movaps   xmm1, xmm6
0x140a73ca3 call     0x142923f10
0x140a73ca8 movaps   xmm1, xmm0
0x140a73cab addss    xmm6, xmm0
0x140a73caf xorps    xmm2, xmm2
0x140a73cb2 cmpltss  xmm0, xmm2
0x140a73cb7 andps    xmm6, xmm0
0x140a73cba andnps   xmm0, xmm1
0x140a73cbd orps     xmm0, xmm6
0x140a73cc0 mov      dword ptr [rbx], r15d
0x140a73cc3 movss    dword ptr [r14 + 8], xmm0
0x140a73cc9 movss    xmm1, dword ptr [rip + 0x1ec092f]
0x140a73cd1 mulss    xmm0, xmm1
0x140a73cd5 addss    xmm0, dword ptr [rip + 0x1ec0927]
0x140a73cdd call     0x142923f10
0x140a73ce2 movaps   xmm6, xmm0
0x140a73ce5 mov      rax, qword ptr [r13]
0x140a73ce9 cmp      byte ptr [rax + 0x58], 4
0x140a73ced sete     dl
0x140a73cf0 call     0x140a74790
0x140a73cf5 mov      rcx, qword ptr [r12]
0x140a73cf9 test     rcx, rcx
0x140a73cfc je       0x140a73d17
0x140a73cfe add      rcx, 0x10
0x140a73d02 lea      rdx, [rip + 0x20626fb] ; @0x142ad6404 'DayNight'
0x140a73d09 mov      r8d, 8
0x140a73d0f movaps   xmm3, xmm0
0x140a73d12 call     0x140a53db0
0x140a73d17 mov      ecx, dword ptr [r14 + 0x28]
0x140a73d1b movzx    eax, cx
0x140a73d1e dec      ecx
0x140a73d20 movzx    ecx, cx
0x140a73d23 cmp      ecx, 0x16d
0x140a73d29 jae      0x140a74496
0x140a73d2f or       eax, 0xfce00
0x140a73d34 mov      dword ptr [rbp + 0x50], eax
0x140a73d37 mov      qword ptr [rbp + 0x54], 0
0x140a73d3f mov      dword ptr [rbp + 0x5b], 0
0x140a73d46 lea      rcx, [rbp + 0x50]
0x140a73d4a call     0x140caff50
0x140a73d4f imul     rax, rax, 0x3e8
0x140a73d56 mulss    xmm6, dword ptr [rip + 0x2061b92]
0x140a73d5e cvttss2si rcx, xmm6
0x140a73d63 ucomiss  xmm6, dword ptr [rip + 0x2061b8a]
0x140a73d6a movabs   r8, 0x7fffffffffffffff
0x140a73d74 cmovbe   r8, rcx
0x140a73d78 xor      edx, edx
0x140a73d7a ucomiss  xmm6, xmm6
0x140a73d7d cmovnp   rdx, r8
0x140a73d81 add      rdx, rax
0x140a73d84 movss    xmm0, dword ptr [r14 + 0x10]
0x140a73d8a xorps    xmm2, xmm2
0x140a73d8d cvtss2sd xmm2, xmm0
0x140a73d91 lea      rcx, [rbp + 0x50]
0x140a73d95 xorps    xmm3, xmm3
0x140a73d98 call     0x1417ab4b0
0x140a73d9d movsd    xmm0, qword ptr [rbp + 0x70]
0x140a73da2 movsd    xmm6, qword ptr [rbp + 0x78]
0x140a73da7 addsd    xmm0, qword ptr [rip + 0x2061b49]
0x140a73daf movsd    xmm7, qword ptr [rip + 0x2061b49]
0x140a73db7 subsd    xmm7, xmm0
0x140a73dbb movapd   xmm0, xmm7
0x140a73dbf call     0x142923e70
0x140a73dc4 movapd   xmm10, xmm0
0x140a73dc9 movapd   xmm0, xmm6
0x140a73dcd call     0x142923e70
0x140a73dd2 movapd   xmm11, xmm0
0x140a73dd7 movapd   xmm0, xmm6
0x140a73ddb call     0x142923fc0
0x140a73de0 movapd   xmm12, xmm0
0x140a73de5 movapd   xmm0, xmm7
0x140a73de9 call     0x142923fc0
0x140a73dee movapd   xmm14, xmm0
0x140a73df3 movss    xmm6, dword ptr [r14 + 8]
0x140a73df9 movss    xmm8, dword ptr [r14 + 0xc]
0x140a73dff movss    xmm7, dword ptr [rip + 0x1ec086d]
0x140a73e07 mulss    xmm7, xmm6
0x140a73e0b addss    xmm7, dword ptr [rip + 0x2061af5]
0x140a73e13 movaps   xmm0, xmm7
0x140a73e16 call     0x142923fd0
0x140a73e1b movaps   xmm13, xmm0
0x140a73e1f movaps   xmm0, xmm7
0x140a73e22 call     0x142923e80
0x140a73e27 movaps   xmm9, xmm0
0x140a73e2b mulss    xmm8, dword ptr [rip + 0x1eb1a3c]
0x140a73e34 movaps   xmm0, xmm8
0x140a73e38 call     0x142923fd0
0x140a73e3d movaps   xmmword ptr [rbp + 0xa0], xmm0
0x140a73e44 movaps   xmm0, xmm8
0x140a73e48 call     0x142923e80
0x140a73e4d movaps   xmm8, xmm0
0x140a73e51 mov      rax, qword ptr [rbp]
0x140a73e55 mov      rdx, qword ptr [rax]
0x140a73e58 lea      rcx, [rbp + 0x50]
0x140a73e5c movaps   xmm2, xmm6
0x140a73e5f call     0x140b067d0
0x140a73e64 xorps    xmm7, xmm7
0x140a73e67 movss    xmm0, dword ptr [rbp + 0x50]
0x140a73e6c movaps   xmmword ptr [rbp - 0x30], xmm0
0x140a73e70 movsd    xmm15, qword ptr [rbp + 0x54]
0x140a73e76 movsd    xmm0, qword ptr [rbp + 0x5c]
0x140a73e7b movaps   xmmword ptr [rbp - 0x40], xmm0
0x140a73e7f movss    xmm0, dword ptr [rbp + 0x64]
0x140a73e84 movaps   xmmword ptr [rbp], xmm0
0x140a73e88 mov      rax, qword ptr [rbp + 0x68]
0x140a73e8c mov      qword ptr [rbp + 0x20], rax
0x140a73e90 mov      eax, dword ptr [rbp + 0x70]
0x140a73e93 mov      dword ptr [rbp + 0x28], eax
0x140a73e96 mov      rax, qword ptr [rbp + 0x74]
0x140a73e9a mov      qword ptr [rbp + 0x10], rax
0x140a73e9e mov      eax, dword ptr [rbp + 0x7c]
0x140a73ea1 mov      dword ptr [rbp + 0x18], eax
0x140a73ea4 movss    xmm0, dword ptr [rbp + 0x80]
0x140a73eac movss    dword ptr [rbp + 0xb0], xmm0
0x140a73eb4 mov      rax, qword ptr [rbp + 0x84]
0x140a73ebb mov      qword ptr [rbp + 0x30], rax
0x140a73ebf mov      eax, dword ptr [rbp + 0x8c]
0x140a73ec5 mov      dword ptr [rbp + 0x38], eax
0x140a73ec8 movss    xmm0, dword ptr [rbp + 0x90]
0x140a73ed0 movss    dword ptr [rip + 0x2bc0eac], xmm0
0x140a73ed8 movss    xmm2, dword ptr [r14 + 8]
0x140a73ede movss    xmm3, dword ptr [r14 + 0x14]
0x140a73ee4 movss    xmm1, dword ptr [r14 + 0x18]
0x140a73eea movaps   xmm0, xmm2
0x140a73eed cmpltps  xmm0, xmm1
0x140a73ef1 movaps   xmm4, xmm3
0x140a73ef4 cmpleps  xmm4, xmm2
0x140a73ef8 andps    xmm4, xmm0
0x140a73efb movd     esi, xmm4
0x140a73eff xorps    xmm0, xmm0
0x140a73f02 test     sil, 1
0x140a73f06 je       0x140a73f10
0x140a73f08 movaps   xmm0, xmm3
0x140a73f0b call     0x140c904a0
0x140a73f10 and      sil, 1
0x140a73f14 movzx    eax, sil
0x140a73f18 xorps    xmm6, xmm6
0x140a73f1b cvtsi2ss xmm6, eax
0x140a73f1f and      dil, byte ptr [rbp + 0xc7]
0x140a73f26 movss    dword ptr [r14 + 0x24], xmm0
0x140a73f2c mov      dword ptr [rbx], r15d
0x140a73f2f test     dil, dil
0x140a73f32 je       0x140a73f5c
0x140a73f34 mov      rax, qword ptr [rbp + 0xb8]
0x140a73f3b mov      rax, qword ptr [rax]
0x140a73f3e lea      rcx, [r14 + 0x20]
0x140a73f42 movss    xmm2, dword ptr [rax + 0x28]
0x140a73f47 movss    xmm3, dword ptr [rip + 0x1eb19b5]
0x140a73f4f movaps   xmm1, xmm6
0x140a73f52 call     0x140c90670
0x140a73f57 movaps   xmm6, xmm0
0x140a73f5a jmp      0x140a73f62
0x140a73f5c movss    dword ptr [r14 + 0x20], xmm6
0x140a73f62 mov      rax, qword ptr [rbp + 0x1e8]
0x140a73f69 mulsd    xmm10, xmm11
0x140a73f6e xorps    xmm0, xmm0
0x140a73f71 cvtsd2ss xmm0, xmm10
0x140a73f76 xorpd    xmm14, xmmword ptr [rip + 0x1eba9e1]
0x140a73f7f mulsd    xmm14, xmm11
0x140a73f84 unpcklpd xmm12, xmm14
0x140a73f89 cvtpd2ps xmm3, xmm12
0x140a73f8e movss    xmm10, dword ptr [rip + 0x1eb196d]
0x140a73f97 movaps   xmm1, xmm10
0x140a73f9b subss    xmm1, xmm9
0x140a73fa0 movsldup xmm11, xmm1
0x140a73fa5 mulss    xmm1, dword ptr [rip + 0x206195f]
0x140a73fad mulss    xmm7, xmm13
0x140a73fb2 movsldup xmm2, xmm13
0x140a73fb7 mulps    xmm2, xmmword ptr [rip + 0x2061952]
0x140a73fbe movsd    xmm4, qword ptr [rip + 0x200d49a]
0x140a73fc6 mulps    xmm4, xmm11
0x140a73fca mulps    xmm11, xmmword ptr [rip + 0x206194e]
0x140a73fd2 addss    xmm1, xmm9
0x140a73fd7 movaps   xmm12, xmm9
0x140a73fdb unpcklps xmm12, xmm7
0x140a73fdf addps    xmm12, xmm11
0x140a73fe3 movaps   xmm5, xmm4
0x140a73fe6 subps    xmm5, xmm2
0x140a73fe9 movshdup xmm11, xmm11
0x140a73fee subss    xmm11, xmm7
0x140a73ff3 unpcklps xmm11, xmm1
0x140a73ff7 movaps   xmm7, xmm4
0x140a73ffa addps    xmm7, xmm2
0x140a73ffd movaps   xmm1, xmm5
0x140a74000 shufps   xmm1, xmm7, 1
0x140a74004 shufps   xmm1, xmm7, 0xe2
0x140a74008 mulps    xmm12, xmmword ptr [rip + 0x2061920]
0x140a74010 mulss    xmm5, dword ptr [rip + 0x2061928]
0x140a74018 mulps    xmm11, xmmword ptr [rip + 0x2061930]
0x140a74020 addps    xmm11, xmm12
0x140a74024 xorps    xmm7, xmm7
0x140a74027 mulps    xmm1, xmm7
0x140a7402a addps    xmm1, xmm11
0x140a7402e movss    xmm2, xmm9
0x140a74033 addps    xmm2, xmm4
0x140a74036 mulps    xmm2, xmmword ptr [rip + 0x2061923]
0x140a7403d movshdup xmm4, xmm2
0x140a74041 addss    xmm4, xmm5
0x140a74045 addss    xmm4, xmm2
0x140a74049 movss    xmm5, dword ptr [rip + 0x206191f]
0x140a74051 mulss    xmm5, xmm1
0x140a74055 movss    xmm2, dword ptr [rip + 0x2061917]
0x140a7405d movaps   xmm11, xmm1
0x140a74061 mulss    xmm11, xmm2
0x140a74066 movshdup xmm9, xmm1
0x140a7406b mulps    xmm1, xmm7
0x140a7406e movshdup xmm12, xmm1
0x140a74073 subss    xmm12, xmm5
0x140a74078 movsldup xmm5, xmm11
0x140a7407d addps    xmm9, xmm1
0x140a74081 subps    xmm1, xmm5
0x140a74084 mulss    xmm2, xmm4
0x140a74088 movsldup xmm4, xmm4
0x140a7408c mulps    xmm4, xmmword ptr [rip + 0x20618ed]
0x140a74093 addss    xmm2, xmm12
0x140a74098 addps    xmm9, xmm4
0x140a7409c subps    xmm1, xmm4
0x140a7409f shufps   xmm1, xmm9, 0xe4
0x140a740a4 movsd    xmm4, qword ptr [rbp + 0xa0]
0x140a740ac movaps   xmm5, xmm4
0x140a740af shufps   xmm5, xmm7, 0xe2
0x140a740b3 movq     xmm7, xmm8
0x140a740b8 shufps   xmm4, xmm7, 0x22
0x140a740bc movlhps  xmm0, xmm3
0x140a740bf movaps   xmm8, xmm0
0x140a740c3 shufps   xmm8, xmm3, 0xd8
0x140a740c8 shufps   xmm0, xmm3, 0x58
0x140a740cc movaps   xmm3, xmm4
0x140a740cf mulps    xmm3, xmm4
0x140a740d2 movshdup xmm7, xmm3
0x140a740d6 addss    xmm7, xmm3
0x140a740da movaps   xmm11, xmm3
0x140a740de unpckhpd xmm11, xmm3
0x140a740e3 addss    xmm11, xmm7
0x140a740e8 shufps   xmm11, xmm11, 0
0x140a740ed shufps   xmm3, xmm3, 0xff
0x140a740f1 subps    xmm3, xmm11
0x140a740f5 mulps    xmm0, xmm3
0x140a740f8 movaps   xmm11, xmm5
0x140a740fc mulps    xmm11, xmm8
0x140a74100 movshdup xmm7, xmm11
0x140a74105 addss    xmm7, xmm11
0x140a7410a movhlps  xmm11, xmm11
0x140a7410e addss    xmm11, xmm7
0x140a74113 addss    xmm11, xmm11
0x140a74118 shufps   xmm11, xmm11, 0
0x140a7411d mulps    xmm11, xmm4
0x140a74121 addps    xmm11, xmm0
0x140a74125 movaps   xmm7, xmm5
0x140a74128 shufps   xmm7, xmm5, 0xd2
0x140a7412c movaps   xmm0, xmm7
0x140a7412f mulps    xmm0, xmm8
0x140a74133 shufps   xmm8, xmm8, 0xd2
0x140a74138 mulps    xmm8, xmm5
0x140a7413c subps    xmm0, xmm8
0x140a74140 shufps   xmm0, xmm0, 0xd2
0x140a74144 addps    xmm4, xmm4
0x140a74147 shufps   xmm4, xmm4, 0xff
0x140a7414b mulps    xmm0, xmm4
0x140a7414e addps    xmm0, xmm11
0x140a74152 movlhps  xmm2, xmm9
0x140a74156 shufps   xmm2, xmm1, 0xd8
0x140a7415a mulps    xmm3, xmm2
0x140a7415d movaps   xmm1, xmm5
0x140a74160 mulps    xmm1, xmm2
0x140a74163 movshdup xmm8, xmm1
0x140a74168 addss    xmm8, xmm1
0x140a7416d movhlps  xmm1, xmm1
0x140a74170 addss    xmm1, xmm8
0x140a74175 addss    xmm1, xmm1
0x140a74179 shufps   xmm1, xmm1, 0
0x140a7417d mulps    xmm1, xmm5
0x140a74180 addps    xmm1, xmm3
0x140a74183 mulps    xmm7, xmm2
0x140a74186 shufps   xmm2, xmm2, 0xd2
0x140a7418a mulps    xmm2, xmm5
0x140a7418d subps    xmm7, xmm2
0x140a74190 shufps   xmm7, xmm7, 0xd2
0x140a74194 mulps    xmm7, xmm4
0x140a74197 addps    xmm7, xmm1
0x140a7419a movshdup xmm1, xmm0
0x140a7419e addss    xmm1, dword ptr [rip + 0x1f79a5a]
0x140a741a6 movaps   xmm2, xmm0
0x140a741a9 movaps   xmm3, xmm1
0x140a741ac movlhps  xmm1, xmm0
0x140a741af shufps   xmm1, xmm0, 0xe2
0x140a741b3 movhlps  xmm0, xmm0
0x140a741b6 mulss    xmm2, xmm2
0x140a741ba mulss    xmm3, xmm3
0x140a741be addss    xmm3, xmm2
0x140a741c2 movaps   xmm2, xmm0
0x140a741c5 mulss    xmm2, xmm0
0x140a741c9 addss    xmm2, xmm3
0x140a741cd sqrtss   xmm2, xmm2
0x140a741d1 movaps   xmm3, xmm10
0x140a741d5 divss    xmm3, xmm2
0x140a741d9 movsldup xmm8, xmm3
0x140a741de mulps    xmm8, xmm1
0x140a741e2 mulss    xmm3, xmm0
0x140a741e6 shufps   xmm3, xmm8, 0x30
0x140a741eb shufps   xmm8, xmm3, 0x84
0x140a741f0 movss    xmm0, dword ptr [rip + 0x1eb1678]
0x140a741f8 ucomiss  xmm0, xmm6
0x140a741fb movss    xmm12, dword ptr [r14 + 0x1c]
0x140a74201 movaps   xmm9, xmm7
0x140a74205 unpckhpd xmm9, xmm7
0x140a7420a movaps   xmm11, xmm8
0x140a7420e unpckhpd xmm11, xmm8
0x140a74213 jbe      0x140a7428a
0x140a74215 mov      ecx, dword ptr [rax + 0x1c]
0x140a74218 mov      rdx, qword ptr [rax + 0x10]
0x140a7421c mov      dword ptr [rdx], ecx
0x140a7421e mov      rcx, qword ptr [rax]
0x140a74221 movss    dword ptr [rbp + 0x58], xmm11
0x140a74227 movlps   qword ptr [rbp + 0x50], xmm8
0x140a7422c lea      rdx, [rbp + 0x50]
0x140a74230 call     0x140b18830
0x140a74235 addss    xmm6, xmm6
0x140a74239 movaps   xmm0, xmm10
0x140a7423d subss    xmm0, xmm6
0x140a74241 movaps   xmm1, xmm10
0x140a74245 subss    xmm1, xmm12
0x140a7424a mulss    xmm1, xmm0
0x140a7424e movaps   xmm3, xmmword ptr [rbp - 0x30]
0x140a74252 movaps   xmm0, xmm3
0x140a74255 mulss    xmm0, xmm1
0x140a74259 subss    xmm10, xmm1
0x140a7425e shufps   xmm3, xmm15, 0x30
0x140a74263 movshdup xmm2, xmm15
0x140a74268 shufps   xmm15, xmm3, 0x84
0x140a7426d shufps   xmm15, xmm15, 0x24
0x140a74272 shufps   xmm1, xmm10, 0
0x140a74277 mulps    xmm1, xmm15
0x140a7427b mulss    xmm2, xmm10
0x140a74280 movaps   xmm4, xmmword ptr [rbp]
0x140a74284 movaps   xmm5, xmmword ptr [rbp - 0x40]
0x140a74288 jmp      0x140a74305
0x140a7428a mov      ecx, dword ptr [rax + 0x1c]
0x140a7428d mov      rdx, qword ptr [rax + 0x10]
0x140a74291 mov      dword ptr [rdx], ecx
0x140a74293 mov      rcx, qword ptr [rax]
0x140a74296 movss    dword ptr [rbp + 0x58], xmm9
0x140a7429c movlps   qword ptr [rbp + 0x50], xmm7
0x140a742a0 lea      rdx, [rbp + 0x50]
0x140a742a4 call     0x140b18830
0x140a742a9 addss    xmm6, dword ptr [rip + 0x1ee5e37]
0x140a742b1 addss    xmm6, xmm6
0x140a742b5 movaps   xmm2, xmm10
0x140a742b9 subss    xmm2, xmm12
0x140a742be mulss    xmm2, xmm6
0x140a742c2 movaps   xmm5, xmmword ptr [rbp - 0x40]
0x140a742c6 movaps   xmm0, xmm5
0x140a742c9 mulss    xmm0, xmm2
0x140a742cd movaps   xmm4, xmmword ptr [rbp]
0x140a742d1 movaps   xmm3, xmm4
0x140a742d4 shufps   xmm3, xmm5, 0xd4
0x140a742d8 shufps   xmm3, xmm3, 0xe2
0x140a742dc movsldup xmm1, xmm2
0x140a742e0 mulps    xmm1, xmm3
0x140a742e3 subss    xmm10, xmm2
0x140a742e8 movsldup xmm2, xmm10
0x140a742ed mulps    xmm5, xmm2
0x140a742f0 mulss    xmm4, xmm10
0x140a742f5 movshdup xmm2, xmm15
0x140a742fa unpcklpd xmm15, xmmword ptr [rbp - 0x30]
0x140a74300 shufps   xmm1, xmm15, 0x24
0x140a74305 mov      rax, qword ptr [rbp + 0x1f8]
0x140a7430c mov      rcx, qword ptr [rbp - 8]
0x140a74310 mov      rcx, qword ptr [rcx]
0x140a74313 mov      edx, dword ptr [rax + 0x1c]
0x140a74316 mov      r8, qword ptr [rax + 0x10]
0x140a7431a movss    xmm3, dword ptr [rbp + 0xb0]
0x140a74322 addss    xmm3, dword ptr [rcx]
0x140a74326 mov      dword ptr [r8], edx
0x140a74329 mov      rax, qword ptr [rax]
0x140a7432c movss    dword ptr [rax], xmm0
0x140a74330 movups   xmmword ptr [rax + 4], xmm1
0x140a74334 movss    dword ptr [rax + 0x14], xmm2
0x140a74339 movss    dword ptr [rax + 0x20], xmm11
0x140a7433f movlps   qword ptr [rax + 0x18], xmm8
0x140a74344 movlps   qword ptr [rax + 0x24], xmm5
0x140a74348 movss    dword ptr [rax + 0x2c], xmm4
0x140a7434d movss    dword ptr [rax + 0x38], xmm9
0x140a74353 movlps   qword ptr [rax + 0x30], xmm7
0x140a74357 mov      rcx, qword ptr [rbp + 0x20]
0x140a7435b mov      qword ptr [rax + 0x3c], rcx
0x140a7435f mov      ecx, dword ptr [rbp + 0x28]
0x140a74362 mov      dword ptr [rax + 0x44], ecx
0x140a74365 mov      rcx, qword ptr [rbp + 0x10]
0x140a74369 mov      qword ptr [rax + 0x48], rcx
0x140a7436d mov      ecx, dword ptr [rbp + 0x18]
0x140a74370 mov      dword ptr [rax + 0x50], ecx
0x140a74373 movss    dword ptr [rax + 0x54], xmm3
0x140a74378 mov      rcx, qword ptr [rbp + 0x30]
0x140a7437c mov      qword ptr [rax + 0x58], rcx
0x140a74380 mov      ecx, dword ptr [rbp + 0x38]
0x140a74383 mov      dword ptr [rax + 0x60], ecx
0x140a74386 movss    dword ptr [rax + 0x6c], xmm11
0x140a7438c movlps   qword ptr [rax + 0x64], xmm8
0x140a74391 movss    dword ptr [rax + 0x78], xmm9
0x140a74397 movlps   qword ptr [rax + 0x70], xmm7
0x140a7439b cmp      qword ptr [rbp + 0x40], 0
0x140a743a0 je       0x140a743ea
0x140a743a2 call     0x140c18130
0x140a743a7 mov      rcx, rax
0x140a743aa mov      rax, qword ptr [rax]
0x140a743ad cmp      rax, 1
0x140a743b1 jne      0x140a7444c
0x140a743b7 add      rcx, 8
0x140a743bb cmp      qword ptr [rcx], 0
0x140a743bf jne      0x140a7448a
0x140a743c5 mov      qword ptr [rcx], 0xffffffffffffffff
0x140a743cc mov      qword ptr [rbp + 0xb8], rcx
0x140a743d3 add      rcx, 8
0x140a743d7 mov      rdx, qword ptr [rbp + 0x48]
0x140a743db call     0x140c170f0
0x140a743e0 mov      rax, qword ptr [rbp + 0xb8]
0x140a743e7 inc      qword ptr [rax]
0x140a743ea movaps   xmm6, xmmword ptr [rbp + 0xd0]
0x140a743f1 movaps   xmm7, xmmword ptr [rbp + 0xe0]
0x140a743f8 movaps   xmm8, xmmword ptr [rbp + 0xf0]
0x140a74400 movaps   xmm9, xmmword ptr [rbp + 0x100]
0x140a74408 movaps   xmm10, xmmword ptr [rbp + 0x110]
0x140a74410 movaps   xmm11, xmmword ptr [rbp + 0x120]
0x140a74418 movaps   xmm12, xmmword ptr [rbp + 0x130]
0x140a74420 movaps   xmm13, xmmword ptr [rbp + 0x140]
0x140a74428 movaps   xmm14, xmmword ptr [rbp + 0x150]
0x140a74430 movaps   xmm15, xmmword ptr [rbp + 0x160]
0x140a74438 add      rsp, 0x1f8
0x140a7443f pop      rbx
0x140a74440 pop      rdi
0x140a74441 pop      rsi
0x140a74442 pop      r12
0x140a74444 pop      r13
0x140a74446 pop      r14
0x140a74448 pop      r15
0x140a7444a pop      rbp
0x140a7444b ret      
0x140a7444c test     rax, rax
0x140a7444f jne      0x140a7447e
0x140a74451 xor      edx, edx
0x140a74453 call     0x1428f87d0
0x140a74458 mov      rcx, rax
0x140a7445b test     rax, rax
0x140a7445e jne      0x140a743bb
0x140a74464 jmp      0x140a7447e
0x140a74466 test     rax, rax
0x140a74469 jne      0x140a7447e
0x140a7446b xor      edx, edx
0x140a7446d call     0x1428f87d0
0x140a74472 mov      rcx, rax
0x140a74475 test     rax, rax
0x140a74478 jne      0x140a736d6
0x140a7447e lea      rcx, [rip + 0x21c661b]
0x140a74485 call     0x1428d80d0
0x140a7448a lea      rcx, [rip + 0x21c66df]
0x140a74491 call     0x1428d9250
0x140a74496 lea      rcx, [rip + 0x2061a9b] ; @0x142ad5f38 'ordinal/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\\library\\alloc\\src\\slice.rs'
0x140a7449d mov      qword ptr [rbp + 0x50], rcx
0x140a744a1 mov      qword ptr [rbp + 0x58], 7
0x140a744a9 mov      qword ptr [rbp + 0x60], 1
0x140a744b1 mov      qword ptr [rbp + 0x68], 0x16d
0x140a744b9 mov      qword ptr [rbp + 0x70], rax
0x140a744bd mov      byte ptr [rbp + 0x78], 1
0x140a744c1 lea      rax, [rip + 0x2061f48]
0x140a744c8 mov      qword ptr [rsp + 0x20], rax
0x140a744cd lea      rcx, [rip + 0x2061a14] ; @0x142ad5ee8 'called `Result::unwrap()` on an `Err` value'
0x140a744d4 lea      r9, [rip + 0x2061a3d]