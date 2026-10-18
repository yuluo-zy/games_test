140a48880: 56  push rsi
140a48881: 4883ec20  sub rsp, 0x20
140a48885: 4889c8  mov rax, rcx
140a48888: 498d48fc  lea rcx, [r8 - 4]
140a4888c: 4883f909  cmp rcx, 9
140a48890: 0f8718010000  ja 0x140a489ae
140a48896: 4c8d0d2b6f0802  lea r9, [rip + 0x2086f2b]
140a4889d: 49630c89  movsxd rcx, dword ptr [r9 + rcx*4]
140a488a1: 4c01c9  add rcx, r9
140a488a4: ffe1  jmp rcx
140a488a6: 813a466c6167  cmp dword ptr [rdx], 0x67616c46
140a488ac: 0f85fc000000  jne 0x140a489ae
140a488b2: c6400105  mov byte ptr [rax + 1], 5
140a488b6: e9e9000000  jmp 0x140a489a4
140a488bb: 48b9436f747461676557  movabs rcx, 0x5765676174746f43
140a488c5: 48330a  xor rcx, qword ptr [rdx]
140a488c8: 49b9676557696e646f77  movabs r9, 0x776f646e69576567
140a488d2: 4c334a05  xor r9, qword ptr [rdx + 5]
140a488d6: 4909c9  or r9, rcx
140a488d9: 0f85cf000000  jne 0x140a489ae
140a488df: c6400100  mov byte ptr [rax + 1], 0
140a488e3: e9bc000000  jmp 0x140a489a4
140a488e8: b94368696d  mov ecx, 0x6d696843
140a488ed: 330a  xor ecx, dword ptr [rdx]
140a488ef: 41b96d6e6579  mov r9d, 0x79656e6d
140a488f5: 44334a03  xor r9d, dword ptr [rdx + 3]
140a488f9: 4109c9  or r9d, ecx
140a488fc: 0f8498000000  je 0x140a4899a
140a48902: b94c616e74  mov ecx, 0x746e614c
140a48907: 330a  xor ecx, dword ptr [rdx]
140a48909: 41b97465726e  mov r9d, 0x6e726574
140a4890f: 44334a03  xor r9d, dword ptr [rdx + 3]
140a48913: 4109c9  or r9d, ecx
140a48916: 0f8484000000  je 0x140a489a0
140a4891c: b9436c7574  mov ecx, 0x74756c43
140a48921: 330a  xor ecx, dword ptr [rdx]
140a48923: 41b974746572  mov r9d, 0x72657474
140a48929: 44334a03  xor r9d, dword ptr [rdx + 3]
140a4892d: 4109c9  or r9d, ecx
140a48930: 757c  jne 0x140a489ae
140a48932: c6400106  mov byte ptr [rax + 1], 6
140a48936: eb6c  jmp 0x140a489a4
140a48938: 48b9476f746869635769  movabs rcx, 0x6957636968746f47
140a48942: 48330a  xor rcx, qword ptr [rdx]
140a48945: 448b4a08  mov r9d, dword ptr [rdx + 8]
140a48949: 4981f16e646f77  xor r9, 0x776f646e
140a48950: 4909c9  or r9, rcx
140a48953: 7559  jne 0x140a489ae
140a48955: c6400101  mov byte ptr [rax + 1], 1
140a48959: eb49  jmp 0x140a489a4
140a4895b: 48b94172726f77536c69  movabs rcx, 0x696c53776f727241
140a48965: 48330a  xor rcx, qword ptr [rdx]
140a48968: 440fb64a08  movzx r9d, byte ptr [rdx + 8]
140a4896d: 4983f174  xor r9, 0x74
140a48971: 4909c9  or r9, rcx
140a48974: 7538  jne 0x140a489ae
140a48976: c6400102  mov byte ptr [rax + 1], 2
140a4897a: eb28  jmp 0x140a489a4
140a4897c: b953746169  mov ecx, 0x69617453
140a48981: 330a  xor ecx, dword ptr [rdx]
140a48983: 440fb74a04  movzx r9d, word ptr [rdx + 4]
140a48988: 4181f172730000  xor r9d, 0x7372
140a4898f: 4109c9  or r9d, ecx
140a48992: 751a  jne 0x140a489ae
140a48994: c6400107  mov byte ptr [rax + 1], 7
140a48998: eb0a  jmp 0x140a489a4
140a4899a: c6400103  mov byte ptr [rax + 1], 3
140a4899e: eb04  jmp 0x140a489a4
140a489a0: c6400104  mov byte ptr [rax + 1], 4
140a489a4: 31c9  xor ecx, ecx
140a489a6: 8808  mov byte ptr [rax], cl
140a489a8: 4883c420  add rsp, 0x20
140a489ac: 5e  pop rsi
140a489ad: c3  ret 
140a489ae: 4c8d15eb630802  lea r10, [rip + 0x20863eb]
140a489b5: 41b908000000  mov r9d, 8
140a489bb: 4889d1  mov rcx, rdx
140a489be: 4c89c2  mov rdx, r8
140a489c1: 4d89d0  mov r8, r10
140a489c4: 4889c6  mov rsi, rax
140a489c7: e85414eb01  call 0x1428f9e20
140a489cc: 4889c1  mov rcx, rax
140a489cf: 4889f0  mov rax, rsi
140a489d2: 48894e08  mov qword ptr [rsi + 8], rcx
140a489d6: b101  mov cl, 1
140a489d8: ebcc  jmp 0x140a489a6
