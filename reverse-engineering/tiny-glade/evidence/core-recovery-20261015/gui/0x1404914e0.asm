0x1404914e0 push     rbp
0x1404914e1 push     r15
0x1404914e3 push     r14
0x1404914e5 push     r13
0x1404914e7 push     r12
0x1404914e9 push     rsi
0x1404914ea push     rdi
0x1404914eb push     rbx
0x1404914ec sub      rsp, 0x258
0x1404914f3 lea      rbp, [rsp + 0x80]
0x1404914fb movaps   xmmword ptr [rbp + 0x1c0], xmm12
0x140491503 movaps   xmmword ptr [rbp + 0x1b0], xmm11
0x14049150b movaps   xmmword ptr [rbp + 0x1a0], xmm10
0x140491513 movaps   xmmword ptr [rbp + 0x190], xmm9
0x14049151b movaps   xmmword ptr [rbp + 0x180], xmm8
0x140491523 movaps   xmmword ptr [rbp + 0x170], xmm7
0x14049152a movaps   xmmword ptr [rbp + 0x160], xmm6
0x140491531 mov      qword ptr [rbp + 0x158], 0xfffffffffffffffe
0x14049153c mov      rsi, rdx
0x14049153f movups   xmm0, xmmword ptr [rcx]
0x140491542 mov      r15, qword ptr [rcx + 0x10]
0x140491546 mov      rax, qword ptr [rcx + 0x18]
0x14049154a mov      qword ptr [rbp + 0x18], rax
0x14049154e mov      r13, qword ptr [rcx + 0x20]
0x140491552 mov      rbx, qword ptr [rcx + 0x28]
0x140491556 movups   xmm12, xmmword ptr [rcx + 0x30]
0x14049155b mov      r14, qword ptr [rcx + 0x40]
0x14049155f mov      rdi, qword ptr [rcx + 0x48]
0x140491563 movups   xmm11, xmmword ptr [rcx + 0x50]
0x140491568 mov      r12, qword ptr [rcx + 0x60]
0x14049156c mov      rax, qword ptr [rcx + 0x68]
0x140491570 mov      qword ptr [rbp - 0x40], rax
0x140491574 mov      rax, qword ptr [rcx + 0x70]
0x140491578 mov      qword ptr [rbp - 0x30], rax
0x14049157c movups   xmm10, xmmword ptr [rcx + 0x78]
0x140491581 mov      rax, qword ptr [rcx + 0x88]
0x140491588 mov      qword ptr [rbp - 0x38], rax
0x14049158c movups   xmm8, xmmword ptr [rcx + 0x90]
0x140491594 mov      rax, qword ptr [rcx + 0xa0]
0x14049159b mov      qword ptr [rbp - 0x28], rax
0x14049159f movups   xmm9, xmmword ptr [rcx + 0xb0]
0x1404915a7 movups   xmm7, xmmword ptr [rcx + 0xd0]
0x1404915ae movups   xmm6, xmmword ptr [rcx + 0xf8]
0x1404915b5 mov      rax, qword ptr [rcx + 0xa8]
0x1404915bc mov      qword ptr [rbp - 0x48], rax
0x1404915c0 mov      rax, qword ptr [rcx + 0xc0]
0x1404915c7 mov      qword ptr [rbp - 0x20], rax
0x1404915cb mov      rax, qword ptr [rcx + 0xc8]
0x1404915d2 mov      qword ptr [rbp - 0x18], rax
0x1404915d6 mov      rax, qword ptr [rcx + 0xe0]
0x1404915dd mov      qword ptr [rbp - 0x10], rax
0x1404915e1 mov      rax, qword ptr [rcx + 0xe8]
0x1404915e8 mov      qword ptr [rbp - 8], rax
0x1404915ec mov      rax, qword ptr [rcx + 0xf0]
0x1404915f3 mov      qword ptr [rbp], rax
0x1404915f7 movups   xmmword ptr [rsp + 0x20], xmm0
0x1404915fc lea      r8, [rip + 0x254dae5] ; @0x1429df0e8 'ClutterFreecam Position SaveHistoryAudioLogoVisualsGfxVermeerLightingTime of dayWeatherColorCame'
0x140491603 lea      rcx, [rbp + 0x90]
0x14049160a mov      r9d, 7
0x140491610 call     0x1400c21e0
0x140491615 mov      rax, qword ptr [rbp + 0x100]
0x14049161c lock dec qword ptr [rax]
0x140491620 jne      0x14049162e
0x140491622 lea      rcx, [rbp + 0x100]
0x140491629 call     0x140c41170
0x14049162e cmp      dword ptr [rbp + 0xd0], 2
0x140491635 je       0x140491650
0x140491637 mov      rax, qword ptr [rbp + 0xa0]
0x14049163e lock dec qword ptr [rax]
0x140491642 jne      0x140491650
0x140491644 lea      rcx, [rbp + 0xa0]
0x14049164b call     0x140c41170
0x140491650 mov      rax, qword ptr [rbp + 0x18]
0x140491654 mov      qword ptr [rsp + 0x28], rax
0x140491659 mov      qword ptr [rsp + 0x20], r15
0x14049165e lea      r8, [rip + 0x254da8a] ; @0x1429df0ef 'Freecam Position SaveHistoryAudioLogoVisualsGfxVermeerLightingTime of dayWeatherColorCameraSetti'
0x140491665 lea      rcx, [rbp + 0x90]
0x14049166c mov      r9d, 0x15
0x140491672 mov      rdx, rsi
0x140491675 call     0x1400c2b40
0x14049167a mov      rax, qword ptr [rbp + 0x100]
0x140491681 lock dec qword ptr [rax]
0x140491685 jne      0x140491693
0x140491687 lea      rcx, [rbp + 0x100]
0x14049168e call     0x140c41170
0x140491693 cmp      dword ptr [rbp + 0xd0], 2
0x14049169a je       0x1404916b5
0x14049169c mov      rax, qword ptr [rbp + 0xa0]
0x1404916a3 lock dec qword ptr [rax]
0x1404916a7 jne      0x1404916b5
0x1404916a9 lea      rcx, [rbp + 0xa0]
0x1404916b0 call     0x140c41170
0x1404916b5 mov      qword ptr [rsp + 0x20], r13
0x1404916ba lea      r8, [rip + 0x254da43] ; @0x1429df104 'HistoryAudioLogoVisualsGfxVermeerLightingTime of dayWeatherColorCameraSettingsTrailer stuffsMisc'
0x1404916c1 lea      rcx, [rbp + 0x90]
0x1404916c8 mov      r9d, 7
0x1404916ce mov      rdx, rsi
0x1404916d1 call     0x1400c14b0
0x1404916d6 mov      rax, qword ptr [rbp + 0x100]
0x1404916dd lock dec qword ptr [rax]
0x1404916e1 jne      0x1404916ef
0x1404916e3 lea      rcx, [rbp + 0x100]
0x1404916ea call     0x140c41170
0x1404916ef cmp      dword ptr [rbp + 0xd0], 2
0x1404916f6 je       0x140491711
0x1404916f8 mov      rax, qword ptr [rbp + 0xa0]
0x1404916ff lock dec qword ptr [rax]
0x140491703 jne      0x140491711
0x140491705 lea      rcx, [rbp + 0xa0]
0x14049170c call     0x140c41170
0x140491711 mov      qword ptr [rbp + 0x20], rbx
0x140491715 movups   xmmword ptr [rbp + 0x28], xmm12
0x14049171a mov      qword ptr [rbp + 0x38], r14
0x14049171e mov      rax, qword ptr [rbp + 0x18]
0x140491722 mov      qword ptr [rbp + 0x40], rax
0x140491726 lea      rax, [rbp + 0x20]
0x14049172a mov      qword ptr [rsp + 0x20], rax
0x14049172f lea      r8, [rip + 0x254d9d5] ; @0x1429df10b 'AudioLogoVisualsGfxVermeerLightingTime of dayWeatherColorCameraSettingsTrailer stuffsMiscDebug'
0x140491736 lea      rcx, [rbp + 0x90]
0x14049173d mov      r9d, 5
0x140491743 mov      rdx, rsi
0x140491746 call     0x1400c12c0
0x14049174b mov      rax, qword ptr [rbp + 0x100]
0x140491752 lock dec qword ptr [rax]
0x140491756 jne      0x140491764
0x140491758 lea      rcx, [rbp + 0x100]
0x14049175f call     0x140c41170
0x140491764 cmp      dword ptr [rbp + 0xd0], 2
0x14049176b je       0x140491786
0x14049176d mov      rax, qword ptr [rbp + 0xa0]
0x140491774 lock dec qword ptr [rax]
0x140491778 jne      0x140491786
0x14049177a lea      rcx, [rbp + 0xa0]
0x140491781 call     0x140c41170
0x140491786 mov      qword ptr [rsp + 0x20], rdi
0x14049178b lea      r8, [rip + 0x254d97e] ; @0x1429df110 'LogoVisualsGfxVermeerLightingTime of dayWeatherColorCameraSettingsTrailer stuffsMiscDebug'
0x140491792 lea      rcx, [rbp + 0x90]
0x140491799 mov      r9d, 4
0x14049179f mov      rdx, rsi
0x1404917a2 call     0x1400c2000
0x1404917a7 mov      rax, qword ptr [rbp + 0x100]
0x1404917ae lock dec qword ptr [rax]
0x1404917b2 jne      0x1404917c0
0x1404917b4 lea      rcx, [rbp + 0x100]
0x1404917bb call     0x140c41170
0x1404917c0 cmp      dword ptr [rbp + 0xd0], 2
0x1404917c7 je       0x1404917e2
0x1404917c9 mov      rax, qword ptr [rbp + 0xa0]
0x1404917d0 lock dec qword ptr [rax]
0x1404917d4 jne      0x1404917e2
0x1404917d6 lea      rcx, [rbp + 0xa0]
0x1404917dd call     0x140c41170
0x1404917e2 lea      rdx, [rip + 0x254d92b] ; @0x1429df114 'VisualsGfxVermeerLightingTime of dayWeatherColorCameraSettingsTrailer stuffsMiscDebug'
0x1404917e9 lea      rcx, [rbp + 0x20]
0x1404917ed mov      r8d, 7
0x1404917f3 call     0x1400c1200
0x1404917f8 mov      rax, qword ptr [rbp + 0x38]
0x1404917fc cmp      rax, 6
0x140491800 je       0x14049181a
0x140491802 cmp      eax, 5
0x140491805 jb       0x14049181a
0x140491807 mov      rax, qword ptr [rbp + 0x40]
0x14049180b lock dec qword ptr [rax]
0x14049180f jne      0x14049181a
0x140491811 lea      rcx, [rbp + 0x40]
0x140491815 call     0x14064eb30
0x14049181a mov      qword ptr [rbp + 0x38], 4
0x140491822 movups   xmm0, xmmword ptr [rbp + 8]
0x140491826 movups   xmm1, xmmword ptr [rbp + 0x20]
0x14049182a movups   xmm2, xmmword ptr [rbp + 0x50]
0x14049182e movups   xmm3, xmmword ptr [rbp + 0x60]
0x140491832 movups   xmm4, xmmword ptr [rbp + 0x70]
0x140491836 movaps   xmmword ptr [rbp + 0x90], xmm1
0x14049183d movaps   xmmword ptr [rbp + 0xb0], xmm0
0x140491844 movaps   xmmword ptr [rbp + 0xc0], xmm2
0x14049184b movaps   xmmword ptr [rbp + 0xd0], xmm3
0x140491852 movaps   xmmword ptr [rbp + 0xe0], xmm4
0x140491859 movups   xmm0, xmmword ptr [rbp + 0x80]
0x140491860 movaps   xmmword ptr [rbp + 0xf0], xmm0
0x140491867 mov      rax, qword ptr [rbp + 0x30]
0x14049186b mov      qword ptr [rbp + 0xa0], rax
0x140491872 mov      rax, qword ptr [rbp + 0x38]
0x140491876 mov      qword ptr [rbp + 0xa8], rax
0x14049187d mov      byte ptr [rbp + 0x101], 2
0x140491884 mov      byte ptr [rbp + 0x100], 0
0x14049188b mov      byte ptr [rbp + 0x102], 2
0x140491892 mov      byte ptr [rbp + 0x105], 2
0x140491899 lea      rdi, [rbp + 0x20]
0x14049189d lea      rdx, [rbp + 0x90]
0x1404918a4 mov      rcx, rdi
0x1404918a7 mov      r8, rsi
0x1404918aa call     0x140c28d40
0x1404918af mov      rax, qword ptr [rbp + 0x30]
0x1404918b3 lock dec qword ptr [rax]
0x1404918b7 jne      0x1404918c2
0x1404918b9 lea      rcx, [rbp + 0x30]
0x1404918bd call     0x140c41170
0x1404918c2 movaps   xmmword ptr [rbp + 0x20], xmm11
0x1404918c7 mov      qword ptr [rbp + 0x30], r12
0x1404918cb mov      qword ptr [rsp + 0x20], rdi
0x1404918d0 lea      r8, [rip + 0x254d844] ; @0x1429df11b 'GfxVermeerLightingTime of dayWeatherColorCameraSettingsTrailer stuffsMiscDebug'
0x1404918d7 lea      rcx, [rbp + 0x90]
0x1404918de mov      r9d, 3
0x1404918e4 mov      rdx, rsi
0x1404918e7 call     0x1400c1e20
0x1404918ec mov      rax, qword ptr [rbp + 0x100]
0x1404918f3 lock dec qword ptr [rax]
0x1404918f7 jne      0x140491905
0x1404918f9 lea      rcx, [rbp + 0x100]
0x140491900 call     0x140c41170
0x140491905 cmp      dword ptr [rbp + 0xd0], 2
0x14049190c mov      r14, qword ptr [rbp - 0x48]
0x140491910 je       0x14049192b
0x140491912 mov      rax, qword ptr [rbp + 0xa0]
0x140491919 lock dec qword ptr [rax]
0x14049191d jne      0x14049192b
0x14049191f lea      rcx, [rbp + 0xa0]
0x140491926 call     0x140c41170
0x14049192b mov      rax, qword ptr [rbp - 0x40]
0x14049192f mov      qword ptr [rsp + 0x20], rax
0x140491934 lea      r8, [rip + 0x254d7e3] ; @0x1429df11e 'VermeerLightingTime of dayWeatherColorCameraSettingsTrailer stuffsMiscDebug'
0x14049193b lea      rcx, [rbp + 0x90]
0x140491942 mov      r9d, 7
0x140491948 mov      rdx, rsi
0x14049194b call     0x1400c25a0
0x140491950 mov      rax, qword ptr [rbp + 0x100]
0x140491957 lock dec qword ptr [rax]
0x14049195b jne      0x140491969
0x14049195d lea      rcx, [rbp + 0x100]
0x140491964 call     0x140c41170
0x140491969 cmp      dword ptr [rbp + 0xd0], 2
0x140491970 je       0x14049198b
0x140491972 mov      rax, qword ptr [rbp + 0xa0]
0x140491979 lock dec qword ptr [rax]
0x14049197d jne      0x14049198b
0x14049197f lea      rcx, [rbp + 0xa0]
0x140491986 call     0x140c41170
0x14049198b movaps   xmmword ptr [rbp + 0x20], xmm10
0x140491990 mov      rax, qword ptr [rbp - 0x38]
0x140491994 mov      qword ptr [rbp + 0x30], rax
0x140491998 mov      qword ptr [rsp + 0x20], rdi
0x14049199d lea      r8, [rip + 0x254d781] ; @0x1429df125 'LightingTime of dayWeatherColorCameraSettingsTrailer stuffsMiscDebug'
0x1404919a4 lea      rcx, [rbp + 0x90]
0x1404919ab mov      r9d, 8
0x1404919b1 mov      rdx, rsi
0x1404919b4 call     0x1400c2960
0x1404919b9 mov      rax, qword ptr [rbp + 0x100]
0x1404919c0 lock dec qword ptr [rax]
0x1404919c4 jne      0x1404919d2
0x1404919c6 lea      rcx, [rbp + 0x100]
0x1404919cd call     0x140c41170
0x1404919d2 cmp      dword ptr [rbp + 0xd0], 2
0x1404919d9 je       0x1404919f4
0x1404919db mov      rax, qword ptr [rbp + 0xa0]
0x1404919e2 lock dec qword ptr [rax]
0x1404919e6 jne      0x1404919f4
0x1404919e8 lea      rcx, [rbp + 0xa0]
0x1404919ef call     0x140c41170
0x1404919f4 mov      qword ptr [rbp + 0x20], r14
0x1404919f8 mov      rax, qword ptr [rbp - 0x30]
0x1404919fc mov      qword ptr [rbp + 0x28], rax
0x140491a00 movups   xmmword ptr [rbp + 0x30], xmm8
0x140491a05 mov      rax, qword ptr [rbp - 0x28]
0x140491a09 mov      qword ptr [rbp + 0x40], rax
0x140491a0d movups   xmmword ptr [rbp + 0x48], xmm9
0x140491a12 mov      qword ptr [rbp + 0x58], rbx
0x140491a16 mov      qword ptr [rsp + 0x20], rdi
0x140491a1b lea      r8, [rip + 0x254d70b] ; @0x1429df12d 'Time of dayWeatherColorCameraSettingsTrailer stuffsMiscDebug'
0x140491a22 lea      rcx, [rbp + 0x90]
0x140491a29 mov      r9d, 0xb
0x140491a2f mov      rdx, rsi
0x140491a32 call     0x1400c1c30
0x140491a37 mov      rax, qword ptr [rbp + 0x100]
0x140491a3e lock dec qword ptr [rax]
0x140491a42 jne      0x140491a50
0x140491a44 lea      rcx, [rbp + 0x100]
0x140491a4b call     0x140c41170
0x140491a50 cmp      dword ptr [rbp + 0xd0], 2
0x140491a57 je       0x140491a72
0x140491a59 mov      rax, qword ptr [rbp + 0xa0]
0x140491a60 lock dec qword ptr [rax]
0x140491a64 jne      0x140491a72
0x140491a66 lea      rcx, [rbp + 0xa0]
0x140491a6d call     0x140c41170
0x140491a72 mov      rax, qword ptr [rbp - 0x18]
0x140491a76 mov      qword ptr [rsp + 0x28], rax
0x140491a7b mov      rax, qword ptr [rbp - 0x20]
0x140491a7f mov      qword ptr [rsp + 0x20], rax
0x140491a84 lea      r8, [rip + 0x254d6ad] ; @0x1429df138 'WeatherColorCameraSettingsTrailer stuffsMiscDebug'
0x140491a8b lea      rcx, [rbp + 0x90]
0x140491a92 mov      r9d, 7
0x140491a98 mov      rdx, rsi
0x140491a9b call     0x1400c1870
0x140491aa0 mov      rax, qword ptr [rbp + 0x100]
0x140491aa7 lock dec qword ptr [rax]
0x140491aab jne      0x140491ab9
0x140491aad lea      rcx, [rbp + 0x100]
0x140491ab4 call     0x140c41170
0x140491ab9 cmp      dword ptr [rbp + 0xd0], 2
0x140491ac0 je       0x140491adb
0x140491ac2 mov      rax, qword ptr [rbp + 0xa0]
0x140491ac9 lock dec qword ptr [rax]
0x140491acd jne      0x140491adb
0x140491acf lea      rcx, [rbp + 0xa0]
0x140491ad6 call     0x140c41170
0x140491adb movaps   xmmword ptr [rbp + 0x20], xmm7
0x140491adf mov      qword ptr [rbp + 0x30], r14
0x140491ae3 mov      rax, qword ptr [rbp - 0x10]
0x140491ae7 mov      qword ptr [rbp + 0x38], rax
0x140491aeb mov      qword ptr [rsp + 0x20], rdi
0x140491af0 lea      r8, [rip + 0x254d648] ; @0x1429df13f 'ColorCameraSettingsTrailer stuffsMiscDebug'
0x140491af7 lea      rcx, [rbp + 0x90]
0x140491afe mov      r9d, 5
0x140491b04 mov      rdx, rsi
0x140491b07 call     0x1400c2780
0x140491b0c mov      rax, qword ptr [rbp + 0x100]
0x140491b13 lock dec qword ptr [rax]
0x140491b17 jne      0x140491b25
0x140491b19 lea      rcx, [rbp + 0x100]
0x140491b20 call     0x140c41170
0x140491b25 cmp      dword ptr [rbp + 0xd0], 2
0x140491b2c je       0x140491b47
0x140491b2e mov      rax, qword ptr [rbp + 0xa0]
0x140491b35 lock dec qword ptr [rax]
0x140491b39 jne      0x140491b47
0x140491b3b lea      rcx, [rbp + 0xa0]
0x140491b42 call     0x140c41170
0x140491b47 lea      rdx, [rip + 0x254d5f6] ; @0x1429df144 'CameraSettingsTrailer stuffsMiscDebug'
0x140491b4e lea      rcx, [rbp + 0x20]
0x140491b52 mov      r8d, 6
0x140491b58 call     0x1400c1200
0x140491b5d mov      rax, qword ptr [rbp + 0x38]
0x140491b61 cmp      rax, 6
0x140491b65 je       0x140491b7f
0x140491b67 cmp      eax, 5
0x140491b6a jb       0x140491b7f
0x140491b6c mov      rax, qword ptr [rbp + 0x40]
0x140491b70 lock dec qword ptr [rax]
0x140491b74 jne      0x140491b7f
0x140491b76 lea      rcx, [rbp + 0x40]
0x140491b7a call     0x14064eb30
0x140491b7f mov      qword ptr [rbp + 0x38], 4
0x140491b87 movups   xmm0, xmmword ptr [rbp + 8]
0x140491b8b movups   xmm1, xmmword ptr [rbp + 0x20]
0x140491b8f movups   xmm2, xmmword ptr [rbp + 0x50]
0x140491b93 movups   xmm3, xmmword ptr [rbp + 0x60]
0x140491b97 movups   xmm4, xmmword ptr [rbp + 0x70]
0x140491b9b movaps   xmmword ptr [rbp + 0x90], xmm1
0x140491ba2 movaps   xmmword ptr [rbp + 0xb0], xmm0
0x140491ba9 movaps   xmmword ptr [rbp + 0xc0], xmm2
0x140491bb0 movaps   xmmword ptr [rbp + 0xd0], xmm3
0x140491bb7 movaps   xmmword ptr [rbp + 0xe0], xmm4
0x140491bbe movups   xmm0, xmmword ptr [rbp + 0x80]
0x140491bc5 movaps   xmmword ptr [rbp + 0xf0], xmm0
0x140491bcc mov      rax, qword ptr [rbp + 0x30]
0x140491bd0 mov      qword ptr [rbp + 0xa0], rax
0x140491bd7 mov      rax, qword ptr [rbp + 0x38]
0x140491bdb mov      qword ptr [rbp + 0xa8], rax
0x140491be2 mov      byte ptr [rbp + 0x101], 2
0x140491be9 mov      byte ptr [rbp + 0x100], 0
0x140491bf0 mov      byte ptr [rbp + 0x102], 2
0x140491bf7 mov      byte ptr [rbp + 0x105], 2
0x140491bfe lea      rdx, [rbp + 0x90]
0x140491c05 mov      rcx, rdi
0x140491c08 mov      r8, rsi
0x140491c0b call     0x140c28d40
0x140491c10 mov      rax, qword ptr [rbp + 0x30]
0x140491c14 lock dec qword ptr [rax]
0x140491c18 jne      0x140491c23
0x140491c1a lea      rcx, [rbp + 0x30]
0x140491c1e call     0x140c41170
0x140491c23 mov      rax, qword ptr [rbp + 0x18]
0x140491c27 mov      qword ptr [rbp + 0x20], rax
0x140491c2b mov      qword ptr [rbp + 0x28], rbx
0x140491c2f mov      rax, qword ptr [rbp - 8]
0x140491c33 mov      qword ptr [rbp + 0x30], rax
0x140491c37 mov      qword ptr [rsp + 0x20], rdi
0x140491c3c lea      r8, [rip + 0x254d507] ; @0x1429df14a 'SettingsTrailer stuffsMiscDebug'
0x140491c43 lea      rcx, [rbp + 0x90]
0x140491c4a mov      r9d, 8
0x140491c50 mov      rdx, rsi
0x140491c53 call     0x1400c1a50
0x140491c58 mov      rax, qword ptr [rbp + 0x100]
0x140491c5f lock dec qword ptr [rax]
0x140491c63 jne      0x140491c71
0x140491c65 lea      rcx, [rbp + 0x100]
0x140491c6c call     0x140c41170
0x140491c71 cmp      dword ptr [rbp + 0xd0], 2
0x140491c78 je       0x140491c93
0x140491c7a mov      rax, qword ptr [rbp + 0xa0]
0x140491c81 lock dec qword ptr [rax]
0x140491c85 jne      0x140491c93
0x140491c87 lea      rcx, [rbp + 0xa0]
0x140491c8e call     0x140c41170
0x140491c93 mov      rax, qword ptr [rbp]
0x140491c97 mov      qword ptr [rsp + 0x28], rax
0x140491c9c mov      qword ptr [rsp + 0x20], rbx
0x140491ca1 lea      r8, [rip + 0x254d4aa] ; @0x1429df152 'Trailer stuffsMiscDebug'
0x140491ca8 lea      rcx, [rbp + 0x90]
0x140491caf mov      r9d, 0xe
0x140491cb5 mov      rdx, rsi
0x140491cb8 call     0x1400c23c0
0x140491cbd mov      rax, qword ptr [rbp + 0x100]
0x140491cc4 lock dec qword ptr [rax]
0x140491cc8 jne      0x140491cd6
0x140491cca lea      rcx, [rbp + 0x100]
0x140491cd1 call     0x140c41170
0x140491cd6 cmp      dword ptr [rbp + 0xd0], 2
0x140491cdd je       0x140491cf8
0x140491cdf mov      rax, qword ptr [rbp + 0xa0]
0x140491ce6 lock dec qword ptr [rax]
0x140491cea jne      0x140491cf8
0x140491cec lea      rcx, [rbp + 0xa0]
0x140491cf3 call     0x140c41170
0x140491cf8 lea      rdx, [rip + 0x254d461] ; @0x1429df160 'MiscDebug'
0x140491cff lea      rcx, [rbp + 0x20]
0x140491d03 mov      r8d, 4
0x140491d09 call     0x1400c1200
0x140491d0e mov      rax, qword ptr [rbp + 0x38]
0x140491d12 cmp      rax, 6
0x140491d16 je       0x140491d30
0x140491d18 cmp      eax, 5
0x140491d1b jb       0x140491d30
0x140491d1d mov      rax, qword ptr [rbp + 0x40]
0x140491d21 lock dec qword ptr [rax]
0x140491d25 jne      0x140491d30
0x140491d27 lea      rcx, [rbp + 0x40]
0x140491d2b call     0x14064eb30
0x140491d30 mov      qword ptr [rbp + 0x38], 4
0x140491d38 movups   xmm0, xmmword ptr [rbp + 8]
0x140491d3c movups   xmm1, xmmword ptr [rbp + 0x20]
0x140491d40 movups   xmm2, xmmword ptr [rbp + 0x50]
0x140491d44 movups   xmm3, xmmword ptr [rbp + 0x60]
0x140491d48 movups   xmm4, xmmword ptr [rbp + 0x70]
0x140491d4c movaps   xmmword ptr [rbp + 0x90], xmm1
0x140491d53 movaps   xmmword ptr [rbp + 0xb0], xmm0
0x140491d5a movaps   xmmword ptr [rbp + 0xc0], xmm2
0x140491d61 movaps   xmmword ptr [rbp + 0xd0], xmm3
0x140491d68 movaps   xmmword ptr [rbp + 0xe0], xmm4
0x140491d6f movups   xmm0, xmmword ptr [rbp + 0x80]
0x140491d76 movaps   xmmword ptr [rbp + 0xf0], xmm0
0x140491d7d mov      rax, qword ptr [rbp + 0x30]
0x140491d81 mov      qword ptr [rbp + 0xa0], rax
0x140491d88 mov      rax, qword ptr [rbp + 0x38]
0x140491d8c mov      qword ptr [rbp + 0xa8], rax
0x140491d93 mov      byte ptr [rbp + 0x101], 2
0x140491d9a mov      byte ptr [rbp + 0x100], 0
0x140491da1 mov      byte ptr [rbp + 0x102], 2
0x140491da8 mov      byte ptr [rbp + 0x105], 2
0x140491daf lea      rdx, [rbp + 0x90]
0x140491db6 mov      rcx, rdi
0x140491db9 mov      r8, rsi
0x140491dbc call     0x140c28d40
0x140491dc1 mov      rax, qword ptr [rbp + 0x30]
0x140491dc5 lock dec qword ptr [rax]
0x140491dc9 jne      0x140491dd4
0x140491dcb lea      rcx, [rbp + 0x30]
0x140491dcf call     0x140c41170
0x140491dd4 mov      qword ptr [rbp + 0x20], rbx
0x140491dd8 movups   xmmword ptr [rbp + 0x28], xmm6
0x140491ddc mov      qword ptr [rsp + 0x20], rdi
0x140491de1 lea      r8, [rip + 0x254d37c] ; @0x1429df164 'Debug'
0x140491de8 lea      rcx, [rbp + 0x90]
0x140491def mov      r9d, 5
0x140491df5 mov      rdx, rsi
0x140491df8 call     0x1400c1690
0x140491dfd mov      rax, qword ptr [rbp + 0x100]
0x140491e04 lock dec qword ptr [rax]
0x140491e08 jne      0x140491e16
0x140491e0a lea      rcx, [rbp + 0x100]
0x140491e11 call     0x140c41170
0x140491e16 cmp      dword ptr [rbp + 0xd0], 2
0x140491e1d je       0x140491e38
0x140491e1f mov      rax, qword ptr [rbp + 0xa0]
0x140491e26 lock dec qword ptr [rax]
0x140491e2a jne      0x140491e38
0x140491e2c lea      rcx, [rbp + 0xa0]
0x140491e33 call     0x140c41170
0x140491e38 movaps   xmm6, xmmword ptr [rbp + 0x160]
0x140491e3f movaps   xmm7, xmmword ptr [rbp + 0x170]
0x140491e46 movaps   xmm8, xmmword ptr [rbp + 0x180]
0x140491e4e movaps   xmm9, xmmword ptr [rbp + 0x190]
0x140491e56 movaps   xmm10, xmmword ptr [rbp + 0x1a0]
0x140491e5e movaps   xmm11, xmmword ptr [rbp + 0x1b0]
0x140491e66 movaps   xmm12, xmmword ptr [rbp + 0x1c0]
0x140491e6e add      rsp, 0x258
0x140491e75 pop      rbx
0x140491e76 pop      rdi
0x140491e77 pop      rsi
0x140491e78 pop      r12
0x140491e7a pop      r13
0x140491e7c pop      r14
0x140491e7e pop      r15
0x140491e80 pop      rbp
0x140491e81 ret      