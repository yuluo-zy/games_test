0x140afbe10 push     rsi
0x140afbe11 sub      rsp, 0x20
0x140afbe15 mov      rax, rcx
0x140afbe18 lea      rcx, [r8 - 3]
0x140afbe1c cmp      rcx, 5
0x140afbe20 ja       0x140afbed8
0x140afbe26 lea      r9, [rip + 0x1fea31b]
0x140afbe2d movsxd   rcx, dword ptr [r9 + rcx*4]
0x140afbe31 add      rcx, r9
0x140afbe34 jmp      rcx
0x140afbe36 movzx    ecx, word ptr [rdx]
0x140afbe39 xor      ecx, 0x6c41
0x140afbe3f movzx    r9d, byte ptr [rdx + 2]
0x140afbe44 xor      r9d, 0x6c
0x140afbe48 or       r9w, cx
0x140afbe4c jne      0x140afbed8
0x140afbe52 mov      byte ptr [rax + 1], 0
0x140afbe56 jmp      0x140afbece ; 
0x140afbe58 cmp      dword ptr [rdx], 0x6c6c6157
0x140afbe5e je       0x140afbec4
0x140afbe60 cmp      dword ptr [rdx], 0x68746150
0x140afbe66 je       0x140afbeca
0x140afbe68 cmp      dword ptr [rdx], 0x65657254
0x140afbe6e jne      0x140afbed8
0x140afbe70 mov      byte ptr [rax + 1], 6
0x140afbe74 jmp      0x140afbece ; 
0x140afbe76 movabs   rcx, 0x657a6f646c6c7542
0x140afbe80 cmp      qword ptr [rdx], rcx
0x140afbe83 jne      0x140afbed8
0x140afbe85 mov      byte ptr [rax + 1], 3
0x140afbe89 jmp      0x140afbece ; 
0x140afbe8b mov      ecx, 0x64726147
0x140afbe90 xor      ecx, dword ptr [rdx]
0x140afbe92 movzx    r9d, word ptr [rdx + 4]
0x140afbe97 xor      r9d, 0x6e65
0x140afbe9e or       r9d, ecx
0x140afbea1 jne      0x140afbed8
0x140afbea3 mov      byte ptr [rax + 1], 5
0x140afbea7 jmp      0x140afbece ; 
0x140afbea9 mov      ecx, 0x65746157
0x140afbeae xor      ecx, dword ptr [rdx]
0x140afbeb0 movzx    r9d, byte ptr [rdx + 4]
0x140afbeb5 xor      r9d, 0x72
0x140afbeb9 or       r9d, ecx
0x140afbebc jne      0x140afbed8
0x140afbebe mov      byte ptr [rax + 1], 4
0x140afbec2 jmp      0x140afbece ; 
0x140afbec4 mov      byte ptr [rax + 1], 1
0x140afbec8 jmp      0x140afbece ; 
0x140afbeca mov      byte ptr [rax + 1], 2
0x140afbece xor      ecx, ecx
0x140afbed0 mov      byte ptr [rax], cl
0x140afbed2 add      rsp, 0x20
0x140afbed6 pop      rsi
0x140afbed7 ret      
0x140afbed8 lea      r10, [rip + 0x1fe9b49]
0x140afbedf mov      r9d, 7
0x140afbee5 mov      rcx, rdx
0x140afbee8 mov      rdx, r8
0x140afbeeb mov      r8, r10
0x140afbeee mov      rsi, rax
0x140afbef1 call     0x1428f9e20 ; serde::de::Error::unknown_variant::h81e52211f9e120e7
0x140afbef6 mov      rcx, rax
0x140afbef9 mov      rax, rsi
0x140afbefc mov      qword ptr [rsi + 8], rcx
0x140afbf00 mov      cl, 1
0x140afbf02 jmp      0x140afbed0 ; 