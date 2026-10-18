140a4b1f0: 56  push rsi
140a4b1f1: 4883ec20  sub rsp, 0x20
140a4b1f5: 4889c8  mov rax, rcx
140a4b1f8: 4983f804  cmp r8, 4
140a4b1fc: 7422  je 0x140a4b220
140a4b1fe: 4983f807  cmp r8, 7
140a4b202: 7532  jne 0x140a4b236
140a4b204: b954657272  mov ecx, 0x72726554
140a4b209: 330a  xor ecx, dword ptr [rdx]
140a4b20b: 41b97261696e  mov r9d, 0x6e696172
140a4b211: 44334a03  xor r9d, dword ptr [rdx + 3]
140a4b215: 4109c9  or r9d, ecx
140a4b218: 754b  jne 0x140a4b265
140a4b21a: c6400102  mov byte ptr [rax + 1], 2
140a4b21e: eb3b  jmp 0x140a4b25b
140a4b220: 813a57616c6c  cmp dword ptr [rdx], 0x6c6c6157
140a4b226: 742f  je 0x140a4b257
140a4b228: 813a526f6f66  cmp dword ptr [rdx], 0x666f6f52
140a4b22e: 7535  jne 0x140a4b265
140a4b230: c6400101  mov byte ptr [rax + 1], 1
140a4b234: eb25  jmp 0x140a4b25b
140a4b236: 4983f805  cmp r8, 5
140a4b23a: 7529  jne 0x140a4b265
140a4b23c: b953746169  mov ecx, 0x69617453
140a4b241: 330a  xor ecx, dword ptr [rdx]
140a4b243: 440fb64a04  movzx r9d, byte ptr [rdx + 4]
140a4b248: 4183f172  xor r9d, 0x72
140a4b24c: 4109c9  or r9d, ecx
140a4b24f: 7514  jne 0x140a4b265
140a4b251: c6400103  mov byte ptr [rax + 1], 3
140a4b255: eb04  jmp 0x140a4b25b
140a4b257: c6400100  mov byte ptr [rax + 1], 0
140a4b25b: 31c9  xor ecx, ecx
140a4b25d: 8808  mov byte ptr [rax], cl
140a4b25f: 4883c420  add rsp, 0x20
140a4b263: 5e  pop rsi
140a4b264: c3  ret 
140a4b265: 4c8d155c410802  lea r10, [rip + 0x208415c]
140a4b26c: 41b904000000  mov r9d, 4
140a4b272: 4889d1  mov rcx, rdx
140a4b275: 4c89c2  mov rdx, r8
140a4b278: 4d89d0  mov r8, r10
140a4b27b: 4889c6  mov rsi, rax
140a4b27e: e89debea01  call 0x1428f9e20
140a4b283: 4889c1  mov rcx, rax
140a4b286: 4889f0  mov rax, rsi
140a4b289: 48894e08  mov qword ptr [rsi + 8], rcx
140a4b28d: b101  mov cl, 1
140a4b28f: ebcc  jmp 0x140a4b25d
