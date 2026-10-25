0x140afb5b0 push     rsi
0x140afb5b1 push     rdi
0x140afb5b2 push     rbx
0x140afb5b3 sub      rsp, 0x20
0x140afb5b7 mov      rax, rcx
0x140afb5ba lea      rcx, [r8 - 4]
0x140afb5be cmp      rcx, 0xd
0x140afb5c2 ja       0x140afb913
0x140afb5c8 lea      r9, [rip + 0x1feaae5]
0x140afb5cf movsxd   rcx, dword ptr [r9 + rcx*4]
0x140afb5d3 add      rcx, r9
0x140afb5d6 jmp      rcx
0x140afb5d8 cmp      dword ptr [rdx], 0x68746150
0x140afb5de je       0x140afb95d
0x140afb5e4 lea      r9, [rip + 0x1fea1db] ; @0x142ae57c6 'TreeTerrainEditingWaterGardenPhotomodeTimeOfDayPhotomodeFolderPhotomodeCameraPhotomodeFovPhotomodeFo'
0x140afb5eb mov      rcx, rdx
0x140afb5ee mov      rbx, rdx
0x140afb5f1 mov      rdx, r9
0x140afb5f4 mov      rsi, r8
0x140afb5f7 mov      rdi, rax
0x140afb5fa call     0x142923bf0 ; memcmp
0x140afb5ff mov      rdx, rbx
0x140afb602 mov      r8, rsi
0x140afb605 mov      ecx, eax
0x140afb607 mov      rax, rdi
0x140afb60a test     ecx, ecx
0x140afb60c jne      0x140afb913
0x140afb612 mov      byte ptr [rax + 1], 5
0x140afb616 jmp      0x140afb97c ; 
0x140afb61b movabs   rcx, 0x676e69646c697542
0x140afb625 cmp      qword ptr [rdx], rcx
0x140afb628 jne      0x140afb913
0x140afb62e mov      byte ptr [rax + 1], 0
0x140afb632 jmp      0x140afb97c ; 
0x140afb637 mov      ecx, 0x65746157
0x140afb63c xor      ecx, dword ptr [rdx]
0x140afb63e movzx    r9d, byte ptr [rdx + 4]
0x140afb643 xor      r9d, 0x72
0x140afb647 or       r9d, ecx
0x140afb64a jne      0x140afb913
0x140afb650 mov      byte ptr [rax + 1], 8
0x140afb654 jmp      0x140afb97c ; 
0x140afb659 movabs   rcx, 0x7a696d6f74737543
0x140afb663 xor      rcx, qword ptr [rdx]
0x140afb666 movabs   r9, 0x6e6f6974617a696d
0x140afb670 xor      r9, qword ptr [rdx + 5]
0x140afb674 or       r9, rcx
0x140afb677 je       0x140afb957
0x140afb67d movabs   rcx, 0x646f6d6f746f6850
0x140afb687 xor      rcx, qword ptr [rdx]
0x140afb68a movabs   r9, 0x7374754c65646f6d
0x140afb694 xor      r9, qword ptr [rdx + 5]
0x140afb698 or       r9, rcx
0x140afb69b je       0x140afb96f
0x140afb6a1 lea      r9, [rip + 0x1fea1cf] ; @0x142ae5877 'PhotomodeBack'
0x140afb6a8 mov      rcx, rdx
0x140afb6ab mov      rbx, rdx
0x140afb6ae mov      rdx, r9
0x140afb6b1 mov      rsi, r8
0x140afb6b4 mov      rdi, rax
0x140afb6b7 call     0x142923bf0 ; memcmp
0x140afb6bc mov      rdx, rbx
0x140afb6bf mov      r8, rsi
0x140afb6c2 mov      ecx, eax
0x140afb6c4 mov      rax, rdi
0x140afb6c7 test     ecx, ecx
0x140afb6c9 jne      0x140afb913
0x140afb6cf mov      byte ptr [rax + 1], 0x15
0x140afb6d3 jmp      0x140afb97c ; 
0x140afb6d8 movabs   rcx, 0x6f7461726f636544
0x140afb6e2 xor      rcx, qword ptr [rdx]
0x140afb6e5 movzx    r9d, word ptr [rdx + 8]
0x140afb6ea xor      r9, 0x7372
0x140afb6f1 or       r9, rcx
0x140afb6f4 jne      0x140afb913
0x140afb6fa mov      byte ptr [rax + 1], 2
0x140afb6fe jmp      0x140afb97c ; 
0x140afb703 mov      ecx, 0x73617245
0x140afb708 xor      ecx, dword ptr [rdx]
0x140afb70a movzx    r9d, word ptr [rdx + 4]
0x140afb70f xor      r9d, 0x7265
0x140afb716 or       r9d, ecx
0x140afb719 je       0x140afb963
0x140afb71f mov      rsi, rax
0x140afb722 lea      rax, [rip + 0x1fea0b4] ; @0x142ae57dd 'GardenPhotomodeTimeOfDayPhotomodeFolderPhotomodeCameraPhotomodeFovPhotomodeFocusPhotomodeDofPhotomod'
0x140afb729 mov      rdi, rdx
0x140afb72c mov      rcx, rdx
0x140afb72f mov      rdx, rax
0x140afb732 mov      rbx, r8
0x140afb735 call     0x142923bf0 ; memcmp
0x140afb73a test     eax, eax
0x140afb73c je       0x140afb975
0x140afb742 mov      r8, rbx
0x140afb745 lea      rcx, [rbx - 9]
0x140afb749 cmp      rcx, 8
0x140afb74d mov      rax, rsi
0x140afb750 mov      rdx, rdi
0x140afb753 ja       0x140afb913
0x140afb759 lea      r9, [rip + 0x1fea98c]
0x140afb760 movsxd   rcx, dword ptr [r9 + rcx*4]
0x140afb764 add      rcx, r9
0x140afb767 jmp      rcx
0x140afb769 movabs   rcx, 0x646f6d6f746f6850
0x140afb773 xor      rcx, qword ptr [rdx]
0x140afb776 movzx    r9d, byte ptr [rdx + 8]
0x140afb77b xor      r9, 0x65
0x140afb77f or       r9, rcx
0x140afb782 je       0x140afb93f
0x140afb788 movabs   rcx, 0x6144664f656d6954
0x140afb792 xor      rcx, qword ptr [rdx]
0x140afb795 movzx    r9d, byte ptr [rdx + 8]
0x140afb79a xor      r9, 0x79
0x140afb79e or       r9, rcx
0x140afb7a1 jne      0x140afb913
0x140afb7a7 mov      byte ptr [rax + 1], 0xb
0x140afb7ab jmp      0x140afb97c ; 
0x140afb7b0 movabs   rcx, 0x646f6d6f746f6850
0x140afb7ba xor      rcx, qword ptr [rdx]
0x140afb7bd movabs   r9, 0x7375636f4665646f
0x140afb7c7 xor      r9, qword ptr [rdx + 6]
0x140afb7cb or       r9, rcx
0x140afb7ce jne      0x140afb913
0x140afb7d4 mov      byte ptr [rax + 1], 0xf
0x140afb7d8 jmp      0x140afb97c ; 
0x140afb7dd movdqu   xmm0, xmmword ptr [rdx]
0x140afb7e1 movzx    ecx, byte ptr [rdx + 0x10]
0x140afb7e5 movd     xmm1, ecx
0x140afb7e9 pcmpeqb  xmm0, xmmword ptr [rip + 0x1fe9c9f] ; @0x142ae5490 'PhotomodeArtistic'
0x140afb7f1 pcmpeqb  xmm1, xmmword ptr [rip + 0x1fe9ca7]
0x140afb7f9 pand     xmm1, xmm0
0x140afb7fd pmovmskb ecx, xmm1
0x140afb801 cmp      ecx, 0xffff
0x140afb807 jne      0x140afb913
0x140afb80d mov      byte ptr [rax + 1], 0x14
0x140afb811 jmp      0x140afb97c ; 
0x140afb816 movabs   rcx, 0x646f6d6f746f6850
0x140afb820 mov      r9, qword ptr [rdx]
0x140afb823 xor      r9, rcx
0x140afb826 mov      r10d, dword ptr [rdx + 8]
0x140afb82a xor      r10, 0x766f4665
0x140afb831 or       r10, r9
0x140afb834 je       0x140afb945
0x140afb83a xor      rcx, qword ptr [rdx]
0x140afb83d mov      r9d, dword ptr [rdx + 8]
0x140afb841 xor      r9, 0x666f4465
0x140afb848 or       r9, rcx
0x140afb84b jne      0x140afb913
0x140afb851 mov      byte ptr [rax + 1], 0x10
0x140afb855 jmp      0x140afb97c ; 
0x140afb85a movdqu   xmm0, xmmword ptr [rdx]
0x140afb85e pcmpeqb  xmm0, xmmword ptr [rip + 0x1fe9c4a] ; @0x142ae54b0 'PhotomodeTakePicPhotomodeEvShift/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\\library\\std\\src\\io\\i'
0x140afb866 pmovmskb ecx, xmm0
0x140afb86a cmp      ecx, 0xffff
0x140afb870 je       0x140afb94b
0x140afb876 movdqu   xmm0, xmmword ptr [rdx]
0x140afb87a pcmpeqb  xmm0, xmmword ptr [rip + 0x1fe9c3e] ; @0x142ae54c0 'PhotomodeEvShift/rustc/05f9846f893b09a1be1fc8560e33fc3c815cfecb\\library\\std\\src\\io\\impls.rs'
0x140afb882 pmovmskb ecx, xmm0
0x140afb886 cmp      ecx, 0xffff
0x140afb88c jne      0x140afb913
0x140afb892 mov      byte ptr [rax + 1], 0x12
0x140afb896 jmp      0x140afb97c ; 
0x140afb89b movabs   rcx, 0x646f6d6f746f6850
0x140afb8a5 mov      r9, qword ptr [rdx]
0x140afb8a8 xor      r9, rcx
0x140afb8ab movabs   r10, 0x7265646c6f466564
0x140afb8b5 xor      r10, qword ptr [rdx + 7]
0x140afb8b9 or       r10, r9
0x140afb8bc je       0x140afb951
0x140afb8c2 xor      rcx, qword ptr [rdx]
0x140afb8c5 movabs   r9, 0x6172656d61436564
0x140afb8cf xor      r9, qword ptr [rdx + 7]
0x140afb8d3 or       r9, rcx
0x140afb8d6 jne      0x140afb913
0x140afb8d8 mov      byte ptr [rax + 1], 0xd
0x140afb8dc jmp      0x140afb97c ; 
0x140afb8e1 mov      ecx, 0x72726554
0x140afb8e6 xor      ecx, dword ptr [rdx]
0x140afb8e8 mov      r9d, 0x6e696172
0x140afb8ee xor      r9d, dword ptr [rdx + 3]
0x140afb8f2 or       r9d, ecx
0x140afb8f5 je       0x140afb969
0x140afb8f7 mov      ecx, 0x74696445
0x140afb8fc xor      ecx, dword ptr [rdx]
0x140afb8fe mov      r9d, 0x676e6974
0x140afb904 xor      r9d, dword ptr [rdx + 3]
0x140afb908 or       r9d, ecx
0x140afb90b jne      0x140afb913
0x140afb90d mov      byte ptr [rax + 1], 7
0x140afb911 jmp      0x140afb97c ; 
0x140afb913 lea      r10, [rip + 0x1fe9f6e]
0x140afb91a mov      r9d, 0x16
0x140afb920 mov      rcx, rdx
0x140afb923 mov      rdx, r8
0x140afb926 mov      r8, r10
0x140afb929 mov      rsi, rax
0x140afb92c call     0x1428f9e20 ; serde::de::Error::unknown_variant::h81e52211f9e120e7
0x140afb931 mov      rcx, rax
0x140afb934 mov      rax, rsi
0x140afb937 mov      qword ptr [rsi + 8], rcx
0x140afb93b mov      cl, 1
0x140afb93d jmp      0x140afb97e ; 
0x140afb93f mov      byte ptr [rax + 1], 0xa
0x140afb943 jmp      0x140afb97c ; 
0x140afb945 mov      byte ptr [rax + 1], 0xe
0x140afb949 jmp      0x140afb97c ; 
0x140afb94b mov      byte ptr [rax + 1], 0x11
0x140afb94f jmp      0x140afb97c ; 
0x140afb951 mov      byte ptr [rax + 1], 0xc
0x140afb955 jmp      0x140afb97c ; 
0x140afb957 mov      byte ptr [rax + 1], 3
0x140afb95b jmp      0x140afb97c ; 
0x140afb95d mov      byte ptr [rax + 1], 1
0x140afb961 jmp      0x140afb97c ; 
0x140afb963 mov      byte ptr [rax + 1], 4
0x140afb967 jmp      0x140afb97c ; 
0x140afb969 mov      byte ptr [rax + 1], 6
0x140afb96d jmp      0x140afb97c ; 
0x140afb96f mov      byte ptr [rax + 1], 0x13
0x140afb973 jmp      0x140afb97c ; 
0x140afb975 mov      rax, rsi
0x140afb978 mov      byte ptr [rsi + 1], 9
0x140afb97c xor      ecx, ecx
0x140afb97e mov      byte ptr [rax], cl
0x140afb980 add      rsp, 0x20
0x140afb984 pop      rbx
0x140afb985 pop      rdi
0x140afb986 pop      rsi
0x140afb987 ret      