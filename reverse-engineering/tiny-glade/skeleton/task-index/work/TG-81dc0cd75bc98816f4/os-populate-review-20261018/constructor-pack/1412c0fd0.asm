1412c0fd0: push rbp
1412c0fd1: push rsi
1412c0fd2: sub rsp, 0xd8
1412c0fd9: lea rbp, [rsp + 0x80]
1412c0fe1: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412c0fe9: mov rsi, rcx
1412c0fec: lea rcx, [rbp - 0x58]
1412c0ff0: xor r8d, r8d
1412c0ff3: call 0x141613720
1412c0ff8: movzx eax, byte ptr [rip + 0x2373c0a]
1412c0fff: mov ecx, 0x28
1412c1004: mov edx, 8
1412c1009: call 0x140613c10
1412c100e: test rax, rax
1412c1011: je 0x1412c10c7
1412c1017: mov rcx, qword ptr [rbp - 0x38]
1412c101b: mov qword ptr [rax + 0x20], rcx
1412c101f: movups xmm0, xmmword ptr [rbp - 0x58]
1412c1023: movups xmm1, xmmword ptr [rbp - 0x48]
1412c1027: movups xmmword ptr [rax + 0x10], xmm1
1412c102b: movups xmmword ptr [rax], xmm0
1412c102e: lea rcx, [rip + 0x17dc6ea]
1412c1035: mov qword ptr [rbp], rcx
1412c1039: mov qword ptr [rbp + 8], 0x5c
1412c1041: mov qword ptr [rbp + 0x10], rax
1412c1045: lea rax, [rip + 0x196bbfc]
1412c104c: mov qword ptr [rbp + 0x18], rax
1412c1050: mov qword ptr [rbp + 0x30], 0
1412c1058: mov byte ptr [rbp + 0x46], 5
1412c105c: mov qword ptr [rbp + 0x20], rcx
1412c1060: mov qword ptr [rbp + 0x28], 0x5c
1412c1068: movzx eax, byte ptr [rip + 0x2373b9a]
1412c106f: mov ecx, 0x40
1412c1074: mov edx, 8
1412c1079: call 0x140613c10
1412c107e: test rax, rax
1412c1081: je 0x1412c10d8
1412c1083: movups xmm0, xmmword ptr [rbp + 0x10]
1412c1087: movups xmm1, xmmword ptr [rbp + 0x20]
1412c108b: movups xmm2, xmmword ptr [rbp + 0x30]
1412c108f: movups xmm3, xmmword ptr [rbp + 0x40]
1412c1093: movups xmmword ptr [rax + 0x30], xmm3
1412c1097: movups xmmword ptr [rax + 0x20], xmm2
1412c109b: movups xmmword ptr [rax + 0x10], xmm1
1412c109f: movups xmmword ptr [rax], xmm0
1412c10a2: movabs rdx, 0x480eafc7a5a8a812
1412c10ac: mov rcx, rsi
1412c10af: mov r8, rax
1412c10b2: call 0x140969bf0
1412c10b7: cmp rax, 1
1412c10bb: je 0x1412c10e9
1412c10bd: add rsp, 0xd8
1412c10c4: pop rsi
1412c10c5: pop rbp
1412c10c6: ret 
1412c10c7: mov ecx, 8
1412c10cc: mov edx, 0x28
1412c10d1: call 0x1428d9000
1412c10d6: jmp 0x1412c10e7
1412c10d8: mov ecx, 8
1412c10dd: mov edx, 0x40
1412c10e2: call 0x1428d9000
1412c10e7: ud2 
1412c10e9: mov rax, rdx
1412c10ec: add rdx, 0x10
1412c10f0: mov rsi, rdx
1412c10f3: cmp qword ptr [rax + 0x18], 0x5c
1412c10f8: jne 0x1412c1176
1412c10fa: mov rdx, qword ptr [rsi]
1412c10fd: lea rcx, [rip + 0x17dc61b]
1412c1104: mov r8d, 0x5c
1412c110a: call 0x142923bf0
1412c110f: test eax, eax
1412c1111: jne 0x1412c1176
1412c1113: lea rax, [rip + 0x196c676]
1412c111a: mov qword ptr [rbp - 0x30], rax
1412c111e: lea rax, [rip - 0xd8d885]
1412c1125: mov qword ptr [rbp - 0x28], rax
1412c1129: mov rax, rbp
1412c112c: mov qword ptr [rbp - 0x20], rax
1412c1130: lea rax, [rip - 0x11b72a7]
1412c1137: mov qword ptr [rbp - 0x18], rax
1412c113b: lea rax, [rip + 0x196c4e6]
1412c1142: mov qword ptr [rbp + 0x10], rax
1412c1146: mov qword ptr [rbp + 0x18], 3
1412c114e: mov qword ptr [rbp + 0x30], 0
1412c1156: lea rax, [rbp - 0x30]
1412c115a: mov qword ptr [rbp + 0x20], rax
1412c115e: mov qword ptr [rbp + 0x28], 2
1412c1166: lea rdx, [rip + 0x196c4eb]
1412c116d: lea rcx, [rbp + 0x10]
1412c1171: call 0x1428d9390
1412c1176: lea rax, [rip + 0x196c613]
1412c117d: mov qword ptr [rbp + 0x10], rax
1412c1181: lea rax, [rip - 0xd8d8e8]
1412c1188: mov qword ptr [rbp + 0x18], rax
1412c118c: mov qword ptr [rbp + 0x20], rsi
1412c1190: lea rax, [rip - 0x11b7307]
1412c1197: mov qword ptr [rbp + 0x28], rax
1412c119b: mov rcx, rbp
1412c119e: mov qword ptr [rbp + 0x30], rcx
1412c11a2: mov qword ptr [rbp + 0x38], rax
1412c11a6: lea rax, [rip + 0x196c3bb]
1412c11ad: mov qword ptr [rbp - 0x30], rax
1412c11b1: mov qword ptr [rbp - 0x28], 3
1412c11b9: mov qword ptr [rbp - 0x10], 0
1412c11c1: lea rax, [rbp + 0x10]
1412c11c5: mov qword ptr [rbp - 0x20], rax
1412c11c9: mov qword ptr [rbp - 0x18], 3
1412c11d1: lea rdx, [rip + 0x196c3f8]
1412c11d8: lea rcx, [rbp - 0x30]
1412c11dc: call 0x1428d9390
1412c11e1: int3 
1412c11e2: nop word ptr cs:[rax + rax]
1412c11f0: mov qword ptr [rsp + 0x10], rdx
1412c11f5: push rbp
1412c11f6: push rsi
1412c11f7: sub rsp, 0x28
1412c11fb: lea rbp, [rdx + 0x80]
1412c1202: lea rcx, [rbp - 0x58]
1412c1206: call 0x140be24b0
1412c120b: nop 
1412c120c: add rsp, 0x28
1412c1210: pop rsi
1412c1211: pop rbp
1412c1212: ret 
1412c1213: nop word ptr cs:[rax + rax]
1412c1220: mov qword ptr [rsp + 0x10], rdx
1412c1225: push rbp
1412c1226: push rsi
1412c1227: sub rsp, 0x28
1412c122b: lea rbp, [rdx + 0x80]
1412c1232: lea rcx, [rbp + 0x10]
1412c1236: call 0x1400c04b0
1412c123b: nop 
1412c123c: add rsp, 0x28
1412c1240: pop rsi
1412c1241: pop rbp
1412c1242: ret 
