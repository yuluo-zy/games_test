0x140a48880 push     rsi
0x140a48881 sub      rsp, 0x20
0x140a48885 mov      rax, rcx
0x140a48888 lea      rcx, [r8 - 4]
0x140a4888c cmp      rcx, 9
0x140a48890 ja       0x140a489ae
0x140a48896 lea      r9, [rip + 0x2086f2b]
0x140a4889d movsxd   rcx, dword ptr [r9 + rcx*4]
0x140a488a1 add      rcx, r9
0x140a488a4 jmp      rcx
0x140a488a6 cmp      dword ptr [rdx], 0x67616c46
0x140a488ac jne      0x140a489ae
0x140a488b2 mov      byte ptr [rax + 1], 5
0x140a488b6 jmp      0x140a489a4 ; 
0x140a488bb movabs   rcx, 0x5765676174746f43
0x140a488c5 xor      rcx, qword ptr [rdx]
0x140a488c8 movabs   r9, 0x776f646e69576567
0x140a488d2 xor      r9, qword ptr [rdx + 5]
0x140a488d6 or       r9, rcx
0x140a488d9 jne      0x140a489ae
0x140a488df mov      byte ptr [rax + 1], 0
0x140a488e3 jmp      0x140a489a4 ; 
0x140a488e8 mov      ecx, 0x6d696843
0x140a488ed xor      ecx, dword ptr [rdx]
0x140a488ef mov      r9d, 0x79656e6d
0x140a488f5 xor      r9d, dword ptr [rdx + 3]
0x140a488f9 or       r9d, ecx
0x140a488fc je       0x140a4899a
0x140a48902 mov      ecx, 0x746e614c
0x140a48907 xor      ecx, dword ptr [rdx]
0x140a48909 mov      r9d, 0x6e726574
0x140a4890f xor      r9d, dword ptr [rdx + 3]
0x140a48913 or       r9d, ecx
0x140a48916 je       0x140a489a0
0x140a4891c mov      ecx, 0x74756c43
0x140a48921 xor      ecx, dword ptr [rdx]
0x140a48923 mov      r9d, 0x72657474
0x140a48929 xor      r9d, dword ptr [rdx + 3]
0x140a4892d or       r9d, ecx
0x140a48930 jne      0x140a489ae
0x140a48932 mov      byte ptr [rax + 1], 6
0x140a48936 jmp      0x140a489a4 ; 
0x140a48938 movabs   rcx, 0x6957636968746f47
0x140a48942 xor      rcx, qword ptr [rdx]
0x140a48945 mov      r9d, dword ptr [rdx + 8]
0x140a48949 xor      r9, 0x776f646e
0x140a48950 or       r9, rcx
0x140a48953 jne      0x140a489ae
0x140a48955 mov      byte ptr [rax + 1], 1
0x140a48959 jmp      0x140a489a4 ; 
0x140a4895b movabs   rcx, 0x696c53776f727241
0x140a48965 xor      rcx, qword ptr [rdx]
0x140a48968 movzx    r9d, byte ptr [rdx + 8]
0x140a4896d xor      r9, 0x74
0x140a48971 or       r9, rcx
0x140a48974 jne      0x140a489ae
0x140a48976 mov      byte ptr [rax + 1], 2
0x140a4897a jmp      0x140a489a4 ; 
0x140a4897c mov      ecx, 0x69617453
0x140a48981 xor      ecx, dword ptr [rdx]
0x140a48983 movzx    r9d, word ptr [rdx + 4]
0x140a48988 xor      r9d, 0x7372
0x140a4898f or       r9d, ecx
0x140a48992 jne      0x140a489ae
0x140a48994 mov      byte ptr [rax + 1], 7
0x140a48998 jmp      0x140a489a4 ; 
0x140a4899a mov      byte ptr [rax + 1], 3
0x140a4899e jmp      0x140a489a4 ; 
0x140a489a0 mov      byte ptr [rax + 1], 4
0x140a489a4 xor      ecx, ecx
0x140a489a6 mov      byte ptr [rax], cl
0x140a489a8 add      rsp, 0x20
0x140a489ac pop      rsi
0x140a489ad ret      
0x140a489ae lea      r10, [rip + 0x20863eb]
0x140a489b5 mov      r9d, 8
0x140a489bb mov      rcx, rdx
0x140a489be mov      rdx, r8
0x140a489c1 mov      r8, r10
0x140a489c4 mov      rsi, rax
0x140a489c7 call     0x1428f9e20 ; serde::de::Error::unknown_variant::h81e52211f9e120e7
0x140a489cc mov      rcx, rax
0x140a489cf mov      rax, rsi
0x140a489d2 mov      qword ptr [rsi + 8], rcx
0x140a489d6 mov      cl, 1
0x140a489d8 jmp      0x140a489a6 ; 