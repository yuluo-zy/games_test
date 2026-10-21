0x140161cd0: push rbp
0x140161cd1: push r15
0x140161cd3: push r14
0x140161cd5: push r13
0x140161cd7: push r12
0x140161cd9: push rsi
0x140161cda: push rdi
0x140161cdb: push rbx
0x140161cdc: mov eax, 0x1b38
0x140161ce1: call 0x14289aa40
0x140161ce6: sub rsp, rax
0x140161ce9: lea rbp, [rsp + 0x80]
0x140161cf1: movaps xmmword ptr [rbp + 0x1aa0], xmm6
0x140161cf8: mov qword ptr [rbp + 0x1a98], 0xfffffffffffffffe
0x140161d03: mov r12, r8
0x140161d06: mov rsi, rdx
0x140161d09: mov rax, qword ptr [rdx]
0x140161d0c: movabs r8, 0x8000000000000000
0x140161d16: mov qword ptr [rbp + 0x1a08], r8
0x140161d1d: mov qword ptr [rdx], r8
0x140161d20: mov rdx, rax
0x140161d23: neg rdx
0x140161d26: jo 0x14016405d
0x140161d2c: mov qword ptr [rbp + 0x1798], r9
0x140161d33: mov qword ptr [rbp + 0x1790], rcx
0x140161d3a: lea rdx, [rsi + 8]
0x140161d3e: mov qword ptr [rbp + 0x1198], rax
0x140161d45: lea rcx, [rbp + 0x11a0]
0x140161d4c: mov r8d, 0x88
0x140161d52: call 0x142923c00
0x140161d57: mov r14, qword ptr [rsi + 0x90]
0x140161d5e: mov qword ptr [rsi + 0x90], 0
0x140161d69: test r14, r14
0x140161d6c: mov qword ptr [rbp + 0x17a8], r14
0x140161d73: je 0x140164069
0x140161d79: mov rbx, qword ptr [rbp + 0x1b20]
0x140161d80: mov rax, qword ptr [rsi + 0xe8]
0x140161d87: mov qword ptr [rbp + 0x1330], rax
0x140161d8e: movups xmm0, xmmword ptr [rsi + 0xd8]
0x140161d95: movups xmmword ptr [rbp + 0x1320], xmm0
0x140161d9c: movups xmm0, xmmword ptr [rsi + 0x98]
0x140161da3: movups xmm1, xmmword ptr [rsi + 0xa8]
0x140161daa: movups xmm2, xmmword ptr [rsi + 0xb8]
0x140161db1: mov qword ptr [rbp + 0x19b0], rsi
0x140161db8: movups xmm3, xmmword ptr [rsi + 0xc8]
0x140161dbf: movups xmmword ptr [rbp + 0x1310], xmm3
0x140161dc6: movups xmmword ptr [rbp + 0x1300], xmm2
0x140161dcd: movups xmmword ptr [rbp + 0x12f0], xmm1
0x140161dd4: movups xmmword ptr [rbp + 0x12e0], xmm0
0x140161ddb: lea rcx, [rbp + 0x1228]
0x140161de2: lea rdx, [rbp + 0x1198]
0x140161de9: mov r8d, 0x90
0x140161def: call 0x142923c00
0x140161df4: mov r15, qword ptr [rbx]
0x140161df7: mov rsi, qword ptr [rbx + 8]
0x140161dfb: mov rax, qword ptr [rbx + 0x10]
0x140161dff: mov qword ptr [rbp + 0x1a48], rax
0x140161e06: mov rdi, qword ptr [rbx + 0x18]
0x140161e0a: mov rax, qword ptr [rbx + 0x20]
0x140161e0e: mov qword ptr [rbp + 0x1a00], rax
0x140161e15: mov rax, qword ptr [rbx + 0x28]
0x140161e19: mov qword ptr [rbp + 0x19f0], rax
0x140161e20: mov rax, qword ptr [rbx + 0x30]
0x140161e24: mov qword ptr [rbp + 0x1780], rax
0x140161e2b: mov rax, qword ptr [rbx + 0x38]
0x140161e2f: mov qword ptr [rbp + 0x19b8], rax
0x140161e36: mov qword ptr [rbp + 0x12b8], 0
0x140161e41: mov qword ptr [rbp + 0x12c0], 8
0x140161e4c: mov qword ptr [rbp + 0x12c8], 0
0x140161e57: mov qword ptr [rbp + 0x1788], r12
0x140161e5e: mov qword ptr [rbp + 0x12d0], r12
0x140161e65: mov qword ptr [rbp + 0x12d8], r14
0x140161e6c: lea r12, [r15 + 0x38]
0x140161e70: mov byte ptr [rbp + 0x1a93], 1
0x140161e77: movabs rdx, 0xa68561b396784eaa
0x140161e81: movabs r8, 0xf5fd74e6faf4ed14
0x140161e8b: mov rcx, r12
0x140161e8e: call 0x1400dd6f0
0x140161e93: test rax, rax
0x140161e96: je 0x140163f0e
0x140161e9c: cmp qword ptr [r15 + 0x218], rdx
0x140161ea3: jbe 0x140163f0e
0x140161ea9: mov rax, qword ptr [r15 + 0x210]
0x140161eb0: mov rax, qword ptr [rax + rdx*8]
0x140161eb4: test rax, rax
0x140161eb7: je 0x140163f0e
0x140161ebd: mov r13, qword ptr [r15 + 0x1e0]
0x140161ec4: not rax
0x140161ec7: lea rax, [rax + rax*2]
0x140161ecb: shl rax, 5
0x140161ecf: cmp qword ptr [r13 + rax + 0x20], 0
0x140161ed5: je 0x140163f0e
0x140161edb: add r13, rax
0x140161ede: mov rbx, qword ptr [r13 + 0x10]
0x140161ee2: mov r14d, dword ptr [r15 + 0x5b0]
0x140161ee9: mov dword ptr [r13 + 0x5c], r14d
0x140161eed: mov byte ptr [rbp + 0x1a93], 1
0x140161ef4: lea rdx, [rbp + 0x1228]
0x140161efb: mov rcx, rbx
0x140161efe: call 0x14093cd30
0x140161f03: mov dword ptr [r13 + 0x5c], r14d
0x140161f07: mov byte ptr [rbp + 0x1a93], 1
0x140161f0e: mov rcx, rbx
0x140161f11: mov rdx, rsi
0x140161f14: call 0x14093d040
0x140161f19: mov byte ptr [rbp + 0x1a93], 1
0x140161f20: lea rcx, [rbp + 0x1568]
0x140161f27: mov rdx, rax
0x140161f2a: call 0x140bd6b00
0x140161f2f: mov byte ptr [rbp + 0x1a94], 1
0x140161f36: movabs rdx, 0x662978458699a5ef
0x140161f40: movabs r8, 0x3ec057c162419b46
0x140161f4a: mov rcx, r12
0x140161f4d: call 0x1400dd6f0
0x140161f52: test rax, rax
0x140161f55: je 0x140163f26
0x140161f5b: cmp qword ptr [r15 + 0x218], rdx
0x140161f62: jbe 0x140163f26
0x140161f68: mov rax, qword ptr [r15 + 0x210]
0x140161f6f: mov rcx, qword ptr [rax + rdx*8]
0x140161f73: test rcx, rcx
0x140161f76: je 0x140163f26
0x140161f7c: mov rax, qword ptr [r15 + 0x1e0]
0x140161f83: not rcx
0x140161f86: lea rcx, [rcx + rcx*2]
0x140161f8a: shl rcx, 5
0x140161f8e: cmp qword ptr [rax + rcx + 0x20], 0
0x140161f94: je 0x140163f26
0x140161f9a: mov r14, rdi
0x140161f9d: add rax, rcx
0x140161fa0: mov rcx, qword ptr [rax + 0x10]
0x140161fa4: mov edx, dword ptr [r15 + 0x5b0]
0x140161fab: mov dword ptr [rax + 0x5c], edx
0x140161fae: mov byte ptr [rbp + 0x1a94], 1
0x140161fb5: lea rdx, [rbp + 0x1228]
0x140161fbc: call 0x1409921c0
0x140161fc1: mov byte ptr [rbp + 0x1a94], 1
0x140161fc8: movabs rdx, 0x8fd9dc19c7973bca
0x140161fd2: movabs r8, 0x361ff4c5eb9b45fd
0x140161fdc: mov rcx, r12
0x140161fdf: call 0x1400dd6f0
0x140161fe4: test rax, rax
0x140161fe7: je 0x140163f3e
0x140161fed: cmp qword ptr [r15 + 0x218], rdx
0x140161ff4: jbe 0x140163f3e
0x140161ffa: mov rax, qword ptr [r15 + 0x210]
0x140162001: mov rcx, qword ptr [rax + rdx*8]
0x140162005: test rcx, rcx
0x140162008: je 0x140163f3e
0x14016200e: mov rax, qword ptr [r15 + 0x1e0]
0x140162015: not rcx
0x140162018: lea rcx, [rcx + rcx*2]
0x14016201c: shl rcx, 5
0x140162020: cmp qword ptr [rax + rcx + 0x20], 0
0x140162026: je 0x140163f3e
0x14016202c: add rax, rcx
0x14016202f: mov rdx, qword ptr [rax + 0x10]
0x140162033: lea rcx, [rbp + 0xe0]
0x14016203a: mov r8d, 0x1090
0x140162040: call 0x142923c00
0x140162045: mov byte ptr [rbp + 0x1a94], 1
0x14016204c: movabs rdx, 0x47c94cb78283a72e
0x140162056: movabs r8, 0x8c173ded97323e9d
0x140162060: mov rcx, r12
0x140162063: call 0x1400dd6f0
0x140162068: test rax, rax
0x14016206b: je 0x140163f56
0x140162071: cmp qword ptr [r15 + 0x218], rdx
0x140162078: jbe 0x140163f56
0x14016207e: mov rax, qword ptr [r15 + 0x210]
0x140162085: mov rcx, qword ptr [rax + rdx*8]
0x140162089: test rcx, rcx
0x14016208c: je 0x140163f56
0x140162092: mov rax, qword ptr [r15 + 0x1e0]
0x140162099: not rcx
0x14016209c: lea rcx, [rcx + rcx*2]
0x1401620a0: shl rcx, 5
0x1401620a4: cmp qword ptr [rax + rcx + 0x20], 0
0x1401620aa: je 0x140163f56
0x1401620b0: add rax, rcx
0x1401620b3: mov rcx, qword ptr [rax + 0x10]
0x1401620b7: mov byte ptr [rbp + 0x1a94], 1
0x1401620be: call 0x14067dd80
0x1401620c3: mov rdi, rax
0x1401620c6: mov cl, 1
0x1401620c8: xor eax, eax
0x1401620ca: lock cmpxchg byte ptr [rdi], cl
0x1401620ce: jne 0x140163e90
0x1401620d4: mov byte ptr [rbp + 0x1a94], 1
0x1401620db: mov rcx, rdi
0x1401620de: call 0x140c81c70
0x1401620e3: mov qword ptr [rbp + 0x1a80], rdi
0x1401620ea: add rdi, 8
0x1401620ee: mov qword ptr [rsp + 0x20], 0
0x1401620f7: lea rcx, [rbp + 0x1810]
0x1401620fe: mov r9d, 0x1090
0x140162104: mov rdx, rdi
0x140162107: mov r8, rsi
0x14016210a: call 0x140c02740
0x14016210f: mov rcx, qword ptr [rbp + 0x1818]
0x140162116: cmp rcx, 0x1090
0x14016211d: jne 0x140163eaa
0x140162123: mov rcx, qword ptr [rbp + 0x1810]
0x14016212a: lea rdx, [rbp + 0xe0]
0x140162131: mov r8d, 0x1090
0x140162137: call 0x142923c00
0x14016213c: movups xmm0, xmmword ptr [rbp + 0x1810]
0x140162143: movups xmm1, xmmword ptr [rbp + 0x1820]
0x14016214a: movups xmm2, xmmword ptr [rbp + 0x1830]
0x140162151: movups xmm3, xmmword ptr [rbp + 0x1840]
0x140162158: movaps xmmword ptr [rbp + 0x15e0], xmm0
0x14016215f: movaps xmmword ptr [rbp + 0x15f0], xmm1
0x140162166: movaps xmmword ptr [rbp + 0x1600], xmm2
0x14016216d: movaps xmmword ptr [rbp + 0x1610], xmm3
0x140162174: mov rax, qword ptr [rbp + 0x1850]
0x14016217b: mov qword ptr [rbp + 0x1620], rax
0x140162182: lea rcx, [rbp + 0x1338]
0x140162189: lea rdx, [rbp + 0x15e0]
0x140162190: mov r8, rsi
0x140162193: call 0x140c02370
0x140162198: mov byte ptr [rbp + 0x1a8e], 1
0x14016219f: mov rdi, qword ptr [rbp + 0x1a80]
0x1401621a6: mov rcx, rdi
0x1401621a9: call 0x140c81e10
0x1401621ae: xor ecx, ecx
0x1401621b0: mov al, 1
0x1401621b2: lock cmpxchg byte ptr [rdi], cl
0x1401621b6: jne 0x140163ec0
0x1401621bc: lea rcx, [rbp + 0x15e0]
0x1401621c3: lea rdx, [rbp + 0x1228]
0x1401621ca: mov r8d, 0x110
0x1401621d0: call 0x142923c00
0x1401621d5: movups xmm0, xmmword ptr [rbp + 0x1338]
0x1401621dc: movups xmm1, xmmword ptr [rbp + 0x1348]
0x1401621e3: movups xmm2, xmmword ptr [rbp + 0x1358]
0x1401621ea: movups xmm3, xmmword ptr [rbp + 0x1368]
0x1401621f1: movups xmmword ptr [rbp + 0x16f0], xmm0
0x1401621f8: movups xmmword ptr [rbp + 0x1700], xmm1
0x1401621ff: movups xmmword ptr [rbp + 0x1710], xmm2
0x140162206: movups xmmword ptr [rbp + 0x1720], xmm3
0x14016220d: movups xmm0, xmmword ptr [rbp + 0x1378]
0x140162214: movups xmmword ptr [rbp + 0x1730], xmm0
0x14016221b: movups xmm0, xmmword ptr [rbp + 0x1388]
0x140162222: movups xmmword ptr [rbp + 0x1740], xmm0
0x140162229: movups xmm0, xmmword ptr [rbp + 0x1568]
0x140162230: movups xmmword ptr [rbp + 0x1750], xmm0
0x140162237: mov rax, qword ptr [rbp + 0x1578]
0x14016223e: mov qword ptr [rbp + 0x1760], rax
0x140162245: mov rax, qword ptr [rbp + 0x1a48]
0x14016224c: mov qword ptr [rbp + 0x1768], rax
0x140162253: mov qword ptr [rbp + 0x1770], rsi
0x14016225a: mov byte ptr [rbp + 0x1a97], 1
0x140162261: movabs rdx, 0x162d45f443f2465a
0x14016226b: movabs r8, 0x7db0d9595508fc0f
0x140162275: mov rcx, r12
0x140162278: call 0x1400dd6f0
0x14016227d: test rax, rax
0x140162280: je 0x140163f6e
0x140162286: cmp qword ptr [r15 + 0x218], rdx
0x14016228d: jbe 0x140163f6e
0x140162293: mov rax, qword ptr [r15 + 0x210]
0x14016229a: mov rcx, qword ptr [rax + rdx*8]
0x14016229e: test rcx, rcx
0x1401622a1: je 0x140163f6e
0x1401622a7: mov rax, qword ptr [r15 + 0x1e0]
0x1401622ae: not rcx
0x1401622b1: lea rcx, [rcx + rcx*2]
0x1401622b5: shl rcx, 5
0x1401622b9: cmp qword ptr [rax + rcx + 0x20], 0
0x1401622bf: je 0x140163f6e
0x1401622c5: add rax, rcx
0x1401622c8: mov rcx, qword ptr [rax + 0x10]
0x1401622cc: mov edx, dword ptr [r15 + 0x5b0]
0x1401622d3: mov dword ptr [rax + 0x5c], edx
0x1401622d6: mov byte ptr [rbp + 0x1a97], 1
0x1401622dd: lea rdx, [rbp + 0x15e0]
0x1401622e4: mov r8, r15
0x1401622e7: call 0x14090e280
0x1401622ec: mov byte ptr [rbp + 0x1a97], 1
0x1401622f3: lea rcx, [rbp + 0x15e0]
0x1401622fa: mov rdx, r15
0x1401622fd: call 0x14090a870
0x140162302: mov rdx, qword ptr [r14]
0x140162305: mov byte ptr [rbp + 0x1a97], 1
0x14016230c: lea rcx, [rbp + 0x1810]
0x140162313: mov r8d, 0x7a
0x140162319: call 0x140bd52a0
0x14016231e: mov byte ptr [rbp + 0x1a97], 1
0x140162325: lea r13, [rbp + 0x1a10]
0x14016232c: lea rdx, [rbp + 0x1810]
0x140162333: mov rcx, r13
0x140162336: mov r8b, 6
0x140162339: call 0x140bd5460
0x14016233e: movups xmm0, xmmword ptr [rbp + 0x1a10]
0x140162345: movups xmm1, xmmword ptr [rbp + 0x1a20]
0x14016234c: movaps xmmword ptr [rbp + 0x1460], xmm1
0x140162353: movaps xmmword ptr [rbp + 0x1450], xmm0
0x14016235a: lea rcx, [rbp + 0x1820]
0x140162361: mov byte ptr [rbp + 0x1a97], 1
0x140162368: lea rdx, [rbp + 0x1450]
0x14016236f: call 0x140bd6ce0
0x140162374: lea rax, [rip + 0x27fb32d]
0x14016237b: mov qword ptr [rbp + 0x1810], rax
0x140162382: mov qword ptr [rbp + 0x1818], 0xa
0x14016238d: mov byte ptr [rbp + 0x1a97], 1
0x140162394: lea rcx, [rbp + 0x15e0]
0x14016239b: lea rdx, [rbp + 0x1810]
0x1401623a2: call 0x140be94d0
0x1401623a7: movups xmm0, xmmword ptr [rbp + 0x1a10]
0x1401623ae: movups xmm1, xmmword ptr [rbp + 0x1a20]
0x1401623b5: movups xmmword ptr [rbp + 0x19c4], xmm0
0x1401623bc: movups xmmword ptr [rbp + 0x19d4], xmm1
0x1401623c3: mov dword ptr [rbp + 0x19c0], eax
0x1401623c9: mov byte ptr [rbp + 0x1a97], 1
0x1401623d0: movabs rdx, 0xd38dae617fb79c7a
0x1401623da: movabs r8, 0x22aadf5c5dad1eae
0x1401623e4: mov rcx, r12
0x1401623e7: call 0x1400dd6f0
0x1401623ec: test rax, rax
0x1401623ef: je 0x140163f86
0x1401623f5: cmp qword ptr [r15 + 0x218], rdx
0x1401623fc: jbe 0x140163f86
0x140162402: mov rax, qword ptr [r15 + 0x210]
0x140162409: mov rcx, qword ptr [rax + rdx*8]
0x14016240d: test rcx, rcx
0x140162410: je 0x140163f86
0x140162416: mov rax, qword ptr [r15 + 0x1e0]
0x14016241d: not rcx
0x140162420: lea rcx, [rcx + rcx*2]
0x140162424: shl rcx, 5
0x140162428: cmp qword ptr [rax + rcx + 0x20], 0
0x14016242e: je 0x140163f86
0x140162434: add rax, rcx
0x140162437: mov rsi, qword ptr [rax + 0x10]
0x14016243b: mov byte ptr [rbp + 0x1a97], 1
0x140162442: movabs rdx, 0x35385c45b6f578e9
0x14016244c: movabs r8, 0xf1623d81d5dd0c9f
0x140162456: mov rcx, r12
0x140162459: call 0x1400dd6f0
0x14016245e: test rax, rax
0x140162461: je 0x140163f9e
0x140162467: cmp qword ptr [r15 + 0x218], rdx
0x14016246e: jbe 0x140163f9e
0x140162474: mov rax, qword ptr [r15 + 0x210]
0x14016247b: mov rcx, qword ptr [rax + rdx*8]
0x14016247f: test rcx, rcx
0x140162482: je 0x140163f9e
0x140162488: mov rax, qword ptr [r15 + 0x1e0]
0x14016248f: not rcx
0x140162492: lea rcx, [rcx + rcx*2]
0x140162496: shl rcx, 5
0x14016249a: cmp qword ptr [rax + rcx + 0x20], 0
0x1401624a0: je 0x140163f9e
0x1401624a6: add rax, rcx
0x1401624a9: mov rax, qword ptr [rax + 0x10]
0x1401624ad: cmp byte ptr [rax], 1
0x1401624b0: jne 0x1401627b4
0x1401624b6: mov byte ptr [rbp + 0x1a97], 1
0x1401624bd: mov rcx, rsi
0x1401624c0: call 0x14093fc20
0x1401624c5: test al, al
0x1401624c7: je 0x1401627b4
0x1401624cd: mov byte ptr [rbp + 0x1a97], 1
0x1401624d4: movabs rdx, 0xd38dae617fb79c7a
0x1401624de: movabs r8, 0x22aadf5c5dad1eae
0x1401624e8: mov rcx, r12
0x1401624eb: call 0x1400dd6f0
0x1401624f0: test rax, rax
0x1401624f3: je 0x140164077
0x1401624f9: cmp qword ptr [r15 + 0x218], rdx
0x140162500: jbe 0x140164077
0x140162506: mov rax, qword ptr [r15 + 0x210]
0x14016250d: mov rcx, qword ptr [rax + rdx*8]
0x140162511: test rcx, rcx
0x140162514: je 0x140164077
0x14016251a: mov rax, qword ptr [r15 + 0x1e0]
0x140162521: not rcx
0x140162524: lea rcx, [rcx + rcx*2]
0x140162528: shl rcx, 5
0x14016252c: cmp qword ptr [rax + rcx + 0x20], 0
0x140162532: je 0x140164077
0x140162538: add rax, rcx
0x14016253b: mov rcx, qword ptr [rax + 0x10]
0x14016253f: mov byte ptr [rbp + 0x1a97], 1
0x140162546: call 0x14093fbc0
0x14016254b: mov dword ptr [rbp + 0x1a48], eax
0x140162551: mov byte ptr [rbp + 0x1a97], 1
0x140162558: mov rcx, rsi
0x14016255b: call 0x14093fbe0
0x140162560: test al, al
0x140162562: jne 0x140162840
0x140162568: cmp byte ptr [rbp + 0x1a48], 0
0x14016256f: je 0x140162614
0x140162575: mov byte ptr [rbp + 0x1a97], 1
0x14016257c: movabs rdx, 0x1f18d9c9bdba6c60
0x140162586: movabs r8, 0x8547986648e90b67
0x140162590: mov rcx, r12
0x140162593: call 0x1400dd6f0
0x140162598: test rax, rax
0x14016259b: je 0x1401640af
0x1401625a1: cmp qword ptr [r15 + 0x218], rdx
0x1401625a8: jbe 0x1401640af
0x1401625ae: mov rax, qword ptr [r15 + 0x210]
0x1401625b5: mov rcx, qword ptr [rax + rdx*8]
0x1401625b9: test rcx, rcx
0x1401625bc: je 0x1401640af
0x1401625c2: mov rax, qword ptr [r15 + 0x1e0]
0x1401625c9: not rcx
0x1401625cc: lea rcx, [rcx + rcx*2]
0x1401625d0: shl rcx, 5
0x1401625d4: cmp qword ptr [rax + rcx + 0x20], 0
0x1401625da: je 0x1401640af
0x1401625e0: add rax, rcx
0x1401625e3: mov rcx, qword ptr [rax + 0x10]
0x1401625e7: mov byte ptr [rbp + 0x1a97], 1
0x1401625ee: call 0x1408a63d0
0x1401625f3: lea rcx, [rip + 0x27fb11e]
0x1401625fa: mov qword ptr [rbp + 0x1a10], rcx
0x140162601: mov qword ptr [rbp + 0x1a18], 0x18
0x14016260c: test al, al
0x14016260e: je 0x140163d8f
0x140162614: mov byte ptr [rbp + 0x1a97], 1
0x14016261b: movabs rdx, 0x8fd9dc19c7973bca
0x140162625: movabs r8, 0x361ff4c5eb9b45fd
0x14016262f: mov rcx, r12
0x140162632: call 0x1400dd6f0
0x140162637: test rax, rax
0x14016263a: je 0x14016409a
0x140162640: cmp qword ptr [r15 + 0x218], rdx
0x140162647: jbe 0x14016409a
0x14016264d: mov rax, qword ptr [r15 + 0x210]
0x140162654: mov rcx, qword ptr [rax + rdx*8]
0x140162658: test rcx, rcx
0x14016265b: je 0x14016409a
0x140162661: mov rax, qword ptr [r15 + 0x1e0]
0x140162668: not rcx
0x14016266b: lea rcx, [rcx + rcx*2]
0x14016266f: shl rcx, 5
0x140162673: cmp qword ptr [rax + rcx + 0x20], 0
0x140162679: je 0x14016409a
0x14016267f: add rax, rcx
0x140162682: mov rcx, qword ptr [rax + 0x10]
0x140162686: mov byte ptr [rbp + 0x1a97], 1
0x14016268d: call 0x140976880
0x140162692: lea rcx, [rip + 0x27fb0af]
0x140162699: mov qword ptr [rbp + 0x1a10], rcx
0x1401626a0: mov qword ptr [rbp + 0x1a18], 0x14
0x1401626ab: test al, al
0x1401626ad: je 0x140163c5e
0x1401626b3: mov rax, qword ptr [rip + 0x34c9cee]
0x1401626ba: mov rax, qword ptr [rax]
0x1401626bd: cmp rax, 5
0x1401626c1: jb 0x140162840
0x1401626c7: mov qword ptr [rbp + 0x1450], r13
0x1401626ce: lea rax, [rip - 0x58845]
0x1401626d5: mov qword ptr [rbp + 0x1458], rax
0x1401626dc: mov byte ptr [rbp + 0x1a97], 1
0x1401626e3: lea rcx, [rip + 0x288946e]
0x1401626ea: call 0x14067dd80
0x1401626ef: movups xmm0, xmmword ptr [rax]
0x1401626f2: mov eax, dword ptr [rax + 0x10]
0x1401626f5: mov qword ptr [rbp + 0x1840], 5
0x140162700: lea rcx, [rip + 0x2889469]
0x140162707: mov qword ptr [rbp + 0x1848], rcx
0x14016270e: mov qword ptr [rbp + 0x1850], 0x17
0x140162719: lea rdx, [rip + 0x2889400]
0x140162720: mov qword ptr [rbp + 0x1860], rdx
0x140162727: mov qword ptr [rbp + 0x1868], 1
0x140162732: lea rdx, [rbp + 0x1450]
0x140162739: mov qword ptr [rbp + 0x1870], rdx
0x140162740: mov qword ptr [rbp + 0x1878], 1
0x14016274b: mov qword ptr [rbp + 0x1880], 0
0x140162756: mov qword ptr [rbp + 0x1810], 0
0x140162761: mov qword ptr [rbp + 0x1818], rcx
0x140162768: mov qword ptr [rbp + 0x1820], 0x17
0x140162773: mov qword ptr [rbp + 0x1828], 0
0x14016277e: movups xmmword ptr [rbp + 0x1830], xmm0
0x140162785: mov dword ptr [rbp + 0x1858], 1
0x14016278f: mov dword ptr [rbp + 0x185c], eax
0x140162795: mov byte ptr [rbp + 0x1a97], 1
0x14016279c: lea rcx, [rbp + 0x1a50]
0x1401627a3: lea rdx, [rbp + 0x1810]
0x1401627aa: call 0x140d4a330
0x1401627af: jmp 0x140162840
0x1401627b4: mov dword ptr [rbp + 0x1a48], 0
0x1401627be: mov byte ptr [rbp + 0x1a97], 1
0x1401627c5: movabs rdx, 0x6831e8f98c915159
0x1401627cf: movabs r8, 0xd515c479e4000725
0x1401627d9: mov rcx, r12
0x1401627dc: call 0x1400dd6f0
0x1401627e1: test rax, rax
0x1401627e4: je 0x140164048
0x1401627ea: cmp qword ptr [r15 + 0x218], rdx
0x1401627f1: jbe 0x140164048
0x1401627f7: mov rax, qword ptr [r15 + 0x210]
0x1401627fe: mov rcx, qword ptr [rax + rdx*8]
0x140162802: test rcx, rcx
0x140162805: je 0x140164048
0x14016280b: mov rax, qword ptr [r15 + 0x1e0]
0x140162812: not rcx
0x140162815: lea rcx, [rcx + rcx*2]
0x140162819: shl rcx, 5
0x14016281d: cmp qword ptr [rax + rcx + 0x20], 0
0x140162823: je 0x140164048
0x140162829: add rax, rcx
0x14016282c: mov rcx, qword ptr [rax + 0x10]
0x140162830: mov byte ptr [rbp + 0x1a97], 1
0x140162837: call 0x14091f0f0
0x14016283c: test al, al
0x14016283e: je 0x140162874
0x140162840: cmp byte ptr [rip + 0x34d2349], 1
0x140162847: jne 0x1401629e3
0x14016284d: mov byte ptr [rbp + 0x1a97], 1
0x140162854: mov al, 1
0x140162856: mov dword ptr [rbp + 0x1a80], eax
0x14016285c: lea rcx, [rbp + 0x15e0]
0x140162863: lea r8, [rbp + 0x19c0]
0x14016286a: mov rdx, r15
0x14016286d: call 0x1409beca0
0x140162872: jmp 0x14016287e
0x140162874: mov dword ptr [rbp + 0x1a80], 0
0x14016287e: mov bl, 7
0x140162880: mov rdx, qword ptr [r14]
0x140162883: mov byte ptr [rbp + 0x1a97], 1
0x14016288a: lea rcx, [rbp + 0x1a10]
0x140162891: mov r8d, 0x82
0x140162897: call 0x140bd52a0
0x14016289c: movups xmm0, xmmword ptr [rbp + 0x1a10]
0x1401628a3: movups xmm1, xmmword ptr [rbp + 0x1a20]
0x1401628aa: movaps xmmword ptr [rbp + 0x1460], xmm1
0x1401628b1: movaps xmmword ptr [rbp + 0x1450], xmm0
0x1401628b8: lea rcx, [rbp + 0x1820]
0x1401628bf: mov byte ptr [rbp + 0x1a97], 1
0x1401628c6: lea rdx, [rbp + 0x1450]
0x1401628cd: call 0x140bd6ce0
0x1401628d2: lea rax, [rip + 0x27faebf]
0x1401628d9: mov qword ptr [rbp + 0x1810], rax
0x1401628e0: mov qword ptr [rbp + 0x1818], 0xa
0x1401628eb: mov byte ptr [rbp + 0x1a97], 1
0x1401628f2: lea rcx, [rbp + 0x15e0]
0x1401628f9: lea rdx, [rbp + 0x1810]
0x140162900: call 0x140be94d0
0x140162905: movups xmm0, xmmword ptr [rbp + 0x1a10]
0x14016290c: movups xmm1, xmmword ptr [rbp + 0x1a20]
0x140162913: movups xmmword ptr [rbp + 0x17e4], xmm0
0x14016291a: movups xmmword ptr [rbp + 0x17f4], xmm1
0x140162921: mov dword ptr [rbp + 0x17e0], eax
0x140162927: cmp bl, 7
0x14016292a: jne 0x140162bed
0x140162930: mov byte ptr [rbp + 0x1a97], 1
0x140162937: lea rcx, [rbp + 0x1a10]
0x14016293e: movabs rdx, 0x100000001
0x140162948: mov r8d, 9
0x14016294e: call 0x140bd52a0
0x140162953: movups xmm0, xmmword ptr [rbp + 0x1a10]
0x14016295a: movups xmm1, xmmword ptr [rbp + 0x1a20]
0x140162961: movaps xmmword ptr [rbp + 0x1460], xmm1
0x140162968: movaps xmmword ptr [rbp + 0x1450], xmm0
0x14016296f: lea rcx, [rbp + 0x1820]
0x140162976: mov byte ptr [rbp + 0x1a97], 1
0x14016297d: lea rdx, [rbp + 0x1450]
0x140162984: call 0x140bd6ce0
0x140162989: lea rax, [rip + 0x27faf18]
0x140162990: mov qword ptr [rbp + 0x1810], rax
0x140162997: mov qword ptr [rbp + 0x1818], 3
0x1401629a2: mov byte ptr [rbp + 0x1a97], 1
0x1401629a9: lea rcx, [rbp + 0x15e0]
0x1401629b0: lea rdx, [rbp + 0x1810]
0x1401629b7: call 0x140be94d0
0x1401629bc: movups xmm0, xmmword ptr [rbp + 0x1a10]
0x1401629c3: movups xmm1, xmmword ptr [rbp + 0x1a20]
0x1401629ca: movups xmmword ptr [rbp + 0x17b4], xmm0
0x1401629d1: movups xmmword ptr [rbp + 0x17c4], xmm1
0x1401629d8: mov dword ptr [rbp + 0x17b0], eax
0x1401629de: jmp 0x140162c24
0x1401629e3: mov rdx, qword ptr [r14]
0x1401629e6: mov byte ptr [rbp + 0x1a97], 1
0x1401629ed: lea rcx, [rbp + 0x1a10]
0x1401629f4: mov r8d, 0x82
0x1401629fa: call 0x140bd52a0
0x1401629ff: movups xmm0, xmmword ptr [rbp + 0x1a10]
0x140162a06: movups xmm1, xmmword ptr [rbp + 0x1a20]
0x140162a0d: movaps xmmword ptr [rbp + 0x1460], xmm1
0x140162a14: movaps xmmword ptr [rbp + 0x1450], xmm0
0x140162a1b: lea rcx, [rbp + 0x1820]
0x140162a22: mov byte ptr [rbp + 0x1a97], 1
0x140162a29: lea r13, [rbp + 0x1450]
0x140162a30: mov rdx, r13
0x140162a33: call 0x140bd6ce0
0x140162a38: lea rax, [rip + 0x27fad59]
0x140162a3f: mov qword ptr [rbp + 0x1810], rax
0x140162a46: mov qword ptr [rbp + 0x1818], 0xa
0x140162a51: mov byte ptr [rbp + 0x1a97], 1
0x140162a58: lea rcx, [rbp + 0x15e0]
0x140162a5f: lea rdx, [rbp + 0x1810]
0x140162a66: call 0x140be94d0
0x140162a6b: movups xmm0, xmmword ptr [rbp + 0x1a10]
0x140162a72: movups xmm1, xmmword ptr [rbp + 0x1a20]
0x140162a79: movups xmmword ptr [rbp + 0x13d0], xmm0
0x140162a80: movups xmmword ptr [rbp + 0x13e0], xmm1
0x140162a87: mov dword ptr [rbp + 0x13cc], eax
0x140162a8d: mov rax, qword ptr [r14]
0x140162a90: mov ecx, dword ptr [rbp + 0x19e0]
0x140162a96: mov dword ptr [rbp + 0x1470], ecx
0x140162a9c: movups xmm0, xmmword ptr [rbp + 0x19c0]
0x140162aa3: movups xmm1, xmmword ptr [rbp + 0x19d0]
0x140162aaa: movaps xmmword ptr [rbp + 0x1460], xmm1
0x140162ab1: movaps xmmword ptr [rbp + 0x1450], xmm0
0x140162ab8: mov byte ptr [rbp + 0x1a97], 1
0x140162abf: mov qword ptr [rsp + 0x38], r13
0x140162ac4: lea rcx, [rbp + 0x13cc]
0x140162acb: mov qword ptr [rsp + 0x30], rcx
0x140162ad0: mov rcx, qword ptr [rbp + 0x19f0]
0x140162ad7: mov qword ptr [rsp + 0x28], rcx
0x140162adc: mov qword ptr [rsp + 0x20], rax
0x140162ae1: lea rcx, [rbp + 0x1810]
0x140162ae8: lea rdx, [rbp + 0x15e0]
0x140162aef: mov r8, r15
0x140162af2: mov r9, qword ptr [rbp + 0x1a00]
0x140162af9: call 0x1404ec110
0x140162afe: movups xmm0, xmmword ptr [rbp + 0x1810]
0x140162b05: movups xmm1, xmmword ptr [rbp + 0x1820]
0x140162b0c: movaps xmmword ptr [rbp + 0x15b0], xmm0
0x140162b13: movaps xmmword ptr [rbp + 0x15c0], xmm1
0x140162b1a: movzx eax, word ptr [rbp + 0x1830]
0x140162b21: mov word ptr [rbp + 0x15d0], ax
0x140162b28: movzx eax, byte ptr [rbp + 0x1832]
0x140162b2f: movzx ecx, byte ptr [rbp + 0x1833]
0x140162b36: movups xmm0, xmmword ptr [rbp + 0x1834]
0x140162b3d: movaps xmmword ptr [rbp + 0x1580], xmm0
0x140162b44: movups xmm0, xmmword ptr [rbp + 0x1844]
0x140162b4b: movaps xmmword ptr [rbp + 0x1590], xmm0
0x140162b52: movzx edx, word ptr [rbp + 0x1854]
0x140162b59: mov word ptr [rbp + 0x15a0], dx
0x140162b60: movzx ebx, byte ptr [rbp + 0x1856]
0x140162b67: movzx edi, byte ptr [rbp + 0x1857]
0x140162b6e: mov edx, dword ptr [rbp + 0x1878]
0x140162b74: mov dword ptr [rbp + 0x19e0], edx
0x140162b7a: movups xmm0, xmmword ptr [rbp + 0x1868]
0x140162b81: movaps xmmword ptr [rbp + 0x19d0], xmm0
0x140162b88: movups xmm0, xmmword ptr [rbp + 0x1858]
0x140162b8f: movaps xmmword ptr [rbp + 0x19c0], xmm0
0x140162b96: mov dl, 1
0x140162b98: mov dword ptr [rbp + 0x1a80], edx
0x140162b9e: cmp al, 7
0x140162ba0: je 0x140162880
0x140162ba6: movzx edx, word ptr [rbp + 0x15d0]
0x140162bad: mov word ptr [rbp + 0x1800], dx
0x140162bb4: movaps xmm0, xmmword ptr [rbp + 0x15b0]
0x140162bbb: movaps xmm1, xmmword ptr [rbp + 0x15c0]
0x140162bc2: movaps xmmword ptr [rbp + 0x17f0], xmm1
0x140162bc9: movaps xmmword ptr [rbp + 0x17e0], xmm0
0x140162bd0: mov byte ptr [rbp + 0x1802], al
0x140162bd6: mov byte ptr [rbp + 0x1803], cl
0x140162bdc: mov al, 1
0x140162bde: mov dword ptr [rbp + 0x1a80], eax
0x140162be4: cmp bl, 7
0x140162be7: je 0x140162930
0x140162bed: movzx eax, word ptr [rbp + 0x15a0]
0x140162bf4: mov word ptr [rbp + 0x17d0], ax
0x140162bfb: movaps xmm0, xmmword ptr [rbp + 0x1580]
0x140162c02: movaps xmm1, xmmword ptr [rbp + 0x1590]
0x140162c09: movaps xmmword ptr [rbp + 0x17c0], xmm1
0x140162c10: movaps xmmword ptr [rbp + 0x17b0], xmm0
0x140162c17: mov byte ptr [rbp + 0x17d2], bl
0x140162c1d: mov byte ptr [rbp + 0x17d3], dil
0x140162c24: mov cl, 1
0x140162c26: xor eax, eax
0x140162c28: lock cmpxchg byte ptr [rip + 0x34c8420], cl
0x140162c30: jne 0x140163ed6
0x140162c36: mov byte ptr [rbp + 0x1a97], 1
0x140162c3d: lea rcx, [rip + 0x34c840c]
0x140162c44: call 0x140c81c70
0x140162c49: mov rax, qword ptr [rbp + 0x19b0]
0x140162c50: lea rbx, [rax + 0x90]
0x140162c57: mov rax, qword ptr [rip + 0x34c8402]
0x140162c5e: imul rdx, qword ptr [rip + 0x34c83ff], 0x98
0x140162c69: xor ecx, ecx
0x140162c6b: mov r8, rax
0x140162c6e: nop 
0x140162c70: cmp rdx, rcx
0x140162c73: je 0x140162cb8
0x140162c75: add rcx, 0x98
0x140162c7c: cmp byte ptr [r8 + 0x96], 6
0x140162c84: lea r8, [r8 + 0x98]
0x140162c8b: jne 0x140162c70
0x140162c8d: movzx edi, byte ptr [rax + rcx - 0x4c]
0x140162c92: mov byte ptr [rbp + 0x1a97], 1
0x140162c99: lea rcx, [rip + 0x34c83b0]
0x140162ca0: call 0x140c81e10
0x140162ca5: xor ecx, ecx
0x140162ca7: mov al, 1
0x140162ca9: lock cmpxchg byte ptr [rip + 0x34c839f], cl
0x140162cb1: je 0x140162ce0
0x140162cb3: jmp 0x140163ef4
0x140162cb8: mov byte ptr [rbp + 0x1a97], 1
0x140162cbf: lea rcx, [rip + 0x34c838a]
0x140162cc6: call 0x140c81e10
0x140162ccb: xor ecx, ecx
0x140162ccd: mov al, 1
0x140162ccf: lock cmpxchg byte ptr [rip + 0x34c8379], cl
0x140162cd7: mov dil, 7
0x140162cda: jne 0x140163ef4
0x140162ce0: mov byte ptr [rbp + 0x1a97], 1
0x140162ce7: movabs rdx, 0x4a142e09642375b9
0x140162cf1: movabs r8, 0x43f6154cfa8c847f
0x140162cfb: mov rcx, r12
0x140162cfe: call 0x1400dd6f0
0x140162d03: test rax, rax
0x140162d06: je 0x140163fb6
0x140162d0c: cmp qword ptr [r15 + 0x218], rdx
0x140162d13: jbe 0x140163fb6
0x140162d19: mov rax, qword ptr [r15 + 0x210]
0x140162d20: mov rcx, qword ptr [rax + rdx*8]
0x140162d24: test rcx, rcx
0x140162d27: je 0x140163fb6
0x140162d2d: mov rax, qword ptr [r15 + 0x1e0]
0x140162d34: not rcx
0x140162d37: lea rcx, [rcx + rcx*2]
0x140162d3b: shl rcx, 5
0x140162d3f: cmp qword ptr [rax + rcx + 0x20], 0
0x140162d45: je 0x140163fb6
0x140162d4b: add rax, rcx
0x140162d4e: mov rcx, qword ptr [rax + 0x10]
0x140162d52: cmp dil, 7
0x140162d56: je 0x140162e90
0x140162d5c: lea eax, [rdi - 2]
0x140162d5f: cmp al, 5
0x140162d61: setae dl
0x140162d64: cmp al, 3
0x140162d66: sete al
0x140162d69: or al, dl
0x140162d6b: test al, dil
0x140162d6e: je 0x140162e90
0x140162d74: mov rax, qword ptr [rip + 0x34c962d]
0x140162d7b: mov rax, qword ptr [rax]
0x140162d7e: cmp rax, 3
0x140162d82: jb 0x140162e82
0x140162d88: mov byte ptr [rbp + 0x1a97], 1
0x140162d8f: mov r13, rcx
0x140162d92: call 0x14067dd80
0x140162d97: mov qword ptr [rbp + 0x1450], rax
0x140162d9e: lea rax, [rip - 0x1272d5]
0x140162da5: mov qword ptr [rbp + 0x1458], rax
0x140162dac: mov byte ptr [rbp + 0x1a97], 1
0x140162db3: lea rcx, [rip + 0x27faa46]
0x140162dba: call 0x14067dd80
0x140162dbf: movups xmm0, xmmword ptr [rax]
0x140162dc2: mov eax, dword ptr [rax + 0x10]
0x140162dc5: mov qword ptr [rbp + 0x1840], 3
0x140162dd0: lea rcx, [rip + 0x27faa41]
0x140162dd7: mov qword ptr [rbp + 0x1848], rcx
0x140162dde: mov qword ptr [rbp + 0x1850], 0x17
0x140162de9: lea rdx, [rip + 0x27faa00]
0x140162df0: mov qword ptr [rbp + 0x1860], rdx
0x140162df7: mov qword ptr [rbp + 0x1868], 1
0x140162e02: lea rdx, [rbp + 0x1450]
0x140162e09: mov qword ptr [rbp + 0x1870], rdx
0x140162e10: mov qword ptr [rbp + 0x1878], 1
0x140162e1b: mov qword ptr [rbp + 0x1880], 0
0x140162e26: mov qword ptr [rbp + 0x1810], 0
0x140162e31: mov qword ptr [rbp + 0x1818], rcx
0x140162e38: mov qword ptr [rbp + 0x1820], 0x17
0x140162e43: mov qword ptr [rbp + 0x1828], 0
0x140162e4e: movups xmmword ptr [rbp + 0x1830], xmm0
0x140162e55: mov dword ptr [rbp + 0x1858], 1
0x140162e5f: mov dword ptr [rbp + 0x185c], eax
0x140162e65: mov byte ptr [rbp + 0x1a97], 1
0x140162e6c: lea rcx, [rbp + 0x1a50]
0x140162e73: lea rdx, [rbp + 0x1810]
0x140162e7a: call 0x140d4a330
0x140162e7f: mov rcx, r13
0x140162e82: mov byte ptr [rbp + 0x1a97], 1
0x140162e89: call 0x14067dd80
0x140162e8e: jmp 0x140162e9c
0x140162e90: mov byte ptr [rbp + 0x1a97], 1
0x140162e97: call 0x1408aea50
0x140162e9c: mov byte ptr [rbp + 0x1a97], 1
0x140162ea3: lea rcx, [rbp + 0x1a10]
0x140162eaa: mov rdx, rax
0x140162ead: call 0x140b00b20
0x140162eb2: cmp dil, 7
0x140162eb6: mov qword ptr [rbp + 0x17a0], rsi
0x140162ebd: je 0x140162ee0
0x140162ebf: lea eax, [rdi - 2]
0x140162ec2: cmp al, 5
0x140162ec4: setae cl
0x140162ec7: cmp al, 3
0x140162ec9: sete al
0x140162ecc: or al, cl
0x140162ece: je 0x140162ee0
0x140162ed0: mov eax, edi
0x140162ed2: and al, 1
0x140162ed4: movzx eax, al
0x140162ed7: mov qword ptr [rbp + 0x19f8], rax
0x140162ede: jmp 0x140162f25
0x140162ee0: mov byte ptr [rbp + 0x1a8d], 1
0x140162ee7: lea rcx, [rbp + 0x1810]
0x140162eee: lea rdx, [rbp + 0x1a10]
0x140162ef5: call 0x140d7bd90
0x140162efa: movss xmm6, dword ptr [rbp + 0x1a28]
0x140162f02: mov rax, qword ptr [rbp + 0x1810]
0x140162f09: mov qword ptr [rbp + 0x1a08], rax
0x140162f10: mov rax, qword ptr [rbp + 0x1818]
0x140162f17: mov qword ptr [rbp + 0x19f8], rax
0x140162f1e: mov rsi, qword ptr [rbp + 0x1820]
0x140162f25: mov byte ptr [rbp + 0x1a96], 1
0x140162f2c: mov byte ptr [rbp + 0x1a95], 1
0x140162f33: lea r13, [rbp + 0x1174]
0x140162f3a: lea rdx, [rbp + 0x15e0]
0x140162f41: lea r9, [rbp + 0x19c0]
0x140162f48: mov rcx, r13
0x140162f4b: mov r8, r15
0x140162f4e: call 0x1409ed860
0x140162f53: mov rcx, qword ptr [rbp + 0x1780]
0x140162f5a: lea rax, [rcx + 8]
0x140162f5e: test byte ptr [rcx], 1
0x140162f61: cmovne r14, rax
0x140162f65: mov r9, qword ptr [r14]
0x140162f68: mov eax, dword ptr [rbp + 0x19e0]
0x140162f6e: mov dword ptr [rbp + 0x1830], eax
0x140162f74: movaps xmm0, xmmword ptr [rbp + 0x19c0]
0x140162f7b: movaps xmm1, xmmword ptr [rbp + 0x19d0]
0x140162f82: movaps xmmword ptr [rbp + 0x1820], xmm1
0x140162f89: movaps xmmword ptr [rbp + 0x1810], xmm0
0x140162f90: mov rax, qword ptr [rbp + 0x1a08]
0x140162f97: mov qword ptr [rbp + 0x1450], rax
0x140162f9e: mov rax, qword ptr [rbp + 0x19f8]
0x140162fa5: mov qword ptr [rbp + 0x1458], rax
0x140162fac: mov qword ptr [rbp + 0x1460], rsi
0x140162fb3: movss dword ptr [rbp + 0x1468], xmm6
0x140162fbb: mov byte ptr [rbp + 0x146c], 0
0x140162fc2: mov rax, qword ptr [rbp + 0x19f0]
0x140162fc9: movss xmm0, dword ptr [rax + 0x9c]
0x140162fd1: movss xmm1, dword ptr [rax + 0xa4]
0x140162fd9: mov byte ptr [rbp + 0x1a96], 0
0x140162fe0: mov byte ptr [rbp + 0x1a95], 1
0x140162fe7: movss dword ptr [rsp + 0x48], xmm1
0x140162fed: movss dword ptr [rsp + 0x40], xmm0
0x140162ff3: lea rax, [rbp + 0x1450]
0x140162ffa: mov qword ptr [rsp + 0x38], rax
0x140162fff: mov eax, dword ptr [rbp + 0x1a80]
0x140163005: mov byte ptr [rsp + 0x30], al
0x140163009: mov qword ptr [rsp + 0x28], r13
0x14016300e: lea rax, [rbp + 0x1810]
0x140163015: mov qword ptr [rsp + 0x20], rax
0x14016301a: lea rcx, [rbp + 0x1a50]
0x140163021: lea rdx, [rbp + 0x15e0]
0x140163028: mov r8, r15
0x14016302b: call 0x14090d000
0x140163030: mov qword ptr [rbp + 0x1818], 2
0x14016303b: mov word ptr [rbp + 0x1820], 0x100
0x140163044: xor eax, eax
0x140163046: lea r13, [rbp + 0x15e0]
0x14016304d: lea r14, [rbp + 0x1a50]
0x140163054: nop word ptr cs:[rax + rax]
0x140163060: cmp rax, 2
0x140163064: je 0x1401630b1
0x140163066: lea rsi, [rax + 1]
0x14016306a: mov qword ptr [rbp + 0x1810], rsi
0x140163071: movzx r8d, byte ptr [rbp + rax + 0x1820]
0x14016307a: mov byte ptr [rbp + 0x1a96], 0
0x140163081: mov byte ptr [rbp + 0x1a95], 1
0x140163088: mov rcx, r13
0x14016308b: mov rdx, r15
0x14016308e: mov r9, r14
0x140163091: call 0x14031d140
0x140163096: mov byte ptr [rbp + 0x1a96], 0
0x14016309d: mov byte ptr [rbp + 0x1a95], 1
0x1401630a4: mov rcx, rax
0x1401630a7: call 0x1404ebfb0
0x1401630ac: mov rax, rsi
0x1401630af: jmp 0x140163060
0x1401630b1: mov byte ptr [rbp + 0x1a96], 0
0x1401630b8: mov byte ptr [rbp + 0x1a95], 1
0x1401630bf: lea rcx, [rbp + 0x15e0]
0x1401630c6: lea r9, [rbp + 0x1a50]
0x1401630cd: mov rdx, r15
0x1401630d0: mov r8b, 2
0x1401630d3: call 0x14031d140
0x1401630d8: mov byte ptr [rbp + 0x1a96], 0
0x1401630df: mov byte ptr [rbp + 0x1a95], 1
0x1401630e6: mov rcx, rax
0x1401630e9: call 0x1404ebfb0
0x1401630ee: mov byte ptr [rbp + 0x1a96], 0
0x1401630f5: mov byte ptr [rbp + 0x1a95], 1
0x1401630fc: lea rcx, [rbp + 0x15e0]
0x140163103: lea r8, [rbp + 0x1a50]
0x14016310a: mov rdx, r15
0x14016310d: call 0x1408c9a30
0x140163112: mov byte ptr [rbp + 0x1a96], 0
0x140163119: mov byte ptr [rbp + 0x1a95], 1
0x140163120: mov eax, dword ptr [rbp + 0x1a48]
0x140163126: mov byte ptr [rsp + 0x28], al
0x14016312a: lea rax, [rbp + 0x17b0]
0x140163131: mov qword ptr [rsp + 0x20], rax
0x140163136: lea rcx, [rbp + 0x15e0]
0x14016313d: lea r8, [rbp + 0x1a50]
0x140163144: lea r9, [rbp + 0x17e0]
0x14016314b: mov rdx, r15
0x14016314e: call 0x1408a6410
0x140163153: mov byte ptr [rbp + 0x1a96], 0
0x14016315a: mov byte ptr [rbp + 0x1a95], 1
0x140163161: movabs rdx, 0xc2b8e941c91a6428
0x14016316b: movabs r8, 0x59f3c6d1d3da43a0
0x140163175: mov rcx, r12
0x140163178: call 0x1400dd6f0
0x14016317d: test rax, rax
0x140163180: je 0x140163fda
0x140163186: cmp qword ptr [r15 + 0x218], rdx
0x14016318d: jbe 0x140163fda
0x140163193: mov rax, qword ptr [r15 + 0x210]
0x14016319a: mov rcx, qword ptr [rax + rdx*8]
0x14016319e: test rcx, rcx
0x1401631a1: je 0x140163fda
0x1401631a7: mov rax, qword ptr [r15 + 0x1e0]
0x1401631ae: not rcx
0x1401631b1: lea rcx, [rcx + rcx*2]
0x1401631b5: shl rcx, 5
0x1401631b9: cmp qword ptr [rax + rcx + 0x20], 0
0x1401631bf: je 0x140163fda
0x1401631c5: add rax, rcx
0x1401631c8: mov rax, qword ptr [rax + 0x10]
0x1401631cc: mov r14d, dword ptr [rax]
0x1401631cf: mov byte ptr [rbp + 0x1a96], 0
0x1401631d6: mov byte ptr [rbp + 0x1a95], 1
0x1401631dd: movabs rdx, 0xa4c02661e67b048b
0x1401631e7: movabs r8, 0xeae761e6aa2a67ec
0x1401631f1: mov rcx, r12
0x1401631f4: call 0x1400dd6f0
0x1401631f9: test rax, rax
0x1401631fc: je 0x140163ff9
0x140163202: cmp qword ptr [r15 + 0x218], rdx
0x140163209: jbe 0x140163ff9
0x14016320f: mov rax, qword ptr [r15 + 0x210]
0x140163216: mov rcx, qword ptr [rax + rdx*8]
0x14016321a: test rcx, rcx
0x14016321d: je 0x140163ff9
0x140163223: mov rax, qword ptr [r15 + 0x1e0]
0x14016322a: not rcx
0x14016322d: lea rcx, [rcx + rcx*2]
0x140163231: shl rcx, 5
0x140163235: cmp qword ptr [rax + rcx + 0x20], 0
0x14016323b: je 0x140163ff9
0x140163241: add rax, rcx
0x140163244: cmp r14d, 0x13
0x140163248: sete r9b
0x14016324c: mov rax, qword ptr [rax + 0x10]
0x140163250: movzx eax, byte ptr [rax + 0x20]
0x140163254: mov byte ptr [rbp + 0x1a96], 0
0x14016325b: mov byte ptr [rbp + 0x1a95], 1
0x140163262: mov byte ptr [rsp + 0x20], al
0x140163266: lea rcx, [rbp + 0x15e0]
0x14016326d: lea r8, [rbp + 0x1a50]
0x140163274: mov rdx, r15
0x140163277: call 0x1408a82f0
0x14016327c: mov esi, eax
0x14016327e: cmp r14d, 0x13
0x140163282: setne r14b
0x140163286: mov rax, qword ptr [rip + 0x34c8e0b]
0x14016328d: cmp byte ptr [rax], 1
0x140163290: jne 0x14016329e
0x140163292: add dil, 0xfd
0x140163296: cmp dil, 2
0x14016329a: jb 0x1401632b4
0x14016329c: jmp 0x140163315
0x14016329e: cmp dil, 3
0x1401632a2: jne 0x140163315
0x1401632a4: mov rax, qword ptr [rbp + 0x19f0]
0x1401632ab: cmp byte ptr [rax + 0xf3], 1
0x1401632b2: jne 0x1401632d8
0x1401632b4: mov byte ptr [rbp + 0x1a96], 0
0x1401632bb: mov byte ptr [rbp + 0x1a95], 1
0x1401632c2: lea rcx, [rbp + 0x15e0]
0x1401632c9: lea r8, [rbp + 0x1a50]
0x1401632d0: mov rdx, r15
0x1401632d3: call 0x1408a5f50
0x1401632d8: mov byte ptr [rbp + 0x1a96], 0
0x1401632df: mov byte ptr [rbp + 0x1a95], 1
0x1401632e6: lea rcx, [rbp + 0x15e0]
0x1401632ed: lea r9, [rbp + 0x1a50]
0x1401632f4: mov rdx, r15
0x1401632f7: mov r8b, 3
0x1401632fa: call 0x14031d140
0x1401632ff: mov byte ptr [rbp + 0x1a96], 0
0x140163306: mov byte ptr [rbp + 0x1a95], 1
0x14016330d: mov rcx, rax
0x140163310: call 0x1404ebfb0
0x140163315: and sil, r14b
0x140163318: jne 0x1401634af
0x14016331e: mov byte ptr [rbp + 0x1a96], 0
0x140163325: mov byte ptr [rbp + 0x1a95], 1
0x14016332c: lea rcx, [rbp + 0x15e0]
0x140163333: lea r8, [rbp + 0x1a50]
0x14016333a: mov rdx, r15
0x14016333d: call 0x1408f6c00
0x140163342: mov byte ptr [rbp + 0x1a96], 0
0x140163349: mov byte ptr [rbp + 0x1a95], 1
0x140163350: lea rcx, [rbp + 0x15e0]
0x140163357: lea r8, [rbp + 0x1a50]
0x14016335e: mov rdx, r15
0x140163361: call 0x1408b0560
0x140163366: mov byte ptr [rbp + 0x1a96], 0
0x14016336d: mov byte ptr [rbp + 0x1a95], 1
0x140163374: lea rcx, [rbp + 0x15e0]
0x14016337b: lea r8, [rbp + 0x1a50]
0x140163382: mov rdx, r15
0x140163385: call 0x1408a5a70
0x14016338a: cmp byte ptr [rbp + 0x1a80], 0
0x140163391: je 0x1401633e9
0x140163393: mov byte ptr [rbp + 0x1a96], 0
0x14016339a: mov byte ptr [rbp + 0x1a95], 1
0x1401633a1: lea rcx, [rbp + 0x15e0]
0x1401633a8: lea r8, [rbp + 0x1a50]
0x1401633af: mov rdx, r15
0x1401633b2: mov r9, qword ptr [rbp + 0x1a00]
0x1401633b9: call 0x1409ccb20
0x1401633be: mov byte ptr [rbp + 0x1a96], 0
0x1401633c5: mov byte ptr [rbp + 0x1a95], 1
0x1401633cc: lea rcx, [rbp + 0x15e0]
0x1401633d3: lea r8, [rbp + 0x1a50]
0x1401633da: mov rdx, r15
0x1401633dd: mov r9, qword ptr [rbp + 0x1a00]
0x1401633e4: call 0x140837c40
0x1401633e9: mov byte ptr [rbp + 0x1a96], 0
0x1401633f0: mov byte ptr [rbp + 0x1a95], 1
0x1401633f7: lea rcx, [rbp + 0x15e0]
0x1401633fe: lea r8, [rbp + 0x1a50]
0x140163405: mov rdx, r15
0x140163408: call 0x1409c4470
0x14016340d: mov byte ptr [rbp + 0x1a96], 0
0x140163414: mov byte ptr [rbp + 0x1a95], 1
0x14016341b: mov rcx, qword ptr [rbp + 0x17a0]
0x140163422: call 0x14093fc00
0x140163427: mov byte ptr [rbp + 0x1a96], 0
0x14016342e: mov byte ptr [rbp + 0x1a95], 1
0x140163435: lea rcx, [rbp + 0x15e0]
0x14016343c: lea r8, [rbp + 0x1a50]
0x140163443: mov rdx, r15
0x140163446: mov r9d, eax
0x140163449: call 0x1409c6700
0x14016344e: mov byte ptr [rbp + 0x1a96], 0
0x140163455: mov byte ptr [rbp + 0x1a95], 1
0x14016345c: lea rcx, [rbp + 0x15e0]
0x140163463: lea r8, [rbp + 0x1a50]
0x14016346a: mov rdx, r15
0x14016346d: call 0x1409d1710
0x140163472: mov byte ptr [rbp + 0x1a96], 0
0x140163479: mov byte ptr [rbp + 0x1a95], 1
0x140163480: lea rcx, [rbp + 0x15e0]
0x140163487: lea r9, [rbp + 0x1a50]
0x14016348e: mov rdx, r15
0x140163491: mov r8b, 4
0x140163494: call 0x14031d140
0x140163499: mov byte ptr [rbp + 0x1a96], 0
0x1401634a0: mov byte ptr [rbp + 0x1a95], 1
0x1401634a7: mov rcx, rax
0x1401634aa: call 0x1404ebfb0
0x1401634af: mov byte ptr [rbp + 0x1a96], 0
0x1401634b6: mov byte ptr [rbp + 0x1a95], 1
0x1401634bd: lea rcx, [rbp + 0x15e0]
0x1401634c4: lea r8, [rbp + 0x1a50]
0x1401634cb: mov rdx, r15
0x1401634ce: mov r9, qword ptr [rbp + 0x1a00]
0x1401634d5: call 0x140837ff0
0x1401634da: cmp qword ptr [rbp + 0x19b8], 0
0x1401634e2: je 0x14016350f
0x1401634e4: mov byte ptr [rbp + 0x1a96], 0
0x1401634eb: mov byte ptr [rbp + 0x1a95], 1
0x1401634f2: lea rdx, [rbp + 0x15e0]
0x1401634f9: lea r9, [rbp + 0x1a50]
0x140163500: mov rcx, qword ptr [rbp + 0x19b8]
0x140163507: mov r8, r15
0x14016350a: call 0x1400ddc80
0x14016350f: mov byte ptr [rbp + 0x1a96], 0
0x140163516: mov byte ptr [rbp + 0x1a95], 1
0x14016351d: lea rcx, [rbp + 0x15e0]
0x140163524: lea r9, [rbp + 0x1a50]
0x14016352b: mov rdx, r15
0x14016352e: mov r8b, 6
0x140163531: call 0x14031d140
0x140163536: mov byte ptr [rbp + 0x1a96], 0
0x14016353d: mov byte ptr [rbp + 0x1a95], 1
0x140163544: mov rcx, rax
0x140163547: call 0x1404ebfb0
0x14016354c: mov byte ptr [rbp + 0x1a96], 0
0x140163553: mov byte ptr [rbp + 0x1a95], 1
0x14016355a: lea rcx, [rbp + 0x15e0]
0x140163561: lea r8, [rbp + 0x1a50]
0x140163568: mov rdx, r15
0x14016356b: call 0x140589c90
0x140163570: movups xmm0, xmmword ptr [rbp + 0x1a50]
0x140163577: movups xmm1, xmmword ptr [rbp + 0x1a60]
0x14016357e: movaps xmmword ptr [rbp + 0x1810], xmm0
0x140163585: mov eax, dword ptr [rbp + 0x1a70]
0x14016358b: mov dword ptr [rbp + 0x1830], eax
0x140163591: movaps xmmword ptr [rbp + 0x1820], xmm1
0x140163598: mov r14d, dword ptr [rbp + 0x1810]
0x14016359f: mov byte ptr [rbp + 0x1a96], 0
0x1401635a6: mov byte ptr [rbp + 0x1a95], 1
0x1401635ad: lea rcx, [rbp + 0x1810]
0x1401635b4: lea rdx, [rbp + 0x15e0]
0x1401635bb: mov r8b, 0x22
0x1401635be: call 0x140bd6380
0x1401635c3: lea rsi, [rbp + 0x1810]
0x1401635ca: lea rdx, [rbp + 0x15e0]
0x1401635d1: mov r8d, 0x198
0x1401635d7: mov rcx, rsi
0x1401635da: call 0x142923c00
0x1401635df: mov byte ptr [rbp + 0x1a96], 0
0x1401635e6: mov byte ptr [rbp + 0x1a95], 0
0x1401635ed: lea rcx, [rbp + 0x1450]
0x1401635f4: mov rdx, rsi
0x1401635f7: call 0x1409dda50
0x1401635fc: movabs rdx, 0xa68561b396784eaa
0x140163606: movabs r8, 0xf5fd74e6faf4ed14
0x140163610: mov rcx, r12
0x140163613: call 0x1400dd6f0
0x140163618: mov rdi, qword ptr [rbp + 0x19b0]
0x14016361f: mov r13, qword ptr [rbp + 0x1788]
0x140163626: test rax, rax
0x140163629: je 0x140164018
0x14016362f: cmp qword ptr [r15 + 0x218], rdx
0x140163636: jbe 0x140164018
0x14016363c: mov rax, qword ptr [r15 + 0x210]
0x140163643: mov rcx, qword ptr [rax + rdx*8]
0x140163647: test rcx, rcx
0x14016364a: je 0x140164018
0x140163650: mov rax, qword ptr [r15 + 0x1e0]
0x140163657: not rcx
0x14016365a: lea rcx, [rcx + rcx*2]
0x14016365e: shl rcx, 5
0x140163662: cmp qword ptr [rax + rcx + 0x20], 0
0x140163668: je 0x140164018
0x14016366e: add rax, rcx
0x140163671: mov rcx, qword ptr [rax + 0x10]
0x140163675: mov edx, dword ptr [r15 + 0x5b0]
0x14016367c: mov dword ptr [rax + 0x5c], edx
0x14016367f: lea rdx, [rbp + 0x1450]
0x140163686: call 0x14093ce30
0x14016368b: movabs rdx, 0x8226cc786e679355
0x140163695: movabs r8, 0x65cfda6a55a841ac
0x14016369f: mov rcx, r12
0x1401636a2: call 0x1400dd6f0
0x1401636a7: test rax, rax
0x1401636aa: je 0x140164029
0x1401636b0: cmp qword ptr [r15 + 0x218], rdx
0x1401636b7: jbe 0x140164029
0x1401636bd: mov rax, qword ptr [r15 + 0x210]
0x1401636c4: mov rcx, qword ptr [rax + rdx*8]
0x1401636c8: test rcx, rcx
0x1401636cb: je 0x140164029
0x1401636d1: mov rax, qword ptr [r15 + 0x1e0]
0x1401636d8: not rcx
0x1401636db: lea rcx, [rcx + rcx*2]
0x1401636df: shl rcx, 5
0x1401636e3: cmp qword ptr [rax + rcx + 0x20], 0
0x1401636e9: je 0x140164029
0x1401636ef: add rax, rcx
0x1401636f2: mov rcx, qword ptr [rax + 0x10]
0x1401636f6: mov edx, dword ptr [r15 + 0x5b0]
0x1401636fd: mov dword ptr [rax + 0x5c], edx
0x140163700: lea rdx, [rbp + 0x1450]
0x140163707: call 0x14091ebb0
0x14016370c: movabs rdx, 0x662978458699a5ef
0x140163716: movabs r8, 0x3ec057c162419b46
0x140163720: mov rcx, r12
0x140163723: call 0x1400dd6f0
0x140163728: test rax, rax
0x14016372b: je 0x14016403a
0x140163731: cmp qword ptr [r15 + 0x218], rdx
0x140163738: jbe 0x14016403a
0x14016373e: mov rax, qword ptr [r15 + 0x210]
0x140163745: mov rcx, qword ptr [rax + rdx*8]
0x140163749: test rcx, rcx
0x14016374c: je 0x14016403a
0x140163752: mov rax, qword ptr [r15 + 0x1e0]
0x140163759: not rcx
0x14016375c: lea rcx, [rcx + rcx*2]
0x140163760: shl rcx, 5
0x140163764: cmp qword ptr [rax + rcx + 0x20], 0
0x14016376a: je 0x14016403a
0x140163770: add rax, rcx
0x140163773: mov rcx, qword ptr [rax + 0x10]
0x140163777: mov edx, dword ptr [r15 + 0x5b0]
0x14016377e: mov dword ptr [rax + 0x5c], edx
0x140163781: lea rdx, [rbp + 0x1450]
0x140163788: call 0x140992220
0x14016378d: lea rcx, [rbp + 0x1810]
0x140163794: lea rdx, [rbp + 0x1450]
0x14016379b: mov r8d, 0x110
0x1401637a1: call 0x142923c00
0x1401637a6: mov rdx, qword ptr [rbp + 0x1a10]
0x1401637ad: test rdx, rdx
0x1401637b0: je 0x1401637c4
0x1401637b2: mov rcx, qword ptr [rbp + 0x1a18]
0x1401637b9: mov r8d, 1
0x1401637bf: call 0x140613c20
0x1401637c4: lea rsi, [rbp - 0x30]
0x1401637c8: lea rdx, [rbp + 0x1810]
0x1401637cf: mov r8d, 0x110
0x1401637d5: mov rcx, rsi
0x1401637d8: call 0x142923c00
0x1401637dd: lea rcx, [rbp + 0xe0]
0x1401637e4: mov rdx, rsi
0x1401637e7: call 0x1404f7ca0
0x1401637ec: lea rcx, [rbp + 0x1450]
0x1401637f3: lea rdx, [rbp + 0xe0]
0x1401637fa: mov r8d, 0xa8
0x140163800: call 0x142923c00
0x140163805: movups xmm0, xmmword ptr [rbp + 0x188]
0x14016380c: movups xmm1, xmmword ptr [rbp + 0x198]
0x140163813: movups xmm2, xmmword ptr [rbp + 0x1a8]
0x14016381a: movups xmm3, xmmword ptr [rbp + 0x1b8]
0x140163821: movaps xmmword ptr [rbp + 0x13f0], xmm0
0x140163828: movaps xmmword ptr [rbp + 0x1400], xmm1
0x14016382f: movaps xmmword ptr [rbp + 0x1410], xmm2
0x140163836: movaps xmmword ptr [rbp + 0x1420], xmm3
0x14016383d: movups xmm0, xmmword ptr [rbp + 0x1c8]
0x140163844: movaps xmmword ptr [rbp + 0x1430], xmm0
0x14016384b: movups xmm0, xmmword ptr [rbp + 0x1d8]
0x140163852: movaps xmmword ptr [rbp + 0x1440], xmm0
0x140163859: movups xmm0, xmmword ptr [rbx + 0x50]
0x14016385d: movaps xmmword ptr [rbp + 0x130], xmm0
0x140163864: movups xmm0, xmmword ptr [rbx + 0x40]
0x140163868: movaps xmmword ptr [rbp + 0x120], xmm0
0x14016386f: movups xmm0, xmmword ptr [rbx]
0x140163872: movups xmm1, xmmword ptr [rbx + 0x10]
0x140163876: movups xmm2, xmmword ptr [rbx + 0x20]
0x14016387a: movups xmm3, xmmword ptr [rbx + 0x30]
0x14016387e: movaps xmmword ptr [rbp + 0x110], xmm3
0x140163885: movaps xmmword ptr [rbp + 0x100], xmm2
0x14016388c: movaps xmmword ptr [rbp + 0xf0], xmm1
0x140163893: movaps xmmword ptr [rbp + 0xe0], xmm0
0x14016389a: movaps xmm0, xmmword ptr [rbp + 0x13f0]
0x1401638a1: movaps xmm1, xmmword ptr [rbp + 0x1400]
0x1401638a8: movaps xmm2, xmmword ptr [rbp + 0x1410]
0x1401638af: movaps xmm3, xmmword ptr [rbp + 0x1420]
0x1401638b6: movups xmmword ptr [rbx], xmm0
0x1401638b9: movups xmmword ptr [rbx + 0x10], xmm1
0x1401638bd: movups xmmword ptr [rbx + 0x20], xmm2
0x1401638c1: movups xmmword ptr [rbx + 0x30], xmm3
0x1401638c5: movaps xmm0, xmmword ptr [rbp + 0x1430]
0x1401638cc: movups xmmword ptr [rbx + 0x40], xmm0
0x1401638d0: movaps xmm0, xmmword ptr [rbp + 0x1440]
0x1401638d7: movups xmmword ptr [rbx + 0x50], xmm0
0x1401638db: cmp qword ptr [rbp + 0xe0], 0
0x1401638e3: mov rbx, qword ptr [rbp + 0x1790]
0x1401638ea: je 0x140163943
0x1401638ec: lea rdx, [rbp + 0x100]
0x1401638f3: lea rcx, [rbp + 0xe0]
0x1401638fa: mov r8d, 0x68
0x140163900: mov r9d, 0x10
0x140163906: call 0x14004ceb0
0x14016390b: mov rdx, qword ptr [rbp + 0x118]
0x140163912: test rdx, rdx
0x140163915: je 0x140163943
0x140163917: mov rax, rdx
0x14016391a: shl rax, 4
0x14016391e: add rdx, rax
0x140163921: add rdx, 0x21
0x140163925: je 0x140163943
0x140163927: mov rcx, qword ptr [rbp + 0x110]
0x14016392e: neg rax
0x140163931: add rcx, rax
0x140163934: add rcx, -0x10
0x140163938: mov r8d, 0x10
0x14016393e: call 0x140613c20
0x140163943: lea rcx, [rbp + 0x1338]
0x14016394a: lea rdx, [rbp + 0x1450]
0x140163951: mov r8d, 0x90
0x140163957: call 0x142923c00
0x14016395c: mov rax, qword ptr [rbp + 0x14f0]
0x140163963: mov qword ptr [rbp + 0x1a60], rax
0x14016396a: movups xmm0, xmmword ptr [rbp + 0x14e0]
0x140163971: movaps xmmword ptr [rbp + 0x1a50], xmm0
0x140163978: mov qword ptr [rbp + 0xe0], 0
0x140163983: mov qword ptr [rbp + 0xe8], 8
0x14016398e: mov qword ptr [rbp + 0xf0], 0
0x140163999: xor ecx, ecx
0x14016399b: call 0x1400d9b20
0x1401639a0: test rax, rax
0x1401639a3: je 0x14016408c
0x1401639a9: movups xmm0, xmmword ptr [rax]
0x1401639ac: inc qword ptr [rax]
0x1401639af: movups xmm1, xmmword ptr [rip + 0x284c67a]
0x1401639b6: movups xmmword ptr [rbp + 0x16c8], xmm1
0x1401639bd: movups xmm1, xmmword ptr [rip + 0x284c65c]
0x1401639c4: movups xmmword ptr [rbp + 0x16b8], xmm1
0x1401639cb: movups xmmword ptr [rbp + 0x16d8], xmm0
0x1401639d2: mov qword ptr [rbp + 0x15e0], 0
0x1401639dd: mov qword ptr [rbp + 0x15e8], 2
0x1401639e8: xorps xmm0, xmm0
0x1401639eb: movups xmmword ptr [rbp + 0x15f0], xmm0
0x1401639f2: mov qword ptr [rbp + 0x1600], 1
0x1401639fd: mov qword ptr [rbp + 0x1608], 0
0x140163a08: movups xmm1, xmmword ptr [rbp + 0xe0]
0x140163a0f: movups xmmword ptr [rbp + 0x1610], xmm1
0x140163a16: mov rax, qword ptr [rbp + 0xf0]
0x140163a1d: mov qword ptr [rbp + 0x1620], rax
0x140163a24: lea rcx, [rbp + 0x1628]
0x140163a2b: mov qword ptr [rbp + 0x1628], 0
0x140163a36: mov qword ptr [rbp + 0x1630], 4
0x140163a41: movups xmmword ptr [rbp + 0x1638], xmm0
0x140163a48: mov qword ptr [rbp + 0x1648], 4
0x140163a53: movups xmmword ptr [rbp + 0x1650], xmm0
0x140163a5a: mov qword ptr [rbp + 0x1660], 4
0x140163a65: movups xmmword ptr [rbp + 0x1668], xmm0
0x140163a6c: mov qword ptr [rbp + 0x1678], 4
0x140163a77: movups xmmword ptr [rbp + 0x1680], xmm0
0x140163a7e: mov qword ptr [rbp + 0x1690], 4
0x140163a89: movups xmmword ptr [rbp + 0x1698], xmm0
0x140163a90: mov qword ptr [rbp + 0x16a8], 4
0x140163a9b: mov qword ptr [rbp + 0x16b0], 0
0x140163aa6: mov r8, qword ptr [rbp + 0x1a58]
0x140163aad: mov r9, qword ptr [rbp + 0x1a60]
0x140163ab4: mov byte ptr [rbp + 0x1a90], 1
0x140163abb: mov byte ptr [rbp + 0x1a8f], 1
0x140163ac2: lea rdx, [rbp + 0x1338]
0x140163ac9: call 0x140530430
0x140163ace: mov byte ptr [rbp + 0x1a90], 1
0x140163ad5: mov byte ptr [rbp + 0x1a8f], 1
0x140163adc: lea rcx, [rbp + 0x15e0]
0x140163ae3: lea rdx, [rbp + 0x1338]
0x140163aea: mov r8, qword ptr [rbp + 0x1798]
0x140163af1: mov r9, r13
0x140163af4: call 0x140be7ba0
0x140163af9: test al, al
0x140163afb: je 0x140163ba9
0x140163b01: mov byte ptr [rbp + 0x1a90], 1
0x140163b08: mov byte ptr [rbp + 0x1a8f], 1
0x140163b0f: lea rcx, [rbp + 0x1338]
0x140163b16: call 0x140be9420
0x140163b1b: lea rcx, [rbp + 0xe0]
0x140163b22: mov r8d, 0x90
0x140163b28: mov rdx, rdi
0x140163b2b: call 0x142923c00
0x140163b30: lea rdx, [rbp + 0x1338]
0x140163b37: mov r8d, 0x90
0x140163b3d: mov rcx, rdi
0x140163b40: call 0x142923c00
0x140163b45: xor eax, eax
0x140163b47: cmp rax, qword ptr [rbp + 0xe0]
0x140163b4e: jo 0x140163b6a
0x140163b50: mov byte ptr [rbp + 0x1a90], 0
0x140163b57: mov byte ptr [rbp + 0x1a8f], 1
0x140163b5e: lea rcx, [rbp + 0xe0]
0x140163b65: call 0x1405ef210
0x140163b6a: movabs rax, 0x8000000000000000
0x140163b74: mov qword ptr [rbx], rax
0x140163b77: mov byte ptr [rbp + 0x1a92], 0
0x140163b7e: mov byte ptr [rbp + 0x1a91], 1
0x140163b85: lea rcx, [rbp + 0x15e0]
0x140163b8c: call 0x1405fd1d0
0x140163b91: mov byte ptr [rbp + 0x1a8b], 0
0x140163b98: lea rcx, [rbp + 0x1a50]
0x140163b9f: call 0x1405da820
0x140163ba4: jmp 0x140163c40
0x140163ba9: lea rsi, [rbp + 0x1228]
0x140163bb0: lea rdx, [rbp + 0x1338]
0x140163bb7: mov r8d, 0x90
0x140163bbd: mov rcx, rsi
0x140163bc0: call 0x142923c00
0x140163bc5: mov rax, qword ptr [rbp + 0x1a60]
0x140163bcc: mov qword ptr [rbp + 0x1a20], rax
0x140163bd3: movaps xmm0, xmmword ptr [rbp + 0x1a50]
0x140163bda: movaps xmmword ptr [rbp + 0x1a10], xmm0
0x140163be1: lea rdi, [rbp + 0x1810]
0x140163be8: lea rdx, [rbp + 0x15e0]
0x140163bef: mov r8d, 0x108
0x140163bf5: mov rcx, rdi
0x140163bf8: call 0x142923c00
0x140163bfd: mov byte ptr [rbp + 0x1a90], 0
0x140163c04: mov byte ptr [rbp + 0x1a8f], 0
0x140163c0b: lea rcx, [rbp + 0xe0]
0x140163c12: lea r8, [rbp + 0x1a10]
0x140163c19: mov rdx, rsi
0x140163c1c: mov r9, rdi
0x140163c1f: call 0x1402c4280
0x140163c24: lea rdx, [rbp + 0xe0]
0x140163c2b: mov r8d, 0x218
0x140163c31: mov rcx, rbx
0x140163c34: call 0x142923c00
0x140163c39: mov dword ptr [rbx + 0x218], r14d
0x140163c40: mov rax, rbx
0x140163c43: movaps xmm6, xmmword ptr [rbp + 0x1aa0]
0x140163c4a: add rsp, 0x1b38
0x140163c51: pop rbx
0x140163c52: pop rdi
0x140163c53: pop rsi
0x140163c54: pop r12
0x140163c56: pop r13
0x140163c58: pop r14
0x140163c5a: pop r15
0x140163c5c: pop rbp
0x140163c5d: ret 
0x140163c5e: mov byte ptr [rbp + 0x1a97], 1
0x140163c65: mov rcx, rsi
0x140163c68: call 0x14093fc60
0x140163c6d: lea rcx, [rip + 0x27f9ae8]
0x140163c74: mov qword ptr [rbp + 0x1a10], rcx
0x140163c7b: mov qword ptr [rbp + 0x1a18], 0x1f
0x140163c86: test al, al
0x140163c88: jne 0x1401627be
0x140163c8e: mov rax, qword ptr [rip + 0x34c8713]
0x140163c95: mov rax, qword ptr [rax]
0x140163c98: cmp rax, 5
0x140163c9c: jb 0x140162840
0x140163ca2: mov qword ptr [rbp + 0x1450], r13
0x140163ca9: lea rax, [rip - 0x59e20]
0x140163cb0: mov qword ptr [rbp + 0x1458], rax
0x140163cb7: mov byte ptr [rbp + 0x1a97], 1
0x140163cbe: lea rcx, [rip + 0x2887e93]
0x140163cc5: call 0x14067dd80
0x140163cca: movups xmm0, xmmword ptr [rax]
0x140163ccd: mov eax, dword ptr [rax + 0x10]
0x140163cd0: mov qword ptr [rbp + 0x1840], 5
0x140163cdb: lea rcx, [rip + 0x2887e8e]
0x140163ce2: mov qword ptr [rbp + 0x1848], rcx
0x140163ce9: mov qword ptr [rbp + 0x1850], 0x17
0x140163cf4: lea rdx, [rip + 0x2887e25]
0x140163cfb: mov qword ptr [rbp + 0x1860], rdx
0x140163d02: mov qword ptr [rbp + 0x1868], 1
0x140163d0d: lea rdx, [rbp + 0x1450]
0x140163d14: mov qword ptr [rbp + 0x1870], rdx
0x140163d1b: mov qword ptr [rbp + 0x1878], 1
0x140163d26: mov qword ptr [rbp + 0x1880], 0
0x140163d31: mov qword ptr [rbp + 0x1810], 0
0x140163d3c: mov qword ptr [rbp + 0x1818], rcx
0x140163d43: mov qword ptr [rbp + 0x1820], 0x17
0x140163d4e: mov qword ptr [rbp + 0x1828], 0
0x140163d59: movups xmmword ptr [rbp + 0x1830], xmm0
0x140163d60: mov dword ptr [rbp + 0x1858], 1
0x140163d6a: mov dword ptr [rbp + 0x185c], eax
0x140163d70: mov byte ptr [rbp + 0x1a97], 1
0x140163d77: lea rcx, [rbp + 0x1a50]
0x140163d7e: lea rdx, [rbp + 0x1810]
0x140163d85: call 0x140d4a330
0x140163d8a: jmp 0x140162840
0x140163d8f: mov rax, qword ptr [rip + 0x34c8612]
0x140163d96: mov rax, qword ptr [rax]
0x140163d99: cmp rax, 5
0x140163d9d: jb 0x140162840
0x140163da3: mov qword ptr [rbp + 0x1450], r13
0x140163daa: lea rax, [rip - 0x59f21]
0x140163db1: mov qword ptr [rbp + 0x1458], rax
0x140163db8: mov byte ptr [rbp + 0x1a97], 1
0x140163dbf: lea rcx, [rip + 0x2887d92]
0x140163dc6: call 0x14067dd80
0x140163dcb: movups xmm0, xmmword ptr [rax]
0x140163dce: mov eax, dword ptr [rax + 0x10]
0x140163dd1: mov qword ptr [rbp + 0x1840], 5
0x140163ddc: lea rcx, [rip + 0x2887d8d]
0x140163de3: mov qword ptr [rbp + 0x1848], rcx
0x140163dea: mov qword ptr [rbp + 0x1850], 0x17
0x140163df5: lea rdx, [rip + 0x2887d24]
0x140163dfc: mov qword ptr [rbp + 0x1860], rdx
0x140163e03: mov qword ptr [rbp + 0x1868], 1
0x140163e0e: lea rdx, [rbp + 0x1450]
0x140163e15: mov qword ptr [rbp + 0x1870], rdx
0x140163e1c: mov qword ptr [rbp + 0x1878], 1
0x140163e27: mov qword ptr [rbp + 0x1880], 0
0x140163e32: mov qword ptr [rbp + 0x1810], 0
0x140163e3d: mov qword ptr [rbp + 0x1818], rcx
0x140163e44: mov qword ptr [rbp + 0x1820], 0x17
0x140163e4f: mov qword ptr [rbp + 0x1828], 0
0x140163e5a: movups xmmword ptr [rbp + 0x1830], xmm0
0x140163e61: mov dword ptr [rbp + 0x1858], 1
0x140163e6b: mov dword ptr [rbp + 0x185c], eax
0x140163e71: mov byte ptr [rbp + 0x1a97], 1
0x140163e78: lea rcx, [rbp + 0x1a50]
0x140163e7f: lea rdx, [rbp + 0x1810]
0x140163e86: call 0x140d4a330
0x140163e8b: jmp 0x140162840
0x140163e90: mov byte ptr [rbp + 0x1a94], 1
0x140163e97: mov rcx, rdi
0x140163e9a: mov r8d, 0x3b9aca00
0x140163ea0: call 0x1428cd210
0x140163ea5: jmp 0x1401620d4
0x140163eaa: lea r8, [rip + 0x287a9ff]
0x140163eb1: mov edx, 0x1090
0x140163eb6: call 0x1428d98e0
0x140163ebb: jmp 0x1401640c2
0x140163ec0: mov byte ptr [rbp + 0x1a8e], 1
0x140163ec7: mov rcx, rdi
0x140163eca: xor edx, edx
0x140163ecc: call 0x1428cdb90
0x140163ed1: jmp 0x1401621bc
0x140163ed6: mov byte ptr [rbp + 0x1a97], 1
0x140163edd: lea rcx, [rip + 0x34c716c]
0x140163ee4: mov r8d, 0x3b9aca00
0x140163eea: call 0x1428cd210
0x140163eef: jmp 0x140162c36
0x140163ef4: mov byte ptr [rbp + 0x1a97], 1
0x140163efb: lea rcx, [rip + 0x34c714e]
0x140163f02: xor edx, edx
0x140163f04: call 0x1428cdb90
0x140163f09: jmp 0x140162ce0
0x140163f0e: mov byte ptr [rbp + 0x1a93], 1
0x140163f15: lea rcx, [rip + 0x27f9714]
0x140163f1c: call 0x1428d9310
0x140163f21: jmp 0x1401640c2
0x140163f26: mov byte ptr [rbp + 0x1a94], 1
0x140163f2d: lea rcx, [rip + 0x27f9714]
0x140163f34: call 0x1428d9310
0x140163f39: jmp 0x1401640c2
0x140163f3e: mov byte ptr [rbp + 0x1a94], 1
0x140163f45: lea rcx, [rip + 0x27f9714]
0x140163f4c: call 0x1428d9310
0x140163f51: jmp 0x1401640c2
0x140163f56: mov byte ptr [rbp + 0x1a94], 1
0x140163f5d: lea rcx, [rip + 0x27f9714]
0x140163f64: call 0x1428d9310
0x140163f69: jmp 0x1401640c2
0x140163f6e: mov byte ptr [rbp + 0x1a97], 1
0x140163f75: lea rcx, [rip + 0x27f9714]
0x140163f7c: call 0x1428d9310
0x140163f81: jmp 0x1401640c2
0x140163f86: mov byte ptr [rbp + 0x1a97], 1
0x140163f8d: lea rcx, [rip + 0x27f9724]
0x140163f94: call 0x1428d9310
0x140163f99: jmp 0x1401640c2
0x140163f9e: mov byte ptr [rbp + 0x1a97], 1
0x140163fa5: lea rcx, [rip + 0x27f9724]
0x140163fac: call 0x1428d9310
0x140163fb1: jmp 0x1401640c2
0x140163fb6: mov byte ptr [rbp + 0x1a97], 1
0x140163fbd: lea rcx, [rip + 0x27f97de]
0x140163fc4: lea r8, [rip + 0x27f97ed]
0x140163fcb: mov edx, 0x15
0x140163fd0: call 0x1428d9330
0x140163fd5: jmp 0x1401640c2
0x140163fda: mov byte ptr [rbp + 0x1a96], 0
0x140163fe1: mov byte ptr [rbp + 0x1a95], 1
0x140163fe8: lea rcx, [rip + 0x27f9841]
0x140163fef: call 0x1428d9310
0x140163ff4: jmp 0x1401640c2
0x140163ff9: mov byte ptr [rbp + 0x1a96], 0
0x140164000: mov byte ptr [rbp + 0x1a95], 1
0x140164007: lea rcx, [rip + 0x27f983a]
0x14016400e: call 0x1428d9310
0x140164013: jmp 0x1401640c2
0x140164018: lea rcx, [rip + 0x27f9841]
0x14016401f: call 0x1428d9310
0x140164024: jmp 0x1401640c2
0x140164029: lea rcx, [rip + 0x27f9848]
0x140164030: call 0x1428d9310
0x140164035: jmp 0x1401640c2
0x14016403a: lea rcx, [rip + 0x27f984f]
0x140164041: call 0x1428d9310
0x140164046: jmp 0x1401640c2
0x140164048: mov byte ptr [rbp + 0x1a97], 1
0x14016404f: lea rcx, [rip + 0x27f972a]
0x140164056: call 0x1428d9310
0x14016405b: jmp 0x1401640c2
0x14016405d: lea rcx, [rip + 0x27f9574]
0x140164064: call 0x1428d9310
0x140164069: lea rcx, [rip + 0x27f9580]
0x140164070: call 0x1428d9310
0x140164075: jmp 0x1401640c2
0x140164077: mov byte ptr [rbp + 0x1a97], 1
0x14016407e: lea rcx, [rip + 0x27f9663]
0x140164085: call 0x1428d9310
0x14016408a: jmp 0x1401640c2
0x14016408c: lea rcx, [rip + 0x27ce97d]
0x140164093: call 0x1428d80d0
0x140164098: jmp 0x1401640c2
0x14016409a: mov byte ptr [rbp + 0x1a97], 1
0x1401640a1: lea rcx, [rip + 0x27f9688]
0x1401640a8: call 0x1428d9310
0x1401640ad: jmp 0x1401640c2
0x1401640af: mov byte ptr [rbp + 0x1a97], 1
0x1401640b6: lea rcx, [rip + 0x27f9643]
0x1401640bd: call 0x1428d9310
0x1401640c2: ud2 
0x1401640c4: nop word ptr cs:[rax + rax]
0x1401640d0: mov qword ptr [rsp + 0x10], rdx
0x1401640d5: push rbp
0x1401640d6: push r15
0x1401640d8: push r14
0x1401640da: push r13
0x1401640dc: push r12
0x1401640de: push rsi
0x1401640df: push rdi
0x1401640e0: push rbx
0x1401640e1: sub rsp, 0x68
0x1401640e5: lea rbp, [rdx + 0x80]
0x1401640ec: movaps xmmword ptr [rsp + 0x50], xmm6
0x1401640f1: movups xmm0, xmmword ptr [rbp + 0x1830]
0x1401640f8: movups xmm1, xmmword ptr [rbp + 0x1840]
0x1401640ff: movaps xmmword ptr [rbp + 0x15f0], xmm1
0x140164106: movaps xmmword ptr [rbp + 0x15e0], xmm0
0x14016410d: lea rcx, [rbp + 0x15e0]
0x140164114: call 0x140be24b0
0x140164119: movaps xmm6, xmmword ptr [rsp + 0x50]
0x14016411e: add rsp, 0x68
0x140164122: pop rbx
0x140164123: pop rdi
0x140164124: pop rsi
0x140164125: pop r12
0x140164127: pop r13
0x140164129: pop r14
0x14016412b: pop r15
0x14016412d: pop rbp
0x14016412e: ret 
0x14016412f: nop 
0x140164130: mov qword ptr [rsp + 0x10], rdx
0x140164135: push rbp
0x140164136: push r15
0x140164138: push r14
0x14016413a: push r13
0x14016413c: push r12
0x14016413e: push rsi
0x14016413f: push rdi
0x140164140: push rbx
0x140164141: sub rsp, 0x68
0x140164145: lea rbp, [rdx + 0x80]
0x14016414c: movaps xmmword ptr [rsp + 0x50], xmm6
0x140164151: movzx ebx, byte ptr [rbp + 0x1a8d]
0x140164158: mov rdx, qword ptr [rbp + 0x1a10]
0x14016415f: test rdx, rdx
0x140164162: je 0x140164176
0x140164164: mov rcx, qword ptr [rbp + 0x1a18]
0x14016416b: mov r8d, 1
0x140164171: call 0x140613c20
0x140164176: and bl, 1
0x140164179: mov byte ptr [rbp + 0x1a97], bl
0x14016417f: movaps xmm6, xmmword ptr [rsp + 0x50]
0x140164184: add rsp, 0x68
0x140164188: pop rbx
0x140164189: pop rdi
0x14016418a: pop rsi
0x14016418b: pop r12
0x14016418d: pop r13
0x14016418f: pop r14
0x140164191: pop r15
0x140164193: pop rbp
0x140164194: ret 
0x140164195: nop word ptr cs:[rax + rax]
0x1401641a0: mov qword ptr [rsp + 0x10], rdx
0x1401641a5: push rbp
0x1401641a6: push r15
0x1401641a8: push r14
0x1401641aa: push r13
0x1401641ac: push r12
0x1401641ae: push rsi
0x1401641af: push rdi
0x1401641b0: push rbx
0x1401641b1: sub rsp, 0x68
0x1401641b5: lea rbp, [rdx + 0x80]
0x1401641bc: movaps xmmword ptr [rsp + 0x50], xmm6
0x1401641c1: movzx ebx, byte ptr [rbp + 0x1a95]
0x1401641c8: cmp byte ptr [rbp + 0x1a96], 0
0x1401641cf: je 0x14016421d
0x1401641d1: mov rax, qword ptr [rbp + 0x1a08]
0x1401641d8: lea rax, [rax*2]
0x1401641e0: test rax, rax
0x1401641e3: je 0x1401641fe
0x1401641e5: mov r8d, 1
0x1401641eb: mov rcx, qword ptr [rbp + 0x19f8]
0x1401641f2: mov rdx, qword ptr [rbp + 0x1a08]
0x1401641f9: call 0x140613c20
0x1401641fe: and bl, 1
0x140164201: mov byte ptr [rbp + 0x1a8d], bl
0x140164207: movaps xmm6, xmmword ptr [rsp + 0x50]
0x14016420c: add rsp, 0x68
0x140164210: pop rbx
0x140164211: pop rdi
0x140164212: pop rsi
0x140164213: pop r12
0x140164215: pop r13
0x140164217: pop r14
0x140164219: pop r15
0x14016421b: pop rbp
0x14016421c: ret 
0x14016421d: and bl, 1
0x140164220: mov byte ptr [rbp + 0x1a8d], bl
0x140164226: movaps xmm6, xmmword ptr [rsp + 0x50]
0x14016422b: add rsp, 0x68
0x14016422f: pop rbx
0x140164230: pop rdi
0x140164231: pop rsi
0x140164232: pop r12
0x140164234: pop r13
0x140164236: pop r14
0x140164238: pop r15
0x14016423a: pop rbp
0x14016423b: ret 
0x14016423c: nop dword ptr [rax]
0x140164240: mov qword ptr [rsp + 0x10], rdx
0x140164245: push rbp
0x140164246: push r15
0x140164248: push r14
0x14016424a: push r13
0x14016424c: push r12
0x14016424e: push rsi
0x14016424f: push rdi
0x140164250: push rbx
0x140164251: sub rsp, 0x68
0x140164255: lea rbp, [rdx + 0x80]
0x14016425c: movaps xmmword ptr [rsp + 0x50], xmm6
0x140164261: cmp byte ptr [rbp + 0x1a97], 0
0x140164268: je 0x140164293
0x14016426a: lea rcx, [rbp + 0x15e0]
0x140164271: call 0x1405fcbe0
0x140164276: mov byte ptr [rbp + 0x1a8e], 0
0x14016427d: movaps xmm6, xmmword ptr [rsp + 0x50]
0x140164282: add rsp, 0x68
0x140164286: pop rbx
0x140164287: pop rdi
0x140164288: pop rsi
0x140164289: pop r12
0x14016428b: pop r13
0x14016428d: pop r14
0x14016428f: pop r15
0x140164291: pop rbp
0x140164292: ret 
0x140164293: mov byte ptr [rbp + 0x1a8e], 0
0x14016429a: movaps xmm6, xmmword ptr [rsp + 0x50]
0x14016429f: add rsp, 0x68
0x1401642a3: pop rbx
0x1401642a4: pop rdi
0x1401642a5: pop rsi
0x1401642a6: pop r12
0x1401642a8: pop r13
0x1401642aa: pop r14
0x1401642ac: pop r15
0x1401642ae: pop rbp
0x1401642af: ret 
0x1401642b0: mov qword ptr [rsp + 0x10], rdx
0x1401642b5: push rbp
0x1401642b6: push r15
0x1401642b8: push r14
0x1401642ba: push r13
0x1401642bc: push r12
0x1401642be: push rsi
0x1401642bf: push rdi
0x1401642c0: push rbx
0x1401642c1: sub rsp, 0x68
0x1401642c5: lea rbp, [rdx + 0x80]
0x1401642cc: movaps xmmword ptr [rsp + 0x50], xmm6
0x1401642d1: movzx ebx, byte ptr [rbp + 0x1a8e]
0x1401642d8: test bl, 1
0x1401642db: je 0x140164305
0x1401642dd: lea rcx, [rbp + 0x1338]
0x1401642e4: call 0x140161b50
0x1401642e9: mov byte ptr [rbp + 0x1a94], bl
0x1401642ef: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1401642f4: add rsp, 0x68
0x1401642f8: pop rbx
0x1401642f9: pop rdi
0x1401642fa: pop rsi
0x1401642fb: pop r12
0x1401642fd: pop r13
0x1401642ff: pop r14
0x140164301: pop r15
0x140164303: pop rbp
0x140164304: ret 
0x140164305: mov byte ptr [rbp + 0x1a94], bl
0x14016430b: movaps xmm6, xmmword ptr [rsp + 0x50]
0x140164310: add rsp, 0x68
0x140164314: pop rbx
0x140164315: pop rdi
0x140164316: pop rsi
0x140164317: pop r12
0x140164319: pop r13
0x14016431b: pop r14
0x14016431d: pop r15
0x14016431f: pop rbp
0x140164320: ret 
0x140164321: nop word ptr cs:[rax + rax]
0x140164330: mov qword ptr [rsp + 0x10], rdx
0x140164335: push rbp
0x140164336: push r15
0x140164338: push r14
0x14016433a: push r13
0x14016433c: push r12
0x14016433e: push rsi
0x14016433f: push rdi
0x140164340: push rbx
0x140164341: sub rsp, 0x68
0x140164345: lea rbp, [rdx + 0x80]
0x14016434c: movaps xmmword ptr [rsp + 0x50], xmm6
0x140164351: movzx ebx, byte ptr [rbp + 0x1a94]
0x140164358: test bl, 1
0x14016435b: je 0x140164385
0x14016435d: lea rcx, [rbp + 0x1568]
0x140164364: call 0x140be24b0
0x140164369: mov byte ptr [rbp + 0x1a93], bl
0x14016436f: movaps xmm6, xmmword ptr [rsp + 0x50]
0x140164374: add rsp, 0x68
0x140164378: pop rbx
0x140164379: pop rdi
0x14016437a: pop rsi
0x14016437b: pop r12
0x14016437d: pop r13
0x14016437f: pop r14
0x140164381: pop r15
0x140164383: pop rbp
0x140164384: ret 
0x140164385: mov byte ptr [rbp + 0x1a93], bl
0x14016438b: movaps xmm6, xmmword ptr [rsp + 0x50]
0x140164390: add rsp, 0x68
0x140164394: pop rbx
0x140164395: pop rdi
0x140164396: pop rsi
0x140164397: pop r12
0x140164399: pop r13
0x14016439b: pop r14
0x14016439d: pop r15
0x14016439f: pop rbp
0x1401643a0: ret 
0x1401643a1: nop word ptr cs:[rax + rax]
0x1401643b0: mov qword ptr [rsp + 0x10], rdx
0x1401643b5: push rbp
0x1401643b6: push r15
0x1401643b8: push r14
0x1401643ba: push r13
0x1401643bc: push r12
0x1401643be: push rsi
0x1401643bf: push rdi
0x1401643c0: push rbx
0x1401643c1: sub rsp, 0x68
0x1401643c5: lea rbp, [rdx + 0x80]
0x1401643cc: movaps xmmword ptr [rsp + 0x50], xmm6
0x1401643d1: test byte ptr [rbp + 0x1a93], 1
0x1401643d8: je 0x1401643e6
0x1401643da: lea rcx, [rbp + 0x1228]
0x1401643e1: call 0x1405dcdc0
0x1401643e6: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1401643eb: add rsp, 0x68
0x1401643ef: pop rbx
0x1401643f0: pop rdi
0x1401643f1: pop rsi
0x1401643f2: pop r12
0x1401643f4: pop r13
0x1401643f6: pop r14
0x1401643f8: pop r15
0x1401643fa: pop rbp
0x1401643fb: ret 
0x1401643fc: nop dword ptr [rax]
0x140164400: mov qword ptr [rsp + 0x10], rdx
0x140164405: push rbp
0x140164406: push r15
0x140164408: push r14
0x14016440a: push r13
0x14016440c: push r12
0x14016440e: push rsi
0x14016440f: push rdi
0x140164410: push rbx
0x140164411: sub rsp, 0x68
0x140164415: lea rbp, [rdx + 0x80]
0x14016441c: movaps xmmword ptr [rsp + 0x50], xmm6
0x140164421: lea rcx, [rbp + 0x110]
0x140164428: lea rdx, [rbp + 0x130]
0x14016442f: mov r8d, 0x10
0x140164435: mov r9d, 0x10
0x14016443b: call 0x14004c400
0x140164440: mov byte ptr [rbp + 0x1a8c], 1
0x140164447: movaps xmm6, xmmword ptr [rsp + 0x50]
0x14016444c: add rsp, 0x68
0x140164450: pop rbx
0x140164451: pop rdi
0x140164452: pop rsi
0x140164453: pop r12
0x140164455: pop r13
0x140164457: pop r14
0x140164459: pop r15
0x14016445b: pop rbp
0x14016445c: ret 
0x14016445d: nop dword ptr [rax]
0x140164460: mov qword ptr [rsp + 0x10], rdx
0x140164465: push rbp
0x140164466: push r15
0x140164468: push r14
0x14016446a: push r13
0x14016446c: push r12
0x14016446e: push rsi
0x14016446f: push rdi
0x140164470: push rbx
0x140164471: sub rsp, 0x68
0x140164475: lea rbp, [rdx + 0x80]
0x14016447c: movaps xmmword ptr [rsp + 0x50], xmm6
0x140164481: mov byte ptr [rbp + 0x1a92], 1
0x140164488: mov byte ptr [rbp + 0x1a91], 1
0x14016448f: lea rcx, [rbp + 0xe0]
0x140164496: call 0x14060b5b0
0x14016449b: mov byte ptr [rbp + 0x1a92], 1
0x1401644a2: mov byte ptr [rbp + 0x1a91], 1
0x1401644a9: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1401644ae: add rsp, 0x68
0x1401644b2: pop rbx
0x1401644b3: pop rdi
0x1401644b4: pop rsi
0x1401644b5: pop r12
0x1401644b7: pop r13
0x1401644b9: pop r14
0x1401644bb: pop r15
0x1401644bd: pop rbp
0x1401644be: ret 
0x1401644bf: nop 
0x1401644c0: mov qword ptr [rsp + 0x10], rdx
0x1401644c5: push rbp
0x1401644c6: push r15
0x1401644c8: push r14
0x1401644ca: push r13
0x1401644cc: push r12
0x1401644ce: push rsi
0x1401644cf: push rdi
0x1401644d0: push rbx
0x1401644d1: sub rsp, 0x68
0x1401644d5: lea rbp, [rdx + 0x80]
0x1401644dc: movaps xmmword ptr [rsp + 0x50], xmm6
0x1401644e1: movzx ebx, byte ptr [rbp + 0x1a8f]
0x1401644e8: movzx esi, byte ptr [rbp + 0x1a90]
0x1401644ef: test bl, 1
0x1401644f2: je 0x140164527
0x1401644f4: lea rcx, [rbp + 0x15e0]
0x1401644fb: call 0x1405fd1d0
0x140164500: and sil, 1
0x140164504: mov byte ptr [rbp + 0x1a92], sil
0x14016450b: mov byte ptr [rbp + 0x1a91], bl
0x140164511: movaps xmm6, xmmword ptr [rsp + 0x50]
0x140164516: add rsp, 0x68
0x14016451a: pop rbx
0x14016451b: pop rdi
0x14016451c: pop rsi
0x14016451d: pop r12
0x14016451f: pop r13
0x140164521: pop r14
0x140164523: pop r15
0x140164525: pop rbp
0x140164526: ret 
0x140164527: and sil, 1
0x14016452b: mov byte ptr [rbp + 0x1a92], sil
0x140164532: mov byte ptr [rbp + 0x1a91], bl
0x140164538: movaps xmm6, xmmword ptr [rsp + 0x50]
0x14016453d: add rsp, 0x68
0x140164541: pop rbx
0x140164542: pop rdi
0x140164543: pop rsi
0x140164544: pop r12
0x140164546: pop r13
0x140164548: pop r14
0x14016454a: pop r15
0x14016454c: pop rbp
0x14016454d: ret 
0x14016454e: nop 
0x140164550: mov qword ptr [rsp + 0x10], rdx
0x140164555: push rbp
0x140164556: push r15
0x140164558: push r14
0x14016455a: push r13
0x14016455c: push r12
0x14016455e: push rsi
0x14016455f: push rdi
0x140164560: push rbx
0x140164561: sub rsp, 0x68
0x140164565: lea rbp, [rdx + 0x80]
0x14016456c: movaps xmmword ptr [rsp + 0x50], xmm6
0x140164571: movzx ebx, byte ptr [rbp + 0x1a92]
0x140164578: test byte ptr [rbp + 0x1a91], 1
0x14016457f: je 0x1401645ac
0x140164581: lea rcx, [rbp + 0x1a50]
0x140164588: call 0x1405da820
0x14016458d: and bl, 1
0x140164590: mov byte ptr [rbp + 0x1a8b], bl
0x140164596: movaps xmm6, xmmword ptr [rsp + 0x50]
0x14016459b: add rsp, 0x68
0x14016459f: pop rbx
0x1401645a0: pop rdi
0x1401645a1: pop rsi
0x1401645a2: pop r12
0x1401645a4: pop r13
0x1401645a6: pop r14
0x1401645a8: pop r15
0x1401645aa: pop rbp
0x1401645ab: ret 
0x1401645ac: and bl, 1
0x1401645af: mov byte ptr [rbp + 0x1a8b], bl
0x1401645b5: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1401645ba: add rsp, 0x68
0x1401645be: pop rbx
0x1401645bf: pop rdi
0x1401645c0: pop rsi
0x1401645c1: pop r12
0x1401645c3: pop r13
0x1401645c5: pop r14
0x1401645c7: pop r15
0x1401645c9: pop rbp
0x1401645ca: ret 
0x1401645cb: nop dword ptr [rax + rax]
0x1401645d0: mov qword ptr [rsp + 0x10], rdx
0x1401645d5: push rbp
0x1401645d6: push r15
0x1401645d8: push r14
0x1401645da: push r13
0x1401645dc: push r12
0x1401645de: push rsi
0x1401645df: push rdi
0x1401645e0: push rbx
0x1401645e1: sub rsp, 0x68
0x1401645e5: lea rbp, [rdx + 0x80]
0x1401645ec: movaps xmmword ptr [rsp + 0x50], xmm6
0x1401645f1: cmp byte ptr [rbp + 0x1a8b], 0
0x1401645f8: je 0x140164623
0x1401645fa: lea rcx, [rbp + 0x1338]
0x140164601: call 0x1405ef210
0x140164606: mov byte ptr [rbp + 0x1a8c], 0
0x14016460d: movaps xmm6, xmmword ptr [rsp + 0x50]
0x140164612: add rsp, 0x68
0x140164616: pop rbx
0x140164617: pop rdi
0x140164618: pop rsi
0x140164619: pop r12
0x14016461b: pop r13
0x14016461d: pop r14
0x14016461f: pop r15
0x140164621: pop rbp
0x140164622: ret 
0x140164623: mov byte ptr [rbp + 0x1a8c], 0
0x14016462a: movaps xmm6, xmmword ptr [rsp + 0x50]
0x14016462f: add rsp, 0x68
0x140164633: pop rbx
0x140164634: pop rdi
0x140164635: pop rsi
0x140164636: pop r12
0x140164638: pop r13
0x14016463a: pop r14
0x14016463c: pop r15
0x14016463e: pop rbp
0x14016463f: ret 
0x140164640: mov qword ptr [rsp + 0x10], rdx
0x140164645: push rbp
0x140164646: push r15
0x140164648: push r14
0x14016464a: push r13
0x14016464c: push r12
0x14016464e: push rsi
0x14016464f: push rdi
0x140164650: push rbx
0x140164651: sub rsp, 0x68
0x140164655: lea rbp, [rdx + 0x80]
0x14016465c: movaps xmmword ptr [rsp + 0x50], xmm6
0x140164661: cmp byte ptr [rbp + 0x1a8c], 0
0x140164668: je 0x14016468c
0x14016466a: lea rcx, [rbp + 0x1450]
0x140164671: call 0x1405daf00
0x140164676: movaps xmm6, xmmword ptr [rsp + 0x50]
0x14016467b: add rsp, 0x68
0x14016467f: pop rbx
0x140164680: pop rdi
0x140164681: pop rsi
0x140164682: pop r12
0x140164684: pop r13
0x140164686: pop r14
0x140164688: pop r15
0x14016468a: pop rbp
0x14016468b: ret 
0x14016468c: movaps xmm6, xmmword ptr [rsp + 0x50]
0x140164691: add rsp, 0x68
0x140164695: pop rbx
0x140164696: pop rdi
0x140164697: pop rsi
0x140164698: pop r12
0x14016469a: pop r13
0x14016469c: pop r14
0x14016469e: pop r15
0x1401646a0: pop rbp
0x1401646a1: ret 
0x1401646a2: nop word ptr cs:[rax + rax]
0x1401646b0: mov qword ptr [rsp + 0x10], rdx
0x1401646b5: push rbp
0x1401646b6: push r15
0x1401646b8: push r14
0x1401646ba: push r13
0x1401646bc: push r12
0x1401646be: push rsi
0x1401646bf: push rdi
0x1401646c0: push rbx
0x1401646c1: sub rsp, 0x68
0x1401646c5: lea rbp, [rdx + 0x80]
0x1401646cc: movaps xmmword ptr [rsp + 0x50], xmm6
0x1401646d1: cmp qword ptr [rbp + 0x17a8], 0
0x1401646d9: je 0x1401646f1
0x1401646db: movaps xmm6, xmmword ptr [rsp + 0x50]
0x1401646e0: add rsp, 0x68
0x1401646e4: pop rbx
0x1401646e5: pop rdi
0x1401646e6: pop rsi
0x1401646e7: pop r12
0x1401646e9: pop r13
0x1401646eb: pop r14
0x1401646ed: pop r15
0x1401646ef: pop rbp
0x1401646f0: ret 
0x1401646f1: lea rcx, [rbp + 0x1198]
0x1401646f8: call 0x1405ef210
0x1401646fd: jmp 0x1401646db
0x1401646ff: nop 
0x140164700: mov qword ptr [rsp + 0x10], rdx
0x140164705: push rbp
0x140164706: push r15
0x140164708: push r14
0x14016470a: push r13
0x14016470c: push r12
0x14016470e: push rsi
0x14016470f: push rdi
0x140164710: push rbx
0x140164711: sub rsp, 0x68
0x140164715: lea rbp, [rdx + 0x80]
0x14016471c: movaps xmmword ptr [rsp + 0x50], xmm6
0x140164721: mov rcx, qword ptr [rbp + 0x1a80]
0x140164728: call 0x140161c30
0x14016472d: mov byte ptr [rbp + 0x1a94], 1
0x140164734: movaps xmm6, xmmword ptr [rsp + 0x50]
0x140164739: add rsp, 0x68
0x14016473d: pop rbx
0x14016473e: pop rdi
0x14016473f: pop rsi
0x140164740: pop r12
0x140164742: pop r13
0x140164744: pop r14
0x140164746: pop r15
0x140164748: pop rbp
0x140164749: ret 
0x14016474a: nop word ptr [rax + rax]
0x140164750: mov qword ptr [rsp + 0x10], rdx
0x140164755: push rbp
0x140164756: push r15
0x140164758: push r14
0x14016475a: push r13
0x14016475c: push r12
0x14016475e: push rsi
0x14016475f: push rdi
0x140164760: push rbx
0x140164761: sub rsp, 0x68
0x140164765: lea rbp, [rdx + 0x80]
0x14016476c: movaps xmmword ptr [rsp + 0x50], xmm6
0x140164771: lea rcx, [rbp + 0x1450]
0x140164778: call 0x1405dcdc0
0x14016477d: mov byte ptr [rbp + 0x1a96], 0
0x140164784: mov byte ptr [rbp + 0x1a95], 0
0x14016478b: movaps xmm6, xmmword ptr [rsp + 0x50]
0x140164790: add rsp, 0x68
0x140164794: pop rbx
0x140164795: pop rdi
0x140164796: pop rsi
0x140164797: pop r12
0x140164799: pop r13
0x14016479b: pop r14
0x14016479d: pop r15
0x14016479f: pop rbp
0x1401647a0: ret 
