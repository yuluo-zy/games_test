0x1400b6580 push     rbp
0x1400b6581 push     r14
0x1400b6583 push     rsi
0x1400b6584 push     rdi
0x1400b6585 push     rbx
0x1400b6586 sub      rsp, 0x2c0
0x1400b658d lea      rbp, [rsp + 0x80]
0x1400b6595 mov      qword ptr [rbp + 0x238], 0xfffffffffffffffe
0x1400b65a0 mov      rsi, rdx
0x1400b65a3 mov      rdi, rcx
0x1400b65a6 movzx    eax, byte ptr [rip + 0x357e65c]
0x1400b65ad mov      ecx, 0xe
0x1400b65b2 mov      edx, 1
0x1400b65b7 call     0x140613c10
0x1400b65bc test     rax, rax
0x1400b65bf je       0x1400b6e16
0x1400b65c5 movabs   rcx, 0x72657469746a2041
0x1400b65cf mov      qword ptr [rax + 6], rcx
0x1400b65d3 movabs   rcx, 0x2041415420657355
0x1400b65dd mov      qword ptr [rax], rcx
0x1400b65e0 mov      qword ptr [rbp + 0x138], 0xe
0x1400b65eb mov      qword ptr [rbp + 0x140], rax
0x1400b65f2 mov      qword ptr [rbp + 0x148], 0xe
0x1400b65fd mov      dword ptr [rbp + 0x150], 0
0x1400b6607 mov      qword ptr [rbp + 0x160], 0
0x1400b6612 mov      dword ptr [rbp + 0x158], 0
0x1400b661c mov      qword ptr [rbp + 0x108], 3
0x1400b6627 mov      qword ptr [rbp + 0x120], 6
0x1400b6632 mov      byte ptr [rbp + 0x168], 0
0x1400b6639 mov      dword ptr [rbp + 0x16d], 0
0x1400b6643 mov      dword ptr [rbp + 0x170], 0
0x1400b664d mov      rax, qword ptr [rip + 0x3575a54]
0x1400b6654 mov      qword ptr [rbp + 0x178], rax
0x1400b665b mov      byte ptr [rbp + 0x180], 0
0x1400b6662 lea      rcx, [rbp + 0x38]
0x1400b6666 lea      rdx, [rbp + 0x108]
0x1400b666d mov      r8, rsi
0x1400b6670 call     0x140c48930
0x1400b6675 mov      rax, qword ptr [rbp + 0x48]
0x1400b6679 lock dec qword ptr [rax]
0x1400b667d jne      0x1400b6688
0x1400b667f lea      rcx, [rbp + 0x48]
0x1400b6683 call     0x140c41170
0x1400b6688 movzx    eax, byte ptr [rip + 0x357e57a]
0x1400b668f mov      ecx, 0x13
0x1400b6694 mov      edx, 1
0x1400b6699 call     0x140613c10
0x1400b669e test     rax, rax
0x1400b66a1 je       0x1400b6e2c
0x1400b66a7 movups   xmm0, xmmword ptr [rip + 0x287ea44] ; @0x1429350f2 'Force TAA pass-thruFreeze occlusion cullingShow memory breakdownShow memory blocksGPU crash mark'
0x1400b66ae movups   xmmword ptr [rax], xmm0
0x1400b66b1 mov      dword ptr [rax + 0xf], 0x75726874
0x1400b66b8 mov      qword ptr [rbp + 0x138], 0x13
0x1400b66c3 mov      qword ptr [rbp + 0x140], rax
0x1400b66ca mov      qword ptr [rbp + 0x148], 0x13
0x1400b66d5 mov      dword ptr [rbp + 0x150], 0
0x1400b66df mov      qword ptr [rbp + 0x160], 0
0x1400b66ea mov      dword ptr [rbp + 0x158], 0
0x1400b66f4 mov      qword ptr [rbp + 0x108], 3
0x1400b66ff mov      qword ptr [rbp + 0x120], 6
0x1400b670a mov      byte ptr [rbp + 0x168], 0
0x1400b6711 mov      dword ptr [rbp + 0x16d], 0
0x1400b671b mov      dword ptr [rbp + 0x170], 0
0x1400b6725 mov      rax, qword ptr [rip + 0x357594c] ; @0x14362c078 'PMcC'
0x1400b672c mov      qword ptr [rbp + 0x178], rax
0x1400b6733 mov      byte ptr [rbp + 0x180], 0
0x1400b673a lea      rcx, [rbp + 0x38]
0x1400b673e lea      rdx, [rbp + 0x108]
0x1400b6745 mov      r8, rsi
0x1400b6748 call     0x140c48930
0x1400b674d mov      rax, qword ptr [rbp + 0x48]
0x1400b6751 lock dec qword ptr [rax]
0x1400b6755 jne      0x1400b6760
0x1400b6757 lea      rcx, [rbp + 0x48]
0x1400b675b call     0x140c41170
0x1400b6760 movzx    eax, byte ptr [rip + 0x357e4a2]
0x1400b6767 mov      ecx, 0xd
0x1400b676c mov      edx, 1
0x1400b6771 call     0x140613c10
0x1400b6776 test     rax, rax
0x1400b6779 je       0x1400b6e42
0x1400b677f movabs   rcx, 0x466f44207365722d
0x1400b6789 mov      qword ptr [rax + 5], rcx
0x1400b678d movabs   rcx, 0x65722d69746c754d
0x1400b6797 mov      qword ptr [rax], rcx
0x1400b679a mov      qword ptr [rbp + 0x138], 0xd
0x1400b67a5 mov      qword ptr [rbp + 0x140], rax
0x1400b67ac mov      qword ptr [rbp + 0x148], 0xd
0x1400b67b7 mov      dword ptr [rbp + 0x150], 0
0x1400b67c1 mov      qword ptr [rbp + 0x160], 0
0x1400b67cc mov      dword ptr [rbp + 0x158], 0
0x1400b67d6 mov      qword ptr [rbp + 0x108], 3
0x1400b67e1 mov      qword ptr [rbp + 0x120], 6
0x1400b67ec mov      byte ptr [rbp + 0x168], 0
0x1400b67f3 mov      dword ptr [rbp + 0x16d], 0
0x1400b67fd mov      dword ptr [rbp + 0x170], 0
0x1400b6807 mov      rax, qword ptr [rip + 0x357587a]
0x1400b680e mov      qword ptr [rbp + 0x178], rax
0x1400b6815 mov      byte ptr [rbp + 0x180], 0
0x1400b681c lea      rcx, [rbp + 0x38]
0x1400b6820 lea      rdx, [rbp + 0x108]
0x1400b6827 mov      r8, rsi
0x1400b682a call     0x140c48930
0x1400b682f mov      rax, qword ptr [rbp + 0x48]
0x1400b6833 lock dec qword ptr [rax]
0x1400b6837 jne      0x1400b6842
0x1400b6839 lea      rcx, [rbp + 0x48]
0x1400b683d call     0x140c41170
0x1400b6842 movzx    eax, byte ptr [rip + 0x357e3c0]
0x1400b6849 mov      ecx, 0x18
0x1400b684e mov      edx, 1
0x1400b6853 call     0x140613c10
0x1400b6858 test     rax, rax
0x1400b685b je       0x1400b6e58
0x1400b6861 movups   xmm0, xmmword ptr [rip + 0x287e89d] ; @0x142935105 'Freeze occlusion cullingShow memory breakdownShow memory blocksGPU crash markersParallel command'
0x1400b6868 movups   xmmword ptr [rax], xmm0
0x1400b686b movabs   rcx, 0x676e696c6c756320
0x1400b6875 mov      qword ptr [rax + 0x10], rcx
0x1400b6879 mov      qword ptr [rbp + 0x138], 0x18
0x1400b6884 mov      qword ptr [rbp + 0x140], rax
0x1400b688b mov      qword ptr [rbp + 0x148], 0x18
0x1400b6896 mov      dword ptr [rbp + 0x150], 0
0x1400b68a0 mov      qword ptr [rbp + 0x160], 0
0x1400b68ab mov      dword ptr [rbp + 0x158], 0
0x1400b68b5 mov      qword ptr [rbp + 0x108], 3
0x1400b68c0 mov      qword ptr [rbp + 0x120], 6
0x1400b68cb mov      byte ptr [rbp + 0x168], 0
0x1400b68d2 mov      dword ptr [rbp + 0x16d], 0
0x1400b68dc mov      dword ptr [rbp + 0x170], 0
0x1400b68e6 mov      rax, qword ptr [rip + 0x35757eb]
0x1400b68ed mov      qword ptr [rbp + 0x178], rax
0x1400b68f4 mov      byte ptr [rbp + 0x180], 0
0x1400b68fb lea      rcx, [rbp + 0x38]
0x1400b68ff lea      rdx, [rbp + 0x108]
0x1400b6906 mov      r8, rsi
0x1400b6909 call     0x140c48930
0x1400b690e mov      rax, qword ptr [rbp + 0x48]
0x1400b6912 lock dec qword ptr [rax]
0x1400b6916 jne      0x1400b6921
0x1400b6918 lea      rcx, [rbp + 0x48]
0x1400b691c call     0x140c41170
0x1400b6921 mov      rbx, qword ptr [rdi]
0x1400b6924 mov      r14, qword ptr [rbx]
0x1400b6927 movzx    eax, byte ptr [rip + 0x357e2db]
0x1400b692e mov      ecx, 0x15
0x1400b6933 mov      edx, 1
0x1400b6938 call     0x140613c10
0x1400b693d test     rax, rax
0x1400b6940 je       0x1400b6e6e
0x1400b6946 add      r14, 0x48
0x1400b694a movups   xmm0, xmmword ptr [rip + 0x287e7cc] ; @0x14293511d 'Show memory breakdownShow memory blocksGPU crash markersParallel command buffer recordingStatsdi'
0x1400b6951 movups   xmmword ptr [rax], xmm0
0x1400b6954 movabs   rcx, 0x6e776f646b616572
0x1400b695e mov      qword ptr [rax + 0xd], rcx
0x1400b6962 mov      qword ptr [rbp + 0x138], 0x15
0x1400b696d mov      qword ptr [rbp + 0x140], rax
0x1400b6974 mov      qword ptr [rbp + 0x148], 0x15
0x1400b697f mov      dword ptr [rbp + 0x150], 0
0x1400b6989 mov      qword ptr [rbp + 0x160], 0
0x1400b6994 mov      dword ptr [rbp + 0x158], 0
0x1400b699e mov      qword ptr [rbp + 0x108], 3
0x1400b69a9 mov      qword ptr [rbp + 0x120], 6
0x1400b69b4 mov      byte ptr [rbp + 0x168], 0
0x1400b69bb mov      dword ptr [rbp + 0x16d], 0
0x1400b69c5 mov      dword ptr [rbp + 0x170], 0
0x1400b69cf mov      qword ptr [rbp + 0x178], r14
0x1400b69d6 mov      byte ptr [rbp + 0x180], 0
0x1400b69dd lea      rcx, [rbp + 0x38]
0x1400b69e1 lea      rdx, [rbp + 0x108]
0x1400b69e8 mov      r8, rsi
0x1400b69eb call     0x140c48930
0x1400b69f0 mov      rax, qword ptr [rbp + 0x48]
0x1400b69f4 lock dec qword ptr [rax]
0x1400b69f8 jne      0x1400b6a03
0x1400b69fa lea      rcx, [rbp + 0x48]
0x1400b69fe call     0x140c41170
0x1400b6a03 mov      rbx, qword ptr [rbx]
0x1400b6a06 movzx    eax, byte ptr [rip + 0x357e1fc]
0x1400b6a0d mov      ecx, 0x12
0x1400b6a12 mov      edx, 1
0x1400b6a17 call     0x140613c10
0x1400b6a1c test     rax, rax
0x1400b6a1f je       0x1400b6e84
0x1400b6a25 add      rbx, 0x49
0x1400b6a29 movups   xmm0, xmmword ptr [rip + 0x287e702] ; @0x142935132 'Show memory blocksGPU crash markersParallel command buffer recordingStatsdispatch: '
0x1400b6a30 movups   xmmword ptr [rax], xmm0
0x1400b6a33 mov      word ptr [rax + 0x10], 0x736b
0x1400b6a39 mov      qword ptr [rbp + 0x138], 0x12
0x1400b6a44 mov      qword ptr [rbp + 0x140], rax
0x1400b6a4b mov      qword ptr [rbp + 0x148], 0x12
0x1400b6a56 mov      dword ptr [rbp + 0x150], 0
0x1400b6a60 mov      qword ptr [rbp + 0x160], 0
0x1400b6a6b mov      dword ptr [rbp + 0x158], 0
0x1400b6a75 mov      qword ptr [rbp + 0x108], 3
0x1400b6a80 mov      qword ptr [rbp + 0x120], 6
0x1400b6a8b mov      byte ptr [rbp + 0x168], 0
0x1400b6a92 mov      dword ptr [rbp + 0x16d], 0
0x1400b6a9c mov      dword ptr [rbp + 0x170], 0
0x1400b6aa6 mov      qword ptr [rbp + 0x178], rbx
0x1400b6aad mov      byte ptr [rbp + 0x180], 0
0x1400b6ab4 lea      rcx, [rbp + 0x38]
0x1400b6ab8 lea      rdx, [rbp + 0x108]
0x1400b6abf mov      r8, rsi
0x1400b6ac2 call     0x140c48930
0x1400b6ac7 mov      rax, qword ptr [rbp + 0x48]
0x1400b6acb lock dec qword ptr [rax]
0x1400b6acf jne      0x1400b6ada
0x1400b6ad1 lea      rcx, [rbp + 0x48]
0x1400b6ad5 call     0x140c41170
0x1400b6ada movzx    eax, byte ptr [rip + 0x357e128]
0x1400b6ae1 mov      ecx, 0x11
0x1400b6ae6 mov      edx, 1
0x1400b6aeb call     0x140613c10
0x1400b6af0 test     rax, rax
0x1400b6af3 je       0x1400b6e9a
0x1400b6af9 movups   xmm0, xmmword ptr [rip + 0x287e644] ; @0x142935144 'GPU crash markersParallel command buffer recordingStatsdispatch: '
0x1400b6b00 movups   xmmword ptr [rax], xmm0
0x1400b6b03 mov      byte ptr [rax + 0x10], 0x73
0x1400b6b07 mov      qword ptr [rbp + 0x138], 0x11
0x1400b6b12 mov      qword ptr [rbp + 0x140], rax
0x1400b6b19 mov      qword ptr [rbp + 0x148], 0x11
0x1400b6b24 mov      dword ptr [rbp + 0x150], 0
0x1400b6b2e mov      qword ptr [rbp + 0x160], 0
0x1400b6b39 mov      dword ptr [rbp + 0x158], 0
0x1400b6b43 mov      qword ptr [rbp + 0x108], 3
0x1400b6b4e mov      qword ptr [rbp + 0x120], 6
0x1400b6b59 mov      byte ptr [rbp + 0x168], 0
0x1400b6b60 mov      dword ptr [rbp + 0x16d], 0
0x1400b6b6a mov      dword ptr [rbp + 0x170], 0
0x1400b6b74 mov      rax, qword ptr [rip + 0x35756a5]
0x1400b6b7b mov      qword ptr [rbp + 0x178], rax
0x1400b6b82 mov      byte ptr [rbp + 0x180], 0
0x1400b6b89 lea      rcx, [rbp + 0x38]
0x1400b6b8d lea      rdx, [rbp + 0x108]
0x1400b6b94 mov      r8, rsi
0x1400b6b97 call     0x140c48930
0x1400b6b9c mov      rax, qword ptr [rbp + 0x48]
0x1400b6ba0 lock dec qword ptr [rax]
0x1400b6ba4 jne      0x1400b6baf
0x1400b6ba6 lea      rcx, [rbp + 0x48]
0x1400b6baa call     0x140c41170
0x1400b6baf mov      rax, qword ptr [rdi + 8]
0x1400b6bb3 mov      rax, qword ptr [rax]
0x1400b6bb6 mov      rax, qword ptr [rax]
0x1400b6bb9 movzx    eax, byte ptr [rax + 0xacb]
0x1400b6bc0 xor      al, 1
0x1400b6bc2 mov      rbx, qword ptr [rip + 0x357565f]
0x1400b6bc9 movzx    ecx, byte ptr [rbx]
0x1400b6bcc cmp      cl, 2
0x1400b6bcf movzx    r14d, al
0x1400b6bd3 cmovne   r14d, ecx
0x1400b6bd7 and      r14b, 1
0x1400b6bdb mov      byte ptr [rbp - 0x58], r14b
0x1400b6bdf movzx    eax, byte ptr [rip + 0x357e023]
0x1400b6be6 mov      ecx, 0x21
0x1400b6beb mov      edx, 1
0x1400b6bf0 call     0x140613c10
0x1400b6bf5 test     rax, rax
0x1400b6bf8 je       0x1400b6eb0
0x1400b6bfe movups   xmm0, xmmword ptr [rip + 0x287e560] ; @0x142935165 ' buffer recordingStatsdispatch: '
0x1400b6c05 movups   xmmword ptr [rax + 0x10], xmm0
0x1400b6c09 movups   xmm0, xmmword ptr [rip + 0x287e545] ; @0x142935155 'Parallel command buffer recordingStatsdispatch: '
0x1400b6c10 movups   xmmword ptr [rax], xmm0
0x1400b6c13 mov      byte ptr [rax + 0x20], 0x67
0x1400b6c17 mov      qword ptr [rbp + 0x138], 0x21
0x1400b6c22 mov      qword ptr [rbp + 0x140], rax
0x1400b6c29 mov      qword ptr [rbp + 0x148], 0x21
0x1400b6c34 mov      dword ptr [rbp + 0x150], 0
0x1400b6c3e mov      qword ptr [rbp + 0x160], 0
0x1400b6c49 mov      dword ptr [rbp + 0x158], 0
0x1400b6c53 mov      qword ptr [rbp + 0x108], 3
0x1400b6c5e mov      qword ptr [rbp + 0x120], 6
0x1400b6c69 mov      byte ptr [rbp + 0x168], 0
0x1400b6c70 mov      dword ptr [rbp + 0x16d], 0
0x1400b6c7a mov      dword ptr [rbp + 0x170], 0
0x1400b6c84 lea      rax, [rbp - 0x58]
0x1400b6c88 mov      qword ptr [rbp + 0x178], rax
0x1400b6c8f mov      byte ptr [rbp + 0x180], 0
0x1400b6c96 lea      rcx, [rbp + 0x38]
0x1400b6c9a lea      rdx, [rbp + 0x108]
0x1400b6ca1 mov      r8, rsi
0x1400b6ca4 call     0x140c48930
0x1400b6ca9 mov      rax, qword ptr [rbp + 0x48]
0x1400b6cad lock dec qword ptr [rax]
0x1400b6cb1 jne      0x1400b6cbc
0x1400b6cb3 lea      rcx, [rbp + 0x48]
0x1400b6cb7 call     0x140c41170
0x1400b6cbc movzx    eax, byte ptr [rbp - 0x58]
0x1400b6cc0 cmp      r14b, al
0x1400b6cc3 je       0x1400b6cc7
0x1400b6cc5 mov      byte ptr [rbx], al
0x1400b6cc7 mov      rdi, qword ptr [rdi + 0x10]
0x1400b6ccb lea      rdx, [rip + 0x287e4a4] ; @0x142935176 'Statsdispatch: '
0x1400b6cd2 lea      rcx, [rbp - 0x58]
0x1400b6cd6 mov      r8d, 5
0x1400b6cdc call     0x14015eea0
0x1400b6ce1 movzx    eax, byte ptr [rip + 0x357df21]
0x1400b6ce8 mov      ecx, 8
0x1400b6ced mov      edx, 8
0x1400b6cf2 call     0x140613c10
0x1400b6cf7 test     rax, rax
0x1400b6cfa mov      qword ptr [rbp + 0x100], rax
0x1400b6d01 je       0x1400b6ec6
0x1400b6d07 mov      rbx, rax
0x1400b6d0a mov      qword ptr [rax], rdi
0x1400b6d0d mov      byte ptr [rbp + 0x237], 1
0x1400b6d14 lea      rcx, [rbp + 0x38]
0x1400b6d18 lea      rdx, [rbp - 0x58]
0x1400b6d1c mov      r8d, 0x90
0x1400b6d22 call     0x142923c00
0x1400b6d27 lea      rax, [rbp + 0x237]
0x1400b6d2e mov      qword ptr [rbp + 0xc8], rax
0x1400b6d35 mov      qword ptr [rbp + 0xd0], rbx
0x1400b6d3c lea      rax, [rip + 0x287d9cd]
0x1400b6d43 mov      qword ptr [rbp + 0xd8], rax
0x1400b6d4a movzx    eax, byte ptr [rip + 0x357deb8]
0x1400b6d51 mov      ecx, 0xa8
0x1400b6d56 mov      edx, 8
0x1400b6d5b call     0x140613c10
0x1400b6d60 test     rax, rax
0x1400b6d63 je       0x1400b6ed7
0x1400b6d69 mov      rdi, rax
0x1400b6d6c lea      rdx, [rbp + 0x38]
0x1400b6d70 mov      r8d, 0xa8
0x1400b6d76 mov      rcx, rax
0x1400b6d79 call     0x142923c00
0x1400b6d7e lea      rax, [rip + 0x28a5a7b]
0x1400b6d85 mov      qword ptr [rsp + 0x20], rax
0x1400b6d8a lea      rcx, [rbp + 0x108]
0x1400b6d91 movabs   r8, 0x20001000000
0x1400b6d9b mov      rdx, rsi
0x1400b6d9e mov      r9, rdi
0x1400b6da1 call     0x1400c2f70
0x1400b6da6 lea      rcx, [rbp + 0x38]
0x1400b6daa lea      rdx, [rbp + 0x108]
0x1400b6db1 mov      r8d, 0xc8
0x1400b6db7 call     0x142923c00
0x1400b6dbc mov      rax, qword ptr [rbp + 0x1e0]
0x1400b6dc3 lock dec qword ptr [rax]
0x1400b6dc7 jne      0x1400b6dd5
0x1400b6dc9 lea      rcx, [rbp + 0x1e0]
0x1400b6dd0 call     0x140c41170
0x1400b6dd5 mov      rax, qword ptr [rbp + 0xa8]
0x1400b6ddc lock dec qword ptr [rax]
0x1400b6de0 jne      0x1400b6dee
0x1400b6de2 lea      rcx, [rbp + 0xa8]
0x1400b6de9 call     0x140c41170
0x1400b6dee cmp      dword ptr [rbp + 0x78], 2
0x1400b6df2 je       0x1400b6e07
0x1400b6df4 mov      rax, qword ptr [rbp + 0x48]
0x1400b6df8 lock dec qword ptr [rax]
0x1400b6dfc jne      0x1400b6e07
0x1400b6dfe lea      rcx, [rbp + 0x48]
0x1400b6e02 call     0x140c41170
0x1400b6e07 nop      
0x1400b6e08 add      rsp, 0x2c0
0x1400b6e0f pop      rbx
0x1400b6e10 pop      rdi
0x1400b6e11 pop      rsi
0x1400b6e12 pop      r14
0x1400b6e14 pop      rbp
0x1400b6e15 ret      
0x1400b6e16 lea      r8, [rip + 0x287df2b]
0x1400b6e1d mov      ecx, 1
0x1400b6e22 mov      edx, 0xe
0x1400b6e27 call     0x1428d8fe3
0x1400b6e2c lea      r8, [rip + 0x287df15]
0x1400b6e33 mov      ecx, 1
0x1400b6e38 mov      edx, 0x13
0x1400b6e3d call     0x1428d8fe3
0x1400b6e42 lea      r8, [rip + 0x287deff]
0x1400b6e49 mov      ecx, 1
0x1400b6e4e mov      edx, 0xd
0x1400b6e53 call     0x1428d8fe3
0x1400b6e58 lea      r8, [rip + 0x287dee9]
0x1400b6e5f mov      ecx, 1
0x1400b6e64 mov      edx, 0x18
0x1400b6e69 call     0x1428d8fe3
0x1400b6e6e lea      r8, [rip + 0x287ded3]
0x1400b6e75 mov      ecx, 1
0x1400b6e7a mov      edx, 0x15
0x1400b6e7f call     0x1428d8fe3
0x1400b6e84 lea      r8, [rip + 0x287debd]
0x1400b6e8b mov      ecx, 1
0x1400b6e90 mov      edx, 0x12
0x1400b6e95 call     0x1428d8fe3
0x1400b6e9a lea      r8, [rip + 0x287dea7]
0x1400b6ea1 mov      ecx, 1
0x1400b6ea6 mov      edx, 0x11
0x1400b6eab call     0x1428d8fe3
0x1400b6eb0 lea      r8, [rip + 0x287de91]
0x1400b6eb7 mov      ecx, 1
0x1400b6ebc mov      edx, 0x21
0x1400b6ec1 call     0x1428d8fe3
0x1400b6ec6 mov      ecx, 8
0x1400b6ecb mov      edx, 8
0x1400b6ed0 call     0x1428d9000
0x1400b6ed5 jmp      0x1400b6ee6
0x1400b6ed7 mov      ecx, 8
0x1400b6edc mov      edx, 0xa8