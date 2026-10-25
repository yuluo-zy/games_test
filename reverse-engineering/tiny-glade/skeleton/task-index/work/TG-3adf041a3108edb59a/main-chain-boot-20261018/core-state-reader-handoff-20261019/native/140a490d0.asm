140a490d0: 55  push rbp
140a490d1: 4157  push r15
140a490d3: 4156  push r14
140a490d5: 4154  push r12
140a490d7: 56  push rsi
140a490d8: 57  push rdi
140a490d9: 53  push rbx
140a490da: 4881ec40010000  sub rsp, 0x140
140a490e1: 488dac2480000000  lea rbp, [rsp + 0x80]
140a490e9: 48c785b8000000feffffff  mov qword ptr [rbp + 0xb8], 0xfffffffffffffffe
140a490f4: 4889ce  mov rsi, rcx
140a490f7: 0fb602  movzx eax, byte ptr [rdx]
140a490fa: 83f803  cmp eax, 3
140a490fd: 8985a0000000  mov dword ptr [rbp + 0xa0], eax
140a49103: 48895578  mov qword ptr [rbp + 0x78], rdx
140a49107: 0f84ed000000  je 0x140a491fa
140a4910d: 83f805  cmp eax, 5
140a49110: 0f85cf010000  jne 0x140a492e5
140a49116: 4c8b7a08  mov r15, qword ptr [rdx + 8]
140a4911a: 4c8b6210  mov r12, qword ptr [rdx + 0x10]
140a4911e: 488b7a18  mov rdi, qword ptr [rdx + 0x18]
140a49122: 31c0  xor eax, eax
140a49124: 4d85ff  test r15, r15
140a49127: 0f95c0  setne al
140a4912a: 4889f9  mov rcx, rdi
140a4912d: 490f44cf  cmove rcx, r15
140a49131: 48894520  mov qword ptr [rbp + 0x20], rax
140a49135: 48c7452800000000  mov qword ptr [rbp + 0x28], 0
140a4913d: 4c897d30  mov qword ptr [rbp + 0x30], r15
140a49141: 4c896538  mov qword ptr [rbp + 0x38], r12
140a49145: 48894540  mov qword ptr [rbp + 0x40], rax
140a49149: 48c7454800000000  mov qword ptr [rbp + 0x48], 0
140a49151: 4c897d50  mov qword ptr [rbp + 0x50], r15
140a49155: 4c896558  mov qword ptr [rbp + 0x58], r12
140a49159: 48894d60  mov qword ptr [rbp + 0x60], rcx
140a4915d: 488d8d80000000  lea rcx, [rbp + 0x80]
140a49164: 488d5520  lea rdx, [rbp + 0x20]
140a49168: e8e3329f00  call 0x14143c450
140a4916d: 488b8580000000  mov rax, qword ptr [rbp + 0x80]
140a49174: 4885c0  test rax, rax
140a49177: 7444  je 0x140a491bd
140a49179: 488b8d90000000  mov rcx, qword ptr [rbp + 0x90]
140a49180: 488d1449  lea rdx, [rcx + rcx*2]
140a49184: 4c8b84d068010000  mov r8, qword ptr [rax + rdx*8 + 0x168]
140a4918c: 4c8b8cd070010000  mov r9, qword ptr [rax + rdx*8 + 0x170]
140a49194: 488b9cd078010000  mov rbx, qword ptr [rax + rdx*8 + 0x178]
140a4919c: 48c1e105  shl rcx, 5
140a491a0: 0f100408  movups xmm0, xmmword ptr [rax + rcx]
140a491a4: 0f104c0810  movups xmm1, xmmword ptr [rax + rcx + 0x10]
140a491a9: 0f2945e0  movaps xmmword ptr [rbp - 0x20], xmm0
140a491ad: 0f294df0  movaps xmmword ptr [rbp - 0x10], xmm1
140a491b1: 4c89c0  mov rax, r8
140a491b4: 48f7d8  neg rax
140a491b7: 0f81b5010000  jno 0x140a49372
140a491bd: c685800000000b  mov byte ptr [rbp + 0x80], 0xb
140a491c4: 488d157d520802  lea rdx, [rip + 0x208527d]
140a491cb: 4c8d0586520802  lea r8, [rip + 0x2085286]
140a491d2: 488d8d80000000  lea rcx, [rbp + 0x80]
140a491d9: e86209eb01  call 0x1428f9b40
140a491de: 48894608  mov qword ptr [rsi + 8], rax
140a491e2: c60601  mov byte ptr [rsi], 1
140a491e5: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a491ec: 488d4d20  lea rcx, [rbp + 0x20]
140a491f0: e8fbebe3ff  call 0x140887df0
140a491f5: e973020000  jmp 0x140a4946d
140a491fa: 4c8b7a08  mov r15, qword ptr [rdx + 8]
140a491fe: 4c8b6210  mov r12, qword ptr [rdx + 0x10]
140a49202: 488b7a18  mov rdi, qword ptr [rdx + 0x18]
140a49206: 41b606  mov r14b, 6
140a49209: 4c89f8  mov rax, r15
140a4920c: 4c89e1  mov rcx, r12
140a4920f: 4889fb  mov rbx, rdi
140a49212: 48894520  mov qword ptr [rbp + 0x20], rax
140a49216: 48894d28  mov qword ptr [rbp + 0x28], rcx
140a4921a: 48895d30  mov qword ptr [rbp + 0x30], rbx
140a4921e: 0f2845c0  movaps xmm0, xmmword ptr [rbp - 0x40]
140a49222: 0f114539  movups xmmword ptr [rbp + 0x39], xmm0
140a49226: 0f1045cf  movups xmm0, xmmword ptr [rbp - 0x31]
140a4922a: 0f114548  movups xmmword ptr [rbp + 0x48], xmm0
140a4922e: 44887538  mov byte ptr [rbp + 0x38], r14b
140a49232: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a49239: 488d4de0  lea rcx, [rbp - 0x20]
140a4923d: 488d5520  lea rdx, [rbp + 0x20]
140a49241: e80a8bb800  call 0x1415d1d50
140a49246: 0fb65de0  movzx ebx, byte ptr [rbp - 0x20]
140a4924a: 488b45e8  mov rax, qword ptr [rbp - 0x18]
140a4924e: 80fb04  cmp bl, 4
140a49251: 7439  je 0x140a4928c
140a49253: 488b4d00  mov rcx, qword ptr [rbp]
140a49257: 48898d98000000  mov qword ptr [rbp + 0x98], rcx
140a4925e: 0f1045f0  movups xmm0, xmmword ptr [rbp - 0x10]
140a49262: 0f118588000000  movups xmmword ptr [rbp + 0x88], xmm0
140a49269: 48898580000000  mov qword ptr [rbp + 0x80], rax
140a49270: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a49277: 488d8d80000000  lea rcx, [rbp + 0x80]
140a4927e: e87ded1300  call 0x140b88000
140a49283: 4885c0  test rax, rax
140a49286: 0f849f010000  je 0x140a4942b
140a4928c: 48894608  mov qword ptr [rsi + 8], rax
140a49290: b001  mov al, 1
140a49292: 8806  mov byte ptr [rsi], al
140a49294: 8b85a0000000  mov eax, dword ptr [rbp + 0xa0]
140a4929a: 83f803  cmp eax, 3
140a4929d: 0f84ca010000  je 0x140a4946d
140a492a3: 83f805  cmp eax, 5
140a492a6: 0f84c1010000  je 0x140a4946d
140a492ac: 4c89bda0000000  mov qword ptr [rbp + 0xa0], r15
140a492b3: 4c896578  mov qword ptr [rbp + 0x78], r12
140a492b7: 4c89e1  mov rcx, r12
140a492ba: 4889fa  mov rdx, rdi
140a492bd: e8aefae3ff  call 0x140888d70
140a492c2: 488b95a0000000  mov rdx, qword ptr [rbp + 0xa0]
140a492c9: 4885d2  test rdx, rdx
140a492cc: 488b4d78  mov rcx, qword ptr [rbp + 0x78]
140a492d0: 0f8497010000  je 0x140a4946d
140a492d6: 48c1e205  shl rdx, 5
140a492da: 41b808000000  mov r8d, 8
140a492e0: e971010000  jmp 0x140a49456
140a492e5: 0f1002  movups xmm0, xmmword ptr [rdx]
140a492e8: 0f104a10  movups xmm1, xmmword ptr [rdx + 0x10]
140a492ec: 0f294d30  movaps xmmword ptr [rbp + 0x30], xmm1
140a492f0: 0f294520  movaps xmmword ptr [rbp + 0x20], xmm0
140a492f4: 488d4de0  lea rcx, [rbp - 0x20]
140a492f8: 488d5520  lea rdx, [rbp + 0x20]
140a492fc: e80f09e801  call 0x1428c9c10
140a49301: 488d1580510802  lea rdx, [rip + 0x2085180]
140a49308: 4c8d0549510802  lea r8, [rip + 0x2085149]
140a4930f: 488d4de0  lea rcx, [rbp - 0x20]
140a49313: e8e806e801  call 0x1428c9a00
140a49318: 48894608  mov qword ptr [rsi + 8], rax
140a4931c: c60601  mov byte ptr [rsi], 1
140a4931f: 0fb64520  movzx eax, byte ptr [rbp + 0x20]
140a49323: 83f803  cmp eax, 3
140a49326: 0f8241010000  jb 0x140a4946d
140a4932c: 0f8411010000  je 0x140a49443
140a49332: 83f804  cmp eax, 4
140a49335: 0f8522010000  jne 0x140a4945d
140a4933b: 488b4d30  mov rcx, qword ptr [rbp + 0x30]
140a4933f: 488b5538  mov rdx, qword ptr [rbp + 0x38]
140a49343: 48898da8000000  mov qword ptr [rbp + 0xa8], rcx
140a4934a: e821fae3ff  call 0x140888d70
140a4934f: 488b5528  mov rdx, qword ptr [rbp + 0x28]
140a49353: 4885d2  test rdx, rdx
140a49356: 0f8411010000  je 0x140a4946d
140a4935c: 48c1e205  shl rdx, 5
140a49360: 41b808000000  mov r8d, 8
140a49366: 488b8da8000000  mov rcx, qword ptr [rbp + 0xa8]
140a4936d: e9e4000000  jmp 0x140a49456
140a49372: 4c894d70  mov qword ptr [rbp + 0x70], r9
140a49376: 4c8985a8000000  mov qword ptr [rbp + 0xa8], r8
140a4937d: 0f2845e0  movaps xmm0, xmmword ptr [rbp - 0x20]
140a49381: 0f284df0  movaps xmm1, xmmword ptr [rbp - 0x10]
140a49385: 0f298d90000000  movaps xmmword ptr [rbp + 0x90], xmm1
140a4938c: 0f298580000000  movaps xmmword ptr [rbp + 0x80], xmm0
140a49393: 488d4da8  lea rcx, [rbp - 0x58]
140a49397: 488d5520  lea rdx, [rbp + 0x20]
140a4939b: e8b0309f00  call 0x14143c450
140a493a0: 488b45a8  mov rax, qword ptr [rbp - 0x58]
140a493a4: 4885c0  test rax, rax
140a493a7: 7445  je 0x140a493ee
140a493a9: 488b4db8  mov rcx, qword ptr [rbp - 0x48]
140a493ad: 488d1449  lea rdx, [rcx + rcx*2]
140a493b1: 4c8b84d078010000  mov r8, qword ptr [rax + rdx*8 + 0x178]
140a493b9: 4c8945f0  mov qword ptr [rbp - 0x10], r8
140a493bd: 0f1084d068010000  movups xmm0, xmmword ptr [rax + rdx*8 + 0x168]
140a493c5: 0f2945e0  movaps xmmword ptr [rbp - 0x20], xmm0
140a493c9: 48c1e105  shl rcx, 5
140a493cd: 0f100408  movups xmm0, xmmword ptr [rax + rcx]
140a493d1: 0f104c0810  movups xmm1, xmmword ptr [rax + rcx + 0x10]
140a493d6: 0f1145f8  movups xmmword ptr [rbp - 8], xmm0
140a493da: 0f114d08  movups xmmword ptr [rbp + 8], xmm1
140a493de: 488b55e0  mov rdx, qword ptr [rbp - 0x20]
140a493e2: 4889d0  mov rax, rdx
140a493e5: 48f7d8  neg rax
140a493e8: 0f8194000000  jno 0x140a49482
140a493ee: 448ab580000000  mov r14b, byte ptr [rbp + 0x80]
140a493f5: 0f108581000000  movups xmm0, xmmword ptr [rbp + 0x81]
140a493fc: 0f2945c0  movaps xmmword ptr [rbp - 0x40], xmm0
140a49400: 0f108590000000  movups xmm0, xmmword ptr [rbp + 0x90]
140a49407: 0f1145cf  movups xmmword ptr [rbp - 0x31], xmm0
140a4940b: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a49412: 488d4d20  lea rcx, [rbp + 0x20]
140a49416: e8d5e9e3ff  call 0x140887df0
140a4941b: 488b85a8000000  mov rax, qword ptr [rbp + 0xa8]
140a49422: 488b4d70  mov rcx, qword ptr [rbp + 0x70]
140a49426: e9e7fdffff  jmp 0x140a49212
140a4942b: 885e01  mov byte ptr [rsi + 1], bl
140a4942e: 31c0  xor eax, eax
140a49430: 8806  mov byte ptr [rsi], al
140a49432: 8b85a0000000  mov eax, dword ptr [rbp + 0xa0]
140a49438: 83f803  cmp eax, 3
140a4943b: 0f8562feffff  jne 0x140a492a3
140a49441: eb2a  jmp 0x140a4946d
140a49443: 488b5528  mov rdx, qword ptr [rbp + 0x28]
140a49447: 4885d2  test rdx, rdx
140a4944a: 7421  je 0x140a4946d
140a4944c: 488b4d30  mov rcx, qword ptr [rbp + 0x30]
140a49450: 41b801000000  mov r8d, 1
140a49456: e8c5a7bcff  call 0x140613c20
140a4945b: eb10  jmp 0x140a4946d
140a4945d: 488d4d28  lea rcx, [rbp + 0x28]
140a49461: c685b700000000  mov byte ptr [rbp + 0xb7], 0
140a49468: e8d3fae4ff  call 0x140898f40
140a4946d: 4889f0  mov rax, rsi
140a49470: 4881c440010000  add rsp, 0x140
140a49477: 5b  pop rbx
140a49478: 5f  pop rdi
140a49479: 5e  pop rsi
140a4947a: 415c  pop r12
140a4947c: 415e  pop r14
140a4947e: 415f  pop r15
140a49480: 5d  pop rbp
140a49481: c3  ret 
140a49482: 730f  jae 0x140a49493
140a49484: 488b4de8  mov rcx, qword ptr [rbp - 0x18]
140a49488: 41b801000000  mov r8d, 1
140a4948e: e88da7bcff  call 0x140613c20
140a49493: 0fb645f8  movzx eax, byte ptr [rbp - 8]
140a49497: 83f803  cmp eax, 3
140a4949a: 7231  jb 0x140a494cd
140a4949c: 7410  je 0x140a494ae
140a4949e: 488d4d00  lea rcx, [rbp]
140a494a2: 83f804  cmp eax, 4
140a494a5: 7521  jne 0x140a494c8
140a494a7: e8e4fee3ff  call 0x140889390
140a494ac: eb1f  jmp 0x140a494cd
140a494ae: 488b5500  mov rdx, qword ptr [rbp]
140a494b2: 4885d2  test rdx, rdx
140a494b5: 7416  je 0x140a494cd
140a494b7: 488b4d08  mov rcx, qword ptr [rbp + 8]
140a494bb: 41b801000000  mov r8d, 1
140a494c1: e85aa7bcff  call 0x140613c20
140a494c6: eb05  jmp 0x140a494cd
140a494c8: e873fae4ff  call 0x140898f40
140a494cd: c645e00b  mov byte ptr [rbp - 0x20], 0xb
140a494d1: 488d15704f0802  lea rdx, [rip + 0x2084f70]
140a494d8: 4c8d05794f0802  lea r8, [rip + 0x2084f79]
140a494df: 488d4de0  lea rcx, [rbp - 0x20]
140a494e3: e85806eb01  call 0x1428f9b40
140a494e8: 48894608  mov qword ptr [rsi + 8], rax
140a494ec: c60601  mov byte ptr [rsi], 1
140a494ef: 0fb68580000000  movzx eax, byte ptr [rbp + 0x80]
140a494f6: 83f803  cmp eax, 3
140a494f9: 723a  jb 0x140a49535
140a494fb: 7413  je 0x140a49510
140a494fd: 488d8d88000000  lea rcx, [rbp + 0x88]
140a49504: 83f804  cmp eax, 4
140a49507: 7527  jne 0x140a49530
140a49509: e882fee3ff  call 0x140889390
140a4950e: eb25  jmp 0x140a49535
140a49510: 488b9588000000  mov rdx, qword ptr [rbp + 0x88]
140a49517: 4885d2  test rdx, rdx
140a4951a: 7419  je 0x140a49535
140a4951c: 488b8d90000000  mov rcx, qword ptr [rbp + 0x90]
140a49523: 41b801000000  mov r8d, 1
140a49529: e8f2a6bcff  call 0x140613c20
140a4952e: eb05  jmp 0x140a49535
140a49530: e80bfae4ff  call 0x140898f40
140a49535: 488b95a8000000  mov rdx, qword ptr [rbp + 0xa8]
140a4953c: 4885d2  test rdx, rdx
140a4953f: 0f84a0fcffff  je 0x140a491e5
140a49545: 41b801000000  mov r8d, 1
140a4954b: 488b4d70  mov rcx, qword ptr [rbp + 0x70]
140a4954f: e8cca6bcff  call 0x140613c20
140a49554: e98cfcffff  jmp 0x140a491e5
140a49559: 0f1f8000000000  nop dword ptr [rax]
140a49560: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a49565: 55  push rbp
140a49566: 4157  push r15
140a49568: 4156  push r14
140a4956a: 4154  push r12
140a4956c: 56  push rsi
140a4956d: 57  push rdi
140a4956e: 53  push rbx
140a4956f: 4883ec20  sub rsp, 0x20
140a49573: 488daa80000000  lea rbp, [rdx + 0x80]
140a4957a: 488d4d20  lea rcx, [rbp + 0x20]
140a4957e: e86de8e3ff  call 0x140887df0
140a49583: c685b700000001  mov byte ptr [rbp + 0xb7], 1
140a4958a: 4883c420  add rsp, 0x20
140a4958e: 5b  pop rbx
140a4958f: 5f  pop rdi
140a49590: 5e  pop rsi
140a49591: 415c  pop r12
140a49593: 415e  pop r14
140a49595: 415f  pop r15
140a49597: 5d  pop rbp
140a49598: c3  ret 
140a49599: 0f1f8000000000  nop dword ptr [rax]
140a495a0: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a495a5: 55  push rbp
140a495a6: 4157  push r15
140a495a8: 4156  push r14
140a495aa: 4154  push r12
140a495ac: 56  push rsi
140a495ad: 57  push rdi
140a495ae: 53  push rbx
140a495af: 4883ec20  sub rsp, 0x20
140a495b3: 488daa80000000  lea rbp, [rdx + 0x80]
140a495ba: 4883bda800000000  cmp qword ptr [rbp + 0xa8], 0
140a495c2: 7416  je 0x140a495da
140a495c4: 41b801000000  mov r8d, 1
140a495ca: 488b4d70  mov rcx, qword ptr [rbp + 0x70]
140a495ce: 488b95a8000000  mov rdx, qword ptr [rbp + 0xa8]
140a495d5: e846a6bcff  call 0x140613c20
140a495da: 90  nop 
140a495db: 4883c420  add rsp, 0x20
140a495df: 5b  pop rbx
140a495e0: 5f  pop rdi
140a495e1: 5e  pop rsi
140a495e2: 415c  pop r12
140a495e4: 415e  pop r14
140a495e6: 415f  pop r15
140a495e8: 5d  pop rbp
140a495e9: c3  ret 
140a495ea: 660f1f440000  nop word ptr [rax + rax]
140a495f0: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a495f5: 55  push rbp
140a495f6: 4157  push r15
140a495f8: 4156  push r14
140a495fa: 4154  push r12
140a495fc: 56  push rsi
140a495fd: 57  push rdi
140a495fe: 53  push rbx
140a495ff: 4883ec20  sub rsp, 0x20
140a49603: 488daa80000000  lea rbp, [rdx + 0x80]
140a4960a: 4883bda000000000  cmp qword ptr [rbp + 0xa0], 0
140a49612: 741a  je 0x140a4962e
140a49614: 488b95a0000000  mov rdx, qword ptr [rbp + 0xa0]
140a4961b: 48c1e205  shl rdx, 5
140a4961f: 41b808000000  mov r8d, 8
140a49625: 488b4d78  mov rcx, qword ptr [rbp + 0x78]
140a49629: e8f2a5bcff  call 0x140613c20
140a4962e: 90  nop 
140a4962f: 4883c420  add rsp, 0x20
140a49633: 5b  pop rbx
140a49634: 5f  pop rdi
140a49635: 5e  pop rsi
140a49636: 415c  pop r12
140a49638: 415e  pop r14
140a4963a: 415f  pop r15
140a4963c: 5d  pop rbp
140a4963d: c3  ret 
140a4963e: 6690  nop 
140a49640: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a49645: 55  push rbp
140a49646: 4157  push r15
140a49648: 4156  push r14
140a4964a: 4154  push r12
140a4964c: 56  push rsi
140a4964d: 57  push rdi
140a4964e: 53  push rbx
140a4964f: 4883ec20  sub rsp, 0x20
140a49653: 488daa80000000  lea rbp, [rdx + 0x80]
140a4965a: 488d4d20  lea rcx, [rbp + 0x20]
140a4965e: e82d44e1ff  call 0x14085da90
140a49663: c685b700000000  mov byte ptr [rbp + 0xb7], 0
140a4966a: 4883c420  add rsp, 0x20
140a4966e: 5b  pop rbx
140a4966f: 5f  pop rdi
140a49670: 5e  pop rsi
140a49671: 415c  pop r12
140a49673: 415e  pop r14
140a49675: 415f  pop r15
140a49677: 5d  pop rbp
140a49678: c3  ret 
140a49679: 0f1f8000000000  nop dword ptr [rax]
140a49680: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a49685: 55  push rbp
140a49686: 4157  push r15
140a49688: 4156  push r14
140a4968a: 4154  push r12
140a4968c: 56  push rsi
140a4968d: 57  push rdi
140a4968e: 53  push rbx
140a4968f: 4883ec20  sub rsp, 0x20
140a49693: 488daa80000000  lea rbp, [rdx + 0x80]
140a4969a: 488b5528  mov rdx, qword ptr [rbp + 0x28]
140a4969e: 4885d2  test rdx, rdx
140a496a1: 7416  je 0x140a496b9
140a496a3: 48c1e205  shl rdx, 5
140a496a7: 41b808000000  mov r8d, 8
140a496ad: 488b8da8000000  mov rcx, qword ptr [rbp + 0xa8]
140a496b4: e867a5bcff  call 0x140613c20
140a496b9: c685b700000000  mov byte ptr [rbp + 0xb7], 0
140a496c0: 4883c420  add rsp, 0x20
140a496c4: 5b  pop rbx
140a496c5: 5f  pop rdi
140a496c6: 5e  pop rsi
140a496c7: 415c  pop r12
140a496c9: 415e  pop r14
140a496cb: 415f  pop r15
140a496cd: 5d  pop rbp
140a496ce: c3  ret 
140a496cf: 90  nop 
140a496d0: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a496d5: 55  push rbp
140a496d6: 4157  push r15
140a496d8: 4156  push r14
140a496da: 4154  push r12
140a496dc: 56  push rsi
140a496dd: 57  push rdi
140a496de: 53  push rbx
140a496df: 4883ec20  sub rsp, 0x20
140a496e3: 488daa80000000  lea rbp, [rdx + 0x80]
140a496ea: 80bdb700000000  cmp byte ptr [rbp + 0xb7], 0
140a496f1: 7415  je 0x140a49708
140a496f3: 80bda000000003  cmp byte ptr [rbp + 0xa0], 3
140a496fa: 740c  je 0x140a49708
140a496fc: 0fb685a0000000  movzx eax, byte ptr [rbp + 0xa0]
140a49703: 83f805  cmp eax, 5
140a49706: 750f  jne 0x140a49717
140a49708: 4883c420  add rsp, 0x20
140a4970c: 5b  pop rbx
140a4970d: 5f  pop rdi
140a4970e: 5e  pop rsi
140a4970f: 415c  pop r12
140a49711: 415e  pop r14
140a49713: 415f  pop r15
140a49715: 5d  pop rbp
140a49716: c3  ret 
140a49717: 488b4d78  mov rcx, qword ptr [rbp + 0x78]
140a4971b: e87043e1ff  call 0x14085da90
140a49720: ebe6  jmp 0x140a49708
140a49722: 66666666662e0f1f840000000000  nop word ptr cs:[rax + rax]
140a49730: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a49735: 55  push rbp
140a49736: 4157  push r15
140a49738: 4156  push r14
140a4973a: 4154  push r12
140a4973c: 56  push rsi
140a4973d: 57  push rdi
140a4973e: 53  push rbx
140a4973f: 4883ec20  sub rsp, 0x20
140a49743: 488daa80000000  lea rbp, [rdx + 0x80]
140a4974a: 488d8d80000000  lea rcx, [rbp + 0x80]
140a49751: e83a43e1ff  call 0x14085da90
140a49756: 90  nop 
140a49757: 4883c420  add rsp, 0x20
140a4975b: 5b  pop rbx
140a4975c: 5f  pop rdi
140a4975d: 5e  pop rsi
140a4975e: 415c  pop r12
140a49760: 415e  pop r14
140a49762: 415f  pop r15
140a49764: 5d  pop rbp
140a49765: c3  ret 
