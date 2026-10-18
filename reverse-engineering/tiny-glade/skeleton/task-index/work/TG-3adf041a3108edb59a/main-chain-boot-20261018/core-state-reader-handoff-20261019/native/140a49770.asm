140a49770: 56  push rsi
140a49771: 4883ec20  sub rsp, 0x20
140a49775: 4889c8  mov rax, rcx
140a49778: 4983f803  cmp r8, 3
140a4977c: 741c  je 0x140a4979a
140a4977e: 4983f804  cmp r8, 4
140a49782: 7534  jne 0x140a497b8
140a49784: 813a4e6f6e65  cmp dword ptr [rdx], 0x656e6f4e
140a4978a: 7450  je 0x140a497dc
140a4978c: 813a48696768  cmp dword ptr [rdx], 0x68676948
140a49792: 7556  jne 0x140a497ea
140a49794: c6400102  mov byte ptr [rax + 1], 2
140a49798: eb46  jmp 0x140a497e0
140a4979a: 0fb70a  movzx ecx, word ptr [rdx]
140a4979d: 81f14c6f0000  xor ecx, 0x6f4c
140a497a3: 440fb64a02  movzx r9d, byte ptr [rdx + 2]
140a497a8: 4183f177  xor r9d, 0x77
140a497ac: 664109c9  or r9w, cx
140a497b0: 7538  jne 0x140a497ea
140a497b2: c6400101  mov byte ptr [rax + 1], 1
140a497b6: eb28  jmp 0x140a497e0
140a497b8: 4983f806  cmp r8, 6
140a497bc: 752c  jne 0x140a497ea
140a497be: b9576f6f64  mov ecx, 0x646f6f57
140a497c3: 330a  xor ecx, dword ptr [rdx]
140a497c5: 440fb74a04  movzx r9d, word ptr [rdx + 4]
140a497ca: 4181f1656e0000  xor r9d, 0x6e65
140a497d1: 4109c9  or r9d, ecx
140a497d4: 7514  jne 0x140a497ea
140a497d6: c6400103  mov byte ptr [rax + 1], 3
140a497da: eb04  jmp 0x140a497e0
140a497dc: c6400100  mov byte ptr [rax + 1], 0
140a497e0: 31c9  xor ecx, ecx
140a497e2: 8808  mov byte ptr [rax], cl
140a497e4: 4883c420  add rsp, 0x20
140a497e8: 5e  pop rsi
140a497e9: c3  ret 
140a497ea: 4c8d151f590802  lea r10, [rip + 0x208591f]
140a497f1: 41b904000000  mov r9d, 4
140a497f7: 4889d1  mov rcx, rdx
140a497fa: 4c89c2  mov rdx, r8
140a497fd: 4d89d0  mov r8, r10
140a49800: 4889c6  mov rsi, rax
140a49803: e81806eb01  call 0x1428f9e20
140a49808: 4889c1  mov rcx, rax
140a4980b: 4889f0  mov rax, rsi
140a4980e: 48894e08  mov qword ptr [rsi + 8], rcx
140a49812: b101  mov cl, 1
140a49814: ebcc  jmp 0x140a497e2
