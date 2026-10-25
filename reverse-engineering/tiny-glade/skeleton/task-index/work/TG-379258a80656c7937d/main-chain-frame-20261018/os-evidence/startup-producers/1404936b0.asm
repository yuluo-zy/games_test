1404936b0: push rbp
1404936b1: push rsi
1404936b2: push rdi
1404936b3: push rbx
1404936b4: sub rsp, 0x2f8
1404936bb: lea rbp, [rsp + 0x80]
1404936c3: movaps xmmword ptr [rbp + 0x260], xmm9
1404936cb: movaps xmmword ptr [rbp + 0x250], xmm8
1404936d3: movaps xmmword ptr [rbp + 0x240], xmm7
1404936da: movaps xmmword ptr [rbp + 0x230], xmm6
1404936e1: mov qword ptr [rbp + 0x228], 0xfffffffffffffffe
1404936ec: mov rsi, rcx
1404936ef: lea rdi, [rcx + 0x38]
1404936f3: movabs rdx, 0xe720026d3fbfc97d
1404936fd: movabs r8, 0xb937b7dad78fad7c
140493707: mov rcx, rdi
14049370a: call 0x1400dd6f0
14049370f: test rax, rax
140493712: je 0x140493747
140493714: cmp qword ptr [rsi + 0x218], rdx
14049371b: jbe 0x140493747
14049371d: mov rax, qword ptr [rsi + 0x210]
140493724: mov rax, qword ptr [rax + rdx*8]
140493728: test rax, rax
14049372b: je 0x140493747
14049372d: mov rcx, qword ptr [rsi + 0x1e0]
140493734: not rax
140493737: lea rax, [rax + rax*2]
14049373b: shl rax, 5
14049373f: cmp qword ptr [rcx + rax + 0x20], 0
140493745: jne 0x14049374f
140493747: mov rcx, rsi
14049374a: call 0x14056c580
14049374f: movabs rdx, 0x8042b3a7b1372c00
140493759: movabs r8, 0xd68ff19c1f8d2497
140493763: mov rcx, rdi
140493766: call 0x1400dd6f0
14049376b: test rax, rax
14049376e: je 0x1404937a3
140493770: cmp qword ptr [rsi + 0x218], rdx
140493777: jbe 0x1404937a3
140493779: mov rax, qword ptr [rsi + 0x210]
140493780: mov rax, qword ptr [rax + rdx*8]
140493784: test rax, rax
140493787: je 0x1404937a3
140493789: mov rcx, qword ptr [rsi + 0x1e0]
140493790: not rax
140493793: lea rax, [rax + rax*2]
140493797: shl rax, 5
14049379b: cmp qword ptr [rcx + rax + 0x20], 0
1404937a1: jne 0x1404937ab
1404937a3: mov rcx, rsi
1404937a6: call 0x14057b6d0
1404937ab: movabs rdx, 0xa8a10fd8c046fe59
1404937b5: movabs r8, 0x81e8ffc8d715c092
1404937bf: mov rcx, rdi
1404937c2: call 0x1400dd6f0
1404937c7: test rax, rax
1404937ca: je 0x1404937ff
1404937cc: cmp qword ptr [rsi + 0x218], rdx
1404937d3: jbe 0x1404937ff
1404937d5: mov rax, qword ptr [rsi + 0x210]
1404937dc: mov rax, qword ptr [rax + rdx*8]
1404937e0: test rax, rax
1404937e3: je 0x1404937ff
1404937e5: mov rcx, qword ptr [rsi + 0x1e0]
1404937ec: not rax
1404937ef: lea rax, [rax + rax*2]
1404937f3: shl rax, 5
1404937f7: cmp qword ptr [rcx + rax + 0x20], 0
1404937fd: jne 0x140493807
1404937ff: mov rcx, rsi
140493802: call 0x140569a30
140493807: movabs rdx, 0x8513c2de71b39e70
140493811: movabs r8, 0xd6ae0700028bef89
14049381b: mov rcx, rdi
14049381e: call 0x1400dd6f0
140493823: test rax, rax
140493826: je 0x14049385b
140493828: cmp qword ptr [rsi + 0x218], rdx
14049382f: jbe 0x14049385b
140493831: mov rax, qword ptr [rsi + 0x210]
140493838: mov rax, qword ptr [rax + rdx*8]
14049383c: test rax, rax
14049383f: je 0x14049385b
140493841: mov rcx, qword ptr [rsi + 0x1e0]
140493848: not rax
14049384b: lea rax, [rax + rax*2]
14049384f: shl rax, 5
140493853: cmp qword ptr [rcx + rax + 0x20], 0
140493859: jne 0x140493863
14049385b: mov rcx, rsi
14049385e: call 0x140586a40
140493863: movabs rdx, 0x79e71d8ed7ab7fdf
14049386d: movabs r8, 0xa05b3cfd4d7aa99a
140493877: mov rcx, rdi
14049387a: call 0x1400dd6f0
14049387f: test rax, rax
140493882: je 0x1404938b7
140493884: cmp qword ptr [rsi + 0x218], rdx
14049388b: jbe 0x1404938b7
14049388d: mov rax, qword ptr [rsi + 0x210]
140493894: mov rax, qword ptr [rax + rdx*8]
140493898: test rax, rax
14049389b: je 0x1404938b7
14049389d: mov rcx, qword ptr [rsi + 0x1e0]
1404938a4: not rax
1404938a7: lea rax, [rax + rax*2]
1404938ab: shl rax, 5
1404938af: cmp qword ptr [rcx + rax + 0x20], 0
1404938b5: jne 0x1404938bf
1404938b7: mov rcx, rsi
1404938ba: call 0x14056e470
1404938bf: movabs rdx, 0x1b0e037e491a7be3
1404938c9: movabs r8, 0xd615fb86ff082b8e
1404938d3: mov rcx, rdi
1404938d6: call 0x1400dd6f0
1404938db: test rax, rax
1404938de: je 0x140493913
1404938e0: cmp qword ptr [rsi + 0x218], rdx
1404938e7: jbe 0x140493913
1404938e9: mov rax, qword ptr [rsi + 0x210]
1404938f0: mov rax, qword ptr [rax + rdx*8]
1404938f4: test rax, rax
1404938f7: je 0x140493913
1404938f9: mov rcx, qword ptr [rsi + 0x1e0]
140493900: not rax
140493903: lea rax, [rax + rax*2]
140493907: shl rax, 5
14049390b: cmp qword ptr [rcx + rax + 0x20], 0
140493911: jne 0x14049391b
140493913: mov rcx, rsi
140493916: call 0x140583ef0
14049391b: movabs rdx, 0xe25be0e1685456bf
140493925: movabs r8, 0xcbf7e93aa0560e10
14049392f: mov rcx, rdi
140493932: call 0x1400dd6f0
140493937: test rax, rax
14049393a: je 0x14049396f
14049393c: cmp qword ptr [rsi + 0x218], rdx
140493943: jbe 0x14049396f
140493945: mov rax, qword ptr [rsi + 0x210]
14049394c: mov rax, qword ptr [rax + rdx*8]
140493950: test rax, rax
140493953: je 0x14049396f
140493955: mov rcx, qword ptr [rsi + 0x1e0]
14049395c: not rax
14049395f: lea rax, [rax + rax*2]
140493963: shl rax, 5
140493967: cmp qword ptr [rcx + rax + 0x20], 0
14049396d: jne 0x140493977
14049396f: mov rcx, rsi
140493972: call 0x14056f700
140493977: movabs rbx, 0x8000000000000000
140493981: mov qword ptr [rbp - 0x60], rbx
140493985: lea rdx, [rbp - 0x60]
140493989: mov rcx, rsi
14049398c: call 0x140122620
140493991: mov rsi, rax
140493994: lea rdi, [rbp - 0x60]
140493998: mov rcx, rdi
14049399b: call 0x140aa5150
1404939a0: mov rcx, rsi
1404939a3: mov rdx, rdi
1404939a6: call 0x140129b00
1404939ab: mov rcx, rax
1404939ae: xor edx, edx
1404939b0: call 0x14012dee0
1404939b5: mov rcx, rax
1404939b8: xor edx, edx
1404939ba: call 0x14012d9a0
1404939bf: mov rsi, rax
1404939c2: lea rcx, [rax + 0x38]
1404939c6: movabs rdx, 0x22d15b1cde734cb4
1404939d0: movabs r8, 0x89d40ed439c1a49f
1404939da: call 0x1400dd6f0
1404939df: test rax, rax
1404939e2: je 0x140493a17
1404939e4: cmp qword ptr [rsi + 0x218], rdx
1404939eb: jbe 0x140493a17
1404939ed: mov rax, qword ptr [rsi + 0x210]
1404939f4: mov rax, qword ptr [rax + rdx*8]
1404939f8: test rax, rax
1404939fb: je 0x140493a17
1404939fd: mov rcx, qword ptr [rsi + 0x1e0]
140493a04: not rax
140493a07: lea rax, [rax + rax*2]
140493a0b: shl rax, 5
140493a0f: cmp qword ptr [rcx + rax + 0x20], 0
140493a15: jne 0x140493a1f
140493a17: mov rcx, rsi
140493a1a: call 0x140573b10
140493a1f: mov byte ptr [rbp - 0x58], 0
140493a23: mov qword ptr [rbp - 0x60], 0
140493a2b: lea rdx, [rbp - 0x60]
140493a2f: mov rcx, rsi
140493a32: call 0x140134630
140493a37: mov rsi, rax
140493a3a: call 0x140d4d740
140493a3f: mov rdi, rax
140493a42: test byte ptr [rax], 1
140493a45: je 0x140493a58
140493a47: mov rcx, rdi
140493a4a: add rcx, 8
140493a4e: mov rax, qword ptr [rdi + 8]
140493a52: mov rdx, qword ptr [rdi + 0x10]
140493a56: jmp 0x140493a73
140493a58: call 0x140d64e00
140493a5d: mov rcx, rdi
140493a60: add rcx, 8
140493a64: mov qword ptr [rdi], 1
140493a6b: mov qword ptr [rdi + 8], rax
140493a6f: mov qword ptr [rdi + 0x10], rdx
140493a73: lea r8, [rax + 1]
140493a77: mov qword ptr [rcx], r8
140493a7a: movups xmm6, xmmword ptr [rip + 0x254b5cf]
140493a81: movaps xmmword ptr [rbp + 0x1d0], xmm6
140493a88: movups xmm7, xmmword ptr [rip + 0x254b5b1]
140493a8f: movaps xmmword ptr [rbp + 0x1c0], xmm7
140493a96: mov qword ptr [rbp + 0x1e0], rax
140493a9d: mov qword ptr [rbp + 0x1e8], rdx
140493aa4: call 0x140d4d740
140493aa9: mov rdi, rax
140493aac: test byte ptr [rax], 1
140493aaf: je 0x140493ac2
140493ab1: mov rcx, rdi
140493ab4: add rcx, 8
140493ab8: mov rax, qword ptr [rdi + 8]
140493abc: mov rdx, qword ptr [rdi + 0x10]
140493ac0: jmp 0x140493add
140493ac2: call 0x140d64e00
140493ac7: mov rcx, rdi
140493aca: add rcx, 8
140493ace: mov qword ptr [rdi], 1
140493ad5: mov qword ptr [rdi + 8], rax
140493ad9: mov qword ptr [rdi + 0x10], rdx
140493add: lea r8, [rax + 1]
140493ae1: mov qword ptr [rcx], r8
140493ae4: movaps xmmword ptr [rbp + 0x200], xmm6
140493aeb: movaps xmmword ptr [rbp + 0x1f0], xmm7
140493af2: mov qword ptr [rbp + 0x210], rax
140493af9: mov qword ptr [rbp + 0x218], rdx
140493b00: call 0x140d4d740
140493b05: mov rdi, rax
140493b08: test byte ptr [rax], 1
140493b0b: je 0x140493b1e
140493b0d: mov rcx, rdi
140493b10: add rcx, 8
140493b14: mov rax, qword ptr [rdi + 8]
140493b18: mov rdx, qword ptr [rdi + 0x10]
140493b1c: jmp 0x140493b39
140493b1e: call 0x140d64e00
140493b23: mov rcx, rdi
140493b26: add rcx, 8
140493b2a: mov qword ptr [rdi], 1
140493b31: mov qword ptr [rdi + 8], rax
140493b35: mov qword ptr [rdi + 0x10], rdx
140493b39: lea r8, [rax + 1]
140493b3d: mov qword ptr [rcx], r8
140493b40: movaps xmm0, xmmword ptr [rbp + 0x1c0]
140493b47: movaps xmm1, xmmword ptr [rbp + 0x1d0]
140493b4e: movaps xmm2, xmmword ptr [rbp + 0x1e0]
140493b55: movaps xmmword ptr [rbp - 0x40], xmm2
140493b59: movaps xmmword ptr [rbp - 0x50], xmm1
140493b5d: movaps xmmword ptr [rbp - 0x60], xmm0
140493b61: movaps xmm0, xmmword ptr [rbp + 0x1f0]
140493b68: movaps xmm1, xmmword ptr [rbp + 0x200]
140493b6f: movaps xmm2, xmmword ptr [rbp + 0x210]
140493b76: movaps xmmword ptr [rbp - 0x30], xmm0
140493b7a: movaps xmmword ptr [rbp - 0x20], xmm1
140493b7e: movaps xmmword ptr [rbp - 0x10], xmm2
140493b82: movaps xmmword ptr [rbp], xmm7
140493b86: movaps xmmword ptr [rbp + 0x10], xmm6
140493b8a: mov qword ptr [rbp + 0x20], rax
140493b8e: mov qword ptr [rbp + 0x28], rdx
140493b92: lea rdx, [rbp - 0x60]
140493b96: mov rcx, rsi
140493b99: call 0x140139c00
140493b9e: mov rsi, rax
140493ba1: call 0x140d4d740
140493ba6: mov rdi, rax
140493ba9: test byte ptr [rax], 1
140493bac: je 0x140493bbf
140493bae: mov rcx, rdi
140493bb1: add rcx, 8
140493bb5: mov rax, qword ptr [rdi + 8]
140493bb9: mov rdx, qword ptr [rdi + 0x10]
140493bbd: jmp 0x140493bda
140493bbf: call 0x140d64e00
140493bc4: mov rcx, rdi
140493bc7: add rcx, 8
140493bcb: mov qword ptr [rdi], 1
140493bd2: mov qword ptr [rdi + 8], rax
140493bd6: mov qword ptr [rdi + 0x10], rdx
140493bda: lea r8, [rax + 1]
140493bde: mov qword ptr [rcx], r8
140493be1: movaps xmmword ptr [rbp - 0x50], xmm6
140493be5: movaps xmmword ptr [rbp - 0x60], xmm7
140493be9: mov qword ptr [rbp - 0x40], rax
140493bed: mov qword ptr [rbp - 0x38], rdx
140493bf1: lea rdx, [rbp - 0x60]
140493bf5: mov rcx, rsi
140493bf8: call 0x140123200
140493bfd: mov rsi, rax
140493c00: call 0x140d4d740
140493c05: mov rdi, rax
140493c08: test byte ptr [rax], 1
140493c0b: je 0x140493c1e
140493c0d: mov rcx, rdi
140493c10: add rcx, 8
140493c14: mov rax, qword ptr [rdi + 8]
140493c18: mov rdx, qword ptr [rdi + 0x10]
140493c1c: jmp 0x140493c39
140493c1e: call 0x140d64e00
140493c23: mov rcx, rdi
140493c26: add rcx, 8
140493c2a: mov qword ptr [rdi], 1
140493c31: mov qword ptr [rdi + 8], rax
140493c35: mov qword ptr [rdi + 0x10], rdx
140493c39: lea r8, [rax + 1]
140493c3d: mov qword ptr [rcx], r8
140493c40: movaps xmmword ptr [rbp - 0x50], xmm6
140493c44: movaps xmmword ptr [rbp - 0x60], xmm7
140493c48: mov qword ptr [rbp - 0x40], rax
140493c4c: mov qword ptr [rbp - 0x38], rdx
140493c50: lea rdx, [rbp - 0x60]
140493c54: mov rcx, rsi
140493c57: call 0x140132f10
140493c5c: mov rsi, rax
140493c5f: call 0x140d4d740
140493c64: mov rdi, rax
140493c67: test byte ptr [rax], 1
140493c6a: je 0x140493c7d
140493c6c: mov rcx, rdi
140493c6f: add rcx, 8
140493c73: mov rax, qword ptr [rdi + 8]
140493c77: mov rdx, qword ptr [rdi + 0x10]
140493c7b: jmp 0x140493c98
140493c7d: call 0x140d64e00
140493c82: mov rcx, rdi
140493c85: add rcx, 8
140493c89: mov qword ptr [rdi], 1
140493c90: mov qword ptr [rdi + 8], rax
140493c94: mov qword ptr [rdi + 0x10], rdx
140493c98: lea r8, [rax + 1]
140493c9c: mov qword ptr [rcx], r8
140493c9f: movaps xmmword ptr [rbp - 0x50], xmm6
140493ca3: movaps xmmword ptr [rbp - 0x60], xmm7
140493ca7: mov qword ptr [rbp - 0x40], rax
140493cab: mov qword ptr [rbp - 0x38], rdx
140493caf: lea rdx, [rbp - 0x60]
140493cb3: mov rcx, rsi
140493cb6: call 0x14013e720
140493cbb: mov rsi, rax
140493cbe: lea rdi, [rbp - 0x60]
140493cc2: mov rcx, rdi
140493cc5: call 0x140859f90
140493cca: mov rcx, rsi
140493ccd: mov rdx, rdi
140493cd0: call 0x14011dc00
140493cd5: mov qword ptr [rbp - 0x60], rbx
140493cd9: lea rdx, [rbp - 0x60]
140493cdd: mov rcx, rax
140493ce0: call 0x140119de0
140493ce5: mov byte ptr [rbp - 0x50], 0
140493ce9: lea rdx, [rbp - 0x60]
140493ced: mov rcx, rax
140493cf0: call 0x14011baa0
140493cf5: mov qword ptr [rbp - 0x5b], 0
140493cfd: mov qword ptr [rbp - 0x60], 0
140493d05: lea rdx, [rbp - 0x60]
140493d09: mov rcx, rax
140493d0c: call 0x1401175d0
140493d11: mov rsi, rax
140493d14: mov rdi, rax
140493d17: add rdi, 0x38
140493d1b: movabs rdx, 0x576dff9de241abe1
140493d25: movabs r8, 0xf0bc02fcecf4b952
140493d2f: mov rcx, rdi
140493d32: call 0x1400dd6f0
140493d37: test rax, rax
140493d3a: je 0x140493d6f
140493d3c: cmp qword ptr [rsi + 0x218], rdx
140493d43: jbe 0x140493d6f
140493d45: mov rax, qword ptr [rsi + 0x210]
140493d4c: mov rax, qword ptr [rax + rdx*8]
140493d50: test rax, rax
140493d53: je 0x140493d6f
140493d55: mov rcx, qword ptr [rsi + 0x1e0]
140493d5c: not rax
140493d5f: lea rax, [rax + rax*2]
140493d63: shl rax, 5
140493d67: cmp qword ptr [rcx + rax + 0x20], 0
140493d6d: jne 0x140493d77
140493d6f: mov rcx, rsi
140493d72: call 0x14056de40
140493d77: movabs rdx, 0x601cb891fc59f04e
140493d81: movabs r8, 0x48fc69b3ae04e6e6
140493d8b: mov rcx, rdi
140493d8e: call 0x1400dd6f0
140493d93: test rax, rax
140493d96: je 0x140493dcb
140493d98: cmp qword ptr [rsi + 0x218], rdx
140493d9f: jbe 0x140493dcb
140493da1: mov rax, qword ptr [rsi + 0x210]
140493da8: mov rax, qword ptr [rax + rdx*8]
140493dac: test rax, rax
140493daf: je 0x140493dcb
140493db1: mov rcx, qword ptr [rsi + 0x1e0]
140493db8: not rax
140493dbb: lea rax, [rax + rax*2]
140493dbf: shl rax, 5
140493dc3: cmp qword ptr [rcx + rax + 0x20], 0
140493dc9: jne 0x140493dd3
140493dcb: mov rcx, rsi
140493dce: call 0x140574da0
140493dd3: movabs rdx, 0xd50d9d88448b147e
140493ddd: movabs r8, 0xb4fffe74597bba92
140493de7: mov rcx, rdi
140493dea: call 0x1400dd6f0
140493def: test rax, rax
140493df2: je 0x140493e27
140493df4: cmp qword ptr [rsi + 0x218], rdx
140493dfb: jbe 0x140493e27
140493dfd: mov rax, qword ptr [rsi + 0x210]
140493e04: mov rax, qword ptr [rax + rdx*8]
140493e08: test rax, rax
140493e0b: je 0x140493e27
140493e0d: mov rcx, qword ptr [rsi + 0x1e0]
140493e14: not rax
140493e17: lea rax, [rax + rax*2]
140493e1b: shl rax, 5
140493e1f: cmp qword ptr [rcx + rax + 0x20], 0
140493e25: jne 0x140493e2f
140493e27: mov rcx, rsi
140493e2a: call 0x140576030
140493e2f: movabs rdx, 0x8dd7de3841b9d690
140493e39: movabs r8, 0x47e25c7354f27f6d
140493e43: mov rcx, rdi
140493e46: call 0x1400dd6f0
140493e4b: test rax, rax
140493e4e: je 0x140493e83
140493e50: cmp qword ptr [rsi + 0x218], rdx
140493e57: jbe 0x140493e83
140493e59: mov rax, qword ptr [rsi + 0x210]
140493e60: mov rax, qword ptr [rax + rdx*8]
140493e64: test rax, rax
140493e67: je 0x140493e83
140493e69: mov rcx, qword ptr [rsi + 0x1e0]
140493e70: not rax
140493e73: lea rax, [rax + rax*2]
140493e77: shl rax, 5
140493e7b: cmp qword ptr [rcx + rax + 0x20], 0
140493e81: jne 0x140493e8b
140493e83: mov rcx, rsi
140493e86: call 0x14056fd30
140493e8b: movabs rdx, 0xd64c11374a7ca229
140493e95: movabs r8, 0x3f74d6fed35e7640
140493e9f: mov rcx, rdi
140493ea2: call 0x1400dd6f0
140493ea7: test rax, rax
140493eaa: je 0x140493edf
140493eac: cmp qword ptr [rsi + 0x218], rdx
140493eb3: jbe 0x140493edf
140493eb5: mov rax, qword ptr [rsi + 0x210]
140493ebc: mov rax, qword ptr [rax + rdx*8]
140493ec0: test rax, rax
140493ec3: je 0x140493edf
140493ec5: mov rcx, qword ptr [rsi + 0x1e0]
140493ecc: not rax
140493ecf: lea rax, [rax + rax*2]
140493ed3: shl rax, 5
140493ed7: cmp qword ptr [rcx + rax + 0x20], 0
140493edd: jne 0x140493ee7
140493edf: mov rcx, rsi
140493ee2: call 0x14056f0d0
140493ee7: movabs rdx, 0x3f61271c4ced0f0a
140493ef1: movabs r8, 0xccc9d7fe6ebf5bdd
140493efb: mov rcx, rdi
140493efe: call 0x1400dd6f0
140493f03: test rax, rax
140493f06: je 0x140493f3b
140493f08: cmp qword ptr [rsi + 0x218], rdx
140493f0f: jbe 0x140493f3b
140493f11: mov rax, qword ptr [rsi + 0x210]
140493f18: mov rax, qword ptr [rax + rdx*8]
140493f1c: test rax, rax
140493f1f: je 0x140493f3b
140493f21: mov rcx, qword ptr [rsi + 0x1e0]
140493f28: not rax
140493f2b: lea rax, [rax + rax*2]
140493f2f: shl rax, 5
140493f33: cmp qword ptr [rcx + rax + 0x20], 0
140493f39: jne 0x140493f43
140493f3b: mov rcx, rsi
140493f3e: call 0x14056eaa0
140493f43: movabs rdx, 0x562171a7baf8e365
140493f4d: movabs r8, 0xc27a98a3e239713d
140493f57: mov rcx, rdi
140493f5a: call 0x1400dd6f0
140493f5f: test rax, rax
140493f62: je 0x140493f97
140493f64: cmp qword ptr [rsi + 0x218], rdx
140493f6b: jbe 0x140493f97
140493f6d: mov rax, qword ptr [rsi + 0x210]
140493f74: mov rax, qword ptr [rax + rdx*8]
140493f78: test rax, rax
140493f7b: je 0x140493f97
140493f7d: mov rcx, qword ptr [rsi + 0x1e0]
140493f84: not rax
140493f87: lea rax, [rax + rax*2]
140493f8b: shl rax, 5
140493f8f: cmp qword ptr [rcx + rax + 0x20], 0
140493f95: jne 0x140493f9f
140493f97: mov rcx, rsi
140493f9a: call 0x14057aa70
140493f9f: lea rdi, [rbp - 0x60]
140493fa3: mov rcx, rdi
140493fa6: call 0x140ade610
140493fab: mov rcx, rsi
140493fae: mov rdx, rdi
140493fb1: call 0x140133ae0
140493fb6: mov rsi, rax
140493fb9: mov rdi, rax
140493fbc: add rdi, 0x38
140493fc0: movabs rdx, 0x585c5fad562d094d
140493fca: movabs r8, 0xe08c7d898375d8cf
140493fd4: mov rcx, rdi
140493fd7: call 0x1400dd6f0
140493fdc: test rax, rax
140493fdf: je 0x140494014
140493fe1: cmp qword ptr [rsi + 0x218], rdx
140493fe8: jbe 0x140494014
140493fea: mov rax, qword ptr [rsi + 0x210]
140493ff1: mov rax, qword ptr [rax + rdx*8]
140493ff5: test rax, rax
140493ff8: je 0x140494014
140493ffa: mov rcx, qword ptr [rsi + 0x1e0]
140494001: not rax
140494004: lea rax, [rax + rax*2]
140494008: shl rax, 5
14049400c: cmp qword ptr [rcx + rax + 0x20], 0
140494012: jne 0x14049401c
140494014: mov rcx, rsi
140494017: call 0x14057a440
14049401c: movabs rdx, 0xb42d6290ba7df1f8
140494026: movabs r8, 0xc1d621cee763b746
140494030: mov rcx, rdi
140494033: call 0x1400dd6f0
140494038: test rax, rax
14049403b: je 0x140494070
14049403d: cmp qword ptr [rsi + 0x218], rdx
140494044: jbe 0x140494070
140494046: mov rax, qword ptr [rsi + 0x210]
14049404d: mov rax, qword ptr [rax + rdx*8]
140494051: test rax, rax
140494054: je 0x140494070
140494056: mov rcx, qword ptr [rsi + 0x1e0]
14049405d: not rax
140494060: lea rax, [rax + rax*2]
140494064: shl rax, 5
140494068: cmp qword ptr [rcx + rax + 0x20], 0
14049406e: jne 0x140494078
140494070: mov rcx, rsi
140494073: call 0x1405813a0
140494078: mov byte ptr [rbp - 0x48], 0
14049407c: mov qword ptr [rbp - 0x60], 0
140494084: mov qword ptr [rbp - 0x58], 4
14049408c: mov qword ptr [rbp - 0x50], 0
140494094: lea rdx, [rbp - 0x60]
140494098: mov rcx, rsi
14049409b: call 0x14013ed10
1404940a0: mov rsi, rax
1404940a3: lea rdi, [rbp - 0x60]
1404940a7: mov rcx, rdi
1404940aa: call 0x140a82b20
1404940af: mov rcx, rsi
1404940b2: mov rdx, rdi
1404940b5: call 0x1401164c0
1404940ba: movss xmm1, dword ptr [rip + 0x2491842]
1404940c2: mov rcx, rax
1404940c5: call 0x140121b00
1404940ca: mov qword ptr [rbp - 0x60], 2
1404940d2: mov qword ptr [rbp + 0xc0], rbx
1404940d9: lea rdx, [rbp - 0x60]
1404940dd: mov rcx, rax
1404940e0: call 0x140115400
1404940e5: movabs rcx, 0x8000000000000002
1404940ef: mov qword ptr [rbp - 0x60], rcx
1404940f3: lea rdx, [rbp - 0x60]
1404940f7: mov rcx, rax
1404940fa: call 0x140118c30
1404940ff: mov rsi, rax
140494102: call 0x140d4d740
140494107: mov rdi, rax
14049410a: test byte ptr [rax], 1
14049410d: je 0x140494120
14049410f: mov rcx, rdi
140494112: add rcx, 8
140494116: mov rax, qword ptr [rdi + 8]
14049411a: mov rdx, qword ptr [rdi + 0x10]
14049411e: jmp 0x14049413b
140494120: call 0x140d64e00
140494125: mov rcx, rdi
140494128: add rcx, 8
14049412c: mov qword ptr [rdi], 1
140494133: mov qword ptr [rdi + 8], rax
140494137: mov qword ptr [rdi + 0x10], rdx
14049413b: lea r8, [rax + 1]
14049413f: mov qword ptr [rcx], r8
140494142: movaps xmmword ptr [rbp - 0x50], xmm6
140494146: movaps xmmword ptr [rbp - 0x60], xmm7
14049414a: mov qword ptr [rbp - 0x40], rax
14049414e: mov qword ptr [rbp - 0x38], rdx
140494152: lea rdx, [rbp - 0x60]
140494156: mov rcx, rsi
140494159: call 0x1401305a0
14049415e: mov rsi, rax
140494161: call 0x140d4d740
140494166: mov rdi, rax
140494169: test byte ptr [rax], 1
14049416c: je 0x14049417f
14049416e: mov rcx, rdi
140494171: add rcx, 8
140494175: mov rax, qword ptr [rdi + 8]
140494179: mov rdx, qword ptr [rdi + 0x10]
14049417d: jmp 0x14049419a
14049417f: call 0x140d64e00
140494184: mov rcx, rdi
140494187: add rcx, 8
14049418b: mov qword ptr [rdi], 1
140494192: mov qword ptr [rdi + 8], rax
140494196: mov qword ptr [rdi + 0x10], rdx
14049419a: lea r8, [rax + 1]
14049419e: mov qword ptr [rcx], r8
1404941a1: movaps xmmword ptr [rbp - 0x50], xmm6
1404941a5: movaps xmmword ptr [rbp - 0x60], xmm7
1404941a9: mov qword ptr [rbp - 0x40], rax
1404941ad: mov qword ptr [rbp - 0x38], rdx
1404941b1: lea rdx, [rbp - 0x60]
1404941b5: mov rcx, rsi
1404941b8: call 0x14013fed0
1404941bd: mov rsi, rax
1404941c0: call 0x140d4d740
1404941c5: mov rdi, rax
1404941c8: test byte ptr [rax], 1
1404941cb: je 0x1404941de
1404941cd: mov rcx, rdi
1404941d0: add rcx, 8
1404941d4: mov rax, qword ptr [rdi + 8]
1404941d8: mov rdx, qword ptr [rdi + 0x10]
1404941dc: jmp 0x1404941f9
1404941de: call 0x140d64e00
1404941e3: mov rcx, rdi
1404941e6: add rcx, 8
1404941ea: mov qword ptr [rdi], 1
1404941f1: mov qword ptr [rdi + 8], rax
1404941f5: mov qword ptr [rdi + 0x10], rdx
1404941f9: lea r8, [rax + 1]
1404941fd: mov qword ptr [rcx], r8
140494200: movaps xmmword ptr [rbp + 0x200], xmm6
140494207: movaps xmmword ptr [rbp + 0x1f0], xmm7
14049420e: mov qword ptr [rbp + 0x210], rax
140494215: mov qword ptr [rbp + 0x218], rdx
14049421c: xor ecx, ecx
14049421e: call 0x1400d9b20
140494223: test rax, rax
140494226: je 0x140494bf9
14049422c: movups xmm0, xmmword ptr [rax]
14049422f: inc qword ptr [rax]
140494232: movaps xmm1, xmmword ptr [rbp + 0x1f0]
140494239: movaps xmm2, xmmword ptr [rbp + 0x200]
140494240: movaps xmm3, xmmword ptr [rbp + 0x210]
140494247: movups xmmword ptr [rbp - 0x28], xmm3
14049424b: movups xmmword ptr [rbp - 0x38], xmm2
14049424f: movups xmmword ptr [rbp - 0x48], xmm1
140494253: mov qword ptr [rbp - 0x60], 0
14049425b: mov qword ptr [rbp - 0x58], 8
140494263: mov qword ptr [rbp - 0x50], 0
14049426b: movups xmm8, xmmword ptr [rip + 0x250f62d]
140494273: movups xmmword ptr [rbp - 0x18], xmm8
140494278: movups xmm9, xmmword ptr [rip + 0x250f630]
140494280: movups xmmword ptr [rbp - 8], xmm9
140494285: movups xmmword ptr [rbp + 8], xmm0
140494289: lea rdx, [rbp - 0x60]
14049428d: mov rcx, rsi
140494290: call 0x1401243f0
140494295: mov rsi, rax
140494298: call 0x140d4d740
14049429d: mov rdi, rax
1404942a0: test byte ptr [rax], 1
1404942a3: je 0x1404942b6
1404942a5: mov rcx, rdi
1404942a8: add rcx, 8
1404942ac: mov rax, qword ptr [rdi + 8]
1404942b0: mov rdx, qword ptr [rdi + 0x10]
1404942b4: jmp 0x1404942d1
1404942b6: call 0x140d64e00
1404942bb: mov rcx, rdi
1404942be: add rcx, 8
1404942c2: mov qword ptr [rdi], 1
1404942c9: mov qword ptr [rdi + 8], rax
1404942cd: mov qword ptr [rdi + 0x10], rdx
1404942d1: lea r8, [rax + 1]
1404942d5: mov qword ptr [rcx], r8
1404942d8: movaps xmmword ptr [rbp + 0x1d0], xmm6
1404942df: movaps xmmword ptr [rbp + 0x1c0], xmm7
1404942e6: mov qword ptr [rbp + 0x1e0], rax
1404942ed: mov qword ptr [rbp + 0x1e8], rdx
1404942f4: call 0x140d4d740
1404942f9: mov rdi, rax
1404942fc: test byte ptr [rax], 1
1404942ff: je 0x140494312
140494301: mov rcx, rdi
140494304: add rcx, 8
140494308: mov rax, qword ptr [rdi + 8]
14049430c: mov rdx, qword ptr [rdi + 0x10]
140494310: jmp 0x14049432d
140494312: call 0x140d64e00
140494317: mov rcx, rdi
14049431a: add rcx, 8
14049431e: mov qword ptr [rdi], 1
140494325: mov qword ptr [rdi + 8], rax
140494329: mov qword ptr [rdi + 0x10], rdx
14049432d: lea r8, [rax + 1]
140494331: mov qword ptr [rcx], r8
140494334: movaps xmmword ptr [rbp + 0x200], xmm6
14049433b: movaps xmmword ptr [rbp + 0x1f0], xmm7
140494342: mov qword ptr [rbp + 0x210], rax
140494349: mov qword ptr [rbp + 0x218], rdx
140494350: xor ecx, ecx
140494352: call 0x1400d9b20
140494357: test rax, rax
14049435a: je 0x140494c07
140494360: movups xmm0, xmmword ptr [rax]
140494363: inc qword ptr [rax]
140494366: movaps xmm1, xmmword ptr [rbp + 0x1c0]
14049436d: movaps xmm2, xmmword ptr [rbp + 0x1d0]
140494374: movaps xmm3, xmmword ptr [rbp + 0x1e0]
14049437b: movups xmmword ptr [rbp - 0x28], xmm3
14049437f: movups xmmword ptr [rbp - 0x38], xmm2
140494383: movups xmmword ptr [rbp - 0x48], xmm1
140494387: movaps xmm1, xmmword ptr [rbp + 0x1f0]
14049438e: movaps xmm2, xmmword ptr [rbp + 0x200]
140494395: movaps xmm3, xmmword ptr [rbp + 0x210]
14049439c: movups xmmword ptr [rbp - 0x18], xmm1
1404943a0: movups xmmword ptr [rbp - 8], xmm2
1404943a4: movups xmmword ptr [rbp + 8], xmm3
1404943a8: mov qword ptr [rbp - 0x60], 0
1404943b0: mov qword ptr [rbp - 0x58], 8
1404943b8: mov qword ptr [rbp - 0x50], 0
1404943c0: movups xmmword ptr [rbp + 0x18], xmm8
1404943c5: movups xmmword ptr [rbp + 0x28], xmm9
1404943ca: movups xmmword ptr [rbp + 0x38], xmm0
1404943ce: lea rdx, [rbp - 0x60]
1404943d2: mov rcx, rsi
1404943d5: call 0x140130ba0
1404943da: mov rsi, rax
1404943dd: mov rdi, rax
1404943e0: add rdi, 0x38
1404943e4: movabs rdx, 0x8803ee35630da027
1404943ee: movabs r8, 0x3fed5f4ed6c39038
1404943f8: mov rcx, rdi
1404943fb: call 0x1400dd6f0
140494400: test rax, rax
140494403: je 0x140494438
140494405: cmp qword ptr [rsi + 0x218], rdx
14049440c: jbe 0x140494438
14049440e: mov rax, qword ptr [rsi + 0x210]
140494415: mov rax, qword ptr [rax + rdx*8]
140494419: test rax, rax
14049441c: je 0x140494438
14049441e: mov rcx, qword ptr [rsi + 0x1e0]
140494425: not rax
140494428: lea rax, [rax + rax*2]
14049442c: shl rax, 5
140494430: cmp qword ptr [rcx + rax + 0x20], 0
140494436: jne 0x140494440
140494438: mov rcx, rsi
14049443b: call 0x140580740
140494440: movabs rdx, 0xb0c94e6e65358b73
14049444a: movabs r8, 0x2fa24e902a6ac754
140494454: mov rcx, rdi
140494457: call 0x1400dd6f0
14049445c: test rax, rax
14049445f: je 0x140494494
140494461: cmp qword ptr [rsi + 0x218], rdx
140494468: jbe 0x140494494
14049446a: mov rax, qword ptr [rsi + 0x210]
140494471: mov rax, qword ptr [rax + rdx*8]
140494475: test rax, rax
140494478: je 0x140494494
14049447a: mov rcx, qword ptr [rsi + 0x1e0]
140494481: not rax
140494484: lea rax, [rax + rax*2]
140494488: shl rax, 5
14049448c: cmp qword ptr [rcx + rax + 0x20], 0
140494492: jne 0x14049449c
140494494: mov rcx, rsi
140494497: call 0x140565c50
14049449c: movabs rdx, 0x3a66e0ea923b0b
1404944a6: movabs r8, 0xe43f261c6120cb87
1404944b0: mov rcx, rdi
1404944b3: call 0x1400dd6f0
1404944b8: test rax, rax
1404944bb: je 0x1404944f0
1404944bd: cmp qword ptr [rsi + 0x218], rdx
1404944c4: jbe 0x1404944f0
1404944c6: mov rax, qword ptr [rsi + 0x210]
1404944cd: mov rax, qword ptr [rax + rdx*8]
1404944d1: test rax, rax
1404944d4: je 0x1404944f0
1404944d6: mov rcx, qword ptr [rsi + 0x1e0]
1404944dd: not rax
1404944e0: lea rax, [rax + rax*2]
1404944e4: shl rax, 5
1404944e8: cmp qword ptr [rcx + rax + 0x20], 0
1404944ee: jne 0x1404944f8
1404944f0: mov rcx, rsi
1404944f3: call 0x14057bd00
1404944f8: movabs rdx, 0x6d8612b53a242390
140494502: movabs r8, 0x373774dffaeccc26
14049450c: mov rcx, rdi
14049450f: call 0x1400dd6f0
140494514: test rax, rax
140494517: je 0x14049454c
140494519: cmp qword ptr [rsi + 0x218], rdx
140494520: jbe 0x14049454c
140494522: mov rax, qword ptr [rsi + 0x210]
140494529: mov rax, qword ptr [rax + rdx*8]
14049452d: test rax, rax
140494530: je 0x14049454c
140494532: mov rcx, qword ptr [rsi + 0x1e0]
140494539: not rax
14049453c: lea rax, [rax + rax*2]
140494540: shl rax, 5
140494544: cmp qword ptr [rcx + rax + 0x20], 0
14049454a: jne 0x140494554
14049454c: mov rcx, rsi
14049454f: call 0x140582c60
140494554: movabs rdx, 0x8d948fbffe45eab0
14049455e: movabs r8, 0xe58a404132b1109a
140494568: mov rcx, rdi
14049456b: call 0x1400dd6f0
140494570: test rax, rax
140494573: je 0x1404945a8
140494575: cmp qword ptr [rsi + 0x218], rdx
14049457c: jbe 0x1404945a8
14049457e: mov rax, qword ptr [rsi + 0x210]
140494585: mov rax, qword ptr [rax + rdx*8]
140494589: test rax, rax
14049458c: je 0x1404945a8
14049458e: mov rcx, qword ptr [rsi + 0x1e0]
140494595: not rax
140494598: lea rax, [rax + rax*2]
14049459c: shl rax, 5
1404945a0: cmp qword ptr [rcx + rax + 0x20], 0
1404945a6: jne 0x1404945b0
1404945a8: mov rcx, rsi
1404945ab: call 0x140567b40
1404945b0: movabs rdx, 0x9d80e00ee1d342b3
1404945ba: movabs r8, 0x86cce2dc3f090359
1404945c4: mov rcx, rdi
1404945c7: call 0x1400dd6f0
1404945cc: test rax, rax
1404945cf: je 0x140494604
1404945d1: cmp qword ptr [rsi + 0x218], rdx
1404945d8: jbe 0x140494604
1404945da: mov rax, qword ptr [rsi + 0x210]
1404945e1: mov rax, qword ptr [rax + rdx*8]
1404945e5: test rax, rax
1404945e8: je 0x140494604
1404945ea: mov rcx, qword ptr [rsi + 0x1e0]
1404945f1: not rax
1404945f4: lea rax, [rax + rax*2]
1404945f8: shl rax, 5
1404945fc: cmp qword ptr [rcx + rax + 0x20], 0
140494602: jne 0x14049460c
140494604: mov rcx, rsi
140494607: call 0x140582000
14049460c: movabs rdx, 0xc1faf1912c106825
140494616: movabs r8, 0xf7dd054bc531b239
140494620: mov rcx, rdi
140494623: call 0x1400dd6f0
140494628: test rax, rax
14049462b: je 0x140494660
14049462d: cmp qword ptr [rsi + 0x218], rdx
140494634: jbe 0x140494660
140494636: mov rax, qword ptr [rsi + 0x210]
14049463d: mov rax, qword ptr [rax + rdx*8]
140494641: test rax, rax
140494644: je 0x140494660
140494646: mov rcx, qword ptr [rsi + 0x1e0]
14049464d: not rax
140494650: lea rax, [rax + rax*2]
140494654: shl rax, 5
140494658: cmp qword ptr [rcx + rax + 0x20], 0
14049465e: jne 0x140494668
140494660: mov rcx, rsi
140494663: call 0x140577f20
140494668: movabs rdx, 0xec7dad02bb4e5d35
140494672: movabs r8, 0xa518cd018d654cef
14049467c: mov rcx, rdi
14049467f: call 0x1400dd6f0
140494684: test rax, rax
140494687: je 0x1404946bc
140494689: cmp qword ptr [rsi + 0x218], rdx
140494690: jbe 0x1404946bc
140494692: mov rax, qword ptr [rsi + 0x210]
140494699: mov rax, qword ptr [rax + rdx*8]
14049469d: test rax, rax
1404946a0: je 0x1404946bc
1404946a2: mov rcx, qword ptr [rsi + 0x1e0]
1404946a9: not rax
1404946ac: lea rax, [rax + rax*2]
1404946b0: shl rax, 5
1404946b4: cmp qword ptr [rcx + rax + 0x20], 0
1404946ba: jne 0x1404946c4
1404946bc: mov rcx, rsi
1404946bf: call 0x140574140
1404946c4: movabs rdx, 0x861319290d3fe10f
1404946ce: movabs r8, 0xa639a181d9fb863d
1404946d8: mov rcx, rdi
1404946db: call 0x1400dd6f0
1404946e0: test rax, rax
1404946e3: je 0x140494718
1404946e5: cmp qword ptr [rsi + 0x218], rdx
1404946ec: jbe 0x140494718
1404946ee: mov rax, qword ptr [rsi + 0x210]
1404946f5: mov rax, qword ptr [rax + rdx*8]
1404946f9: test rax, rax
1404946fc: je 0x140494718
1404946fe: mov rcx, qword ptr [rsi + 0x1e0]
140494705: not rax
140494708: lea rax, [rax + rax*2]
14049470c: shl rax, 5
140494710: cmp qword ptr [rcx + rax + 0x20], 0
140494716: jne 0x140494720
140494718: mov rcx, rsi
14049471b: call 0x1405838c0
140494720: movabs rdx, 0xf0f612171767d68f
14049472a: movabs r8, 0xc65fbcfaf262c6be
140494734: mov rcx, rdi
140494737: call 0x1400dd6f0
14049473c: test rax, rax
14049473f: je 0x140494774
140494741: cmp qword ptr [rsi + 0x218], rdx
140494748: jbe 0x140494774
14049474a: mov rax, qword ptr [rsi + 0x210]
140494751: mov rax, qword ptr [rax + rdx*8]
140494755: test rax, rax
140494758: je 0x140494774
14049475a: mov rcx, qword ptr [rsi + 0x1e0]
140494761: not rax
140494764: lea rax, [rax + rax*2]
140494768: shl rax, 5
14049476c: cmp qword ptr [rcx + rax + 0x20], 0
140494772: jne 0x14049477c
140494774: mov rcx, rsi
140494777: call 0x14057e220
14049477c: movabs rdx, 0xf99c083c5514d906
140494786: movabs r8, 0xb9411aab00dbf6d2
140494790: mov rcx, rdi
140494793: call 0x1400dd6f0
140494798: test rax, rax
14049479b: je 0x1404947d0
14049479d: cmp qword ptr [rsi + 0x218], rdx
1404947a4: jbe 0x1404947d0
1404947a6: mov rax, qword ptr [rsi + 0x210]
1404947ad: mov rax, qword ptr [rax + rdx*8]
1404947b1: test rax, rax
1404947b4: je 0x1404947d0
1404947b6: mov rcx, qword ptr [rsi + 0x1e0]
1404947bd: not rax
1404947c0: lea rax, [rax + rax*2]
1404947c4: shl rax, 5
1404947c8: cmp qword ptr [rcx + rax + 0x20], 0
1404947ce: jne 0x1404947d8
1404947d0: mov rcx, rsi
1404947d3: call 0x140567510
1404947d8: movabs rdx, 0xd26f4b443c1c2c89
1404947e2: movabs r8, 0x2f9bfe1536612e9b
1404947ec: mov rcx, rdi
1404947ef: call 0x1400dd6f0
1404947f4: test rax, rax
1404947f7: je 0x14049482c
1404947f9: cmp qword ptr [rsi + 0x218], rdx
140494800: jbe 0x14049482c
140494802: mov rax, qword ptr [rsi + 0x210]
140494809: mov rax, qword ptr [rax + rdx*8]
14049480d: test rax, rax
140494810: je 0x14049482c
140494812: mov rcx, qword ptr [rsi + 0x1e0]
140494819: not rax
14049481c: lea rax, [rax + rax*2]
140494820: shl rax, 5
140494824: cmp qword ptr [rcx + rax + 0x20], 0
14049482a: jne 0x140494834
14049482c: mov rcx, rsi
14049482f: call 0x140580d70
140494834: movabs rdx, 0x7fa25bb007361d49
14049483e: movabs r8, 0xcd3c27d6cfd816a
140494848: mov rcx, rdi
14049484b: call 0x1400dd6f0
140494850: test rax, rax
140494853: je 0x140494888
140494855: cmp qword ptr [rsi + 0x218], rdx
14049485c: jbe 0x140494888
14049485e: mov rax, qword ptr [rsi + 0x210]
140494865: mov rax, qword ptr [rax + rdx*8]
140494869: test rax, rax
14049486c: je 0x140494888
14049486e: mov rcx, qword ptr [rsi + 0x1e0]
140494875: not rax
140494878: lea rax, [rax + rax*2]
14049487c: shl rax, 5
140494880: cmp qword ptr [rcx + rax + 0x20], 0
140494886: jne 0x140494890
140494888: mov rcx, rsi
14049488b: call 0x14057b0a0
140494890: movabs rdx, 0xfddfe3d8acc1440d
14049489a: movabs r8, 0xe93385e29d1c2106
1404948a4: mov rcx, rdi
1404948a7: call 0x1400dd6f0
1404948ac: test rax, rax
1404948af: je 0x1404948e4
1404948b1: cmp qword ptr [rsi + 0x218], rdx
1404948b8: jbe 0x1404948e4
1404948ba: mov rax, qword ptr [rsi + 0x210]
1404948c1: mov rax, qword ptr [rax + rdx*8]
1404948c5: test rax, rax
1404948c8: je 0x1404948e4
1404948ca: mov rcx, qword ptr [rsi + 0x1e0]
1404948d1: not rax
1404948d4: lea rax, [rax + rax*2]
1404948d8: shl rax, 5
1404948dc: cmp qword ptr [rcx + rax + 0x20], 0
1404948e2: jne 0x1404948ec
1404948e4: mov rcx, rsi
1404948e7: call 0x140576660
1404948ec: movabs rdx, 0x70a391b8f514e3f3
1404948f6: movabs r8, 0xf95fcfdda4536172
140494900: mov rcx, rdi
140494903: call 0x1400dd6f0
140494908: test rax, rax
14049490b: je 0x140494940
14049490d: cmp qword ptr [rsi + 0x218], rdx
140494914: jbe 0x140494940
140494916: mov rax, qword ptr [rsi + 0x210]
14049491d: mov rax, qword ptr [rax + rdx*8]
140494921: test rax, rax
140494924: je 0x140494940
140494926: mov rcx, qword ptr [rsi + 0x1e0]
14049492d: not rax
140494930: lea rax, [rax + rax*2]
140494934: shl rax, 5
140494938: cmp qword ptr [rcx + rax + 0x20], 0
14049493e: jne 0x140494948
140494940: mov rcx, rsi
140494943: call 0x140578550
140494948: movabs rdx, 0x2e3306176a07e5d4
140494952: movabs r8, 0x98278cf2af6cd2a4
14049495c: mov rcx, rdi
14049495f: call 0x1400dd6f0
140494964: test rax, rax
140494967: je 0x14049499c
140494969: cmp qword ptr [rsi + 0x218], rdx
140494970: jbe 0x14049499c
140494972: mov rax, qword ptr [rsi + 0x210]
140494979: mov rax, qword ptr [rax + rdx*8]
14049497d: test rax, rax
140494980: je 0x14049499c
140494982: mov rcx, qword ptr [rsi + 0x1e0]
140494989: not rax
14049498c: lea rax, [rax + rax*2]
140494990: shl rax, 5
140494994: cmp qword ptr [rcx + rax + 0x20], 0
14049499a: jne 0x1404949a4
14049499c: mov rcx, rsi
14049499f: call 0x14056cbb0
1404949a4: movabs rdx, 0x9514aca8b8fc2f68
1404949ae: movabs r8, 0x660cf7474a9fda42
1404949b8: mov rcx, rdi
1404949bb: call 0x1400dd6f0
1404949c0: test rax, rax
1404949c3: je 0x1404949f8
1404949c5: cmp qword ptr [rsi + 0x218], rdx
1404949cc: jbe 0x1404949f8
1404949ce: mov rax, qword ptr [rsi + 0x210]
1404949d5: mov rax, qword ptr [rax + rdx*8]
1404949d9: test rax, rax
1404949dc: je 0x1404949f8
1404949de: mov rcx, qword ptr [rsi + 0x1e0]
1404949e5: not rax
1404949e8: lea rax, [rax + rax*2]
1404949ec: shl rax, 5
1404949f0: cmp qword ptr [rcx + rax + 0x20], 0
1404949f6: jne 0x140494a00
1404949f8: mov rcx, rsi
1404949fb: call 0x140576c90
140494a00: movabs rdx, 0x82e26c7dfdccd0b0
140494a0a: movabs r8, 0x33c3eed50a5110f7
140494a14: mov rcx, rdi
140494a17: call 0x1400dd6f0
140494a1c: test rax, rax
140494a1f: je 0x140494a54
140494a21: cmp qword ptr [rsi + 0x218], rdx
140494a28: jbe 0x140494a54
140494a2a: mov rax, qword ptr [rsi + 0x210]
140494a31: mov rax, qword ptr [rax + rdx*8]
140494a35: test rax, rax
140494a38: je 0x140494a54
140494a3a: mov rcx, qword ptr [rsi + 0x1e0]
140494a41: not rax
140494a44: lea rax, [rax + rax*2]
140494a48: shl rax, 5
140494a4c: cmp qword ptr [rcx + rax + 0x20], 0
140494a52: jne 0x140494a5c
140494a54: mov rcx, rsi
140494a57: call 0x14056bf50
140494a5c: movabs rdx, 0xfe2c62427b81ef74
140494a66: movabs r8, 0xfe12651c33d4f8d4
140494a70: mov rcx, rdi
140494a73: call 0x1400dd6f0
140494a78: test rax, rax
140494a7b: je 0x140494ab0
140494a7d: cmp qword ptr [rsi + 0x218], rdx
140494a84: jbe 0x140494ab0
140494a86: mov rax, qword ptr [rsi + 0x210]
140494a8d: mov rax, qword ptr [rax + rdx*8]
140494a91: test rax, rax
140494a94: je 0x140494ab0
140494a96: mov rcx, qword ptr [rsi + 0x1e0]
140494a9d: not rax
140494aa0: lea rax, [rax + rax*2]
140494aa4: shl rax, 5
140494aa8: cmp qword ptr [rcx + rax + 0x20], 0
140494aae: jne 0x140494ab8
140494ab0: mov rcx, rsi
140494ab3: call 0x14056acc0
140494ab8: movabs rdx, 0xbd45698a0818db2
140494ac2: movabs r8, 0x1ac6e022b50a9cfc
140494acc: mov rcx, rdi
140494acf: call 0x1400dd6f0
140494ad4: test rax, rax
140494ad7: je 0x140494b0c
140494ad9: cmp qword ptr [rsi + 0x218], rdx
140494ae0: jbe 0x140494b0c
140494ae2: mov rax, qword ptr [rsi + 0x210]
140494ae9: mov rax, qword ptr [rax + rdx*8]
140494aed: test rax, rax
140494af0: je 0x140494b0c
140494af2: mov rcx, qword ptr [rsi + 0x1e0]
140494af9: not rax
140494afc: lea rax, [rax + rax*2]
140494b00: shl rax, 5
140494b04: cmp qword ptr [rcx + rax + 0x20], 0
140494b0a: jne 0x140494b14
140494b0c: mov rcx, rsi
140494b0f: call 0x140580110
140494b14: movabs rdx, 0x4c979a39f3a96d98
140494b1e: movabs r8, 0x7dca9261f36c173
140494b28: mov rcx, rdi
140494b2b: call 0x1400dd6f0
140494b30: test rax, rax
140494b33: je 0x140494b68
140494b35: cmp qword ptr [rsi + 0x218], rdx
140494b3c: jbe 0x140494b68
140494b3e: mov rax, qword ptr [rsi + 0x210]
140494b45: mov rax, qword ptr [rax + rdx*8]
140494b49: test rax, rax
140494b4c: je 0x140494b68
140494b4e: mov rcx, qword ptr [rsi + 0x1e0]
140494b55: not rax
140494b58: lea rax, [rax + rax*2]
140494b5c: shl rax, 5
140494b60: cmp qword ptr [rcx + rax + 0x20], 0
140494b66: jne 0x140494b70
140494b68: mov rcx, rsi
140494b6b: call 0x140574770
140494b70: movabs rdx, 0xbd0c373b550b02f1
140494b7a: movabs r8, 0xaa5bd86927378744
140494b84: mov rcx, rdi
140494b87: call 0x1400dd6f0
140494b8c: test rax, rax
140494b8f: je 0x140494bc4
140494b91: cmp qword ptr [rsi + 0x218], rdx
140494b98: jbe 0x140494bc4
140494b9a: mov rax, qword ptr [rsi + 0x210]
140494ba1: mov rax, qword ptr [rax + rdx*8]
140494ba5: test rax, rax
140494ba8: je 0x140494bc4
140494baa: mov rcx, qword ptr [rsi + 0x1e0]
140494bb1: not rax
140494bb4: lea rax, [rax + rax*2]
140494bb8: shl rax, 5
140494bbc: cmp qword ptr [rcx + rax + 0x20], 0
140494bc2: jne 0x140494bcc
140494bc4: mov rcx, rsi
140494bc7: call 0x140579e10
140494bcc: mov rax, rsi
140494bcf: movaps xmm6, xmmword ptr [rbp + 0x230]
140494bd6: movaps xmm7, xmmword ptr [rbp + 0x240]
140494bdd: movaps xmm8, xmmword ptr [rbp + 0x250]
140494be5: movaps xmm9, xmmword ptr [rbp + 0x260]
140494bed: add rsp, 0x2f8
140494bf4: pop rbx
140494bf5: pop rdi
140494bf6: pop rsi
140494bf7: pop rbp
140494bf8: ret 
140494bf9: lea rcx, [rip + 0x249de10]
140494c00: call 0x1428d80d0
140494c05: jmp 0x140494c13
140494c07: lea rcx, [rip + 0x249de02]
140494c0e: call 0x1428d80d0
140494c13: ud2 
140494c15: nop word ptr cs:[rax + rax]
140494c20: mov qword ptr [rsp + 0x10], rdx
140494c25: push rbp
140494c26: push rsi
140494c27: push rdi
140494c28: push rbx
140494c29: sub rsp, 0x68
140494c2d: lea rbp, [rdx + 0x80]
140494c34: movaps xmmword ptr [rsp + 0x20], xmm9
140494c3a: movaps xmmword ptr [rsp + 0x30], xmm8
140494c40: movaps xmmword ptr [rsp + 0x40], xmm7
140494c45: movaps xmmword ptr [rsp + 0x50], xmm6
140494c4a: lea rcx, [rbp + 0x1c0]
140494c51: mov r8d, 0x2a8
140494c57: mov r9d, 0x10
140494c5d: lea rdx, [rbp + 0x1e0]
140494c64: call 0x1400523e0
140494c69: movaps xmm6, xmmword ptr [rsp + 0x50]
140494c6e: movaps xmm7, xmmword ptr [rsp + 0x40]
140494c73: movaps xmm8, xmmword ptr [rsp + 0x30]
140494c79: movaps xmm9, xmmword ptr [rsp + 0x20]
140494c7f: add rsp, 0x68
140494c83: pop rbx
140494c84: pop rdi
140494c85: pop rsi
140494c86: pop rbp
140494c87: ret 
140494c88: nop dword ptr [rax + rax]
140494c90: mov qword ptr [rsp + 0x10], rdx
140494c95: push rbp
140494c96: push rsi
140494c97: push rdi
140494c98: push rbx
140494c99: sub rsp, 0x68
140494c9d: lea rbp, [rdx + 0x80]
140494ca4: movaps xmmword ptr [rsp + 0x20], xmm9
140494caa: movaps xmmword ptr [rsp + 0x30], xmm8
140494cb0: movaps xmmword ptr [rsp + 0x40], xmm7
140494cb5: movaps xmmword ptr [rsp + 0x50], xmm6
140494cba: lea rcx, [rbp + 0x1f0]
140494cc1: call 0x140149c70
140494cc6: movaps xmm6, xmmword ptr [rsp + 0x50]
140494ccb: movaps xmm7, xmmword ptr [rsp + 0x40]
140494cd0: movaps xmm8, xmmword ptr [rsp + 0x30]
140494cd6: movaps xmm9, xmmword ptr [rsp + 0x20]
140494cdc: add rsp, 0x68
140494ce0: pop rbx
140494ce1: pop rdi
140494ce2: pop rsi
140494ce3: pop rbp
140494ce4: ret 
140494ce5: nop word ptr cs:[rax + rax]
140494cf0: mov qword ptr [rsp + 0x10], rdx
140494cf5: push rbp
140494cf6: push rsi
140494cf7: push rdi
140494cf8: push rbx
140494cf9: sub rsp, 0x68
140494cfd: lea rbp, [rdx + 0x80]
140494d04: movaps xmmword ptr [rsp + 0x20], xmm9
140494d0a: movaps xmmword ptr [rsp + 0x30], xmm8
140494d10: movaps xmmword ptr [rsp + 0x40], xmm7
140494d15: movaps xmmword ptr [rsp + 0x50], xmm6
140494d1a: lea rcx, [rbp + 0x1f0]
140494d21: mov r8d, 0x20
140494d27: mov r9d, 0x10
140494d2d: lea rdx, [rbp + 0x210]
140494d34: call 0x140050050
140494d39: movaps xmm6, xmmword ptr [rsp + 0x50]
140494d3e: movaps xmm7, xmmword ptr [rsp + 0x40]
140494d43: movaps xmm8, xmmword ptr [rsp + 0x30]
140494d49: movaps xmm9, xmmword ptr [rsp + 0x20]
140494d4f: add rsp, 0x68
140494d53: pop rbx
140494d54: pop rdi
140494d55: pop rsi
140494d56: pop rbp
140494d57: ret 
140494d58: nop dword ptr [rax + rax]
140494d60: mov qword ptr [rsp + 0x10], rdx
140494d65: push rbp
140494d66: push rsi
140494d67: push rdi
140494d68: push rbx
140494d69: sub rsp, 0x68
140494d6d: lea rbp, [rdx + 0x80]
140494d74: movaps xmmword ptr [rsp + 0x20], xmm9
140494d7a: movaps xmmword ptr [rsp + 0x30], xmm8
140494d80: movaps xmmword ptr [rsp + 0x40], xmm7
140494d85: movaps xmmword ptr [rsp + 0x50], xmm6
140494d8a: lea rcx, [rbp + 0x1c0]
140494d91: mov r8d, 0x20
140494d97: mov r9d, 0x10
140494d9d: lea rdx, [rbp + 0x1e0]
140494da4: call 0x1400520b0
140494da9: movaps xmm6, xmmword ptr [rsp + 0x50]
140494dae: movaps xmm7, xmmword ptr [rsp + 0x40]
140494db3: movaps xmm8, xmmword ptr [rsp + 0x30]
140494db9: movaps xmm9, xmmword ptr [rsp + 0x20]
140494dbf: add rsp, 0x68
140494dc3: pop rbx
140494dc4: pop rdi
140494dc5: pop rsi
140494dc6: pop rbp
140494dc7: ret 
140494dc8: nop dword ptr [rax + rax]
140494dd0: mov qword ptr [rsp + 0x10], rdx
140494dd5: push rbp
140494dd6: push rsi
140494dd7: push rdi
140494dd8: push rbx
140494dd9: sub rsp, 0x68
140494ddd: lea rbp, [rdx + 0x80]
140494de4: movaps xmmword ptr [rsp + 0x20], xmm9
140494dea: movaps xmmword ptr [rsp + 0x30], xmm8
140494df0: movaps xmmword ptr [rsp + 0x40], xmm7
140494df5: movaps xmmword ptr [rsp + 0x50], xmm6
140494dfa: lea rcx, [rbp + 0x1f0]
140494e01: mov r8d, 0x20
140494e07: mov r9d, 0x10
140494e0d: lea rdx, [rbp + 0x210]
140494e14: call 0x14004ec10
140494e19: movaps xmm6, xmmword ptr [rsp + 0x50]
140494e1e: movaps xmm7, xmmword ptr [rsp + 0x40]
140494e23: movaps xmm8, xmmword ptr [rsp + 0x30]
140494e29: movaps xmm9, xmmword ptr [rsp + 0x20]
140494e2f: add rsp, 0x68
140494e33: pop rbx
140494e34: pop rdi
140494e35: pop rsi
140494e36: pop rbp
140494e37: ret 
