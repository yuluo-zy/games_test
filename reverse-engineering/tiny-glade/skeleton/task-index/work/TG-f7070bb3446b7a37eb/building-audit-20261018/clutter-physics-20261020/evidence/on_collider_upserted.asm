0x1408ce460 4883ec38 sub rsp, 0x38
0x1408ce464 4881c1b03c0000 add rcx, 0x3cb0
0x1408ce46b 8b4208 mov eax, dword ptr [rdx + 8]
0x1408ce46e 89442434 mov dword ptr [rsp + 0x34], eax
0x1408ce472 488b02 mov rax, qword ptr [rdx]
0x1408ce475 488944242c mov qword ptr [rsp + 0x2c], rax
0x1408ce47a c744242800000000 mov dword ptr [rsp + 0x28], 0
0x1408ce482 488d542428 lea rdx, [rsp + 0x28]
0x1408ce487 e87457b800 call 0x141453c00
0x1408ce48c 90 nop
0x1408ce48d 4883c438 add rsp, 0x38
0x1408ce491 c3 ret
