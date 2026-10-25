1412fff70: push rbp
1412fff71: push r15
1412fff73: push r14
1412fff75: push r13
1412fff77: push r12
1412fff79: push rsi
1412fff7a: push rdi
1412fff7b: push rbx
1412fff7c: sub rsp, 0x118
1412fff83: lea rbp, [rsp + 0x80]
1412fff8b: mov qword ptr [rbp + 0x90], 0xfffffffffffffffe
1412fff96: mov rax, qword ptr [rdx + 0x78]
1412fff9a: test rax, rax
1412fff9d: je 0x14130058c
1412fffa3: dec rax
1412fffa6: mov qword ptr [rdx + 0x78], rax
1412fffaa: lea rdi, [rdx + 0x18]
1412fffae: mov r9, qword ptr [rdx]
1412fffb1: test r9b, 1
1412fffb5: je 0x141300487
1412fffbb: mov r10, qword ptr [rdx + 8]
1412fffbf: mov r11, qword ptr [rdx + 0x10]
1412fffc3: cmp r11, r10
1412fffc6: je 0x14130047d
1412fffcc: lea r11, [r10 + 1]
1412fffd0: mov qword ptr [rdx + 8], r11
1412fffd4: lea r12, [rdi + r10*4]
1412fffd8: test rax, rax
1412fffdb: mov qword ptr [rbp + 0x38], rcx
1412fffdf: je 0x14130007c
1412fffe5: mov r10, qword ptr [rdx + 0x10]
1412fffe9: mov r11, qword ptr [rdx + 0x40]
1412fffed: sub r10, qword ptr [rdx + 8]
1412ffff1: xor esi, esi
1412ffff3: test r9b, 1
1412ffff7: cmove r10, rsi
1412ffffb: sub r11, qword ptr [rdx + 0x38]
1412fffff: cmp byte ptr [rdx + 0x30], 0
141300003: cmove r11, rsi
141300007: mov r9d, dword ptr [rdx + 0x60]
14130000b: cmp r9d, 2
14130000f: je 0x141300041
141300011: cmp byte ptr [rdx + 0x74], 0
141300015: jne 0x14130001f
141300017: mov esi, dword ptr [rdx + 0x70]
14130001a: sub esi, dword ptr [rdx + 0x6c]
14130001d: jae 0x141300023
14130001f: xor esi, esi
141300021: jmp 0x141300026
141300023: inc rsi
141300026: xor edi, edi
141300028: mov rbx, rsi
14130002b: sub rbx, 1
14130002f: cmovae rdi, rbx
141300033: test r9d, r9d
141300036: cmovne rdi, rsi
14130003a: add rdi, rdi
14130003d: lea rsi, [rdi + rdi*2]
141300041: add r10, r11
141300044: mov r9, 0xffffffffffffffff
14130004b: cmovb r10, r9
14130004f: add r10, rsi
141300052: cmovb r10, r9
141300056: cmp rax, r10
141300059: cmovb r10, rax
14130005d: jmp 0x14130007f
14130005f: lea r10, [r11 + 1]
141300063: mov qword ptr [rdx + 0x38], r10
141300067: lea r12, [rdx + r11*4]
14130006b: add r12, 0x48
14130006f: test rax, rax
141300072: mov qword ptr [rbp + 0x38], rcx
141300076: jne 0x1412fffe5
14130007c: xor r10d, r10d
14130007f: mov rdi, r8
141300082: inc r10
141300085: mov rax, 0xffffffffffffffff
14130008c: cmovne rax, r10
141300090: cmp rax, 5
141300094: mov esi, 4
141300099: cmovae rsi, rax
14130009d: lea rbx, [rsi*4]
1413000a5: xor r14d, r14d
1413000a8: shr rax, 0x3e
1413000ac: jne 0x14130046f
1413000b2: movabs rax, 0x7ffffffffffffffc
1413000bc: cmp rbx, rax
1413000bf: ja 0x14130046f
1413000c5: mov r15, rdx
1413000c8: mov r12d, dword ptr [r12]
1413000cc: movzx eax, byte ptr [rip + 0x2334b36]
1413000d3: mov r14d, 4
1413000d9: mov edx, 4
1413000de: mov rcx, rbx
1413000e1: call 0x140613c10
1413000e6: test rax, rax
1413000e9: je 0x14130046f
1413000ef: mov dword ptr [rax], r12d
1413000f2: mov qword ptr [rbp + 0x50], rsi
1413000f6: mov qword ptr [rbp + 0x58], rax
1413000fa: mov qword ptr [rbp + 0x60], 1
141300102: movups xmm0, xmmword ptr [r15 + 0x70]
141300107: movaps xmmword ptr [rbp + 0x20], xmm0
14130010b: movups xmm0, xmmword ptr [r15 + 0x60]
141300110: movaps xmmword ptr [rbp + 0x10], xmm0
141300114: movups xmm0, xmmword ptr [r15 + 0x50]
141300119: movaps xmmword ptr [rbp], xmm0
14130011d: movups xmm0, xmmword ptr [r15 + 0x40]
141300122: movaps xmmword ptr [rbp - 0x10], xmm0
141300126: movups xmm0, xmmword ptr [r15]
14130012a: movups xmm1, xmmword ptr [r15 + 0x10]
14130012f: movups xmm2, xmmword ptr [r15 + 0x20]
141300134: movups xmm3, xmmword ptr [r15 + 0x30]
141300139: movaps xmmword ptr [rbp - 0x20], xmm3
14130013d: movaps xmmword ptr [rbp - 0x30], xmm2
141300141: movaps xmmword ptr [rbp - 0x40], xmm1
141300145: movaps xmmword ptr [rbp - 0x50], xmm0
141300149: mov r14, qword ptr [rbp + 0x28]
14130014d: test r14, r14
141300150: je 0x141300457
141300156: mov r15, qword ptr [rbp - 0x18]
14130015a: mov r12d, dword ptr [rbp + 0x10]
14130015e: mov r13, qword ptr [rbp - 0x50]
141300162: mov rsi, qword ptr [rbp - 0x48]
141300166: mov rcx, qword ptr [rbp - 0x40]
14130016a: mov qword ptr [rbp + 0x70], rcx
14130016e: mov ecx, dword ptr [rbp + 0x20]
141300171: mov dword ptr [rbp + 0x84], ecx
141300177: movzx ecx, byte ptr [rbp - 0x20]
14130017b: mov byte ptr [rbp + 0x8f], cl
141300181: mov rcx, qword ptr [rbp - 0x10]
141300185: mov qword ptr [rbp + 0x48], rcx
141300189: movzx ecx, byte ptr [rbp + 0x24]
14130018d: mov dword ptr [rbp + 0x88], ecx
141300193: mov ecx, dword ptr [rbp + 0x1c]
141300196: mov qword ptr [rbp + 0x78], rcx
14130019a: mov ecx, dword ptr [rbp + 0x18]
14130019d: mov qword ptr [rbp + 0x68], rcx
1413001a1: mov rcx, r14
1413001a4: neg rcx
1413001a7: mov qword ptr [rbp + 0x40], rcx
1413001ab: dec r14
1413001ae: mov ebx, 1
1413001b3: jmp 0x141300200
1413001b5: nop word ptr cs:[rax + rax]
1413001c0: mov qword ptr [rsp + 0x20], 4
1413001c9: mov r9d, 4
1413001cf: lea rcx, [rbp + 0x50]
1413001d3: mov rdx, rbx
1413001d6: call 0x1428a9a60 ; alloc::raw_vec::impl$4::reserve::do_reserve_and_handle<alloc::alloc::Global>
1413001db: mov rax, qword ptr [rbp + 0x58]
1413001df: mov dword ptr [rax + rbx*4], edi
1413001e2: mov rcx, qword ptr [rbp + 0x40]
1413001e6: add rcx, rbx
1413001e9: inc rcx
1413001ec: inc rbx
1413001ef: mov qword ptr [rbp + 0x60], rbx
1413001f3: dec r14
1413001f6: cmp rcx, 1
1413001fa: je 0x141300457
141300200: mov ecx, r12d
141300203: test r13b, 1
141300207: je 0x14130021d
141300209: cmp qword ptr [rbp + 0x70], rsi
14130020d: jne 0x14130033f
141300213: mov ecx, r12d
141300216: mov qword ptr [rbp + 0x70], rsi
14130021a: xor r13d, r13d
14130021d: cmp ecx, 2
141300220: je 0x141300260
141300222: test byte ptr [rbp + 0x88], 1
141300229: jne 0x141300260
14130022b: mov r11, qword ptr [rbp + 0x78]
14130022f: cmp r11d, dword ptr [rbp + 0x84]
141300236: ja 0x141300260
141300238: jae 0x1413002a0
14130023a: lea r8d, [r11 + 1]
14130023e: test cl, 1
141300241: mov r10, qword ptr [rbp + 0x68]
141300245: je 0x1413002bd
141300247: mov edx, r8d
14130024a: mov r8d, r11d
14130024d: jmp 0x1413002ea
141300252: nop word ptr cs:[rax + rax]
141300260: mov r12d, ecx
141300263: cmp qword ptr [rbp + 0x48], r15
141300267: setne cl
14130026a: and cl, byte ptr [rbp + 0x8f]
141300270: cmp cl, 1
141300273: jne 0x141300457
141300279: lea rcx, [rbp - 0x38]
14130027d: lea rcx, [rcx + r15*4]
141300281: add rcx, 0x30
141300285: inc r15
141300288: mov edi, dword ptr [rcx]
14130028a: cmp rbx, qword ptr [rbp + 0x50]
14130028e: jne 0x1413001df
141300294: jmp 0x141300356
141300299: nop dword ptr [rax]
1413002a0: xor r12d, r12d
1413002a3: mov dl, 1
1413002a5: mov dword ptr [rbp + 0x88], edx
1413002ab: test cl, 1
1413002ae: je 0x141300263
1413002b0: mov r8, qword ptr [rbp + 0x78]
1413002b4: mov edx, r8d
1413002b7: mov r10, qword ptr [rbp + 0x68]
1413002bb: jmp 0x1413002ea
1413002bd: lea edx, [r11 + 2]
1413002c1: cmp r8d, dword ptr [rbp + 0x84]
1413002c8: cmovae edx, r8d
1413002cc: mov r9d, dword ptr [rbp + 0x88]
1413002d3: movzx r9d, r9b
1413002d7: mov ecx, 1
1413002dc: cmovae r9d, ecx
1413002e0: mov dword ptr [rbp + 0x88], r9d
1413002e7: mov r10d, r11d
1413002ea: lea r9d, [r10*2 + 1]
1413002f2: add r10d, r10d
1413002f5: lea r11d, [r8 + r8]
1413002f9: lea esi, [r8*2 + 1]
141300301: mov dword ptr [rbp - 0x38], r10d
141300305: mov dword ptr [rbp - 0x34], r9d
141300309: mov dword ptr [rbp - 0x30], r11d
14130030d: mov dword ptr [rbp - 0x2c], r11d
141300311: mov dword ptr [rbp - 0x28], r9d
141300315: mov dword ptr [rbp - 0x24], esi
141300318: mov r13d, 1
14130031e: mov r9d, 6
141300324: mov qword ptr [rbp + 0x70], r9
141300328: mov qword ptr [rbp + 0x68], r8
14130032c: mov qword ptr [rbp + 0x78], rdx
141300330: mov r12d, ecx
141300333: xor esi, esi
141300335: cmp qword ptr [rbp + 0x70], rsi
141300339: je 0x141300213
14130033f: lea rcx, [rbp - 0x38]
141300343: lea rcx, [rcx + rsi*4]
141300347: inc rsi
14130034a: mov edi, dword ptr [rcx]
14130034c: cmp rbx, qword ptr [rbp + 0x50]
141300350: jne 0x1413001df
141300356: mov rax, qword ptr [rbp + 0x40]
14130035a: add rax, rbx
14130035d: je 0x14130043d
141300363: mov rax, qword ptr [rbp + 0x70]
141300367: sub rax, rsi
14130036a: test r13b, 1
14130036e: mov edx, 0
141300373: cmove rax, rdx
141300377: mov rcx, qword ptr [rbp + 0x48]
14130037b: sub rcx, r15
14130037e: test byte ptr [rbp + 0x8f], 1
141300385: cmove rcx, rdx
141300389: mov edx, 0
14130038e: cmp r12d, 2
141300392: je 0x1413003d1
141300394: mov edx, dword ptr [rbp + 0x84]
14130039a: mov r8d, edx
14130039d: sub r8d, dword ptr [rbp + 0x78]
1413003a1: setb dl
1413003a4: inc r8
1413003a7: test dl, 1
1413003aa: mov r9d, 0
1413003b0: cmovne r8, r9
1413003b4: test byte ptr [rbp + 0x88], 1
1413003bb: cmovne r8, r9
1413003bf: mov rdx, r8
1413003c2: sub rdx, 1
1413003c6: cmovb rdx, r9
1413003ca: test r12d, r12d
1413003cd: cmovne rdx, r8
1413003d1: add rdx, rdx
1413003d4: lea rdx, [rdx + rdx*2]
1413003d8: add rax, rcx
1413003db: mov rcx, 0xffffffffffffffff
1413003e2: cmovb rax, rcx
1413003e6: add rax, rdx
1413003e9: jb 0x141300420
1413003eb: mov rcx, rax
1413003ee: mov rax, r14
1413003f1: cmp r14, rcx
1413003f4: jae 0x141300428
1413003f6: nop word ptr cs:[rax + rax]
141300400: mov r8, 0xffffffffffffffff
141300407: inc rax
14130040a: je 0x1413001c0
141300410: jmp 0x14130044f
141300412: nop word ptr cs:[rax + rax]
141300420: mov rax, r14
141300423: cmp r14, rcx
141300426: jb 0x141300400
141300428: mov rax, rcx
14130042b: mov r8, 0xffffffffffffffff
141300432: inc rax
141300435: je 0x1413001c0
14130043b: jmp 0x14130044f
14130043d: xor eax, eax
14130043f: mov r8, 0xffffffffffffffff
141300446: inc rax
141300449: je 0x1413001c0
14130044f: mov r8, rax
141300452: jmp 0x1413001c0
141300457: mov rax, qword ptr [rbp + 0x60]
14130045b: mov rcx, qword ptr [rbp + 0x38]
14130045f: mov qword ptr [rcx + 0x10], rax
141300463: movups xmm0, xmmword ptr [rbp + 0x50]
141300467: movups xmmword ptr [rcx], xmm0
14130046a: jmp 0x1413005a3
14130046f: mov rcx, r14
141300472: mov rdx, rbx
141300475: mov r8, rdi
141300478: call 0x1428d8fe3 ; alloc::raw_vec::handle_error
14130047d: mov qword ptr [rdx], 0
141300484: xor r9d, r9d
141300487: mov r11d, dword ptr [rdx + 0x60]
14130048b: cmp r11d, 2
14130048f: je 0x141300570
141300495: cmp byte ptr [rdx + 0x74], 0
141300499: jne 0x141300570
14130049f: mov r10d, dword ptr [rdx + 0x6c]
1413004a3: mov esi, dword ptr [rdx + 0x70]
1413004a6: cmp r10d, esi
1413004a9: ja 0x141300570
1413004af: jae 0x1413004ce
1413004b1: lea r9d, [r10 + 1]
1413004b5: mov dword ptr [rdx + 0x6c], r9d
1413004b9: test r11b, 1
1413004bd: jne 0x1413004dc
1413004bf: cmp r9d, esi
1413004c2: jae 0x1413004ea
1413004c4: lea r11d, [r10 + 2]
1413004c8: mov dword ptr [rdx + 0x6c], r11d
1413004cc: jmp 0x1413004ee
1413004ce: mov byte ptr [rdx + 0x74], 1
1413004d2: test r11b, 1
1413004d6: je 0x141300569
1413004dc: mov r11d, dword ptr [rdx + 0x68]
1413004e0: mov dword ptr [rdx + 0x68], r10d
1413004e4: mov dword ptr [rdx + 0x64], r11d
1413004e8: jmp 0x141300503
1413004ea: mov byte ptr [rdx + 0x74], 1
1413004ee: mov dword ptr [rdx + 0x60], 1
1413004f5: mov dword ptr [rdx + 0x64], r10d
1413004f9: mov dword ptr [rdx + 0x68], r9d
1413004fd: mov r11d, r10d
141300500: mov r10d, r9d
141300503: mov r9d, r11d
141300506: add r9d, r11d
141300509: lea r11d, [r11*2 + 1]
141300511: mov esi, r10d
141300514: add esi, r10d
141300517: lea r10d, [r10*2 + 1]
14130051f: mov qword ptr [rdx], 1
141300526: mov qword ptr [rdx + 8], 0
14130052e: mov qword ptr [rdx + 0x10], 6
141300536: mov dword ptr [rdx + 0x18], r9d
14130053a: mov dword ptr [rdx + 0x1c], r11d
14130053e: mov dword ptr [rdx + 0x20], esi
141300541: mov dword ptr [rdx + 0x24], esi
141300544: mov dword ptr [rdx + 0x28], r11d
141300548: mov dword ptr [rdx + 0x2c], r10d
14130054c: mov r11d, 6
141300552: mov r9d, 1
141300558: xor r10d, r10d
14130055b: cmp r11, r10
14130055e: je 0x14130047d
141300564: jmp 0x1412fffcc
141300569: mov dword ptr [rdx + 0x60], 0
141300570: cmp dword ptr [rdx + 0x30], 1
141300574: jne 0x14130058c
141300576: mov r11, qword ptr [rdx + 0x38]
14130057a: cmp qword ptr [rdx + 0x40], r11
14130057e: jne 0x14130005f
141300584: mov qword ptr [rdx + 0x30], 0
14130058c: mov qword ptr [rcx], 0
141300593: mov qword ptr [rcx + 8], 4
14130059b: mov qword ptr [rcx + 0x10], 0
1413005a3: mov rax, rcx
1413005a6: add rsp, 0x118
1413005ad: pop rbx
1413005ae: pop rdi
1413005af: pop rsi
1413005b0: pop r12
1413005b2: pop r13
1413005b4: pop r14
1413005b6: pop r15
1413005b8: pop rbp
1413005b9: ret 
1413005ba: nop word ptr [rax + rax]
1413005c0: mov qword ptr [rsp + 0x10], rdx
1413005c5: push rbp
1413005c6: push r15
1413005c8: push r14
1413005ca: push r13
1413005cc: push r12
1413005ce: push rsi
1413005cf: push rdi
1413005d0: push rbx
1413005d1: sub rsp, 0x28
1413005d5: lea rbp, [rdx + 0x80]
1413005dc: mov rdx, qword ptr [rbp + 0x50]
1413005e0: test rdx, rdx
1413005e3: je 0x1413005f8
1413005e5: mov rcx, qword ptr [rbp + 0x58]
1413005e9: shl rdx, 2
1413005ed: mov r8d, 4
1413005f3: call 0x140613c20
1413005f8: nop 
1413005f9: add rsp, 0x28
1413005fd: pop rbx
1413005fe: pop rdi
1413005ff: pop rsi
141300600: pop r12
141300602: pop r13
141300604: pop r14
141300606: pop r15
141300608: pop rbp
141300609: ret 
