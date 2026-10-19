140c90050: 55 push rbp
140c90051: 4157 push r15
140c90053: 4156 push r14
140c90055: 4155 push r13
140c90057: 4154 push r12
140c90059: 56 push rsi
140c9005a: 57 push rdi
140c9005b: 53 push rbx
140c9005c: 4881ec98000000 sub rsp, 0x98
140c90063: 488dac2480000000 lea rbp, [rsp + 0x80]
140c9006b: 440f294d00 movaps xmmword ptr [rbp], xmm9
140c90070: 440f2945f0 movaps xmmword ptr [rbp - 0x10], xmm8
140c90075: 0f297de0 movaps xmmword ptr [rbp - 0x20], xmm7
140c90079: 0f2975d0 movaps xmmword ptr [rbp - 0x30], xmm6
140c9007d: 48c745c8feffffff mov qword ptr [rbp - 0x38], 0xfffffffffffffffe
140c90085: 4c89ce mov rsi, r9
140c90088: 4883f903 cmp rcx, 3
140c9008c: 732f jae 0x140c900bd
140c9008e: 488b06 mov rax, qword ptr [rsi]
140c90091: 488b5610 mov rdx, qword ptr [rsi + 0x10]
140c90095: 4829d0 sub rax, rdx
140c90098: 4883f801 cmp rax, 1
140c9009c: 0f86e8010000 jbe 0x140c9028a
140c900a2: 488b4608 mov rax, qword ptr [rsi + 8]
140c900a6: 4883c610 add rsi, 0x10
140c900aa: 48b9000000000000803f movabs rcx, 0x3f80000000000000
140c900b4: 48890c90 mov qword ptr [rax + rdx*4], rcx
140c900b8: e9a0010000 jmp 0x140c9025d
140c900bd: 4c89c7 mov rdi, r8
140c900c0: 0f28f1 movaps xmm6, xmm1
140c900c3: 4889cb mov rbx, rcx
140c900c6: 4889c8 mov rax, rcx
140c900c9: 48ffc8 dec rax
140c900cc: 7807 js 0x140c900d5
140c900ce: f3480f2ac0 cvtsi2ss xmm0, rax
140c900d3: eb15 jmp 0x140c900ea
140c900d5: 4889c1 mov rcx, rax
140c900d8: 48d1e9 shr rcx, 1
140c900db: 83e001 and eax, 1
140c900de: 4809c8 or rax, rcx
140c900e1: f3480f2ac0 cvtsi2ss xmm0, rax
140c900e6: f30f58c0 addss xmm0, xmm0
140c900ea: 4c8bb580000000 mov r14, qword ptr [rbp + 0x80]
140c900f1: f30f103d0b58c901 movss xmm7, dword ptr [rip + 0x1c9580b]
140c900f9: 440f28c7 movaps xmm8, xmm7
140c900fd: f3440f5ec0 divss xmm8, xmm0
140c90102: f3440f59056dd7e901 mulss xmm8, dword ptr [rip + 0x1e9d76d]
140c9010b: 450f28c8 movaps xmm9, xmm8
140c9010f: f3440f5dce minss xmm9, xmm6
140c90114: f30fc2f603 cmpunordss xmm6, xmm6
140c90119: 4c8b7e10 mov r15, qword ptr [rsi + 0x10]
140c9011d: 4c3b3e cmp r15, qword ptr [rsi]
140c90120: 750b jne 0x140c9012d
140c90122: 4889f1 mov rcx, rsi
140c90125: 4c89f2 mov rdx, r14
140c90128: e843f7ecff call 0x140b5f870
140c9012d: 440f54c6 andps xmm8, xmm6
140c90131: 410f55f1 andnps xmm6, xmm9
140c90135: 488b4608 mov rax, qword ptr [rsi + 8]
140c90139: 42c704b800000000 mov dword ptr [rax + r15*4], 0
140c90141: 49ffc7 inc r15
140c90144: 4c897e10 mov qword ptr [rsi + 0x10], r15
140c90148: 4885db test rbx, rbx
140c9014b: 780b js 0x140c90158
140c9014d: 450f57c9 xorps xmm9, xmm9
140c90151: f34c0f2acb cvtsi2ss xmm9, rbx
140c90156: eb1c jmp 0x140c90174
140c90158: 4889d8 mov rax, rbx
140c9015b: 48d1e8 shr rax, 1
140c9015e: 89d9 mov ecx, ebx
140c90160: 83e101 and ecx, 1
140c90163: 4809c1 or rcx, rax
140c90166: 450f57c9 xorps xmm9, xmm9
140c9016a: f34c0f2ac9 cvtsi2ss xmm9, rcx
140c9016f: f3450f58c9 addss xmm9, xmm9
140c90174: 410f56f0 orps xmm6, xmm8
140c90178: f3440f580d639fcc01 addss xmm9, dword ptr [rip + 0x1cc9f63]
140c90181: 4883c3fe add rbx, -2
140c90185: 488b06 mov rax, qword ptr [rsi]
140c90188: 4c29f8 sub rax, r15
140c9018b: 4839d8 cmp rax, rbx
140c9018e: 0f821c010000 jb 0x140c902b0
140c90194: 488d4610 lea rax, [rsi + 0x10]
140c90198: 488945b8 mov qword ptr [rbp - 0x48], rax
140c9019c: f3410f5ef9 divss xmm7, xmm9
140c901a1: 4c897db0 mov qword ptr [rbp - 0x50], r15
140c901a5: 4e8d3cbd00000000 lea r15, [r15*4]
140c901ad: 4c8b6608 mov r12, qword ptr [rsi + 8]
140c901b1: 4d01fc add r12, r15
140c901b4: 48c745c000000000 mov qword ptr [rbp - 0x40], 0
140c901bc: f3440f1005239fcc01 movss xmm8, dword ptr [rip + 0x1cc9f23]
140c901c5: eb35 jmp 0x140c901fc
140c901c7: 660f1f840000000000 nop word ptr [rax + rax]
140c901d0: 0f57c9 xorps xmm1, xmm1
140c901d3: f3490f2acd cvtsi2ss xmm1, r13
140c901d8: f30f59cf mulss xmm1, xmm7
140c901dc: f3410f58c0 addss xmm0, xmm8
140c901e1: f30f59c6 mulss xmm0, xmm6
140c901e5: f30f58c1 addss xmm0, xmm1
140c901e9: 488b45c0 mov rax, qword ptr [rbp - 0x40]
140c901ed: f3410f110484 movss dword ptr [r12 + rax*4], xmm0
140c901f3: 4c896dc0 mov qword ptr [rbp - 0x40], r13
140c901f7: 4c39eb cmp rbx, r13
140c901fa: 742e je 0x140c9022a
140c901fc: 4889f9 mov rcx, rdi
140c901ff: e87cf10100 call 0x140caf380
140c90204: 4c8b6dc0 mov r13, qword ptr [rbp - 0x40]
140c90208: 49ffc5 inc r13
140c9020b: 79c3 jns 0x140c901d0
140c9020d: 4c89e8 mov rax, r13
140c90210: 48d1e8 shr rax, 1
140c90213: 4489e9 mov ecx, r13d
140c90216: 83e101 and ecx, 1
140c90219: 4809c1 or rcx, rax
140c9021c: 0f57c9 xorps xmm1, xmm1
140c9021f: f3480f2ac9 cvtsi2ss xmm1, rcx
140c90224: f30f58c9 addss xmm1, xmm1
140c90228: ebae jmp 0x140c901d8
140c9022a: 488b7db0 mov rdi, qword ptr [rbp - 0x50]
140c9022e: 4a8d042f lea rax, [rdi + r13]
140c90232: 48894610 mov qword ptr [rsi + 0x10], rax
140c90236: 483b06 cmp rax, qword ptr [rsi]
140c90239: 750b jne 0x140c90246
140c9023b: 4889f1 mov rcx, rsi
140c9023e: 4c89f2 mov rdx, r14
140c90241: e82af6ecff call 0x140b5f870
140c90246: 4a8d142f lea rdx, [rdi + r13]
140c9024a: 48ffca dec rdx
140c9024d: 4c037e08 add r15, qword ptr [rsi + 8]
140c90251: 43c704af0000803f mov dword ptr [r15 + r13*4], 0x3f800000
140c90259: 488b75b8 mov rsi, qword ptr [rbp - 0x48]
140c9025d: 4883c202 add rdx, 2
140c90261: 488916 mov qword ptr [rsi], rdx
140c90264: 0f2875d0 movaps xmm6, xmmword ptr [rbp - 0x30]
140c90268: 0f287de0 movaps xmm7, xmmword ptr [rbp - 0x20]
140c9026c: 440f2845f0 movaps xmm8, xmmword ptr [rbp - 0x10]
140c90271: 440f284d00 movaps xmm9, xmmword ptr [rbp]
140c90276: 4881c498000000 add rsp, 0x98
140c9027d: 5b pop rbx
140c9027e: 5f pop rdi
140c9027f: 5e pop rsi
140c90280: 415c pop r12
140c90282: 415d pop r13
140c90284: 415e pop r14
140c90286: 415f pop r15
140c90288: 5d pop rbp
140c90289: c3 ret 
140c9028a: 48c744242004000000 mov qword ptr [rsp + 0x20], 4
140c90293: 41b802000000 mov r8d, 2
140c90299: 41b904000000 mov r9d, 4
140c9029f: 4889f1 mov rcx, rsi
140c902a2: e8f9bec601 call 0x1428fc1a0
140c902a7: 488b5610 mov rdx, qword ptr [rsi + 0x10]
140c902ab: e9f2fdffff jmp 0x140c900a2
140c902b0: 48c744242004000000 mov qword ptr [rsp + 0x20], 4
140c902b9: 41b904000000 mov r9d, 4
140c902bf: 4889f1 mov rcx, rsi
140c902c2: 4c89fa mov rdx, r15
140c902c5: 4989d8 mov r8, rbx
140c902c8: e8d3bec601 call 0x1428fc1a0
140c902cd: 4c8b7e10 mov r15, qword ptr [rsi + 0x10]
140c902d1: e9befeffff jmp 0x140c90194
140c902d6: 662e0f1f840000000000 nop word ptr cs:[rax + rax]
140c902e0: 4889542410 mov qword ptr [rsp + 0x10], rdx
140c902e5: 55 push rbp
140c902e6: 4157 push r15
140c902e8: 4156 push r14
140c902ea: 4155 push r13
140c902ec: 4154 push r12
140c902ee: 56 push rsi
140c902ef: 57 push rdi
140c902f0: 53 push rbx
140c902f1: 4883ec68 sub rsp, 0x68
140c902f5: 488daa80000000 lea rbp, [rdx + 0x80]
140c902fc: 440f294c2420 movaps xmmword ptr [rsp + 0x20], xmm9
140c90302: 440f29442430 movaps xmmword ptr [rsp + 0x30], xmm8
140c90308: 0f297c2440 movaps xmmword ptr [rsp + 0x40], xmm7
140c9030d: 0f29742450 movaps xmmword ptr [rsp + 0x50], xmm6
140c90312: 488b45b0 mov rax, qword ptr [rbp - 0x50]
140c90316: 480345c0 add rax, qword ptr [rbp - 0x40]
140c9031a: 488b4db8 mov rcx, qword ptr [rbp - 0x48]
140c9031e: 488901 mov qword ptr [rcx], rax
140c90321: 0f28742450 movaps xmm6, xmmword ptr [rsp + 0x50]
140c90326: 0f287c2440 movaps xmm7, xmmword ptr [rsp + 0x40]
140c9032b: 440f28442430 movaps xmm8, xmmword ptr [rsp + 0x30]
140c90331: 440f284c2420 movaps xmm9, xmmword ptr [rsp + 0x20]
140c90337: 4883c468 add rsp, 0x68
140c9033b: 5b pop rbx
140c9033c: 5f pop rdi
140c9033d: 5e pop rsi
140c9033e: 415c pop r12
140c90340: 415d pop r13
140c90342: 415e pop r14
140c90344: 415f pop r15
140c90346: 5d pop rbp
140c90347: c3 ret 
