0x1417c3650 4889c8 mov rax, rcx
0x1417c3653 4983c0f9 add r8, -7
0x1417c3657 4983f808 cmp r8, 8
0x1417c365b 0f879a000000 ja 0x1417c36fb
0x1417c3661 488d0db4ee5101 lea rcx, [rip + 0x151eeb4]
0x1417c3668 4e630481 movsxd r8, dword ptr [rcx + r8*4]
0x1417c366c 4901c8 add r8, rcx
0x1417c366f 41ffe0 jmp r8
0x1417c3672 b963656c6c mov ecx, 0x6c6c6563
0x1417c3677 330a xor ecx, dword ptr [rdx]
0x1417c3679 41b86c5f6964 mov r8d, 0x64695f6c
0x1417c367f 44334203 xor r8d, dword ptr [rdx + 3]
0x1417c3683 4109c8 or r8d, ecx
0x1417c3686 7573 jne 0x1417c36fb
0x1417c3688 c6400102 mov byte ptr [rax + 1], 2
0x1417c368c c60000 mov byte ptr [rax], 0
0x1417c368f c3 ret 
0x1417c3690 49b869735f636f726e65 movabs r8, 0x656e726f635f7369
0x1417c369a 4c3302 xor r8, qword ptr [rdx]
0x1417c369d 0fb65208 movzx edx, byte ptr [rdx + 8]
0x1417c36a1 4883f272 xor rdx, 0x72
0x1417c36a5 488d4801 lea rcx, [rax + 1]
0x1417c36a9 4c09c2 or rdx, r8
0x1417c36ac 7458 je 0x1417c3706
0x1417c36ae c60105 mov byte ptr [rcx], 5
0x1417c36b1 c60000 mov byte ptr [rax], 0
0x1417c36b4 c3 ret 
0x1417c36b5 48b9706f736974696f6e movabs rcx, 0x6e6f697469736f70
0x1417c36bf 48390a cmp qword ptr [rdx], rcx
0x1417c36c2 7449 je 0x1417c370d
0x1417c36c4 48b963656c6c5f626279 movabs rcx, 0x7962625f6c6c6563
0x1417c36ce 48390a cmp qword ptr [rdx], rcx
0x1417c36d1 7528 jne 0x1417c36fb
0x1417c36d3 c6400101 mov byte ptr [rax + 1], 1
0x1417c36d7 c60000 mov byte ptr [rax], 0
0x1417c36da c3 ret 
0x1417c36db 48b96469725f746f5f63 movabs rcx, 0x635f6f745f726964
0x1417c36e5 48330a xor rcx, qword ptr [rdx]
0x1417c36e8 49b863656e74726f6964 movabs r8, 0x64696f72746e6563
0x1417c36f2 4c334207 xor r8, qword ptr [rdx + 7]
0x1417c36f6 4909c8 or r8, rcx
0x1417c36f9 741a je 0x1417c3715
0x1417c36fb 488d4801 lea rcx, [rax + 1]
0x1417c36ff c60105 mov byte ptr [rcx], 5
0x1417c3702 c60000 mov byte ptr [rax], 0
0x1417c3705 c3 ret 
0x1417c3706 c60104 mov byte ptr [rcx], 4
0x1417c3709 c60000 mov byte ptr [rax], 0
0x1417c370c c3 ret 
0x1417c370d c6400100 mov byte ptr [rax + 1], 0
0x1417c3711 c60000 mov byte ptr [rax], 0
0x1417c3714 c3 ret 
0x1417c3715 c6400103 mov byte ptr [rax + 1], 3
0x1417c3719 c60000 mov byte ptr [rax], 0
0x1417c371c c3 ret 
