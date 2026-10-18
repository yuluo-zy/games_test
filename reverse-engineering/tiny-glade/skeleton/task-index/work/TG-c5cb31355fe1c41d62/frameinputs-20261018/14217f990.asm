0x14217f990 push rsi
0x14217f991 push rdi
0x14217f992 push rbx
0x14217f993 sub rsp, 0x40
0x14217f997 movaps xmmword ptr [rsp + 0x30], xmm9
0x14217f99d movaps xmmword ptr [rsp + 0x20], xmm8
0x14217f9a3 movaps xmmword ptr [rsp + 0x10], xmm7
0x14217f9a8 movaps xmmword ptr [rsp], xmm6
0x14217f9ac mov r10d, dword ptr [rcx]
0x14217f9af mov r9d, dword ptr [rcx + 4]
0x14217f9b3 mov rax, qword ptr [rdx]
0x14217f9b6 cmp r10d, r9d
0x14217f9b9 seta r11b
0x14217f9bd mov r8, qword ptr [rdx + 8]
0x14217f9c1 or r11b, byte ptr [rcx + 8]
0x14217f9c5 test r11b, 1
0x14217f9c9 jne 0x14217fbe4
0x14217f9cf mov rcx, qword ptr [rdx + 0x10]
0x14217f9d3 cmp r10d, r9d
0x14217f9d6 jae 0x14217fae4
0x14217f9dc movaps xmm0, xmmword ptr [rip + 0x89780d] ; 0x142a171f0 000000bf000000bf0000000000000000 f32=-0.5
0x14217f9e3 movss xmm1, dword ptr [rip + 0x7a5e85] ; 0x142925870 0000003fc10ffc3e34a6c93900000042 f32=0.5
0x14217f9eb xor edx, edx
0x14217f9ed xorps xmm2, xmm2
0x14217f9f0 movss xmm3, dword ptr [rip + 0x820dd8] ; 0x1429a07d0 ffff7f4f000080430000000000000000 f32=4294967040.0
0x14217f9f8 mov r11d, 0xffffffff
0x14217f9fe movss xmm4, dword ptr [rip + 0x90692a] ; 0x142a86330 abaaaa3e2f72757374632f3035663938 f32=0.3333333432674408
0x14217fa06 mov esi, 0xaaaaaaab
0x14217fa0b jmp 0x14217fa2a
0x14217fa0d nop dword ptr [rax]
0x14217fa10 xorps xmm5, xmm5
0x14217fa13 inc r10d
0x14217fa16 addps xmm5, xmm0
0x14217fa19 movlps qword ptr [rcx + r8*8], xmm5
0x14217fa1e inc r8
0x14217fa21 cmp r10d, r9d
0x14217fa24 je 0x14217faea
0x14217fa2a test r10d, r10d
0x14217fa2d je 0x14217fa10
0x14217fa2f xorps xmm5, xmm5
0x14217fa32 mov edi, r10d
0x14217fa35 movaps xmm6, xmm1
0x14217fa38 nop dword ptr [rax + rax]
0x14217fa40 mov ebx, edi
0x14217fa42 and edi, 1
0x14217fa45 xorps xmm7, xmm7
0x14217fa48 cvtsi2ss xmm7, edi
0x14217fa4c xorps xmm8, xmm8
0x14217fa50 cvtsi2ss xmm8, rbx
0x14217fa55 mulss xmm8, xmm1
0x14217fa5a cvttss2si rdi, xmm8
0x14217fa5f ucomiss xmm8, xmm2
0x14217fa63 cmovb edi, edx
0x14217fa66 ucomiss xmm8, xmm3
0x14217fa6a cmova edi, r11d
0x14217fa6e mulss xmm7, xmm6
0x14217fa72 addss xmm5, xmm7
0x14217fa76 mulss xmm6, xmm1
0x14217fa7a test edi, edi
0x14217fa7c jne 0x14217fa40
0x14217fa7e xorps xmm6, xmm6
0x14217fa81 mov edi, r10d
0x14217fa84 movaps xmm7, xmm4
0x14217fa87 nop word ptr [rax + rax]
0x14217fa90 mov ebx, edi
0x14217fa92 xorps xmm8, xmm8
0x14217fa96 cvtsi2ss xmm8, rbx
0x14217fa9b imul rbx, rsi
0x14217fa9f shr rbx, 0x21
0x14217faa3 lea ebx, [rbx + rbx*2]
0x14217faa6 sub edi, ebx
0x14217faa8 xorps xmm9, xmm9
0x14217faac cvtsi2ss xmm9, edi
0x14217fab1 mulss xmm8, xmm4
0x14217fab6 cvttss2si rdi, xmm8
0x14217fabb ucomiss xmm8, xmm2
0x14217fabf cmovb edi, edx
0x14217fac2 ucomiss xmm8, xmm3
0x14217fac6 cmova edi, r11d
0x14217faca mulss xmm9, xmm7
0x14217facf addss xmm6, xmm9
0x14217fad4 mulss xmm7, xmm4
0x14217fad8 test edi, edi
0x14217fada jne 0x14217fa90
0x14217fadc unpcklps xmm5, xmm6
0x14217fadf jmp 0x14217fa13
0x14217fae4 jne 0x14217fbe4
0x14217faea test r9d, r9d
0x14217faed je 0x14217fbd5
0x14217faf3 movss xmm2, dword ptr [rip + 0x7a5d75] ; 0x142925870 0000003fc10ffc3e34a6c93900000042 f32=0.5
0x14217fafb xorps xmm3, xmm3
0x14217fafe xor edx, edx
0x14217fb00 movss xmm1, dword ptr [rip + 0x820cc8] ; 0x1429a07d0 ffff7f4f000080430000000000000000 f32=4294967040.0
0x14217fb08 mov r10d, 0xffffffff
0x14217fb0e mov r11d, r9d
0x14217fb11 xorps xmm0, xmm0
0x14217fb14 movaps xmm4, xmm2
0x14217fb17 nop word ptr [rax + rax]
0x14217fb20 mov esi, r11d
0x14217fb23 and r11d, 1
0x14217fb27 xorps xmm5, xmm5
0x14217fb2a cvtsi2ss xmm5, r11d
0x14217fb2f xorps xmm6, xmm6
0x14217fb32 cvtsi2ss xmm6, rsi
0x14217fb37 mulss xmm6, xmm2
0x14217fb3b cvttss2si r11, xmm6
0x14217fb40 ucomiss xmm6, xmm3
0x14217fb43 cmovb r11d, edx
0x14217fb47 ucomiss xmm6, xmm1
0x14217fb4a cmova r11d, r10d
0x14217fb4e mulss xmm5, xmm4
0x14217fb52 addss xmm0, xmm5
0x14217fb56 mulss xmm4, xmm2
0x14217fb5a test r11d, r11d
0x14217fb5d jne 0x14217fb20
0x14217fb5f movss xmm2, dword ptr [rip + 0x9067c9] ; 0x142a86330 abaaaa3e2f72757374632f3035663938 f32=0.3333333432674408
0x14217fb67 xorps xmm3, xmm3
0x14217fb6a mov edx, 0xaaaaaaab
0x14217fb6f xor r10d, r10d
0x14217fb72 mov r11d, 0xffffffff
0x14217fb78 xorps xmm4, xmm4
0x14217fb7b movaps xmm5, xmm2
0x14217fb7e nop 
0x14217fb80 mov esi, r9d
0x14217fb83 xorps xmm6, xmm6
0x14217fb86 cvtsi2ss xmm6, rsi
0x14217fb8b imul rsi, rdx
0x14217fb8f shr rsi, 0x21
0x14217fb93 lea esi, [rsi + rsi*2]
0x14217fb96 sub r9d, esi
0x14217fb99 xorps xmm7, xmm7
0x14217fb9c cvtsi2ss xmm7, r9d
0x14217fba1 mulss xmm6, xmm2
0x14217fba5 cvttss2si r9, xmm6
0x14217fbaa ucomiss xmm6, xmm3
0x14217fbad cmovb r9d, r10d
0x14217fbb1 ucomiss xmm6, xmm1
0x14217fbb4 cmova r9d, r11d
0x14217fbb8 mulss xmm7, xmm5
0x14217fbbc addss xmm4, xmm7
0x14217fbc0 mulss xmm5, xmm2
0x14217fbc4 test r9d, r9d
0x14217fbc7 jne 0x14217fb80
0x14217fbc9 unpcklps xmm0, xmm4
0x14217fbcc addps xmm0, xmmword ptr [rip + 0x89761d] ; 0x142a171f0 000000bf000000bf0000000000000000 f32=-0.5
0x14217fbd3 jmp 0x14217fbdc
0x14217fbd5 movaps xmm0, xmmword ptr [rip + 0x897614] ; 0x142a171f0 000000bf000000bf0000000000000000 f32=-0.5
0x14217fbdc movlps qword ptr [rcx + r8*8], xmm0
0x14217fbe1 inc r8
0x14217fbe4 mov qword ptr [rax], r8
0x14217fbe7 movaps xmm6, xmmword ptr [rsp]
0x14217fbeb movaps xmm7, xmmword ptr [rsp + 0x10]
0x14217fbf0 movaps xmm8, xmmword ptr [rsp + 0x20]
0x14217fbf6 movaps xmm9, xmmword ptr [rsp + 0x30]
0x14217fbfc add rsp, 0x40
0x14217fc00 pop rbx