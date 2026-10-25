140a3e7a0: 55  push rbp
140a3e7a1: 4883ec30  sub rsp, 0x30
140a3e7a5: 488d6c2430  lea rbp, [rsp + 0x30]
140a3e7aa: 48c745f8feffffff  mov qword ptr [rbp - 8], 0xfffffffffffffffe
140a3e7b2: 48894df0  mov qword ptr [rbp - 0x10], rcx
140a3e7b6: e8851ae5ff  call 0x140890240
140a3e7bb: ba98010000  mov edx, 0x198
140a3e7c0: 41b808000000  mov r8d, 8
140a3e7c6: 488b4df0  mov rcx, qword ptr [rbp - 0x10]
140a3e7ca: 4883c430  add rsp, 0x30
140a3e7ce: 5d  pop rbp
140a3e7cf: e94c54bdff  jmp 0x140613c20
140a3e7d4: 6666662e0f1f840000000000  nop word ptr cs:[rax + rax]
140a3e7e0: 4889542410  mov qword ptr [rsp + 0x10], rdx
140a3e7e5: 55  push rbp
140a3e7e6: 4883ec20  sub rsp, 0x20
140a3e7ea: 488d6a30  lea rbp, [rdx + 0x30]
140a3e7ee: ba98010000  mov edx, 0x198
140a3e7f3: 41b808000000  mov r8d, 8
140a3e7f9: 488b4df0  mov rcx, qword ptr [rbp - 0x10]
140a3e7fd: e81e54bdff  call 0x140613c20
140a3e802: 90  nop
140a3e803: 4883c420  add rsp, 0x20
140a3e807: 5d  pop rbp
140a3e808: c3  ret
