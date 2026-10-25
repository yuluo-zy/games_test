1412c5100: push rbp
1412c5101: push rsi
1412c5102: sub rsp, 0xd8
1412c5109: lea rbp, [rsp + 0x80]
1412c5111: mov qword ptr [rbp + 0x50], 0xfffffffffffffffe
1412c5119: mov rsi, rcx
1412c511c: lea rcx, [rbp - 0x58]
1412c5120: xor r8d, r8d
1412c5123: call 0x141612170
1412c5128: movzx eax, byte ptr [rip + 0x236fada]
1412c512f: mov ecx, 0x28
1412c5134: mov edx, 8
1412c5139: call 0x140613c10
1412c513e: test rax, rax
1412c5141: je 0x1412c51f7
1412c5147: mov rcx, qword ptr [rbp - 0x38]
1412c514b: mov qword ptr [rax + 0x20], rcx
1412c514f: movups xmm0, xmmword ptr [rbp - 0x58]
1412c5153: movups xmm1, xmmword ptr [rbp - 0x48]
1412c5157: movups xmmword ptr [rax + 0x10], xmm1
1412c515b: movups xmmword ptr [rax], xmm0
1412c515e: lea rcx, [rip + 0x17d8b39]
1412c5165: mov qword ptr [rbp], rcx
1412c5169: mov qword ptr [rbp + 8], 0x61
1412c5171: mov qword ptr [rbp + 0x10], rax
1412c5175: lea rax, [rip + 0x196807c]
1412c517c: mov qword ptr [rbp + 0x18], rax
1412c5180: mov qword ptr [rbp + 0x30], 0
1412c5188: mov byte ptr [rbp + 0x46], 5
1412c518c: mov qword ptr [rbp + 0x20], rcx
1412c5190: mov qword ptr [rbp + 0x28], 0x61
1412c5198: movzx eax, byte ptr [rip + 0x236fa6a]
1412c519f: mov ecx, 0x40
1412c51a4: mov edx, 8
1412c51a9: call 0x140613c10
1412c51ae: test rax, rax
1412c51b1: je 0x1412c5208
1412c51b3: movups xmm0, xmmword ptr [rbp + 0x10]
1412c51b7: movups xmm1, xmmword ptr [rbp + 0x20]
1412c51bb: movups xmm2, xmmword ptr [rbp + 0x30]
1412c51bf: movups xmm3, xmmword ptr [rbp + 0x40]
1412c51c3: movups xmmword ptr [rax + 0x30], xmm3
1412c51c7: movups xmmword ptr [rax + 0x20], xmm2
1412c51cb: movups xmmword ptr [rax + 0x10], xmm1
1412c51cf: movups xmmword ptr [rax], xmm0
1412c51d2: movabs rdx, 0xdec66ec37fabbb05
1412c51dc: mov rcx, rsi
1412c51df: mov r8, rax
1412c51e2: call 0x140969bf0
1412c51e7: cmp rax, 1
1412c51eb: je 0x1412c5219
1412c51ed: add rsp, 0xd8
1412c51f4: pop rsi
1412c51f5: pop rbp
1412c51f6: ret 
1412c51f7: mov ecx, 8
1412c51fc: mov edx, 0x28
1412c5201: call 0x1428d9000
1412c5206: jmp 0x1412c5217
1412c5208: mov ecx, 8
1412c520d: mov edx, 0x40
1412c5212: call 0x1428d9000
1412c5217: ud2 
1412c5219: mov rax, rdx
1412c521c: add rdx, 0x10
1412c5220: mov rsi, rdx
1412c5223: cmp qword ptr [rax + 0x18], 0x61
1412c5228: jne 0x1412c52a6
1412c522a: mov rdx, qword ptr [rsi]
1412c522d: lea rcx, [rip + 0x17d8a6a]
1412c5234: mov r8d, 0x61
1412c523a: call 0x142923bf0
1412c523f: test eax, eax
1412c5241: jne 0x1412c52a6
1412c5243: lea rax, [rip + 0x19684c6]
1412c524a: mov qword ptr [rbp - 0x30], rax
1412c524e: lea rax, [rip - 0xd919b5]
1412c5255: mov qword ptr [rbp - 0x28], rax
1412c5259: mov rax, rbp
1412c525c: mov qword ptr [rbp - 0x20], rax
1412c5260: lea rax, [rip - 0x11bb3d7]
1412c5267: mov qword ptr [rbp - 0x18], rax
1412c526b: lea rax, [rip + 0x19683b6]
1412c5272: mov qword ptr [rbp + 0x10], rax
1412c5276: mov qword ptr [rbp + 0x18], 3
1412c527e: mov qword ptr [rbp + 0x30], 0
1412c5286: lea rax, [rbp - 0x30]
1412c528a: mov qword ptr [rbp + 0x20], rax
1412c528e: mov qword ptr [rbp + 0x28], 2
1412c5296: lea rdx, [rip + 0x19683bb]
1412c529d: lea rcx, [rbp + 0x10]
1412c52a1: call 0x1428d9390
1412c52a6: lea rax, [rip + 0x1968463]
1412c52ad: mov qword ptr [rbp + 0x10], rax
1412c52b1: lea rax, [rip - 0xd91a18]
1412c52b8: mov qword ptr [rbp + 0x18], rax
1412c52bc: mov qword ptr [rbp + 0x20], rsi
1412c52c0: lea rax, [rip - 0x11bb437]
1412c52c7: mov qword ptr [rbp + 0x28], rax
1412c52cb: mov rcx, rbp
1412c52ce: mov qword ptr [rbp + 0x30], rcx
1412c52d2: mov qword ptr [rbp + 0x38], rax
1412c52d6: lea rax, [rip + 0x196828b]
1412c52dd: mov qword ptr [rbp - 0x30], rax
1412c52e1: mov qword ptr [rbp - 0x28], 3
1412c52e9: mov qword ptr [rbp - 0x10], 0
1412c52f1: lea rax, [rbp + 0x10]
1412c52f5: mov qword ptr [rbp - 0x20], rax
1412c52f9: mov qword ptr [rbp - 0x18], 3
1412c5301: lea rdx, [rip + 0x19682c8]
1412c5308: lea rcx, [rbp - 0x30]
1412c530c: call 0x1428d9390
1412c5311: int3 
1412c5312: nop word ptr cs:[rax + rax]
1412c5320: mov qword ptr [rsp + 0x10], rdx
1412c5325: push rbp
1412c5326: push rsi
1412c5327: sub rsp, 0x28
1412c532b: lea rbp, [rdx + 0x80]
1412c5332: lea rcx, [rbp - 0x58]
1412c5336: call 0x140be24b0
1412c533b: nop 
1412c533c: add rsp, 0x28
1412c5340: pop rsi
1412c5341: pop rbp
1412c5342: ret 
1412c5343: nop word ptr cs:[rax + rax]
1412c5350: mov qword ptr [rsp + 0x10], rdx
1412c5355: push rbp
1412c5356: push rsi
1412c5357: sub rsp, 0x28
1412c535b: lea rbp, [rdx + 0x80]
1412c5362: lea rcx, [rbp + 0x10]
1412c5366: call 0x1400c04b0
1412c536b: nop 
1412c536c: add rsp, 0x28
1412c5370: pop rsi
1412c5371: pop rbp
1412c5372: ret 
