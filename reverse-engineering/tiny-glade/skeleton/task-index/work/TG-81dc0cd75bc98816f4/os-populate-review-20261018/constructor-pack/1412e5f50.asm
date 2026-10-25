1412e5f50: push rbp
1412e5f51: push rsi
1412e5f52: push rdi
1412e5f53: push rbx
1412e5f54: sub rsp, 0xd8
1412e5f5b: lea rbp, [rsp + 0x80]
1412e5f63: movdqa xmmword ptr [rbp + 0x40], xmm9
1412e5f69: movdqa xmmword ptr [rbp + 0x30], xmm8
1412e5f6f: movdqa xmmword ptr [rbp + 0x20], xmm7
1412e5f74: movdqa xmmword ptr [rbp + 0x10], xmm6
1412e5f79: mov qword ptr [rbp + 8], 0xfffffffffffffffe
1412e5f81: test r8, r8
1412e5f84: jne 0x1412e626c
1412e5f8a: mov rdi, rdx
1412e5f8d: mov rsi, rcx
1412e5f90: movzx eax, byte ptr [rip + 0x234ec72]
1412e5f97: mov ecx, 0x301
1412e5f9c: mov edx, 1
1412e5fa1: call 0x140613c10
1412e5fa6: test rax, rax
1412e5fa9: je 0x1412e6284
1412e5faf: mov rbx, rax
1412e5fb2: mov r8d, 0x300
1412e5fb8: mov rcx, rax
1412e5fbb: xor edx, edx
1412e5fbd: call 0x142923c20
1412e5fc2: mov byte ptr [rbx + 0x300], 0x20
1412e5fc9: movzx eax, byte ptr [rip + 0x234ec39]
1412e5fd0: mov ecx, 0xc0000
1412e5fd5: mov edx, 4
1412e5fda: call 0x140613c10
1412e5fdf: test rax, rax
1412e5fe2: mov qword ptr [rbp - 0x20], rbx
1412e5fe6: je 0x1412e629a
1412e5fec: xor ecx, ecx
1412e5fee: pxor xmm0, xmm0
1412e5ff2: nop word ptr cs:[rax + rax]
1412e6000: movdqu xmmword ptr [rax + rcx + 0x10], xmm0
1412e6006: movdqu xmmword ptr [rax + rcx], xmm0
1412e600b: movdqu xmmword ptr [rax + rcx + 0x20], xmm0
1412e6011: movdqu xmmword ptr [rax + rcx + 0x30], xmm0
1412e6017: movdqu xmmword ptr [rax + rcx + 0x40], xmm0
1412e601d: movdqu xmmword ptr [rax + rcx + 0x50], xmm0
1412e6023: movdqu xmmword ptr [rax + rcx + 0x60], xmm0
1412e6029: movdqu xmmword ptr [rax + rcx + 0x70], xmm0
1412e602f: movdqu xmmword ptr [rax + rcx + 0x80], xmm0
1412e6038: movdqu xmmword ptr [rax + rcx + 0x90], xmm0
1412e6041: add rcx, 0xa0
1412e6048: cmp rcx, 0xbffe0
1412e604f: jne 0x1412e6000
1412e6051: pxor xmm0, xmm0
1412e6055: movdqu xmmword ptr [rax + rcx + 0x10], xmm0
1412e605b: mov qword ptr [rbp - 0x18], rax
1412e605f: movdqu xmmword ptr [rax + rcx], xmm0
1412e6064: movzx eax, byte ptr [rip + 0x234eb9e]
1412e606b: mov ecx, 0xc00
1412e6070: mov edx, 4
1412e6075: call 0x140613c10
1412e607a: test rax, rax
1412e607d: je 0x1412e62b6
1412e6083: movdqa xmm0, xmmword ptr [rip + 0x17ee8d5]
1412e608b: mov ecx, 0x14
1412e6090: movdqa xmm1, xmmword ptr [rip + 0x19486b8]
1412e6098: movdqa xmm2, xmmword ptr [rip + 0x19486c0]
1412e60a0: movdqa xmm3, xmmword ptr [rip + 0x19486c8]
1412e60a8: movdqa xmm4, xmmword ptr [rip + 0x19486d0]
1412e60b0: movdqa xmm5, xmmword ptr [rip + 0x19486d8]
1412e60b8: movdqa xmm6, xmmword ptr [rip + 0x19486e0]
1412e60c0: movdqa xmm7, xmmword ptr [rip + 0x19486e8]
1412e60c8: nop dword ptr [rax + rax]
1412e60d0: movdqa xmm8, xmm1
1412e60d5: psubd xmm8, xmm0
1412e60da: movdqa xmm9, xmm2
1412e60df: psubd xmm9, xmm0
1412e60e4: movdqu xmmword ptr [rax + rcx*4 - 0x50], xmm8
1412e60eb: movdqu xmmword ptr [rax + rcx*4 - 0x40], xmm9
1412e60f2: movdqa xmm8, xmm3
1412e60f7: psubd xmm8, xmm0
1412e60fc: movdqa xmm9, xmm4
1412e6101: psubd xmm9, xmm0
1412e6106: movdqu xmmword ptr [rax + rcx*4 - 0x30], xmm8
1412e610d: movdqu xmmword ptr [rax + rcx*4 - 0x20], xmm9
1412e6114: movdqa xmm8, xmm5
1412e6119: psubd xmm8, xmm0
1412e611e: movdqa xmm9, xmm6
1412e6123: psubd xmm9, xmm0
1412e6128: movdqu xmmword ptr [rax + rcx*4 - 0x10], xmm8
1412e612f: movdqu xmmword ptr [rax + rcx*4], xmm9
1412e6135: paddd xmm0, xmm7
1412e6139: add rcx, 0x18
1412e613d: cmp rcx, 0x314
1412e6144: jne 0x1412e60d0
1412e6146: mov qword ptr [rbp - 0x10], rax
1412e614a: mov byte ptr [rbp + 4], 1
1412e614e: mov byte ptr [rbp + 3], 1
1412e6152: lea rcx, [rbp - 0x60]
1412e6156: mov rdx, rdi
1412e6159: xor r8d, r8d
1412e615c: call 0x141617a10
1412e6161: mov byte ptr [rbp + 2], 1
1412e6165: mov edx, 0xc
1412e616a: xor ecx, ecx
1412e616c: call 0x1416969a0
1412e6171: mov qword ptr [rbp - 0x30], rax
1412e6175: mov qword ptr [rbp - 8], rdx
1412e6179: mov edx, 0xc
1412e617e: xor ecx, ecx
1412e6180: call 0x1416969a0
1412e6185: mov rbx, rax
1412e6188: mov rdi, rdx
1412e618b: movzx eax, byte ptr [rip + 0x234ea77]
1412e6192: mov ecx, 0x30
1412e6197: mov edx, 8
1412e619c: call 0x140613c10
1412e61a1: test rax, rax
1412e61a4: je 0x1412e62d6
1412e61aa: mov qword ptr [rax], 1
1412e61b1: mov qword ptr [rax + 8], 1
1412e61b9: mov byte ptr [rax + 0x10], 0
1412e61bd: mov qword ptr [rax + 0x18], 0
1412e61c5: mov qword ptr [rax + 0x20], 8
1412e61cd: mov qword ptr [rax + 0x28], 0
1412e61d5: mov rcx, qword ptr [rbp - 0x40]
1412e61d9: mov qword ptr [rsi + 0x88], rcx
1412e61e0: movups xmm0, xmmword ptr [rbp - 0x60]
1412e61e4: movups xmm1, xmmword ptr [rbp - 0x50]
1412e61e8: movups xmmword ptr [rsi + 0x78], xmm1
1412e61ec: movups xmmword ptr [rsi + 0x68], xmm0
1412e61f0: mov rcx, qword ptr [rbp - 0x20]
1412e61f4: mov qword ptr [rsi + 0x20], rcx
1412e61f8: mov qword ptr [rsi + 0x28], 0x301
1412e6200: mov rcx, qword ptr [rbp - 0x30]
1412e6204: mov qword ptr [rsi + 0x30], rcx
1412e6208: mov rcx, qword ptr [rbp - 8]
1412e620c: mov qword ptr [rsi + 0x38], rcx
1412e6210: mov qword ptr [rsi], 0
1412e6217: mov qword ptr [rsi + 8], 0x300
1412e621f: mov rcx, qword ptr [rbp - 0x10]
1412e6223: mov qword ptr [rsi + 0x10], rcx
1412e6227: mov qword ptr [rsi + 0x18], 0x300
1412e622f: mov rcx, qword ptr [rbp - 0x18]
1412e6233: mov qword ptr [rsi + 0x40], rcx
1412e6237: mov qword ptr [rsi + 0x48], 0x6000
1412e623f: mov qword ptr [rsi + 0x50], rbx
1412e6243: mov qword ptr [rsi + 0x58], rdi
1412e6247: mov qword ptr [rsi + 0x60], rax
1412e624b: mov rax, rsi
1412e624e: movaps xmm6, xmmword ptr [rbp + 0x10]
1412e6252: movaps xmm7, xmmword ptr [rbp + 0x20]
1412e6256: movaps xmm8, xmmword ptr [rbp + 0x30]
1412e625b: movaps xmm9, xmmword ptr [rbp + 0x40]
1412e6260: add rsp, 0xd8
1412e6267: pop rbx
1412e6268: pop rdi
1412e6269: pop rsi
1412e626a: pop rbp
1412e626b: ret 
1412e626c: lea rcx, [rip + 0x1948ac5]
1412e6273: lea r8, [rip + 0x1948ade]
1412e627a: mov edx, 0x20
1412e627f: call 0x1428d9430
1412e6284: lea r8, [rip + 0x1948ae5]
1412e628b: mov ecx, 1
1412e6290: mov edx, 0x301
1412e6295: call 0x1428d8fe3
1412e629a: mov byte ptr [rbp + 7], 1
1412e629e: lea r8, [rip + 0x1948ae3]
1412e62a5: mov ecx, 4
1412e62aa: mov edx, 0xc0000
1412e62af: call 0x1428d8fe3
1412e62b4: jmp 0x1412e62ed
1412e62b6: mov byte ptr [rbp + 6], 1
1412e62ba: mov byte ptr [rbp + 5], 1
1412e62be: lea r8, [rip + 0x19486d3]
1412e62c5: mov ecx, 4
1412e62ca: mov edx, 0xc00
1412e62cf: call 0x1428d8fe3
1412e62d4: jmp 0x1412e62ed
1412e62d6: mov qword ptr [rbp - 0x38], rbx
1412e62da: mov qword ptr [rbp - 0x28], rdi
1412e62de: mov ecx, 8
1412e62e3: mov edx, 0x30
1412e62e8: call 0x1428d9000
1412e62ed: ud2 
1412e62ef: nop 
1412e62f0: mov qword ptr [rsp + 0x10], rdx
1412e62f5: push rbp
1412e62f6: push rsi
1412e62f7: push rdi
1412e62f8: push rbx
1412e62f9: sub rsp, 0x68
1412e62fd: lea rbp, [rdx + 0x80]
1412e6304: movdqa xmmword ptr [rsp + 0x20], xmm9
1412e630b: movdqa xmmword ptr [rsp + 0x30], xmm8
1412e6312: movdqa xmmword ptr [rsp + 0x40], xmm7
1412e6318: movdqa xmmword ptr [rsp + 0x50], xmm6
1412e631e: movzx ebx, byte ptr [rbp + 2]
1412e6322: mov edx, 0x301
1412e6327: mov r8d, 1
1412e632d: mov rcx, qword ptr [rbp - 0x20]
1412e6331: call 0x140613c20
1412e6336: lea rcx, [rbp - 0x60]
1412e633a: call 0x140be24b0
1412e633f: mov byte ptr [rbp + 4], 0
1412e6343: mov byte ptr [rbp + 3], bl
1412e6346: movdqa xmm6, xmmword ptr [rsp + 0x50]
1412e634c: movdqa xmm7, xmmword ptr [rsp + 0x40]
1412e6352: movdqa xmm8, xmmword ptr [rsp + 0x30]
1412e6359: movdqa xmm9, xmmword ptr [rsp + 0x20]
1412e6360: add rsp, 0x68
1412e6364: pop rbx
1412e6365: pop rdi
1412e6366: pop rsi
1412e6367: pop rbp
1412e6368: ret 
1412e6369: nop dword ptr [rax]
1412e6370: mov qword ptr [rsp + 0x10], rdx
1412e6375: push rbp
1412e6376: push rsi
1412e6377: push rdi
1412e6378: push rbx
1412e6379: sub rsp, 0x68
1412e637d: lea rbp, [rdx + 0x80]
1412e6384: movdqa xmmword ptr [rsp + 0x20], xmm9
1412e638b: movdqa xmmword ptr [rsp + 0x30], xmm8
1412e6392: movdqa xmmword ptr [rsp + 0x40], xmm7
1412e6398: movdqa xmmword ptr [rsp + 0x50], xmm6
1412e639e: mov edx, 0xc00
1412e63a3: mov r8d, 4
1412e63a9: mov rcx, qword ptr [rbp - 0x10]
1412e63ad: call 0x140613c20
1412e63b2: mov edx, 0xc0000
1412e63b7: mov r8d, 4
1412e63bd: mov rcx, qword ptr [rbp - 0x18]
1412e63c1: call 0x140613c20
1412e63c6: cmp qword ptr [rbp - 8], 0
1412e63cb: je 0x1412e63e4
1412e63cd: mov rdx, qword ptr [rbp - 8]
1412e63d1: shl rdx, 3
1412e63d5: mov r8d, 8
1412e63db: mov rcx, qword ptr [rbp - 0x30]
1412e63df: call 0x140613c20
1412e63e4: mov byte ptr [rbp + 2], 0
1412e63e8: movdqa xmm6, xmmword ptr [rsp + 0x50]
1412e63ee: movdqa xmm7, xmmword ptr [rsp + 0x40]
1412e63f4: movdqa xmm8, xmmword ptr [rsp + 0x30]
1412e63fb: movdqa xmm9, xmmword ptr [rsp + 0x20]
1412e6402: add rsp, 0x68
1412e6406: pop rbx
1412e6407: pop rdi
1412e6408: pop rsi
1412e6409: pop rbp
1412e640a: ret 
1412e640b: nop dword ptr [rax + rax]
1412e6410: mov qword ptr [rsp + 0x10], rdx
1412e6415: push rbp
1412e6416: push rsi
1412e6417: push rdi
1412e6418: push rbx
1412e6419: sub rsp, 0x68
1412e641d: lea rbp, [rdx + 0x80]
1412e6424: movdqa xmmword ptr [rsp + 0x20], xmm9
1412e642b: movdqa xmmword ptr [rsp + 0x30], xmm8
1412e6432: movdqa xmmword ptr [rsp + 0x40], xmm7
1412e6438: movdqa xmmword ptr [rsp + 0x50], xmm6
1412e643e: cmp qword ptr [rbp - 0x28], 0
1412e6443: je 0x1412e645c
1412e6445: mov rdx, qword ptr [rbp - 0x28]
1412e6449: shl rdx, 3
1412e644d: mov r8d, 8
1412e6453: mov rcx, qword ptr [rbp - 0x38]
1412e6457: call 0x140613c20
1412e645c: movdqa xmm6, xmmword ptr [rsp + 0x50]
1412e6462: movdqa xmm7, xmmword ptr [rsp + 0x40]
1412e6468: movdqa xmm8, xmmword ptr [rsp + 0x30]
1412e646f: movdqa xmm9, xmmword ptr [rsp + 0x20]
1412e6476: add rsp, 0x68
1412e647a: pop rbx
1412e647b: pop rdi
1412e647c: pop rsi
1412e647d: pop rbp
1412e647e: ret 
1412e647f: nop 
1412e6480: mov qword ptr [rsp + 0x10], rdx
1412e6485: push rbp
1412e6486: push rsi
1412e6487: push rdi
1412e6488: push rbx
1412e6489: sub rsp, 0x68
1412e648d: lea rbp, [rdx + 0x80]
1412e6494: movdqa xmmword ptr [rsp + 0x20], xmm9
1412e649b: movdqa xmmword ptr [rsp + 0x30], xmm8
1412e64a2: movdqa xmmword ptr [rsp + 0x40], xmm7
1412e64a8: movdqa xmmword ptr [rsp + 0x50], xmm6
1412e64ae: movzx ebx, byte ptr [rbp + 4]
1412e64b2: cmp byte ptr [rbp + 3], 0
1412e64b6: je 0x1412e64f9
1412e64b8: mov edx, 0xc00
1412e64bd: mov r8d, 4
1412e64c3: mov rcx, qword ptr [rbp - 0x10]
1412e64c7: call 0x140613c20
1412e64cc: and bl, 1
1412e64cf: mov byte ptr [rbp + 6], bl
1412e64d2: mov byte ptr [rbp + 5], 1
1412e64d6: movdqa xmm6, xmmword ptr [rsp + 0x50]
1412e64dc: movdqa xmm7, xmmword ptr [rsp + 0x40]
1412e64e2: movdqa xmm8, xmmword ptr [rsp + 0x30]
1412e64e9: movdqa xmm9, xmmword ptr [rsp + 0x20]
1412e64f0: add rsp, 0x68
1412e64f4: pop rbx
1412e64f5: pop rdi
1412e64f6: pop rsi
1412e64f7: pop rbp
1412e64f8: ret 
1412e64f9: and bl, 1
1412e64fc: mov byte ptr [rbp + 6], bl
1412e64ff: mov byte ptr [rbp + 5], 0
1412e6503: movdqa xmm6, xmmword ptr [rsp + 0x50]
1412e6509: movdqa xmm7, xmmword ptr [rsp + 0x40]
1412e650f: movdqa xmm8, xmmword ptr [rsp + 0x30]
1412e6516: movdqa xmm9, xmmword ptr [rsp + 0x20]
1412e651d: add rsp, 0x68
1412e6521: pop rbx
1412e6522: pop rdi
1412e6523: pop rsi
1412e6524: pop rbp
1412e6525: ret 
1412e6526: nop word ptr cs:[rax + rax]
1412e6530: mov qword ptr [rsp + 0x10], rdx
1412e6535: push rbp
1412e6536: push rsi
1412e6537: push rdi
1412e6538: push rbx
1412e6539: sub rsp, 0x68
1412e653d: lea rbp, [rdx + 0x80]
1412e6544: movdqa xmmword ptr [rsp + 0x20], xmm9
1412e654b: movdqa xmmword ptr [rsp + 0x30], xmm8
1412e6552: movdqa xmmword ptr [rsp + 0x40], xmm7
1412e6558: movdqa xmmword ptr [rsp + 0x50], xmm6
1412e655e: movzx ebx, byte ptr [rbp + 6]
1412e6562: cmp byte ptr [rbp + 5], 0
1412e6566: je 0x1412e65a5
1412e6568: mov edx, 0xc0000
1412e656d: mov r8d, 4
1412e6573: mov rcx, qword ptr [rbp - 0x18]
1412e6577: call 0x140613c20
1412e657c: and bl, 1
1412e657f: mov byte ptr [rbp + 7], bl
1412e6582: movdqa xmm6, xmmword ptr [rsp + 0x50]
1412e6588: movdqa xmm7, xmmword ptr [rsp + 0x40]
1412e658e: movdqa xmm8, xmmword ptr [rsp + 0x30]
1412e6595: movdqa xmm9, xmmword ptr [rsp + 0x20]
1412e659c: add rsp, 0x68
1412e65a0: pop rbx
1412e65a1: pop rdi
1412e65a2: pop rsi
1412e65a3: pop rbp
1412e65a4: ret 
1412e65a5: and bl, 1
1412e65a8: mov byte ptr [rbp + 7], bl
1412e65ab: movdqa xmm6, xmmword ptr [rsp + 0x50]
1412e65b1: movdqa xmm7, xmmword ptr [rsp + 0x40]
1412e65b7: movdqa xmm8, xmmword ptr [rsp + 0x30]
1412e65be: movdqa xmm9, xmmword ptr [rsp + 0x20]
1412e65c5: add rsp, 0x68
1412e65c9: pop rbx
1412e65ca: pop rdi
1412e65cb: pop rsi
1412e65cc: pop rbp
1412e65cd: ret 
1412e65ce: nop 
1412e65d0: mov qword ptr [rsp + 0x10], rdx
1412e65d5: push rbp
1412e65d6: push rsi
1412e65d7: push rdi
1412e65d8: push rbx
1412e65d9: sub rsp, 0x68
1412e65dd: lea rbp, [rdx + 0x80]
1412e65e4: movdqa xmmword ptr [rsp + 0x20], xmm9
1412e65eb: movdqa xmmword ptr [rsp + 0x30], xmm8
1412e65f2: movdqa xmmword ptr [rsp + 0x40], xmm7
1412e65f8: movdqa xmmword ptr [rsp + 0x50], xmm6
1412e65fe: cmp byte ptr [rbp + 7], 0
1412e6602: je 0x1412e6618
1412e6604: mov edx, 0x301
1412e6609: mov r8d, 1
1412e660f: mov rcx, qword ptr [rbp - 0x20]
1412e6613: call 0x140613c20
1412e6618: movaps xmm6, xmmword ptr [rsp + 0x50]
1412e661d: movaps xmm7, xmmword ptr [rsp + 0x40]
1412e6622: movaps xmm8, xmmword ptr [rsp + 0x30]
1412e6628: movaps xmm9, xmmword ptr [rsp + 0x20]
1412e662e: add rsp, 0x68
1412e6632: pop rbx
1412e6633: pop rdi
1412e6634: pop rsi
1412e6635: pop rbp
1412e6636: ret 
