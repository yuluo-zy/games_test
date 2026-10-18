0x1412bc1f0 55 push rbp
0x1412bc1f1 56 push rsi
0x1412bc1f2 4881ecd8000000 sub rsp, 0xd8
0x1412bc1f9 488dac2480000000 lea rbp, [rsp + 0x80]
0x1412bc201 48c74550feffffff mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
0x1412bc209 4889ce mov rsi, rcx
0x1412bc20c 49c1e104 shl r9, 4
0x1412bc210 4f8d0c49 lea r9, [r9 + r9*2]
0x1412bc214 488d4da8 lea rcx, [rbp - 0x58]
0x1412bc218 e883593500 call 0x141611ba0
0x1412bc21d 0fb605e5893702 movzx eax, byte ptr [rip + 0x23789e5]
0x1412bc224 b928000000 mov ecx, 0x28
0x1412bc229 ba08000000 mov edx, 8
0x1412bc22e e8dd7935ff call 0x140613c10
0x1412bc233 4885c0 test rax, rax
0x1412bc236 0f84b0000000 je 0x1412bc2ec
0x1412bc23c 488b4dc8 mov rcx, qword ptr [rbp - 0x38]
0x1412bc240 48894820 mov qword ptr [rax + 0x20], rcx
0x1412bc244 0f1045a8 movups xmm0, xmmword ptr [rbp - 0x58]
0x1412bc248 0f104db8 movups xmm1, xmmword ptr [rbp - 0x48]
0x1412bc24c 0f114810 movups xmmword ptr [rax + 0x10], xmm1
0x1412bc250 0f1100 movups xmmword ptr [rax], xmm0
0x1412bc253 488d0dc42a7e01 lea rcx, [rip + 0x17e2ac4]
0x1412bc25a 48894d00 mov qword ptr [rbp], rcx
0x1412bc25e 48c745083e000000 mov qword ptr [rbp + 8], 0x3e
0x1412bc266 48894510 mov qword ptr [rbp + 0x10], rax
0x1412bc26a 488d050f039701 lea rax, [rip + 0x197030f]
0x1412bc271 48894518 mov qword ptr [rbp + 0x18], rax
0x1412bc275 48c7453000000000 mov qword ptr [rbp + 0x30], 0
0x1412bc27d c6454605 mov byte ptr [rbp + 0x46], 5
0x1412bc281 48894d20 mov qword ptr [rbp + 0x20], rcx
0x1412bc285 48c745283e000000 mov qword ptr [rbp + 0x28], 0x3e
0x1412bc28d 0fb60575893702 movzx eax, byte ptr [rip + 0x2378975]
0x1412bc294 b940000000 mov ecx, 0x40
0x1412bc299 ba08000000 mov edx, 8
0x1412bc29e e86d7935ff call 0x140613c10
0x1412bc2a3 4885c0 test rax, rax
0x1412bc2a6 7455 je 0x1412bc2fd
0x1412bc2a8 0f104510 movups xmm0, xmmword ptr [rbp + 0x10]
0x1412bc2ac 0f104d20 movups xmm1, xmmword ptr [rbp + 0x20]
0x1412bc2b0 0f105530 movups xmm2, xmmword ptr [rbp + 0x30]
0x1412bc2b4 0f105d40 movups xmm3, xmmword ptr [rbp + 0x40]
0x1412bc2b8 0f115830 movups xmmword ptr [rax + 0x30], xmm3
0x1412bc2bc 0f115020 movups xmmword ptr [rax + 0x20], xmm2
0x1412bc2c0 0f114810 movups xmmword ptr [rax + 0x10], xmm1
0x1412bc2c4 0f1100 movups xmmword ptr [rax], xmm0
0x1412bc2c7 48babb3664bf04425c1c movabs rdx, 0x1c5c4204bf6436bb
0x1412bc2d1 4889f1 mov rcx, rsi
0x1412bc2d4 4989c0 mov r8, rax
0x1412bc2d7 e814d96aff call 0x140969bf0
0x1412bc2dc 4883f801 cmp rax, 1
0x1412bc2e0 742c je 0x1412bc30e
0x1412bc2e2 4881c4d8000000 add rsp, 0xd8
0x1412bc2e9 5e pop rsi
0x1412bc2ea 5d pop rbp
0x1412bc2eb c3 ret 
0x1412bc2ec b908000000 mov ecx, 8
0x1412bc2f1 ba28000000 mov edx, 0x28
0x1412bc2f6 e805cd6101 call 0x1428d9000
0x1412bc2fb eb0f jmp 0x1412bc30c
0x1412bc2fd b908000000 mov ecx, 8
0x1412bc302 ba40000000 mov edx, 0x40
0x1412bc307 e8f4cc6101 call 0x1428d9000
0x1412bc30c 0f0b ud2 
0x1412bc30e 4889d0 mov rax, rdx
0x1412bc311 4883c210 add rdx, 0x10
0x1412bc315 4889d6 mov rsi, rdx
0x1412bc318 488378183e cmp qword ptr [rax + 0x18], 0x3e
0x1412bc31d 757c jne 0x1412bc39b
0x1412bc31f 488b16 mov rdx, qword ptr [rsi]
0x1412bc322 488d0df5297e01 lea rcx, [rip + 0x17e29f5]
0x1412bc329 41b83e000000 mov r8d, 0x3e
0x1412bc32f e8bc786601 call 0x142923bf0
0x1412bc334 85c0 test eax, eax
0x1412bc336 7563 jne 0x1412bc39b
0x1412bc338 488d05111b9701 lea rax, [rip + 0x1971b11]
0x1412bc33f 488945d0 mov qword ptr [rbp - 0x30], rax
0x1412bc343 488d05567527ff lea rax, [rip - 0xd88aaa]
0x1412bc34a 488945d8 mov qword ptr [rbp - 0x28], rax
0x1412bc34e 4889e8 mov rax, rbp
0x1412bc351 488945e0 mov qword ptr [rbp - 0x20], rax
0x1412bc355 488d0534dbe4fe lea rax, [rip - 0x11b24cc]
0x1412bc35c 488945e8 mov qword ptr [rbp - 0x18], rax
0x1412bc360 488d05c1129701 lea rax, [rip + 0x19712c1]
0x1412bc367 48894510 mov qword ptr [rbp + 0x10], rax
0x1412bc36b 48c7451803000000 mov qword ptr [rbp + 0x18], 3
0x1412bc373 48c7453000000000 mov qword ptr [rbp + 0x30], 0
0x1412bc37b 488d45d0 lea rax, [rbp - 0x30]
0x1412bc37f 48894520 mov qword ptr [rbp + 0x20], rax
0x1412bc383 48c7452802000000 mov qword ptr [rbp + 0x28], 2
0x1412bc38b 488d15c6129701 lea rdx, [rip + 0x19712c6]
0x1412bc392 488d4d10 lea rcx, [rbp + 0x10]
0x1412bc396 e8f5cf6101 call 0x1428d9390
0x1412bc39b 488d05ae1a9701 lea rax, [rip + 0x1971aae]
0x1412bc3a2 48894510 mov qword ptr [rbp + 0x10], rax
0x1412bc3a6 488d05f37427ff lea rax, [rip - 0xd88b0d]
0x1412bc3ad 48894518 mov qword ptr [rbp + 0x18], rax
0x1412bc3b1 48897520 mov qword ptr [rbp + 0x20], rsi
0x1412bc3b5 488d05d4dae4fe lea rax, [rip - 0x11b252c]
0x1412bc3bc 48894528 mov qword ptr [rbp + 0x28], rax
0x1412bc3c0 4889e9 mov rcx, rbp
0x1412bc3c3 48894d30 mov qword ptr [rbp + 0x30], rcx
0x1412bc3c7 48894538 mov qword ptr [rbp + 0x38], rax
0x1412bc3cb 488d0596119701 lea rax, [rip + 0x1971196]
0x1412bc3d2 488945d0 mov qword ptr [rbp - 0x30], rax
0x1412bc3d6 48c745d803000000 mov qword ptr [rbp - 0x28], 3
0x1412bc3de 48c745f000000000 mov qword ptr [rbp - 0x10], 0
0x1412bc3e6 488d4510 lea rax, [rbp + 0x10]
0x1412bc3ea 488945e0 mov qword ptr [rbp - 0x20], rax
0x1412bc3ee 48c745e803000000 mov qword ptr [rbp - 0x18], 3
0x1412bc3f6 488d15d3119701 lea rdx, [rip + 0x19711d3]
0x1412bc3fd 488d4dd0 lea rcx, [rbp - 0x30]
0x1412bc401 e88acf6101 call 0x1428d9390
0x1412bc406 cc int3 
0x1412bc407 660f1f840000000000 nop word ptr [rax + rax]
0x1412bc410 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1412bc415 55 push rbp
0x1412bc416 56 push rsi
0x1412bc417 4883ec28 sub rsp, 0x28
0x1412bc41b 488daa80000000 lea rbp, [rdx + 0x80]
0x1412bc422 488d4da8 lea rcx, [rbp - 0x58]
0x1412bc426 e8856092ff call 0x140be24b0
0x1412bc42b 90 nop 
0x1412bc42c 4883c428 add rsp, 0x28
0x1412bc430 5e pop rsi
0x1412bc431 5d pop rbp
0x1412bc432 c3 ret 
0x1412bc433 666666662e0f1f840000000000 nop word ptr cs:[rax + rax]
0x1412bc440 4889542410 mov qword ptr [rsp + 0x10], rdx
0x1412bc445 55 push rbp
0x1412bc446 56 push rsi
0x1412bc447 4883ec28 sub rsp, 0x28
0x1412bc44b 488daa80000000 lea rbp, [rdx + 0x80]
0x1412bc452 488d4d10 lea rcx, [rbp + 0x10]
0x1412bc456 e85540e0fe call 0x1400c04b0
0x1412bc45b 90 nop 
0x1412bc45c 4883c428 add rsp, 0x28
0x1412bc460 5e pop rsi
0x1412bc461 5d pop rbp
0x1412bc462 c3 ret 
