1417c1170: 56  push rsi
1417c1171: 57  push rdi
1417c1172: 53  push rbx
1417c1173: 4883ec20  sub rsp, 0x20
1417c1177: 4889ce  mov rsi, rcx
1417c117a: 498d40fe  lea rax, [r8 - 2]
1417c117e: 4883f810  cmp rax, 0x10
1417c1182: 0f87e1010000  ja 0x1417c1369
1417c1188: 488d0db5125201  lea rcx, [rip + 0x15212b5]
1417c118f: 48630481  movsxd rax, dword ptr [rcx + rax*4]
1417c1193: 4801c8  add rax, rcx
1417c1196: ffe0  jmp rax
1417c1198: 66813a7479  cmp word ptr [rdx], 0x7974
1417c119d: 0f85c6010000  jne 0x1417c1369
1417c11a3: c6460101  mov byte ptr [rsi + 1], 1
1417c11a7: e9c4010000  jmp 0x1417c1370
1417c11ac: 488d05aa055201  lea rax, [rip + 0x15205aa]
1417c11b3: 4889d7  mov rdi, rdx
1417c11b6: 4889d1  mov rcx, rdx
1417c11b9: 4889c2  mov rdx, rax
1417c11bc: 4c89c3  mov rbx, r8
1417c11bf: e82c2a1601  call 0x142923bf0
1417c11c4: 85c0  test eax, eax
1417c11c6: 0f84b8010000  je 0x1417c1384
1417c11cc: 488d159c055201  lea rdx, [rip + 0x152059c]
1417c11d3: 4889f9  mov rcx, rdi
1417c11d6: 4989d8  mov r8, rbx
1417c11d9: e8122a1601  call 0x142923bf0
1417c11de: 85c0  test eax, eax
1417c11e0: 0f8583010000  jne 0x1417c1369
1417c11e6: c6460109  mov byte ptr [rsi + 1], 9
1417c11ea: e981010000  jmp 0x1417c1370
1417c11ef: 48b8636f6c6f725f6964  movabs rax, 0x64695f726f6c6f63
1417c11f9: 483902  cmp qword ptr [rdx], rax
1417c11fc: 0f8488010000  je 0x1417c138a
1417c1202: 48b8685f6f6666736574  movabs rax, 0x74657366666f5f68
1417c120c: 483902  cmp qword ptr [rdx], rax
1417c120f: 0f8554010000  jne 0x1417c1369
1417c1215: e964010000  jmp 0x1417c137e
1417c121a: 488d056d055201  lea rax, [rip + 0x152056d]
1417c1221: 4889d7  mov rdi, rdx
1417c1224: 4889d1  mov rcx, rdx
1417c1227: 4889c2  mov rdx, rax
1417c122a: 4c89c3  mov rbx, r8
1417c122d: e8be291601  call 0x142923bf0
1417c1232: 85c0  test eax, eax
1417c1234: 0f8456010000  je 0x1417c1390
1417c123a: 488d157a055201  lea rdx, [rip + 0x152057a]
1417c1241: 4889f9  mov rcx, rdi
1417c1244: 4989d8  mov r8, rbx
1417c1247: e8a4291601  call 0x142923bf0
1417c124c: 488d4e01  lea rcx, [rsi + 1]
1417c1250: 85c0  test eax, eax
1417c1252: 0f8515010000  jne 0x1417c136d
1417c1258: c6010f  mov byte ptr [rcx], 0xf
1417c125b: e910010000  jmp 0x1417c1370
1417c1260: 488d053e055201  lea rax, [rip + 0x152053e]
1417c1267: 4889d1  mov rcx, rdx
1417c126a: 4889c2  mov rdx, rax
1417c126d: e87e291601  call 0x142923bf0
1417c1272: 85c0  test eax, eax
1417c1274: 0f85ef000000  jne 0x1417c1369
1417c127a: c646010d  mov byte ptr [rsi + 1], 0xd
1417c127e: e9ed000000  jmp 0x1417c1370
1417c1283: 813a72616e6b  cmp dword ptr [rdx], 0x6b6e6172
1417c1289: 0f8407010000  je 0x1417c1396
1417c128f: 813a73656564  cmp dword ptr [rdx], 0x64656573
1417c1295: 0f85ce000000  jne 0x1417c1369
1417c129b: c6460104  mov byte ptr [rsi + 1], 4
1417c129f: e9cc000000  jmp 0x1417c1370
1417c12a4: 0fb702  movzx eax, word ptr [rdx]
1417c12a7: 3564730000  xor eax, 0x7364
1417c12ac: 0fb64a02  movzx ecx, byte ptr [rdx + 2]
1417c12b0: 83f174  xor ecx, 0x74
1417c12b3: 6609c1  or cx, ax
1417c12b6: 0f84e0000000  je 0x1417c139c
1417c12bc: 0fb702  movzx eax, word ptr [rdx]
1417c12bf: 35636c0000  xor eax, 0x6c63
1417c12c4: 0fb64a02  movzx ecx, byte ptr [rdx + 2]
1417c12c8: 83f176  xor ecx, 0x76
1417c12cb: 6609c1  or cx, ax
1417c12ce: 0f8595000000  jne 0x1417c1369
1417c12d4: c6460105  mov byte ptr [rsi + 1], 5
1417c12d8: e993000000  jmp 0x1417c1370
1417c12dd: 488d05b5045201  lea rax, [rip + 0x15204b5]
1417c12e4: 4889d1  mov rcx, rdx
1417c12e7: 4889c2  mov rdx, rax
1417c12ea: e801291601  call 0x142923bf0
1417c12ef: 85c0  test eax, eax
1417c12f1: 7576  jne 0x1417c1369
1417c12f3: c646010c  mov byte ptr [rsi + 1], 0xc
1417c12f7: eb77  jmp 0x1417c1370
1417c12f9: 488d0581045201  lea rax, [rip + 0x1520481]
1417c1300: 4889d1  mov rcx, rdx
1417c1303: 4889c2  mov rdx, rax
1417c1306: e8e5281601  call 0x142923bf0
1417c130b: 85c0  test eax, eax
1417c130d: 755a  jne 0x1417c1369
1417c130f: c646010a  mov byte ptr [rsi + 1], 0xa
1417c1313: eb5b  jmp 0x1417c1370
1417c1315: f30f6f02  movdqu xmm0, xmmword ptr [rdx]
1417c1319: 660f74059ffe5101  pcmpeqb xmm0, xmmword ptr [rip + 0x151fe9f]
1417c1321: 660fd7c0  pmovmskb eax, xmm0
1417c1325: 3dffff0000  cmp eax, 0xffff
1417c132a: 7452  je 0x1417c137e
1417c132c: f30f6f02  movdqu xmm0, xmmword ptr [rdx]
1417c1330: 660f740598fe5101  pcmpeqb xmm0, xmmword ptr [rip + 0x151fe98]
1417c1338: 660fd7c0  pmovmskb eax, xmm0
1417c133c: 3dffff0000  cmp eax, 0xffff
1417c1341: 7526  jne 0x1417c1369
1417c1343: c6460107  mov byte ptr [rsi + 1], 7
1417c1347: eb27  jmp 0x1417c1370
1417c1349: 48b87374616972735f73  movabs rax, 0x735f737269617473
1417c1353: 483302  xor rax, qword ptr [rdx]
1417c1356: 48b9737570706f727473  movabs rcx, 0x7374726f70707573
1417c1360: 48334a07  xor rcx, qword ptr [rdx + 7]
1417c1364: 4809c1  or rcx, rax
1417c1367: 7439  je 0x1417c13a2
1417c1369: 488d4e01  lea rcx, [rsi + 1]
1417c136d: c60110  mov byte ptr [rcx], 0x10
1417c1370: c60600  mov byte ptr [rsi], 0
1417c1373: 4889f0  mov rax, rsi
1417c1376: 4883c420  add rsp, 0x20
1417c137a: 5b  pop rbx
1417c137b: 5f  pop rdi
1417c137c: 5e  pop rsi
1417c137d: c3  ret 
1417c137e: c6460106  mov byte ptr [rsi + 1], 6
1417c1382: ebec  jmp 0x1417c1370
1417c1384: c6460108  mov byte ptr [rsi + 1], 8
1417c1388: ebe6  jmp 0x1417c1370
1417c138a: c6460103  mov byte ptr [rsi + 1], 3
1417c138e: ebe0  jmp 0x1417c1370
1417c1390: c646010b  mov byte ptr [rsi + 1], 0xb
1417c1394: ebda  jmp 0x1417c1370
1417c1396: c6460102  mov byte ptr [rsi + 1], 2
1417c139a: ebd4  jmp 0x1417c1370
1417c139c: c6460100  mov byte ptr [rsi + 1], 0
1417c13a0: ebce  jmp 0x1417c1370
1417c13a2: c646010e  mov byte ptr [rsi + 1], 0xe
1417c13a6: ebc8  jmp 0x1417c1370
