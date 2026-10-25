140a4a0d0: 56  push rsi
140a4a0d1: 4883ec40  sub rsp, 0x40
140a4a0d5: 0fb64222  movzx eax, byte ptr [rdx + 0x22]
140a4a0d9: 448d40fe  lea r8d, [rax - 2]
140a4a0dd: 48ffc8  dec rax
140a4a0e0: 4531c9  xor r9d, r9d
140a4a0e3: 4180f803  cmp r8b, 3
140a4a0e7: 4c0f42c8  cmovb r9, rax
140a4a0eb: 4889ce  mov rsi, rcx
140a4a0ee: 488d0537570802  lea rax, [rip + 0x2085737]
140a4a0f5: 4a630c88  movsxd rcx, dword ptr [rax + r9*4]
140a4a0f9: 4801c1  add rcx, rax
140a4a0fc: ffe1  jmp rcx
140a4a0fe: 4889542430  mov qword ptr [rsp + 0x30], rdx
140a4a103: 488d0536520802  lea rax, [rip + 0x2085236]
140a4a10a: 4889442420  mov qword ptr [rsp + 0x20], rax
140a4a10f: 48c744242804000000  mov qword ptr [rsp + 0x28], 4
140a4a118: 488d1599520802  lea rdx, [rip + 0x2085299]
140a4a11f: 41b80c000000  mov r8d, 0xc
140a4a125: 4889f1  mov rcx, rsi
140a4a128: 4531c9  xor r9d, r9d
140a4a12b: e8a074b400  call 0x1415915d0
140a4a130: e9a3000000  jmp 0x140a4a1d8
140a4a135: 4889542430  mov qword ptr [rsp + 0x30], rdx
140a4a13a: 488d054f520802  lea rax, [rip + 0x208524f]
140a4a141: 4889442420  mov qword ptr [rsp + 0x20], rax
140a4a146: 48c744242807000000  mov qword ptr [rsp + 0x28], 7
140a4a14f: 488d1562520802  lea rdx, [rip + 0x2085262]
140a4a156: 41b80c000000  mov r8d, 0xc
140a4a15c: 4889f1  mov rcx, rsi
140a4a15f: 41b902000000  mov r9d, 2
140a4a165: e8666fb400  call 0x1415910d0
140a4a16a: eb6c  jmp 0x140a4a1d8
140a4a16c: 4889542430  mov qword ptr [rsp + 0x30], rdx
140a4a171: 488d05a0510802  lea rax, [rip + 0x20851a0]
140a4a178: 4889442420  mov qword ptr [rsp + 0x20], rax
140a4a17d: 48c744242805000000  mov qword ptr [rsp + 0x28], 5
140a4a186: 488d152b520802  lea rdx, [rip + 0x208522b]
140a4a18d: 41b80c000000  mov r8d, 0xc
140a4a193: 4889f1  mov rcx, rsi
140a4a196: 41b903000000  mov r9d, 3
140a4a19c: e8af76b400  call 0x141591850
140a4a1a1: eb35  jmp 0x140a4a1d8
140a4a1a3: 4889542430  mov qword ptr [rsp + 0x30], rdx
140a4a1a8: 488d05b9510802  lea rax, [rip + 0x20851b9]
140a4a1af: 4889442420  mov qword ptr [rsp + 0x20], rax
140a4a1b4: 48c744242804000000  mov qword ptr [rsp + 0x28], 4
140a4a1bd: 488d15f4510802  lea rdx, [rip + 0x20851f4]
140a4a1c4: 41b80c000000  mov r8d, 0xc
140a4a1ca: 4889f1  mov rcx, rsi
140a4a1cd: 41b901000000  mov r9d, 1
140a4a1d3: e8587db400  call 0x141591f30
140a4a1d8: 4889f0  mov rax, rsi
140a4a1db: 4883c440  add rsp, 0x40
140a4a1df: 5e  pop rsi
140a4a1e0: c3  ret
