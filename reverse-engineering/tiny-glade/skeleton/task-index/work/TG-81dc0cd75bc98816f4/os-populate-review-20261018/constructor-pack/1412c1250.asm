1412c1250: push rbp
1412c1251: push rsi
1412c1252: sub rsp, 0xd8
1412c1259: lea rbp, [rsp + 0x80]
1412c1261: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412c1269: mov rsi, rcx
1412c126c: lea rcx, [rbp - 0x58]
1412c1270: xor r8d, r8d
1412c1273: call 0x141613b90
1412c1278: movzx eax, byte ptr [rip + 0x237398a]
1412c127f: mov ecx, 0x28
1412c1284: mov edx, 8
1412c1289: call 0x140613c10
1412c128e: test rax, rax
1412c1291: je 0x1412c1347
1412c1297: mov rcx, qword ptr [rbp - 0x38]
1412c129b: mov qword ptr [rax + 0x20], rcx
1412c129f: movups xmm0, xmmword ptr [rbp - 0x58]
1412c12a3: movups xmm1, xmmword ptr [rbp - 0x48]
1412c12a7: movups xmmword ptr [rax + 0x10], xmm1
1412c12ab: movups xmmword ptr [rax], xmm0
1412c12ae: lea rcx, [rip + 0x17dd95e]
1412c12b5: mov qword ptr [rbp], rcx
1412c12b9: mov qword ptr [rbp + 8], 0x4e
1412c12c1: mov qword ptr [rbp + 0x10], rax
1412c12c5: lea rax, [rip + 0x196b9b4]
1412c12cc: mov qword ptr [rbp + 0x18], rax
1412c12d0: mov qword ptr [rbp + 0x30], 0
1412c12d8: mov byte ptr [rbp + 0x46], 5
1412c12dc: mov qword ptr [rbp + 0x20], rcx
1412c12e0: mov qword ptr [rbp + 0x28], 0x4e
1412c12e8: movzx eax, byte ptr [rip + 0x237391a]
1412c12ef: mov ecx, 0x40
1412c12f4: mov edx, 8
1412c12f9: call 0x140613c10
1412c12fe: test rax, rax
1412c1301: je 0x1412c1358
1412c1303: movups xmm0, xmmword ptr [rbp + 0x10]
1412c1307: movups xmm1, xmmword ptr [rbp + 0x20]
1412c130b: movups xmm2, xmmword ptr [rbp + 0x30]
1412c130f: movups xmm3, xmmword ptr [rbp + 0x40]
1412c1313: movups xmmword ptr [rax + 0x30], xmm3
1412c1317: movups xmmword ptr [rax + 0x20], xmm2
1412c131b: movups xmmword ptr [rax + 0x10], xmm1
1412c131f: movups xmmword ptr [rax], xmm0
1412c1322: movabs rdx, 0xb9559015b7cea7e6
1412c132c: mov rcx, rsi
1412c132f: mov r8, rax
1412c1332: call 0x140969bf0
1412c1337: cmp rax, 1
1412c133b: je 0x1412c1369
1412c133d: add rsp, 0xd8
1412c1344: pop rsi
1412c1345: pop rbp
1412c1346: ret 
1412c1347: mov ecx, 8
1412c134c: mov edx, 0x28
1412c1351: call 0x1428d9000
1412c1356: jmp 0x1412c1367
1412c1358: mov ecx, 8
1412c135d: mov edx, 0x40
1412c1362: call 0x1428d9000
1412c1367: ud2 
1412c1369: mov rax, rdx
1412c136c: add rdx, 0x10
1412c1370: mov rsi, rdx
1412c1373: cmp qword ptr [rax + 0x18], 0x4e
1412c1378: jne 0x1412c13f6
1412c137a: mov rdx, qword ptr [rsi]
1412c137d: lea rcx, [rip + 0x17dd88f]
1412c1384: mov r8d, 0x4e
1412c138a: call 0x142923bf0
1412c138f: test eax, eax
1412c1391: jne 0x1412c13f6
1412c1393: lea rax, [rip + 0x196cb3e]
1412c139a: mov qword ptr [rbp - 0x30], rax
1412c139e: lea rax, [rip - 0xd8db05]
1412c13a5: mov qword ptr [rbp - 0x28], rax
1412c13a9: mov rax, rbp
1412c13ac: mov qword ptr [rbp - 0x20], rax
1412c13b0: lea rax, [rip - 0x11b7527]
1412c13b7: mov qword ptr [rbp - 0x18], rax
1412c13bb: lea rax, [rip + 0x196c266]
1412c13c2: mov qword ptr [rbp + 0x10], rax
1412c13c6: mov qword ptr [rbp + 0x18], 3
1412c13ce: mov qword ptr [rbp + 0x30], 0
1412c13d6: lea rax, [rbp - 0x30]
1412c13da: mov qword ptr [rbp + 0x20], rax
1412c13de: mov qword ptr [rbp + 0x28], 2
1412c13e6: lea rdx, [rip + 0x196c26b]
1412c13ed: lea rcx, [rbp + 0x10]
1412c13f1: call 0x1428d9390
1412c13f6: lea rax, [rip + 0x196cadb]
1412c13fd: mov qword ptr [rbp + 0x10], rax
1412c1401: lea rax, [rip - 0xd8db68]
1412c1408: mov qword ptr [rbp + 0x18], rax
1412c140c: mov qword ptr [rbp + 0x20], rsi
1412c1410: lea rax, [rip - 0x11b7587]
1412c1417: mov qword ptr [rbp + 0x28], rax
1412c141b: mov rcx, rbp
1412c141e: mov qword ptr [rbp + 0x30], rcx
1412c1422: mov qword ptr [rbp + 0x38], rax
1412c1426: lea rax, [rip + 0x196c13b]
1412c142d: mov qword ptr [rbp - 0x30], rax
1412c1431: mov qword ptr [rbp - 0x28], 3
1412c1439: mov qword ptr [rbp - 0x10], 0
1412c1441: lea rax, [rbp + 0x10]
1412c1445: mov qword ptr [rbp - 0x20], rax
1412c1449: mov qword ptr [rbp - 0x18], 3
1412c1451: lea rdx, [rip + 0x196c178]
1412c1458: lea rcx, [rbp - 0x30]
1412c145c: call 0x1428d9390
1412c1461: int3 
1412c1462: nop word ptr cs:[rax + rax]
1412c1470: mov qword ptr [rsp + 0x10], rdx
1412c1475: push rbp
1412c1476: push rsi
1412c1477: sub rsp, 0x28
1412c147b: lea rbp, [rdx + 0x80]
1412c1482: lea rcx, [rbp - 0x58]
1412c1486: call 0x140be24b0
1412c148b: nop 
1412c148c: add rsp, 0x28
1412c1490: pop rsi
1412c1491: pop rbp
1412c1492: ret 
1412c1493: nop word ptr cs:[rax + rax]
1412c14a0: mov qword ptr [rsp + 0x10], rdx
1412c14a5: push rbp
1412c14a6: push rsi
1412c14a7: sub rsp, 0x28
1412c14ab: lea rbp, [rdx + 0x80]
1412c14b2: lea rcx, [rbp + 0x10]
1412c14b6: call 0x1400c04b0
1412c14bb: nop 
1412c14bc: add rsp, 0x28
1412c14c0: pop rsi
1412c14c1: pop rbp
1412c14c2: ret 
