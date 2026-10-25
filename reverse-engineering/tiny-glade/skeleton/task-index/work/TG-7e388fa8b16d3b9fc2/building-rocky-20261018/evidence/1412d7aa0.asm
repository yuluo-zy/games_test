0x1412d7aa0 56 push rsi
0x1412d7aa1 4883ec20 sub rsp, 0x20
0x1412d7aa5 49830008 add qword ptr [r8], 8
0x1412d7aa9 4885d2 test rdx, rdx
0x1412d7aac 7428 je 0x1412d7ad6
0x1412d7aae 4889d6 mov rsi, rdx
0x1412d7ab1 488b09 mov rcx, qword ptr [rcx]
0x1412d7ab4 e8c76ed800 call 0x14205e980
0x1412d7ab9 4889f1 mov rcx, rsi
0x1412d7abc e8ef06a2ff call 0x140cf81b0
0x1412d7ac1 4889f1 mov rcx, rsi
0x1412d7ac4 e88707a2ff call 0x140cf8250
0x1412d7ac9 4889f1 mov rcx, rsi
0x1412d7acc 4883c420 add rsp, 0x20
0x1412d7ad0 5e pop rsi
0x1412d7ad1 e92a07a2ff jmp 0x140cf8200
0x1412d7ad6 90 nop 
0x1412d7ad7 4883c420 add rsp, 0x20
0x1412d7adb 5e pop rsi
0x1412d7adc c3 ret 
