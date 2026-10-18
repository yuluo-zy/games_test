14146f541: 48 8b 85 40 0a 00 00        	mov	rax, qword ptr [rbp + 0xa40]
14146f548: 48 8b 95 18 09 00 00        	mov	rdx, qword ptr [rbp + 0x918]
14146f54f: 48 8b 4a 10                 	mov	rcx, qword ptr [rdx + 0x10]
14146f553: 4c 8b 85 70 05 00 00        	mov	r8, qword ptr [rbp + 0x570]
14146f55a: 44 8b 8d c4 08 00 00        	mov	r9d, dword ptr [rbp + 0x8c4]
14146f561: 45 89 08                    	mov	dword ptr [r8], r9d
14146f564: 48 89 4a 58                 	mov	qword ptr [rdx + 0x58], rcx
14146f568: 44 8b 40 1c                 	mov	r8d, dword ptr [rax + 0x1c]
14146f56c: 48 8b 10                    	mov	rdx, qword ptr [rax]
14146f56f: 48 8b 40 10                 	mov	rax, qword ptr [rax + 0x10]
14146f573: 48 89 8a 20 01 00 00        	mov	qword ptr [rdx + 0x120], rcx
14146f57a: 8b 8d c8 01 00 00           	mov	ecx, dword ptr [rbp + 0x1c8]
14146f580: 44 89 00                    	mov	dword ptr [rax], r8d
14146f583: 89 8a 40 01 00 00           	mov	dword ptr [rdx + 0x140], ecx
