0x140afde20 push     rsi
0x140afde21 push     rdi
0x140afde22 push     rbx
0x140afde23 sub      rsp, 0x20
0x140afde27 mov      rax, rcx
0x140afde2a lea      rcx, [r8 - 4]
0x140afde2e cmp      rcx, 8
0x140afde32 ja       0x140afe06b
0x140afde38 lea      r9, [rip + 0x1fe84a1]
0x140afde3f movsxd   rcx, dword ptr [r9 + rcx*4]
0x140afde43 add      rcx, r9
0x140afde46 jmp      rcx
0x140afde48 cmp      dword ptr [rdx], 0x6c6c6157
0x140afde4e je       0x140afe059
0x140afde54 cmp      dword ptr [rdx], 0x68746150
0x140afde5a je       0x140afe0a0
0x140afde60 lea      r9, [rip + 0x1fe795f] ; @0x142ae57c6 'TreeTerrainEditingWaterGardenPhotomodeTimeOfDayPhotomodeFolderPhotomodeCameraPhotomodeFovPhotomodeFo'
0x140afde67 mov      rcx, rdx
0x140afde6a mov      rbx, rdx
0x140afde6d mov      rdx, r9
0x140afde70 mov      rsi, r8
0x140afde73 mov      rdi, rax
0x140afde76 call     0x142923bf0 ; memcmp
0x140afde7b mov      rdx, rbx
0x140afde7e mov      r8, rsi
0x140afde81 mov      ecx, eax
0x140afde83 mov      rax, rdi
0x140afde86 test     ecx, ecx
0x140afde88 jne      0x140afe06b
0x140afde8e mov      byte ptr [rax + 1], 0xb
0x140afde92 jmp      0x140afe0ad ; 
0x140afde97 mov      ecx, 0x74696445
0x140afde9c xor      ecx, dword ptr [rdx]
0x140afde9e mov      r9d, 0x676e6974
0x140afdea4 xor      r9d, dword ptr [rdx + 3]
0x140afdea8 or       r9d, ecx
0x140afdeab je       0x140afe04d
0x140afdeb1 mov      ecx, 0x72726554
0x140afdeb6 xor      ecx, dword ptr [rdx]
0x140afdeb8 mov      r9d, 0x6e696172
0x140afdebe xor      r9d, dword ptr [rdx + 3]
0x140afdec2 or       r9d, ecx
0x140afdec5 jne      0x140afe06b
0x140afdecb mov      byte ptr [rax + 1], 5
0x140afdecf jmp      0x140afe0ad ; 
0x140afded4 movabs   rcx, 0x756c43656e6f6c43
0x140afdede xor      rcx, qword ptr [rdx]
0x140afdee1 mov      r9d, dword ptr [rdx + 8]
0x140afdee5 xor      r9, 0x72657474
0x140afdeec or       r9, rcx
0x140afdeef jne      0x140afe06b
0x140afdef5 mov      byte ptr [rax + 1], 0xf
0x140afdef9 jmp      0x140afe0ad ; 
0x140afdefe mov      ecx, 0x73617245
0x140afdf03 xor      ecx, dword ptr [rdx]
0x140afdf05 movzx    r9d, word ptr [rdx + 4]
0x140afdf0a xor      r9d, 0x7265
0x140afdf11 or       r9d, ecx
0x140afdf14 je       0x140afe053
0x140afdf1a mov      rbx, rax
0x140afdf1d lea      rax, [rip + 0x1fe7d82] ; @0x142ae5ca6 'ReplayLoadGameFencesCloneClutter'
0x140afdf24 mov      rdi, rdx
0x140afdf27 mov      rcx, rdx
0x140afdf2a mov      rdx, rax
0x140afdf2d mov      rsi, r8
0x140afdf30 call     0x142923bf0 ; memcmp
0x140afdf35 test     eax, eax
0x140afdf37 je       0x140afe097
0x140afdf3d lea      rdx, [rip + 0x1fe7899] ; @0x142ae57dd 'GardenPhotomodeTimeOfDayPhotomodeFolderPhotomodeCameraPhotomodeFovPhotomodeFocusPhotomodeDofPhotomod'
0x140afdf44 mov      rcx, rdi
0x140afdf47 mov      r8, rsi
0x140afdf4a call     0x142923bf0 ; memcmp
0x140afdf4f test     eax, eax
0x140afdf51 je       0x140afe0a6
0x140afdf57 lea      rdx, [rip + 0x1fe7d56] ; @0x142ae5cb4 'FencesCloneClutter'
0x140afdf5e mov      rcx, rdi
0x140afdf61 mov      r8, rsi
0x140afdf64 call     0x142923bf0 ; memcmp
0x140afdf69 mov      rdx, rdi
0x140afdf6c mov      r8, rsi
0x140afdf6f test     eax, eax
0x140afdf71 mov      rax, rbx
0x140afdf74 jne      0x140afe06b
0x140afdf7a mov      byte ptr [rax + 1], 0xd
0x140afdf7e jmp      0x140afe0ad ; 
0x140afdf83 mov      ecx, 0x6f6c6f43
0x140afdf88 xor      ecx, dword ptr [rdx]
0x140afdf8a movzx    r9d, byte ptr [rdx + 4]
0x140afdf8f xor      r9d, 0x72
0x140afdf93 or       r9d, ecx
0x140afdf96 je       0x140afe05f
0x140afdf9c lea      r9, [rip + 0x1fe7835] ; @0x142ae57d8 'WaterGardenPhotomodeTimeOfDayPhotomodeFolderPhotomodeCameraPhotomodeFovPhotomodeFocusPhotomodeDofPho'
0x140afdfa3 mov      rcx, rdx
0x140afdfa6 mov      rbx, rdx
0x140afdfa9 mov      rdx, r9
0x140afdfac mov      rsi, r8
0x140afdfaf mov      rdi, rax
0x140afdfb2 call     0x142923bf0 ; memcmp
0x140afdfb7 mov      rdx, rbx
0x140afdfba mov      r8, rsi
0x140afdfbd mov      ecx, eax
0x140afdfbf mov      rax, rdi
0x140afdfc2 test     ecx, ecx
0x140afdfc4 jne      0x140afe06b
0x140afdfca mov      byte ptr [rax + 1], 0xa
0x140afdfce jmp      0x140afe0ad ; 
0x140afdfd3 movabs   rcx, 0x6f7461726f636544
0x140afdfdd xor      rcx, qword ptr [rdx]
0x140afdfe0 movzx    r9d, byte ptr [rdx + 8]
0x140afdfe5 xor      r9, 0x72
0x140afdfe9 or       r9, rcx
0x140afdfec je       0x140afe065
0x140afdfee movabs   rcx, 0x646f6d6f746f6850
0x140afdff8 xor      rcx, qword ptr [rdx]
0x140afdffb movzx    r9d, byte ptr [rdx + 8]
0x140afe000 xor      r9, 0x65
0x140afe004 or       r9, rcx
0x140afe007 jne      0x140afe06b
0x140afe009 mov      byte ptr [rax + 1], 0xe
0x140afe00d jmp      0x140afe0ad ; 
0x140afe012 movabs   rcx, 0x7473656c62626f43
0x140afe01c xor      rcx, qword ptr [rdx]
0x140afe01f movabs   r9, 0x656e6f7473656c62
0x140afe029 xor      r9, qword ptr [rdx + 3]
0x140afe02d or       r9, rcx
0x140afe030 jne      0x140afe06b
0x140afe032 mov      byte ptr [rax + 1], 3
0x140afe036 jmp      0x140afe0ad ; 
0x140afe038 movabs   rcx, 0x656d614764616f4c
0x140afe042 cmp      qword ptr [rdx], rcx
0x140afe045 jne      0x140afe06b
0x140afe047 mov      byte ptr [rax + 1], 9
0x140afe04b jmp      0x140afe0ad ; 
0x140afe04d mov      byte ptr [rax + 1], 0
0x140afe051 jmp      0x140afe0ad ; 
0x140afe053 mov      byte ptr [rax + 1], 4
0x140afe057 jmp      0x140afe0ad ; 
0x140afe059 mov      byte ptr [rax + 1], 1
0x140afe05d jmp      0x140afe0ad ; 
0x140afe05f mov      byte ptr [rax + 1], 7
0x140afe063 jmp      0x140afe0ad ; 
0x140afe065 mov      byte ptr [rax + 1], 6
0x140afe069 jmp      0x140afe0ad ; 
0x140afe06b lea      r10, [rip + 0x1fe7c56]
0x140afe072 mov      r9d, 0x10
0x140afe078 mov      rcx, rdx
0x140afe07b mov      rdx, r8
0x140afe07e mov      r8, r10
0x140afe081 mov      rsi, rax
0x140afe084 call     0x1428f9e20 ; serde::de::Error::unknown_variant::h81e52211f9e120e7
0x140afe089 mov      rcx, rax
0x140afe08c mov      rax, rsi
0x140afe08f mov      qword ptr [rsi + 8], rcx
0x140afe093 mov      cl, 1
0x140afe095 jmp      0x140afe0af ; 
0x140afe097 mov      rax, rbx
0x140afe09a mov      byte ptr [rbx + 1], 8
0x140afe09e jmp      0x140afe0ad ; 
0x140afe0a0 mov      byte ptr [rax + 1], 2
0x140afe0a4 jmp      0x140afe0ad ; 
0x140afe0a6 mov      rax, rbx
0x140afe0a9 mov      byte ptr [rbx + 1], 0xc
0x140afe0ad xor      ecx, ecx
0x140afe0af mov      byte ptr [rax], cl
0x140afe0b1 add      rsp, 0x20
0x140afe0b5 pop      rbx
0x140afe0b6 pop      rdi
0x140afe0b7 pop      rsi
0x140afe0b8 ret      