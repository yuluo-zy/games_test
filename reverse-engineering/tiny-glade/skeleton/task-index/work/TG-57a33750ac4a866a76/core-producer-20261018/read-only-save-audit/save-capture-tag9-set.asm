140a40124: c6855703000001  mov byte ptr [rbp + 0x357], 1
140a4012b: 488d8d50010000  lea rcx, [rbp + 0x150]
140a40132: e8f902d600  call 0x1417a0430
140a40137: 4889c6  mov rsi, rax
140a4013a: 0fb605c84abf02  movzx eax, byte ptr [rip + 0x2bf4ac8]
140a40141: b918000000  mov ecx, 0x18
140a40146: ba08000000  mov edx, 8
140a4014b: e8c03abdff  call 0x140613c10
140a40150: 4885c0  test rax, rax
140a40153: 4889b590020000  mov qword ptr [rbp + 0x290], rsi
140a4015a: 488985a8020000  mov qword ptr [rbp + 0x2a8], rax
140a40161: 0f84af220000  je 0x140a42416
140a40167: 48c70001000000  mov qword ptr [rax], 1
140a4016e: 48c7400801000000  mov qword ptr [rax + 8], 1
140a40176: 48c7401000000000  mov qword ptr [rax + 0x10], 0
140a4017e: 48898538010000  mov qword ptr [rbp + 0x138], rax
140a40185: f048ff00  lock inc qword ptr [rax]
140a40189: 0f8efb220000  jle 0x140a4248a
140a4018f: 4889b558010000  mov qword ptr [rbp + 0x158], rsi
140a40196: 48898560010000  mov qword ptr [rbp + 0x160], rax
140a4019d: 48c7855001000009000000  mov qword ptr [rbp + 0x150], 9
140a401a8: c6855603000001  mov byte ptr [rbp + 0x356], 1
140a401af: 488d8de0020000  lea rcx, [rbp + 0x2e0]
140a401b6: 4c8d8550010000  lea r8, [rbp + 0x150]
140a401bd: 4889fa  mov rdx, rdi
140a401c0: e8bba0e9ff  call 0x1408da280
140a401c5: 83bde00200000b  cmp dword ptr [rbp + 0x2e0], 0xb
140a401cc: 0f8555220000  jne 0x140a42427
140a401d2: c6855603000000  mov byte ptr [rbp + 0x356], 0
140a401d9: 4889d9  mov rcx, rbx
140a401dc: 488b95a8020000  mov rdx, qword ptr [rbp + 0x2a8]
140a401e3: e878450400  call 0x140a84760
