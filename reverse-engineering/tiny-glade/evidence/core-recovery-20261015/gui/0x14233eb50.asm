0x14233eb50 push     r15
0x14233eb52 push     r14
0x14233eb54 push     r13
0x14233eb56 push     r12
0x14233eb58 push     rsi
0x14233eb59 push     rdi
0x14233eb5a push     rbp
0x14233eb5b push     rbx
0x14233eb5c sub      rsp, 0x68
0x14233eb60 mov      rbx, r9
0x14233eb63 mov      rsi, r8
0x14233eb66 mov      rdi, rdx
0x14233eb69 mov      r12, qword ptr [rsp + 0xe0]
0x14233eb71 mov      r15, qword ptr [rsp + 0xd0]
0x14233eb79 mov      rcx, qword ptr [rcx]
0x14233eb7c call     0x14093fbe0
0x14233eb81 test     al, al
0x14233eb83 je       0x14233ebb1
0x14233eb85 mov      rcx, qword ptr [rbx]
0x14233eb88 call     0x1409a9140
0x14233eb8d test     al, al
0x14233eb8f je       0x14233ebb1
0x14233eb91 mov      rax, qword ptr [rsp + 0xd8]
0x14233eb99 mov      rax, qword ptr [rax]
0x14233eb9c movzx    eax, byte ptr [rax]
0x14233eb9f mov      ecx, eax
0x14233eba1 xor      cl, 1
0x14233eba4 mov      dword ptr [rsp + 0x24], ecx
0x14233eba8 test     al, al
0x14233ebaa jne      0x14233ebb9
0x14233ebac mov      r14, qword ptr [r15]
0x14233ebaf jmp      0x14233ebdd
0x14233ebb1 mov      dword ptr [rsp + 0x24], 0
0x14233ebb9 mov      r14, qword ptr [r15]
0x14233ebbc lea      rbx, [r14 + 0x10]
0x14233ebc0 mov      rcx, rbx
0x14233ebc3 call     0x1409a9150
0x14233ebc8 test     al, al
0x14233ebca je       0x14233ebdd
0x14233ebcc mov      eax, dword ptr [r15 + 0x1c]
0x14233ebd0 mov      rcx, qword ptr [r15 + 0x10]
0x14233ebd4 mov      dword ptr [rcx], eax
0x14233ebd6 mov      qword ptr [rbx], 0
0x14233ebdd mov      rax, qword ptr [r12]
0x14233ebe1 mov      rcx, qword ptr [r12 + 8]
0x14233ebe6 mov      qword ptr [rsp + 0x30], rcx
0x14233ebeb mov      ecx, dword ptr [r12 + 0x14]
0x14233ebf0 mov      dword ptr [rsp + 0x2c], ecx
0x14233ebf4 mov      r8, qword ptr [rax + 0xf8]
0x14233ebfb mov      rcx, qword ptr [rax + 0x100]
0x14233ec02 lea      r9, [r8 + rcx*4]
0x14233ec06 movzx    r10d, byte ptr [rax + 0x108]
0x14233ec0e mov      r12, qword ptr [rdi]
0x14233ec11 inc      r12
0x14233ec14 mov      rcx, qword ptr [rsi]
0x14233ec17 add      rcx, 0xa
0x14233ec1b mov      qword ptr [rsp + 0x58], rcx
0x14233ec20 mov      ecx, dword ptr [r15 + 0x1c]
0x14233ec24 mov      dword ptr [rsp + 0x28], ecx
0x14233ec28 mov      rbx, qword ptr [r15 + 0x10]
0x14233ec2c mov      ecx, 8
0x14233ec31 mov      qword ptr [rsp + 0x50], rcx
0x14233ec36 xor      ebp, ebp
0x14233ec38 xor      r13d, r13d
0x14233ec3b xor      r15d, r15d
0x14233ec3e mov      qword ptr [rsp + 0x48], rcx
0x14233ec43 xor      esi, esi
0x14233ec45 test     r10b, r10b
0x14233ec48 je       0x14233ec90
0x14233ec4a cmp      r13, r15
0x14233ec4d jne      0x14233ed53
0x14233ec53 cmp      r8, r9
0x14233ec56 je       0x14233eebd
0x14233ec5c mov      rcx, qword ptr [rsp + 0x30]
0x14233ec61 mov      rcx, qword ptr [rcx + 0x1a8]
0x14233ec68 nop      dword ptr [rax + rax]
0x14233ec70 mov      edx, dword ptr [r8]
0x14233ec73 lea      rdx, [rdx + rdx*8]
0x14233ec77 mov      r15, qword ptr [rcx + rdx*8 + 0x10]
0x14233ec7c test     r15, r15
0x14233ec7f jne      0x14233ecd3
0x14233ec81 add      r8, 4
0x14233ec85 cmp      r8, r9
0x14233ec88 jne      0x14233ec70
0x14233ec8a jmp      0x14233eebd
0x14233ec8f nop      
0x14233ec90 cmp      r13, r15
0x14233ec93 jne      0x14233edbd
0x14233ec99 cmp      r8, r9
0x14233ec9c je       0x14233eebd
0x14233eca2 mov      rcx, qword ptr [rsp + 0x30]
0x14233eca7 mov      rdx, qword ptr [rcx + 0x100]
0x14233ecae nop      
0x14233ecb0 mov      ecx, dword ptr [r8]
0x14233ecb3 lea      rcx, [rcx + rcx*4]
0x14233ecb7 shl      rcx, 5
0x14233ecbb mov      r15, qword ptr [rdx + rcx + 0x58]
0x14233ecc0 test     r15, r15
0x14233ecc3 jne      0x14233ecf3
0x14233ecc5 add      r8, 4
0x14233ecc9 cmp      r8, r9
0x14233eccc jne      0x14233ecb0
0x14233ecce jmp      0x14233eebd
0x14233ecd3 lea      rdx, [rcx + rdx*8]
0x14233ecd7 mov      rcx, qword ptr [rdx + 0x38]
0x14233ecdb mov      r11, qword ptr [rax + 0x120]
0x14233ece2 cmp      qword ptr [rdx + 0x40], r11
0x14233ece6 jbe      0x14233ed18
0x14233ece8 cmp      qword ptr [rcx + r11*8], 0
0x14233eced setne    dil
0x14233ecf1 jmp      0x14233ed1a
0x14233ecf3 add      rdx, rcx
0x14233ecf6 mov      rcx, qword ptr [rax + 0x120]
0x14233ecfd cmp      qword ptr [rdx + 0x88], rcx
0x14233ed04 jbe      0x14233ed66
0x14233ed06 mov      r11, qword ptr [rdx + 0x80]
0x14233ed0d cmp      qword ptr [r11 + rcx*8], 0
0x14233ed12 setne    bpl
0x14233ed16 jmp      0x14233ed68
0x14233ed18 xor      edi, edi
0x14233ed1a mov      r11, qword ptr [rdx + 0x18]
0x14233ed1e mov      rsi, qword ptr [rax + 0x110]
0x14233ed25 mov      rcx, qword ptr [rcx + rsi*8]
0x14233ed29 not      rcx
0x14233ed2c lea      rcx, [rcx + rcx*2]
0x14233ed30 shl      rcx, 4
0x14233ed34 mov      rbp, qword ptr [r11 + rcx + 0x10]
0x14233ed39 mov      r11, qword ptr [r11 + rcx + 0x28]
0x14233ed3e add      r8, 4
0x14233ed42 mov      rcx, qword ptr [rdx + 8]
0x14233ed46 mov      qword ptr [rsp + 0x48], rcx
0x14233ed4b xor      r13d, r13d
0x14233ed4e mov      esi, edi
0x14233ed50 mov      rdi, r11
0x14233ed53 mov      rcx, qword ptr [rsp + 0x48]
0x14233ed58 mov      rcx, qword ptr [rcx + r13*8]
0x14233ed5c mov      qword ptr [rsp + 0x38], rcx
0x14233ed61 mov      r11d, r13d
0x14233ed64 jmp      0x14233edd7
0x14233ed66 xor      ebp, ebp
0x14233ed68 mov      ecx, dword ptr [rdx + 0x94]
0x14233ed6e mov      r11, qword ptr [rsp + 0x30]
0x14233ed73 mov      r11, qword ptr [r11 + 0x1a8]
0x14233ed7a lea      rcx, [rcx + rcx*8]
0x14233ed7e mov      rsi, qword ptr [r11 + rcx*8 + 0x18]
0x14233ed83 mov      rcx, qword ptr [r11 + rcx*8 + 0x38]
0x14233ed88 mov      r11, qword ptr [rax + 0x110]
0x14233ed8f mov      rcx, qword ptr [rcx + r11*8]
0x14233ed93 not      rcx
0x14233ed96 lea      rcx, [rcx + rcx*2]
0x14233ed9a shl      rcx, 4
0x14233ed9e mov      r11, qword ptr [rsi + rcx + 0x10]
0x14233eda3 mov      rdi, qword ptr [rsi + rcx + 0x28]
0x14233eda8 add      r8, 4
0x14233edac mov      rcx, qword ptr [rdx + 0x50]
0x14233edb0 mov      qword ptr [rsp + 0x50], rcx
0x14233edb5 xor      r13d, r13d
0x14233edb8 mov      esi, ebp
0x14233edba mov      rbp, r11
0x14233edbd mov      rdx, r13
0x14233edc0 shl      rdx, 4
0x14233edc4 mov      rcx, qword ptr [rsp + 0x50]
0x14233edc9 mov      r11, qword ptr [rcx + rdx]
0x14233edcd mov      qword ptr [rsp + 0x38], r11
0x14233edd2 mov      r11d, dword ptr [rcx + rdx + 8]
0x14233edd7 lea      rdx, [r11 + r11*2]
0x14233eddb add      rdx, rbp
0x14233edde lea      rcx, [rdi + r11*4]
0x14233ede2 inc      r13
0x14233ede5 test     sil, 1
0x14233ede9 je       0x14233ee10
0x14233edeb cmp      byte ptr [rsp + 0x24], 0
0x14233edf0 je       0x14233ee00
0x14233edf2 cmp      byte ptr [r12], 0
0x14233edf7 mov      r11, qword ptr [rsp + 0x58]
0x14233edfc jne      0x14233ee1a
0x14233edfe nop      
0x14233ee00 mov      r11d, dword ptr [rsp + 0x2c]
0x14233ee05 mov      dword ptr [rcx], r11d
0x14233ee08 mov      byte ptr [rdx], 0
0x14233ee0b jmp      0x14233ee72
0x14233ee0d nop      dword ptr [rax]
0x14233ee10 mov      r11, r12
0x14233ee13 cmp      byte ptr [rsp + 0x24], 0
0x14233ee18 je       0x14233ee00
0x14233ee1a mov      dword ptr [rsp + 0x44], esi
0x14233ee1e mov      qword ptr [rsp + 0x60], rbp
0x14233ee23 mov      rbp, r14
0x14233ee26 mov      r14, rbx
0x14233ee29 mov      rbx, rdi
0x14233ee2c mov      rdi, rax
0x14233ee2f mov      rsi, r12
0x14233ee32 movzx    r12d, byte ptr [r11]
0x14233ee36 mov      r11d, r12d
0x14233ee39 xor      r11b, 1
0x14233ee3d mov      rax, rcx
0x14233ee40 mov      ecx, r10d
0x14233ee43 mov      r10d, dword ptr [rsp + 0x2c]
0x14233ee48 mov      dword ptr [rax], r10d
0x14233ee4b mov      r10d, ecx
0x14233ee4e mov      byte ptr [rdx], r11b
0x14233ee51 test     r12b, r12b
0x14233ee54 mov      r12, rsi
0x14233ee57 mov      rax, rdi
0x14233ee5a mov      rdi, rbx
0x14233ee5d mov      rbx, r14
0x14233ee60 mov      r14, rbp
0x14233ee63 mov      esi, dword ptr [rsp + 0x44]
0x14233ee67 mov      rbp, qword ptr [rsp + 0x60]
0x14233ee6c je       0x14233ec45
0x14233ee72 cmp      dword ptr [r14 + 0x10], 1
0x14233ee77 jne      0x14233ee92
0x14233ee79 mov      rcx, qword ptr [rsp + 0x38]
0x14233ee7e cmp      qword ptr [r14 + 0x18], rcx
0x14233ee82 jne      0x14233ee92
0x14233ee84 mov      ecx, dword ptr [rsp + 0x28]
0x14233ee88 mov      dword ptr [rbx], ecx
0x14233ee8a mov      qword ptr [r14 + 0x10], 0
0x14233ee92 cmp      dword ptr [r14], 1
0x14233ee96 jne      0x14233ec45
0x14233ee9c mov      rcx, qword ptr [rsp + 0x38]
0x14233eea1 cmp      qword ptr [r14 + 8], rcx
0x14233eea5 jne      0x14233ec45
0x14233eeab mov      ecx, dword ptr [rsp + 0x28]
0x14233eeaf mov      dword ptr [rbx], ecx
0x14233eeb1 mov      qword ptr [r14], 0
0x14233eeb8 jmp      0x14233ec45
0x14233eebd add      rsp, 0x68
0x14233eec1 pop      rbx
0x14233eec2 pop      rbp
0x14233eec3 pop      rdi
0x14233eec4 pop      rsi
0x14233eec5 pop      r12
0x14233eec7 pop      r13