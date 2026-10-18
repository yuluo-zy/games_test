140a48fc0: 56  push rsi
140a48fc1: 57  push rdi
140a48fc2: 4883ec28  sub rsp, 0x28
140a48fc6: 4889ce  mov rsi, rcx
140a48fc9: 0fb602  movzx eax, byte ptr [rdx]
140a48fcc: 0fb60d36bcbe02  movzx ecx, byte ptr [rip + 0x2bebc36]
140a48fd3: 488d0d16680802  lea rcx, [rip + 0x2086816]
140a48fda: 48630481  movsxd rax, dword ptr [rcx + rax*4]
140a48fde: 4801c8  add rax, rcx
140a48fe1: ffe0  jmp rax
140a48fe3: bf04000000  mov edi, 4
140a48fe8: b904000000  mov ecx, 4
140a48fed: ba01000000  mov edx, 1
140a48ff2: e819acbcff  call 0x140613c10
140a48ff7: 4885c0  test rax, rax
140a48ffa: 0f848b000000  je 0x140a4908b
140a49000: c7004e6f6e65  mov dword ptr [rax], 0x656e6f4e
140a49006: eb6a  jmp 0x140a49072
140a49008: bf04000000  mov edi, 4
140a4900d: b904000000  mov ecx, 4
140a49012: ba01000000  mov edx, 1
140a49017: e8f4abbcff  call 0x140613c10
140a4901c: 4885c0  test rax, rax
140a4901f: 746a  je 0x140a4908b
140a49021: c70048696768  mov dword ptr [rax], 0x68676948
140a49027: eb49  jmp 0x140a49072
140a49029: bf06000000  mov edi, 6
140a4902e: b906000000  mov ecx, 6
140a49033: ba01000000  mov edx, 1
140a49038: e8d3abbcff  call 0x140613c10
140a4903d: 4885c0  test rax, rax
140a49040: 745f  je 0x140a490a1
140a49042: 66c74004656e  mov word ptr [rax + 4], 0x6e65
140a49048: c700576f6f64  mov dword ptr [rax], 0x646f6f57
140a4904e: eb22  jmp 0x140a49072
140a49050: bf03000000  mov edi, 3
140a49055: b903000000  mov ecx, 3
140a4905a: ba01000000  mov edx, 1
140a4905f: e8acabbcff  call 0x140613c10
140a49064: 4885c0  test rax, rax
140a49067: 744e  je 0x140a490b7
140a49069: c6400277  mov byte ptr [rax + 2], 0x77
140a4906d: 66c7004c6f  mov word ptr [rax], 0x6f4c
140a49072: c60603  mov byte ptr [rsi], 3
140a49075: 48897e08  mov qword ptr [rsi + 8], rdi
140a49079: 48894610  mov qword ptr [rsi + 0x10], rax
140a4907d: 48897e18  mov qword ptr [rsi + 0x18], rdi
140a49081: 4889f0  mov rax, rsi
140a49084: 4883c428  add rsp, 0x28
140a49088: 5f  pop rdi
140a49089: 5e  pop rsi
140a4908a: c3  ret
140a4908b: 4c8d05f6550802  lea r8, [rip + 0x20855f6]
140a49092: b901000000  mov ecx, 1
140a49097: ba04000000  mov edx, 4
140a4909c: e842ffe801  call 0x1428d8fe3
140a490a1: 4c8d05e0550802  lea r8, [rip + 0x20855e0]
140a490a8: b901000000  mov ecx, 1
140a490ad: ba06000000  mov edx, 6
140a490b2: e82cffe801  call 0x1428d8fe3
140a490b7: 4c8d05ca550802  lea r8, [rip + 0x20855ca]
140a490be: b901000000  mov ecx, 1
140a490c3: ba03000000  mov edx, 3
140a490c8: e816ffe801  call 0x1428d8fe3
140a490cd: cc  int3
