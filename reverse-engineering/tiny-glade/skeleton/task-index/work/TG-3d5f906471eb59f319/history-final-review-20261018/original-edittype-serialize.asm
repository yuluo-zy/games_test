140316660: push r15
140316662: push r14
140316664: push rsi
140316665: push rdi
140316666: push rbx
140316667: sub rsp, 0x30
14031666b: mov rsi, rdx
14031666e: mov rax, qword ptr [rcx]
140316671: lea rdx, [rip + 0x26a4cc8]
140316678: movsxd rax, dword ptr [rdx + rax*4]
14031667c: add rax, rdx
14031667f: jmp rax
140316681: mov rdi, qword ptr [rcx + 8]
140316685: lea rdx, [rsi + 8]
140316689: mov rbx, qword ptr [rsi]
14031668c: mov rax, qword ptr [rbx + 0x10]
140316690: cmp qword ptr [rbx], rax
140316693: je 0x140316e72
140316699: mov rcx, qword ptr [rbx + 8]
14031669d: mov byte ptr [rcx + rax], 0x7b
1403166a1: inc rax
1403166a4: mov qword ptr [rbx + 0x10], rax
1403166a8: lea r8, [rip + 0x26a4bc4]
1403166af: mov r9d, 7
1403166b5: mov rcx, rsi
1403166b8: call 0x1404b6100
1403166bd: test rax, rax
1403166c0: jne 0x140316c9b
1403166c6: mov rbx, qword ptr [rsi]
1403166c9: mov rdx, qword ptr [rbx + 0x10]
1403166cd: cmp qword ptr [rbx], rdx
1403166d0: je 0x1403170c5
1403166d6: mov rax, qword ptr [rbx + 8]
1403166da: mov byte ptr [rax + rdx], 0x3a
1403166de: inc rdx
1403166e1: mov qword ptr [rbx + 0x10], rdx
1403166e5: add rdi, 0x10
1403166e9: mov rcx, rdi
1403166ec: mov rdx, rsi
1403166ef: call 0x1404f41b0
1403166f4: jmp 0x140316b7d
1403166f9: mov rdi, qword ptr [rcx + 8]
1403166fd: lea rdx, [rsi + 8]
140316701: mov rbx, qword ptr [rsi]
140316704: mov rax, qword ptr [rbx + 0x10]
140316708: cmp qword ptr [rbx], rax
14031670b: je 0x140316cfa
140316711: mov rcx, qword ptr [rbx + 8]
140316715: mov byte ptr [rcx + rax], 0x7b
140316719: inc rax
14031671c: mov qword ptr [rbx + 0x10], rax
140316720: lea r8, [rip + 0x26a4bb9]
140316727: mov r9d, 6
14031672d: mov rcx, rsi
140316730: call 0x1404b6100
140316735: test rax, rax
140316738: jne 0x140316c9b
14031673e: mov rbx, qword ptr [rsi]
140316741: mov rdx, qword ptr [rbx + 0x10]
140316745: cmp qword ptr [rbx], rdx
140316748: je 0x140316fbb
14031674e: mov rax, qword ptr [rbx + 8]
140316752: mov byte ptr [rax + rdx], 0x3a
140316756: inc rdx
140316759: mov qword ptr [rbx + 0x10], rdx
14031675d: add rdi, 0x10
140316761: mov rcx, rdi
140316764: mov rdx, rsi
140316767: call 0x140259af0
14031676c: jmp 0x140316b7d
140316771: lea rdx, [rsi + 8]
140316775: mov rdi, qword ptr [rsi]
140316778: mov rax, qword ptr [rdi + 0x10]
14031677c: cmp qword ptr [rdi], rax
14031677f: je 0x140316d29
140316785: mov rcx, qword ptr [rdi + 8]
140316789: mov byte ptr [rcx + rax], 0x7b
14031678d: inc rax
140316790: mov qword ptr [rdi + 0x10], rax
140316794: lea r8, [rip + 0x26a4b2a]
14031679b: mov r9d, 0xc
1403167a1: mov rcx, rsi
1403167a4: call 0x1404b6100
1403167a9: test rax, rax
1403167ac: jne 0x140316c9b
1403167b2: mov rdi, qword ptr [rsi]
1403167b5: mov rdx, qword ptr [rdi + 0x10]
1403167b9: cmp qword ptr [rdi], rdx
1403167bc: je 0x140316fe1
1403167c2: mov rax, qword ptr [rdi + 8]
1403167c6: mov byte ptr [rax + rdx], 0x3a
1403167ca: inc rdx
1403167cd: mov qword ptr [rdi + 0x10], rdx
1403167d1: mov rdi, qword ptr [rsi]
1403167d4: mov rdx, qword ptr [rdi + 0x10]
1403167d8: cmp qword ptr [rdi], rdx
1403167db: je 0x140317007
1403167e1: mov rax, qword ptr [rdi + 8]
1403167e5: mov byte ptr [rax + rdx], 0x7b
1403167e9: inc rdx
1403167ec: mov qword ptr [rdi + 0x10], rdx
1403167f0: cmp qword ptr [rdi], rdx
1403167f3: je 0x14031702d
1403167f9: mov rax, qword ptr [rdi + 8]
1403167fd: mov byte ptr [rax + rdx], 0x7d
140316801: inc rdx
140316804: mov qword ptr [rdi + 0x10], rdx
140316808: jmp 0x140316b82
14031680d: mov rdi, qword ptr [rcx + 8]
140316811: lea rdx, [rsi + 8]
140316815: mov rbx, qword ptr [rsi]
140316818: mov rax, qword ptr [rbx + 0x10]
14031681c: cmp qword ptr [rbx], rax
14031681f: je 0x140316d58
140316825: mov rcx, qword ptr [rbx + 8]
140316829: mov byte ptr [rcx + rax], 0x7b
14031682d: inc rax
140316830: mov qword ptr [rbx + 0x10], rax
140316834: lea r8, [rip + 0x26a4a5b]
14031683b: mov r9d, 0xc
140316841: jmp 0x140316b3e
140316846: mov rdi, qword ptr [rcx + 8]
14031684a: lea rdx, [rsi + 8]
14031684e: mov rbx, qword ptr [rsi]
140316851: mov rax, qword ptr [rbx + 0x10]
140316855: cmp qword ptr [rbx], rax
140316858: je 0x140316d87
14031685e: mov rcx, qword ptr [rbx + 8]
140316862: mov byte ptr [rcx + rax], 0x7b
140316866: inc rax
140316869: mov qword ptr [rbx + 0x10], rax
14031686d: lea r8, [rip + 0x26a4a5d]
140316874: mov r9d, 4
14031687a: mov rcx, rsi
14031687d: call 0x1404b6100
140316882: test rax, rax
140316885: jne 0x140316c9b
14031688b: mov rbx, qword ptr [rsi]
14031688e: mov rdx, qword ptr [rbx + 0x10]
140316892: cmp qword ptr [rbx], rdx
140316895: je 0x140317053
14031689b: mov rax, qword ptr [rbx + 8]
14031689f: mov byte ptr [rax + rdx], 0x3a
1403168a3: inc rdx
1403168a6: mov qword ptr [rbx + 0x10], rdx
1403168aa: add rdi, 0x10
1403168ae: mov rcx, rdi
1403168b1: mov rdx, rsi
1403168b4: call 0x14003d650
1403168b9: jmp 0x140316b7d
1403168be: mov rdi, qword ptr [rcx + 8]
1403168c2: lea rdx, [rsi + 8]
1403168c6: mov rbx, qword ptr [rsi]
1403168c9: mov rax, qword ptr [rbx + 0x10]
1403168cd: cmp qword ptr [rbx], rax
1403168d0: je 0x140316db6
1403168d6: mov rcx, qword ptr [rbx + 8]
1403168da: mov byte ptr [rcx + rax], 0x7b
1403168de: inc rax
1403168e1: mov qword ptr [rbx + 0x10], rax
1403168e5: lea r8, [rip + 0x26a49c9]
1403168ec: mov r9d, 5
1403168f2: mov rcx, rsi
1403168f5: call 0x1404b6100
1403168fa: test rax, rax
1403168fd: jne 0x140316c9b
140316903: mov rbx, qword ptr [rsi]
140316906: mov rdx, qword ptr [rbx + 0x10]
14031690a: cmp qword ptr [rbx], rdx
14031690d: je 0x140317079
140316913: mov rax, qword ptr [rbx + 8]
140316917: mov byte ptr [rax + rdx], 0x3a
14031691b: inc rdx
14031691e: mov qword ptr [rbx + 0x10], rdx
140316922: add rdi, 0x10
140316926: mov rcx, rdi
140316929: mov rdx, rsi
14031692c: call 0x14003c5d0
140316931: jmp 0x140316b7d
140316936: mov rdi, qword ptr [rcx + 8]
14031693a: lea rdx, [rsi + 8]
14031693e: mov rbx, qword ptr [rsi]
140316941: mov rax, qword ptr [rbx + 0x10]
140316945: cmp qword ptr [rbx], rax
140316948: je 0x140316de5
14031694e: mov rcx, qword ptr [rbx + 8]
140316952: mov byte ptr [rcx + rax], 0x7b
140316956: inc rax
140316959: mov qword ptr [rbx + 0x10], rax
14031695d: lea r8, [rip + 0x26a491a]
140316964: mov r9d, 0xc
14031696a: jmp 0x140316b3e
14031696f: mov rdi, qword ptr [rcx + 8]
140316973: lea rdx, [rsi + 8]
140316977: mov rbx, qword ptr [rsi]
14031697a: mov rax, qword ptr [rbx + 0x10]
14031697e: cmp qword ptr [rbx], rax
140316981: je 0x140316e14
140316987: mov rcx, qword ptr [rbx + 8]
14031698b: mov byte ptr [rcx + rax], 0x7b
14031698f: inc rax
140316992: mov qword ptr [rbx + 0x10], rax
140316996: lea r8, [rip + 0x26a48ed]
14031699d: mov r9d, 0xc
1403169a3: jmp 0x140316b3e
1403169a8: mov rdi, qword ptr [rcx + 8]
1403169ac: mov rbx, qword ptr [rcx + 0x10]
1403169b0: lea rdx, [rsi + 8]
1403169b4: mov r14, qword ptr [rsi]
1403169b7: mov rax, qword ptr [r14 + 0x10]
1403169bb: cmp qword ptr [r14], rax
1403169be: je 0x140316e43
1403169c4: mov rcx, qword ptr [r14 + 8]
1403169c8: mov byte ptr [rcx + rax], 0x7b
1403169cc: inc rax
1403169cf: mov qword ptr [r14 + 0x10], rax
1403169d3: lea r8, [rip + 0x26a48d3]
1403169da: mov r9d, 4
1403169e0: mov rcx, rsi
1403169e3: call 0x1404b6100
1403169e8: test rax, rax
1403169eb: jne 0x140316c9b
1403169f1: mov r14, qword ptr [rsi]
1403169f4: mov rdx, qword ptr [r14 + 0x10]
1403169f8: cmp qword ptr [r14], rdx
1403169fb: je 0x14031709f
140316a01: mov rax, qword ptr [r14 + 8]
140316a05: mov byte ptr [rax + rdx], 0x3a
140316a09: inc rdx
140316a0c: mov qword ptr [r14 + 0x10], rdx
140316a10: mov rcx, rsi
140316a13: mov rdx, rdi
140316a16: mov r8, rbx
140316a19: call 0x1404b7430
140316a1e: jmp 0x140316b7d
140316a23: mov rdi, qword ptr [rcx + 8]
140316a27: lea rdx, [rsi + 8]
140316a2b: mov rbx, qword ptr [rsi]
140316a2e: mov rax, qword ptr [rbx + 0x10]
140316a32: cmp qword ptr [rbx], rax
140316a35: je 0x140316ea1
140316a3b: mov rcx, qword ptr [rbx + 8]
140316a3f: mov byte ptr [rcx + rax], 0x7b
140316a43: inc rax
140316a46: mov qword ptr [rbx + 0x10], rax
140316a4a: lea r8, [rip + 0x26a4829]
140316a51: mov r9d, 4
140316a57: jmp 0x140316b3e
140316a5c: mov rdi, qword ptr [rcx + 8]
140316a60: lea rdx, [rsi + 8]
140316a64: mov rbx, qword ptr [rsi]
140316a67: mov rax, qword ptr [rbx + 0x10]
140316a6b: cmp qword ptr [rbx], rax
140316a6e: je 0x140316ed0
140316a74: mov rcx, qword ptr [rbx + 8]
140316a78: mov byte ptr [rcx + rax], 0x7b
140316a7c: inc rax
140316a7f: mov qword ptr [rbx + 0x10], rax
140316a83: lea r8, [rip + 0x26a4818]
140316a8a: mov r9d, 5
140316a90: jmp 0x140316b3e
140316a95: mov rdi, qword ptr [rcx + 8]
140316a99: lea rdx, [rsi + 8]
140316a9d: mov rbx, qword ptr [rsi]
140316aa0: mov rax, qword ptr [rbx + 0x10]
140316aa4: cmp qword ptr [rbx], rax
140316aa7: je 0x140316eff
140316aad: mov rcx, qword ptr [rbx + 8]
140316ab1: mov byte ptr [rcx + rax], 0x7b
140316ab5: inc rax
140316ab8: mov qword ptr [rbx + 0x10], rax
140316abc: lea r8, [rip + 0x26a4812]
140316ac3: mov r9d, 0xb
140316ac9: mov rcx, rsi
140316acc: call 0x1404b6100
140316ad1: test rax, rax
140316ad4: jne 0x140316c9b
140316ada: mov rbx, qword ptr [rsi]
140316add: mov rdx, qword ptr [rbx + 0x10]
140316ae1: cmp qword ptr [rbx], rdx
140316ae4: je 0x1403170eb
140316aea: mov rax, qword ptr [rbx + 8]
140316aee: mov byte ptr [rax + rdx], 0x3a
140316af2: inc rdx
140316af5: mov qword ptr [rbx + 0x10], rdx
140316af9: add rdi, 0x10
140316afd: mov rcx, rdi
140316b00: mov rdx, rsi
140316b03: call 0x14003d300
140316b08: jmp 0x140316b7d
140316b0a: mov rdi, qword ptr [rcx + 8]
140316b0e: lea rdx, [rsi + 8]
140316b12: mov rbx, qword ptr [rsi]
140316b15: mov rax, qword ptr [rbx + 0x10]
140316b19: cmp qword ptr [rbx], rax
140316b1c: je 0x140316f2e
140316b22: mov rcx, qword ptr [rbx + 8]
140316b26: mov byte ptr [rcx + rax], 0x7b
140316b2a: inc rax
140316b2d: mov qword ptr [rbx + 0x10], rax
140316b31: lea r8, [rip + 0x26a476f]
140316b38: mov r9d, 6
140316b3e: mov rcx, rsi
140316b41: call 0x1404b6100
140316b46: test rax, rax
140316b49: jne 0x140316c9b
140316b4f: mov rbx, qword ptr [rsi]
140316b52: mov rdx, qword ptr [rbx + 0x10]
140316b56: cmp qword ptr [rbx], rdx
140316b59: je 0x140316cd4
140316b5f: mov rax, qword ptr [rbx + 8]
140316b63: mov byte ptr [rax + rdx], 0x3a
140316b67: inc rdx
140316b6a: mov qword ptr [rbx + 0x10], rdx
140316b6e: add rdi, 0x10
140316b72: mov rcx, rdi
140316b75: mov rdx, rsi
140316b78: call 0x1404896f0
140316b7d: test rax, rax
140316b80: jne 0x140316ba3
140316b82: mov rsi, qword ptr [rsi]
140316b85: mov rdx, qword ptr [rsi + 0x10]
140316b89: cmp qword ptr [rsi], rdx
140316b8c: je 0x140316cae
140316b92: mov rax, qword ptr [rsi + 8]
140316b96: mov byte ptr [rax + rdx], 0x7d
140316b9a: inc rdx
140316b9d: mov qword ptr [rsi + 0x10], rdx
140316ba1: xor eax, eax
140316ba3: add rsp, 0x30
140316ba7: pop rbx
140316ba8: pop rdi
140316ba9: pop rsi
140316baa: pop r14
140316bac: pop r15
140316bae: ret 
140316baf: mov rdi, qword ptr [rcx + 8]
140316bb3: lea rdx, [rsi + 8]
140316bb7: mov rbx, qword ptr [rsi]
140316bba: mov rax, qword ptr [rbx + 0x10]
140316bbe: cmp qword ptr [rbx], rax
140316bc1: je 0x140316f5d
140316bc7: mov rcx, qword ptr [rbx + 8]
140316bcb: mov byte ptr [rcx + rax], 0x7b
140316bcf: inc rax
140316bd2: mov qword ptr [rbx + 0x10], rax
140316bd6: lea r8, [rip + 0x26a46dd]
140316bdd: mov r9d, 0xb
140316be3: mov rcx, rsi
140316be6: call 0x1404b6100
140316beb: test rax, rax
140316bee: jne 0x140316c9b
140316bf4: mov rbx, qword ptr [rsi]
140316bf7: mov rdx, qword ptr [rbx + 0x10]
140316bfb: cmp qword ptr [rbx], rdx
140316bfe: je 0x140317111
140316c04: mov rax, qword ptr [rbx + 8]
140316c08: mov byte ptr [rax + rdx], 0x3a
140316c0c: inc rdx
140316c0f: mov qword ptr [rbx + 0x10], rdx
140316c13: add rdi, 0x10
140316c17: mov rcx, rdi
140316c1a: mov rdx, rsi
140316c1d: call 0x1404f47c0
140316c22: jmp 0x140316b7d
140316c27: mov rdi, qword ptr [rcx + 8]
140316c2b: lea rdx, [rsi + 8]
140316c2f: mov rbx, qword ptr [rsi]
140316c32: mov rax, qword ptr [rbx + 0x10]
140316c36: cmp qword ptr [rbx], rax
140316c39: je 0x140316f8c
140316c3f: mov rcx, qword ptr [rbx + 8]
140316c43: mov byte ptr [rcx + rax], 0x7b
140316c47: inc rax
140316c4a: mov qword ptr [rbx + 0x10], rax
140316c4e: lea r8, [rip + 0x26a465c]
140316c55: mov r9d, 4
140316c5b: mov rcx, rsi
140316c5e: call 0x1404b6100
140316c63: test rax, rax
140316c66: jne 0x140316c9b
140316c68: mov rbx, qword ptr [rsi]
140316c6b: mov rdx, qword ptr [rbx + 0x10]
140316c6f: cmp qword ptr [rbx], rdx
140316c72: je 0x140317137
140316c78: mov rax, qword ptr [rbx + 8]
140316c7c: mov byte ptr [rax + rdx], 0x3a
140316c80: inc rdx
140316c83: mov qword ptr [rbx + 0x10], rdx
140316c87: add rdi, 0x10
140316c8b: mov rcx, rdi
140316c8e: mov rdx, rsi
140316c91: call 0x140259e30
140316c96: jmp 0x140316b7d
140316c9b: mov rcx, rax
140316c9e: add rsp, 0x30
140316ca2: pop rbx
140316ca3: pop rdi
140316ca4: pop rsi
140316ca5: pop r14
140316ca7: pop r15
140316ca9: jmp 0x1428c98a0
140316cae: mov qword ptr [rsp + 0x20], 1
140316cb7: mov r8d, 1
140316cbd: mov r9d, 1
140316cc3: mov rcx, rsi
140316cc6: call 0x1428a29e0
140316ccb: mov rdx, qword ptr [rsi + 0x10]
140316ccf: jmp 0x140316b92
140316cd4: mov qword ptr [rsp + 0x20], 1
140316cdd: mov r8d, 1
140316ce3: mov r9d, 1
140316ce9: mov rcx, rbx
140316cec: call 0x1428a29e0
140316cf1: mov rdx, qword ptr [rbx + 0x10]
140316cf5: jmp 0x140316b5f
140316cfa: mov qword ptr [rsp + 0x20], 1
140316d03: mov r8d, 1
140316d09: mov r9d, 1
140316d0f: mov rcx, rbx
140316d12: mov r14, rdx
140316d15: mov rdx, rax
140316d18: call 0x1428a29e0
140316d1d: mov rdx, r14
140316d20: mov rax, qword ptr [rbx + 0x10]
140316d24: jmp 0x140316711
140316d29: mov qword ptr [rsp + 0x20], 1
140316d32: mov r8d, 1
140316d38: mov r9d, 1
140316d3e: mov rcx, rdi
140316d41: mov rbx, rdx
140316d44: mov rdx, rax
140316d47: call 0x1428a29e0
140316d4c: mov rdx, rbx
140316d4f: mov rax, qword ptr [rdi + 0x10]
140316d53: jmp 0x140316785
140316d58: mov qword ptr [rsp + 0x20], 1
140316d61: mov r8d, 1
140316d67: mov r9d, 1
140316d6d: mov rcx, rbx
140316d70: mov r14, rdx
140316d73: mov rdx, rax
140316d76: call 0x1428a29e0
140316d7b: mov rdx, r14
140316d7e: mov rax, qword ptr [rbx + 0x10]
140316d82: jmp 0x140316825
140316d87: mov qword ptr [rsp + 0x20], 1
140316d90: mov r8d, 1
140316d96: mov r9d, 1
140316d9c: mov rcx, rbx
140316d9f: mov r14, rdx
140316da2: mov rdx, rax
140316da5: call 0x1428a29e0
140316daa: mov rdx, r14
140316dad: mov rax, qword ptr [rbx + 0x10]
140316db1: jmp 0x14031685e
140316db6: mov qword ptr [rsp + 0x20], 1
140316dbf: mov r8d, 1
140316dc5: mov r9d, 1
140316dcb: mov rcx, rbx
140316dce: mov r14, rdx
140316dd1: mov rdx, rax
140316dd4: call 0x1428a29e0
140316dd9: mov rdx, r14
140316ddc: mov rax, qword ptr [rbx + 0x10]
140316de0: jmp 0x1403168d6
140316de5: mov qword ptr [rsp + 0x20], 1
140316dee: mov r8d, 1
140316df4: mov r9d, 1
140316dfa: mov rcx, rbx
140316dfd: mov r14, rdx
140316e00: mov rdx, rax
140316e03: call 0x1428a29e0
140316e08: mov rdx, r14
140316e0b: mov rax, qword ptr [rbx + 0x10]
140316e0f: jmp 0x14031694e
140316e14: mov qword ptr [rsp + 0x20], 1
140316e1d: mov r8d, 1
140316e23: mov r9d, 1
140316e29: mov rcx, rbx
140316e2c: mov r14, rdx
140316e2f: mov rdx, rax
140316e32: call 0x1428a29e0
140316e37: mov rdx, r14
140316e3a: mov rax, qword ptr [rbx + 0x10]
140316e3e: jmp 0x140316987
140316e43: mov qword ptr [rsp + 0x20], 1
140316e4c: mov r8d, 1
140316e52: mov r9d, 1
140316e58: mov rcx, r14
140316e5b: mov r15, rdx
140316e5e: mov rdx, rax
140316e61: call 0x1428a29e0
140316e66: mov rdx, r15
140316e69: mov rax, qword ptr [r14 + 0x10]
140316e6d: jmp 0x1403169c4
140316e72: mov qword ptr [rsp + 0x20], 1
140316e7b: mov r8d, 1
140316e81: mov r9d, 1
140316e87: mov rcx, rbx
140316e8a: mov r14, rdx
140316e8d: mov rdx, rax
140316e90: call 0x1428a29e0
140316e95: mov rdx, r14
140316e98: mov rax, qword ptr [rbx + 0x10]
140316e9c: jmp 0x140316699
140316ea1: mov qword ptr [rsp + 0x20], 1
140316eaa: mov r8d, 1
140316eb0: mov r9d, 1
140316eb6: mov rcx, rbx
140316eb9: mov r14, rdx
140316ebc: mov rdx, rax
140316ebf: call 0x1428a29e0
140316ec4: mov rdx, r14
140316ec7: mov rax, qword ptr [rbx + 0x10]
140316ecb: jmp 0x140316a3b
140316ed0: mov qword ptr [rsp + 0x20], 1
140316ed9: mov r8d, 1
140316edf: mov r9d, 1
140316ee5: mov rcx, rbx
140316ee8: mov r14, rdx
140316eeb: mov rdx, rax
140316eee: call 0x1428a29e0
140316ef3: mov rdx, r14
140316ef6: mov rax, qword ptr [rbx + 0x10]
140316efa: jmp 0x140316a74
140316eff: mov qword ptr [rsp + 0x20], 1
140316f08: mov r8d, 1
140316f0e: mov r9d, 1
140316f14: mov rcx, rbx
140316f17: mov r14, rdx
140316f1a: mov rdx, rax
140316f1d: call 0x1428a29e0
140316f22: mov rdx, r14
140316f25: mov rax, qword ptr [rbx + 0x10]
140316f29: jmp 0x140316aad
140316f2e: mov qword ptr [rsp + 0x20], 1
140316f37: mov r8d, 1
140316f3d: mov r9d, 1
140316f43: mov rcx, rbx
140316f46: mov r14, rdx
140316f49: mov rdx, rax
140316f4c: call 0x1428a29e0
140316f51: mov rdx, r14
140316f54: mov rax, qword ptr [rbx + 0x10]
140316f58: jmp 0x140316b22
140316f5d: mov qword ptr [rsp + 0x20], 1
140316f66: mov r8d, 1
140316f6c: mov r9d, 1
140316f72: mov rcx, rbx
140316f75: mov r14, rdx
140316f78: mov rdx, rax
140316f7b: call 0x1428a29e0
140316f80: mov rdx, r14
140316f83: mov rax, qword ptr [rbx + 0x10]
140316f87: jmp 0x140316bc7
140316f8c: mov qword ptr [rsp + 0x20], 1
140316f95: mov r8d, 1
140316f9b: mov r9d, 1
140316fa1: mov rcx, rbx
140316fa4: mov r14, rdx
140316fa7: mov rdx, rax
140316faa: call 0x1428a29e0
140316faf: mov rdx, r14
140316fb2: mov rax, qword ptr [rbx + 0x10]
140316fb6: jmp 0x140316c3f
140316fbb: mov qword ptr [rsp + 0x20], 1
140316fc4: mov r8d, 1
140316fca: mov r9d, 1
140316fd0: mov rcx, rbx
140316fd3: call 0x1428a29e0
140316fd8: mov rdx, qword ptr [rbx + 0x10]
140316fdc: jmp 0x14031674e
140316fe1: mov qword ptr [rsp + 0x20], 1
140316fea: mov r8d, 1
140316ff0: mov r9d, 1
140316ff6: mov rcx, rdi
140316ff9: call 0x1428a29e0
140316ffe: mov rdx, qword ptr [rdi + 0x10]
140317002: jmp 0x1403167c2
140317007: mov qword ptr [rsp + 0x20], 1
140317010: mov r8d, 1
140317016: mov r9d, 1
14031701c: mov rcx, rdi
14031701f: call 0x1428a29e0
140317024: mov rdx, qword ptr [rdi + 0x10]
140317028: jmp 0x1403167e1
14031702d: mov qword ptr [rsp + 0x20], 1
140317036: mov r8d, 1
14031703c: mov r9d, 1
140317042: mov rcx, rdi
140317045: call 0x1428a29e0
14031704a: mov rdx, qword ptr [rdi + 0x10]
14031704e: jmp 0x1403167f9
140317053: mov qword ptr [rsp + 0x20], 1
14031705c: mov r8d, 1
140317062: mov r9d, 1
140317068: mov rcx, rbx
14031706b: call 0x1428a29e0
140317070: mov rdx, qword ptr [rbx + 0x10]
140317074: jmp 0x14031689b
140317079: mov qword ptr [rsp + 0x20], 1
140317082: mov r8d, 1
140317088: mov r9d, 1
14031708e: mov rcx, rbx
140317091: call 0x1428a29e0
140317096: mov rdx, qword ptr [rbx + 0x10]
14031709a: jmp 0x140316913
14031709f: mov qword ptr [rsp + 0x20], 1
1403170a8: mov r8d, 1
1403170ae: mov r9d, 1
1403170b4: mov rcx, r14
1403170b7: call 0x1428a29e0
1403170bc: mov rdx, qword ptr [r14 + 0x10]
1403170c0: jmp 0x140316a01
1403170c5: mov qword ptr [rsp + 0x20], 1
1403170ce: mov r8d, 1
1403170d4: mov r9d, 1
1403170da: mov rcx, rbx
1403170dd: call 0x1428a29e0
1403170e2: mov rdx, qword ptr [rbx + 0x10]
1403170e6: jmp 0x1403166d6
1403170eb: mov qword ptr [rsp + 0x20], 1
1403170f4: mov r8d, 1
1403170fa: mov r9d, 1
140317100: mov rcx, rbx
140317103: call 0x1428a29e0
140317108: mov rdx, qword ptr [rbx + 0x10]
14031710c: jmp 0x140316aea
140317111: mov qword ptr [rsp + 0x20], 1
14031711a: mov r8d, 1
140317120: mov r9d, 1
140317126: mov rcx, rbx
140317129: call 0x1428a29e0
14031712e: mov rdx, qword ptr [rbx + 0x10]
140317132: jmp 0x140316c04
140317137: mov qword ptr [rsp + 0x20], 1
140317140: mov r8d, 1
140317146: mov r9d, 1
14031714c: mov rcx, rbx
14031714f: call 0x1428a29e0
140317154: mov rdx, qword ptr [rbx + 0x10]
140317158: jmp 0x140316c78
