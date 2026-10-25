0000000141233e50 mov       rax, rcx
0000000141233e53 movdqu    xmm0, xmmword ptr [r8]
0000000141233e58 movd      ecx, xmm0
0000000141233e5c and       ecx, 0x7fffffff
0000000141233e62 cmp       ecx, 0x7f800000
0000000141233e68 setge     cl
0000000141233e6b movshdup  xmm1, xmm0
0000000141233e6f movd      edx, xmm1
0000000141233e73 and       edx, 0x7fffffff
0000000141233e79 cmp       edx, 0x7f800000
0000000141233e7f setge     dl
0000000141233e82 or        dl, cl
0000000141233e84 movdqa    xmm1, xmm0
0000000141233e88 punpckhqdq xmm1, xmm0
0000000141233e8c movd      ecx, xmm1
0000000141233e90 and       ecx, 0x7fffffff
0000000141233e96 cmp       ecx, 0x7f800000
0000000141233e9c setge     r9b
0000000141233ea0 xor       ecx, ecx
0000000141233ea2 or        r9b, dl
0000000141233ea5 jne       0x141233f74
0000000141233eab movdqu    xmm1, xmmword ptr [r8 + 0x10]
0000000141233eb1 movaps    xmm2, xmm0
0000000141233eb4 shufps    xmm2, xmm0, 0xff
0000000141233eb8 movd      edx, xmm2
0000000141233ebc and       edx, 0x7fffffff
0000000141233ec2 cmp       edx, 0x7f800000
0000000141233ec8 setge     dl
0000000141233ecb movd      r9d, xmm1
0000000141233ed0 and       r9d, 0x7fffffff
0000000141233ed7 cmp       r9d, 0x7f800000
0000000141233ede setge     r9b
0000000141233ee2 or        r9b, dl
0000000141233ee5 movshdup  xmm2, xmm1
0000000141233ee9 movd      edx, xmm2
0000000141233eed and       edx, 0x7fffffff
0000000141233ef3 cmp       edx, 0x7f800000
0000000141233ef9 setge     dl
0000000141233efc or        dl, r9b
0000000141233eff jne       0x141233f74
0000000141233f01 movd      xmm2, dword ptr [r8 + 0x20]
0000000141233f07 movdqa    xmm3, xmm1
0000000141233f0b punpckhqdq xmm3, xmm1
0000000141233f0f movd      edx, xmm3
0000000141233f13 and       edx, 0x7fffffff
0000000141233f19 cmp       edx, 0x7f800000
0000000141233f1f setge     dl
0000000141233f22 movaps    xmm3, xmm1
0000000141233f25 shufps    xmm3, xmm1, 0xff
0000000141233f29 movd      r9d, xmm3
0000000141233f2e and       r9d, 0x7fffffff
0000000141233f35 cmp       r9d, 0x7f800000
0000000141233f3c setge     r9b
0000000141233f40 or        r9b, dl
0000000141233f43 movd      edx, xmm2
0000000141233f47 and       edx, 0x7fffffff
0000000141233f4d cmp       edx, 0x7f800000
0000000141233f53 setge     dl
0000000141233f56 or        dl, r9b
0000000141233f59 jne       0x141233f74
0000000141233f5b mov       ecx, dword ptr [r8 + 0x24]
0000000141233f5f movups    xmmword ptr [rax + 4], xmm0
0000000141233f63 movups    xmmword ptr [rax + 0x14], xmm1
0000000141233f67 movd      dword ptr [rax + 0x24], xmm2
0000000141233f6c mov       dword ptr [rax + 0x28], ecx
0000000141233f6f mov       ecx, 1
0000000141233f74 mov       dword ptr [rax], ecx
0000000141233f76 ret       
