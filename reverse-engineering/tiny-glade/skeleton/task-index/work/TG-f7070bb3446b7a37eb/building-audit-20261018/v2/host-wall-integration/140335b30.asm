0x140335b30 55 push rbp
0x140335b31 56 push rsi
0x140335b32 57 push rdi
0x140335b33 53 push rbx
0x140335b34 4881ec680a0000 sub rsp, 0xa68
0x140335b3b 488dac2480000000 lea rbp, [rsp + 0x80]
0x140335b43 48c785e0090000feffffff mov qword ptr [rbp + 0x9e0], 0xfffffffffffffffe
0x140335b4e 4889ce mov rsi, rcx
0x140335b51 0fb605b1f02f03 movzx eax, byte ptr [rip + 0x32ff0b1]
0x140335b58 b900070000 mov ecx, 0x700
0x140335b5d ba08000000 mov edx, 8
0x140335b62 e8a9e02d00 call 0x140613c10
0x140335b67 4885c0 test rax, rax
0x140335b6a 0f848a080000 je 0x1403363fa
0x140335b70 488985d8090000 mov qword ptr [rbp + 0x9d8], rax
0x140335b77 0f57c0 xorps xmm0, xmm0
0x140335b7a 0f1145c0 movups xmmword ptr [rbp - 0x40], xmm0
0x140335b7e 0f1145d8 movups xmmword ptr [rbp - 0x28], xmm0
0x140335b82 0f1145f0 movups xmmword ptr [rbp - 0x10], xmm0
0x140335b86 0f114508 movups xmmword ptr [rbp + 8], xmm0
0x140335b8a 0f114520 movups xmmword ptr [rbp + 0x20], xmm0
0x140335b8e c7453000000000 mov dword ptr [rbp + 0x30], 0
0x140335b95 0f114558 movups xmmword ptr [rbp + 0x58], xmm0
0x140335b99 0f114570 movups xmmword ptr [rbp + 0x70], xmm0
0x140335b9d 0f118588000000 movups xmmword ptr [rbp + 0x88], xmm0
0x140335ba4 0f1185a0000000 movups xmmword ptr [rbp + 0xa0], xmm0
0x140335bab 0f1185b8000000 movups xmmword ptr [rbp + 0xb8], xmm0
0x140335bb2 c785c800000000000000 mov dword ptr [rbp + 0xc8], 0
0x140335bbc 48c745a000000000 mov qword ptr [rbp - 0x60], 0
0x140335bc4 48c745a808000000 mov qword ptr [rbp - 0x58], 8
0x140335bcc 48c745b000000000 mov qword ptr [rbp - 0x50], 0
0x140335bd4 48c745b810000000 mov qword ptr [rbp - 0x48], 0x10
0x140335bdc 48c745d010000000 mov qword ptr [rbp - 0x30], 0x10
0x140335be4 48c745e810000000 mov qword ptr [rbp - 0x18], 0x10
0x140335bec 48c7450010000000 mov qword ptr [rbp], 0x10
0x140335bf4 48c7451810000000 mov qword ptr [rbp + 0x18], 0x10
0x140335bfc 48bb0000000000000080 movabs rbx, 0x8000000000000000
0x140335c06 48895d38 mov qword ptr [rbp + 0x38], rbx
0x140335c0a 488d059fa16802 lea rax, [rip + 0x268a19f]
0x140335c11 48894540 mov qword ptr [rbp + 0x40], rax
0x140335c15 48c745486f000000 mov qword ptr [rbp + 0x48], 0x6f
0x140335c1d 48c7455010000000 mov qword ptr [rbp + 0x50], 0x10
0x140335c25 48c7456810000000 mov qword ptr [rbp + 0x68], 0x10
0x140335c2d 48c7858000000010000000 mov qword ptr [rbp + 0x80], 0x10
0x140335c38 48c7859800000010000000 mov qword ptr [rbp + 0x98], 0x10
0x140335c43 48c785b000000010000000 mov qword ptr [rbp + 0xb0], 0x10
0x140335c4e c785d000000000000000 mov dword ptr [rbp + 0xd0], 0
0x140335c58 66c785d40000000100 mov word ptr [rbp + 0xd4], 1
0x140335c61 48899dc8010000 mov qword ptr [rbp + 0x1c8], rbx
0x140335c68 c7854003000000000000 mov dword ptr [rbp + 0x340], 0
0x140335c72 0fb60590ef2f03 movzx eax, byte ptr [rip + 0x32fef90]
0x140335c79 b9a8030000 mov ecx, 0x3a8
0x140335c7e ba08000000 mov edx, 8
0x140335c83 e888df2d00 call 0x140613c10
0x140335c88 4885c0 test rax, rax
0x140335c8b 0f8478070000 je 0x140336409
0x140335c91 4889c7 mov rdi, rax
0x140335c94 488d55a0 lea rdx, [rbp - 0x60]
0x140335c98 41b8a8030000 mov r8d, 0x3a8
0x140335c9e 4889c1 mov rcx, rax
0x140335ca1 e85adf5e02 call 0x142923c00
0x140335ca6 4c8d05cb9e6802 lea r8, [rip + 0x2689ecb]
0x140335cad 488d8df8080000 lea rcx, [rbp + 0x8f8]
0x140335cb4 4889fa mov rdx, rdi
0x140335cb7 e854ee9a00 call 0x140ce4b10
0x140335cbc 488d8d88080000 lea rcx, [rbp + 0x888]
0x140335cc3 e8885ed9ff call 0x1400cbb50
0x140335cc8 488d8d18080000 lea rcx, [rbp + 0x818]
0x140335ccf e8dc6fe2ff call 0x14015ccb0
0x140335cd4 488d8da8070000 lea rcx, [rbp + 0x7a8]
0x140335cdb e830ceddff call 0x140112b10
0x140335ce0 488d8d38070000 lea rcx, [rbp + 0x738]
0x140335ce7 e884ae2700 call 0x1405b0b70
0x140335cec 488d8dc8060000 lea rcx, [rbp + 0x6c8]
0x140335cf3 e8d89ef2ff call 0x14025fbd0
0x140335cf8 488d8d58060000 lea rcx, [rbp + 0x658]
0x140335cff e80cc21f00 call 0x140531f10
0x140335d04 488d8de8050000 lea rcx, [rbp + 0x5e8]
0x140335d0b e820f3f5ff call 0x140295030
0x140335d10 488d8d78050000 lea rcx, [rbp + 0x578]
0x140335d17 e8f4a0faff call 0x1402dfe10
0x140335d1c 488d8d08050000 lea rcx, [rbp + 0x508]
0x140335d23 e868c7f1ff call 0x140252490
0x140335d28 488d8d98040000 lea rcx, [rbp + 0x498]
0x140335d2f e87c122900 call 0x1405c6fb0
0x140335d34 488d8d28040000 lea rcx, [rbp + 0x428]
0x140335d3b e810330300 call 0x140369050
0x140335d40 488d8db8030000 lea rcx, [rbp + 0x3b8]
0x140335d47 e844310300 call 0x140368e90
0x140335d4c 488d8d48030000 lea rcx, [rbp + 0x348]
0x140335d53 e838111b00 call 0x1404e6e90
0x140335d58 488d8d68090000 lea rcx, [rbp + 0x968]
0x140335d5f e84cf0e3ff call 0x140174db0
0x140335d64 488d4da0 lea rcx, [rbp - 0x60]
0x140335d68 e8a3222000 call 0x140538010
0x140335d6d 0f108558090000 movups xmm0, xmmword ptr [rbp + 0x958]
0x140335d74 488b85d8090000 mov rax, qword ptr [rbp + 0x9d8]
0x140335d7b 0f114060 movups xmmword ptr [rax + 0x60], xmm0
0x140335d7f 0f108548090000 movups xmm0, xmmword ptr [rbp + 0x948]
0x140335d86 0f114050 movups xmmword ptr [rax + 0x50], xmm0
0x140335d8a 0f108538090000 movups xmm0, xmmword ptr [rbp + 0x938]
0x140335d91 0f114040 movups xmmword ptr [rax + 0x40], xmm0
0x140335d95 0f1085f8080000 movups xmm0, xmmword ptr [rbp + 0x8f8]
0x140335d9c 0f108d08090000 movups xmm1, xmmword ptr [rbp + 0x908]
0x140335da3 0f109518090000 movups xmm2, xmmword ptr [rbp + 0x918]
0x140335daa 0f109d28090000 movups xmm3, xmmword ptr [rbp + 0x928]
0x140335db1 0f115830 movups xmmword ptr [rax + 0x30], xmm3
0x140335db5 0f115020 movups xmmword ptr [rax + 0x20], xmm2
0x140335db9 0f114810 movups xmmword ptr [rax + 0x10], xmm1
0x140335dbd 0f1100 movups xmmword ptr [rax], xmm0
0x140335dc0 0f108588080000 movups xmm0, xmmword ptr [rbp + 0x888]
0x140335dc7 0f108d98080000 movups xmm1, xmmword ptr [rbp + 0x898]
0x140335dce 0f1095a8080000 movups xmm2, xmmword ptr [rbp + 0x8a8]
0x140335dd5 0f109db8080000 movups xmm3, xmmword ptr [rbp + 0x8b8]
0x140335ddc 0f114070 movups xmmword ptr [rax + 0x70], xmm0
0x140335de0 0f118880000000 movups xmmword ptr [rax + 0x80], xmm1
0x140335de7 0f119090000000 movups xmmword ptr [rax + 0x90], xmm2
0x140335dee 0f1198a0000000 movups xmmword ptr [rax + 0xa0], xmm3
0x140335df5 0f1085c8080000 movups xmm0, xmmword ptr [rbp + 0x8c8]
0x140335dfc 0f1180b0000000 movups xmmword ptr [rax + 0xb0], xmm0
0x140335e03 0f1085d8080000 movups xmm0, xmmword ptr [rbp + 0x8d8]
0x140335e0a 0f1180c0000000 movups xmmword ptr [rax + 0xc0], xmm0
0x140335e11 0f1085e8080000 movups xmm0, xmmword ptr [rbp + 0x8e8]
0x140335e18 0f1180d0000000 movups xmmword ptr [rax + 0xd0], xmm0
0x140335e1f 0f108578080000 movups xmm0, xmmword ptr [rbp + 0x878]
0x140335e26 0f118040010000 movups xmmword ptr [rax + 0x140], xmm0
0x140335e2d 0f108568080000 movups xmm0, xmmword ptr [rbp + 0x868]
0x140335e34 0f118030010000 movups xmmword ptr [rax + 0x130], xmm0
0x140335e3b 0f108558080000 movups xmm0, xmmword ptr [rbp + 0x858]
0x140335e42 0f118020010000 movups xmmword ptr [rax + 0x120], xmm0
0x140335e49 0f108518080000 movups xmm0, xmmword ptr [rbp + 0x818]
0x140335e50 0f108d28080000 movups xmm1, xmmword ptr [rbp + 0x828]
0x140335e57 0f109538080000 movups xmm2, xmmword ptr [rbp + 0x838]
0x140335e5e 0f109d48080000 movups xmm3, xmmword ptr [rbp + 0x848]
0x140335e65 0f119810010000 movups xmmword ptr [rax + 0x110], xmm3
0x140335e6c 0f119000010000 movups xmmword ptr [rax + 0x100], xmm2
0x140335e73 0f1188f0000000 movups xmmword ptr [rax + 0xf0], xmm1
0x140335e7a 0f1180e0000000 movups xmmword ptr [rax + 0xe0], xmm0
0x140335e81 0f1085a8070000 movups xmm0, xmmword ptr [rbp + 0x7a8]
0x140335e88 0f108db8070000 movups xmm1, xmmword ptr [rbp + 0x7b8]
0x140335e8f 0f1095c8070000 movups xmm2, xmmword ptr [rbp + 0x7c8]
0x140335e96 0f109dd8070000 movups xmm3, xmmword ptr [rbp + 0x7d8]
0x140335e9d 0f118050010000 movups xmmword ptr [rax + 0x150], xmm0
0x140335ea4 0f118860010000 movups xmmword ptr [rax + 0x160], xmm1
0x140335eab 0f119070010000 movups xmmword ptr [rax + 0x170], xmm2
0x140335eb2 0f119880010000 movups xmmword ptr [rax + 0x180], xmm3
0x140335eb9 0f1085e8070000 movups xmm0, xmmword ptr [rbp + 0x7e8]
0x140335ec0 0f118090010000 movups xmmword ptr [rax + 0x190], xmm0
0x140335ec7 0f1085f8070000 movups xmm0, xmmword ptr [rbp + 0x7f8]
0x140335ece 0f1180a0010000 movups xmmword ptr [rax + 0x1a0], xmm0
0x140335ed5 0f108508080000 movups xmm0, xmmword ptr [rbp + 0x808]
0x140335edc 0f1180b0010000 movups xmmword ptr [rax + 0x1b0], xmm0
0x140335ee3 0f108538070000 movups xmm0, xmmword ptr [rbp + 0x738]
0x140335eea 0f108d48070000 movups xmm1, xmmword ptr [rbp + 0x748]
0x140335ef1 0f109558070000 movups xmm2, xmmword ptr [rbp + 0x758]
0x140335ef8 0f109d68070000 movups xmm3, xmmword ptr [rbp + 0x768]
0x140335eff 0f1180c0010000 movups xmmword ptr [rax + 0x1c0], xmm0
0x140335f06 0f1188d0010000 movups xmmword ptr [rax + 0x1d0], xmm1
0x140335f0d 0f1190e0010000 movups xmmword ptr [rax + 0x1e0], xmm2
0x140335f14 0f1198f0010000 movups xmmword ptr [rax + 0x1f0], xmm3
0x140335f1b 0f108578070000 movups xmm0, xmmword ptr [rbp + 0x778]
0x140335f22 0f118000020000 movups xmmword ptr [rax + 0x200], xmm0
0x140335f29 0f108598070000 movups xmm0, xmmword ptr [rbp + 0x798]
0x140335f30 0f118020020000 movups xmmword ptr [rax + 0x220], xmm0
0x140335f37 0f108588070000 movups xmm0, xmmword ptr [rbp + 0x788]
0x140335f3e 0f118010020000 movups xmmword ptr [rax + 0x210], xmm0
0x140335f45 0f108528070000 movups xmm0, xmmword ptr [rbp + 0x728]
0x140335f4c 0f118090020000 movups xmmword ptr [rax + 0x290], xmm0
0x140335f53 0f108518070000 movups xmm0, xmmword ptr [rbp + 0x718]
0x140335f5a 0f118080020000 movups xmmword ptr [rax + 0x280], xmm0
0x140335f61 0f108508070000 movups xmm0, xmmword ptr [rbp + 0x708]
0x140335f68 0f118070020000 movups xmmword ptr [rax + 0x270], xmm0
0x140335f6f 0f1085c8060000 movups xmm0, xmmword ptr [rbp + 0x6c8]
0x140335f76 0f108dd8060000 movups xmm1, xmmword ptr [rbp + 0x6d8]
0x140335f7d 0f1095e8060000 movups xmm2, xmmword ptr [rbp + 0x6e8]
0x140335f84 0f109df8060000 movups xmm3, xmmword ptr [rbp + 0x6f8]
0x140335f8b 0f119860020000 movups xmmword ptr [rax + 0x260], xmm3
0x140335f92 0f119050020000 movups xmmword ptr [rax + 0x250], xmm2
0x140335f99 0f118840020000 movups xmmword ptr [rax + 0x240], xmm1
0x140335fa0 0f118030020000 movups xmmword ptr [rax + 0x230], xmm0
0x140335fa7 0f108558060000 movups xmm0, xmmword ptr [rbp + 0x658]
0x140335fae 0f108d68060000 movups xmm1, xmmword ptr [rbp + 0x668]
0x140335fb5 0f109578060000 movups xmm2, xmmword ptr [rbp + 0x678]
0x140335fbc 0f109d88060000 movups xmm3, xmmword ptr [rbp + 0x688]
0x140335fc3 0f1180a0020000 movups xmmword ptr [rax + 0x2a0], xmm0
0x140335fca 0f1188b0020000 movups xmmword ptr [rax + 0x2b0], xmm1
0x140335fd1 0f1190c0020000 movups xmmword ptr [rax + 0x2c0], xmm2
0x140335fd8 0f1198d0020000 movups xmmword ptr [rax + 0x2d0], xmm3
0x140335fdf 0f108598060000 movups xmm0, xmmword ptr [rbp + 0x698]
0x140335fe6 0f1180e0020000 movups xmmword ptr [rax + 0x2e0], xmm0
0x140335fed 0f1085a8060000 movups xmm0, xmmword ptr [rbp + 0x6a8]
0x140335ff4 0f1180f0020000 movups xmmword ptr [rax + 0x2f0], xmm0
0x140335ffb 0f1085b8060000 movups xmm0, xmmword ptr [rbp + 0x6b8]
0x140336002 0f118000030000 movups xmmword ptr [rax + 0x300], xmm0
0x140336009 0f1085e8050000 movups xmm0, xmmword ptr [rbp + 0x5e8]
0x140336010 0f108df8050000 movups xmm1, xmmword ptr [rbp + 0x5f8]
0x140336017 0f109508060000 movups xmm2, xmmword ptr [rbp + 0x608]
0x14033601e 0f109d18060000 movups xmm3, xmmword ptr [rbp + 0x618]
0x140336025 0f118010030000 movups xmmword ptr [rax + 0x310], xmm0
0x14033602c 0f118820030000 movups xmmword ptr [rax + 0x320], xmm1
0x140336033 0f119030030000 movups xmmword ptr [rax + 0x330], xmm2
0x14033603a 0f119840030000 movups xmmword ptr [rax + 0x340], xmm3
0x140336041 0f108528060000 movups xmm0, xmmword ptr [rbp + 0x628]
0x140336048 0f118050030000 movups xmmword ptr [rax + 0x350], xmm0
0x14033604f 0f108538060000 movups xmm0, xmmword ptr [rbp + 0x638]
0x140336056 0f118060030000 movups xmmword ptr [rax + 0x360], xmm0
0x14033605d 0f108548060000 movups xmm0, xmmword ptr [rbp + 0x648]
0x140336064 0f118070030000 movups xmmword ptr [rax + 0x370], xmm0
0x14033606b 0f108578050000 movups xmm0, xmmword ptr [rbp + 0x578]
0x140336072 0f108d88050000 movups xmm1, xmmword ptr [rbp + 0x588]
0x140336079 0f109598050000 movups xmm2, xmmword ptr [rbp + 0x598]
0x140336080 0f109da8050000 movups xmm3, xmmword ptr [rbp + 0x5a8]
0x140336087 0f118080030000 movups xmmword ptr [rax + 0x380], xmm0
0x14033608e 0f118890030000 movups xmmword ptr [rax + 0x390], xmm1
0x140336095 0f1190a0030000 movups xmmword ptr [rax + 0x3a0], xmm2
0x14033609c 0f1198b0030000 movups xmmword ptr [rax + 0x3b0], xmm3
0x1403360a3 0f1085b8050000 movups xmm0, xmmword ptr [rbp + 0x5b8]
0x1403360aa 0f1180c0030000 movups xmmword ptr [rax + 0x3c0], xmm0
0x1403360b1 0f1085c8050000 movups xmm0, xmmword ptr [rbp + 0x5c8]
0x1403360b8 0f1180d0030000 movups xmmword ptr [rax + 0x3d0], xmm0
0x1403360bf 0f1085d8050000 movups xmm0, xmmword ptr [rbp + 0x5d8]
0x1403360c6 0f1180e0030000 movups xmmword ptr [rax + 0x3e0], xmm0
0x1403360cd 0f108508050000 movups xmm0, xmmword ptr [rbp + 0x508]
0x1403360d4 0f108d18050000 movups xmm1, xmmword ptr [rbp + 0x518]
0x1403360db 0f109528050000 movups xmm2, xmmword ptr [rbp + 0x528]
0x1403360e2 0f109d38050000 movups xmm3, xmmword ptr [rbp + 0x538]
0x1403360e9 0f1180f0030000 movups xmmword ptr [rax + 0x3f0], xmm0
0x1403360f0 0f118800040000 movups xmmword ptr [rax + 0x400], xmm1
0x1403360f7 0f119010040000 movups xmmword ptr [rax + 0x410], xmm2
0x1403360fe 0f119820040000 movups xmmword ptr [rax + 0x420], xmm3
0x140336105 0f108548050000 movups xmm0, xmmword ptr [rbp + 0x548]
0x14033610c 0f118030040000 movups xmmword ptr [rax + 0x430], xmm0
0x140336113 0f108558050000 movups xmm0, xmmword ptr [rbp + 0x558]
0x14033611a 0f118040040000 movups xmmword ptr [rax + 0x440], xmm0
0x140336121 0f108568050000 movups xmm0, xmmword ptr [rbp + 0x568]
0x140336128 0f118050040000 movups xmmword ptr [rax + 0x450], xmm0
0x14033612f 0f108598040000 movups xmm0, xmmword ptr [rbp + 0x498]
0x140336136 0f108da8040000 movups xmm1, xmmword ptr [rbp + 0x4a8]
0x14033613d 0f1095b8040000 movups xmm2, xmmword ptr [rbp + 0x4b8]
0x140336144 0f109dc8040000 movups xmm3, xmmword ptr [rbp + 0x4c8]
0x14033614b 0f118060040000 movups xmmword ptr [rax + 0x460], xmm0
0x140336152 0f118870040000 movups xmmword ptr [rax + 0x470], xmm1
0x140336159 0f119080040000 movups xmmword ptr [rax + 0x480], xmm2
0x140336160 0f119890040000 movups xmmword ptr [rax + 0x490], xmm3
0x140336167 0f1085d8040000 movups xmm0, xmmword ptr [rbp + 0x4d8]
0x14033616e 0f1180a0040000 movups xmmword ptr [rax + 0x4a0], xmm0
0x140336175 0f1085e8040000 movups xmm0, xmmword ptr [rbp + 0x4e8]
0x14033617c 0f1180b0040000 movups xmmword ptr [rax + 0x4b0], xmm0
0x140336183 0f1085f8040000 movups xmm0, xmmword ptr [rbp + 0x4f8]
0x14033618a 0f1180c0040000 movups xmmword ptr [rax + 0x4c0], xmm0
0x140336191 0f108528040000 movups xmm0, xmmword ptr [rbp + 0x428]
0x140336198 0f108d38040000 movups xmm1, xmmword ptr [rbp + 0x438]
0x14033619f 0f109548040000 movups xmm2, xmmword ptr [rbp + 0x448]
0x1403361a6 0f109d58040000 movups xmm3, xmmword ptr [rbp + 0x458]
0x1403361ad 0f1180d0040000 movups xmmword ptr [rax + 0x4d0], xmm0
0x1403361b4 0f1188e0040000 movups xmmword ptr [rax + 0x4e0], xmm1
0x1403361bb 0f1190f0040000 movups xmmword ptr [rax + 0x4f0], xmm2
0x1403361c2 0f119800050000 movups xmmword ptr [rax + 0x500], xmm3
0x1403361c9 0f108568040000 movups xmm0, xmmword ptr [rbp + 0x468]
0x1403361d0 0f118010050000 movups xmmword ptr [rax + 0x510], xmm0
0x1403361d7 0f108578040000 movups xmm0, xmmword ptr [rbp + 0x478]
0x1403361de 0f118020050000 movups xmmword ptr [rax + 0x520], xmm0
0x1403361e5 0f108588040000 movups xmm0, xmmword ptr [rbp + 0x488]
0x1403361ec 0f118030050000 movups xmmword ptr [rax + 0x530], xmm0
0x1403361f3 0f1085b8030000 movups xmm0, xmmword ptr [rbp + 0x3b8]
0x1403361fa 0f108dc8030000 movups xmm1, xmmword ptr [rbp + 0x3c8]
0x140336201 0f1095d8030000 movups xmm2, xmmword ptr [rbp + 0x3d8]
0x140336208 0f109de8030000 movups xmm3, xmmword ptr [rbp + 0x3e8]
0x14033620f 0f118040050000 movups xmmword ptr [rax + 0x540], xmm0
0x140336216 0f118850050000 movups xmmword ptr [rax + 0x550], xmm1
0x14033621d 0f119060050000 movups xmmword ptr [rax + 0x560], xmm2
0x140336224 0f119870050000 movups xmmword ptr [rax + 0x570], xmm3
0x14033622b 0f1085f8030000 movups xmm0, xmmword ptr [rbp + 0x3f8]
0x140336232 0f118080050000 movups xmmword ptr [rax + 0x580], xmm0
0x140336239 0f108508040000 movups xmm0, xmmword ptr [rbp + 0x408]
0x140336240 0f118090050000 movups xmmword ptr [rax + 0x590], xmm0
0x140336247 0f108518040000 movups xmm0, xmmword ptr [rbp + 0x418]
0x14033624e 0f1180a0050000 movups xmmword ptr [rax + 0x5a0], xmm0
0x140336255 0f108548030000 movups xmm0, xmmword ptr [rbp + 0x348]
0x14033625c 0f108d58030000 movups xmm1, xmmword ptr [rbp + 0x358]
0x140336263 0f109568030000 movups xmm2, xmmword ptr [rbp + 0x368]
0x14033626a 0f109d78030000 movups xmm3, xmmword ptr [rbp + 0x378]
0x140336271 0f1180b0050000 movups xmmword ptr [rax + 0x5b0], xmm0
0x140336278 0f1188c0050000 movups xmmword ptr [rax + 0x5c0], xmm1
0x14033627f 0f1190d0050000 movups xmmword ptr [rax + 0x5d0], xmm2
0x140336286 0f1198e0050000 movups xmmword ptr [rax + 0x5e0], xmm3
0x14033628d 0f108588030000 movups xmm0, xmmword ptr [rbp + 0x388]
0x140336294 0f1180f0050000 movups xmmword ptr [rax + 0x5f0], xmm0
0x14033629b 0f108598030000 movups xmm0, xmmword ptr [rbp + 0x398]
0x1403362a2 0f118000060000 movups xmmword ptr [rax + 0x600], xmm0
0x1403362a9 0f1085a8030000 movups xmm0, xmmword ptr [rbp + 0x3a8]
0x1403362b0 0f118010060000 movups xmmword ptr [rax + 0x610], xmm0
0x1403362b7 0f108568090000 movups xmm0, xmmword ptr [rbp + 0x968]
0x1403362be 0f108d78090000 movups xmm1, xmmword ptr [rbp + 0x978]
0x1403362c5 0f109588090000 movups xmm2, xmmword ptr [rbp + 0x988]
0x1403362cc 0f109d98090000 movups xmm3, xmmword ptr [rbp + 0x998]
0x1403362d3 0f118020060000 movups xmmword ptr [rax + 0x620], xmm0
0x1403362da 0f118830060000 movups xmmword ptr [rax + 0x630], xmm1
0x1403362e1 0f119040060000 movups xmmword ptr [rax + 0x640], xmm2
0x1403362e8 0f119850060000 movups xmmword ptr [rax + 0x650], xmm3
0x1403362ef 0f1085a8090000 movups xmm0, xmmword ptr [rbp + 0x9a8]
0x1403362f6 0f118060060000 movups xmmword ptr [rax + 0x660], xmm0
0x1403362fd 0f1085b8090000 movups xmm0, xmmword ptr [rbp + 0x9b8]
0x140336304 0f118070060000 movups xmmword ptr [rax + 0x670], xmm0
0x14033630b 0f1085c8090000 movups xmm0, xmmword ptr [rbp + 0x9c8]
0x140336312 0f118080060000 movups xmmword ptr [rax + 0x680], xmm0
0x140336319 0f1045a0 movups xmm0, xmmword ptr [rbp - 0x60]
0x14033631d 0f104db0 movups xmm1, xmmword ptr [rbp - 0x50]
0x140336321 0f1055c0 movups xmm2, xmmword ptr [rbp - 0x40]
0x140336325 0f105dd0 movups xmm3, xmmword ptr [rbp - 0x30]
0x140336329 0f118090060000 movups xmmword ptr [rax + 0x690], xmm0
0x140336330 0f1188a0060000 movups xmmword ptr [rax + 0x6a0], xmm1
0x140336337 0f1190b0060000 movups xmmword ptr [rax + 0x6b0], xmm2
0x14033633e 0f1198c0060000 movups xmmword ptr [rax + 0x6c0], xmm3
0x140336345 0f1045e0 movups xmm0, xmmword ptr [rbp - 0x20]
0x140336349 0f1180d0060000 movups xmmword ptr [rax + 0x6d0], xmm0
0x140336350 0f104500 movups xmm0, xmmword ptr [rbp]
0x140336354 0f1180f0060000 movups xmmword ptr [rax + 0x6f0], xmm0
0x14033635b 0f1045f0 movups xmm0, xmmword ptr [rbp - 0x10]
0x14033635f 0f1180e0060000 movups xmmword ptr [rax + 0x6e0], xmm0
0x140336366 48c745a810000000 mov qword ptr [rbp - 0x58], 0x10
0x14033636e 488945b0 mov qword ptr [rbp - 0x50], rax
0x140336372 48c745b810000000 mov qword ptr [rbp - 0x48], 0x10
0x14033637a 48c745c000000000 mov qword ptr [rbp - 0x40], 0
0x140336382 48c745c808000000 mov qword ptr [rbp - 0x38], 8
0x14033638a 0f57c0 xorps xmm0, xmm0
0x14033638d 0f1145d0 movups xmmword ptr [rbp - 0x30], xmm0
0x140336391 488b8578090000 mov rax, qword ptr [rbp + 0x978]
0x140336398 488945f0 mov qword ptr [rbp - 0x10], rax
0x14033639c 0f108568090000 movups xmm0, xmmword ptr [rbp + 0x968]
0x1403363a3 0f1145e0 movups xmmword ptr [rbp - 0x20], xmm0
0x1403363a7 48895da0 mov qword ptr [rbp - 0x60], rbx
0x1403363ab 488d4dd8 lea rcx, [rbp - 0x28]
0x1403363af e81ca2e3ff call 0x1401705d0
0x1403363b4 0f104500 movups xmm0, xmmword ptr [rbp]
0x1403363b8 0f114660 movups xmmword ptr [rsi + 0x60], xmm0
0x1403363bc 0f1045f0 movups xmm0, xmmword ptr [rbp - 0x10]
0x1403363c0 0f114650 movups xmmword ptr [rsi + 0x50], xmm0
0x1403363c4 0f1045e0 movups xmm0, xmmword ptr [rbp - 0x20]
0x1403363c8 0f114640 movups xmmword ptr [rsi + 0x40], xmm0
0x1403363cc 0f1045a0 movups xmm0, xmmword ptr [rbp - 0x60]
0x1403363d0 0f104db0 movups xmm1, xmmword ptr [rbp - 0x50]
0x1403363d4 0f1055c0 movups xmm2, xmmword ptr [rbp - 0x40]
0x1403363d8 0f105dd0 movups xmm3, xmmword ptr [rbp - 0x30]
0x1403363dc 0f115e30 movups xmmword ptr [rsi + 0x30], xmm3
0x1403363e0 0f115620 movups xmmword ptr [rsi + 0x20], xmm2
0x1403363e4 0f114e10 movups xmmword ptr [rsi + 0x10], xmm1
0x1403363e8 0f1106 movups xmmword ptr [rsi], xmm0
0x1403363eb 4889f0 mov rax, rsi
0x1403363ee 4881c4680a0000 add rsp, 0xa68
0x1403363f5 5b pop rbx
0x1403363f6 5f pop rdi
0x1403363f7 5e pop rsi
0x1403363f8 5d pop rbp
0x1403363f9 c3 ret 
0x1403363fa b908000000 mov ecx, 8
0x1403363ff ba00070000 mov edx, 0x700
0x140336404 e8f72b5a02 call 0x1428d9000
0x140336409 b908000000 mov ecx, 8
0x14033640e baa8030000 mov edx, 0x3a8
0x140336413 e8e82b5a02 call 0x1428d9000
0x140336418 0f0b ud2 
0x14033641a 660f1f440000 nop word ptr [rax + rax]
0x140336420 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140336425 55 push rbp
0x140336426 56 push rsi
0x140336427 57 push rdi
0x140336428 53 push rbx
0x140336429 4883ec28 sub rsp, 0x28
0x14033642d 488daa80000000 lea rbp, [rdx + 0x80]
0x140336434 488d4da0 lea rcx, [rbp - 0x60]
0x140336438 e8b3b32900 call 0x1405d17f0
0x14033643d 90 nop 
0x14033643e 4883c428 add rsp, 0x28
0x140336442 5b pop rbx
0x140336443 5f pop rdi
0x140336444 5e pop rsi
0x140336445 5d pop rbp
0x140336446 c3 ret 
0x140336447 660f1f840000000000 nop word ptr [rax + rax]
0x140336450 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140336455 55 push rbp
0x140336456 56 push rsi
0x140336457 57 push rdi
0x140336458 53 push rbx
0x140336459 4883ec28 sub rsp, 0x28
0x14033645d 488daa80000000 lea rbp, [rdx + 0x80]
0x140336464 ba00070000 mov edx, 0x700
0x140336469 41b808000000 mov r8d, 8
0x14033646f 488b8dd8090000 mov rcx, qword ptr [rbp + 0x9d8]
0x140336476 e8a5d72d00 call 0x140613c20
0x14033647b 90 nop 
0x14033647c 4883c428 add rsp, 0x28
0x140336480 5b pop rbx
0x140336481 5f pop rdi
0x140336482 5e pop rsi
0x140336483 5d pop rbp
0x140336484 c3 ret 
0x140336485 66662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x140336490 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140336495 55 push rbp
0x140336496 56 push rsi
0x140336497 57 push rdi
0x140336498 53 push rbx
0x140336499 4883ec28 sub rsp, 0x28
0x14033649d 488daa80000000 lea rbp, [rdx + 0x80]
0x1403364a4 488d8df8080000 lea rcx, [rbp + 0x8f8]
0x1403364ab e8d033cfff call 0x140029880
0x1403364b0 90 nop 
0x1403364b1 4883c428 add rsp, 0x28
0x1403364b5 5b pop rbx
0x1403364b6 5f pop rdi
0x1403364b7 5e pop rsi
0x1403364b8 5d pop rbp
0x1403364b9 c3 ret 
0x1403364ba 660f1f440000 nop word ptr [rax + rax]
0x1403364c0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1403364c5 55 push rbp
0x1403364c6 56 push rsi
0x1403364c7 57 push rdi
0x1403364c8 53 push rbx
0x1403364c9 4883ec28 sub rsp, 0x28
0x1403364cd 488daa80000000 lea rbp, [rdx + 0x80]
0x1403364d4 488d8d88080000 lea rcx, [rbp + 0x888]
0x1403364db e8a033cfff call 0x140029880
0x1403364e0 90 nop 
0x1403364e1 4883c428 add rsp, 0x28
0x1403364e5 5b pop rbx
0x1403364e6 5f pop rdi
0x1403364e7 5e pop rsi
0x1403364e8 5d pop rbp
0x1403364e9 c3 ret 
0x1403364ea 660f1f440000 nop word ptr [rax + rax]
0x1403364f0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1403364f5 55 push rbp
0x1403364f6 56 push rsi
0x1403364f7 57 push rdi
0x1403364f8 53 push rbx
0x1403364f9 4883ec28 sub rsp, 0x28
0x1403364fd 488daa80000000 lea rbp, [rdx + 0x80]
0x140336504 488d8d18080000 lea rcx, [rbp + 0x818]
0x14033650b e87033cfff call 0x140029880
0x140336510 90 nop 
0x140336511 4883c428 add rsp, 0x28
0x140336515 5b pop rbx
0x140336516 5f pop rdi
0x140336517 5e pop rsi
0x140336518 5d pop rbp
0x140336519 c3 ret 
0x14033651a 660f1f440000 nop word ptr [rax + rax]
0x140336520 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140336525 55 push rbp
0x140336526 56 push rsi
0x140336527 57 push rdi
0x140336528 53 push rbx
0x140336529 4883ec28 sub rsp, 0x28
0x14033652d 488daa80000000 lea rbp, [rdx + 0x80]
0x140336534 488d8da8070000 lea rcx, [rbp + 0x7a8]
0x14033653b e84033cfff call 0x140029880
0x140336540 90 nop 
0x140336541 4883c428 add rsp, 0x28
0x140336545 5b pop rbx
0x140336546 5f pop rdi
0x140336547 5e pop rsi
0x140336548 5d pop rbp
0x140336549 c3 ret 
0x14033654a 660f1f440000 nop word ptr [rax + rax]
0x140336550 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140336555 55 push rbp
0x140336556 56 push rsi
0x140336557 57 push rdi
0x140336558 53 push rbx
0x140336559 4883ec28 sub rsp, 0x28
0x14033655d 488daa80000000 lea rbp, [rdx + 0x80]
0x140336564 488d8d38070000 lea rcx, [rbp + 0x738]
0x14033656b e81033cfff call 0x140029880
0x140336570 90 nop 
0x140336571 4883c428 add rsp, 0x28
0x140336575 5b pop rbx
0x140336576 5f pop rdi
0x140336577 5e pop rsi
0x140336578 5d pop rbp
0x140336579 c3 ret 
0x14033657a 660f1f440000 nop word ptr [rax + rax]
0x140336580 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140336585 55 push rbp
0x140336586 56 push rsi
0x140336587 57 push rdi
0x140336588 53 push rbx
0x140336589 4883ec28 sub rsp, 0x28
0x14033658d 488daa80000000 lea rbp, [rdx + 0x80]
0x140336594 488d8dc8060000 lea rcx, [rbp + 0x6c8]
0x14033659b e8e032cfff call 0x140029880
0x1403365a0 90 nop 
0x1403365a1 4883c428 add rsp, 0x28
0x1403365a5 5b pop rbx
0x1403365a6 5f pop rdi
0x1403365a7 5e pop rsi
0x1403365a8 5d pop rbp
0x1403365a9 c3 ret 
0x1403365aa 660f1f440000 nop word ptr [rax + rax]
0x1403365b0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1403365b5 55 push rbp
0x1403365b6 56 push rsi
0x1403365b7 57 push rdi
0x1403365b8 53 push rbx
0x1403365b9 4883ec28 sub rsp, 0x28
0x1403365bd 488daa80000000 lea rbp, [rdx + 0x80]
0x1403365c4 488d8d58060000 lea rcx, [rbp + 0x658]
0x1403365cb e8b032cfff call 0x140029880
0x1403365d0 90 nop 
0x1403365d1 4883c428 add rsp, 0x28
0x1403365d5 5b pop rbx
0x1403365d6 5f pop rdi
0x1403365d7 5e pop rsi
0x1403365d8 5d pop rbp
0x1403365d9 c3 ret 
0x1403365da 660f1f440000 nop word ptr [rax + rax]
0x1403365e0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1403365e5 55 push rbp
0x1403365e6 56 push rsi
0x1403365e7 57 push rdi
0x1403365e8 53 push rbx
0x1403365e9 4883ec28 sub rsp, 0x28
0x1403365ed 488daa80000000 lea rbp, [rdx + 0x80]
0x1403365f4 488d8de8050000 lea rcx, [rbp + 0x5e8]
0x1403365fb e88032cfff call 0x140029880
0x140336600 90 nop 
0x140336601 4883c428 add rsp, 0x28
0x140336605 5b pop rbx
0x140336606 5f pop rdi
0x140336607 5e pop rsi
0x140336608 5d pop rbp
0x140336609 c3 ret 
0x14033660a 660f1f440000 nop word ptr [rax + rax]
0x140336610 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140336615 55 push rbp
0x140336616 56 push rsi
0x140336617 57 push rdi
0x140336618 53 push rbx
0x140336619 4883ec28 sub rsp, 0x28
0x14033661d 488daa80000000 lea rbp, [rdx + 0x80]
0x140336624 488d8d78050000 lea rcx, [rbp + 0x578]
0x14033662b e85032cfff call 0x140029880
0x140336630 90 nop 
0x140336631 4883c428 add rsp, 0x28
0x140336635 5b pop rbx
0x140336636 5f pop rdi
0x140336637 5e pop rsi
0x140336638 5d pop rbp
0x140336639 c3 ret 
0x14033663a 660f1f440000 nop word ptr [rax + rax]
0x140336640 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140336645 55 push rbp
0x140336646 56 push rsi
0x140336647 57 push rdi
0x140336648 53 push rbx
0x140336649 4883ec28 sub rsp, 0x28
0x14033664d 488daa80000000 lea rbp, [rdx + 0x80]
0x140336654 488d8d08050000 lea rcx, [rbp + 0x508]
0x14033665b e82032cfff call 0x140029880
0x140336660 90 nop 
0x140336661 4883c428 add rsp, 0x28
0x140336665 5b pop rbx
0x140336666 5f pop rdi
0x140336667 5e pop rsi
0x140336668 5d pop rbp
0x140336669 c3 ret 
0x14033666a 660f1f440000 nop word ptr [rax + rax]
0x140336670 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140336675 55 push rbp
0x140336676 56 push rsi
0x140336677 57 push rdi
0x140336678 53 push rbx
0x140336679 4883ec28 sub rsp, 0x28
0x14033667d 488daa80000000 lea rbp, [rdx + 0x80]
0x140336684 488d8d98040000 lea rcx, [rbp + 0x498]
0x14033668b e8f031cfff call 0x140029880
0x140336690 90 nop 
0x140336691 4883c428 add rsp, 0x28
0x140336695 5b pop rbx
0x140336696 5f pop rdi
0x140336697 5e pop rsi
0x140336698 5d pop rbp
0x140336699 c3 ret 
0x14033669a 660f1f440000 nop word ptr [rax + rax]
0x1403366a0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1403366a5 55 push rbp
0x1403366a6 56 push rsi
0x1403366a7 57 push rdi
0x1403366a8 53 push rbx
0x1403366a9 4883ec28 sub rsp, 0x28
0x1403366ad 488daa80000000 lea rbp, [rdx + 0x80]
0x1403366b4 488d8d28040000 lea rcx, [rbp + 0x428]
0x1403366bb e8c031cfff call 0x140029880
0x1403366c0 90 nop 
0x1403366c1 4883c428 add rsp, 0x28
0x1403366c5 5b pop rbx
0x1403366c6 5f pop rdi
0x1403366c7 5e pop rsi
0x1403366c8 5d pop rbp
0x1403366c9 c3 ret 
0x1403366ca 660f1f440000 nop word ptr [rax + rax]
0x1403366d0 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1403366d5 55 push rbp
0x1403366d6 56 push rsi
0x1403366d7 57 push rdi
0x1403366d8 53 push rbx
0x1403366d9 4883ec28 sub rsp, 0x28
0x1403366dd 488daa80000000 lea rbp, [rdx + 0x80]
0x1403366e4 488d8db8030000 lea rcx, [rbp + 0x3b8]
0x1403366eb e89031cfff call 0x140029880
0x1403366f0 90 nop 
0x1403366f1 4883c428 add rsp, 0x28
0x1403366f5 5b pop rbx
0x1403366f6 5f pop rdi
0x1403366f7 5e pop rsi
0x1403366f8 5d pop rbp
0x1403366f9 c3 ret 
0x1403366fa 660f1f440000 nop word ptr [rax + rax]
0x140336700 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140336705 55 push rbp
0x140336706 56 push rsi
0x140336707 57 push rdi
0x140336708 53 push rbx
0x140336709 4883ec28 sub rsp, 0x28
0x14033670d 488daa80000000 lea rbp, [rdx + 0x80]
0x140336714 488d8d48030000 lea rcx, [rbp + 0x348]
0x14033671b e86031cfff call 0x140029880
0x140336720 90 nop 
0x140336721 4883c428 add rsp, 0x28
0x140336725 5b pop rbx
0x140336726 5f pop rdi
0x140336727 5e pop rsi
0x140336728 5d pop rbp
0x140336729 c3 ret 
0x14033672a 660f1f440000 nop word ptr [rax + rax]
0x140336730 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140336735 55 push rbp
0x140336736 56 push rsi
0x140336737 57 push rdi
0x140336738 53 push rbx
0x140336739 4883ec28 sub rsp, 0x28
0x14033673d 488daa80000000 lea rbp, [rdx + 0x80]
0x140336744 488d8d68090000 lea rcx, [rbp + 0x968]
0x14033674b e83031cfff call 0x140029880
0x140336750 90 nop 
0x140336751 4883c428 add rsp, 0x28
0x140336755 5b pop rbx
0x140336756 5f pop rdi
0x140336757 5e pop rsi
0x140336758 5d pop rbp
0x140336759 c3 ret 
0x14033675a 660f1f440000 nop word ptr [rax + rax]
0x140336760 4889542410 mov qword ptr [rsp + 0x10], rdx
0x140336765 55 push rbp
0x140336766 56 push rsi
0x140336767 57 push rdi
0x140336768 53 push rbx
0x140336769 4883ec28 sub rsp, 0x28
0x14033676d 488daa80000000 lea rbp, [rdx + 0x80]
0x140336774 488d4da0 lea rcx, [rbp - 0x60]
0x140336778 e80331cfff call 0x140029880
0x14033677d 90 nop 
0x14033677e 4883c428 add rsp, 0x28
0x140336782 5b pop rbx
0x140336783 5f pop rdi
0x140336784 5e pop rsi
0x140336785 5d pop rbp
0x140336786 c3 ret 
