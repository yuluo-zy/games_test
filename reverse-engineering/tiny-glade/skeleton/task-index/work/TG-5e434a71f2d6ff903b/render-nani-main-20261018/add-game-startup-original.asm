
ljxsj_92385/Tiny Glade/tiny-glade.rne:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
140941110: 56                          	push	rsi
140941111: 57                          	push	rdi
140941112: 53                          	push	rbx
140941113: 48 81 ec 00 01 00 00        	sub	rsp, 0x100
14094111a: 48 89 ce                    	mov	rsi, rcx
14094111d: 48 8d 7c 24 20              	lea	rdi, [rsp + 0x20]
140941122: 48 89 f9                    	mov	rcx, rdi
140941125: e8 e6 5f 15 00              	call	0x140a97110 <.text+0xa96110>
14094112a: 48 89 f1                    	mov	rcx, rsi
14094112d: 48 89 fa                    	mov	rdx, rdi
140941130: e8 eb 64 22 00              	call	0x140b67620 <.text+0xb66620>
140941135: 48 8d 7c 24 20              	lea	rdi, [rsp + 0x20]
14094113a: 48 89 f9                    	mov	rcx, rdi
14094113d: e8 de f8 df 00              	call	0x141740a20 <.text+0x173fa20>
140941142: 48 8d 9c 24 90 00 00 00     	lea	rbx, [rsp + 0x90]
14094114a: 48 89 d9                    	mov	rcx, rbx
14094114d: 48 89 fa                    	mov	rdx, rdi
140941150: e8 1b 17 b1 00              	call	0x141452870 <.text+0x1451870>
140941155: 48 89 f1                    	mov	rcx, rsi
140941158: b2 01                       	mov	dl, 0x1
14094115a: 49 89 d8                    	mov	r8, rbx
14094115d: e8 ce 56 22 00              	call	0x140b66830 <.text+0xb65830>
140941162: 48 8d 7c 24 20              	lea	rdi, [rsp + 0x20]
140941167: 48 89 f9                    	mov	rcx, rdi
14094116a: e8 e1 e3 1f 00              	call	0x140b3f550 <.text+0xb3e550>
14094116f: 48 89 f1                    	mov	rcx, rsi
140941172: b2 01                       	mov	dl, 0x1
140941174: 49 89 f8                    	mov	r8, rdi
140941177: e8 04 53 22 00              	call	0x140b66480 <.text+0xb65480>
14094117c: 48 89 f1                    	mov	rcx, rsi
14094117f: e8 cc 69 22 00              	call	0x140b67b50 <.text+0xb66b50>
140941184: 48 89 f1                    	mov	rcx, rsi
140941187: 48 81 c4 00 01 00 00        	add	rsp, 0x100
14094118e: 5b                          	pop	rbx
14094118f: 5f                          	pop	rdi
140941190: 5e                          	pop	rsi
140941191: e9 2a 4a 22 00              	jmp	0x140b65bc0 <.text+0xb64bc0>
