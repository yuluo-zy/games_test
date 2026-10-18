140a12680: 55 push rbp
140a12681: 4157 push r15
140a12683: 4156 push r14
140a12685: 56 push rsi
140a12686: 57 push rdi
140a12687: 53 push rbx
140a12688: 4881ecf8030000 sub rsp, 0x3f8
140a1268f: 488dac2480000000 lea rbp, [rsp + 0x80]
140a12697: 0f29b560030000 movaps xmmword ptr [rbp + 0x360], xmm6
140a1269e: 48c78558030000feffffff mov qword ptr [rbp + 0x358], 0xfffffffffffffffe
140a126a9: 4889d6 mov rsi, rdx
140a126ac: 4889cf mov rdi, rcx
140a126af: 488d4a38 lea rcx, [rdx + 0x38]
140a126b3: 48baeea086629382d11a movabs rdx, 0x1ad182936286a0ee
140a126bd: 49b86f7e12f9b6e304ac movabs r8, 0xac04e3b6f9127e6f
140a126c7: e824b06cff call 0x1400dd6f0
140a126cc: 4885c0 test rax, rax
140a126cf: 0f84450c0000 je 0x140a1331a
140a126d5: 48399618020000 cmp qword ptr [rsi + 0x218], rdx
140a126dc: 0f86380c0000 jbe 0x140a1331a
140a126e2: 488b8610020000 mov rax, qword ptr [rsi + 0x210]
140a126e9: 488b0cd0 mov rcx, qword ptr [rax + rdx*8]
140a126ed: 4885c9 test rcx, rcx
140a126f0: 0f84240c0000 je 0x140a1331a
140a126f6: 488b86e0010000 mov rax, qword ptr [rsi + 0x1e0]
140a126fd: 48f7d1 not rcx
140a12700: 488d0c49 lea rcx, [rcx + rcx*2]
140a12704: 48c1e105 shl rcx, 5
140a12708: 48837c082000 cmp qword ptr [rax + rcx + 0x20], 0
140a1270e: 0f84060c0000 je 0x140a1331a
140a12714: 4801c8 add rax, rcx
140a12717: 488b4810 mov rcx, qword ptr [rax + 0x10]
140a1271b: 8b96b0050000 mov edx, dword ptr [rsi + 0x5b0]
140a12721: 80b99b02000000 cmp byte ptr [rcx + 0x29b], 0
140a12728: 0f84d50b0000 je 0x140a13303
140a1272e: 89505c mov dword ptr [rax + 0x5c], edx
140a12731: c6819b02000000 mov byte ptr [rcx + 0x29b], 0
140a12738: 4881c198000000 add rcx, 0x98
140a1273f: 41b800020000 mov r8d, 0x200
140a12745: 31d2 xor edx, edx
140a12747: e8d414f101 call 0x142923c20
140a1274c: 488d8d00030000 lea rcx, [rbp + 0x300]
140a12753: 48ba0002000000020000 movabs rdx, 0x20000000200
140a1275d: 41b861000000 mov r8d, 0x61
140a12763: e8382b1c00 call 0x140bd52a0
140a12768: 0f108500030000 movups xmm0, xmmword ptr [rbp + 0x300]
140a1276f: 0f108d10030000 movups xmm1, xmmword ptr [rbp + 0x310]
140a12776: 0f298d80010000 movaps xmmword ptr [rbp + 0x180], xmm1
140a1277d: 0f298570010000 movaps xmmword ptr [rbp + 0x170], xmm0
140a12784: 488d4df0 lea rcx, [rbp - 0x10]
140a12788: 488d9570010000 lea rdx, [rbp + 0x170]
140a1278f: e84c451c00 call 0x140bd6ce0
140a12794: 488d052d550b02 lea rax, [rip + 0x20b552d]
140a1279b: 488945e0 mov qword ptr [rbp - 0x20], rax
140a1279f: 48c745e80f000000 mov qword ptr [rbp - 0x18], 0xf
140a127a7: 488d55e0 lea rdx, [rbp - 0x20]
140a127ab: 4889f9 mov rcx, rdi
140a127ae: e81d6d1d00 call 0x140be94d0
140a127b3: 0f108500030000 movups xmm0, xmmword ptr [rbp + 0x300]
140a127ba: 0f108d10030000 movups xmm1, xmmword ptr [rbp + 0x310]
140a127c1: 0f1145c0 movups xmmword ptr [rbp - 0x40], xmm0
140a127c5: 0f114dd0 movups xmmword ptr [rbp - 0x30], xmm1
140a127c9: 8945bc mov dword ptr [rbp - 0x44], eax
140a127cc: 48897c2430 mov qword ptr [rsp + 0x30], rdi
140a127d1: 4889742428 mov qword ptr [rsp + 0x28], rsi
140a127d6: 48c744242018000000 mov qword ptr [rsp + 0x20], 0x18
140a127df: 488d15f1540b02 lea rdx, [rip + 0x20b54f1]
140a127e6: 4c8d0df9540b02 lea r9, [rip + 0x20b54f9]
140a127ed: 488d4de0 lea rcx, [rbp - 0x20]
140a127f1: 41b80f000000 mov r8d, 0xf
140a127f7: e8a40abf00 call 0x1416032a0
140a127fc: 80bd1601000008 cmp byte ptr [rbp + 0x116], 8
140a12803: 7425 je 0x140a1282a
140a12805: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a1280c: bae8000000 mov edx, 0xe8
140a12811: 48039518010000 add rdx, qword ptr [rbp + 0x118]
140a12818: 49b822bc942037f81fd9 movabs r8, 0xd91ff8372094bc22
140a12822: 41b101 mov r9b, 1
140a12825: e82690f5ff call 0x14096b850
140a1282a: 488d9d70010000 lea rbx, [rbp + 0x170]
140a12831: 4c8d75e0 lea r14, [rbp - 0x20]
140a12835: 41b890010000 mov r8d, 0x190
140a1283b: 4889d9 mov rcx, rbx
140a1283e: 4c89f2 mov rdx, r14
140a12841: e8ba13f101 call 0x142923c00
140a12846: c6850003000013 mov byte ptr [rbp + 0x300], 0x13
140a1284d: c745e002000000 mov dword ptr [rbp - 0x20], 2
140a12854: 4c89742420 mov qword ptr [rsp + 0x20], r14
140a12859: 488d8d00030000 lea rcx, [rbp + 0x300]
140a12860: 49b841f0c818cd3cd66a movabs r8, 0x6ad63ccd18c8f041
140a1286a: 4889da mov rdx, rbx
140a1286d: 41b119 mov r9b, 0x19
140a12870: e8eb29efff call 0x140905260
140a12875: 488d5de0 lea rbx, [rbp - 0x20]
140a12879: 4c8db570010000 lea r14, [rbp + 0x170]
140a12880: 41b890010000 mov r8d, 0x190
140a12886: 4889d9 mov rcx, rbx
140a12889: 4c89f2 mov rdx, r14
140a1288c: e86f13f101 call 0x142923c00
140a12891: 4c8d7dbc lea r15, [rbp - 0x44]
140a12895: 4c89bd00030000 mov qword ptr [rbp + 0x300], r15
140a1289c: c7857001000002000000 mov dword ptr [rbp + 0x170], 2
140a128a6: 4c89742420 mov qword ptr [rsp + 0x20], r14
140a128ab: 488d8d00030000 lea rcx, [rbp + 0x300]
140a128b2: 49b868077db5a7c3f7e8 movabs r8, 0xe8f7c3a7b57d0768
140a128bc: 4889da mov rdx, rbx
140a128bf: 41b132 mov r9b, 0x32
140a128c2: e839b4edff call 0x1408edd00
140a128c7: 488d8d70010000 lea rcx, [rbp + 0x170]
140a128ce: 488d55e0 lea rdx, [rbp - 0x20]
140a128d2: 41b890010000 mov r8d, 0x190
140a128d8: e82313f101 call 0x142923c00
140a128dd: 80bda602000008 cmp byte ptr [rbp + 0x2a6], 8
140a128e4: 742a je 0x140a12910
140a128e6: 488d8d80020000 lea rcx, [rbp + 0x280]
140a128ed: bae8000000 mov edx, 0xe8
140a128f2: 480395a8020000 add rdx, qword ptr [rbp + 0x2a8]
140a128f9: 49b894a792304f2c02dc movabs r8, 0xdc022c4f3092a794
140a12903: f30f101d9d1cf201 movss xmm3, dword ptr [rip + 0x1f21c9d]
140a1290b: e82091f5ff call 0x14096ba30
140a12910: 488d4de0 lea rcx, [rbp - 0x20]
140a12914: 488d9570010000 lea rdx, [rbp + 0x170]
140a1291b: 41b890010000 mov r8d, 0x190
140a12921: e8da12f101 call 0x142923c00
140a12926: 80bd1601000008 cmp byte ptr [rbp + 0x116], 8
140a1292d: 7428 je 0x140a12957
140a1292f: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a12936: bae8000000 mov edx, 0xe8
140a1293b: 48039518010000 add rdx, qword ptr [rbp + 0x118]
140a12942: 49b828ae4f9d5b6cd74a movabs r8, 0x4ad76c5b9d4fae28
140a1294c: 41b901000000 mov r9d, 1
140a12952: e8198cf5ff call 0x14096b570
140a12957: 488d8d70010000 lea rcx, [rbp + 0x170]
140a1295e: 488d55e0 lea rdx, [rbp - 0x20]
140a12962: 41b890010000 mov r8d, 0x190
140a12968: e89312f101 call 0x142923c00
140a1296d: 0f108510020000 movups xmm0, xmmword ptr [rbp + 0x210]
140a12974: 0f298540030000 movaps xmmword ptr [rbp + 0x340], xmm0
140a1297b: 0f108500020000 movups xmm0, xmmword ptr [rbp + 0x200]
140a12982: 0f298530030000 movaps xmmword ptr [rbp + 0x330], xmm0
140a12989: 0f1085f0010000 movups xmm0, xmmword ptr [rbp + 0x1f0]
140a12990: 0f298520030000 movaps xmmword ptr [rbp + 0x320], xmm0
140a12997: 0f1085e0010000 movups xmm0, xmmword ptr [rbp + 0x1e0]
140a1299e: 0f298510030000 movaps xmmword ptr [rbp + 0x310], xmm0
140a129a5: 0f1085d0010000 movups xmm0, xmmword ptr [rbp + 0x1d0]
140a129ac: 0f298500030000 movaps xmmword ptr [rbp + 0x300], xmm0
140a129b3: 488b85a8020000 mov rax, qword ptr [rbp + 0x2a8]
140a129ba: 488b8df0020000 mov rcx, qword ptr [rbp + 0x2f0]
140a129c1: 488b9580010000 mov rdx, qword ptr [rbp + 0x180]
140a129c8: 488955f0 mov qword ptr [rbp - 0x10], rdx
140a129cc: 0f108570010000 movups xmm0, xmmword ptr [rbp + 0x170]
140a129d3: 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
140a129d7: 0f108588010000 movups xmm0, xmmword ptr [rbp + 0x188]
140a129de: 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
140a129e2: 488b9598010000 mov rdx, qword ptr [rbp + 0x198]
140a129e9: 48895520 mov qword ptr [rbp + 0x20], rdx
140a129ed: 0f1085a0010000 movups xmm0, xmmword ptr [rbp + 0x1a0]
140a129f4: 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
140a129f8: 488b95b0010000 mov rdx, qword ptr [rbp + 0x1b0]
140a129ff: 48895508 mov qword ptr [rbp + 8], rdx
140a12a03: 0f108520020000 movups xmm0, xmmword ptr [rbp + 0x220]
140a12a0a: 0f108d30020000 movups xmm1, xmmword ptr [rbp + 0x230]
140a12a11: 0f109540020000 movups xmm2, xmmword ptr [rbp + 0x240]
140a12a18: 0f109d50020000 movups xmm3, xmmword ptr [rbp + 0x250]
140a12a1f: 0f114558 movups xmmword ptr [rbp + 0x58], xmm0
140a12a23: 0f114d68 movups xmmword ptr [rbp + 0x68], xmm1
140a12a27: 0f115578 movups xmmword ptr [rbp + 0x78], xmm2
140a12a2b: 0f119d88000000 movups xmmword ptr [rbp + 0x88], xmm3
140a12a32: 0f108560020000 movups xmm0, xmmword ptr [rbp + 0x260]
140a12a39: 0f118598000000 movups xmmword ptr [rbp + 0x98], xmm0
140a12a40: 0f108570020000 movups xmm0, xmmword ptr [rbp + 0x270]
140a12a47: 0f1185a8000000 movups xmmword ptr [rbp + 0xa8], xmm0
140a12a4e: 488b95c8010000 mov rdx, qword ptr [rbp + 0x1c8]
140a12a55: 48895538 mov qword ptr [rbp + 0x38], rdx
140a12a59: 0f1085b8010000 movups xmm0, xmmword ptr [rbp + 0x1b8]
140a12a60: 0f114528 movups xmmword ptr [rbp + 0x28], xmm0
140a12a64: 48898dc0000000 mov qword ptr [rbp + 0xc0], rcx
140a12a6b: 488985b8000000 mov qword ptr [rbp + 0xb8], rax
140a12a72: 0f283587fa0802 movaps xmm6, xmmword ptr [rip + 0x208fa87]
140a12a79: 0f297540 movaps xmmword ptr [rbp + 0x40], xmm6
140a12a7d: 488d8d00030000 lea rcx, [rbp + 0x300]
140a12a84: 488d55e0 lea rdx, [rbp - 0x20]
140a12a88: e843aafcff call 0x1409dd4d0
140a12a8d: 80bda602000008 cmp byte ptr [rbp + 0x2a6], 8
140a12a94: 740c je 0x140a12aa2
140a12a96: 488d8d90020000 lea rcx, [rbp + 0x290]
140a12a9d: e80efa1c00 call 0x140be24b0
140a12aa2: 48897c2430 mov qword ptr [rsp + 0x30], rdi
140a12aa7: 4889742428 mov qword ptr [rsp + 0x28], rsi
140a12aac: 48c744242018000000 mov qword ptr [rsp + 0x20], 0x18
140a12ab5: 488d151b520b02 lea rdx, [rip + 0x20b521b]
140a12abc: 4c8d0d23520b02 lea r9, [rip + 0x20b5223]
140a12ac3: 488d4de0 lea rcx, [rbp - 0x20]
140a12ac7: 41b80f000000 mov r8d, 0xf
140a12acd: e8ce07bf00 call 0x1416032a0
140a12ad2: 80bd1601000008 cmp byte ptr [rbp + 0x116], 8
140a12ad9: 7425 je 0x140a12b00
140a12adb: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a12ae2: bae8000000 mov edx, 0xe8
140a12ae7: 48039518010000 add rdx, qword ptr [rbp + 0x118]
140a12aee: 49b822bc942037f81fd9 movabs r8, 0xd91ff8372094bc22
140a12af8: 4531c9 xor r9d, r9d
140a12afb: e8508df5ff call 0x14096b850
140a12b00: 488d9d70010000 lea rbx, [rbp + 0x170]
140a12b07: 4c8d75e0 lea r14, [rbp - 0x20]
140a12b0b: 41b890010000 mov r8d, 0x190
140a12b11: 4889d9 mov rcx, rbx
140a12b14: 4c89f2 mov rdx, r14
140a12b17: e8e410f101 call 0x142923c00
140a12b1c: 4c89bd00030000 mov qword ptr [rbp + 0x300], r15
140a12b23: c745e002000000 mov dword ptr [rbp - 0x20], 2
140a12b2a: 4c89742420 mov qword ptr [rsp + 0x20], r14
140a12b2f: 488d8d00030000 lea rcx, [rbp + 0x300]
140a12b36: 49b841f0c818cd3cd66a movabs r8, 0x6ad63ccd18c8f041
140a12b40: 4889da mov rdx, rbx
140a12b43: 41b119 mov r9b, 0x19
140a12b46: e8f5b6edff call 0x1408ee240
140a12b4b: 488d5de0 lea rbx, [rbp - 0x20]
140a12b4f: 4c8db570010000 lea r14, [rbp + 0x170]
140a12b56: 41b890010000 mov r8d, 0x190
140a12b5c: 4889d9 mov rcx, rbx
140a12b5f: 4c89f2 mov rdx, r14
140a12b62: e89910f101 call 0x142923c00
140a12b67: c6850003000013 mov byte ptr [rbp + 0x300], 0x13
140a12b6e: c7857001000002000000 mov dword ptr [rbp + 0x170], 2
140a12b78: 4c89742420 mov qword ptr [rsp + 0x20], r14
140a12b7d: 488d8d00030000 lea rcx, [rbp + 0x300]
140a12b84: 49b868077db5a7c3f7e8 movabs r8, 0xe8f7c3a7b57d0768
140a12b8e: 4889da mov rdx, rbx
140a12b91: 41b132 mov r9b, 0x32
140a12b94: e8c726efff call 0x140905260
140a12b99: 488d8d70010000 lea rcx, [rbp + 0x170]
140a12ba0: 488d55e0 lea rdx, [rbp - 0x20]
140a12ba4: 41b890010000 mov r8d, 0x190
140a12baa: e85110f101 call 0x142923c00
140a12baf: 80bda602000008 cmp byte ptr [rbp + 0x2a6], 8
140a12bb6: 742a je 0x140a12be2
140a12bb8: 488d8d80020000 lea rcx, [rbp + 0x280]
140a12bbf: bae8000000 mov edx, 0xe8
140a12bc4: 480395a8020000 add rdx, qword ptr [rbp + 0x2a8]
140a12bcb: 49b894a792304f2c02dc movabs r8, 0xdc022c4f3092a794
140a12bd5: f30f101dcb19f201 movss xmm3, dword ptr [rip + 0x1f219cb]
140a12bdd: e84e8ef5ff call 0x14096ba30
140a12be2: 488d4de0 lea rcx, [rbp - 0x20]
140a12be6: 488d9570010000 lea rdx, [rbp + 0x170]
140a12bed: 41b890010000 mov r8d, 0x190
140a12bf3: e80810f101 call 0x142923c00
140a12bf8: 80bd1601000008 cmp byte ptr [rbp + 0x116], 8
140a12bff: 7428 je 0x140a12c29
140a12c01: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a12c08: bae8000000 mov edx, 0xe8
140a12c0d: 48039518010000 add rdx, qword ptr [rbp + 0x118]
140a12c14: 49b828ae4f9d5b6cd74a movabs r8, 0x4ad76c5b9d4fae28
140a12c1e: 41b901000000 mov r9d, 1
140a12c24: e84789f5ff call 0x14096b570
140a12c29: 488d8d70010000 lea rcx, [rbp + 0x170]
140a12c30: 488d55e0 lea rdx, [rbp - 0x20]
140a12c34: 41b890010000 mov r8d, 0x190
140a12c3a: e8c10ff101 call 0x142923c00
140a12c3f: 0f108510020000 movups xmm0, xmmword ptr [rbp + 0x210]
140a12c46: 0f298540030000 movaps xmmword ptr [rbp + 0x340], xmm0
140a12c4d: 0f108500020000 movups xmm0, xmmword ptr [rbp + 0x200]
140a12c54: 0f298530030000 movaps xmmword ptr [rbp + 0x330], xmm0
140a12c5b: 0f1085f0010000 movups xmm0, xmmword ptr [rbp + 0x1f0]
140a12c62: 0f298520030000 movaps xmmword ptr [rbp + 0x320], xmm0
140a12c69: 0f1085e0010000 movups xmm0, xmmword ptr [rbp + 0x1e0]
140a12c70: 0f298510030000 movaps xmmword ptr [rbp + 0x310], xmm0
140a12c77: 0f1085d0010000 movups xmm0, xmmword ptr [rbp + 0x1d0]
140a12c7e: 0f298500030000 movaps xmmword ptr [rbp + 0x300], xmm0
140a12c85: 488b85a8020000 mov rax, qword ptr [rbp + 0x2a8]
140a12c8c: 488b8df0020000 mov rcx, qword ptr [rbp + 0x2f0]
140a12c93: 488b9580010000 mov rdx, qword ptr [rbp + 0x180]
140a12c9a: 488955f0 mov qword ptr [rbp - 0x10], rdx
140a12c9e: 0f108570010000 movups xmm0, xmmword ptr [rbp + 0x170]
140a12ca5: 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
140a12ca9: 0f108588010000 movups xmm0, xmmword ptr [rbp + 0x188]
140a12cb0: 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
140a12cb4: 488b9598010000 mov rdx, qword ptr [rbp + 0x198]
140a12cbb: 48895520 mov qword ptr [rbp + 0x20], rdx
140a12cbf: 0f1085a0010000 movups xmm0, xmmword ptr [rbp + 0x1a0]
140a12cc6: 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
140a12cca: 488b95b0010000 mov rdx, qword ptr [rbp + 0x1b0]
140a12cd1: 48895508 mov qword ptr [rbp + 8], rdx
140a12cd5: 0f108520020000 movups xmm0, xmmword ptr [rbp + 0x220]
140a12cdc: 0f108d30020000 movups xmm1, xmmword ptr [rbp + 0x230]
140a12ce3: 0f109540020000 movups xmm2, xmmword ptr [rbp + 0x240]
140a12cea: 0f109d50020000 movups xmm3, xmmword ptr [rbp + 0x250]
140a12cf1: 0f114558 movups xmmword ptr [rbp + 0x58], xmm0
140a12cf5: 0f114d68 movups xmmword ptr [rbp + 0x68], xmm1
140a12cf9: 0f115578 movups xmmword ptr [rbp + 0x78], xmm2
140a12cfd: 0f119d88000000 movups xmmword ptr [rbp + 0x88], xmm3
140a12d04: 0f108560020000 movups xmm0, xmmword ptr [rbp + 0x260]
140a12d0b: 0f118598000000 movups xmmword ptr [rbp + 0x98], xmm0
140a12d12: 0f108570020000 movups xmm0, xmmword ptr [rbp + 0x270]
140a12d19: 0f1185a8000000 movups xmmword ptr [rbp + 0xa8], xmm0
140a12d20: 488b95c8010000 mov rdx, qword ptr [rbp + 0x1c8]
140a12d27: 48895538 mov qword ptr [rbp + 0x38], rdx
140a12d2b: 0f1085b8010000 movups xmm0, xmmword ptr [rbp + 0x1b8]
140a12d32: 0f114528 movups xmmword ptr [rbp + 0x28], xmm0
140a12d36: 48898dc0000000 mov qword ptr [rbp + 0xc0], rcx
140a12d3d: 488985b8000000 mov qword ptr [rbp + 0xb8], rax
140a12d44: 0f297540 movaps xmmword ptr [rbp + 0x40], xmm6
140a12d48: 488d8d00030000 lea rcx, [rbp + 0x300]
140a12d4f: 488d55e0 lea rdx, [rbp - 0x20]
140a12d53: e878a7fcff call 0x1409dd4d0
140a12d58: 80bda602000008 cmp byte ptr [rbp + 0x2a6], 8
140a12d5f: 740c je 0x140a12d6d
140a12d61: 488d8d90020000 lea rcx, [rbp + 0x290]
140a12d68: e843f71c00 call 0x140be24b0
140a12d6d: 48897c2430 mov qword ptr [rsp + 0x30], rdi
140a12d72: 4889742428 mov qword ptr [rsp + 0x28], rsi
140a12d77: 48c744242018000000 mov qword ptr [rsp + 0x20], 0x18
140a12d80: 488d15504f0b02 lea rdx, [rip + 0x20b4f50]
140a12d87: 4c8d0d584f0b02 lea r9, [rip + 0x20b4f58]
140a12d8e: 488d4de0 lea rcx, [rbp - 0x20]
140a12d92: 41b80f000000 mov r8d, 0xf
140a12d98: e80305bf00 call 0x1416032a0
140a12d9d: 80bd1601000008 cmp byte ptr [rbp + 0x116], 8
140a12da4: 7425 je 0x140a12dcb
140a12da6: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a12dad: bae8000000 mov edx, 0xe8
140a12db2: 48039518010000 add rdx, qword ptr [rbp + 0x118]
140a12db9: 49b822bc942037f81fd9 movabs r8, 0xd91ff8372094bc22
140a12dc3: 41b101 mov r9b, 1
140a12dc6: e8858af5ff call 0x14096b850
140a12dcb: 488d9d70010000 lea rbx, [rbp + 0x170]
140a12dd2: 4c8d75e0 lea r14, [rbp - 0x20]
140a12dd6: 41b890010000 mov r8d, 0x190
140a12ddc: 4889d9 mov rcx, rbx
140a12ddf: 4c89f2 mov rdx, r14
140a12de2: e8190ef101 call 0x142923c00
140a12de7: c6850003000013 mov byte ptr [rbp + 0x300], 0x13
140a12dee: c745e002000000 mov dword ptr [rbp - 0x20], 2
140a12df5: 4c89742420 mov qword ptr [rsp + 0x20], r14
140a12dfa: 488d8d00030000 lea rcx, [rbp + 0x300]
140a12e01: 49b841f0c818cd3cd66a movabs r8, 0x6ad63ccd18c8f041
140a12e0b: 4889da mov rdx, rbx
140a12e0e: 41b119 mov r9b, 0x19
140a12e11: e84a24efff call 0x140905260
140a12e16: 488d5de0 lea rbx, [rbp - 0x20]
140a12e1a: 4c8db570010000 lea r14, [rbp + 0x170]
140a12e21: 41b890010000 mov r8d, 0x190
140a12e27: 4889d9 mov rcx, rbx
140a12e2a: 4c89f2 mov rdx, r14
140a12e2d: e8ce0df101 call 0x142923c00
140a12e32: 4c89bd00030000 mov qword ptr [rbp + 0x300], r15
140a12e39: c7857001000002000000 mov dword ptr [rbp + 0x170], 2
140a12e43: 4c89742420 mov qword ptr [rsp + 0x20], r14
140a12e48: 488d8d00030000 lea rcx, [rbp + 0x300]
140a12e4f: 49b868077db5a7c3f7e8 movabs r8, 0xe8f7c3a7b57d0768
140a12e59: 4889da mov rdx, rbx
140a12e5c: 41b132 mov r9b, 0x32
140a12e5f: e89caeedff call 0x1408edd00
140a12e64: 488d8d70010000 lea rcx, [rbp + 0x170]
140a12e6b: 488d55e0 lea rdx, [rbp - 0x20]
140a12e6f: 41b890010000 mov r8d, 0x190
140a12e75: e8860df101 call 0x142923c00
140a12e7a: 80bda602000008 cmp byte ptr [rbp + 0x2a6], 8
140a12e81: 742a je 0x140a12ead
140a12e83: 488d8d80020000 lea rcx, [rbp + 0x280]
140a12e8a: bae8000000 mov edx, 0xe8
140a12e8f: 480395a8020000 add rdx, qword ptr [rbp + 0x2a8]
140a12e96: 49b894a792304f2c02dc movabs r8, 0xdc022c4f3092a794
140a12ea0: f30f101d8ca1fb01 movss xmm3, dword ptr [rip + 0x1fba18c]
140a12ea8: e8838bf5ff call 0x14096ba30
140a12ead: 488d4de0 lea rcx, [rbp - 0x20]
140a12eb1: 488d9570010000 lea rdx, [rbp + 0x170]
140a12eb8: 41b890010000 mov r8d, 0x190
140a12ebe: e83d0df101 call 0x142923c00
140a12ec3: 80bd1601000008 cmp byte ptr [rbp + 0x116], 8
140a12eca: 7428 je 0x140a12ef4
140a12ecc: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a12ed3: bae8000000 mov edx, 0xe8
140a12ed8: 48039518010000 add rdx, qword ptr [rbp + 0x118]
140a12edf: 49b828ae4f9d5b6cd74a movabs r8, 0x4ad76c5b9d4fae28
140a12ee9: 41b902000000 mov r9d, 2
140a12eef: e87c86f5ff call 0x14096b570
140a12ef4: 488d8d70010000 lea rcx, [rbp + 0x170]
140a12efb: 488d55e0 lea rdx, [rbp - 0x20]
140a12eff: 41b890010000 mov r8d, 0x190
140a12f05: e8f60cf101 call 0x142923c00
140a12f0a: 0f108510020000 movups xmm0, xmmword ptr [rbp + 0x210]
140a12f11: 0f298540030000 movaps xmmword ptr [rbp + 0x340], xmm0
140a12f18: 0f108500020000 movups xmm0, xmmword ptr [rbp + 0x200]
140a12f1f: 0f298530030000 movaps xmmword ptr [rbp + 0x330], xmm0
140a12f26: 0f1085f0010000 movups xmm0, xmmword ptr [rbp + 0x1f0]
140a12f2d: 0f298520030000 movaps xmmword ptr [rbp + 0x320], xmm0
140a12f34: 0f1085e0010000 movups xmm0, xmmword ptr [rbp + 0x1e0]
140a12f3b: 0f298510030000 movaps xmmword ptr [rbp + 0x310], xmm0
140a12f42: 0f1085d0010000 movups xmm0, xmmword ptr [rbp + 0x1d0]
140a12f49: 0f298500030000 movaps xmmword ptr [rbp + 0x300], xmm0
140a12f50: 488b85a8020000 mov rax, qword ptr [rbp + 0x2a8]
140a12f57: 488b8df0020000 mov rcx, qword ptr [rbp + 0x2f0]
140a12f5e: 488b9580010000 mov rdx, qword ptr [rbp + 0x180]
140a12f65: 488955f0 mov qword ptr [rbp - 0x10], rdx
140a12f69: 0f108570010000 movups xmm0, xmmword ptr [rbp + 0x170]
140a12f70: 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
140a12f74: 0f108588010000 movups xmm0, xmmword ptr [rbp + 0x188]
140a12f7b: 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
140a12f7f: 488b9598010000 mov rdx, qword ptr [rbp + 0x198]
140a12f86: 48895520 mov qword ptr [rbp + 0x20], rdx
140a12f8a: 0f1085a0010000 movups xmm0, xmmword ptr [rbp + 0x1a0]
140a12f91: 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
140a12f95: 488b95b0010000 mov rdx, qword ptr [rbp + 0x1b0]
140a12f9c: 48895508 mov qword ptr [rbp + 8], rdx
140a12fa0: 0f108520020000 movups xmm0, xmmword ptr [rbp + 0x220]
140a12fa7: 0f108d30020000 movups xmm1, xmmword ptr [rbp + 0x230]
140a12fae: 0f109540020000 movups xmm2, xmmword ptr [rbp + 0x240]
140a12fb5: 0f109d50020000 movups xmm3, xmmword ptr [rbp + 0x250]
140a12fbc: 0f114558 movups xmmword ptr [rbp + 0x58], xmm0
140a12fc0: 0f114d68 movups xmmword ptr [rbp + 0x68], xmm1
140a12fc4: 0f115578 movups xmmword ptr [rbp + 0x78], xmm2
140a12fc8: 0f119d88000000 movups xmmword ptr [rbp + 0x88], xmm3
140a12fcf: 0f108560020000 movups xmm0, xmmword ptr [rbp + 0x260]
140a12fd6: 0f118598000000 movups xmmword ptr [rbp + 0x98], xmm0
140a12fdd: 0f108570020000 movups xmm0, xmmword ptr [rbp + 0x270]
140a12fe4: 0f1185a8000000 movups xmmword ptr [rbp + 0xa8], xmm0
140a12feb: 488b95c8010000 mov rdx, qword ptr [rbp + 0x1c8]
140a12ff2: 48895538 mov qword ptr [rbp + 0x38], rdx
140a12ff6: 0f1085b8010000 movups xmm0, xmmword ptr [rbp + 0x1b8]
140a12ffd: 0f114528 movups xmmword ptr [rbp + 0x28], xmm0
140a13001: 48898dc0000000 mov qword ptr [rbp + 0xc0], rcx
140a13008: 488985b8000000 mov qword ptr [rbp + 0xb8], rax
140a1300f: 0f297540 movaps xmmword ptr [rbp + 0x40], xmm6
140a13013: 488d8d00030000 lea rcx, [rbp + 0x300]
140a1301a: 488d55e0 lea rdx, [rbp - 0x20]
140a1301e: e8ada4fcff call 0x1409dd4d0
140a13023: 80bda602000008 cmp byte ptr [rbp + 0x2a6], 8
140a1302a: 740c je 0x140a13038
140a1302c: 488d8d90020000 lea rcx, [rbp + 0x290]
140a13033: e878f41c00 call 0x140be24b0
140a13038: 48897c2430 mov qword ptr [rsp + 0x30], rdi
140a1303d: 4889742428 mov qword ptr [rsp + 0x28], rsi
140a13042: 48c744242018000000 mov qword ptr [rsp + 0x20], 0x18
140a1304b: 488d15854c0b02 lea rdx, [rip + 0x20b4c85]
140a13052: 4c8d0d8d4c0b02 lea r9, [rip + 0x20b4c8d]
140a13059: 488d4de0 lea rcx, [rbp - 0x20]
140a1305d: 41b80f000000 mov r8d, 0xf
140a13063: e83802bf00 call 0x1416032a0
140a13068: 80bd1601000008 cmp byte ptr [rbp + 0x116], 8
140a1306f: 7425 je 0x140a13096
140a13071: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a13078: bae8000000 mov edx, 0xe8
140a1307d: 48039518010000 add rdx, qword ptr [rbp + 0x118]
140a13084: 49b822bc942037f81fd9 movabs r8, 0xd91ff8372094bc22
140a1308e: 4531c9 xor r9d, r9d
140a13091: e8ba87f5ff call 0x14096b850
140a13096: 488db570010000 lea rsi, [rbp + 0x170]
140a1309d: 488d7de0 lea rdi, [rbp - 0x20]
140a130a1: 41b890010000 mov r8d, 0x190
140a130a7: 4889f1 mov rcx, rsi
140a130aa: 4889fa mov rdx, rdi
140a130ad: e84e0bf101 call 0x142923c00
140a130b2: 4c89bd00030000 mov qword ptr [rbp + 0x300], r15
140a130b9: c745e002000000 mov dword ptr [rbp - 0x20], 2
140a130c0: 48897c2420 mov qword ptr [rsp + 0x20], rdi
140a130c5: 488d8d00030000 lea rcx, [rbp + 0x300]
140a130cc: 49b841f0c818cd3cd66a movabs r8, 0x6ad63ccd18c8f041
140a130d6: 4889f2 mov rdx, rsi
140a130d9: 41b119 mov r9b, 0x19
140a130dc: e85fb1edff call 0x1408ee240
140a130e1: 488d75e0 lea rsi, [rbp - 0x20]
140a130e5: 488dbd70010000 lea rdi, [rbp + 0x170]
140a130ec: 41b890010000 mov r8d, 0x190
140a130f2: 4889f1 mov rcx, rsi
140a130f5: 4889fa mov rdx, rdi
140a130f8: e8030bf101 call 0x142923c00
140a130fd: c6850003000013 mov byte ptr [rbp + 0x300], 0x13
140a13104: c7857001000002000000 mov dword ptr [rbp + 0x170], 2
140a1310e: 48897c2420 mov qword ptr [rsp + 0x20], rdi
140a13113: 488d8d00030000 lea rcx, [rbp + 0x300]
140a1311a: 49b868077db5a7c3f7e8 movabs r8, 0xe8f7c3a7b57d0768
140a13124: 4889f2 mov rdx, rsi
140a13127: 41b132 mov r9b, 0x32
140a1312a: e83121efff call 0x140905260
140a1312f: 488d8d70010000 lea rcx, [rbp + 0x170]
140a13136: 488d55e0 lea rdx, [rbp - 0x20]
140a1313a: 41b890010000 mov r8d, 0x190
140a13140: e8bb0af101 call 0x142923c00
140a13145: 80bda602000008 cmp byte ptr [rbp + 0x2a6], 8
140a1314c: 742a je 0x140a13178
140a1314e: 488d8d80020000 lea rcx, [rbp + 0x280]
140a13155: bae8000000 mov edx, 0xe8
140a1315a: 480395a8020000 add rdx, qword ptr [rbp + 0x2a8]
140a13161: 49b894a792304f2c02dc movabs r8, 0xdc022c4f3092a794
140a1316b: f30f101dc19efb01 movss xmm3, dword ptr [rip + 0x1fb9ec1]
140a13173: e8b888f5ff call 0x14096ba30
140a13178: 488d4de0 lea rcx, [rbp - 0x20]
140a1317c: 488d9570010000 lea rdx, [rbp + 0x170]
140a13183: 41b890010000 mov r8d, 0x190
140a13189: e8720af101 call 0x142923c00
140a1318e: 80bd1601000008 cmp byte ptr [rbp + 0x116], 8
140a13195: 7428 je 0x140a131bf
140a13197: 488d8df0000000 lea rcx, [rbp + 0xf0]
140a1319e: bae8000000 mov edx, 0xe8
140a131a3: 48039518010000 add rdx, qword ptr [rbp + 0x118]
140a131aa: 49b828ae4f9d5b6cd74a movabs r8, 0x4ad76c5b9d4fae28
140a131b4: 41b902000000 mov r9d, 2
140a131ba: e8b183f5ff call 0x14096b570
140a131bf: 488d8d70010000 lea rcx, [rbp + 0x170]
140a131c6: 488d55e0 lea rdx, [rbp - 0x20]
140a131ca: 41b890010000 mov r8d, 0x190
140a131d0: e82b0af101 call 0x142923c00
140a131d5: 0f108510020000 movups xmm0, xmmword ptr [rbp + 0x210]
140a131dc: 0f298540030000 movaps xmmword ptr [rbp + 0x340], xmm0
140a131e3: 0f108500020000 movups xmm0, xmmword ptr [rbp + 0x200]
140a131ea: 0f298530030000 movaps xmmword ptr [rbp + 0x330], xmm0
140a131f1: 0f1085f0010000 movups xmm0, xmmword ptr [rbp + 0x1f0]
140a131f8: 0f298520030000 movaps xmmword ptr [rbp + 0x320], xmm0
140a131ff: 0f1085e0010000 movups xmm0, xmmword ptr [rbp + 0x1e0]
140a13206: 0f298510030000 movaps xmmword ptr [rbp + 0x310], xmm0
140a1320d: 0f1085d0010000 movups xmm0, xmmword ptr [rbp + 0x1d0]
140a13214: 0f298500030000 movaps xmmword ptr [rbp + 0x300], xmm0
140a1321b: 488b85a8020000 mov rax, qword ptr [rbp + 0x2a8]
140a13222: 488b8df0020000 mov rcx, qword ptr [rbp + 0x2f0]
140a13229: 488b9580010000 mov rdx, qword ptr [rbp + 0x180]
140a13230: 488955f0 mov qword ptr [rbp - 0x10], rdx
140a13234: 0f108570010000 movups xmm0, xmmword ptr [rbp + 0x170]
140a1323b: 0f2945e0 movaps xmmword ptr [rbp - 0x20], xmm0
140a1323f: 0f108588010000 movups xmm0, xmmword ptr [rbp + 0x188]
140a13246: 0f294510 movaps xmmword ptr [rbp + 0x10], xmm0
140a1324a: 488b9598010000 mov rdx, qword ptr [rbp + 0x198]
140a13251: 48895520 mov qword ptr [rbp + 0x20], rdx
140a13255: 0f1085a0010000 movups xmm0, xmmword ptr [rbp + 0x1a0]
140a1325c: 0f1145f8 movups xmmword ptr [rbp - 8], xmm0
140a13260: 488b95b0010000 mov rdx, qword ptr [rbp + 0x1b0]
140a13267: 48895508 mov qword ptr [rbp + 8], rdx
140a1326b: 0f108520020000 movups xmm0, xmmword ptr [rbp + 0x220]
140a13272: 0f108d30020000 movups xmm1, xmmword ptr [rbp + 0x230]
140a13279: 0f109540020000 movups xmm2, xmmword ptr [rbp + 0x240]
140a13280: 0f109d50020000 movups xmm3, xmmword ptr [rbp + 0x250]
140a13287: 0f114558 movups xmmword ptr [rbp + 0x58], xmm0
140a1328b: 0f114d68 movups xmmword ptr [rbp + 0x68], xmm1
140a1328f: 0f115578 movups xmmword ptr [rbp + 0x78], xmm2
140a13293: 0f119d88000000 movups xmmword ptr [rbp + 0x88], xmm3
140a1329a: 0f108560020000 movups xmm0, xmmword ptr [rbp + 0x260]
140a132a1: 0f118598000000 movups xmmword ptr [rbp + 0x98], xmm0
140a132a8: 0f108570020000 movups xmm0, xmmword ptr [rbp + 0x270]
140a132af: 0f1185a8000000 movups xmmword ptr [rbp + 0xa8], xmm0
140a132b6: 488b95c8010000 mov rdx, qword ptr [rbp + 0x1c8]
140a132bd: 48895538 mov qword ptr [rbp + 0x38], rdx
140a132c1: 0f1085b8010000 movups xmm0, xmmword ptr [rbp + 0x1b8]
140a132c8: 0f114528 movups xmmword ptr [rbp + 0x28], xmm0
140a132cc: 48898dc0000000 mov qword ptr [rbp + 0xc0], rcx
140a132d3: 488985b8000000 mov qword ptr [rbp + 0xb8], rax
140a132da: 0f297540 movaps xmmword ptr [rbp + 0x40], xmm6
140a132de: 488d8d00030000 lea rcx, [rbp + 0x300]
140a132e5: 488d55e0 lea rdx, [rbp - 0x20]
140a132e9: e8e2a1fcff call 0x1409dd4d0
140a132ee: 80bda602000008 cmp byte ptr [rbp + 0x2a6], 8
140a132f5: 740c je 0x140a13303
140a132f7: 488d8d90020000 lea rcx, [rbp + 0x290]
140a132fe: e8adf11c00 call 0x140be24b0
140a13303: 0f28b560030000 movaps xmm6, xmmword ptr [rbp + 0x360]
140a1330a: 4881c4f8030000 add rsp, 0x3f8
140a13311: 5b pop rbx
140a13312: 5f pop rdi
140a13313: 5e pop rsi
140a13314: 415e pop r14
140a13316: 415f pop r15
140a13318: 5d pop rbp
140a13319: c3 ret 
140a1331a: 488d0d8f490b02 lea rcx, [rip + 0x20b498f]
140a13321: e8ea5fec01 call 0x1428d9310
140a13326: cc int3 
140a13327: 660f1f840000000000 nop word ptr [rax + rax]
140a13330: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a13335: 55 push rbp
140a13336: 4157 push r15
140a13338: 4156 push r14
140a1333a: 56 push rsi
140a1333b: 57 push rdi
140a1333c: 53 push rbx
140a1333d: 4883ec48 sub rsp, 0x48
140a13341: 488daa80000000 lea rbp, [rdx + 0x80]
140a13348: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a1334d: 80bda602000008 cmp byte ptr [rbp + 0x2a6], 8
140a13354: 740c je 0x140a13362
140a13356: 488d8d90020000 lea rcx, [rbp + 0x290]
140a1335d: e84ef11c00 call 0x140be24b0
140a13362: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a13367: 4883c448 add rsp, 0x48
140a1336b: 5b pop rbx
140a1336c: 5f pop rdi
140a1336d: 5e pop rsi
140a1336e: 415e pop r14
140a13370: 415f pop r15
140a13372: 5d pop rbp
140a13373: c3 ret 
140a13374: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a13380: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a13385: 55 push rbp
140a13386: 4157 push r15
140a13388: 4156 push r14
140a1338a: 56 push rsi
140a1338b: 57 push rdi
140a1338c: 53 push rbx
140a1338d: 4883ec48 sub rsp, 0x48
140a13391: 488daa80000000 lea rbp, [rdx + 0x80]
140a13398: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a1339d: 488d4de0 lea rcx, [rbp - 0x20]
140a133a1: e80a3ce8ff call 0x140896fb0
140a133a6: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a133ab: 4883c448 add rsp, 0x48
140a133af: 5b pop rbx
140a133b0: 5f pop rdi
140a133b1: 5e pop rsi
140a133b2: 415e pop r14
140a133b4: 415f pop r15
140a133b6: 5d pop rbp
140a133b7: c3 ret 
140a133b8: 0f1f840000000000 nop dword ptr [rax + rax]
140a133c0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a133c5: 55 push rbp
140a133c6: 4157 push r15
140a133c8: 4156 push r14
140a133ca: 56 push rsi
140a133cb: 57 push rdi
140a133cc: 53 push rbx
140a133cd: 4883ec48 sub rsp, 0x48
140a133d1: 488daa80000000 lea rbp, [rdx + 0x80]
140a133d8: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a133dd: 488d8d70010000 lea rcx, [rbp + 0x170]
140a133e4: e8c73be8ff call 0x140896fb0
140a133e9: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a133ee: 4883c448 add rsp, 0x48
140a133f2: 5b pop rbx
140a133f3: 5f pop rdi
140a133f4: 5e pop rsi
140a133f5: 415e pop r14
140a133f7: 415f pop r15
140a133f9: 5d pop rbp
140a133fa: c3 ret 
140a133fb: 0f1f440000 nop dword ptr [rax + rax]
140a13400: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a13405: 55 push rbp
140a13406: 4157 push r15
140a13408: 4156 push r14
140a1340a: 56 push rsi
140a1340b: 57 push rdi
140a1340c: 53 push rbx
140a1340d: 4883ec48 sub rsp, 0x48
140a13411: 488daa80000000 lea rbp, [rdx + 0x80]
140a13418: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a1341d: 488d4de0 lea rcx, [rbp - 0x20]
140a13421: e88a3be8ff call 0x140896fb0
140a13426: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a1342b: 4883c448 add rsp, 0x48
140a1342f: 5b pop rbx
140a13430: 5f pop rdi
140a13431: 5e pop rsi
140a13432: 415e pop r14
140a13434: 415f pop r15
140a13436: 5d pop rbp
140a13437: c3 ret 
140a13438: 0f1f840000000000 nop dword ptr [rax + rax]
140a13440: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a13445: 55 push rbp
140a13446: 4157 push r15
140a13448: 4156 push r14
140a1344a: 56 push rsi
140a1344b: 57 push rdi
140a1344c: 53 push rbx
140a1344d: 4883ec48 sub rsp, 0x48
140a13451: 488daa80000000 lea rbp, [rdx + 0x80]
140a13458: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a1345d: 488d8d70010000 lea rcx, [rbp + 0x170]
140a13464: e8473be8ff call 0x140896fb0
140a13469: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a1346e: 4883c448 add rsp, 0x48
140a13472: 5b pop rbx
140a13473: 5f pop rdi
140a13474: 5e pop rsi
140a13475: 415e pop r14
140a13477: 415f pop r15
140a13479: 5d pop rbp
140a1347a: c3 ret 
140a1347b: 0f1f440000 nop dword ptr [rax + rax]
140a13480: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a13485: 55 push rbp
140a13486: 4157 push r15
140a13488: 4156 push r14
140a1348a: 56 push rsi
140a1348b: 57 push rdi
140a1348c: 53 push rbx
140a1348d: 4883ec48 sub rsp, 0x48
140a13491: 488daa80000000 lea rbp, [rdx + 0x80]
140a13498: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a1349d: 488d4de0 lea rcx, [rbp - 0x20]
140a134a1: e80a3be8ff call 0x140896fb0
140a134a6: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a134ab: 4883c448 add rsp, 0x48
140a134af: 5b pop rbx
140a134b0: 5f pop rdi
140a134b1: 5e pop rsi
140a134b2: 415e pop r14
140a134b4: 415f pop r15
140a134b6: 5d pop rbp
140a134b7: c3 ret 
140a134b8: 0f1f840000000000 nop dword ptr [rax + rax]
140a134c0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a134c5: 55 push rbp
140a134c6: 4157 push r15
140a134c8: 4156 push r14
140a134ca: 56 push rsi
140a134cb: 57 push rdi
140a134cc: 53 push rbx
140a134cd: 4883ec48 sub rsp, 0x48
140a134d1: 488daa80000000 lea rbp, [rdx + 0x80]
140a134d8: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a134dd: 80bda602000008 cmp byte ptr [rbp + 0x2a6], 8
140a134e4: 740c je 0x140a134f2
140a134e6: 488d8d90020000 lea rcx, [rbp + 0x290]
140a134ed: e8beef1c00 call 0x140be24b0
140a134f2: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a134f7: 4883c448 add rsp, 0x48
140a134fb: 5b pop rbx
140a134fc: 5f pop rdi
140a134fd: 5e pop rsi
140a134fe: 415e pop r14
140a13500: 415f pop r15
140a13502: 5d pop rbp
140a13503: c3 ret 
140a13504: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a13510: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a13515: 55 push rbp
140a13516: 4157 push r15
140a13518: 4156 push r14
140a1351a: 56 push rsi
140a1351b: 57 push rdi
140a1351c: 53 push rbx
140a1351d: 4883ec48 sub rsp, 0x48
140a13521: 488daa80000000 lea rbp, [rdx + 0x80]
140a13528: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a1352d: 488d4de0 lea rcx, [rbp - 0x20]
140a13531: e87a3ae8ff call 0x140896fb0
140a13536: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a1353b: 4883c448 add rsp, 0x48
140a1353f: 5b pop rbx
140a13540: 5f pop rdi
140a13541: 5e pop rsi
140a13542: 415e pop r14
140a13544: 415f pop r15
140a13546: 5d pop rbp
140a13547: c3 ret 
140a13548: 0f1f840000000000 nop dword ptr [rax + rax]
140a13550: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a13555: 55 push rbp
140a13556: 4157 push r15
140a13558: 4156 push r14
140a1355a: 56 push rsi
140a1355b: 57 push rdi
140a1355c: 53 push rbx
140a1355d: 4883ec48 sub rsp, 0x48
140a13561: 488daa80000000 lea rbp, [rdx + 0x80]
140a13568: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a1356d: 488d8d70010000 lea rcx, [rbp + 0x170]
140a13574: e8373ae8ff call 0x140896fb0
140a13579: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a1357e: 4883c448 add rsp, 0x48
140a13582: 5b pop rbx
140a13583: 5f pop rdi
140a13584: 5e pop rsi
140a13585: 415e pop r14
140a13587: 415f pop r15
140a13589: 5d pop rbp
140a1358a: c3 ret 
140a1358b: 0f1f440000 nop dword ptr [rax + rax]
140a13590: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a13595: 55 push rbp
140a13596: 4157 push r15
140a13598: 4156 push r14
140a1359a: 56 push rsi
140a1359b: 57 push rdi
140a1359c: 53 push rbx
140a1359d: 4883ec48 sub rsp, 0x48
140a135a1: 488daa80000000 lea rbp, [rdx + 0x80]
140a135a8: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a135ad: 488d4de0 lea rcx, [rbp - 0x20]
140a135b1: e8fa39e8ff call 0x140896fb0
140a135b6: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a135bb: 4883c448 add rsp, 0x48
140a135bf: 5b pop rbx
140a135c0: 5f pop rdi
140a135c1: 5e pop rsi
140a135c2: 415e pop r14
140a135c4: 415f pop r15
140a135c6: 5d pop rbp
140a135c7: c3 ret 
140a135c8: 0f1f840000000000 nop dword ptr [rax + rax]
140a135d0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a135d5: 55 push rbp
140a135d6: 4157 push r15
140a135d8: 4156 push r14
140a135da: 56 push rsi
140a135db: 57 push rdi
140a135dc: 53 push rbx
140a135dd: 4883ec48 sub rsp, 0x48
140a135e1: 488daa80000000 lea rbp, [rdx + 0x80]
140a135e8: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a135ed: 488d8d70010000 lea rcx, [rbp + 0x170]
140a135f4: e8b739e8ff call 0x140896fb0
140a135f9: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a135fe: 4883c448 add rsp, 0x48
140a13602: 5b pop rbx
140a13603: 5f pop rdi
140a13604: 5e pop rsi
140a13605: 415e pop r14
140a13607: 415f pop r15
140a13609: 5d pop rbp
140a1360a: c3 ret 
140a1360b: 0f1f440000 nop dword ptr [rax + rax]
140a13610: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a13615: 55 push rbp
140a13616: 4157 push r15
140a13618: 4156 push r14
140a1361a: 56 push rsi
140a1361b: 57 push rdi
140a1361c: 53 push rbx
140a1361d: 4883ec48 sub rsp, 0x48
140a13621: 488daa80000000 lea rbp, [rdx + 0x80]
140a13628: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a1362d: 488d4de0 lea rcx, [rbp - 0x20]
140a13631: e87a39e8ff call 0x140896fb0
140a13636: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a1363b: 4883c448 add rsp, 0x48
140a1363f: 5b pop rbx
140a13640: 5f pop rdi
140a13641: 5e pop rsi
140a13642: 415e pop r14
140a13644: 415f pop r15
140a13646: 5d pop rbp
140a13647: c3 ret 
140a13648: 0f1f840000000000 nop dword ptr [rax + rax]
140a13650: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a13655: 55 push rbp
140a13656: 4157 push r15
140a13658: 4156 push r14
140a1365a: 56 push rsi
140a1365b: 57 push rdi
140a1365c: 53 push rbx
140a1365d: 4883ec48 sub rsp, 0x48
140a13661: 488daa80000000 lea rbp, [rdx + 0x80]
140a13668: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a1366d: 80bda602000008 cmp byte ptr [rbp + 0x2a6], 8
140a13674: 740c je 0x140a13682
140a13676: 488d8d90020000 lea rcx, [rbp + 0x290]
140a1367d: e82eee1c00 call 0x140be24b0
140a13682: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a13687: 4883c448 add rsp, 0x48
140a1368b: 5b pop rbx
140a1368c: 5f pop rdi
140a1368d: 5e pop rsi
140a1368e: 415e pop r14
140a13690: 415f pop r15
140a13692: 5d pop rbp
140a13693: c3 ret 
140a13694: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a136a0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a136a5: 55 push rbp
140a136a6: 4157 push r15
140a136a8: 4156 push r14
140a136aa: 56 push rsi
140a136ab: 57 push rdi
140a136ac: 53 push rbx
140a136ad: 4883ec48 sub rsp, 0x48
140a136b1: 488daa80000000 lea rbp, [rdx + 0x80]
140a136b8: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a136bd: 488d4de0 lea rcx, [rbp - 0x20]
140a136c1: e8ea38e8ff call 0x140896fb0
140a136c6: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a136cb: 4883c448 add rsp, 0x48
140a136cf: 5b pop rbx
140a136d0: 5f pop rdi
140a136d1: 5e pop rsi
140a136d2: 415e pop r14
140a136d4: 415f pop r15
140a136d6: 5d pop rbp
140a136d7: c3 ret 
140a136d8: 0f1f840000000000 nop dword ptr [rax + rax]
140a136e0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a136e5: 55 push rbp
140a136e6: 4157 push r15
140a136e8: 4156 push r14
140a136ea: 56 push rsi
140a136eb: 57 push rdi
140a136ec: 53 push rbx
140a136ed: 4883ec48 sub rsp, 0x48
140a136f1: 488daa80000000 lea rbp, [rdx + 0x80]
140a136f8: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a136fd: 488d8d70010000 lea rcx, [rbp + 0x170]
140a13704: e8a738e8ff call 0x140896fb0
140a13709: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a1370e: 4883c448 add rsp, 0x48
140a13712: 5b pop rbx
140a13713: 5f pop rdi
140a13714: 5e pop rsi
140a13715: 415e pop r14
140a13717: 415f pop r15
140a13719: 5d pop rbp
140a1371a: c3 ret 
140a1371b: 0f1f440000 nop dword ptr [rax + rax]
140a13720: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a13725: 55 push rbp
140a13726: 4157 push r15
140a13728: 4156 push r14
140a1372a: 56 push rsi
140a1372b: 57 push rdi
140a1372c: 53 push rbx
140a1372d: 4883ec48 sub rsp, 0x48
140a13731: 488daa80000000 lea rbp, [rdx + 0x80]
140a13738: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a1373d: 488d4de0 lea rcx, [rbp - 0x20]
140a13741: e86a38e8ff call 0x140896fb0
140a13746: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a1374b: 4883c448 add rsp, 0x48
140a1374f: 5b pop rbx
140a13750: 5f pop rdi
140a13751: 5e pop rsi
140a13752: 415e pop r14
140a13754: 415f pop r15
140a13756: 5d pop rbp
140a13757: c3 ret 
140a13758: 0f1f840000000000 nop dword ptr [rax + rax]
140a13760: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a13765: 55 push rbp
140a13766: 4157 push r15
140a13768: 4156 push r14
140a1376a: 56 push rsi
140a1376b: 57 push rdi
140a1376c: 53 push rbx
140a1376d: 4883ec48 sub rsp, 0x48
140a13771: 488daa80000000 lea rbp, [rdx + 0x80]
140a13778: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a1377d: 488d8d70010000 lea rcx, [rbp + 0x170]
140a13784: e82738e8ff call 0x140896fb0
140a13789: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a1378e: 4883c448 add rsp, 0x48
140a13792: 5b pop rbx
140a13793: 5f pop rdi
140a13794: 5e pop rsi
140a13795: 415e pop r14
140a13797: 415f pop r15
140a13799: 5d pop rbp
140a1379a: c3 ret 
140a1379b: 0f1f440000 nop dword ptr [rax + rax]
140a137a0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a137a5: 55 push rbp
140a137a6: 4157 push r15
140a137a8: 4156 push r14
140a137aa: 56 push rsi
140a137ab: 57 push rdi
140a137ac: 53 push rbx
140a137ad: 4883ec48 sub rsp, 0x48
140a137b1: 488daa80000000 lea rbp, [rdx + 0x80]
140a137b8: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a137bd: 488d4de0 lea rcx, [rbp - 0x20]
140a137c1: e8ea37e8ff call 0x140896fb0
140a137c6: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a137cb: 4883c448 add rsp, 0x48
140a137cf: 5b pop rbx
140a137d0: 5f pop rdi
140a137d1: 5e pop rsi
140a137d2: 415e pop r14
140a137d4: 415f pop r15
140a137d6: 5d pop rbp
140a137d7: c3 ret 
140a137d8: 0f1f840000000000 nop dword ptr [rax + rax]
140a137e0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a137e5: 55 push rbp
140a137e6: 4157 push r15
140a137e8: 4156 push r14
140a137ea: 56 push rsi
140a137eb: 57 push rdi
140a137ec: 53 push rbx
140a137ed: 4883ec48 sub rsp, 0x48
140a137f1: 488daa80000000 lea rbp, [rdx + 0x80]
140a137f8: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a137fd: 80bda602000008 cmp byte ptr [rbp + 0x2a6], 8
140a13804: 740c je 0x140a13812
140a13806: 488d8d90020000 lea rcx, [rbp + 0x290]
140a1380d: e89eec1c00 call 0x140be24b0
140a13812: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a13817: 4883c448 add rsp, 0x48
140a1381b: 5b pop rbx
140a1381c: 5f pop rdi
140a1381d: 5e pop rsi
140a1381e: 415e pop r14
140a13820: 415f pop r15
140a13822: 5d pop rbp
140a13823: c3 ret 
140a13824: 6666662e0f1f840000000000 nop word ptr cs:[rax + rax]
140a13830: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a13835: 55 push rbp
140a13836: 4157 push r15
140a13838: 4156 push r14
140a1383a: 56 push rsi
140a1383b: 57 push rdi
140a1383c: 53 push rbx
140a1383d: 4883ec48 sub rsp, 0x48
140a13841: 488daa80000000 lea rbp, [rdx + 0x80]
140a13848: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a1384d: 488d4de0 lea rcx, [rbp - 0x20]
140a13851: e85a37e8ff call 0x140896fb0
140a13856: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a1385b: 4883c448 add rsp, 0x48
140a1385f: 5b pop rbx
140a13860: 5f pop rdi
140a13861: 5e pop rsi
140a13862: 415e pop r14
140a13864: 415f pop r15
140a13866: 5d pop rbp
140a13867: c3 ret 
140a13868: 0f1f840000000000 nop dword ptr [rax + rax]
140a13870: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a13875: 55 push rbp
140a13876: 4157 push r15
140a13878: 4156 push r14
140a1387a: 56 push rsi
140a1387b: 57 push rdi
140a1387c: 53 push rbx
140a1387d: 4883ec48 sub rsp, 0x48
140a13881: 488daa80000000 lea rbp, [rdx + 0x80]
140a13888: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a1388d: 488d8d70010000 lea rcx, [rbp + 0x170]
140a13894: e81737e8ff call 0x140896fb0
140a13899: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a1389e: 4883c448 add rsp, 0x48
140a138a2: 5b pop rbx
140a138a3: 5f pop rdi
140a138a4: 5e pop rsi
140a138a5: 415e pop r14
140a138a7: 415f pop r15
140a138a9: 5d pop rbp
140a138aa: c3 ret 
140a138ab: 0f1f440000 nop dword ptr [rax + rax]
140a138b0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a138b5: 55 push rbp
140a138b6: 4157 push r15
140a138b8: 4156 push r14
140a138ba: 56 push rsi
140a138bb: 57 push rdi
140a138bc: 53 push rbx
140a138bd: 4883ec48 sub rsp, 0x48
140a138c1: 488daa80000000 lea rbp, [rdx + 0x80]
140a138c8: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a138cd: 488d4de0 lea rcx, [rbp - 0x20]
140a138d1: e8da36e8ff call 0x140896fb0
140a138d6: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a138db: 4883c448 add rsp, 0x48
140a138df: 5b pop rbx
140a138e0: 5f pop rdi
140a138e1: 5e pop rsi
140a138e2: 415e pop r14
140a138e4: 415f pop r15
140a138e6: 5d pop rbp
140a138e7: c3 ret 
140a138e8: 0f1f840000000000 nop dword ptr [rax + rax]
140a138f0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a138f5: 55 push rbp
140a138f6: 4157 push r15
140a138f8: 4156 push r14
140a138fa: 56 push rsi
140a138fb: 57 push rdi
140a138fc: 53 push rbx
140a138fd: 4883ec48 sub rsp, 0x48
140a13901: 488daa80000000 lea rbp, [rdx + 0x80]
140a13908: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a1390d: 488d8d70010000 lea rcx, [rbp + 0x170]
140a13914: e89736e8ff call 0x140896fb0
140a13919: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a1391e: 4883c448 add rsp, 0x48
140a13922: 5b pop rbx
140a13923: 5f pop rdi
140a13924: 5e pop rsi
140a13925: 415e pop r14
140a13927: 415f pop r15
140a13929: 5d pop rbp
140a1392a: c3 ret 
140a1392b: 0f1f440000 nop dword ptr [rax + rax]
140a13930: 4889542410 mov qword ptr [rsp + 0x10], rdx
140a13935: 55 push rbp
140a13936: 4157 push r15
140a13938: 4156 push r14
140a1393a: 56 push rsi
140a1393b: 57 push rdi
140a1393c: 53 push rbx
140a1393d: 4883ec48 sub rsp, 0x48
140a13941: 488daa80000000 lea rbp, [rdx + 0x80]
140a13948: 0f29742430 movaps xmmword ptr [rsp + 0x30], xmm6
140a1394d: 488d4de0 lea rcx, [rbp - 0x20]
140a13951: e85a36e8ff call 0x140896fb0
140a13956: 0f28742430 movaps xmm6, xmmword ptr [rsp + 0x30]
140a1395b: 4883c448 add rsp, 0x48
140a1395f: 5b pop rbx
140a13960: 5f pop rdi
140a13961: 5e pop rsi
140a13962: 415e pop r14
140a13964: 415f pop r15
140a13966: 5d pop rbp
140a13967: c3 ret 
