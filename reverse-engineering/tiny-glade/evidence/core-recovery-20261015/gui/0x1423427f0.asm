0x1423427f0 push     r15
0x1423427f2 push     r14
0x1423427f4 push     r13
0x1423427f6 push     r12
0x1423427f8 push     rsi
0x1423427f9 push     rdi
0x1423427fa push     rbp
0x1423427fb push     rbx
0x1423427fc sub      rsp, 0x98
0x142342803 movaps   xmmword ptr [rsp + 0x80], xmm6
0x14234280b mov      qword ptr [rsp + 0x50], r9
0x142342810 mov      rbp, qword ptr [rdx]
0x142342813 mov      rdx, qword ptr [rdx + 8]
0x142342817 mov      r10, qword ptr [rbp]
0x14234281b xor      r9d, r9d
0x14234281e mov      rax, r10
0x142342821 sub      rax, qword ptr [rdx + 0x18]
0x142342825 cmovb    rax, r9
0x142342829 sub      r10, qword ptr [rdx + 0x38]
0x14234282d cmovb    r10, r9
0x142342831 mov      rsi, rax
0x142342834 shl      rsi, 4
0x142342838 add      rsi, qword ptr [rdx + 8]
0x14234283c mov      r13, qword ptr [rdx + 0x10]
0x142342840 mov      r14, qword ptr [rdx + 0x30]
0x142342844 sub      r13, rax
0x142342847 cmovb    r13, r9
0x14234284b mov      r11d, 8
0x142342851 cmovb    rsi, r11
0x142342855 mov      rax, r10
0x142342858 shl      rax, 4
0x14234285c add      rax, qword ptr [rdx + 0x28]
0x142342860 sub      r14, r10
0x142342863 cmovb    r14, r9
0x142342867 cmovb    rax, r11
0x14234286b mov      rdx, qword ptr [rdx + 0x40]
0x14234286f sub      rdx, r14
0x142342872 sub      rdx, r13
0x142342875 mov      qword ptr [rbp], rdx
0x142342879 shl      r13, 4
0x14234287d add      r13, rsi
0x142342880 shl      r14, 4
0x142342884 add      r14, rax
0x142342887 mov      r10, qword ptr [rcx]
0x14234288a mov      qword ptr [rsp + 0x58], rcx
0x14234288f mov      r12, qword ptr [rcx + 8]
0x142342893 mov      r11d, dword ptr [r8 + 0x1c]
0x142342897 mov      rcx, qword ptr [r8]
0x14234289a mov      rdx, qword ptr [r8 + 0x10]
0x14234289e mov      qword ptr [rsp + 0x68], rdx
0x1423428a3 mov      qword ptr [rsp + 0x30], rcx
0x1423428a8 add      rcx, 0x20
0x1423428ac mov      qword ptr [rsp + 0x60], rcx
0x1423428b1 mov      edi, 0xffffffff
0x1423428b6 mov      qword ptr [rsp + 0x70], r10
0x1423428bb mov      dword ptr [rsp + 0x40], r11d
0x1423428c0 jmp      0x142342900 ; 
0x1423428c2 mov      rdx, qword ptr [rsp + 0x30]
0x1423428c7 mov      rax, qword ptr [rdx + 0x28]
0x1423428cb mov      rcx, rsi
0x1423428ce shl      rcx, 4
0x1423428d2 mov      qword ptr [rax + rcx], rdi
0x1423428d6 mov      byte ptr [rax + rcx + 8], r8b
0x1423428db inc      rsi
0x1423428de mov      qword ptr [rdx + 0x30], rsi
0x1423428e2 inc      qword ptr [rdx + 0x40]
0x1423428e6 mov      rax, rbx
0x1423428e9 mov      rsi, r15
0x1423428ec mov      edi, 0xffffffff
0x1423428f1 nop      word ptr cs:[rax + rax]
0x142342900 test     rsi, rsi
0x142342903 je       0x142342920
0x142342905 cmp      rsi, r13
0x142342908 je       0x142342920
0x14234290a lea      r15, [rsi + 0x10]
0x14234290e mov      rbx, rax
0x142342911 jmp      0x14234293f ; 
0x142342913 nop      word ptr cs:[rax + rax]
0x142342920 test     rax, rax
0x142342923 je       0x142342a34
0x142342929 cmp      rax, r14
0x14234292c je       0x142342a34
0x142342932 lea      rbx, [rax + 0x10]
0x142342936 xor      r15d, r15d
0x142342939 mov      rsi, rax
0x14234293c mov      rax, rbx
0x14234293f inc      qword ptr [rbp]
0x142342943 mov      rdx, qword ptr [rsi + 8]
0x142342947 mov      r8d, edx
0x14234294a mov      rsi, r15
0x14234294d cmp      r8, qword ptr [r12 + 0x10]
0x142342952 jae      0x142342900
0x142342954 mov      r9, qword ptr [r12 + 8]
0x142342959 shr      rdx, 0x20
0x14234295d lea      r8, [r8 + r8*4]
0x142342961 mov      rax, rbx
0x142342964 mov      rsi, r15
0x142342967 cmp      dword ptr [r9 + r8*4], edx
0x14234296b jne      0x142342900
0x14234296d lea      rdx, [r9 + r8*4]
0x142342971 mov      r8d, dword ptr [rdx + 4]
0x142342975 mov      rax, rbx
0x142342978 mov      rsi, r15
0x14234297b cmp      r8, rdi
0x14234297e je       0x142342900
0x142342980 mov      rax, rbx
0x142342983 mov      rsi, r15
0x142342986 cmp      qword ptr [r10 + 0x38], r8
0x14234298a jbe      0x142342900
0x142342990 mov      rax, qword ptr [r10 + 0x28]
0x142342994 mov      ecx, r8d
0x142342997 shr      ecx, 6
0x14234299a mov      rax, qword ptr [rax + rcx*8]
0x14234299e bt       rax, r8
0x1423429a2 mov      rax, rbx
0x1423429a5 mov      rsi, r15
0x1423429a8 jae      0x142342900
0x1423429ae mov      eax, dword ptr [rdx + 0xc]
0x1423429b1 mov      ecx, dword ptr [rdx + 0x10]
0x1423429b4 mov      rdx, qword ptr [r12 + 0x1a8]
0x1423429bc lea      rax, [rax + rax*8]
0x1423429c0 mov      r8, qword ptr [r10 + 0x110]
0x1423429c7 mov      r9, qword ptr [rdx + rax*8 + 0x18]
0x1423429cc mov      rax, qword ptr [rdx + rax*8 + 0x38]
0x1423429d1 mov      rax, qword ptr [rax + r8*8]
0x1423429d5 not      rax
0x1423429d8 lea      rax, [rax + rax*2]
0x1423429dc shl      rax, 4
0x1423429e0 mov      rax, qword ptr [r9 + rax + 0x10]
0x1423429e5 movzx    r8d, byte ptr [rax + rcx]
0x1423429ea mov      rax, qword ptr [rsp + 0x68]
0x1423429ef mov      dword ptr [rax], r11d
0x1423429f2 mov      rax, qword ptr [rsp + 0x30]
0x1423429f7 mov      rsi, qword ptr [rax + 0x30]
0x1423429fb mov      rdi, qword ptr [rax + 0x40]
0x1423429ff cmp      rsi, qword ptr [rax + 0x20]
0x142342a03 jne      0x1423428c2
0x142342a09 mov      rcx, qword ptr [rsp + 0x60]
0x142342a0e lea      rdx, [rip + 0x87285b]
0x142342a15 mov      byte ptr [rsp + 0x48], r8b
0x142342a1a call     0x140562630 ; alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::habac6ea2a1af3a81 / alloc::raw_vec::RawVec$LT$T$C$A$GT$::grow_one::h3d92912b2ef5335f
0x142342a1f movzx    r8d, byte ptr [rsp + 0x48]
0x142342a25 mov      r11d, dword ptr [rsp + 0x40]
0x142342a2a mov      r10, qword ptr [rsp + 0x70]
0x142342a2f jmp      0x1423428c2 ; 
0x142342a34 mov      rcx, qword ptr [rsp + 0x50]
0x142342a39 mov      rax, qword ptr [rcx]
0x142342a3c mov      rcx, qword ptr [rcx + 8]
0x142342a40 mov      rdx, qword ptr [rax]
0x142342a43 xor      r8d, r8d
0x142342a46 mov      r9, rdx
0x142342a49 sub      r9, qword ptr [rcx + 0x18]
0x142342a4d cmovb    r9, r8
0x142342a51 sub      rdx, qword ptr [rcx + 0x38]
0x142342a55 cmovb    rdx, r8
0x142342a59 mov      r10, qword ptr [rcx + 0x10]
0x142342a5d mov      r11, qword ptr [rcx + 0x30]
0x142342a61 sub      r11, rdx
0x142342a64 cmovb    r11, r8
0x142342a68 setbe    dl
0x142342a6b sub      r10, r9
0x142342a6e cmovb    r10, r8
0x142342a72 mov      rcx, qword ptr [rcx + 0x40]
0x142342a76 setbe    dil
0x142342a7a add      r10, r11
0x142342a7d mov      r11, qword ptr [rsp + 0x70]
0x142342a82 and      dil, dl
0x142342a85 cmove    r10, r8
0x142342a89 sub      rcx, r10
0x142342a8c mov      qword ptr [rax], rcx
0x142342a8f mov      r14, qword ptr [r11 + 0xf8]
0x142342a96 mov      rax, qword ptr [r11 + 0x100]
0x142342a9d mov      rcx, qword ptr [rsp + 0x108]
0x142342aa5 mov      rdx, qword ptr [rcx + 8]
0x142342aa9 mov      qword ptr [rsp + 0x30], rdx
0x142342aae test     rdx, rdx
0x142342ab1 jne      0x142342abc
0x142342ab3 mov      rcx, qword ptr [rcx + 0x10]
0x142342ab7 mov      qword ptr [rsp + 0x30], rcx
0x142342abc mov      rcx, qword ptr [rsp + 0x58]
0x142342ac1 mov      r9d, dword ptr [rcx + 0x14]
0x142342ac5 lea      r13, [r14 + rax*4]
0x142342ac9 mov      rax, qword ptr [rsp + 0x100]
0x142342ad1 mov      rdx, qword ptr [rax]
0x142342ad4 cmp      byte ptr [r11 + 0x108], 0
0x142342adc je       0x142342afb
0x142342ade mov      ecx, 8
0x142342ae3 xor      r8d, r8d
0x142342ae6 movsd    xmm6, qword ptr [rip + 0xd0b5c2]
0x142342aee xor      r10d, r10d
0x142342af1 xor      ebp, ebp
0x142342af3 xor      r15d, r15d
0x142342af6 jmp      0x142342b98 ; 
0x142342afb mov      ecx, 8
0x142342b00 xor      r10d, r10d
0x142342b03 movsd    xmm6, qword ptr [rip + 0xd0b5a5]
0x142342b0b xor      r8d, r8d
0x142342b0e xor      ebp, ebp
0x142342b10 xor      r15d, r15d
0x142342b13 mov      qword ptr [rsp + 0x48], rdx
0x142342b18 jmp      0x142342d68 ; 
0x142342b1d mov      r8, qword ptr [rsp + 0x30]
0x142342b22 mov      rax, qword ptr [r8 + 8]
0x142342b26 lea      rcx, [rip - 0x136392d] ; @0x140fdf200 'VWH'
0x142342b2d mov      qword ptr [rax + rdi], rcx
0x142342b31 lea      rcx, [rip - 0x4f32d8]
0x142342b38 mov      qword ptr [rax + rdi + 8], rcx
0x142342b3d mov      rcx, qword ptr [rsp + 0x50]
0x142342b42 mov      qword ptr [rax + rdi + 0x10], rcx
0x142342b47 movsd    qword ptr [rax + rdi + 0x18], xmm6
0x142342b4d mov      dword ptr [rax + rdi + 0x20], 0
0x142342b55 mov      byte ptr [rax + rdi + 0x24], 0
0x142342b5a add      rdi, 0x28
0x142342b5e mov      qword ptr [r8 + 0x10], rdi
0x142342b62 movzx    edi, byte ptr [rsp + 0x60]
0x142342b67 mov      r8, qword ptr [rsp + 0x40]
0x142342b6c mov      rcx, qword ptr [rsp + 0x58]
0x142342b71 nop      word ptr cs:[rax + rax]
0x142342b80 inc      r15
0x142342b83 movzx    eax, byte ptr [rdx + 0x18]
0x142342b87 cmp      al, byte ptr [r10 + rbx]
0x142342b8b mov      rax, qword ptr [rsp + 0x68]
0x142342b90 mov      dword ptr [rax + rbx*4], r9d
0x142342b94 sete     byte ptr [rsi + 2]
0x142342b98 cmp      r15, rbp
0x142342b9b jne      0x142342c51
0x142342ba1 cmp      r14, r13
0x142342ba4 je       0x142342f09
0x142342baa mov      rax, qword ptr [r12 + 0x1a8]
0x142342bb2 nop      word ptr cs:[rax + rax]
0x142342bc0 mov      ecx, dword ptr [r14]
0x142342bc3 lea      rcx, [rcx + rcx*8]
0x142342bc7 mov      rbp, qword ptr [rax + rcx*8 + 0x10]
0x142342bcc test     rbp, rbp
0x142342bcf jne      0x142342be0
0x142342bd1 add      r14, 4
0x142342bd5 cmp      r14, r13
0x142342bd8 jne      0x142342bc0
0x142342bda jmp      0x142342f09 ; 
0x142342bdf nop      
0x142342be0 mov      rsi, rdx
0x142342be3 lea      rax, [rax + rcx*8]
0x142342be7 mov      rdx, qword ptr [r11 + 0x110]
0x142342bee mov      rcx, qword ptr [rax + 0x38]
0x142342bf2 cmp      qword ptr [rax + 0x40], rdx
0x142342bf6 jbe      0x142342c17
0x142342bf8 mov      r8, qword ptr [rcx + rdx*8]
0x142342bfc mov      rdx, qword ptr [rax + 0x18]
0x142342c00 test     r8, r8
0x142342c03 je       0x142342c1b
0x142342c05 not      r8
0x142342c08 lea      r8, [r8 + r8*2]
0x142342c0c shl      r8, 4
0x142342c10 mov      r10, qword ptr [rdx + r8 + 0x10]
0x142342c15 jmp      0x142342c1e ; 
0x142342c17 mov      rdx, qword ptr [rax + 0x18]
0x142342c1b xor      r10d, r10d
0x142342c1e mov      r8, qword ptr [r11 + 0x118]
0x142342c25 add      r14, 4
0x142342c29 mov      rcx, qword ptr [rcx + r8*8]
0x142342c2d not      rcx
0x142342c30 lea      rcx, [rcx + rcx*2]
0x142342c34 shl      rcx, 4
0x142342c38 mov      r8, qword ptr [rdx + rcx + 0x10]
0x142342c3d mov      rcx, qword ptr [rdx + rcx + 0x28]
0x142342c42 mov      qword ptr [rsp + 0x68], rcx
0x142342c47 mov      rcx, qword ptr [rax + 8]
0x142342c4b xor      r15d, r15d
0x142342c4e mov      rdx, rsi
0x142342c51 mov      ebx, r15d
0x142342c54 lea      rsi, [rbx + rbx*2]
0x142342c58 add      rsi, r8
0x142342c5b test     dil, dil
0x142342c5e jne      0x142342b80
0x142342c64 cmp      byte ptr [rsi + 2], 0
0x142342c68 jne      0x142342b80
0x142342c6e movzx    eax, byte ptr [rdx + 0x18]
0x142342c72 cmp      al, byte ptr [r10 + rbx]
0x142342c76 jne      0x142342b80
0x142342c7c mov      qword ptr [rsp + 0x40], r8
0x142342c81 mov      byte ptr [rsp + 0x60], dil
0x142342c86 mov      qword ptr [rsp + 0x58], rcx
0x142342c8b mov      rax, qword ptr [rcx + r15*8]
0x142342c8f mov      qword ptr [rsp + 0x50], rax
0x142342c94 mov      rcx, qword ptr [rsp + 0x30]
0x142342c99 mov      rax, qword ptr [rcx]
0x142342c9c mov      rdi, qword ptr [rcx + 0x10]
0x142342ca0 sub      rax, rdi
0x142342ca3 cmp      rax, 0x27
0x142342ca7 ja       0x142342b1d
0x142342cad mov      qword ptr [rsp + 0x20], 1
0x142342cb6 mov      r8d, 0x28
0x142342cbc mov      dword ptr [rsp + 0x3c], r9d
0x142342cc1 mov      r9d, 1
0x142342cc7 mov      rcx, qword ptr [rsp + 0x30]
0x142342ccc mov      qword ptr [rsp + 0x48], rdx
0x142342cd1 mov      rdx, rdi
0x142342cd4 mov      qword ptr [rsp + 0x78], r10
0x142342cd9 call     0x1428a58d0 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hbdbbfc746df8f5db
0x142342cde mov      r10, qword ptr [rsp + 0x78]
0x142342ce3 mov      rdx, qword ptr [rsp + 0x48]
0x142342ce8 mov      r9d, dword ptr [rsp + 0x3c]
0x142342ced mov      r11, qword ptr [rsp + 0x70]
0x142342cf2 jmp      0x142342b1d ; 
0x142342cf7 mov      r10, qword ptr [rsp + 0x30]
0x142342cfc mov      rax, qword ptr [r10 + 8]
0x142342d00 lea      rcx, [rip - 0x1363b07] ; @0x140fdf200 'VWH'
0x142342d07 mov      qword ptr [rax + rdi], rcx
0x142342d0b lea      rcx, [rip - 0x4f34b2]
0x142342d12 mov      qword ptr [rax + rdi + 8], rcx
0x142342d17 mov      rcx, qword ptr [rsp + 0x50]
0x142342d1c mov      qword ptr [rax + rdi + 0x10], rcx
0x142342d21 movsd    qword ptr [rax + rdi + 0x18], xmm6
0x142342d27 mov      dword ptr [rax + rdi + 0x20], 0
0x142342d2f mov      byte ptr [rax + rdi + 0x24], 0
0x142342d34 add      rdi, 0x28
0x142342d38 mov      qword ptr [r10 + 0x10], rdi
0x142342d3c movzx    edi, byte ptr [rsp + 0x60]
0x142342d41 mov      r10, qword ptr [rsp + 0x58]
0x142342d46 mov      rcx, qword ptr [rsp + 0x40]
0x142342d4b nop      dword ptr [rax + rax]
0x142342d50 inc      r15
0x142342d53 movzx    eax, byte ptr [rdx + 0x18]
0x142342d57 cmp      al, byte ptr [r8 + rbx]
0x142342d5b mov      rax, qword ptr [rsp + 0x68]
0x142342d60 mov      dword ptr [rax + rbx*4], r9d
0x142342d64 sete     byte ptr [rsi + 2]
0x142342d68 cmp      r15, rbp
0x142342d6b jne      0x142342e5b
0x142342d71 cmp      r14, r13
0x142342d74 je       0x142342f09
0x142342d7a mov      rax, qword ptr [r12 + 0x100]
0x142342d82 nop      word ptr cs:[rax + rax]
0x142342d90 mov      ecx, dword ptr [r14]
0x142342d93 lea      rcx, [rcx + rcx*4]
0x142342d97 shl      rcx, 5
0x142342d9b mov      rbp, qword ptr [rax + rcx + 0x58]
0x142342da0 test     rbp, rbp
0x142342da3 jne      0x142342dc0
0x142342da5 add      r14, 4
0x142342da9 cmp      r14, r13
0x142342dac jne      0x142342d90
0x142342dae jmp      0x142342f09 ; 
0x142342db3 nop      word ptr cs:[rax + rax]
0x142342dc0 mov      ebx, r9d
0x142342dc3 add      rax, rcx
0x142342dc6 mov      ecx, dword ptr [rax + 0x94]
0x142342dcc mov      r9, qword ptr [r12 + 0x1a8]
0x142342dd4 lea      r10, [rcx + rcx*8]
0x142342dd8 lea      rdx, [r9 + r10*8]
0x142342ddc mov      r8, qword ptr [r11 + 0x110]
0x142342de3 mov      rcx, qword ptr [r9 + r10*8 + 0x38]
0x142342de8 cmp      qword ptr [r9 + r10*8 + 0x40], r8
0x142342ded jbe      0x142342e11
0x142342def mov      r8, qword ptr [rcx + r8*8]
0x142342df3 mov      rdx, qword ptr [rdx + 0x18]
0x142342df7 test     r8, r8
0x142342dfa mov      r9d, ebx
0x142342dfd je       0x142342e1d
0x142342dff not      r8
0x142342e02 lea      r8, [r8 + r8*2]
0x142342e06 shl      r8, 4
0x142342e0a mov      r10, qword ptr [rdx + r8 + 0x10]
0x142342e0f jmp      0x142342e20 ; 
0x142342e11 mov      rdx, qword ptr [rdx + 0x18]
0x142342e15 xor      r10d, r10d
0x142342e18 mov      r9d, ebx
0x142342e1b jmp      0x142342e20 ; 
0x142342e1d xor      r10d, r10d
0x142342e20 mov      r8, qword ptr [r11 + 0x118]
0x142342e27 add      r14, 4
0x142342e2b mov      rcx, qword ptr [rcx + r8*8]
0x142342e2f not      rcx
0x142342e32 lea      rcx, [rcx + rcx*2]
0x142342e36 shl      rcx, 4
0x142342e3a mov      rsi, qword ptr [rdx + rcx + 0x10]
0x142342e3f mov      rcx, qword ptr [rdx + rcx + 0x28]
0x142342e44 mov      qword ptr [rsp + 0x68], rcx
0x142342e49 mov      rcx, qword ptr [rax + 0x50]
0x142342e4d xor      r15d, r15d
0x142342e50 mov      rdx, qword ptr [rsp + 0x48]
0x142342e55 mov      r8, r10
0x142342e58 mov      r10, rsi
0x142342e5b mov      rax, r15
0x142342e5e shl      rax, 4
0x142342e62 mov      ebx, dword ptr [rcx + rax + 8]
0x142342e66 lea      rsi, [rbx + rbx*2]
0x142342e6a add      rsi, r10
0x142342e6d test     dil, dil
0x142342e70 jne      0x142342d50
0x142342e76 cmp      byte ptr [rsi + 2], 1
0x142342e7a je       0x142342d50
0x142342e80 mov      qword ptr [rsp + 0x40], rcx
0x142342e85 movzx    ecx, byte ptr [rdx + 0x18]
0x142342e89 cmp      cl, byte ptr [r8 + rbx]
0x142342e8d mov      rcx, qword ptr [rsp + 0x40]
0x142342e92 jne      0x142342d50
0x142342e98 mov      qword ptr [rsp + 0x58], r10
0x142342e9d mov      byte ptr [rsp + 0x60], dil
0x142342ea2 mov      rax, qword ptr [rcx + rax]
0x142342ea6 mov      qword ptr [rsp + 0x50], rax
0x142342eab mov      rcx, qword ptr [rsp + 0x30]
0x142342eb0 mov      rax, qword ptr [rcx]
0x142342eb3 mov      rdi, qword ptr [rcx + 0x10]
0x142342eb7 sub      rax, rdi
0x142342eba cmp      rax, 0x27
0x142342ebe ja       0x142342cf7
0x142342ec4 mov      qword ptr [rsp + 0x20], 1
0x142342ecd mov      qword ptr [rsp + 0x78], r8
0x142342ed2 mov      r8d, 0x28
0x142342ed8 mov      dword ptr [rsp + 0x3c], r9d
0x142342edd mov      r9d, 1
0x142342ee3 mov      rcx, qword ptr [rsp + 0x30]
0x142342ee8 mov      rdx, rdi
0x142342eeb call     0x1428a58d0 ; alloc::raw_vec::RawVecInner$LT$A$GT$::reserve::do_reserve_and_handle::hbdbbfc746df8f5db
0x142342ef0 mov      r8, qword ptr [rsp + 0x78]
0x142342ef5 mov      rdx, qword ptr [rsp + 0x48]
0x142342efa mov      r9d, dword ptr [rsp + 0x3c]
0x142342eff mov      r11, qword ptr [rsp + 0x70]
0x142342f04 jmp      0x142342cf7 ; 
0x142342f09 movaps   xmm6, xmmword ptr [rsp + 0x80]
0x142342f11 add      rsp, 0x98
0x142342f18 pop      rbx
0x142342f19 pop      rbp
0x142342f1a pop      rdi
0x142342f1b pop      rsi
0x142342f1c pop      r12
0x142342f1e pop      r13
0x142342f20 pop      r14
0x142342f22 pop      r15
0x142342f24 ret      