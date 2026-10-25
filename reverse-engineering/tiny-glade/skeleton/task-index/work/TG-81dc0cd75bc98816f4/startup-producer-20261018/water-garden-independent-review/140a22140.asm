140a22140: 55 push rbp
140a22141: 4157 push r15
140a22143: 4156 push r14
140a22145: 56 push rsi
140a22146: 57 push rdi
140a22147: 53 push rbx
140a22148: 4881ecf8030000 sub rsp, 0x3f8
140a2214f: 488dac2480000000 lea rbp, [rsp + 0x80]
140a22157: 0f29b560030000 movaps xmmword ptr [rbp + 0x360], xmm6
140a2215e: 48c78558030000feffffff mov qword ptr [rbp + 0x358], 0xfffffffffffffffe
140a22169: 4889d6 mov rsi, rdx
140a2216c: 4889cf mov rdi, rcx
140a2216f: 488d4a38 lea rcx, [rdx + 0x38]
140a22173: 48ba9a25aa8862fa9f97 movabs rdx, 0x979ffa6288aa259a
140a2217d: 49b89d44ed465b1d7909 movabs r8, 0x9791d5b46ed449d
140a22187: e864b56bff call 0x1400dd6f0
140a2218c: 4885c0 test rax, rax
140a2218f: 0f84af060000 je 0x140a22844
140a22195: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
140a2219c: 0f86a2060000 jbe 0x140a22844
140a221a2: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
140a221a9: 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
140a221ad: 4885c9 test rcx, rcx
140a221b0: 0f848e060000 je 0x140a22844
140a221b6: 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
140a221bd: 48f7d1 not rcx
140a221c0: 488d0c49 lea rcx, [rcx + rcx*2]
140a221c4: 48c1e105 shl rcx, 5
140a221c8: 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
140a221ce: 0f8470060000 je 0x140a22844
140a221d4: 4801c8 add rax, rcx
140a221d7: 488b4810 mov rcx, qword ptr [rax + 0x10]
140a221db: 8b96b0050000 mov edx, dword ptr [rsi + 0x5b0]
140a221e1: 80b99b02000000 cmp byte ptr [rcx + 0x29b], 0
140a221e8: 0f843f060000 je 0x140a2282d
140a221ee: 89505c mov dword ptr [rax + 0x5c], edx
140a221f1: c6819b02000000 mov byte ptr [rcx + 0x29b], 0
140a221f8: 4881c198000000 add rcx, 0x98
140a221ff: 41b800020000 mov r8d, 0x200
140a22205: 31d2 xor edx, edx
140a22207: e8141af001 call 0x142923c20
140a2220c: 488d8d00030000 lea rcx, [rbp + 0x300]
140a22213: 48ba0002000000020000 movabs rdx, 0x20000000200
140a2221d: 41b861000000 mov r8d, 0x61
140a22223: e878301b00 call 0x140bd52a0
140a22228: 0f108500030000 movups xmm0, xmmword ptr [rbp + 0x300]
140a2222f: 0f108d10030000 movups xmm1, xmmword ptr [rbp + 0x310]
140a22236: 0f298d80010000 movaps xmmword ptr [rbp + 0x180], xmm1
140a2223d: 0f298570010000 movaps xmmword ptr [rbp + 0x170], xmm0
140a22244: 488d4df0 lea rcx, [rbp - 0x10]
140a22248: 488d9570010000 lea rdx, [rbp + 0x170]
140a2224f: e88c4a1b00 call 0x140bd6ce0
140a22254: 488d05cd760a02 lea rax, [rip + 0x20a76cd]
140a2225b: 488945e0 mov qword ptr [rbp - 0x20], rax
140a2225f: 48c745e810000000 mov qword ptr [rbp - 0x18], 0x10
140a22267: 488d55e0 lea rdx, [rbp - 0x20]
140a2226b: 4889f9 mov rcx, rdi
140a2226e: e85d721c00 call 0x140be94d0
140a22273: 0f108500030000 movups xmm0, xmmword ptr [rbp + 0x300]
140a2227a: 0f108d10030000 movups xmm1, xmmword ptr [rbp + 0x310]
140a22281: 0f1145c0 movups xmmword ptr [rbp - 0x40], xmm0
140a22285: 0f114dd0 movups xmmword ptr [rbp - 0x30], xmm1
140a22289: 8945bc mov dword ptr [rbp - 0x44], eax
140a2228c: 48897c2430 mov qword ptr [rsp + 0x30], rdi
140a22291: 4889742428 mov qword ptr [rsp + 0x28], rsi
140a22296: 48c744242018000000 mov qword ptr [rsp + 0x20], 0x18
140a2229f: 488d1592760a02 lea rdx, [rip + 0x20a7692]
140a222a6: 4c8d0d9b760a02 lea r9, [rip + 0x20a769b]
140a222ad: 488d4de0 lea rcx, [rbp - 0x20]
140a222b1: 41b810000000 mov r8d, 0x10
140a222b7: e8e40fbe00 call 0x1416032a0
140a222bc: 80bd1601000008 cmp byte ptr [rbp + 0x116], 8
140a222c3: 7425 je 0x140a222ea
140a222c5: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a222cc: bae8000000 mov edx, 0xe8
140a222d1: 48039518010000 add rdx, qword ptr [rbp + 0x118]
140a222d8: 49b822bc942037f81fd9 movabs r8, 0xd91ff8372094bc22
140a222e2: 41b101 mov r9b, 1
140a222e5: e86695f4ff call 0x14096b850
140a222ea: 488d9d70010000 lea rbx, [rbp + 0x170]
140a222f1: 4c8d75e0 lea r14, [rbp - 0x20]
140a222f5: 41b890010000 mov r8d, 0x190
140a222fb: 4889d9 mov rcx, rbx
140a222fe: 4c89f2 mov rdx, r14
140a22301: e8fa18f001 call 0x142923c00
140a22306: c6850003000014 mov byte ptr [rbp + 0x300], 0x14
140a2230d: c745e002000000 mov dword ptr [rbp - 0x20], 2
140a22314: 4c89742420 mov qword ptr [rsp + 0x20], r14
140a22319: 488d8d00030000 lea rcx, [rbp + 0x300]
140a22320: 49b841f0c818cd3cd66a movabs r8, 0x6ad63ccd18c8f041
140a2232a: 4889da mov rdx, rbx
140a2232d: 41b119 mov r9b, 0x19
140a22330: e82b2feeff call 0x140905260
140a22335: 488d5de0 lea rbx, [rbp - 0x20]
140a22339: 4c8db570010000 lea r14, [rbp + 0x170]
140a22340: 41b890010000 mov r8d, 0x190
140a22346: 4889d9 mov rcx, rbx
140a22349: 4c89f2 mov rdx, r14
140a2234c: e8af18f001 call 0x142923c00
140a22351: 4c8d7dbc lea r15, [rbp - 0x44]
140a22355: 4c89bd00030000 mov qword ptr [rbp + 0x300], r15
140a2235c: c7857001000002000000 mov dword ptr [rbp + 0x170], 2
140a22366: 4c89742420 mov qword ptr [rsp + 0x20], r14
140a2236b: 488d8d00030000 lea rcx, [rbp + 0x300]
140a22372: 49b868077db5a7c3f7e8 movabs r8, 0xe8f7c3a7b57d0768
140a2237c: 4889da mov rdx, rbx
140a2237f: 41b132 mov r9b, 0x32
140a22382: e879b9ecff call 0x1408edd00
140a22387: 488d8d70010000 lea rcx, [rbp + 0x170]
140a2238e: 488d55e0 lea rdx, [rbp - 0x20]
140a22392: 41b890010000 mov r8d, 0x190
140a22398: e86318f001 call 0x142923c00
140a2239d: 80bda602000008 cmp byte ptr [rbp + 0x2a6], 8
140a223a4: 742a je 0x140a223d0
140a223a6: 488d8d80020000 lea rcx, [rbp + 0x280]
140a223ad: bae8000000 mov edx, 0xe8
140a223b2: 480395a8020000 add rdx, qword ptr [rbp + 0x2a8]
140a223b9: 49b894a792304f2c02dc movabs r8, 0xdc022c4f3092a794
140a223c3: f30f101dbd35f001 movss xmm3, dword ptr [rip + 0x1f035bd]
140a223cb: e86096f4ff call 0x14096ba30
140a223d0: 488d4de0 lea rcx, [rbp - 0x20]
140a223d4: 488d9570010000 lea rdx, [rbp + 0x170]
140a223db: 41b890010000 mov r8d, 0x190
140a223e1: e81a18f001 call 0x142923c00
140a223e6: 80bd1601000008 cmp byte ptr [rbp + 0x116], 8
140a223ed: 7428 je 0x140a22417
140a223ef: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a223f6: bae8000000 mov edx, 0xe8
140a223fb: 48039518010000 add rdx, qword ptr [rbp + 0x118]
140a22402: 49b828ae4f9d5b6cd74a movabs r8, 0x4ad76c5b9d4fae28
140a2240c: 41b901000000 mov r9d, 1
140a22412: e85991f4ff call 0x14096b570
140a22417: 488d8d70010000 lea rcx, [rbp + 0x170]
140a2241e: 488d55e0 lea rdx, [rbp - 0x20]
140a22422: 41b890010000 mov r8d, 0x190
140a22428: e8d317f001 call 0x142923c00
140a2242d: 0f108510020000 movups xmm0, xmmword ptr [rbp + 0x210]
140a22434: 0f298540030000 movaps xmmword ptr [rbp + 0x340], xmm0
140a2243b: 0f108500020000 movups xmm0, xmmword ptr [rbp + 0x200]
140a22442: 0f298530030000 movaps xmmword ptr [rbp + 0x330], xmm0
140a22449: 0f1085f0010000 movups xmm0, xmmword ptr [rbp + 0x1f0]
140a22450: 0f298520030000 movaps xmmword ptr [rbp + 0x320], xmm0
140a22457: 0f1085e0010000 movups xmm0, xmmword ptr [rbp + 0x1e0]
140a2245e: 0f298510030000 movaps xmmword ptr [rbp + 0x310], xmm0
140a22465: 0f1085d0010000 movups xmm0, xmmword ptr [rbp + 0x1d0]
140a2246c: 0f298500030000 movaps xmmword ptr [rbp + 0x300], xmm0
140a22473: 488b85a8020000 mov rax, qword ptr [rbp + 0x2a8]
140a2247a: 488b8df0020000 mov rcx, qword ptr [rbp + 0x2f0]
140a22481: 488b9580010000 mov rdx, qword ptr [rbp + 0x180]
140a22488: 488955f0 mov qword ptr [rbp - 0x10], rdx
140a2248c: 0f108570010000 movups xmm0, xmmword ptr [rbp + 0x170]
140a22493: 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
140a22497: 0f108588010000 movups xmm0, xmmword ptr [rbp + 0x188]
140a2249e: 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
140a224a2: 488b9598010000 mov rdx, qword ptr [rbp + 0x198]
140a224a9: 48895520 mov qword ptr [rbp + 0x20], rdx
140a224ad: 0f1085a0010000 movups xmm0, xmmword ptr [rbp + 0x1a0]
140a224b4: 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
140a224b8: 488b95b0010000 mov rdx, qword ptr [rbp + 0x1b0]
140a224bf: 48895508 mov qword ptr [rbp + 8], rdx
140a224c3: 0f108520020000 movups xmm0, xmmword ptr [rbp + 0x220]
140a224ca: 0f108d30020000 movups xmm1, xmmword ptr [rbp + 0x230]
140a224d1: 0f109540020000 movups xmm2, xmmword ptr [rbp + 0x240]
140a224d8: 0f109d50020000 movups xmm3, xmmword ptr [rbp + 0x250]
140a224df: 0f114558 movups xmmword ptr [rbp + 0x58], xmm0
140a224e3: 0f114d68 movups xmmword ptr [rbp + 0x68], xmm1
140a224e7: 0f115578 movups xmmword ptr [rbp + 0x78], xmm2
140a224eb: 0f119d88000000 movups xmmword ptr [rbp + 0x88], xmm3
140a224f2: 0f108560020000 movups xmm0, xmmword ptr [rbp + 0x260]
140a224f9: 0f118598000000 movups xmmword ptr [rbp + 0x98], xmm0
140a22500: 0f108570020000 movups xmm0, xmmword ptr [rbp + 0x270]
140a22507: 0f1185a8000000 movups xmmword ptr [rbp + 0xa8], xmm0
140a2250e: 488b95c8010000 mov rdx, qword ptr [rbp + 0x1c8]
140a22515: 48895538 mov qword ptr [rbp + 0x38], rdx
140a22519: 0f1085b8010000 movups xmm0, xmmword ptr [rbp + 0x1b8]
140a22520: 0f114528 movups xmmword ptr [rbp + 0x28], xmm0
140a22524: 48898dc0000000 mov qword ptr [rbp + 0xc0], rcx
140a2252b: 488985b8000000 mov qword ptr [rbp + 0xb8], rax
140a22532: 0f2835c7ff0702 movaps xmm6, xmmword ptr [rip + 0x207ffc7]
140a22539: 0f297540 movaps xmmword ptr [rbp + 0x40], xmm6
140a2253d: 488d8d00030000 lea rcx, [rbp + 0x300]
140a22544: 488d55e0 lea rdx, [rbp - 0x20]
140a22548: e883affbff call 0x1409dd4d0
140a2254d: 80bda602000008 cmp byte ptr [rbp + 0x2a6], 8
140a22554: 740c je 0x140a22562
140a22556: 488d8d90020000 lea rcx, [rbp + 0x290]
140a2255d: e84eff1b00 call 0x140be24b0
140a22562: 48897c2430 mov qword ptr [rsp + 0x30], rdi
140a22567: 4889742428 mov qword ptr [rsp + 0x28], rsi
140a2256c: 48c744242018000000 mov qword ptr [rsp + 0x20], 0x18
140a22575: 488d15bc730a02 lea rdx, [rip + 0x20a73bc]
140a2257c: 4c8d0dc5730a02 lea r9, [rip + 0x20a73c5]
140a22583: 488d4de0 lea rcx, [rbp - 0x20]
140a22587: 41b810000000 mov r8d, 0x10
140a2258d: e80e0dbe00 call 0x1416032a0
140a22592: 80bd1601000008 cmp byte ptr [rbp + 0x116], 8
140a22599: 7425 je 0x140a225c0
140a2259b: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a225a2: bae8000000 mov edx, 0xe8
140a225a7: 48039518010000 add rdx, qword ptr [rbp + 0x118]
140a225ae: 49b822bc942037f81fd9 movabs r8, 0xd91ff8372094bc22
140a225b8: 4531c9 xor r9d, r9d
140a225bb: e89092f4ff call 0x14096b850
140a225c0: 488db570010000 lea rsi, [rbp + 0x170]
140a225c7: 488d7de0 lea rdi, [rbp - 0x20]
140a225cb: 41b890010000 mov r8d, 0x190
140a225d1: 4889f1 mov rcx, rsi
140a225d4: 4889fa mov rdx, rdi
140a225d7: e82416f001 call 0x142923c00
140a225dc: 4c89bd00030000 mov qword ptr [rbp + 0x300], r15
140a225e3: c745e002000000 mov dword ptr [rbp - 0x20], 2
140a225ea: 48897c2420 mov qword ptr [rsp + 0x20], rdi
140a225ef: 488d8d00030000 lea rcx, [rbp + 0x300]
140a225f6: 49b841f0c818cd3cd66a movabs r8, 0x6ad63ccd18c8f041
140a22600: 4889f2 mov rdx, rsi
140a22603: 41b119 mov r9b, 0x19
140a22606: e835bcecff call 0x1408ee240
140a2260b: 488d75e0 lea rsi, [rbp - 0x20]
140a2260f: 488dbd70010000 lea rdi, [rbp + 0x170]
140a22616: 41b890010000 mov r8d, 0x190
140a2261c: 4889f1 mov rcx, rsi
140a2261f: 4889fa mov rdx, rdi
140a22622: e8d915f001 call 0x142923c00
140a22627: c6850003000014 mov byte ptr [rbp + 0x300], 0x14
140a2262e: c7857001000002000000 mov dword ptr [rbp + 0x170], 2
140a22638: 48897c2420 mov qword ptr [rsp + 0x20], rdi
140a2263d: 488d8d00030000 lea rcx, [rbp + 0x300]
140a22644: 49b868077db5a7c3f7e8 movabs r8, 0xe8f7c3a7b57d0768
140a2264e: 4889f2 mov rdx, rsi
140a22651: 41b132 mov r9b, 0x32
140a22654: e8072ceeff call 0x140905260
140a22659: 488d8d70010000 lea rcx, [rbp + 0x170]
140a22660: 488d55e0 lea rdx, [rbp - 0x20]
140a22664: 41b890010000 mov r8d, 0x190
140a2266a: e89115f001 call 0x142923c00
140a2266f: 80bda602000008 cmp byte ptr [rbp + 0x2a6], 8
140a22676: 742a je 0x140a226a2
140a22678: 488d8d80020000 lea rcx, [rbp + 0x280]
140a2267f: bae8000000 mov edx, 0xe8
140a22684: 480395a8020000 add rdx, qword ptr [rbp + 0x2a8]
140a2268b: 49b894a792304f2c02dc movabs r8, 0xdc022c4f3092a794
140a22695: f30f101deb32f001 movss xmm3, dword ptr [rip + 0x1f032eb]
140a2269d: e88e93f4ff call 0x14096ba30
140a226a2: 488d4de0 lea rcx, [rbp - 0x20]
140a226a6: 488d9570010000 lea rdx, [rbp + 0x170]
140a226ad: 41b890010000 mov r8d, 0x190
140a226b3: e84815f001 call 0x142923c00
140a226b8: 80bd1601000008 cmp byte ptr [rbp + 0x116], 8
140a226bf: 7428 je 0x140a226e9
140a226c1: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a226c8: bae8000000 mov edx, 0xe8
140a226cd: 48039518010000 add rdx, qword ptr [rbp + 0x118]
140a226d4: 49b828ae4f9d5b6cd74a movabs r8, 0x4ad76c5b9d4fae28
140a226de: 41b901000000 mov r9d, 1
140a226e4: e8878ef4ff call 0x14096b570
140a226e9: 488d8d70010000 lea rcx, [rbp + 0x170]
140a226f0: 488d55e0 lea rdx, [rbp - 0x20]
140a226f4: 41b890010000 mov r8d, 0x190
140a226fa: e80115f001 call 0x142923c00
140a226ff: 0f108510020000 movups xmm0, xmmword ptr [rbp + 0x210]
140a22706: 0f298540030000 movaps xmmword ptr [rbp + 0x340], xmm0
140a2270d: 0f108500020000 movups xmm0, xmmword ptr [rbp + 0x200]
140a22714: 0f298530030000 movaps xmmword ptr [rbp + 0x330], xmm0
140a2271b: 0f1085f0010000 movups xmm0, xmmword ptr [rbp + 0x1f0]
140a22722: 0f298520030000 movaps xmmword ptr [rbp + 0x320], xmm0
140a22729: 0f1085e0010000 movups xmm0, xmmword ptr [rbp + 0x1e0]
140a22730: 0f298510030000 movaps xmmword ptr [rbp + 0x310], xmm0
140a22737: 0f1085d0010000 movups xmm0, xmmword ptr [rbp + 0x1d0]
140a2273e: 0f298500030000 movaps xmmword ptr [rbp + 0x300], xmm0
140a22745: 488b85a8020000 mov rax, qword ptr [rbp + 0x2a8]
140a2274c: 488b8df0020000 mov rcx, qword ptr [rbp + 0x2f0]
140a22753: 488b9580010000 mov rdx, qword ptr [rbp + 0x180]
140a2275a: 488955f0 mov qword ptr [rbp - 0x10], rdx
140a2275e: 0f108570010000 movups xmm0, xmmword ptr [rbp + 0x170]
140a22765: 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
140a22769: 0f108588010000 movups xmm0, xmmword ptr [rbp + 0x188]
140a22770: 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
140a22774: 488b9598010000 mov rdx, qword ptr [rbp + 0x198]
140a2277b: 48895520 mov qword ptr [rbp + 0x20], rdx
140a2277f: 0f1085a0010000 movups xmm0, xmmword ptr [rbp + 0x1a0]
140a22786: 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
140a2278a: 488b95b0010000 mov rdx, qword ptr [rbp + 0x1b0]
140a22791: 48895508 mov qword ptr [rbp + 8], rdx
140a22795: 0f108520020000 movups xmm0, xmmword ptr [rbp + 0x220]
140a2279c: 0f108d30020000 movups xmm1, xmmword ptr [rbp + 0x230]
140a227a3: 0f109540020000 movups xmm2, xmmword ptr [rbp + 0x240]
140a227aa: 0f109d50020000 movups xmm3, xmmword ptr [rbp + 0x250]
140a227b1: 0f114558 movups xmmword ptr [rbp + 0x58], xmm0
140a227b5: 0f114d68 movups xmmword ptr [rbp + 0x68], xmm1
140a227b9: 0f115578 movups xmmword ptr [rbp + 0x78], xmm2
140a227bd: 0f119d88000000 movups xmmword ptr [rbp + 0x88], xmm3
140a227c4: 0f108560020000 movups xmm0, xmmword ptr [rbp + 0x260]
140a227cb: 0f118598000000 movups xmmword ptr [rbp + 0x98], xmm0
140a227d2: 0f108570020000 movups xmm0, xmmword ptr [rbp + 0x270]
140a227d9: 0f1185a8000000 movups xmmword ptr [rbp + 0xa8], xmm0
140a227e0: 488b95c8010000 mov rdx, qword ptr [rbp + 0x1c8]
140a227e7: 48895538 mov qword ptr [rbp + 0x38], rdx
140a227eb: 0f1085b8010000 movups xmm0, xmmword ptr [rbp + 0x1b8]
140a227f2: 0f114528 movups xmmword ptr [rbp + 0x28], xmm0
140a227f6: 48898dc0000000 mov qword ptr [rbp + 0xc0], rcx
140a227fd: 488985b8000000 mov qword ptr [rbp + 0xb8], rax
140a22804: 0f297540 movaps xmmword ptr [rbp + 0x40], xmm6
140a22808: 488d8d00030000 lea rcx, [rbp + 0x300]
140a2280f: 488d55e0 lea rdx, [rbp - 0x20]
140a22813: e8b8acfbff call 0x1409dd4d0
140a22818: 80bda602000008 cmp byte ptr [rbp + 0x2a6], 8
140a2281f: 740c je 0x140a2282d
140a22821: 488d8d90020000 lea rcx, [rbp + 0x290]
140a22828: e883fc1b00 call 0x140be24b0
140a2282d: 0f28b560030000 movaps xmm6, xmmword ptr [rbp + 0x360]
140a22834: 4881c4f8030000 add rsp, 0x3f8
140a2283b: 5b pop rbx
140a2283c: 5f pop rdi
140a2283d: 5e pop rsi
140a2283e: 415e pop r14
140a22840: 415f pop r15
140a22842: 5d pop rbp
140a22843: c3 ret 
140a22844: 488d0dc5700a02 lea rcx, [rip + 0x20a70c5]
140a2284b: e8c06aeb01 call 0x1428d9310
140a22850: cc int3 
140a22851: 6666666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a22860: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a22865: 55 push rbp
140a22866: 4157 push r15
140a22868: 4156 push r14
140a2286a: 56 push rsi
140a2286b: 57 push rdi
140a2286c: 53 push rbx
140a2286d: 4883ec48 sub rsp, 0x48
140a22871: 488daa80000000 lea rbp, [rdx + 0x80]
140a22878: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a2287d: 80bda602000008 cmp byte ptr [rbp + 0x2a6], 8
140a22884: 740c je 0x140a22892
140a22886: 488d8d90020000 lea rcx, [rbp + 0x290]
140a2288d: e81efc1b00 call 0x140be24b0
140a22892: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a22897: 4883c448 add rsp, 0x48
140a2289b: 5b pop rbx
140a2289c: 5f pop rdi
140a2289d: 5e pop rsi
140a2289e: 415e pop r14
140a228a0: 415f pop r15
140a228a2: 5d pop rbp
140a228a3: c3 ret 
140a228a4: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a228b0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a228b5: 55 push rbp
140a228b6: 4157 push r15
140a228b8: 4156 push r14
140a228ba: 56 push rsi
140a228bb: 57 push rdi
140a228bc: 53 push rbx
140a228bd: 4883ec48 sub rsp, 0x48
140a228c1: 488daa80000000 lea rbp, [rdx + 0x80]
140a228c8: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a228cd: 488d4de0 lea rcx, [rbp - 0x20]
140a228d1: e8da46e7ff call 0x140896fb0
140a228d6: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a228db: 4883c448 add rsp, 0x48
140a228df: 5b pop rbx
140a228e0: 5f pop rdi
140a228e1: 5e pop rsi
140a228e2: 415e pop r14
140a228e4: 415f pop r15
140a228e6: 5d pop rbp
140a228e7: c3 ret 
140a228e8: 0f1f840000000000 nop dword ptr [rax + rax]
140a228f0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a228f5: 55 push rbp
140a228f6: 4157 push r15
140a228f8: 4156 push r14
140a228fa: 56 push rsi
140a228fb: 57 push rdi
140a228fc: 53 push rbx
140a228fd: 4883ec48 sub rsp, 0x48
140a22901: 488daa80000000 lea rbp, [rdx + 0x80]
140a22908: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a2290d: 488d8d70010000 lea rcx, [rbp + 0x170]
140a22914: e89746e7ff call 0x140896fb0
140a22919: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a2291e: 4883c448 add rsp, 0x48
140a22922: 5b pop rbx
140a22923: 5f pop rdi
140a22924: 5e pop rsi
140a22925: 415e pop r14
140a22927: 415f pop r15
140a22929: 5d pop rbp
140a2292a: c3 ret 
140a2292b: 0f1f440000 nop dword ptr [rax + rax]
140a22930: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a22935: 55 push rbp
140a22936: 4157 push r15
140a22938: 4156 push r14
140a2293a: 56 push rsi
140a2293b: 57 push rdi
140a2293c: 53 push rbx
140a2293d: 4883ec48 sub rsp, 0x48
140a22941: 488daa80000000 lea rbp, [rdx + 0x80]
140a22948: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a2294d: 488d4de0 lea rcx, [rbp - 0x20]
140a22951: e85a46e7ff call 0x140896fb0
140a22956: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a2295b: 4883c448 add rsp, 0x48
140a2295f: 5b pop rbx
140a22960: 5f pop rdi
140a22961: 5e pop rsi
140a22962: 415e pop r14
140a22964: 415f pop r15
140a22966: 5d pop rbp
140a22967: c3 ret 
140a22968: 0f1f840000000000 nop dword ptr [rax + rax]
140a22970: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a22975: 55 push rbp
140a22976: 4157 push r15
140a22978: 4156 push r14
140a2297a: 56 push rsi
140a2297b: 57 push rdi
140a2297c: 53 push rbx
140a2297d: 4883ec48 sub rsp, 0x48
140a22981: 488daa80000000 lea rbp, [rdx + 0x80]
140a22988: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a2298d: 488d8d70010000 lea rcx, [rbp + 0x170]
140a22994: e81746e7ff call 0x140896fb0
140a22999: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a2299e: 4883c448 add rsp, 0x48
140a229a2: 5b pop rbx
140a229a3: 5f pop rdi
140a229a4: 5e pop rsi
140a229a5: 415e pop r14
140a229a7: 415f pop r15
140a229a9: 5d pop rbp
140a229aa: c3 ret 
140a229ab: 0f1f440000 nop dword ptr [rax + rax]
140a229b0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a229b5: 55 push rbp
140a229b6: 4157 push r15
140a229b8: 4156 push r14
140a229ba: 56 push rsi
140a229bb: 57 push rdi
140a229bc: 53 push rbx
140a229bd: 4883ec48 sub rsp, 0x48
140a229c1: 488daa80000000 lea rbp, [rdx + 0x80]
140a229c8: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a229cd: 488d4de0 lea rcx, [rbp - 0x20]
140a229d1: e8da45e7ff call 0x140896fb0
140a229d6: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a229db: 4883c448 add rsp, 0x48
140a229df: 5b pop rbx
140a229e0: 5f pop rdi
140a229e1: 5e pop rsi
140a229e2: 415e pop r14
140a229e4: 415f pop r15
140a229e6: 5d pop rbp
140a229e7: c3 ret 
140a229e8: 0f1f840000000000 nop dword ptr [rax + rax]
140a229f0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a229f5: 55 push rbp
140a229f6: 4157 push r15
140a229f8: 4156 push r14
140a229fa: 56 push rsi
140a229fb: 57 push rdi
140a229fc: 53 push rbx
140a229fd: 4883ec48 sub rsp, 0x48
140a22a01: 488daa80000000 lea rbp, [rdx + 0x80]
140a22a08: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a22a0d: 80bda602000008 cmp byte ptr [rbp + 0x2a6], 8
140a22a14: 740c je 0x140a22a22
140a22a16: 488d8d90020000 lea rcx, [rbp + 0x290]
140a22a1d: e88efa1b00 call 0x140be24b0
140a22a22: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a22a27: 4883c448 add rsp, 0x48
140a22a2b: 5b pop rbx
140a22a2c: 5f pop rdi
140a22a2d: 5e pop rsi
140a22a2e: 415e pop r14
140a22a30: 415f pop r15
140a22a32: 5d pop rbp
140a22a33: c3 ret 
140a22a34: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a22a40: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a22a45: 55 push rbp
140a22a46: 4157 push r15
140a22a48: 4156 push r14
140a22a4a: 56 push rsi
140a22a4b: 57 push rdi
140a22a4c: 53 push rbx
140a22a4d: 4883ec48 sub rsp, 0x48
140a22a51: 488daa80000000 lea rbp, [rdx + 0x80]
140a22a58: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a22a5d: 488d4de0 lea rcx, [rbp - 0x20]
140a22a61: e84a45e7ff call 0x140896fb0
140a22a66: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a22a6b: 4883c448 add rsp, 0x48
140a22a6f: 5b pop rbx
140a22a70: 5f pop rdi
140a22a71: 5e pop rsi
140a22a72: 415e pop r14
140a22a74: 415f pop r15
140a22a76: 5d pop rbp
140a22a77: c3 ret 
140a22a78: 0f1f840000000000 nop dword ptr [rax + rax]
140a22a80: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a22a85: 55 push rbp
140a22a86: 4157 push r15
140a22a88: 4156 push r14
140a22a8a: 56 push rsi
140a22a8b: 57 push rdi
140a22a8c: 53 push rbx
140a22a8d: 4883ec48 sub rsp, 0x48
140a22a91: 488daa80000000 lea rbp, [rdx + 0x80]
140a22a98: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a22a9d: 488d8d70010000 lea rcx, [rbp + 0x170]
140a22aa4: e80745e7ff call 0x140896fb0
140a22aa9: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a22aae: 4883c448 add rsp, 0x48
140a22ab2: 5b pop rbx
140a22ab3: 5f pop rdi
140a22ab4: 5e pop rsi
140a22ab5: 415e pop r14
140a22ab7: 415f pop r15
140a22ab9: 5d pop rbp
140a22aba: c3 ret 
140a22abb: 0f1f440000 nop dword ptr [rax + rax]
140a22ac0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a22ac5: 55 push rbp
140a22ac6: 4157 push r15
140a22ac8: 4156 push r14
140a22aca: 56 push rsi
140a22acb: 57 push rdi
140a22acc: 53 push rbx
140a22acd: 4883ec48 sub rsp, 0x48
140a22ad1: 488daa80000000 lea rbp, [rdx + 0x80]
140a22ad8: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a22add: 488d4de0 lea rcx, [rbp - 0x20]
140a22ae1: e8ca44e7ff call 0x140896fb0
140a22ae6: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a22aeb: 4883c448 add rsp, 0x48
140a22aef: 5b pop rbx
140a22af0: 5f pop rdi
140a22af1: 5e pop rsi
140a22af2: 415e pop r14
140a22af4: 415f pop r15
140a22af6: 5d pop rbp
140a22af7: c3 ret 
140a22af8: 0f1f840000000000 nop dword ptr [rax + rax]
140a22b00: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a22b05: 55 push rbp
140a22b06: 4157 push r15
140a22b08: 4156 push r14
140a22b0a: 56 push rsi
140a22b0b: 57 push rdi
140a22b0c: 53 push rbx
140a22b0d: 4883ec48 sub rsp, 0x48
140a22b11: 488daa80000000 lea rbp, [rdx + 0x80]
140a22b18: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a22b1d: 488d8d70010000 lea rcx, [rbp + 0x170]
140a22b24: e88744e7ff call 0x140896fb0
140a22b29: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a22b2e: 4883c448 add rsp, 0x48
140a22b32: 5b pop rbx
140a22b33: 5f pop rdi
140a22b34: 5e pop rsi
140a22b35: 415e pop r14
140a22b37: 415f pop r15
140a22b39: 5d pop rbp
140a22b3a: c3 ret 
140a22b3b: 0f1f440000 nop dword ptr [rax + rax]
140a22b40: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a22b45: 55 push rbp
140a22b46: 4157 push r15
140a22b48: 4156 push r14
140a22b4a: 56 push rsi
140a22b4b: 57 push rdi
140a22b4c: 53 push rbx
140a22b4d: 4883ec48 sub rsp, 0x48
140a22b51: 488daa80000000 lea rbp, [rdx + 0x80]
140a22b58: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a22b5d: 488d4de0 lea rcx, [rbp - 0x20]
140a22b61: e84a44e7ff call 0x140896fb0
140a22b66: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a22b6b: 4883c448 add rsp, 0x48
140a22b6f: 5b pop rbx
140a22b70: 5f pop rdi
140a22b71: 5e pop rsi
140a22b72: 415e pop r14
140a22b74: 415f pop r15
140a22b76: 5d pop rbp
140a22b77: c3 ret 
