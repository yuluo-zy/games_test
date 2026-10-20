140342c10 push rbp
140342c11 push r15
140342c13 push r14
140342c15 push r12
140342c17 push rsi
140342c18 push rdi
140342c19 push rbx
140342c1a sub rsp, 0x740
140342c21 lea rbp, [rsp + 0x80]
140342c29 movaps xmmword ptr [rbp + 0x6b0], xmm12
140342c31 movaps xmmword ptr [rbp + 0x6a0], xmm11
140342c39 movaps xmmword ptr [rbp + 0x690], xmm10
140342c41 movaps xmmword ptr [rbp + 0x680], xmm9
140342c49 movaps xmmword ptr [rbp + 0x670], xmm8
140342c51 movaps xmmword ptr [rbp + 0x660], xmm7
140342c58 movaps xmmword ptr [rbp + 0x650], xmm6
140342c5f mov qword ptr [rbp + 0x648], 0xfffffffffffffffe
140342c6a mov rsi, rdx
140342c6d lea rdi, [rdx + 0x38]
140342c71 movabs rdx, 0x943d269e95349652
140342c7b movabs r8, 0x30e26b7e89c2d791
140342c85 mov rcx, rdi
140342c88 call 0x1400dd6f0
140342c8d test rax, rax
140342c90 je 0x140342cc5
140342c92 cmp qword ptr [rsi + 0x218], rdx
140342c99 jbe 0x140342cc5
140342c9b mov rax, qword ptr [rsi + 0x210]
140342ca2 mov rax, qword ptr [rax + rdx*8]
140342ca6 test rax, rax
140342ca9 je 0x140342cc5
140342cab mov rcx, qword ptr [rsi + 0x1e0]
140342cb2 not rax
140342cb5 lea rax, [rax + rax*2]
140342cb9 shl rax, 5
140342cbd cmp qword ptr [rcx + rax + 0x20], 0
140342cc3 jne 0x140342ccd
140342cc5 mov rcx, rsi
140342cc8 call 0x140584b50
140342ccd movabs rdx, 0x75871b40552aeedd
140342cd7 movabs r8, 0xea54ffd3c51f7d72
140342ce1 mov rcx, rdi
140342ce4 call 0x1400dd6f0
140342ce9 test rax, rax
140342cec je 0x140342d21
140342cee cmp qword ptr [rsi + 0x218], rdx
140342cf5 jbe 0x140342d21
140342cf7 mov rax, qword ptr [rsi + 0x210]
140342cfe mov rax, qword ptr [rax + rdx*8]
140342d02 test rax, rax
140342d05 je 0x140342d21
140342d07 mov rcx, qword ptr [rsi + 0x1e0]
140342d0e not rax
140342d11 lea rax, [rax + rax*2]
140342d15 shl rax, 5
140342d19 cmp qword ptr [rcx + rax + 0x20], 0
140342d1f jne 0x140342d29
140342d21 mov rcx, rsi
140342d24 call 0x14057c330
140342d29 movabs rdx, 0xd1bb63bd09206748
140342d33 movabs r8, 0x75cbc9219ed6335c
140342d3d mov rcx, rdi
140342d40 call 0x1400dd6f0
140342d45 test rax, rax
140342d48 je 0x140342d7d
140342d4a cmp qword ptr [rsi + 0x218], rdx
140342d51 jbe 0x140342d7d
140342d53 mov rax, qword ptr [rsi + 0x210]
140342d5a mov rax, qword ptr [rax + rdx*8]
140342d5e test rax, rax
140342d61 je 0x140342d7d
140342d63 mov rcx, qword ptr [rsi + 0x1e0]
140342d6a not rax
140342d6d lea rax, [rax + rax*2]
140342d71 shl rax, 5
140342d75 cmp qword ptr [rcx + rax + 0x20], 0
140342d7b jne 0x140342d85
140342d7d mov rcx, rsi
140342d80 call 0x14057fae0
140342d85 movabs rdx, 0x8123f91b198a08ca
140342d8f movabs r8, 0xe49931532f714ae4
140342d99 mov rcx, rdi
140342d9c call 0x1400dd6f0
140342da1 test rax, rax
140342da4 je 0x140342dd9
140342da6 cmp qword ptr [rsi + 0x218], rdx
140342dad jbe 0x140342dd9
140342daf mov rax, qword ptr [rsi + 0x210]
140342db6 mov rax, qword ptr [rax + rdx*8]
140342dba test rax, rax
140342dbd je 0x140342dd9
140342dbf mov rcx, qword ptr [rsi + 0x1e0]
140342dc6 not rax
140342dc9 lea rax, [rax + rax*2]
140342dcd shl rax, 5
140342dd1 cmp qword ptr [rcx + rax + 0x20], 0
140342dd7 jne 0x140342de1
140342dd9 mov rcx, rsi
140342ddc call 0x140585180
140342de1 movabs rdx, 0x486478c47a7a8e5f
140342deb movabs r8, 0xbe94d9fa2dbc7e3a
140342df5 mov rcx, rdi
140342df8 call 0x1400dd6f0
140342dfd test rax, rax
140342e00 je 0x140342e35
140342e02 cmp qword ptr [rsi + 0x218], rdx
140342e09 jbe 0x140342e35
140342e0b mov rax, qword ptr [rsi + 0x210]
140342e12 mov rax, qword ptr [rax + rdx*8]
140342e16 test rax, rax
140342e19 je 0x140342e35
140342e1b mov rcx, qword ptr [rsi + 0x1e0]
140342e22 not rax
140342e25 lea rax, [rax + rax*2]
140342e29 shl rax, 5
140342e2d cmp qword ptr [rcx + rax + 0x20], 0
140342e33 jne 0x140342e3d
140342e35 mov rcx, rsi
140342e38 call 0x14057f4b0
140342e3d movabs rdx, 0x937f1da655769d96
140342e47 movabs r8, 0xf0834668987c535f
140342e51 mov rcx, rdi
140342e54 call 0x1400dd6f0
140342e59 test rax, rax
140342e5c je 0x140342e91
140342e5e cmp qword ptr [rsi + 0x218], rdx
140342e65 jbe 0x140342e91
140342e67 mov rax, qword ptr [rsi + 0x210]
140342e6e mov rax, qword ptr [rax + rdx*8]
140342e72 test rax, rax
140342e75 je 0x140342e91
140342e77 mov rcx, qword ptr [rsi + 0x1e0]
140342e7e not rax
140342e81 lea rax, [rax + rax*2]
140342e85 shl rax, 5
140342e89 cmp qword ptr [rcx + rax + 0x20], 0
140342e8f jne 0x140342e99
140342e91 mov rcx, rsi
140342e94 call 0x14056b2f0
140342e99 movabs rdx, 0xb8cc7044f2832921
140342ea3 movabs r8, 0x587ecc8eff8dfe5a
140342ead mov rcx, rdi
140342eb0 call 0x1400dd6f0
140342eb5 test rax, rax
140342eb8 je 0x140342eed
140342eba cmp qword ptr [rsi + 0x218], rdx
140342ec1 jbe 0x140342eed
140342ec3 mov rax, qword ptr [rsi + 0x210]
140342eca mov rax, qword ptr [rax + rdx*8]
140342ece test rax, rax
140342ed1 je 0x140342eed
140342ed3 mov rcx, qword ptr [rsi + 0x1e0]
140342eda not rax
140342edd lea rax, [rax + rax*2]
140342ee1 shl rax, 5
140342ee5 cmp qword ptr [rcx + rax + 0x20], 0
140342eeb jne 0x140342ef5
140342eed mov rcx, rsi
140342ef0 call 0x14056a690
140342ef5 movabs rdx, 0x717f567acfef4c67
140342eff movabs r8, 0x193ba65ffa83b0ae
140342f09 mov rcx, rdi
140342f0c call 0x1400dd6f0
140342f11 test rax, rax
140342f14 je 0x140342f49
140342f16 cmp qword ptr [rsi + 0x218], rdx
140342f1d jbe 0x140342f49
140342f1f mov rax, qword ptr [rsi + 0x210]
140342f26 mov rax, qword ptr [rax + rdx*8]
140342f2a test rax, rax
140342f2d je 0x140342f49
140342f2f mov rcx, qword ptr [rsi + 0x1e0]
140342f36 not rax
140342f39 lea rax, [rax + rax*2]
140342f3d shl rax, 5
140342f41 cmp qword ptr [rcx + rax + 0x20], 0
140342f47 jne 0x140342f51
140342f49 mov rcx, rsi
140342f4c call 0x140587070
140342f51 xorps xmm0, xmm0
140342f54 movaps xmmword ptr [rbp + 0x320], xmm0
140342f5b mov qword ptr [rbp + 0x330], 8
140342f66 mov qword ptr [rbp + 0x338], 0
140342f71 lea rdx, [rbp + 0x320]
140342f78 mov rcx, rsi
140342f7b call 0x140114240
140342f80 mov rsi, rax
140342f83 call 0x140d4d740
140342f88 mov rdi, rax
140342f8b test byte ptr [rax], 1
140342f8e je 0x140342fa1
140342f90 mov rcx, rdi
140342f93 add rcx, 8
140342f97 mov rax, qword ptr [rdi + 8]
140342f9b mov rdx, qword ptr [rdi + 0x10]
140342f9f jmp 0x140342fbc
140342fa1 call 0x140d64e00
140342fa6 mov rcx, rdi
140342fa9 add rcx, 8
140342fad mov qword ptr [rdi], 1
140342fb4 mov qword ptr [rdi + 8], rax
140342fb8 mov qword ptr [rdi + 0x10], rdx
140342fbc lea r8, [rax + 1]
140342fc0 mov qword ptr [rcx], r8
140342fc3 movups xmm6, xmmword ptr [rip + 0x267fe96]
140342fca movaps xmmword ptr [rbp + 0x5c0], xmm6
140342fd1 movups xmm7, xmmword ptr [rip + 0x267fe78]
140342fd8 movaps xmmword ptr [rbp + 0x5b0], xmm7
140342fdf mov qword ptr [rbp + 0x5d0], rax
140342fe6 mov qword ptr [rbp + 0x5d8], rdx
140342fed call 0x140d4d740
140342ff2 mov rdi, rax
140342ff5 test byte ptr [rax], 1
140342ff8 je 0x14034300b
140342ffa mov rcx, rdi
140342ffd add rcx, 8
140343001 mov rax, qword ptr [rdi + 8]
140343005 mov rdx, qword ptr [rdi + 0x10]
140343009 jmp 0x140343026
14034300b call 0x140d64e00
140343010 mov rcx, rdi
140343013 add rcx, 8
140343017 mov qword ptr [rdi], 1
14034301e mov qword ptr [rdi + 8], rax
140343022 mov qword ptr [rdi + 0x10], rdx
140343026 lea r8, [rax + 1]
14034302a mov qword ptr [rcx], r8
14034302d movaps xmmword ptr [rbp + 0x100], xmm6
140343034 movaps xmmword ptr [rbp + 0xf0], xmm7
14034303b mov qword ptr [rbp + 0x110], rax
140343042 mov qword ptr [rbp + 0x118], rdx
140343049 call 0x140d4d740
14034304e mov rdi, rax
140343051 test byte ptr [rax], 1
140343054 je 0x140343067
140343056 mov rcx, rdi
140343059 add rcx, 8
14034305d mov rax, qword ptr [rdi + 8]
140343061 mov rdx, qword ptr [rdi + 0x10]
140343065 jmp 0x140343082
140343067 call 0x140d64e00
14034306c mov rcx, rdi
14034306f add rcx, 8
140343073 mov qword ptr [rdi], 1
14034307a mov qword ptr [rdi + 8], rax
14034307e mov qword ptr [rdi + 0x10], rdx
140343082 lea r8, [rax + 1]
140343086 mov qword ptr [rcx], r8
140343089 movaps xmm0, xmmword ptr [rbp + 0x5b0]
140343090 movaps xmm1, xmmword ptr [rbp + 0x5c0]
140343097 movaps xmm2, xmmword ptr [rbp + 0x5d0]
14034309e movaps xmmword ptr [rbp + 0x340], xmm2
1403430a5 movaps xmmword ptr [rbp + 0x330], xmm1
1403430ac movaps xmmword ptr [rbp + 0x320], xmm0
1403430b3 mov qword ptr [rbp + 0x3b0], 0
1403430be mov qword ptr [rbp + 0x440], 0
1403430c9 movaps xmm0, xmmword ptr [rbp + 0xf0]
1403430d0 movaps xmm1, xmmword ptr [rbp + 0x100]
1403430d7 movaps xmm2, xmmword ptr [rbp + 0x110]
1403430de movaps xmmword ptr [rbp + 0x350], xmm0
1403430e5 movaps xmmword ptr [rbp + 0x360], xmm1
1403430ec movaps xmmword ptr [rbp + 0x370], xmm2
1403430f3 movaps xmmword ptr [rbp + 0x380], xmm7
1403430fa movaps xmmword ptr [rbp + 0x390], xmm6
140343101 mov qword ptr [rbp + 0x3a0], rax
140343108 mov qword ptr [rbp + 0x3a8], rdx
14034310f mov byte ptr [rbp + 0x4d0], 0
140343116 lea rdx, [rbp + 0x320]
14034311d mov rcx, rsi
140343120 call 0x14013db40
140343125 mov rsi, rax
140343128 lea rdi, [rbp + 0x320]
14034312f mov rcx, rdi
140343132 call 0x140341370
140343137 mov rcx, rsi
14034313a mov rdx, rdi
14034313d call 0x14011a980
140343142 mov rsi, rax
140343145 xor ecx, ecx
140343147 call 0x1400d9b20
14034314c test rax, rax
14034314f je 0x1403453c2
140343155 mov qword ptr [rbp + 0x320], 0
140343160 mov qword ptr [rbp + 0x328], 8
14034316b mov qword ptr [rbp + 0x330], 0
140343176 movups xmm0, xmmword ptr [rax]
140343179 inc qword ptr [rax]
14034317c movups xmm8, xmmword ptr [rip + 0x266ceac]
140343184 movups xmmword ptr [rbp + 0x348], xmm8
14034318c movups xmm9, xmmword ptr [rip + 0x266ce8c]
140343194 movups xmmword ptr [rbp + 0x338], xmm9
14034319c movups xmmword ptr [rbp + 0x358], xmm0
1403431a3 mov byte ptr [rbp + 0x368], 0
1403431aa lea rdx, [rbp + 0x320]
1403431b1 mov rcx, rsi
1403431b4 call 0x140134b70
1403431b9 mov rsi, rax
1403431bc call 0x140d4d740
1403431c1 mov rdi, rax
1403431c4 test byte ptr [rax], 1
1403431c7 je 0x1403431da
1403431c9 mov rcx, rdi
1403431cc add rcx, 8
1403431d0 mov rax, qword ptr [rdi + 8]
1403431d4 mov rdx, qword ptr [rdi + 0x10]
1403431d8 jmp 0x1403431f5
1403431da call 0x140d64e00
1403431df mov rcx, rdi
1403431e2 add rcx, 8
1403431e6 mov qword ptr [rdi], 1
1403431ed mov qword ptr [rdi + 8], rax
1403431f1 mov qword ptr [rdi + 0x10], rdx
1403431f5 lea r8, [rax + 1]
1403431f9 mov qword ptr [rcx], r8
1403431fc movaps xmmword ptr [rbp + 0x100], xmm6
140343203 movaps xmmword ptr [rbp + 0xf0], xmm7
14034320a mov qword ptr [rbp + 0x110], rax
140343211 mov qword ptr [rbp + 0x118], rdx
140343218 call 0x140d4d740
14034321d mov rdi, rax
140343220 test byte ptr [rax], 1
140343223 je 0x140343236
140343225 mov rcx, rdi
140343228 add rcx, 8
14034322c mov rax, qword ptr [rdi + 8]
140343230 mov rdx, qword ptr [rdi + 0x10]
140343234 jmp 0x140343251
140343236 call 0x140d64e00
14034323b mov rcx, rdi
14034323e add rcx, 8
140343242 mov qword ptr [rdi], 1
140343249 mov qword ptr [rdi + 8], rax
14034324d mov qword ptr [rdi + 0x10], rdx
140343251 lea r8, [rax + 1]
140343255 mov qword ptr [rcx], r8
140343258 movaps xmm0, xmmword ptr [rbp + 0xf0]
14034325f movaps xmm1, xmmword ptr [rbp + 0x100]
140343266 movaps xmm2, xmmword ptr [rbp + 0x110]
14034326d movups xmmword ptr [rbp + 0x358], xmm2
140343274 movups xmmword ptr [rbp + 0x348], xmm1
14034327b movups xmmword ptr [rbp + 0x338], xmm0
140343282 movups xmmword ptr [rbp + 0x368], xmm7
140343289 movups xmmword ptr [rbp + 0x378], xmm6
140343290 mov qword ptr [rbp + 0x388], rax
140343297 mov qword ptr [rbp + 0x390], rdx
14034329e mov qword ptr [rbp + 0x320], 0
1403432a9 mov qword ptr [rbp + 0x328], 0x10
1403432b4 mov qword ptr [rbp + 0x330], 0
1403432bf lea rdx, [rbp + 0x320]
1403432c6 mov rcx, rsi
1403432c9 call 0x14013cfd0
1403432ce mov rsi, rax
1403432d1 call 0x140d4d740
1403432d6 mov rdi, rax
1403432d9 test byte ptr [rax], 1
1403432dc je 0x1403432ef
1403432de mov rcx, rdi
1403432e1 add rcx, 8
1403432e5 mov rax, qword ptr [rdi + 8]
1403432e9 mov rdx, qword ptr [rdi + 0x10]
1403432ed jmp 0x14034330a
1403432ef call 0x140d64e00
1403432f4 mov rcx, rdi
1403432f7 add rcx, 8
1403432fb mov qword ptr [rdi], 1
140343302 mov qword ptr [rdi + 8], rax
140343306 mov qword ptr [rdi + 0x10], rdx
14034330a lea r8, [rax + 1]
14034330e mov qword ptr [rcx], r8
140343311 movaps xmmword ptr [rbp + 0x330], xmm6
140343318 movaps xmmword ptr [rbp + 0x320], xmm7
14034331f mov qword ptr [rbp + 0x340], rax
140343326 mov qword ptr [rbp + 0x348], rdx
14034332d lea rdx, [rbp + 0x320]
140343334 mov rcx, rsi
140343337 call 0x14012eea0
14034333c mov dword ptr [rbp + 0x320], 0
140343346 mov dword ptr [rbp + 0x330], 0
140343350 mov qword ptr [rbp + 0x340], 0
14034335b lea rdx, [rbp + 0x320]
140343362 mov rcx, rax
140343365 call 0x140126200
14034336a mov rsi, rax
14034336d call 0x140002370
140343372 mov rcx, rsi
140343375 mov edx, eax
140343377 call 0x14013b8c0
14034337c mov dword ptr [rbp + 0x320], 0
140343386 mov qword ptr [rbp + 0x328], 0
140343391 mov dword ptr [rbp + 0x32f], 0
14034339b lea rdx, [rbp + 0x320]
1403433a2 mov rcx, rax
1403433a5 call 0x1401373d0
1403433aa mov edx, 3
1403433af mov rcx, rax
1403433b2 call 0x140117080
1403433b7 mov rsi, rax
1403433ba call 0x140d4d740
1403433bf mov rdi, rax
1403433c2 test byte ptr [rax], 1
1403433c5 je 0x1403433d8
1403433c7 mov rcx, rdi
1403433ca add rcx, 8
1403433ce mov rax, qword ptr [rdi + 8]
1403433d2 mov rdx, qword ptr [rdi + 0x10]
1403433d6 jmp 0x1403433f3
1403433d8 call 0x140d64e00
1403433dd mov rcx, rdi
1403433e0 add rcx, 8
1403433e4 mov qword ptr [rdi], 1
1403433eb mov qword ptr [rdi + 8], rax
1403433ef mov qword ptr [rdi + 0x10], rdx
1403433f3 lea r8, [rax + 1]
1403433f7 mov qword ptr [rcx], r8
1403433fa movaps xmmword ptr [rbp + 0x2c0], xmm6
140343401 movaps xmmword ptr [rbp + 0x2b0], xmm7
140343408 mov qword ptr [rbp + 0x2d0], rax
14034340f mov qword ptr [rbp + 0x2d8], rdx
140343416 call 0x140d4d740
14034341b mov rdi, rax
14034341e test byte ptr [rax], 1
140343421 je 0x140343434
140343423 mov rcx, rdi
140343426 add rcx, 8
14034342a mov rax, qword ptr [rdi + 8]
14034342e mov rdx, qword ptr [rdi + 0x10]
140343432 jmp 0x14034344f
140343434 call 0x140d64e00
140343439 mov rcx, rdi
14034343c add rcx, 8
140343440 mov qword ptr [rdi], 1
140343447 mov qword ptr [rdi + 8], rax
14034344b mov qword ptr [rdi + 0x10], rdx
14034344f lea r8, [rax + 1]
140343453 mov qword ptr [rcx], r8
140343456 movaps xmmword ptr [rbp + 0x5c0], xmm6
14034345d movaps xmmword ptr [rbp + 0x5b0], xmm7
140343464 mov qword ptr [rbp + 0x5d0], rax
14034346b mov qword ptr [rbp + 0x5d8], rdx
140343472 call 0x140d4d740
140343477 mov rdi, rax
14034347a test byte ptr [rax], 1
14034347d je 0x140343490
14034347f mov rcx, rdi
140343482 add rcx, 8
140343486 mov rax, qword ptr [rdi + 8]
14034348a mov rdx, qword ptr [rdi + 0x10]
14034348e jmp 0x1403434ab
140343490 call 0x140d64e00
140343495 mov rcx, rdi
140343498 add rcx, 8
14034349c mov qword ptr [rdi], 1
1403434a3 mov qword ptr [rdi + 8], rax
1403434a7 mov qword ptr [rdi + 0x10], rdx
1403434ab lea r8, [rax + 1]
1403434af mov qword ptr [rcx], r8
1403434b2 movaps xmmword ptr [rbp + 0x100], xmm6
1403434b9 movaps xmmword ptr [rbp + 0xf0], xmm7
1403434c0 mov qword ptr [rbp + 0x110], rax
1403434c7 mov qword ptr [rbp + 0x118], rdx
1403434ce call 0x140d4d740
1403434d3 mov rdi, rax
1403434d6 test byte ptr [rax], 1
1403434d9 je 0x1403434ec
1403434db mov rcx, rdi
1403434de add rcx, 8
1403434e2 mov rax, qword ptr [rdi + 8]
1403434e6 mov rdx, qword ptr [rdi + 0x10]
1403434ea jmp 0x140343507
1403434ec call 0x140d64e00
1403434f1 mov rcx, rdi
1403434f4 add rcx, 8
1403434f8 mov qword ptr [rdi], 1
1403434ff mov qword ptr [rdi + 8], rax
140343503 mov qword ptr [rdi + 0x10], rdx
140343507 lea r8, [rax + 1]
14034350b mov qword ptr [rcx], r8
14034350e movaps xmm0, xmmword ptr [rbp + 0x2b0]
140343515 movaps xmm1, xmmword ptr [rbp + 0x2c0]
14034351c movaps xmm2, xmmword ptr [rbp + 0x2d0]
140343523 movaps xmmword ptr [rbp + 0x340], xmm2
14034352a movaps xmmword ptr [rbp + 0x330], xmm1
140343531 movaps xmmword ptr [rbp + 0x320], xmm0
140343538 movaps xmm0, xmmword ptr [rbp + 0x5b0]
14034353f movaps xmm1, xmmword ptr [rbp + 0x5c0]
140343546 movaps xmm2, xmmword ptr [rbp + 0x5d0]
14034354d movaps xmmword ptr [rbp + 0x350], xmm0
140343554 movaps xmmword ptr [rbp + 0x360], xmm1
14034355b movaps xmmword ptr [rbp + 0x370], xmm2
140343562 movaps xmm0, xmmword ptr [rbp + 0xf0]
140343569 movaps xmm1, xmmword ptr [rbp + 0x100]
140343570 movaps xmm2, xmmword ptr [rbp + 0x110]
140343577 movaps xmmword ptr [rbp + 0x380], xmm0
14034357e movaps xmmword ptr [rbp + 0x390], xmm1
140343585 movaps xmmword ptr [rbp + 0x3a0], xmm2
14034358c movaps xmmword ptr [rbp + 0x3c0], xmm6
140343593 movaps xmmword ptr [rbp + 0x3b0], xmm7
14034359a mov qword ptr [rbp + 0x3d0], rax
1403435a1 mov qword ptr [rbp + 0x3d8], rdx
1403435a8 lea rdx, [rbp + 0x320]
1403435af mov rcx, rsi
1403435b2 call 0x14011e720
1403435b7 mov rsi, rax
1403435ba call 0x140d4d740
1403435bf mov rdi, rax
1403435c2 test byte ptr [rax], 1
1403435c5 je 0x1403435d8
1403435c7 mov rcx, rdi
1403435ca add rcx, 8
1403435ce mov rax, qword ptr [rdi + 8]
1403435d2 mov rdx, qword ptr [rdi + 0x10]
1403435d6 jmp 0x1403435f3
1403435d8 call 0x140d64e00
1403435dd mov rcx, rdi
1403435e0 add rcx, 8
1403435e4 mov qword ptr [rdi], 1
1403435eb mov qword ptr [rdi + 8], rax
1403435ef mov qword ptr [rdi + 0x10], rdx
1403435f3 lea r8, [rax + 1]
1403435f7 mov qword ptr [rcx], r8
1403435fa mov byte ptr [rbp + 0x350], 0
140343601 movaps xmmword ptr [rbp + 0x320], xmm7
140343608 movaps xmmword ptr [rbp + 0x330], xmm6
14034360f mov qword ptr [rbp + 0x340], rax
140343616 mov qword ptr [rbp + 0x348], rdx
14034361d lea rdx, [rbp + 0x320]
140343624 mov rcx, rsi
140343627 call 0x140132920
14034362c mov qword ptr [rbp + 0x328], 0
140343637 mov qword ptr [rbp + 0x320], 6
140343642 lea rdx, [rbp + 0x320]
140343649 mov rcx, rax
14034364c call 0x140139690
140343651 mov rsi, rax
140343654 call 0x140d4d740
140343659 mov rdi, rax
14034365c test byte ptr [rax], 1
14034365f je 0x140343672
140343661 mov rcx, rdi
140343664 add rcx, 8
140343668 mov rax, qword ptr [rdi + 8]
14034366c mov rdx, qword ptr [rdi + 0x10]
140343670 jmp 0x14034368d
140343672 call 0x140d64e00
140343677 mov rcx, rdi
14034367a add rcx, 8
14034367e mov qword ptr [rdi], 1
140343685 mov qword ptr [rdi + 8], rax
140343689 mov qword ptr [rdi + 0x10], rdx
14034368d lea r8, [rax + 1]
140343691 mov qword ptr [rcx], r8
140343694 movaps xmmword ptr [rbp + 0x330], xmm6
14034369b movaps xmmword ptr [rbp + 0x320], xmm7
1403436a2 mov qword ptr [rbp + 0x340], rax
1403436a9 mov qword ptr [rbp + 0x348], rdx
1403436b0 lea rdx, [rbp + 0x320]
1403436b7 mov rcx, rsi
1403436ba call 0x140131770
1403436bf mov rsi, rax
1403436c2 call 0x140d4d740
1403436c7 mov rdi, rax
1403436ca test byte ptr [rax], 1
1403436cd je 0x1403436e0
1403436cf mov rcx, rdi
1403436d2 add rcx, 8
1403436d6 mov rax, qword ptr [rdi + 8]
1403436da mov rdx, qword ptr [rdi + 0x10]
1403436de jmp 0x1403436fb
1403436e0 call 0x140d64e00
1403436e5 mov rcx, rdi
1403436e8 add rcx, 8
1403436ec mov qword ptr [rdi], 1
1403436f3 mov qword ptr [rdi + 8], rax
1403436f7 mov qword ptr [rdi + 0x10], rdx
1403436fb lea r8, [rax + 1]
1403436ff mov qword ptr [rcx], r8
140343702 movaps xmmword ptr [rbp + 0x2c0], xmm6
140343709 movaps xmmword ptr [rbp + 0x2b0], xmm7
140343710 mov qword ptr [rbp + 0x2d0], rax
140343717 mov qword ptr [rbp + 0x2d8], rdx
14034371e call 0x140d4d740
140343723 mov rdi, rax
140343726 test byte ptr [rax], 1
140343729 je 0x14034373c
14034372b mov rcx, rdi
14034372e add rcx, 8
140343732 mov rax, qword ptr [rdi + 8]
140343736 mov rdx, qword ptr [rdi + 0x10]
14034373a jmp 0x140343757
14034373c call 0x140d64e00
140343741 mov rcx, rdi
140343744 add rcx, 8
140343748 mov qword ptr [rdi], 1
14034374f mov qword ptr [rdi + 8], rax
140343753 mov qword ptr [rdi + 0x10], rdx
140343757 lea r8, [rax + 1]
14034375b mov qword ptr [rcx], r8
14034375e movaps xmmword ptr [rbp + 0x5c0], xmm6
140343765 movaps xmmword ptr [rbp + 0x5b0], xmm7
14034376c mov qword ptr [rbp + 0x5d0], rax
140343773 mov qword ptr [rbp + 0x5d8], rdx
14034377a call 0x140d4d740
14034377f mov rdi, rax
140343782 test byte ptr [rax], 1
140343785 je 0x140343798
140343787 mov rcx, rdi
14034378a add rcx, 8
14034378e mov rax, qword ptr [rdi + 8]
140343792 mov rdx, qword ptr [rdi + 0x10]
140343796 jmp 0x1403437b3
140343798 call 0x140d64e00
14034379d mov rcx, rdi
1403437a0 add rcx, 8
1403437a4 mov qword ptr [rdi], 1
1403437ab mov qword ptr [rdi + 8], rax
1403437af mov qword ptr [rdi + 0x10], rdx
1403437b3 lea r8, [rax + 1]
1403437b7 mov qword ptr [rcx], r8
1403437ba movaps xmmword ptr [rbp + 0x100], xmm6
1403437c1 movaps xmmword ptr [rbp + 0xf0], xmm7
1403437c8 mov qword ptr [rbp + 0x110], rax
1403437cf mov qword ptr [rbp + 0x118], rdx
1403437d6 call 0x140d4d740
1403437db mov rdi, rax
1403437de test byte ptr [rax], 1
1403437e1 je 0x1403437f4
1403437e3 mov rcx, rdi
1403437e6 add rcx, 8
1403437ea mov rax, qword ptr [rdi + 8]
1403437ee mov rdx, qword ptr [rdi + 0x10]
1403437f2 jmp 0x14034380f
1403437f4 call 0x140d64e00
1403437f9 mov rcx, rdi
1403437fc add rcx, 8
140343800 mov qword ptr [rdi], 1
140343807 mov qword ptr [rdi + 8], rax
14034380b mov qword ptr [rdi + 0x10], rdx
14034380f lea r8, [rax + 1]
140343813 mov qword ptr [rcx], r8
140343816 movaps xmm0, xmmword ptr [rbp + 0x2b0]
14034381d movaps xmm1, xmmword ptr [rbp + 0x2c0]
140343824 movaps xmm2, xmmword ptr [rbp + 0x2d0]
14034382b movaps xmmword ptr [rbp + 0x340], xmm2
140343832 movaps xmmword ptr [rbp + 0x330], xmm1
140343839 movaps xmmword ptr [rbp + 0x320], xmm0
140343840 movaps xmm0, xmmword ptr [rbp + 0x5b0]
140343847 movaps xmm1, xmmword ptr [rbp + 0x5c0]
14034384e movaps xmm2, xmmword ptr [rbp + 0x5d0]
140343855 movaps xmmword ptr [rbp + 0x350], xmm0
14034385c movaps xmmword ptr [rbp + 0x360], xmm1
140343863 movaps xmmword ptr [rbp + 0x370], xmm2
14034386a movaps xmm0, xmmword ptr [rbp + 0xf0]
140343871 movaps xmm1, xmmword ptr [rbp + 0x100]
140343878 movaps xmm2, xmmword ptr [rbp + 0x110]
14034387f movaps xmmword ptr [rbp + 0x380], xmm0
140343886 movaps xmmword ptr [rbp + 0x390], xmm1
14034388d movaps xmmword ptr [rbp + 0x3a0], xmm2
140343894 movaps xmmword ptr [rbp + 0x3c0], xmm6
14034389b movaps xmmword ptr [rbp + 0x3b0], xmm7
1403438a2 mov qword ptr [rbp + 0x3d0], rax
1403438a9 mov qword ptr [rbp + 0x3d8], rdx
1403438b0 lea rdx, [rbp + 0x320]
1403438b7 mov rcx, rsi
1403438ba call 0x14012ffd0
1403438bf mov rsi, rax
1403438c2 call 0x140d4d740
1403438c7 mov rdi, rax
1403438ca test byte ptr [rax], 1
1403438cd je 0x1403438e0
1403438cf mov rcx, rdi
1403438d2 add rcx, 8
1403438d6 mov rax, qword ptr [rdi + 8]
1403438da mov rdx, qword ptr [rdi + 0x10]
1403438de jmp 0x1403438fb
1403438e0 call 0x140d64e00
1403438e5 mov rcx, rdi
1403438e8 add rcx, 8
1403438ec mov qword ptr [rdi], 1
1403438f3 mov qword ptr [rdi + 8], rax
1403438f7 mov qword ptr [rdi + 0x10], rdx
1403438fb lea r8, [rax + 1]
1403438ff mov qword ptr [rcx], r8
140343902 movaps xmmword ptr [rbp + 0x330], xmm6
140343909 movaps xmmword ptr [rbp + 0x320], xmm7
140343910 mov qword ptr [rbp + 0x340], rax
140343917 mov qword ptr [rbp + 0x348], rdx
14034391e lea rdx, [rbp + 0x320]
140343925 mov rcx, rsi
140343928 call 0x140134030
14034392d mov rsi, rax
140343930 call 0x140d4d740
140343935 mov rdi, rax
140343938 test byte ptr [rax], 1
14034393b je 0x14034394e
14034393d mov rcx, rdi
140343940 add rcx, 8
140343944 mov rax, qword ptr [rdi + 8]
140343948 mov rdx, qword ptr [rdi + 0x10]
14034394c jmp 0x140343969
14034394e call 0x140d64e00
140343953 mov rcx, rdi
140343956 add rcx, 8
14034395a mov qword ptr [rdi], 1
140343961 mov qword ptr [rdi + 8], rax
140343965 mov qword ptr [rdi + 0x10], rdx
140343969 lea r8, [rax + 1]
14034396d mov qword ptr [rcx], r8
140343970 movaps xmmword ptr [rbp + 0x330], xmm6
140343977 movaps xmmword ptr [rbp + 0x320], xmm7
14034397e mov qword ptr [rbp + 0x340], rax
140343985 mov qword ptr [rbp + 0x348], rdx
14034398c lea rdx, [rbp + 0x320]
140343993 mov rcx, rsi
140343996 call 0x14012ce50
14034399b mov rsi, rax
14034399e xor ecx, ecx
1403439a0 call 0x1400d9b20
1403439a5 test rax, rax
1403439a8 je 0x1403453c2
1403439ae movups xmm0, xmmword ptr [rax]
1403439b1 inc qword ptr [rax]
1403439b4 movups xmm10, xmmword ptr [rip + 0x265fef4]
1403439bc movaps xmmword ptr [rbp + 0x100], xmm10
1403439c4 movups xmm11, xmmword ptr [rip + 0x265fed4]
1403439cc movaps xmmword ptr [rbp + 0xf0], xmm11
1403439d4 movaps xmmword ptr [rbp + 0x110], xmm0
1403439db xor ecx, ecx
1403439dd call 0x1400d9b20
1403439e2 test rax, rax
1403439e5 je 0x140345445
1403439eb movups xmm0, xmmword ptr [rax]
1403439ee inc qword ptr [rax]
1403439f1 movaps xmm1, xmmword ptr [rbp + 0xf0]
1403439f8 movaps xmm2, xmmword ptr [rbp + 0x100]
1403439ff movaps xmm3, xmmword ptr [rbp + 0x110]
140343a06 movaps xmmword ptr [rbp + 0x340], xmm3
140343a0d movaps xmmword ptr [rbp + 0x330], xmm2
140343a14 movaps xmmword ptr [rbp + 0x320], xmm1
140343a1b movaps xmmword ptr [rbp + 0x350], xmm11
140343a23 movaps xmmword ptr [rbp + 0x360], xmm10
140343a2b movaps xmmword ptr [rbp + 0x370], xmm0
140343a32 lea rdx, [rbp + 0x320]
140343a39 mov rcx, rsi
140343a3c call 0x14013be00
140343a41 mov rsi, rax
140343a44 call 0x140d4d740
140343a49 mov rdi, rax
140343a4c test byte ptr [rax], 1
140343a4f je 0x140343a62
140343a51 mov rcx, rdi
140343a54 add rcx, 8
140343a58 mov rax, qword ptr [rdi + 8]
140343a5c mov rdx, qword ptr [rdi + 0x10]
140343a60 jmp 0x140343a7d
140343a62 call 0x140d64e00
140343a67 mov rcx, rdi
140343a6a add rcx, 8
140343a6e mov qword ptr [rdi], 1
140343a75 mov qword ptr [rdi + 8], rax
140343a79 mov qword ptr [rdi + 0x10], rdx
140343a7d lea r8, [rax + 1]
140343a81 mov qword ptr [rcx], r8
140343a84 movaps xmmword ptr [rbp + 0x5c0], xmm6
140343a8b movaps xmmword ptr [rbp + 0x5b0], xmm7
140343a92 mov qword ptr [rbp + 0x5d0], rax
140343a99 mov qword ptr [rbp + 0x5d8], rdx
140343aa0 call 0x140d4d740
140343aa5 mov rdi, rax
140343aa8 test byte ptr [rax], 1
140343aab je 0x140343abe
140343aad mov rcx, rdi
140343ab0 add rcx, 8
140343ab4 mov rax, qword ptr [rdi + 8]
140343ab8 mov rdx, qword ptr [rdi + 0x10]
140343abc jmp 0x140343ad9
140343abe call 0x140d64e00
140343ac3 mov rcx, rdi
140343ac6 add rcx, 8
140343aca mov qword ptr [rdi], 1
140343ad1 mov qword ptr [rdi + 8], rax
140343ad5 mov qword ptr [rdi + 0x10], rdx
140343ad9 lea r8, [rax + 1]
140343add mov qword ptr [rcx], r8
140343ae0 movaps xmmword ptr [rbp + 0x330], xmm6
140343ae7 movaps xmmword ptr [rbp + 0x320], xmm7
140343aee mov qword ptr [rbp + 0x340], rax
140343af5 mov qword ptr [rbp + 0x348], rdx
140343afc call 0x140d4d740
140343b01 mov rdi, rax
140343b04 test byte ptr [rax], 1
140343b07 je 0x140343b1a
140343b09 mov rcx, rdi
140343b0c add rcx, 8
140343b10 mov rax, qword ptr [rdi + 8]
140343b14 mov rdx, qword ptr [rdi + 0x10]
140343b18 jmp 0x140343b35
140343b1a call 0x140d64e00
140343b1f mov rcx, rdi
140343b22 add rcx, 8
140343b26 mov qword ptr [rdi], 1
140343b2d mov qword ptr [rdi + 8], rax
140343b31 mov qword ptr [rdi + 0x10], rdx
140343b35 lea r8, [rax + 1]
140343b39 mov qword ptr [rcx], r8
140343b3c movaps xmm0, xmmword ptr [rbp + 0x5b0]
140343b43 movaps xmm1, xmmword ptr [rbp + 0x5c0]
140343b4a movaps xmm2, xmmword ptr [rbp + 0x5d0]
140343b51 movaps xmmword ptr [rbp + 0x110], xmm2
140343b58 movaps xmmword ptr [rbp + 0x100], xmm1
140343b5f movaps xmmword ptr [rbp + 0xf0], xmm0
140343b66 mov qword ptr [rbp + 0x180], 0
140343b71 mov qword ptr [rbp + 0x210], 0
140343b7c movaps xmm0, xmmword ptr [rbp + 0x320]
140343b83 movaps xmm1, xmmword ptr [rbp + 0x330]
140343b8a movaps xmm2, xmmword ptr [rbp + 0x340]
140343b91 movaps xmmword ptr [rbp + 0x120], xmm0
140343b98 movaps xmmword ptr [rbp + 0x130], xmm1
140343b9f movaps xmmword ptr [rbp + 0x140], xmm2
140343ba6 movaps xmmword ptr [rbp + 0x150], xmm7
140343bad movaps xmmword ptr [rbp + 0x160], xmm6
140343bb4 mov qword ptr [rbp + 0x170], rax
140343bbb mov qword ptr [rbp + 0x178], rdx
140343bc2 mov byte ptr [rbp + 0x2a0], 0
140343bc9 call 0x140d4d740
140343bce mov rdi, rax
140343bd1 test byte ptr [rax], 1
140343bd4 je 0x140343be7
140343bd6 mov rcx, rdi
140343bd9 add rcx, 8
140343bdd mov rax, qword ptr [rdi + 8]
140343be1 mov rdx, qword ptr [rdi + 0x10]
140343be5 jmp 0x140343c02
140343be7 call 0x140d64e00
140343bec mov rcx, rdi
140343bef add rcx, 8
140343bf3 mov qword ptr [rdi], 1
140343bfa mov qword ptr [rdi + 8], rax
140343bfe mov qword ptr [rdi + 0x10], rdx
140343c02 lea r8, [rax + 1]
140343c06 mov qword ptr [rcx], r8
140343c09 movaps xmmword ptr [rbp + 0x2c0], xmm6
140343c10 movaps xmmword ptr [rbp + 0x2b0], xmm7
140343c17 mov qword ptr [rbp + 0x2d0], rax
140343c1e mov qword ptr [rbp + 0x2d8], rdx
140343c25 call 0x140d4d740
140343c2a mov rdi, rax
140343c2d test byte ptr [rax], 1
140343c30 je 0x140343c43
140343c32 mov rcx, rdi
140343c35 add rcx, 8
140343c39 mov rax, qword ptr [rdi + 8]
140343c3d mov rdx, qword ptr [rdi + 0x10]
140343c41 jmp 0x140343c5e
140343c43 call 0x140d64e00
140343c48 mov rcx, rdi
140343c4b add rcx, 8
140343c4f mov qword ptr [rdi], 1
140343c56 mov qword ptr [rdi + 8], rax
140343c5a mov qword ptr [rdi + 0x10], rdx
140343c5e lea r8, [rax + 1]
140343c62 mov qword ptr [rcx], r8
140343c65 movaps xmmword ptr [rbp + 0x5c0], xmm6
140343c6c movaps xmmword ptr [rbp + 0x5b0], xmm7
140343c73 mov qword ptr [rbp + 0x5d0], rax
140343c7a mov qword ptr [rbp + 0x5d8], rdx
140343c81 call 0x140d4d740
140343c86 mov r14, rax
140343c89 test byte ptr [rax], 1
140343c8c je 0x140343c9f
140343c8e mov rax, r14
140343c91 add rax, 8
140343c95 mov rdi, qword ptr [r14 + 8]
140343c99 mov rbx, qword ptr [r14 + 0x10]
140343c9d jmp 0x140343cc0
140343c9f call 0x140d64e00
140343ca4 mov rdi, rax
140343ca7 mov rbx, rdx
140343caa mov rax, r14
140343cad add rax, 8
140343cb1 mov qword ptr [r14], 1
140343cb8 mov qword ptr [r14 + 8], rdi
140343cbc mov qword ptr [r14 + 0x10], rdx
140343cc0 lea rcx, [rdi + 1]
140343cc4 mov qword ptr [rax], rcx
140343cc7 xorps xmm0, xmm0
140343cca movups xmmword ptr [rbp + 0x580], xmm0
140343cd1 lea rcx, [rbp + 0x338]
140343cd8 lea rdx, [rbp + 0xf0]
140343cdf mov r8d, 0x1b8
140343ce5 call 0x142923c00
140343cea movaps xmm0, xmmword ptr [rbp + 0x2b0]
140343cf1 movaps xmm1, xmmword ptr [rbp + 0x2c0]
140343cf8 movaps xmm2, xmmword ptr [rbp + 0x2d0]
140343cff movups xmmword ptr [rbp + 0x4f0], xmm0
140343d06 movups xmmword ptr [rbp + 0x500], xmm1
140343d0d movups xmmword ptr [rbp + 0x510], xmm2
140343d14 movaps xmm0, xmmword ptr [rbp + 0x5b0]
140343d1b movaps xmm1, xmmword ptr [rbp + 0x5c0]
140343d22 movaps xmm2, xmmword ptr [rbp + 0x5d0]
140343d29 movups xmmword ptr [rbp + 0x520], xmm0
140343d30 movups xmmword ptr [rbp + 0x530], xmm1
140343d37 movups xmmword ptr [rbp + 0x540], xmm2
140343d3e movups xmmword ptr [rbp + 0x550], xmm7
140343d45 movups xmmword ptr [rbp + 0x560], xmm6
140343d4c mov qword ptr [rbp + 0x570], rdi
140343d53 mov qword ptr [rbp + 0x578], rbx
140343d5a mov qword ptr [rbp + 0x590], 8
140343d65 mov qword ptr [rbp + 0x598], 0
140343d70 mov qword ptr [rbp + 0x320], 0
140343d7b mov qword ptr [rbp + 0x328], 8
140343d86 mov qword ptr [rbp + 0x330], 0
140343d91 mov byte ptr [rbp + 0x5a2], 0
140343d98 mov word ptr [rbp + 0x5a0], 0
140343da1 lea rdx, [rbp + 0x320]
140343da8 mov rcx, rsi
140343dab call 0x140132350
140343db0 mov rsi, rax
140343db3 call 0x140d4d740
140343db8 mov rdi, rax
140343dbb test byte ptr [rax], 1
140343dbe je 0x140343dd1
140343dc0 mov rcx, rdi
140343dc3 add rcx, 8
140343dc7 mov rax, qword ptr [rdi + 8]
140343dcb mov rdx, qword ptr [rdi + 0x10]
140343dcf jmp 0x140343dec
140343dd1 call 0x140d64e00
140343dd6 mov rcx, rdi
140343dd9 add rcx, 8
140343ddd mov qword ptr [rdi], 1
140343de4 mov qword ptr [rdi + 8], rax
140343de8 mov qword ptr [rdi + 0x10], rdx
140343dec lea r8, [rax + 1]
140343df0 mov qword ptr [rcx], r8
140343df3 movaps xmmword ptr [rbp + 0x330], xmm6
140343dfa movaps xmmword ptr [rbp + 0x320], xmm7
140343e01 mov qword ptr [rbp + 0x340], rax
140343e08 mov qword ptr [rbp + 0x348], rdx
140343e0f lea rdx, [rbp + 0x320]
140343e16 mov rcx, rsi
140343e19 call 0x140124a10
140343e1e mov rsi, rax
140343e21 lea rdi, [rbp + 0x320]
140343e28 mov rcx, rdi
140343e2b call 0x14010ba60
140343e30 mov rcx, rsi
140343e33 mov rdx, rdi
140343e36 call 0x14013e110
140343e3b mov rsi, rax
140343e3e mov rcx, rdi
140343e41 call 0x14010b940
140343e46 mov rcx, rsi
140343e49 mov rdx, rdi
140343e4c call 0x140139080
140343e51 mov rsi, rax
140343e54 mov qword ptr [rbp + 0x10], 0
140343e5c mov qword ptr [rbp + 0x18], 8
140343e64 mov qword ptr [rbp + 0x20], 0
140343e6c call 0x140d4d740
140343e71 mov rdi, rax
140343e74 test byte ptr [rax], 1
140343e77 je 0x140343e8a
140343e79 mov rcx, rdi
140343e7c add rcx, 8
140343e80 mov rax, qword ptr [rdi + 8]
140343e84 mov rdx, qword ptr [rdi + 0x10]
140343e88 jmp 0x140343ea5
140343e8a call 0x140d64e00
140343e8f mov rcx, rdi
140343e92 add rcx, 8
140343e96 mov qword ptr [rdi], 1
140343e9d mov qword ptr [rdi + 8], rax
140343ea1 mov qword ptr [rdi + 0x10], rdx
140343ea5 lea r8, [rax + 1]
140343ea9 mov qword ptr [rcx], r8
140343eac movaps xmmword ptr [rbp + 0x90], xmm6
140343eb3 movaps xmmword ptr [rbp + 0x80], xmm7
140343eba mov qword ptr [rbp + 0xa0], rax
140343ec1 mov qword ptr [rbp + 0xa8], rdx
140343ec8 lea rcx, [rbp + 0x5b0]
140343ecf call 0x140244140
140343ed4 lea rcx, [rbp + 0xf0]
140343edb call 0x140244140
140343ee0 call 0x140d4d740
140343ee5 mov rdi, rax
140343ee8 test byte ptr [rax], 1
140343eeb je 0x140343efe
140343eed mov rcx, rdi
140343ef0 add rcx, 8
140343ef4 mov rax, qword ptr [rdi + 8]
140343ef8 mov rdx, qword ptr [rdi + 0x10]
140343efc jmp 0x140343f19
140343efe call 0x140d64e00
140343f03 mov rcx, rdi
140343f06 add rcx, 8
140343f0a mov qword ptr [rdi], 1
140343f11 mov qword ptr [rdi + 8], rax
140343f15 mov qword ptr [rdi + 0x10], rdx
140343f19 lea r8, [rax + 1]
140343f1d mov qword ptr [rcx], r8
140343f20 movaps xmmword ptr [rbp + 0x2c0], xmm6
140343f27 movaps xmmword ptr [rbp + 0x2b0], xmm7
140343f2e mov qword ptr [rbp + 0x2d0], rax
140343f35 mov qword ptr [rbp + 0x2d8], rdx
140343f3c call 0x140d4d740
140343f41 mov r14, rax
140343f44 test byte ptr [rax], 1
140343f47 je 0x140343f5a
140343f49 mov rax, r14
140343f4c add rax, 8
140343f50 mov rdi, qword ptr [r14 + 8]
140343f54 mov rbx, qword ptr [r14 + 0x10]
140343f58 jmp 0x140343f7b
140343f5a call 0x140d64e00
140343f5f mov rdi, rax
140343f62 mov rbx, rdx
140343f65 mov rax, r14
140343f68 add rax, 8
140343f6c mov qword ptr [r14], 1
140343f73 mov qword ptr [r14 + 8], rdi
140343f77 mov qword ptr [r14 + 0x10], rdx
140343f7b lea rcx, [rdi + 1]
140343f7f mov qword ptr [rax], rcx
140343f82 mov rax, qword ptr [rbp + 0x20]
140343f86 mov qword ptr [rbp + 0x330], rax
140343f8d movups xmm0, xmmword ptr [rbp + 0x10]
140343f91 movaps xmmword ptr [rbp + 0x320], xmm0
140343f98 movaps xmm0, xmmword ptr [rbp + 0x80]
140343f9f movaps xmm1, xmmword ptr [rbp + 0x90]
140343fa6 movaps xmm2, xmmword ptr [rbp + 0xa0]
140343fad movups xmmword ptr [rbp + 0x458], xmm0
140343fb4 movups xmmword ptr [rbp + 0x468], xmm1
140343fbb movups xmmword ptr [rbp + 0x478], xmm2
140343fc2 lea rcx, [rbp + 0x338]
140343fc9 lea rdx, [rbp + 0x5b0]
140343fd0 mov r8d, 0x90
140343fd6 call 0x142923c00
140343fdb lea rcx, [rbp + 0x3c8]
140343fe2 lea rdx, [rbp + 0xf0]
140343fe9 mov r8d, 0x90
140343fef call 0x142923c00
140343ff4 movaps xmm0, xmmword ptr [rbp + 0x2b0]
140343ffb movaps xmm1, xmmword ptr [rbp + 0x2c0]
140344002 movaps xmm2, xmmword ptr [rbp + 0x2d0]
140344009 movups xmmword ptr [rbp + 0x488], xmm0
140344010 movups xmmword ptr [rbp + 0x498], xmm1
140344017 movups xmmword ptr [rbp + 0x4a8], xmm2
14034401e movups xmmword ptr [rbp + 0x4b8], xmm7
140344025 movups xmmword ptr [rbp + 0x4c8], xmm6
14034402c mov qword ptr [rbp + 0x4d8], rdi
140344033 mov qword ptr [rbp + 0x4e0], rbx
14034403a lea rdx, [rbp + 0x320]
140344041 mov rcx, rsi
140344044 call 0x14011c5c0
140344049 mov qword ptr [rbp + 0x320], 0
140344054 mov qword ptr [rbp + 0x328], 8
14034405f mov qword ptr [rbp + 0x330], 0
14034406a lea rdx, [rbp + 0x320]
140344071 mov rcx, rax
140344074 call 0x140116aa0
140344079 mov qword ptr [rbp + 0x320], 0
140344084 mov qword ptr [rbp + 0x328], 8
14034408f mov qword ptr [rbp + 0x330], 0
14034409a lea rdx, [rbp + 0x320]
1403440a1 mov rcx, rax
1403440a4 call 0x140125610
1403440a9 mov rsi, rax
1403440ac call 0x140d4d740
1403440b1 mov rdi, rax
1403440b4 test byte ptr [rax], 1
1403440b7 je 0x1403440ca
1403440b9 mov rcx, rdi
1403440bc add rcx, 8
1403440c0 mov rax, qword ptr [rdi + 8]
1403440c4 mov rdx, qword ptr [rdi + 0x10]
1403440c8 jmp 0x1403440e5
1403440ca call 0x140d64e00
1403440cf mov rcx, rdi
1403440d2 add rcx, 8
1403440d6 mov qword ptr [rdi], 1
1403440dd mov qword ptr [rdi + 8], rax
1403440e1 mov qword ptr [rdi + 0x10], rdx
1403440e5 lea r8, [rax + 1]
1403440e9 mov qword ptr [rcx], r8
1403440ec movaps xmmword ptr [rbp + 0x100], xmm6
1403440f3 movaps xmmword ptr [rbp + 0xf0], xmm7
1403440fa mov qword ptr [rbp + 0x110], rax
140344101 mov qword ptr [rbp + 0x118], rdx
140344108 call 0x140d4d740
14034410d mov rdi, rax
140344110 test byte ptr [rax], 1
140344113 je 0x140344126
140344115 mov rcx, rdi
140344118 add rcx, 8
14034411c mov rax, qword ptr [rdi + 8]
140344120 mov rdx, qword ptr [rdi + 0x10]
140344124 jmp 0x140344141
140344126 call 0x140d64e00
14034412b mov rcx, rdi
14034412e add rcx, 8
140344132 mov qword ptr [rdi], 1
140344139 mov qword ptr [rdi + 8], rax
14034413d mov qword ptr [rdi + 0x10], rdx
140344141 lea r8, [rax + 1]
140344145 mov qword ptr [rcx], r8
140344148 movaps xmm0, xmmword ptr [rbp + 0xf0]
14034414f movaps xmm1, xmmword ptr [rbp + 0x100]
140344156 movaps xmm2, xmmword ptr [rbp + 0x110]
14034415d movaps xmmword ptr [rbp + 0x340], xmm2
140344164 movaps xmmword ptr [rbp + 0x330], xmm1
14034416b movaps xmmword ptr [rbp + 0x320], xmm0
140344172 movaps xmmword ptr [rbp + 0x350], xmm7
140344179 movaps xmmword ptr [rbp + 0x360], xmm6
140344180 mov qword ptr [rbp + 0x370], rax
140344187 mov qword ptr [rbp + 0x378], rdx
14034418e lea rdx, [rbp + 0x320]
140344195 mov rcx, rsi
140344198 call 0x140125bf0
14034419d mov rsi, rax
1403441a0 call 0x140d4d740
1403441a5 mov rdi, rax
1403441a8 test byte ptr [rax], 1
1403441ab je 0x1403441be
1403441ad mov rcx, rdi
1403441b0 add rcx, 8
1403441b4 mov rax, qword ptr [rdi + 8]
1403441b8 mov rdx, qword ptr [rdi + 0x10]
1403441bc jmp 0x1403441d9
1403441be call 0x140d64e00
1403441c3 mov rcx, rdi
1403441c6 add rcx, 8
1403441ca mov qword ptr [rdi], 1
1403441d1 mov qword ptr [rdi + 8], rax
1403441d5 mov qword ptr [rdi + 0x10], rdx
1403441d9 lea r8, [rax + 1]
1403441dd mov qword ptr [rcx], r8
1403441e0 movaps xmmword ptr [rbp + 0x330], xmm6
1403441e7 movaps xmmword ptr [rbp + 0x320], xmm7
1403441ee mov qword ptr [rbp + 0x340], rax
1403441f5 mov qword ptr [rbp + 0x348], rdx
1403441fc lea rdx, [rbp + 0x320]
140344203 mov rcx, rsi
140344206 call 0x14013ace0
14034420b mov rdi, rax
14034420e lea rcx, [rbp + 0x5b0]
140344215 call 0x140244140
14034421a lea rcx, [rbp + 0xf0]
140344221 call 0x140244140
140344226 xor ecx, ecx
140344228 call 0x1400d9b20
14034422d test rax, rax
140344230 je 0x140345453
140344236 lea rcx, [rbp + 0x348]
14034423d lea rbx, [rbp + 0x3d8]
140344244 movups xmm12, xmmword ptr [rax]
140344248 inc qword ptr [rax]
14034424b lea rsi, [rbp + 0x5b0]
140344252 mov r8d, 0x90
140344258 mov rdx, rsi
14034425b call 0x142923c00
140344260 lea rdx, [rbp + 0xf0]
140344267 mov r8d, 0x90
14034426d mov rcx, rbx
140344270 call 0x142923c00
140344275 mov byte ptr [rbp + 0x498], 0
14034427c mov qword ptr [rbp + 0x320], 0
140344287 mov qword ptr [rbp + 0x330], 3
140344292 movups xmmword ptr [rbp + 0x468], xmm9
14034429a movups xmmword ptr [rbp + 0x478], xmm8
1403442a2 movups xmmword ptr [rbp + 0x488], xmm12
1403442aa lea rdx, [rbp + 0x320]
1403442b1 mov rcx, rdi
1403442b4 call 0x14011af50
1403442b9 mov rdi, rax
1403442bc call 0x140d4d740
1403442c1 mov rbx, rax
1403442c4 test byte ptr [rax], 1
1403442c7 je 0x1403442da
1403442c9 mov rcx, rbx
1403442cc add rcx, 8
1403442d0 mov rax, qword ptr [rbx + 8]
1403442d4 mov rdx, qword ptr [rbx + 0x10]
1403442d8 jmp 0x1403442f5
1403442da call 0x140d64e00
1403442df mov rcx, rbx
1403442e2 add rcx, 8
1403442e6 mov qword ptr [rbx], 1
1403442ed mov qword ptr [rbx + 8], rax
1403442f1 mov qword ptr [rbx + 0x10], rdx
1403442f5 lea r8, [rax + 1]
1403442f9 mov qword ptr [rcx], r8
1403442fc movaps xmmword ptr [rbp + 0x330], xmm6
140344303 movaps xmmword ptr [rbp + 0x320], xmm7
14034430a mov qword ptr [rbp + 0x340], rax
140344311 mov qword ptr [rbp + 0x348], rdx
140344318 lea rdx, [rbp + 0x320]
14034431f mov rcx, rdi
140344322 call 0x14013b2d0
140344327 mov rdi, rax
14034432a call 0x140d4d740
14034432f mov rbx, rax
140344332 test byte ptr [rax], 1
140344335 je 0x140344348
140344337 mov rcx, rbx
14034433a add rcx, 8
14034433e mov rax, qword ptr [rbx + 8]
140344342 mov rdx, qword ptr [rbx + 0x10]
140344346 jmp 0x140344363
140344348 call 0x140d64e00
14034434d mov rcx, rbx
140344350 add rcx, 8
140344354 mov qword ptr [rbx], 1
14034435b mov qword ptr [rbx + 8], rax
14034435f mov qword ptr [rbx + 0x10], rdx
140344363 lea r8, [rax + 1]
140344367 mov qword ptr [rcx], r8
14034436a movaps xmmword ptr [rbp + 0x330], xmm6
140344371 movaps xmmword ptr [rbp + 0x320], xmm7
140344378 mov qword ptr [rbp + 0x340], rax
14034437f mov qword ptr [rbp + 0x348], rdx
140344386 mov byte ptr [rbp + 0x350], 0
14034438d lea rdx, [rbp + 0x320]
140344394 mov rcx, rdi
140344397 call 0x140120ef0
14034439c mov rdi, rax
14034439f xor ecx, ecx
1403443a1 call 0x1400d9b20
1403443a6 test rax, rax
1403443a9 je 0x1403453c2
1403443af movups xmm0, xmmword ptr [rax]
1403443b2 inc qword ptr [rax]
1403443b5 movaps xmmword ptr [rbp + 0x2c0], xmm10
1403443bd movaps xmmword ptr [rbp + 0x2b0], xmm11
1403443c5 movaps xmmword ptr [rbp + 0x2d0], xmm0
1403443cc xor ecx, ecx
1403443ce call 0x1400d9b20
1403443d3 test rax, rax
1403443d6 je 0x140345461
1403443dc movups xmm0, xmmword ptr [rax]
1403443df inc qword ptr [rax]
1403443e2 movaps xmmword ptr [rbp + 0x5c0], xmm10
1403443ea movaps xmmword ptr [rbp + 0x5b0], xmm11
1403443f2 movaps xmmword ptr [rbp + 0x5d0], xmm0
1403443f9 xor ecx, ecx
1403443fb call 0x1400d9b20
140344400 test rax, rax
140344403 je 0x14034546f
140344409 movups xmm0, xmmword ptr [rax]
14034440c inc qword ptr [rax]
14034440f movaps xmmword ptr [rbp + 0x100], xmm10
140344417 movaps xmmword ptr [rbp + 0xf0], xmm11
14034441f movaps xmmword ptr [rbp + 0x110], xmm0
140344426 xor ecx, ecx
140344428 call 0x1400d9b20
14034442d test rax, rax
140344430 je 0x14034547d
140344436 movups xmm0, xmmword ptr [rax]
140344439 inc qword ptr [rax]
14034443c movaps xmm1, xmmword ptr [rbp + 0x2b0]
140344443 movaps xmm2, xmmword ptr [rbp + 0x2c0]
14034444a movaps xmm3, xmmword ptr [rbp + 0x2d0]
140344451 movaps xmmword ptr [rbp + 0x340], xmm3
140344458 movaps xmmword ptr [rbp + 0x330], xmm2
14034445f movaps xmmword ptr [rbp + 0x320], xmm1
140344466 movaps xmm1, xmmword ptr [rbp + 0x5b0]
14034446d movaps xmm2, xmmword ptr [rbp + 0x5c0]
140344474 movaps xmm3, xmmword ptr [rbp + 0x5d0]
14034447b movaps xmmword ptr [rbp + 0x350], xmm1
140344482 movaps xmmword ptr [rbp + 0x360], xmm2
140344489 movaps xmmword ptr [rbp + 0x370], xmm3
140344490 movaps xmm1, xmmword ptr [rbp + 0xf0]
140344497 movaps xmm2, xmmword ptr [rbp + 0x100]
14034449e movaps xmm3, xmmword ptr [rbp + 0x110]
1403444a5 movaps xmmword ptr [rbp + 0x380], xmm1
1403444ac movaps xmmword ptr [rbp + 0x390], xmm2
1403444b3 movaps xmmword ptr [rbp + 0x3a0], xmm3
1403444ba movaps xmmword ptr [rbp + 0x3c0], xmm10
1403444c2 movaps xmmword ptr [rbp + 0x3b0], xmm11
1403444ca movaps xmmword ptr [rbp + 0x3d0], xmm0
1403444d1 lea rdx, [rbp + 0x320]
1403444d8 mov rcx, rdi
1403444db call 0x14012a110
1403444e0 mov rdi, rax
1403444e3 xor ecx, ecx
1403444e5 call 0x1400d9b20
1403444ea test rax, rax
1403444ed je 0x1403453c2
1403444f3 movups xmm0, xmmword ptr [rax]
1403444f6 inc qword ptr [rax]
1403444f9 movaps xmmword ptr [rbp + 0x340], xmm0
140344500 movaps xmmword ptr [rbp + 0x320], xmm9
140344508 movaps xmmword ptr [rbp + 0x330], xmm8
140344510 lea rdx, [rbp + 0x320]
140344517 mov rcx, rdi
14034451a call 0x14013f300
14034451f mov rdi, rax
140344522 call 0x140d4d740
140344527 mov rbx, rax
14034452a test byte ptr [rax], 1
14034452d je 0x140344540
14034452f mov rcx, rbx
140344532 add rcx, 8
140344536 mov rax, qword ptr [rbx + 8]
14034453a mov rdx, qword ptr [rbx + 0x10]
14034453e jmp 0x14034455b
140344540 call 0x140d64e00
140344545 mov rcx, rbx
140344548 add rcx, 8
14034454c mov qword ptr [rbx], 1
140344553 mov qword ptr [rbx + 8], rax
140344557 mov qword ptr [rbx + 0x10], rdx
14034455b lea r8, [rax + 1]
14034455f mov qword ptr [rcx], r8
140344562 movaps xmmword ptr [rbp + 0x20], xmm6
140344566 movaps xmmword ptr [rbp + 0x10], xmm7
14034456a mov qword ptr [rbp + 0x30], rax
14034456e mov qword ptr [rbp + 0x38], rdx
140344572 call 0x140d4d740
140344577 mov rbx, rax
14034457a test byte ptr [rax], 1
14034457d je 0x140344590
14034457f mov rcx, rbx
140344582 add rcx, 8
140344586 mov rax, qword ptr [rbx + 8]
14034458a mov rdx, qword ptr [rbx + 0x10]
14034458e jmp 0x1403445ab
140344590 call 0x140d64e00
140344595 mov rcx, rbx
140344598 add rcx, 8
14034459c mov qword ptr [rbx], 1
1403445a3 mov qword ptr [rbx + 8], rax
1403445a7 mov qword ptr [rbx + 0x10], rdx
1403445ab lea r8, [rax + 1]
1403445af mov qword ptr [rcx], r8
1403445b2 movaps xmmword ptr [rbp + 0x90], xmm6
1403445b9 movaps xmmword ptr [rbp + 0x80], xmm7
1403445c0 mov qword ptr [rbp + 0xa0], rax
1403445c7 mov qword ptr [rbp + 0xa8], rdx
1403445ce call 0x140d4d740
1403445d3 mov rbx, rax
1403445d6 test byte ptr [rax], 1
1403445d9 je 0x1403445ec
1403445db mov rcx, rbx
1403445de add rcx, 8
1403445e2 mov rax, qword ptr [rbx + 8]
1403445e6 mov rdx, qword ptr [rbx + 0x10]
1403445ea jmp 0x140344607
1403445ec call 0x140d64e00
1403445f1 mov rcx, rbx
1403445f4 add rcx, 8
1403445f8 mov qword ptr [rbx], 1
1403445ff mov qword ptr [rbx + 8], rax
140344603 mov qword ptr [rbx + 0x10], rdx
140344607 lea r8, [rax + 1]
14034460b mov qword ptr [rcx], r8
14034460e movaps xmmword ptr [rbp + 0x2c0], xmm6
140344615 movaps xmmword ptr [rbp + 0x2b0], xmm7
14034461c mov qword ptr [rbp + 0x2d0], rax
140344623 mov qword ptr [rbp + 0x2d8], rdx
14034462a call 0x140d4d740
14034462f mov rbx, rax
140344632 test byte ptr [rax], 1
140344635 je 0x140344648
140344637 mov rcx, rbx
14034463a add rcx, 8
14034463e mov rax, qword ptr [rbx + 8]
140344642 mov rdx, qword ptr [rbx + 0x10]
140344646 jmp 0x140344663
140344648 call 0x140d64e00
14034464d mov rcx, rbx
140344650 add rcx, 8
140344654 mov qword ptr [rbx], 1
14034465b mov qword ptr [rbx + 8], rax
14034465f mov qword ptr [rbx + 0x10], rdx
140344663 lea r8, [rax + 1]
140344667 mov qword ptr [rcx], r8
14034466a movaps xmmword ptr [rbp + 0x5c0], xmm6
140344671 movaps xmmword ptr [rbp + 0x5b0], xmm7
140344678 mov qword ptr [rbp + 0x5d0], rax
14034467f mov qword ptr [rbp + 0x5d8], rdx
140344686 xorps xmm0, xmm0
140344689 movups xmmword ptr [rbp + 0xf8], xmm0
140344690 movups xmmword ptr [rbp + 0x108], xmm0
140344697 mov qword ptr [rbp + 0x118], 0
1403446a2 movups xmmword ptr [rbp - 0x30], xmm0
1403446a6 movups xmmword ptr [rbp - 0x44], xmm0
1403446aa movups xmmword ptr [rbp - 0x58], xmm0
1403446ae movups xmmword ptr [rbp - 0x1c], xmm0
1403446b2 call 0x140d4d740
1403446b7 mov rbx, rax
1403446ba test byte ptr [rax], 1
1403446bd je 0x1403446d0
1403446bf mov rcx, rbx
1403446c2 add rcx, 8
1403446c6 mov rax, qword ptr [rbx + 8]
1403446ca mov rdx, qword ptr [rbx + 0x10]
1403446ce jmp 0x1403446eb
1403446d0 call 0x140d64e00
1403446d5 mov rcx, rbx
1403446d8 add rcx, 8
1403446dc mov qword ptr [rbx], 1
1403446e3 mov qword ptr [rbx + 8], rax
1403446e7 mov qword ptr [rbx + 0x10], rdx
1403446eb lea r8, [rax + 1]
1403446ef mov qword ptr [rcx], r8
1403446f2 movaps xmm0, xmmword ptr [rbp + 0x10]
1403446f6 movaps xmm1, xmmword ptr [rbp + 0x20]
1403446fa movaps xmm2, xmmword ptr [rbp + 0x30]
1403446fe movups xmmword ptr [rbp + 0x3e0], xmm2
140344705 movups xmmword ptr [rbp + 0x3d0], xmm1
14034470c movups xmmword ptr [rbp + 0x3c0], xmm0
140344713 movaps xmm0, xmmword ptr [rbp + 0x80]
14034471a movaps xmm1, xmmword ptr [rbp + 0x90]
140344721 movaps xmm2, xmmword ptr [rbp + 0xa0]
140344728 movups xmmword ptr [rbp + 0x3f0], xmm0
14034472f movups xmmword ptr [rbp + 0x400], xmm1
140344736 movups xmmword ptr [rbp + 0x410], xmm2
14034473d movaps xmm0, xmmword ptr [rbp + 0x2b0]
140344744 movaps xmm1, xmmword ptr [rbp + 0x2c0]
14034474b movaps xmm2, xmmword ptr [rbp + 0x2d0]
140344752 movups xmmword ptr [rbp + 0x420], xmm0
140344759 movups xmmword ptr [rbp + 0x430], xmm1
140344760 movups xmmword ptr [rbp + 0x440], xmm2
140344767 movaps xmm0, xmmword ptr [rbp + 0x5b0]
14034476e movaps xmm1, xmmword ptr [rbp + 0x5c0]
140344775 movaps xmm2, xmmword ptr [rbp + 0x5d0]
14034477c movups xmmword ptr [rbp + 0x470], xmm2
140344783 movups xmmword ptr [rbp + 0x460], xmm1
14034478a movups xmmword ptr [rbp + 0x450], xmm0
140344791 mov dword ptr [rbp + 0x320], 0
14034479b movups xmm0, xmmword ptr [rbp + 0xf0]
1403447a2 movups xmm1, xmmword ptr [rbp + 0x100]
1403447a9 movups xmm2, xmmword ptr [rbp + 0x110]
1403447b0 movups xmmword ptr [rbp + 0x324], xmm0
1403447b7 movups xmmword ptr [rbp + 0x334], xmm1
1403447be movups xmmword ptr [rbp + 0x344], xmm2
1403447c5 mov dword ptr [rbp + 0x354], 0
1403447cf mov qword ptr [rbp + 0x35c], 0
1403447da movups xmm0, xmmword ptr [rbp - 0x34]
1403447de movups xmmword ptr [rbp + 0x364], xmm0
1403447e5 mov ecx, dword ptr [rbp - 0x24]
1403447e8 mov dword ptr [rbp + 0x374], ecx
1403447ee mov dword ptr [rbp + 0x378], 0
1403447f8 mov ecx, dword ptr [rbp - 0x38]
1403447fb mov dword ptr [rbp + 0x38c], ecx
140344801 movups xmm0, xmmword ptr [rbp - 0x48]
140344805 movups xmmword ptr [rbp + 0x37c], xmm0
14034480c mov dword ptr [rbp + 0x390], 0
140344816 mov ecx, dword ptr [rbp - 0x4c]
140344819 mov dword ptr [rbp + 0x3a4], ecx
14034481f movups xmm0, xmmword ptr [rbp - 0x5c]
140344823 movups xmmword ptr [rbp + 0x394], xmm0
14034482a mov dword ptr [rbp + 0x3a8], 0
140344834 mov ecx, dword ptr [rbp - 0x10]
140344837 mov dword ptr [rbp + 0x3bc], ecx
14034483d movups xmm0, xmmword ptr [rbp - 0x20]
140344841 movups xmmword ptr [rbp + 0x3ac], xmm0
140344848 movups xmmword ptr [rbp + 0x490], xmm6
14034484f movups xmmword ptr [rbp + 0x480], xmm7
140344856 mov qword ptr [rbp + 0x4a0], rax
14034485d mov qword ptr [rbp + 0x4a8], rdx
140344864 lea rdx, [rbp + 0x320]
14034486b mov rcx, rdi
14034486e call 0x140122050
140344873 mov rdi, rax
140344876 lea rcx, [rbp + 0xf0]
14034487d call 0x1402442d0
140344882 call 0x140d4d740
140344887 mov rbx, rax
14034488a test byte ptr [rax], 1
14034488d je 0x1403448a0
14034488f mov rcx, rbx
140344892 add rcx, 8
140344896 mov rax, qword ptr [rbx + 8]
14034489a mov rdx, qword ptr [rbx + 0x10]
14034489e jmp 0x1403448bb
1403448a0 call 0x140d64e00
1403448a5 mov rcx, rbx
1403448a8 add rcx, 8
1403448ac mov qword ptr [rbx], 1
1403448b3 mov qword ptr [rbx + 8], rax
1403448b7 mov qword ptr [rbx + 0x10], rdx
1403448bb lea r8, [rax + 1]
1403448bf mov qword ptr [rcx], r8
1403448c2 movaps xmmword ptr [rbp - 0x10], xmm6
1403448c6 movaps xmmword ptr [rbp - 0x20], xmm7
1403448ca mov qword ptr [rbp], rax
1403448ce mov qword ptr [rbp + 8], rdx
1403448d2 call 0x140d4d740
1403448d7 mov rbx, rax
1403448da test byte ptr [rax], 1
1403448dd je 0x1403448f0
1403448df mov rcx, rbx
1403448e2 add rcx, 8
1403448e6 mov rax, qword ptr [rbx + 8]
1403448ea mov rdx, qword ptr [rbx + 0x10]
1403448ee jmp 0x14034490b
1403448f0 call 0x140d64e00
1403448f5 mov rcx, rbx
1403448f8 add rcx, 8
1403448fc mov qword ptr [rbx], 1
140344903 mov qword ptr [rbx + 8], rax
140344907 mov qword ptr [rbx + 0x10], rdx
14034490b lea r8, [rax + 1]
14034490f mov qword ptr [rcx], r8
140344912 movaps xmmword ptr [rbp + 0x20], xmm6
140344916 movaps xmmword ptr [rbp + 0x10], xmm7
14034491a mov qword ptr [rbp + 0x30], rax
14034491e mov qword ptr [rbp + 0x38], rdx
140344922 call 0x140d4d740
140344927 mov rbx, rax
14034492a test byte ptr [rax], 1
14034492d je 0x140344940
14034492f mov rcx, rbx
140344932 add rcx, 8
140344936 mov rax, qword ptr [rbx + 8]
14034493a mov rdx, qword ptr [rbx + 0x10]
14034493e jmp 0x14034495b
140344940 call 0x140d64e00
140344945 mov rcx, rbx
140344948 add rcx, 8
14034494c mov qword ptr [rbx], 1
140344953 mov qword ptr [rbx + 8], rax
140344957 mov qword ptr [rbx + 0x10], rdx
14034495b lea r8, [rax + 1]
14034495f mov qword ptr [rcx], r8
140344962 movaps xmmword ptr [rbp + 0x90], xmm6
140344969 movaps xmmword ptr [rbp + 0x80], xmm7
140344970 mov qword ptr [rbp + 0xa0], rax
140344977 mov qword ptr [rbp + 0xa8], rdx
14034497e call 0x140d4d740
140344983 mov rbx, rax
140344986 test byte ptr [rax], 1
140344989 je 0x14034499c
14034498b mov rcx, rbx
14034498e add rcx, 8
140344992 mov rax, qword ptr [rbx + 8]
140344996 mov rdx, qword ptr [rbx + 0x10]
14034499a jmp 0x1403449b7
14034499c call 0x140d64e00
1403449a1 mov rcx, rbx
1403449a4 add rcx, 8
1403449a8 mov qword ptr [rbx], 1
1403449af mov qword ptr [rbx + 8], rax
1403449b3 mov qword ptr [rbx + 0x10], rdx
1403449b7 lea r8, [rax + 1]
1403449bb mov qword ptr [rcx], r8
1403449be movaps xmmword ptr [rbp + 0x2c0], xmm6
1403449c5 movaps xmmword ptr [rbp + 0x2b0], xmm7
1403449cc mov qword ptr [rbp + 0x2d0], rax
1403449d3 mov qword ptr [rbp + 0x2d8], rdx
1403449da xor ecx, ecx
1403449dc call 0x1400d9b20
1403449e1 test rax, rax
1403449e4 je 0x14034548b
1403449ea movups xmm0, xmmword ptr [rax]
1403449ed inc qword ptr [rax]
1403449f0 movaps xmmword ptr [rbp + 0x5d0], xmm0
1403449f7 movaps xmmword ptr [rbp + 0x5b0], xmm9
1403449ff movaps xmmword ptr [rbp + 0x5c0], xmm8
140344a07 call 0x140d4d740
140344a0c mov rbx, rax
140344a0f test byte ptr [rax], 1
140344a12 je 0x140344a25
140344a14 mov rax, rbx
140344a17 add rax, 8
140344a1b mov r14, qword ptr [rbx + 8]
140344a1f mov r15, qword ptr [rbx + 0x10]
140344a23 jmp 0x140344a46
140344a25 call 0x140d64e00
140344a2a mov r14, rax
140344a2d mov r15, rdx
140344a30 mov rax, rbx
140344a33 add rax, 8
140344a37 mov qword ptr [rbx], 1
140344a3e mov qword ptr [rbx + 8], r14
140344a42 mov qword ptr [rbx + 0x10], rdx
140344a46 lea rcx, [r14 + 1]
140344a4a mov qword ptr [rax], rcx
140344a4d lea r12, [rbp + 0x320]
140344a54 lea rbx, [rbp + 0xf0]
140344a5b mov r8d, 0x90
140344a61 mov rcx, r12
140344a64 mov rdx, rbx
140344a67 call 0x142923c00
140344a6c movaps xmm0, xmmword ptr [rbp - 0x20]
140344a70 movaps xmm1, xmmword ptr [rbp - 0x10]
140344a74 movaps xmm2, xmmword ptr [rbp]
140344a78 movups xmmword ptr [rbp + 0x3b0], xmm0
140344a7f movups xmmword ptr [rbp + 0x3c0], xmm1
140344a86 movups xmmword ptr [rbp + 0x3d0], xmm2
140344a8d movaps xmm0, xmmword ptr [rbp + 0x10]
140344a91 movaps xmm1, xmmword ptr [rbp + 0x20]
140344a95 movaps xmm2, xmmword ptr [rbp + 0x30]
140344a99 movups xmmword ptr [rbp + 0x3e0], xmm0
140344aa0 movups xmmword ptr [rbp + 0x3f0], xmm1
140344aa7 movups xmmword ptr [rbp + 0x400], xmm2
140344aae movaps xmm0, xmmword ptr [rbp + 0x80]
140344ab5 movaps xmm1, xmmword ptr [rbp + 0x90]
140344abc movaps xmm2, xmmword ptr [rbp + 0xa0]
140344ac3 movups xmmword ptr [rbp + 0x410], xmm0
140344aca movups xmmword ptr [rbp + 0x420], xmm1
140344ad1 movups xmmword ptr [rbp + 0x430], xmm2
140344ad8 movaps xmm0, xmmword ptr [rbp + 0x2b0]
140344adf movaps xmm1, xmmword ptr [rbp + 0x2c0]
140344ae6 movaps xmm2, xmmword ptr [rbp + 0x2d0]
140344aed movups xmmword ptr [rbp + 0x460], xmm2
140344af4 movups xmmword ptr [rbp + 0x450], xmm1
140344afb movups xmmword ptr [rbp + 0x440], xmm0
140344b02 movaps xmm0, xmmword ptr [rbp + 0x5b0]
140344b09 movaps xmm1, xmmword ptr [rbp + 0x5c0]
140344b10 movaps xmm2, xmmword ptr [rbp + 0x5d0]
140344b17 movups xmmword ptr [rbp + 0x470], xmm0
140344b1e movups xmmword ptr [rbp + 0x480], xmm1
140344b25 movups xmmword ptr [rbp + 0x490], xmm2
140344b2c movups xmmword ptr [rbp + 0x4a0], xmm7
140344b33 movups xmmword ptr [rbp + 0x4b0], xmm6
140344b3a mov qword ptr [rbp + 0x4c0], r14
140344b41 mov qword ptr [rbp + 0x4c8], r15
140344b48 mov rcx, rdi
140344b4b mov rdx, r12
140344b4e call 0x14013a710
140344b53 mov rdi, rax
140344b56 call 0x140d4d740
140344b5b mov r14, rax
140344b5e test byte ptr [rax], 1
140344b61 je 0x140344b74
140344b63 mov rcx, r14
140344b66 add rcx, 8
140344b6a mov rax, qword ptr [r14 + 8]
140344b6e mov rdx, qword ptr [r14 + 0x10]
140344b72 jmp 0x140344b8f
140344b74 call 0x140d64e00
140344b79 mov rcx, r14
140344b7c add rcx, 8
140344b80 mov qword ptr [r14], 1
140344b87 mov qword ptr [r14 + 8], rax
140344b8b mov qword ptr [r14 + 0x10], rdx
140344b8f lea r8, [rax + 1]
140344b93 mov qword ptr [rcx], r8
140344b96 movaps xmmword ptr [rbp + 0x100], xmm6
140344b9d movaps xmmword ptr [rbp + 0xf0], xmm7
140344ba4 mov qword ptr [rbp + 0x110], rax
140344bab mov qword ptr [rbp + 0x118], rdx
140344bb2 call 0x140d4d740
140344bb7 mov r14, rax
140344bba test byte ptr [rax], 1
140344bbd je 0x140344bd0
140344bbf mov rcx, r14
140344bc2 add rcx, 8
140344bc6 mov rax, qword ptr [r14 + 8]
140344bca mov rdx, qword ptr [r14 + 0x10]
140344bce jmp 0x140344beb
140344bd0 call 0x140d64e00
140344bd5 mov rcx, r14
140344bd8 add rcx, 8
140344bdc mov qword ptr [r14], 1
140344be3 mov qword ptr [r14 + 8], rax
140344be7 mov qword ptr [r14 + 0x10], rdx
140344beb lea r8, [rax + 1]
140344bef mov qword ptr [rcx], r8
140344bf2 movaps xmm0, xmmword ptr [rbp + 0xf0]
140344bf9 movaps xmm1, xmmword ptr [rbp + 0x100]
140344c00 movaps xmm2, xmmword ptr [rbp + 0x110]
140344c07 movaps xmmword ptr [rbp + 0x340], xmm2
140344c0e movaps xmmword ptr [rbp + 0x330], xmm1
140344c15 movaps xmmword ptr [rbp + 0x320], xmm0
140344c1c movaps xmmword ptr [rbp + 0x350], xmm7
140344c23 movaps xmmword ptr [rbp + 0x360], xmm6
140344c2a mov qword ptr [rbp + 0x370], rax
140344c31 mov qword ptr [rbp + 0x378], rdx
140344c38 xorps xmm0, xmm0
140344c3b movaps xmmword ptr [rbp + 0x380], xmm0
140344c42 movaps xmmword ptr [rbp + 0x390], xmm0
140344c49 mov qword ptr [rbp + 0x3a0], 0
140344c54 lea rdx, [rbp + 0x320]
140344c5b mov rcx, rdi
140344c5e call 0x140114840
140344c63 mov rdi, rax
140344c66 call 0x140d4d740
140344c6b mov r14, rax
140344c6e test byte ptr [rax], 1
140344c71 je 0x140344c84
140344c73 mov rcx, r14
140344c76 add rcx, 8
140344c7a mov rax, qword ptr [r14 + 8]
140344c7e mov rdx, qword ptr [r14 + 0x10]
140344c82 jmp 0x140344c9f
140344c84 call 0x140d64e00
140344c89 mov rcx, r14
140344c8c add rcx, 8
140344c90 mov qword ptr [r14], 1
140344c97 mov qword ptr [r14 + 8], rax
140344c9b mov qword ptr [r14 + 0x10], rdx
140344c9f lea r8, [rax + 1]
140344ca3 mov qword ptr [rcx], r8
140344ca6 movups xmmword ptr [rbp + 0x338], xmm6
140344cad movups xmmword ptr [rbp + 0x328], xmm7
140344cb4 mov qword ptr [rbp + 0x348], rax
140344cbb mov qword ptr [rbp + 0x350], rdx
140344cc2 mov dword ptr [rbp + 0x320], 0
140344ccc xorps xmm0, xmm0
140344ccf movups xmmword ptr [rbp + 0x358], xmm0
140344cd6 lea rdx, [rbp + 0x320]
140344cdd mov rcx, rdi
140344ce0 call 0x1401367e0
140344ce5 mov rdi, rax
140344ce8 call 0x140d4d740
140344ced mov r14, rax
140344cf0 test byte ptr [rax], 1
140344cf3 je 0x140344d06
140344cf5 mov rcx, r14
140344cf8 add rcx, 8
140344cfc mov rax, qword ptr [r14 + 8]
140344d00 mov rdx, qword ptr [r14 + 0x10]
140344d04 jmp 0x140344d21
140344d06 call 0x140d64e00
140344d0b mov rcx, r14
140344d0e add rcx, 8
140344d12 mov qword ptr [r14], 1
140344d19 mov qword ptr [r14 + 8], rax
140344d1d mov qword ptr [r14 + 0x10], rdx
140344d21 lea r8, [rax + 1]
140344d25 mov qword ptr [rcx], r8
140344d28 movups xmmword ptr [rbp + 0x338], xmm6
140344d2f movups xmmword ptr [rbp + 0x328], xmm7
140344d36 mov qword ptr [rbp + 0x348], rax
140344d3d mov qword ptr [rbp + 0x350], rdx
140344d44 mov dword ptr [rbp + 0x320], 0
140344d4e xorps xmm0, xmm0
140344d51 movups xmmword ptr [rbp + 0x358], xmm0
140344d58 movups xmmword ptr [rbp + 0x364], xmm0
140344d5f lea rdx, [rbp + 0x320]
140344d66 mov rcx, rdi
140344d69 call 0x1401356b0
140344d6e mov rdi, rax
140344d71 lea rcx, [rax + 0x38]
140344d75 movabs rdx, 0x8e1bd251962ba784
140344d7f movabs r8, 0x6a782374c0e6529b
140344d89 call 0x1400dd6f0
140344d8e test rax, rax
140344d91 je 0x1403453ce
140344d97 cmp qword ptr [rdi + 0x218], rdx
140344d9e jbe 0x1403453ce
140344da4 mov rax, qword ptr [rdi + 0x210]
140344dab mov rcx, qword ptr [rax + rdx*8]
140344daf test rcx, rcx
140344db2 je 0x1403453ce
140344db8 mov rax, qword ptr [rdi + 0x1e0]
140344dbf not rcx
140344dc2 lea rcx, [rcx + rcx*2]
140344dc6 shl rcx, 5
140344dca cmp qword ptr [rax + rcx + 0x20], 0
140344dd0 je 0x1403453ce
140344dd6 add rax, rcx
140344dd9 mov rcx, qword ptr [rax + 0x10]
140344ddd mov edx, dword ptr [rdi + 0x5b0]
140344de3 mov dword ptr [rax + 0x5c], edx
140344de6 call 0x140170fb0
140344deb mov rcx, rdi
140344dee call 0x140327350
140344df3 lea rsi, [rbp + 0x320]
140344dfa mov rcx, rsi
140344dfd call 0x1400b2010
140344e02 lea r9, [rip + 0x267e097]
140344e09 mov rcx, rbx
140344e0c mov rdx, rsi
140344e0f mov r8b, 0xa
140344e12 call 0x1402957c0
140344e17 mov rcx, rdi
140344e1a mov rdx, rbx
140344e1d call 0x1403276f0
140344e22 lea rcx, [rbp + 0x10]
140344e26 call 0x140105360
140344e2b lea rcx, [rbp + 0x5b0]
140344e32 call 0x1402d4810
140344e37 lea rcx, [rbp + 0xf0]
140344e3e call 0x140553030
140344e43 movups xmm0, xmmword ptr [rbp + 0x610]
140344e4a movaps xmmword ptr [rbp + 0x380], xmm0
140344e51 movups xmm0, xmmword ptr [rbp + 0x600]
140344e58 movaps xmmword ptr [rbp + 0x370], xmm0
140344e5f movups xmm0, xmmword ptr [rbp + 0x5f0]
140344e66 movaps xmmword ptr [rbp + 0x360], xmm0
140344e6d movups xmm0, xmmword ptr [rbp + 0x5b0]
140344e74 movups xmm1, xmmword ptr [rbp + 0x5c0]
140344e7b movups xmm2, xmmword ptr [rbp + 0x5d0]
140344e82 movups xmm3, xmmword ptr [rbp + 0x5e0]
140344e89 movaps xmmword ptr [rbp + 0x350], xmm3
140344e90 movaps xmmword ptr [rbp + 0x340], xmm2
140344e97 movaps xmmword ptr [rbp + 0x330], xmm1
140344e9e movaps xmmword ptr [rbp + 0x320], xmm0
140344ea5 movups xmm0, xmmword ptr [rbp + 0xf0]
140344eac movups xmm1, xmmword ptr [rbp + 0x100]
140344eb3 movups xmm2, xmmword ptr [rbp + 0x110]
140344eba movups xmm3, xmmword ptr [rbp + 0x120]
140344ec1 movaps xmmword ptr [rbp + 0x390], xmm0
140344ec8 movaps xmmword ptr [rbp + 0x3a0], xmm1
140344ecf movaps xmmword ptr [rbp + 0x3b0], xmm2
140344ed6 movaps xmmword ptr [rbp + 0x3c0], xmm3
140344edd movups xmm0, xmmword ptr [rbp + 0x130]
140344ee4 movaps xmmword ptr [rbp + 0x3d0], xmm0
140344eeb movups xmm0, xmmword ptr [rbp + 0x140]
140344ef2 movaps xmmword ptr [rbp + 0x3e0], xmm0
140344ef9 movups xmm0, xmmword ptr [rbp + 0x150]
140344f00 movaps xmmword ptr [rbp + 0x3f0], xmm0
140344f07 lea rcx, [rbp + 0x80]
140344f0e lea rdx, [rbp + 0x320]
140344f15 call 0x14026c790
140344f1a lea rcx, [rbp + 0x2b0]
140344f21 call 0x140287410
140344f26 lea rcx, [rbp + 0x5b0]
140344f2d call 0x140286610
140344f32 lea rcx, [rbp + 0x320]
140344f39 call 0x140492d80
140344f3e movups xmm0, xmmword ptr [rbp + 0x380]
140344f45 movaps xmmword ptr [rbp + 0x150], xmm0
140344f4c movups xmm0, xmmword ptr [rbp + 0x370]
140344f53 movaps xmmword ptr [rbp + 0x140], xmm0
140344f5a movups xmm0, xmmword ptr [rbp + 0x360]
140344f61 movaps xmmword ptr [rbp + 0x130], xmm0
140344f68 movups xmm0, xmmword ptr [rbp + 0x320]
140344f6f movups xmm1, xmmword ptr [rbp + 0x330]
140344f76 movups xmm2, xmmword ptr [rbp + 0x340]
140344f7d movups xmm3, xmmword ptr [rbp + 0x350]
140344f84 movaps xmmword ptr [rbp + 0x120], xmm3
140344f8b movaps xmmword ptr [rbp + 0x110], xmm2
140344f92 movaps xmmword ptr [rbp + 0x100], xmm1
140344f99 movaps xmmword ptr [rbp + 0xf0], xmm0
140344fa0 lea rcx, [rbp + 0x320]
140344fa7 lea rdx, [rbp + 0xf0]
140344fae call 0x1403d6ce0
140344fb3 movups xmm0, xmmword ptr [rbp + 0x5b0]
140344fba movups xmm1, xmmword ptr [rbp + 0x5c0]
140344fc1 movups xmm2, xmmword ptr [rbp + 0x5d0]
140344fc8 movups xmm3, xmmword ptr [rbp + 0x5e0]
140344fcf movaps xmmword ptr [rbp + 0xf0], xmm0
140344fd6 movaps xmmword ptr [rbp + 0x100], xmm1
140344fdd movaps xmmword ptr [rbp + 0x110], xmm2
140344fe4 movaps xmmword ptr [rbp + 0x120], xmm3
140344feb movups xmm0, xmmword ptr [rbp + 0x5f0]
140344ff2 movaps xmmword ptr [rbp + 0x130], xmm0
140344ff9 movups xmm0, xmmword ptr [rbp + 0x600]
140345000 movaps xmmword ptr [rbp + 0x140], xmm0
140345007 movups xmm0, xmmword ptr [rbp + 0x610]
14034500e movaps xmmword ptr [rbp + 0x150], xmm0
140345015 movups xmm0, xmmword ptr [rbp + 0x320]
14034501c movups xmm1, xmmword ptr [rbp + 0x330]
140345023 movups xmm2, xmmword ptr [rbp + 0x340]
14034502a movups xmm3, xmmword ptr [rbp + 0x350]
140345031 movaps xmmword ptr [rbp + 0x160], xmm0
140345038 movaps xmmword ptr [rbp + 0x170], xmm1
14034503f movaps xmmword ptr [rbp + 0x180], xmm2
140345046 movaps xmmword ptr [rbp + 0x190], xmm3
14034504d movups xmm0, xmmword ptr [rbp + 0x360]
140345054 movaps xmmword ptr [rbp + 0x1a0], xmm0
14034505b movups xmm0, xmmword ptr [rbp + 0x370]
140345062 movaps xmmword ptr [rbp + 0x1b0], xmm0
140345069 movups xmm0, xmmword ptr [rbp + 0x380]
140345070 movaps xmmword ptr [rbp + 0x1c0], xmm0
140345077 movups xmm0, xmmword ptr [rbp + 0x70]
14034507b movaps xmmword ptr [rbp + 0x380], xmm0
140345082 movups xmm0, xmmword ptr [rbp + 0x60]
140345086 movaps xmmword ptr [rbp + 0x370], xmm0
14034508d movups xmm0, xmmword ptr [rbp + 0x50]
140345091 movaps xmmword ptr [rbp + 0x360], xmm0
140345098 movups xmm0, xmmword ptr [rbp + 0x10]
14034509c movups xmm1, xmmword ptr [rbp + 0x20]
1403450a0 movups xmm2, xmmword ptr [rbp + 0x30]
1403450a4 movups xmm3, xmmword ptr [rbp + 0x40]
1403450a8 movaps xmmword ptr [rbp + 0x350], xmm3
1403450af movaps xmmword ptr [rbp + 0x340], xmm2
1403450b6 movaps xmmword ptr [rbp + 0x330], xmm1
1403450bd movaps xmmword ptr [rbp + 0x320], xmm0
1403450c4 movups xmm0, xmmword ptr [rbp + 0xe0]
1403450cb movaps xmmword ptr [rbp + 0x3f0], xmm0
1403450d2 movups xmm0, xmmword ptr [rbp + 0xd0]
1403450d9 movaps xmmword ptr [rbp + 0x3e0], xmm0
1403450e0 movups xmm0, xmmword ptr [rbp + 0xc0]
1403450e7 movaps xmmword ptr [rbp + 0x3d0], xmm0
1403450ee movups xmm0, xmmword ptr [rbp + 0x80]
1403450f5 movups xmm1, xmmword ptr [rbp + 0x90]
1403450fc movups xmm2, xmmword ptr [rbp + 0xa0]
140345103 movups xmm3, xmmword ptr [rbp + 0xb0]
14034510a movaps xmmword ptr [rbp + 0x3c0], xmm3
140345111 movaps xmmword ptr [rbp + 0x3b0], xmm2
140345118 movaps xmmword ptr [rbp + 0x3a0], xmm1
14034511f movaps xmmword ptr [rbp + 0x390], xmm0
140345126 movups xmm0, xmmword ptr [rbp + 0x310]
14034512d movaps xmmword ptr [rbp + 0x460], xmm0
140345134 movups xmm0, xmmword ptr [rbp + 0x300]
14034513b movaps xmmword ptr [rbp + 0x450], xmm0
140345142 movups xmm0, xmmword ptr [rbp + 0x2f0]
140345149 movaps xmmword ptr [rbp + 0x440], xmm0
140345150 movups xmm0, xmmword ptr [rbp + 0x2b0]
140345157 movups xmm1, xmmword ptr [rbp + 0x2c0]
14034515e movups xmm2, xmmword ptr [rbp + 0x2d0]
140345165 movups xmm3, xmmword ptr [rbp + 0x2e0]
14034516c movaps xmmword ptr [rbp + 0x430], xmm3
140345173 movaps xmmword ptr [rbp + 0x420], xmm2
14034517a movaps xmmword ptr [rbp + 0x410], xmm1
140345181 movaps xmmword ptr [rbp + 0x400], xmm0
140345188 lea rcx, [rbp + 0x470]
14034518f lea rdx, [rbp + 0xf0]
140345196 mov r8d, 0xe0
14034519c call 0x142923c00
1403451a1 lea rcx, [rbp + 0xf0]
1403451a8 lea rdx, [rbp + 0x320]
1403451af call 0x1400d52f0
1403451b4 mov byte ptr [rbp + 0x5b0], 0x19
1403451bb mov rcx, qword ptr [rip + 0x32e71ae]
1403451c2 lea r8, [rip + 0x25e5dcf]
1403451c9 lea rdx, [rbp + 0x5b0]
1403451d0 call 0x140364d20
1403451d5 mov r8, rdx
1403451d8 lea rcx, [rbp + 0xf0]
1403451df mov rdx, rax
1403451e2 call 0x1400d68c0
1403451e7 movups xmm0, xmmword ptr [rbp + 0x150]
1403451ee movaps xmmword ptr [rbp + 0x380], xmm0
1403451f5 movups xmm0, xmmword ptr [rbp + 0x140]
1403451fc movaps xmmword ptr [rbp + 0x370], xmm0
140345203 movups xmm0, xmmword ptr [rbp + 0x130]
14034520a movaps xmmword ptr [rbp + 0x360], xmm0
140345211 movups xmm0, xmmword ptr [rbp + 0xf0]
140345218 movups xmm1, xmmword ptr [rbp + 0x100]
14034521f movups xmm2, xmmword ptr [rbp + 0x110]
140345226 movups xmm3, xmmword ptr [rbp + 0x120]
14034522d movaps xmmword ptr [rbp + 0x350], xmm3
140345234 movaps xmmword ptr [rbp + 0x340], xmm2
14034523b movaps xmmword ptr [rbp + 0x330], xmm1
140345242 movaps xmmword ptr [rbp + 0x320], xmm0
140345249 lea rdx, [rbp + 0x320]
140345250 mov rcx, rdi
140345253 call 0x1403276f0
140345258 lea r8, [rip + 0x267dc59]
14034525f lea rcx, [rbp + 0x5b0]
140345266 mov dl, 0x28
140345268 call 0x140322fc0
14034526d lea r8, [rip + 0x267dc5c]
140345274 lea rcx, [rbp + 0xf0]
14034527b mov dl, 0x22
14034527d call 0x140258060
140345282 movups xmm0, xmmword ptr [rbp + 0x610]
140345289 movaps xmmword ptr [rbp + 0x380], xmm0
140345290 movups xmm0, xmmword ptr [rbp + 0x600]
140345297 movaps xmmword ptr [rbp + 0x370], xmm0
14034529e movups xmm0, xmmword ptr [rbp + 0x5f0]
1403452a5 movaps xmmword ptr [rbp + 0x360], xmm0
1403452ac movups xmm0, xmmword ptr [rbp + 0x5b0]
1403452b3 movups xmm1, xmmword ptr [rbp + 0x5c0]
1403452ba movups xmm2, xmmword ptr [rbp + 0x5d0]
1403452c1 movups xmm3, xmmword ptr [rbp + 0x5e0]
1403452c8 movaps xmmword ptr [rbp + 0x350], xmm3
1403452cf movaps xmmword ptr [rbp + 0x340], xmm2
1403452d6 movaps xmmword ptr [rbp + 0x330], xmm1
1403452dd movaps xmmword ptr [rbp + 0x320], xmm0
1403452e4 movups xmm0, xmmword ptr [rbp + 0xf0]
1403452eb movups xmm1, xmmword ptr [rbp + 0x100]
1403452f2 movups xmm2, xmmword ptr [rbp + 0x110]
1403452f9 movups xmm3, xmmword ptr [rbp + 0x120]
140345300 movaps xmmword ptr [rbp + 0x390], xmm0
140345307 movaps xmmword ptr [rbp + 0x3a0], xmm1
14034530e movaps xmmword ptr [rbp + 0x3b0], xmm2
140345315 movaps xmmword ptr [rbp + 0x3c0], xmm3
14034531c movups xmm0, xmmword ptr [rbp + 0x130]
140345323 movaps xmmword ptr [rbp + 0x3d0], xmm0
14034532a movups xmm0, xmmword ptr [rbp + 0x140]
140345331 movaps xmmword ptr [rbp + 0x3e0], xmm0
140345338 movups xmm0, xmmword ptr [rbp + 0x150]
14034533f movaps xmmword ptr [rbp + 0x3f0], xmm0
140345346 lea rdx, [rbp + 0x320]
14034534d mov rcx, rdi
140345350 call 0x140327950
140345355 lea r8, [rip + 0x267db8c]
14034535c mov rcx, rsi
14034535f mov dl, 0x24
140345361 call 0x1400c9130
140345366 mov rcx, rdi
140345369 mov rdx, rsi
14034536c call 0x1403276f0
140345371 mov rcx, rdi
140345374 mov dl, 1
140345376 movaps xmm6, xmmword ptr [rbp + 0x650]
14034537d movaps xmm7, xmmword ptr [rbp + 0x660]
140345384 movaps xmm8, xmmword ptr [rbp + 0x670]
14034538c movaps xmm9, xmmword ptr [rbp + 0x680]
140345394 movaps xmm10, xmmword ptr [rbp + 0x690]
14034539c movaps xmm11, xmmword ptr [rbp + 0x6a0]
1403453a4 movaps xmm12, xmmword ptr [rbp + 0x6b0]
1403453ac add rsp, 0x740
1403453b3 pop rbx
1403453b4 pop rdi
1403453b5 pop rsi
1403453b6 pop r12
1403453b8 pop r14
1403453ba pop r15
1403453bc pop rbp
1403453bd jmp 0x140326f80
1403453c2 lea rcx, [rip + 0x25ed647]
1403453c9 call 0x1428d80d0
1403453ce lea rax, [rip + 0x2678e63]
1403453d5 mov qword ptr [rbp + 0xf0], rax
1403453dc mov qword ptr [rbp + 0xf8], 0x27
1403453e7 mov qword ptr [rbp + 0x5b0], rbx
1403453ee lea rax, [rip - 0x23b565]
1403453f5 mov qword ptr [rbp + 0x5b8], rax
1403453fc lea rax, [rip + 0x2678f65]
140345403 mov qword ptr [rbp + 0x320], rax
14034540a mov qword ptr [rbp + 0x328], 2
140345415 mov qword ptr [rbp + 0x340], 0
140345420 mov qword ptr [rbp + 0x330], rsi
140345427 mov qword ptr [rbp + 0x338], 1
140345432 lea rdx, [rip + 0x2678db7]
140345439 lea rcx, [rbp + 0x320]
140345440 call 0x1428d9390
140345445 lea rcx, [rip + 0x25ed5c4]
14034544c call 0x1428d80d0
140345451 jmp 0x140345497
140345453 lea rcx, [rip + 0x25ed5b6]
14034545a call 0x1428d80d0
14034545f jmp 0x140345497
140345461 lea rcx, [rip + 0x25ed5a8]
140345468 call 0x1428d80d0
14034546d jmp 0x140345497
14034546f lea rcx, [rip + 0x25ed59a]
140345476 call 0x1428d80d0
14034547b jmp 0x140345497
14034547d lea rcx, [rip + 0x25ed58c]
140345484 call 0x1428d80d0
140345489 jmp 0x140345497