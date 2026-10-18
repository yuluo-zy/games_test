1412e5170: push rbp
1412e5171: push r14
1412e5173: push rsi
1412e5174: push rdi
1412e5175: push rbx
1412e5176: sub rsp, 0xb0
1412e517d: lea rbp, [rsp + 0x80]
1412e5185: movdqa xmmword ptr [rbp + 0x20], xmm7
1412e518a: movdqa xmmword ptr [rbp + 0x10], xmm6
1412e518f: mov qword ptr [rbp + 8], 0xfffffffffffffffe
1412e5197: test r8, r8
1412e519a: jne 0x1412e54a6
1412e51a0: mov rdi, rdx
1412e51a3: mov rsi, rcx
1412e51a6: movzx eax, byte ptr [rip + 0x234fa5c]
1412e51ad: mov ecx, 0x1001
1412e51b2: mov edx, 1
1412e51b7: call 0x140613c10
1412e51bc: test rax, rax
1412e51bf: je 0x1412e54be
1412e51c5: mov rbx, rax
1412e51c8: mov r8d, 0x1000
1412e51ce: mov rcx, rax
1412e51d1: xor edx, edx
1412e51d3: call 0x142923c20
1412e51d8: mov byte ptr [rbx + 0x1000], 0x20
1412e51df: movzx eax, byte ptr [rip + 0x234fa23]
1412e51e6: mov ecx, 0x800000
1412e51eb: mov edx, 4
1412e51f0: call 0x140613c10
1412e51f5: test rax, rax
1412e51f8: mov qword ptr [rbp - 0x20], rbx
1412e51fc: je 0x1412e54d4
1412e5202: mov r14, rax
1412e5205: mov eax, 0xf0
1412e520a: movdqu xmm0, xmmword ptr [rip + 0x17afbfe]
1412e5212: movdqu xmm1, xmmword ptr [rip + 0x17afbe6]
1412e521a: movdqu xmm2, xmmword ptr [rip + 0x17afbce]
1412e5222: movdqa xmm3, xmmword ptr [rip + 0x17af6b6]
1412e522a: nop word ptr [rax + rax]
1412e5230: movdqu xmmword ptr [r14 + rax - 0xd0], xmm0
1412e523a: movdqu xmmword ptr [r14 + rax - 0xe0], xmm1
1412e5244: movdqu xmmword ptr [r14 + rax - 0xf0], xmm2
1412e524e: movdqu xmmword ptr [r14 + rax - 0xc0], xmm3
1412e5258: movdqu xmmword ptr [r14 + rax - 0xb0], xmm2
1412e5262: movdqu xmmword ptr [r14 + rax - 0xa0], xmm1
1412e526c: movdqu xmmword ptr [r14 + rax - 0x90], xmm0
1412e5276: movdqu xmmword ptr [r14 + rax - 0x80], xmm3
1412e527d: movdqu xmmword ptr [r14 + rax - 0x70], xmm2
1412e5284: movdqu xmmword ptr [r14 + rax - 0x60], xmm1
1412e528b: movdqu xmmword ptr [r14 + rax - 0x50], xmm0
1412e5292: movdqu xmmword ptr [r14 + rax - 0x40], xmm3
1412e5299: cmp rax, 0x7ffff0
1412e529f: je 0x1412e52c8
1412e52a1: lea rcx, [r14 + rax]
1412e52a5: add rcx, -0x30
1412e52a9: movdqu xmmword ptr [rcx + 0x20], xmm0
1412e52ae: movdqu xmmword ptr [rcx + 0x10], xmm1
1412e52b3: movdqu xmmword ptr [rcx], xmm2
1412e52b7: movdqu xmmword ptr [r14 + rax], xmm3
1412e52bd: add rax, 0x100
1412e52c3: jmp 0x1412e5230
1412e52c8: movdqu xmmword ptr [r14 + rax - 0x10], xmm0
1412e52cf: movdqu xmmword ptr [r14 + rax - 0x20], xmm1
1412e52d6: movdqu xmmword ptr [r14 + rax - 0x30], xmm2
1412e52dd: movdqu xmmword ptr [r14 + rax], xmm3
1412e52e3: movzx eax, byte ptr [rip + 0x234f91f]
1412e52ea: mov ecx, 0x4000
1412e52ef: mov edx, 4
1412e52f4: call 0x140613c10
1412e52f9: test rax, rax
1412e52fc: mov qword ptr [rbp - 0x18], r14
1412e5300: je 0x1412e54f0
1412e5306: movdqa xmm0, xmmword ptr [rip + 0x17ef652]
1412e530e: mov ecx, 0xc
1412e5313: movdqa xmm1, xmmword ptr [rip + 0x19493c5]
1412e531b: movdqa xmm2, xmmword ptr [rip + 0x19493cd]
1412e5323: movdqa xmm3, xmmword ptr [rip + 0x19493d5]
1412e532b: movdqa xmm4, xmmword ptr [rip + 0x19493dd]
1412e5333: movdqa xmm5, xmmword ptr [rip + 0x1949395]
1412e533b: nop dword ptr [rax + rax]
1412e5340: movdqa xmm6, xmm1
1412e5344: psubd xmm6, xmm0
1412e5348: movdqa xmm7, xmm2
1412e534c: psubd xmm7, xmm0
1412e5350: movdqu xmmword ptr [rax + rcx*4 - 0x30], xmm6
1412e5356: movdqu xmmword ptr [rax + rcx*4 - 0x20], xmm7
1412e535c: movdqa xmm6, xmm3
1412e5360: psubd xmm6, xmm0
1412e5364: movdqa xmm7, xmm4
1412e5368: psubd xmm7, xmm0
1412e536c: movdqu xmmword ptr [rax + rcx*4 - 0x10], xmm6
1412e5372: movdqu xmmword ptr [rax + rcx*4], xmm7
1412e5377: paddd xmm0, xmm5
1412e537b: add rcx, 0x10
1412e537f: cmp rcx, 0x100c
1412e5386: jne 0x1412e5340
1412e5388: mov qword ptr [rbp - 0x10], rax
1412e538c: mov byte ptr [rbp + 4], 1
1412e5390: mov byte ptr [rbp + 3], 1
1412e5394: lea rcx, [rbp - 0x60]
1412e5398: mov rdx, rdi
1412e539b: xor r8d, r8d
1412e539e: call 0x141619c80
1412e53a3: mov byte ptr [rbp + 2], 1
1412e53a7: mov edx, 0x40
1412e53ac: xor ecx, ecx
1412e53ae: call 0x1416969a0
1412e53b3: mov qword ptr [rbp - 0x30], rax
1412e53b7: mov qword ptr [rbp - 8], rdx
1412e53bb: mov edx, 0x40
1412e53c0: xor ecx, ecx
1412e53c2: call 0x1416969a0
1412e53c7: mov rbx, rax
1412e53ca: mov rdi, rdx
1412e53cd: movzx eax, byte ptr [rip + 0x234f835]
1412e53d4: mov ecx, 0x30
1412e53d9: mov edx, 8
1412e53de: call 0x140613c10
1412e53e3: test rax, rax
1412e53e6: je 0x1412e5510
1412e53ec: mov qword ptr [rax], 1
1412e53f3: mov qword ptr [rax + 8], 1
1412e53fb: mov byte ptr [rax + 0x10], 0
1412e53ff: mov qword ptr [rax + 0x18], 0
1412e5407: mov qword ptr [rax + 0x20], 8
1412e540f: mov qword ptr [rax + 0x28], 0
1412e5417: mov rcx, qword ptr [rbp - 0x40]
1412e541b: mov qword ptr [rsi + 0x88], rcx
1412e5422: movups xmm0, xmmword ptr [rbp - 0x60]
1412e5426: movups xmm1, xmmword ptr [rbp - 0x50]
1412e542a: movups xmmword ptr [rsi + 0x78], xmm1
1412e542e: movups xmmword ptr [rsi + 0x68], xmm0
1412e5432: mov rcx, qword ptr [rbp - 0x20]
1412e5436: mov qword ptr [rsi + 0x20], rcx
1412e543a: mov qword ptr [rsi + 0x28], 0x1001
1412e5442: mov rcx, qword ptr [rbp - 0x30]
1412e5446: mov qword ptr [rsi + 0x30], rcx
1412e544a: mov rcx, qword ptr [rbp - 8]
1412e544e: mov qword ptr [rsi + 0x38], rcx
1412e5452: mov qword ptr [rsi], 0
1412e5459: mov qword ptr [rsi + 8], 0x1000
1412e5461: mov rcx, qword ptr [rbp - 0x10]
1412e5465: mov qword ptr [rsi + 0x10], rcx
1412e5469: mov qword ptr [rsi + 0x18], 0x1000
1412e5471: mov rcx, qword ptr [rbp - 0x18]
1412e5475: mov qword ptr [rsi + 0x40], rcx
1412e5479: mov qword ptr [rsi + 0x48], 0x20000
1412e5481: mov qword ptr [rsi + 0x50], rbx
1412e5485: mov qword ptr [rsi + 0x58], rdi
1412e5489: mov qword ptr [rsi + 0x60], rax
1412e548d: mov rax, rsi
1412e5490: movaps xmm6, xmmword ptr [rbp + 0x10]
1412e5494: movaps xmm7, xmmword ptr [rbp + 0x20]
1412e5498: add rsp, 0xb0
1412e549f: pop rbx
1412e54a0: pop rdi
1412e54a1: pop rsi
1412e54a2: pop r14
1412e54a4: pop rbp
1412e54a5: ret 
1412e54a6: lea rcx, [rip + 0x194988b]
1412e54ad: lea r8, [rip + 0x19498a4]
1412e54b4: mov edx, 0x20
1412e54b9: call 0x1428d9430
1412e54be: lea r8, [rip + 0x19498ab]
1412e54c5: mov ecx, 1
1412e54ca: mov edx, 0x1001
1412e54cf: call 0x1428d8fe3
1412e54d4: mov byte ptr [rbp + 7], 1
1412e54d8: lea r8, [rip + 0x19498a9]
1412e54df: mov ecx, 4
1412e54e4: mov edx, 0x800000
1412e54e9: call 0x1428d8fe3
1412e54ee: jmp 0x1412e5527
1412e54f0: mov byte ptr [rbp + 6], 1
1412e54f4: mov byte ptr [rbp + 5], 1
1412e54f8: lea r8, [rip + 0x1949499]
1412e54ff: mov ecx, 4
1412e5504: mov edx, 0x4000
1412e5509: call 0x1428d8fe3
1412e550e: jmp 0x1412e5527
1412e5510: mov qword ptr [rbp - 0x38], rbx
1412e5514: mov qword ptr [rbp - 0x28], rdi
1412e5518: mov ecx, 8
1412e551d: mov edx, 0x30
1412e5522: call 0x1428d9000
1412e5527: ud2 
1412e5529: nop dword ptr [rax]
1412e5530: mov qword ptr [rsp + 0x10], rdx
1412e5535: push rbp
1412e5536: push r14
1412e5538: push rsi
1412e5539: push rdi
1412e553a: push rbx
1412e553b: sub rsp, 0x40
1412e553f: lea rbp, [rdx + 0x80]
1412e5546: movdqa xmmword ptr [rsp + 0x20], xmm7
1412e554c: movdqa xmmword ptr [rsp + 0x30], xmm6
1412e5552: movzx ebx, byte ptr [rbp + 2]
1412e5556: mov edx, 0x1001
1412e555b: mov r8d, 1
1412e5561: mov rcx, qword ptr [rbp - 0x20]
1412e5565: call 0x140613c20
1412e556a: lea rcx, [rbp - 0x60]
1412e556e: call 0x140be24b0
1412e5573: mov byte ptr [rbp + 4], 0
1412e5577: mov byte ptr [rbp + 3], bl
1412e557a: movdqa xmm6, xmmword ptr [rsp + 0x30]
1412e5580: movdqa xmm7, xmmword ptr [rsp + 0x20]
1412e5586: add rsp, 0x40
1412e558a: pop rbx
1412e558b: pop rdi
1412e558c: pop rsi
1412e558d: pop r14
1412e558f: pop rbp
1412e5590: ret 
1412e5591: nop word ptr cs:[rax + rax]
1412e55a0: mov qword ptr [rsp + 0x10], rdx
1412e55a5: push rbp
1412e55a6: push r14
1412e55a8: push rsi
1412e55a9: push rdi
1412e55aa: push rbx
1412e55ab: sub rsp, 0x40
1412e55af: lea rbp, [rdx + 0x80]
1412e55b6: movdqa xmmword ptr [rsp + 0x20], xmm7
1412e55bc: movdqa xmmword ptr [rsp + 0x30], xmm6
1412e55c2: mov edx, 0x4000
1412e55c7: mov r8d, 4
1412e55cd: mov rcx, qword ptr [rbp - 0x10]
1412e55d1: call 0x140613c20
1412e55d6: mov edx, 0x800000
1412e55db: mov r8d, 4
1412e55e1: mov rcx, qword ptr [rbp - 0x18]
1412e55e5: call 0x140613c20
1412e55ea: cmp qword ptr [rbp - 8], 0
1412e55ef: je 0x1412e5608
1412e55f1: mov rdx, qword ptr [rbp - 8]
1412e55f5: shl rdx, 3
1412e55f9: mov r8d, 8
1412e55ff: mov rcx, qword ptr [rbp - 0x30]
1412e5603: call 0x140613c20
1412e5608: mov byte ptr [rbp + 2], 0
1412e560c: movdqa xmm6, xmmword ptr [rsp + 0x30]
1412e5612: movdqa xmm7, xmmword ptr [rsp + 0x20]
1412e5618: add rsp, 0x40
1412e561c: pop rbx
1412e561d: pop rdi
1412e561e: pop rsi
1412e561f: pop r14
1412e5621: pop rbp
1412e5622: ret 
1412e5623: nop word ptr cs:[rax + rax]
1412e5630: mov qword ptr [rsp + 0x10], rdx
1412e5635: push rbp
1412e5636: push r14
1412e5638: push rsi
1412e5639: push rdi
1412e563a: push rbx
1412e563b: sub rsp, 0x40
1412e563f: lea rbp, [rdx + 0x80]
1412e5646: movdqa xmmword ptr [rsp + 0x20], xmm7
1412e564c: movdqa xmmword ptr [rsp + 0x30], xmm6
1412e5652: cmp qword ptr [rbp - 0x28], 0
1412e5657: je 0x1412e5670
1412e5659: mov rdx, qword ptr [rbp - 0x28]
1412e565d: shl rdx, 3
1412e5661: mov r8d, 8
1412e5667: mov rcx, qword ptr [rbp - 0x38]
1412e566b: call 0x140613c20
1412e5670: movdqa xmm6, xmmword ptr [rsp + 0x30]
1412e5676: movdqa xmm7, xmmword ptr [rsp + 0x20]
1412e567c: add rsp, 0x40
1412e5680: pop rbx
1412e5681: pop rdi
1412e5682: pop rsi
1412e5683: pop r14
1412e5685: pop rbp
1412e5686: ret 
1412e5687: nop word ptr [rax + rax]
1412e5690: mov qword ptr [rsp + 0x10], rdx
1412e5695: push rbp
1412e5696: push r14
1412e5698: push rsi
1412e5699: push rdi
1412e569a: push rbx
1412e569b: sub rsp, 0x40
1412e569f: lea rbp, [rdx + 0x80]
1412e56a6: movdqa xmmword ptr [rsp + 0x20], xmm7
1412e56ac: movdqa xmmword ptr [rsp + 0x30], xmm6
1412e56b2: movzx ebx, byte ptr [rbp + 4]
1412e56b6: cmp byte ptr [rbp + 3], 0
1412e56ba: je 0x1412e56f1
1412e56bc: mov edx, 0x4000
1412e56c1: mov r8d, 4
1412e56c7: mov rcx, qword ptr [rbp - 0x10]
1412e56cb: call 0x140613c20
1412e56d0: and bl, 1
1412e56d3: mov byte ptr [rbp + 6], bl
1412e56d6: mov byte ptr [rbp + 5], 1
1412e56da: movdqa xmm6, xmmword ptr [rsp + 0x30]
1412e56e0: movdqa xmm7, xmmword ptr [rsp + 0x20]
1412e56e6: add rsp, 0x40
1412e56ea: pop rbx
1412e56eb: pop rdi
1412e56ec: pop rsi
1412e56ed: pop r14
1412e56ef: pop rbp
1412e56f0: ret 
1412e56f1: and bl, 1
1412e56f4: mov byte ptr [rbp + 6], bl
1412e56f7: mov byte ptr [rbp + 5], 0
1412e56fb: movdqa xmm6, xmmword ptr [rsp + 0x30]
1412e5701: movdqa xmm7, xmmword ptr [rsp + 0x20]
1412e5707: add rsp, 0x40
1412e570b: pop rbx
1412e570c: pop rdi
1412e570d: pop rsi
1412e570e: pop r14
1412e5710: pop rbp
1412e5711: ret 
1412e5712: nop word ptr cs:[rax + rax]
1412e5720: mov qword ptr [rsp + 0x10], rdx
1412e5725: push rbp
1412e5726: push r14
1412e5728: push rsi
1412e5729: push rdi
1412e572a: push rbx
1412e572b: sub rsp, 0x40
1412e572f: lea rbp, [rdx + 0x80]
1412e5736: movdqa xmmword ptr [rsp + 0x20], xmm7
1412e573c: movdqa xmmword ptr [rsp + 0x30], xmm6
1412e5742: movzx ebx, byte ptr [rbp + 6]
1412e5746: cmp byte ptr [rbp + 5], 0
1412e574a: je 0x1412e577d
1412e574c: mov edx, 0x800000
1412e5751: mov r8d, 4
1412e5757: mov rcx, qword ptr [rbp - 0x18]
1412e575b: call 0x140613c20
1412e5760: and bl, 1
1412e5763: mov byte ptr [rbp + 7], bl
1412e5766: movdqa xmm6, xmmword ptr [rsp + 0x30]
1412e576c: movdqa xmm7, xmmword ptr [rsp + 0x20]
1412e5772: add rsp, 0x40
1412e5776: pop rbx
1412e5777: pop rdi
1412e5778: pop rsi
1412e5779: pop r14
1412e577b: pop rbp
1412e577c: ret 
1412e577d: and bl, 1
1412e5780: mov byte ptr [rbp + 7], bl
1412e5783: movdqa xmm6, xmmword ptr [rsp + 0x30]
1412e5789: movdqa xmm7, xmmword ptr [rsp + 0x20]
1412e578f: add rsp, 0x40
1412e5793: pop rbx
1412e5794: pop rdi
1412e5795: pop rsi
1412e5796: pop r14
1412e5798: pop rbp
1412e5799: ret 
1412e579a: nop word ptr [rax + rax]
1412e57a0: mov qword ptr [rsp + 0x10], rdx
1412e57a5: push rbp
1412e57a6: push r14
1412e57a8: push rsi
1412e57a9: push rdi
1412e57aa: push rbx
1412e57ab: sub rsp, 0x40
1412e57af: lea rbp, [rdx + 0x80]
1412e57b6: movdqa xmmword ptr [rsp + 0x20], xmm7
1412e57bc: movdqa xmmword ptr [rsp + 0x30], xmm6
1412e57c2: cmp byte ptr [rbp + 7], 0
1412e57c6: je 0x1412e57dc
1412e57c8: mov edx, 0x1001
1412e57cd: mov r8d, 1
1412e57d3: mov rcx, qword ptr [rbp - 0x20]
1412e57d7: call 0x140613c20
1412e57dc: movaps xmm6, xmmword ptr [rsp + 0x30]
1412e57e1: movaps xmm7, xmmword ptr [rsp + 0x20]
1412e57e6: add rsp, 0x40
1412e57ea: pop rbx
1412e57eb: pop rdi
1412e57ec: pop rsi
1412e57ed: pop r14
1412e57ef: pop rbp
1412e57f0: ret 
