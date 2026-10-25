0x14179b870 push     r15
0x14179b872 push     r14
0x14179b874 push     r13
0x14179b876 push     r12
0x14179b878 push     rsi
0x14179b879 push     rdi
0x14179b87a push     rbp
0x14179b87b push     rbx
0x14179b87c sub      rsp, 0x38
0x14179b880 mov      rdi, r8
0x14179b883 mov      rbx, rdx
0x14179b886 mov      rsi, rcx
0x14179b889 cmp      r8, 0xc
0x14179b88d jne      0x14179b931
0x14179b893 movabs   rax, 0x6f705f746e616c70
0x14179b89d xor      rax, qword ptr [rbx]
0x14179b8a0 mov      ecx, dword ptr [rbx + 8]
0x14179b8a3 xor      rcx, 0x31765f74
0x14179b8aa or       rcx, rax
0x14179b8ad je       0x14179b974
0x14179b8b3 movabs   rax, 0x676e697265746177
0x14179b8bd xor      rax, qword ptr [rbx]
0x14179b8c0 mov      ecx, dword ptr [rbx + 8]
0x14179b8c3 xor      rcx, 0x6e61635f
0x14179b8ca or       rcx, rax
0x14179b8cd je       0x14179b97d
0x14179b8d3 lea      rdx, [rip + 0x153ecf0] ; @0x142cda5ca 'plant_pot_v2birdhousebasketbasket_w_foodbarrel_w_food_basketpotato_sackroad_signsunflowercartcart2cr'
0x14179b8da mov      rcx, rbx
0x14179b8dd mov      r8, rdi
0x14179b8e0 call     0x142923bf0 ; memcmp
0x14179b8e5 test     eax, eax
0x14179b8e7 je       0x14179b986
0x14179b8ed mov      dword ptr [rsp + 0x2c], 0
0x14179b8f5 xor      r13d, r13d
0x14179b8f8 xor      r12d, r12d
0x14179b8fb xor      ebp, ebp
0x14179b8fd mov      dword ptr [rsp + 0x30], 0
0x14179b905 xor      r15d, r15d
0x14179b908 movabs   rax, 0x725f636972626166
0x14179b912 xor      rax, qword ptr [rbx]
0x14179b915 mov      ecx, dword ptr [rbx + 8]
0x14179b918 xor      rcx, 0x736c6c6f
0x14179b91f or       rcx, rax
0x14179b922 jne      0x14179c034
0x14179b928 mov      byte ptr [rsi + 1], 0x2d
0x14179b92c jmp      0x14179c239 ; 
0x14179b931 xor      eax, eax
0x14179b933 lea      rcx, [rdi - 4]
0x14179b937 cmp      rcx, 0x10
0x14179b93b ja       0x14179bd14
0x14179b941 lea      rdx, [rip + 0x153f900]
0x14179b948 movsxd   rcx, dword ptr [rdx + rcx*4]
0x14179b94c add      rcx, rdx
0x14179b94f xor      ebp, ebp
0x14179b951 jmp      rcx
0x14179b953 cmp      dword ptr [rbx], 0x74726163
0x14179b959 je       0x14179be42
0x14179b95f cmp      dword ptr [rbx], 0x64656572
0x14179b965 jne      0x14179bc5f
0x14179b96b mov      byte ptr [rsi + 1], 0x1e
0x14179b96f jmp      0x14179c239 ; 
0x14179b974 mov      byte ptr [rsi + 1], 0
0x14179b978 jmp      0x14179c239 ; 
0x14179b97d mov      byte ptr [rsi + 1], 6
0x14179b981 jmp      0x14179c239 ; 
0x14179b986 mov      byte ptr [rsi + 1], 0xf
0x14179b98a jmp      0x14179c239 ; 
0x14179b98f mov      eax, 0x6464616c
0x14179b994 xor      eax, dword ptr [rbx]
0x14179b996 movzx    ecx, word ptr [rbx + 4]
0x14179b99a xor      ecx, 0x7265
0x14179b9a0 or       ecx, eax
0x14179b9a2 je       0x14179be30
0x14179b9a8 mov      eax, 0x766f6873
0x14179b9ad xor      eax, dword ptr [rbx]
0x14179b9af movzx    ecx, word ptr [rbx + 4]
0x14179b9b3 xor      ecx, 0x6c65
0x14179b9b9 or       ecx, eax
0x14179b9bb je       0x14179bfcb
0x14179b9c1 mov      eax, 0x6b637562
0x14179b9c6 xor      eax, dword ptr [rbx]
0x14179b9c8 movzx    ecx, word ptr [rbx + 4]
0x14179b9cc xor      ecx, 0x7465
0x14179b9d2 or       ecx, eax
0x14179b9d4 je       0x14179c1d5
0x14179b9da lea      rdx, [rip + 0x153ebfe] ; @0x142cda5df 'basketbasket_w_foodbarrel_w_food_basketpotato_sackroad_signsunflowercartcart2crates_w_toolscrates_w_'
0x14179b9e1 mov      rcx, rbx
0x14179b9e4 mov      r8, rdi
0x14179b9e7 call     0x142923bf0 ; memcmp
0x14179b9ec test     eax, eax
0x14179b9ee jne      0x14179bd12
0x14179b9f4 mov      byte ptr [rsi + 1], 0x11
0x14179b9f8 jmp      0x14179c239 ; 
0x14179b9fd movabs   rax, 0x74735f775f786f62
0x14179ba07 xor      rax, qword ptr [rbx]
0x14179ba0a movabs   rcx, 0x66667574735f775f
0x14179ba14 xor      rcx, qword ptr [rbx + 3]
0x14179ba18 or       rcx, rax
0x14179ba1b je       0x14179be39
0x14179ba21 movabs   rax, 0x68735f68636e6562
0x14179ba2b xor      rax, qword ptr [rbx]
0x14179ba2e movabs   rcx, 0x74726f68735f6863
0x14179ba38 xor      rcx, qword ptr [rbx + 3]
0x14179ba3c or       rcx, rax
0x14179ba3f je       0x14179bfd4
0x14179ba45 lea      rdx, [rip + 0x153ebba] ; @0x142cda606 'potato_sackroad_signsunflowercartcart2crates_w_toolscrates_w_flowerswindow_flowerpotwindow_shutterwi'
0x14179ba4c mov      rcx, rbx
0x14179ba4f mov      r8, rdi
0x14179ba52 call     0x142923bf0 ; memcmp
0x14179ba57 test     eax, eax
0x14179ba59 je       0x14179c1db
0x14179ba5f mov      r15b, 1
0x14179ba62 xor      edx, edx
0x14179ba64 xor      r14d, r14d
0x14179ba67 jmp      0x14179bea2 ; 
0x14179ba6c mov      eax, 0x65656877
0x14179ba71 xor      eax, dword ptr [rbx]
0x14179ba73 movzx    ecx, byte ptr [rbx + 4]
0x14179ba77 xor      ecx, 0x6c
0x14179ba7a or       ecx, eax
0x14179ba7c je       0x14179be4b
0x14179ba82 mov      eax, 0x636e6562
0x14179ba87 xor      eax, dword ptr [rbx]
0x14179ba89 movzx    ecx, byte ptr [rbx + 4]
0x14179ba8d xor      ecx, 0x68
0x14179ba90 or       ecx, eax
0x14179ba92 je       0x14179bfdd
0x14179ba98 lea      rdx, [rip + 0x153eb88] ; @0x142cda627 'cart2crates_w_toolscrates_w_flowerswindow_flowerpotwindow_shutterwindow_flowerbedsreedlilypadfallen_'
0x14179ba9f mov      rcx, rbx
0x14179baa2 mov      r8, rdi
0x14179baa5 call     0x142923bf0 ; memcmp
0x14179baaa test     eax, eax
0x14179baac je       0x14179c1e1
0x14179bab2 mov      dl, 1
0x14179bab4 xor      ebp, ebp
0x14179bab6 jmp      0x14179bd8f ; 
0x14179babb movabs   rax, 0x775f74656b736162
0x14179bac5 xor      rax, qword ptr [rbx]
0x14179bac8 movabs   rcx, 0x646f6f665f775f74
0x14179bad2 xor      rcx, qword ptr [rbx + 5]
0x14179bad6 or       rcx, rax
0x14179bad9 je       0x14179be54
0x14179badf mov      bpl, 1
0x14179bae2 xor      eax, eax
0x14179bae4 cmp      rdi, 0x10
0x14179bae8 jne      0x14179bd20
0x14179baee movdqu   xmm0, xmmword ptr [rbx]
0x14179baf2 pcmpeqb  xmm0, xmmword ptr [rip + 0x153e946] ; @0x142cda440 'crates_w_flowerswindow_flowerpotcrates_w_pumpkinmap with a single key'
0x14179bafa pmovmskb ecx, xmm0
0x14179bafe cmp      ecx, 0xffff
0x14179bb04 je       0x14179bdc8
0x14179bb0a movdqu   xmm0, xmmword ptr [rbx]
0x14179bb0e pcmpeqb  xmm0, xmmword ptr [rip + 0x153e93a] ; @0x142cda450 'window_flowerpotcrates_w_pumpkinmap with a single key'
0x14179bb16 pmovmskb ecx, xmm0
0x14179bb1a cmp      ecx, 0xffff
0x14179bb20 je       0x14179be8a
0x14179bb26 movdqu   xmm0, xmmword ptr [rbx]
0x14179bb2a pcmpeqb  xmm0, xmmword ptr [rip + 0x153e8fe] ; @0x142cda430 'olden_tree_stumpcrates_w_flowerswindow_flowerpotcrates_w_pumpkinmap with a single key'
0x14179bb32 pmovmskb ecx, xmm0
0x14179bb36 cmp      ecx, 0xffff
0x14179bb3c je       0x14179c1bd
0x14179bb42 mov      cl, 1
0x14179bb44 xor      r15d, r15d
0x14179bb47 mov      dword ptr [rsp + 0x30], 0
0x14179bb4f mov      dword ptr [rsp + 0x34], 0
0x14179bb57 mov      r12d, 0
0x14179bb5d mov      r13d, 0
0x14179bb63 test     al, al
0x14179bb65 jne      0x14179bf1f
0x14179bb6b jmp      0x14179bf4a ; 
0x14179bb70 movabs   rax, 0x5f775f68636e6562
0x14179bb7a xor      rax, qword ptr [rbx]
0x14179bb7d movabs   rcx, 0x74656b7361625f77
0x14179bb87 xor      rcx, qword ptr [rbx + 6]
0x14179bb8b or       rcx, rax
0x14179bb8e je       0x14179be5d
0x14179bb94 lea      rdx, [rip + 0x153ea91] ; @0x142cda62c 'crates_w_toolscrates_w_flowerswindow_flowerpotwindow_shutterwindow_flowerbedsreedlilypadfallen_leave'
0x14179bb9b mov      rcx, rbx
0x14179bb9e mov      r8, rdi
0x14179bba1 call     0x142923bf0 ; memcmp
0x14179bba6 test     eax, eax
0x14179bba8 je       0x14179bfe6
0x14179bbae lea      rdx, [rip + 0x153eaa5] ; @0x142cda65a 'window_shutterwindow_flowerbedsreedlilypadfallen_leavesfallen_petalsolden_rockolden_tree_logolden_tr'
0x14179bbb5 mov      rcx, rbx
0x14179bbb8 mov      r8, rdi
0x14179bbbb call     0x142923bf0 ; memcmp
0x14179bbc0 test     eax, eax
0x14179bbc2 je       0x14179c1e7
0x14179bbc8 lea      rdx, [rip + 0x153ead9] ; @0x142cda6a8 'olden_tree_logolden_tree_stumppumpkinpumpkin2pumpkin_w_facecart_w_pumpkinscrates_w_pumpkinsolden_tre'
0x14179bbcf mov      rcx, rbx
0x14179bbd2 mov      r8, rdi
0x14179bbd5 call     0x142923bf0 ; memcmp
0x14179bbda test     eax, eax
0x14179bbdc je       0x14179c21d
0x14179bbe2 lea      rdx, [rip + 0x153eaec] ; @0x142cda6d5 'pumpkin_w_facecart_w_pumpkinscrates_w_pumpkinsolden_tree_stump_autumnbarrel_w_candlesanvilfabric_rol'
0x14179bbe9 mov      rcx, rbx
0x14179bbec mov      r8, rdi
0x14179bbef call     0x142923bf0 ; memcmp
0x14179bbf4 test     eax, eax
0x14179bbf6 jne      0x14179c251
0x14179bbfc mov      byte ptr [rsi + 1], 0x27
0x14179bc00 jmp      0x14179c239 ; 
0x14179bc05 movabs   rax, 0x646f6f7765726966
0x14179bc0f cmp      qword ptr [rbx], rax
0x14179bc12 je       0x14179be66
0x14179bc18 movabs   rax, 0x7473656e64726962
0x14179bc22 cmp      qword ptr [rbx], rax
0x14179bc25 je       0x14179bfef
0x14179bc2b lea      rdx, [rip + 0x153e990] ; @0x142cda5c2 'haystackplant_pot_v2birdhousebasketbasket_w_foodbarrel_w_food_basketpotato_sackroad_signsunflowercar'
0x14179bc32 mov      rcx, rbx
0x14179bc35 mov      r8, rdi
0x14179bc38 call     0x142923bf0 ; memcmp
0x14179bc3d test     eax, eax
0x14179bc3f je       0x14179c1ed
0x14179bc45 lea      rdx, [rip + 0x153ea81] ; @0x142cda6cd 'pumpkin2pumpkin_w_facecart_w_pumpkinscrates_w_pumpkinsolden_tree_stump_autumnbarrel_w_candlesanvilfa'
0x14179bc4c mov      rcx, rbx
0x14179bc4f mov      r8, rdi
0x14179bc52 call     0x142923bf0 ; memcmp
0x14179bc57 test     eax, eax
0x14179bc59 je       0x14179c223
0x14179bc5f xor      r15d, r15d
0x14179bc62 xor      r14d, r14d
0x14179bc65 xor      edx, edx
0x14179bc67 jmp      0x14179bed3 ; 
0x14179bc6c movabs   rax, 0x726f666863746970
0x14179bc76 xor      rax, qword ptr [rbx]
0x14179bc79 movzx    ecx, byte ptr [rbx + 8]
0x14179bc7d xor      rcx, 0x6b
0x14179bc81 or       rcx, rax
0x14179bc84 je       0x14179be6f
0x14179bc8a movabs   rax, 0x73756f6864726962
0x14179bc94 xor      rax, qword ptr [rbx]
0x14179bc97 movzx    ecx, byte ptr [rbx + 8]
0x14179bc9b xor      rcx, 0x65
0x14179bc9f or       rcx, rax
0x14179bca2 je       0x14179bff8
0x14179bca8 lea      rdx, [rip + 0x153e962] ; @0x142cda611 'road_signsunflowercartcart2crates_w_toolscrates_w_flowerswindow_flowerpotwindow_shutterwindow_flower'
0x14179bcaf mov      rcx, rbx
0x14179bcb2 mov      r8, rdi
0x14179bcb5 call     0x142923bf0 ; memcmp
0x14179bcba test     eax, eax
0x14179bcbc je       0x14179c1f3
0x14179bcc2 lea      rdx, [rip + 0x153e951] ; @0x142cda61a 'sunflowercartcart2crates_w_toolscrates_w_flowerswindow_flowerpotwindow_shutterwindow_flowerbedsreedl'
0x14179bcc9 mov      rcx, rbx
0x14179bccc mov      r8, rdi
0x14179bccf call     0x142923bf0 ; memcmp
0x14179bcd4 test     eax, eax
0x14179bcd6 je       0x14179c229
0x14179bcdc mov      r14b, 1
0x14179bcdf xor      edx, edx
0x14179bce1 jmp      0x14179be9f ; 
0x14179bce6 movdqu   xmm0, xmmword ptr [rbx]
0x14179bcea movd     xmm1, dword ptr [rbx + 0x10]
0x14179bcef pcmpeqb  xmm1, xmmword ptr [rip + 0x153e6f9] ; @0x142cda3f0 'sket'
0x14179bcf7 pcmpeqb  xmm0, xmmword ptr [rip + 0x153e701] ; @0x142cda400 'barrel_w_food_babarrel_w_plantporee_stump_autumnolden_tree_stumpcrates_w_flowerswindow_flowerpotcrat'
0x14179bcff pand     xmm0, xmm1
0x14179bd03 pmovmskb eax, xmm0
0x14179bd07 cmp      eax, 0xffff
0x14179bd0c je       0x14179be78
0x14179bd12 xor      eax, eax
0x14179bd14 xor      ebp, ebp
0x14179bd16 cmp      rdi, 0x10
0x14179bd1a je       0x14179baee
0x14179bd20 test     al, al
0x14179bd22 je       0x14179bd8d
0x14179bd24 lea      rdx, [rip + 0x153e93d] ; @0x142cda668 'window_flowerbedsreedlilypadfallen_leavesfallen_petalsolden_rockolden_tree_logolden_tree_stumppumpki'
0x14179bd2b mov      rcx, rbx
0x14179bd2e mov      r8, rdi
0x14179bd31 call     0x142923bf0 ; memcmp
0x14179bd36 test     eax, eax
0x14179bd38 je       0x14179be93
0x14179bd3e mov      r13b, 1
0x14179bd41 xor      r15d, r15d
0x14179bd44 xor      r14d, r14d
0x14179bd47 xor      edx, edx
0x14179bd49 jmp      0x14179bed8 ; 
0x14179bd4e movdqu   xmm0, xmmword ptr [rbx]
0x14179bd52 movzx    eax, byte ptr [rbx + 0x10]
0x14179bd56 movd     xmm1, eax
0x14179bd5a pcmpeqb  xmm0, xmmword ptr [rip + 0x153e6ae] ; @0x142cda410 'barrel_w_plantporee_stump_autumnolden_tree_stumpcrates_w_flowerswindow_flowerpotcrates_w_pumpkinmap '
0x14179bd62 pcmpeqb  xmm1, xmmword ptr [rip + 0x11a1bf6]
0x14179bd6a pand     xmm1, xmm0
0x14179bd6e pmovmskb eax, xmm1
0x14179bd72 cmp      eax, 0xffff
0x14179bd77 je       0x14179be81
0x14179bd7d mov      al, 1
0x14179bd7f xor      ebp, ebp
0x14179bd81 cmp      rdi, 0x10
0x14179bd85 je       0x14179baee
0x14179bd8b jmp      0x14179bd20 ; 
0x14179bd8d xor      edx, edx
0x14179bd8f cmp      rdi, 7
0x14179bd93 jne      0x14179bdd1
0x14179bd95 mov      eax, 0x796c696c
0x14179bd9a xor      eax, dword ptr [rbx]
0x14179bd9c mov      ecx, 0x64617079
0x14179bda1 xor      ecx, dword ptr [rbx + 3]
0x14179bda4 or       ecx, eax
0x14179bda6 je       0x14179c1c9
0x14179bdac mov      eax, 0x706d7570
0x14179bdb1 xor      eax, dword ptr [rbx]
0x14179bdb3 mov      ecx, 0x6e696b70
0x14179bdb8 xor      ecx, dword ptr [rbx + 3]
0x14179bdbb or       ecx, eax
0x14179bdbd jne      0x14179be25
0x14179bdbf mov      byte ptr [rsi + 1], 0x25
0x14179bdc3 jmp      0x14179c239 ; 
0x14179bdc8 mov      byte ptr [rsi + 1], 0x1a
0x14179bdcc jmp      0x14179c239 ; 
0x14179bdd1 test     bpl, bpl
0x14179bdd4 je       0x14179be9c
0x14179bdda movabs   rax, 0x6c5f6e656c6c6166
0x14179bde4 xor      rax, qword ptr [rbx]
0x14179bde7 movabs   rcx, 0x73657661656c5f6e
0x14179bdf1 xor      rcx, qword ptr [rbx + 5]
0x14179bdf5 or       rcx, rax
0x14179bdf8 je       0x14179c217
0x14179bdfe movabs   rax, 0x705f6e656c6c6166
0x14179be08 xor      rax, qword ptr [rbx]
0x14179be0b movabs   rcx, 0x736c617465705f6e
0x14179be15 xor      rcx, qword ptr [rbx + 5]
0x14179be19 or       rcx, rax
0x14179be1c je       0x14179c235
0x14179be22 mov      bpl, 1
0x14179be25 xor      r15d, r15d
0x14179be28 xor      r14d, r14d
0x14179be2b jmp      0x14179bed5 ; 
0x14179be30 mov      byte ptr [rsi + 1], 1
0x14179be34 jmp      0x14179c239 ; 
0x14179be39 mov      byte ptr [rsi + 1], 5
0x14179be3d jmp      0x14179c239 ; 
0x14179be42 mov      byte ptr [rsi + 1], 0x17
0x14179be46 jmp      0x14179c239 ; 
0x14179be4b mov      byte ptr [rsi + 1], 2
0x14179be4f jmp      0x14179c239 ; 
0x14179be54 mov      byte ptr [rsi + 1], 0x12
0x14179be58 jmp      0x14179c239 ; 
0x14179be5d mov      byte ptr [rsi + 1], 0xb
0x14179be61 jmp      0x14179c239 ; 
0x14179be66 mov      byte ptr [rsi + 1], 7
0x14179be6a jmp      0x14179c239 ; 
0x14179be6f mov      byte ptr [rsi + 1], 3
0x14179be73 jmp      0x14179c239 ; 
0x14179be78 mov      byte ptr [rsi + 1], 0x13
0x14179be7c jmp      0x14179c239 ; 
0x14179be81 mov      byte ptr [rsi + 1], 0xd
0x14179be85 jmp      0x14179c239 ; 
0x14179be8a mov      byte ptr [rsi + 1], 0x1b
0x14179be8e jmp      0x14179c239 ; 
0x14179be93 mov      byte ptr [rsi + 1], 0x1d
0x14179be97 jmp      0x14179c239 ; 
0x14179be9c xor      r14d, r14d
0x14179be9f xor      r15d, r15d
0x14179bea2 cmp      rdi, 0xa
0x14179bea6 jne      0x14179bed3
0x14179bea8 movabs   rax, 0x6f725f6e65646c6f
0x14179beb2 xor      rax, qword ptr [rbx]
0x14179beb5 movzx    ecx, word ptr [rbx + 8]
0x14179beb9 xor      rcx, 0x6b63
0x14179bec0 or       rcx, rax
0x14179bec3 je       0x14179c205
0x14179bec9 mov      r12b, 1
0x14179becc xor      ebp, ebp
0x14179bece xor      r13d, r13d
0x14179bed1 jmp      0x14179bedb ; 
0x14179bed3 xor      ebp, ebp
0x14179bed5 xor      r13d, r13d
0x14179bed8 xor      r12d, r12d
0x14179bedb cmp      rdi, 0xf
0x14179bedf jne      0x14179bf0f
0x14179bee1 movabs   rax, 0x705f775f74726163
0x14179beeb xor      rax, qword ptr [rbx]
0x14179beee movabs   rcx, 0x736e696b706d7570
0x14179bef8 xor      rcx, qword ptr [rbx + 7]
0x14179befc or       rcx, rax
0x14179beff je       0x14179bfb9
0x14179bf05 mov      dword ptr [rsp + 0x30], r14d
0x14179bf0a jmp      0x14179c004 ; 
0x14179bf0f test     r13b, r13b
0x14179bf12 mov      dword ptr [rsp + 0x30], r14d
0x14179bf17 je       0x14179bf7c
0x14179bf19 mov      dword ptr [rsp + 0x34], edx
0x14179bf1d xor      ecx, ecx
0x14179bf1f mov      r14d, r12d
0x14179bf22 mov      r12d, ecx
0x14179bf25 lea      rdx, [rip + 0x153e7c6] ; @0x142cda6f2 'crates_w_pumpkinsolden_tree_stump_autumnbarrel_w_candlesanvilfabric_rollsstall_breadstall_veggiescra'
0x14179bf2c mov      rcx, rbx
0x14179bf2f mov      r8, rdi
0x14179bf32 call     0x142923bf0 ; memcmp
0x14179bf37 test     eax, eax
0x14179bf39 je       0x14179bfc2
0x14179bf3f mov      r13b, 1
0x14179bf42 test     r12b, r12b
0x14179bf45 mov      r12d, r14d
0x14179bf48 je       0x14179bf64
0x14179bf4a lea      rdx, [rip + 0x153e7c9] ; @0x142cda71a 'barrel_w_candlesanvilfabric_rollsstall_breadstall_veggiescrate_fishrack_broomsflowerboxdoor_belldoor'
0x14179bf51 mov      rcx, rbx
0x14179bf54 mov      r8, rdi
0x14179bf57 call     0x142923bf0 ; memcmp
0x14179bf5c test     eax, eax
0x14179bf5e je       0x14179c1f9
0x14179bf64 mov      dword ptr [rsp + 0x2c], 0
0x14179bf6c cmp      byte ptr [rsp + 0x34], 0
0x14179bf71 jne      0x14179c010
0x14179bf77 jmp      0x14179c02a ; 
0x14179bf7c cmp      rdi, 0x17
0x14179bf80 jne      0x14179c001
0x14179bf82 movdqu   xmm0, xmmword ptr [rbx]
0x14179bf86 movdqu   xmm1, xmmword ptr [rbx + 7]
0x14179bf8b pcmpeqb  xmm1, xmmword ptr [rip + 0x153e48d] ; @0x142cda420 'ree_stump_autumnolden_tree_stumpcrates_w_flowerswindow_flowerpotcrates_w_pumpkinmap with a single ke'
0x14179bf93 pcmpeqb  xmm0, xmmword ptr [rip + 0x153e495] ; @0x142cda430 'olden_tree_stumpcrates_w_flowerswindow_flowerpotcrates_w_pumpkinmap with a single key'
0x14179bf9b pand     xmm0, xmm1
0x14179bf9f pmovmskb eax, xmm0
0x14179bfa3 cmp      eax, 0xffff
0x14179bfa8 je       0x14179c22f
0x14179bfae mov      al, 1
0x14179bfb0 mov      dword ptr [rsp + 0x2c], eax
0x14179bfb4 xor      r13d, r13d
0x14179bfb7 jmp      0x14179c00c ; 
0x14179bfb9 mov      byte ptr [rsi + 1], 0x28
0x14179bfbd jmp      0x14179c239 ; 
0x14179bfc2 mov      byte ptr [rsi + 1], 0x29
0x14179bfc6 jmp      0x14179c239 ; 
0x14179bfcb mov      byte ptr [rsi + 1], 4
0x14179bfcf jmp      0x14179c239 ; 
0x14179bfd4 mov      byte ptr [rsi + 1], 0xc
0x14179bfd8 jmp      0x14179c239 ; 
0x14179bfdd mov      byte ptr [rsi + 1], 0xa
0x14179bfe1 jmp      0x14179c239 ; 
0x14179bfe6 mov      byte ptr [rsi + 1], 0x19
0x14179bfea jmp      0x14179c239 ; 
0x14179bfef mov      byte ptr [rsi + 1], 9
0x14179bff3 jmp      0x14179c239 ; 
0x14179bff8 mov      byte ptr [rsi + 1], 0x10
0x14179bffc jmp      0x14179c239 ; 
0x14179c001 xor      r13d, r13d
0x14179c004 mov      dword ptr [rsp + 0x2c], 0
0x14179c00c test     dl, dl
0x14179c00e je       0x14179c02a
0x14179c010 lea      rdx, [rip + 0x153e713] ; @0x142cda72a 'anvilfabric_rollsstall_breadstall_veggiescrate_fishrack_broomsflowerboxdoor_belldoor_kransdoor_krans'
0x14179c017 mov      rcx, rbx
0x14179c01a mov      r8, rdi
0x14179c01d call     0x142923bf0 ; memcmp
0x14179c022 test     eax, eax
0x14179c024 je       0x14179c1b7
0x14179c02a cmp      rdi, 0xc
0x14179c02e je       0x14179b908
0x14179c034 test     r15b, r15b
0x14179c037 je       0x14179c053
0x14179c039 lea      rdx, [rip + 0x153e6fb] ; @0x142cda73b 'stall_breadstall_veggiescrate_fishrack_broomsflowerboxdoor_belldoor_kransdoor_krans_autumncart_w_pum'
0x14179c040 mov      rcx, rbx
0x14179c043 mov      r8, rdi
0x14179c046 call     0x142923bf0 ; memcmp
0x14179c04b test     eax, eax
0x14179c04d je       0x14179c193
0x14179c053 test     bpl, bpl
0x14179c056 je       0x14179c07c
0x14179c058 movabs   rax, 0x65765f6c6c617473
0x14179c062 xor      rax, qword ptr [rbx]
0x14179c065 movabs   rcx, 0x736569676765765f
0x14179c06f xor      rcx, qword ptr [rbx + 5]
0x14179c073 or       rcx, rax
0x14179c076 je       0x14179c19c
0x14179c07c test     r12b, r12b
0x14179c07f je       0x14179c09b
0x14179c081 lea      rdx, [rip + 0x153e6cb] ; @0x142cda753 'crate_fishrack_broomsflowerboxdoor_belldoor_kransdoor_krans_autumncart_w_pumpkins_regularcrates_w_pu'
0x14179c088 mov      rcx, rbx
0x14179c08b mov      r8, rdi
0x14179c08e call     0x142923bf0 ; memcmp
0x14179c093 test     eax, eax
0x14179c095 je       0x14179c1a5
0x14179c09b test     r15b, r15b
0x14179c09e je       0x14179c0ba
0x14179c0a0 lea      rdx, [rip + 0x153e6b6] ; @0x142cda75d 'rack_broomsflowerboxdoor_belldoor_kransdoor_krans_autumncart_w_pumpkins_regularcrates_w_pumpkins_reg'
0x14179c0a7 mov      rcx, rbx
0x14179c0aa mov      r8, rdi
0x14179c0ad call     0x142923bf0 ; memcmp
0x14179c0b2 test     eax, eax
0x14179c0b4 je       0x14179c1ae
0x14179c0ba cmp      byte ptr [rsp + 0x30], 0
0x14179c0bf je       0x14179c0f5
0x14179c0c1 lea      rdx, [rip + 0x153e6a0] ; @0x142cda768 'flowerboxdoor_belldoor_kransdoor_krans_autumncart_w_pumpkins_regularcrates_w_pumpkins_regular'
0x14179c0c8 mov      rcx, rbx
0x14179c0cb mov      r8, rdi
0x14179c0ce call     0x142923bf0 ; memcmp
0x14179c0d3 test     eax, eax
0x14179c0d5 je       0x14179c1c3
0x14179c0db lea      rdx, [rip + 0x153e68f] ; @0x142cda771 'door_belldoor_kransdoor_krans_autumncart_w_pumpkins_regularcrates_w_pumpkins_regular'
0x14179c0e2 mov      rcx, rbx
0x14179c0e5 mov      r8, rdi
0x14179c0e8 call     0x142923bf0 ; memcmp
0x14179c0ed test     eax, eax
0x14179c0ef je       0x14179c1ff
0x14179c0f5 test     r12b, r12b
0x14179c0f8 je       0x14179c114
0x14179c0fa lea      rdx, [rip + 0x153e679] ; @0x142cda77a 'door_kransdoor_krans_autumncart_w_pumpkins_regularcrates_w_pumpkins_regular'
0x14179c101 mov      rcx, rbx
0x14179c104 mov      r8, rdi
0x14179c107 call     0x142923bf0 ; memcmp
0x14179c10c test     eax, eax
0x14179c10e je       0x14179c1cf
0x14179c114 test     r13b, r13b
0x14179c117 je       0x14179c133
0x14179c119 lea      rdx, [rip + 0x153e664] ; @0x142cda784 'door_krans_autumncart_w_pumpkins_regularcrates_w_pumpkins_regular'
0x14179c120 mov      rcx, rbx
0x14179c123 mov      r8, rdi
0x14179c126 call     0x142923bf0 ; memcmp
0x14179c12b test     eax, eax
0x14179c12d je       0x14179c20b
0x14179c133 cmp      byte ptr [rsp + 0x2c], 0
0x14179c138 je       0x14179c154
0x14179c13a lea      rdx, [rip + 0x153e654] ; @0x142cda795 'cart_w_pumpkins_regularcrates_w_pumpkins_regular'
0x14179c141 mov      rcx, rbx
0x14179c144 mov      r8, rdi
0x14179c147 call     0x142923bf0 ; memcmp
0x14179c14c test     eax, eax
0x14179c14e je       0x14179c211
0x14179c154 cmp      rdi, 0x19
0x14179c158 jne      0x14179c251
0x14179c15e movdqu   xmm0, xmmword ptr [rbx]
0x14179c162 movdqu   xmm1, xmmword ptr [rbx + 9]
0x14179c167 pcmpeqb  xmm1, xmmword ptr [rip + 0x1303b11] ; @0x142a9fc80 'pumpkins_regular'
0x14179c16f pcmpeqb  xmm0, xmmword ptr [rip + 0x153e2e9] ; @0x142cda460 'crates_w_pumpkinmap with a single key'
0x14179c177 pand     xmm0, xmm1
0x14179c17b pmovmskb eax, xmm0
0x14179c17f cmp      eax, 0xffff
0x14179c184 jne      0x14179c251
0x14179c18a mov      byte ptr [rsi + 1], 0x37
0x14179c18e jmp      0x14179c239 ; 
0x14179c193 mov      byte ptr [rsi + 1], 0x2e
0x14179c197 jmp      0x14179c239 ; 
0x14179c19c mov      byte ptr [rsi + 1], 0x2f
0x14179c1a0 jmp      0x14179c239 ; 
0x14179c1a5 mov      byte ptr [rsi + 1], 0x30
0x14179c1a9 jmp      0x14179c239 ; 
0x14179c1ae mov      byte ptr [rsi + 1], 0x31
0x14179c1b2 jmp      0x14179c239 ; 
0x14179c1b7 mov      byte ptr [rsi + 1], 0x2c
0x14179c1bb jmp      0x14179c239 ; 
0x14179c1bd mov      byte ptr [rsi + 1], 0x24
0x14179c1c1 jmp      0x14179c239 ; 
0x14179c1c3 mov      byte ptr [rsi + 1], 0x32
0x14179c1c7 jmp      0x14179c239 ; 
0x14179c1c9 mov      byte ptr [rsi + 1], 0x1f
0x14179c1cd jmp      0x14179c239 ; 
0x14179c1cf mov      byte ptr [rsi + 1], 0x34
0x14179c1d3 jmp      0x14179c239 ; 
0x14179c1d5 mov      byte ptr [rsi + 1], 8
0x14179c1d9 jmp      0x14179c239 ; 
0x14179c1db mov      byte ptr [rsi + 1], 0x14
0x14179c1df jmp      0x14179c239 ; 
0x14179c1e1 mov      byte ptr [rsi + 1], 0x18
0x14179c1e5 jmp      0x14179c239 ; 
0x14179c1e7 mov      byte ptr [rsi + 1], 0x1c
0x14179c1eb jmp      0x14179c239 ; 
0x14179c1ed mov      byte ptr [rsi + 1], 0xe
0x14179c1f1 jmp      0x14179c239 ; 
0x14179c1f3 mov      byte ptr [rsi + 1], 0x15
0x14179c1f7 jmp      0x14179c239 ; 
0x14179c1f9 mov      byte ptr [rsi + 1], 0x2b
0x14179c1fd jmp      0x14179c239 ; 
0x14179c1ff mov      byte ptr [rsi + 1], 0x33
0x14179c203 jmp      0x14179c239 ; 
0x14179c205 mov      byte ptr [rsi + 1], 0x22
0x14179c209 jmp      0x14179c239 ; 
0x14179c20b mov      byte ptr [rsi + 1], 0x35
0x14179c20f jmp      0x14179c239 ; 
0x14179c211 mov      byte ptr [rsi + 1], 0x36
0x14179c215 jmp      0x14179c239 ; 
0x14179c217 mov      byte ptr [rsi + 1], 0x20
0x14179c21b jmp      0x14179c239 ; 
0x14179c21d mov      byte ptr [rsi + 1], 0x23
0x14179c221 jmp      0x14179c239 ; 
0x14179c223 mov      byte ptr [rsi + 1], 0x26
0x14179c227 jmp      0x14179c239 ; 
0x14179c229 mov      byte ptr [rsi + 1], 0x16
0x14179c22d jmp      0x14179c239 ; 
0x14179c22f mov      byte ptr [rsi + 1], 0x2a
0x14179c233 jmp      0x14179c239 ; 
0x14179c235 mov      byte ptr [rsi + 1], 0x21
0x14179c239 xor      eax, eax
0x14179c23b mov      byte ptr [rsi], al
0x14179c23d mov      rax, rsi
0x14179c240 add      rsp, 0x38
0x14179c244 pop      rbx
0x14179c245 pop      rbp
0x14179c246 pop      rdi
0x14179c247 pop      rsi
0x14179c248 pop      r12
0x14179c24a pop      r13
0x14179c24c pop      r14
0x14179c24e pop      r15
0x14179c250 ret      
0x14179c251 lea      r8, [rip + 0x153e570]
0x14179c258 mov      r9d, 0x38
0x14179c25e mov      rcx, rbx
0x14179c261 mov      rdx, rdi
0x14179c264 call     0x1428f9e20 ; serde::de::Error::unknown_variant::h81e52211f9e120e7
0x14179c269 mov      qword ptr [rsi + 8], rax
0x14179c26d mov      al, 1
0x14179c26f jmp      0x14179c23b ; 