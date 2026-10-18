14014aec0: 56                           push rsi
14014aec1: 4883ec20                     sub rsp, 0x20
14014aec5: 4889ce                       mov rsi, rcx
14014aec8: 488b01                       mov rax, qword ptr [rcx]
14014aecb: 4885c0                       test rax, rax
14014aece: 7413                         je 0x14014aee3
14014aed0: 488b4e08                     mov rcx, qword ptr [rsi + 8]
14014aed4: 486bd078                     imul rdx, rax, 0x78
14014aed8: 41b808000000                 mov r8d, 8
14014aede: e83d8d4c00                   call 0x140613c20
14014aee3: 488b4620                     mov rax, qword ptr [rsi + 0x20]
14014aee7: 4885c0                       test rax, rax
14014aeea: 7418                         je 0x14014af04
14014aeec: 488b4e28                     mov rcx, qword ptr [rsi + 0x28]
14014aef0: 486bd078                     imul rdx, rax, 0x78
14014aef4: 41b808000000                 mov r8d, 8
14014aefa: 4883c420                     add rsp, 0x20
14014aefe: 5e                           pop rsi
14014aeff: e91c8d4c00                   jmp 0x140613c20
14014af04: 90                           nop 
14014af05: 4883c420                     add rsp, 0x20
14014af09: 5e                           pop rsi
14014af0a: c3                           ret 
