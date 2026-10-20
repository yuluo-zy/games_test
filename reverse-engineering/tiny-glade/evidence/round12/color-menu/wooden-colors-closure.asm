
reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe:	file format coff-x86-64

Disassembly of section .text:

0000000140001000 <.text>:
1422acce0:     	pushq	%rbp
1422acce1:     	pushq	%r15
1422acce3:     	pushq	%r14
1422acce5:     	pushq	%r13
1422acce7:     	pushq	%r12
1422acce9:     	pushq	%rsi
1422accea:     	pushq	%rdi
1422acceb:     	pushq	%rbx
1422accec:     	subq	$0xb8, %rsp
1422accf3:     	leaq	0x80(%rsp), %rbp
1422accfb:     	movaps	%xmm6, 0x20(%rbp)
1422accff:     	movq	$-0x2, 0x18(%rbp)
1422acd07:     	movq	%rcx, %r14
1422acd0a:     	movq	0x137f387(%rip), %rax   # 0x14362c098
1422acd11:     	movzbl	0x1387ef1(%rip), %ecx   # 0x143634c09
1422acd18:     	movzbl	(%rax), %ebx
1422acd1b:     	movl	$0x190, %ecx            # imm = 0x190
1422acd20:     	movl	$0x8, %edx
1422acd25:     	callq	0x140613c10 <.text+0x612c10>
1422acd2a:     	movq	%rax, %rsi
1422acd2d:     	testb	%bl, %bl
1422acd2f:     	movq	%rax, -0x58(%rbp)
1422acd33:     	movq	%r14, -0x60(%rbp)
1422acd37:     	je	0x1422acf94 <.text+0x22abf94>
1422acd3d:     	testq	%rsi, %rsi
1422acd40:     	je	0x1422ad489 <.text+0x22ac489>
1422acd46:     	movzbl	0x1387ebc(%rip), %eax   # 0x143634c09
1422acd4d:     	movl	$0x17, %ecx
1422acd52:     	movl	$0x1, %edx
1422acd57:     	callq	0x140613c10 <.text+0x612c10>
1422acd5c:     	testq	%rax, %rax
1422acd5f:     	je	0x1422ad498 <.text+0x22ac498>
1422acd65:     	movq	%rax, %rdi
1422acd68:     	movups	0xd8f671(%rip), %xmm0   # 0x14303c3e0
1422acd6f:     	movups	%xmm0, (%rax)
1422acd72:     	movabsq	$0x646f6f772f73776f, %rax # imm = 0x646F6F772F73776F
1422acd7c:     	movq	%rax, 0xf(%rdi)
1422acd80:     	movzbl	0x1387e82(%rip), %eax   # 0x143634c09
1422acd87:     	movl	$0x13, %ecx
1422acd8c:     	movl	$0x1, %edx
1422acd91:     	callq	0x140613c10 <.text+0x612c10>
1422acd96:     	testq	%rax, %rax
1422acd99:     	movq	%rdi, -0x50(%rbp)
1422acd9d:     	je	0x1422ad4ce <.text+0x22ac4ce>
1422acda3:     	movups	0xd8f50c(%rip), %xmm0   # 0x14303c2b6
1422acdaa:     	movups	%xmm0, (%rax)
1422acdad:     	movq	%rax, -0x48(%rbp)
1422acdb1:     	movl	$0x6465725f, 0xf(%rax)  # imm = 0x6465725F
1422acdb8:     	movzbl	0x1387e4a(%rip), %eax   # 0x143634c09
1422acdbf:     	movl	$0x15, %ecx
1422acdc4:     	movl	$0x1, %edx
1422acdc9:     	callq	0x140613c10 <.text+0x612c10>
1422acdce:     	testq	%rax, %rax
1422acdd1:     	je	0x1422ad504 <.text+0x22ac504>
1422acdd7:     	movups	0xd8f563(%rip), %xmm0   # 0x14303c341
1422acdde:     	movups	%xmm0, (%rax)
1422acde1:     	movabsq	$0x6e656572675f666f, %rcx # imm = 0x6E656572675F666F
1422acdeb:     	movq	%rax, -0x40(%rbp)
1422acdef:     	movq	%rcx, 0xd(%rax)
1422acdf3:     	movzbl	0x1387e0f(%rip), %eax   # 0x143634c09
1422acdfa:     	movl	$0x11, %ecx
1422acdff:     	movl	$0x1, %edx
1422ace04:     	callq	0x140613c10 <.text+0x612c10>
1422ace09:     	testq	%rax, %rax
1422ace0c:     	je	0x1422ad53a <.text+0x22ac53a>
1422ace12:     	movups	0xd8f477(%rip), %xmm6   # 0x14303c290
1422ace19:     	movups	%xmm6, (%rax)
1422ace1c:     	movq	%rax, -0x38(%rbp)
1422ace20:     	movb	$0x64, 0x10(%rax)
1422ace24:     	movzbl	0x1387dde(%rip), %eax   # 0x143634c09
1422ace2b:     	movl	$0x11, %ecx
1422ace30:     	movl	$0x1, %edx
1422ace35:     	callq	0x140613c10 <.text+0x612c10>
1422ace3a:     	testq	%rax, %rax
1422ace3d:     	je	0x1422ad570 <.text+0x22ac570>
1422ace43:     	movups	%xmm6, (%rax)
1422ace46:     	movq	%rax, -0x30(%rbp)
1422ace4a:     	movb	$0x64, 0x10(%rax)
1422ace4e:     	movzbl	0x1387db4(%rip), %eax   # 0x143634c09
1422ace55:     	movl	$0x11, %ecx
1422ace5a:     	movl	$0x1, %edx
1422ace5f:     	callq	0x140613c10 <.text+0x612c10>
1422ace64:     	testq	%rax, %rax
1422ace67:     	je	0x1422ad5a6 <.text+0x22ac5a6>
1422ace6d:     	movups	%xmm6, (%rax)
1422ace70:     	movq	%rax, -0x28(%rbp)
1422ace74:     	movb	$0x64, 0x10(%rax)
1422ace78:     	movzbl	0x1387d8a(%rip), %eax   # 0x143634c09
1422ace7f:     	movl	$0x11, %ecx
1422ace84:     	movl	$0x1, %edx
1422ace89:     	callq	0x140613c10 <.text+0x612c10>
1422ace8e:     	testq	%rax, %rax
1422ace91:     	je	0x1422ad5dc <.text+0x22ac5dc>
1422ace97:     	movups	%xmm6, (%rax)
1422ace9a:     	movq	%rax, -0x20(%rbp)
1422ace9e:     	movb	$0x64, 0x10(%rax)
1422acea2:     	movzbl	0x1387d60(%rip), %eax   # 0x143634c09
1422acea9:     	movl	$0x11, %ecx
1422aceae:     	movl	$0x1, %edx
1422aceb3:     	callq	0x140613c10 <.text+0x612c10>
1422aceb8:     	testq	%rax, %rax
1422acebb:     	je	0x1422ad612 <.text+0x22ac612>
1422acec1:     	movups	%xmm6, (%rax)
1422acec4:     	movq	%rax, -0x18(%rbp)
1422acec8:     	movb	$0x64, 0x10(%rax)
1422acecc:     	movzbl	0x1387d36(%rip), %eax   # 0x143634c09
1422aced3:     	movl	$0x11, %ecx
1422aced8:     	movl	$0x1, %edx
1422acedd:     	callq	0x140613c10 <.text+0x612c10>
1422acee2:     	testq	%rax, %rax
1422acee5:     	je	0x1422ad642 <.text+0x22ac642>
1422aceeb:     	movups	%xmm6, (%rax)
1422aceee:     	movq	%rax, -0x10(%rbp)
1422acef2:     	movb	$0x64, 0x10(%rax)
1422acef6:     	movzbl	0x1387d0c(%rip), %eax   # 0x143634c09
1422acefd:     	movl	$0x11, %edi
1422acf02:     	movl	$0x11, %ecx
1422acf07:     	movl	$0x1, %edx
1422acf0c:     	callq	0x140613c10 <.text+0x612c10>
1422acf11:     	testq	%rax, %rax
1422acf14:     	je	0x1422ad672 <.text+0x22ac672>
1422acf1a:     	movups	%xmm6, (%rax)
1422acf1d:     	movb	$0x64, 0x10(%rax)
1422acf21:     	movl	$0x309, 0x4(%rbp)       # imm = 0x309
1422acf28:     	movl	$0x5, 0x10(%rbp)
1422acf2f:     	movl	$0x15, %esi
1422acf34:     	movl	$0x3, %r9d
1422acf3a:     	movl	$0x13, %r10d
1422acf40:     	xorl	%r11d, %r11d
1422acf43:     	movl	$0x17, %r15d
1422acf49:     	movl	$0x11, %edx
1422acf4e:     	movl	$0x309, 0x14(%rbp)      # imm = 0x309
1422acf55:     	movl	$0x11, %r13d
1422acf5b:     	movl	$0x309, 0xc(%rbp)       # imm = 0x309
1422acf62:     	movl	$0x11, %r14d
1422acf68:     	movl	$0x309, 0x8(%rbp)       # imm = 0x309
1422acf6f:     	movl	$0x309, (%rbp)          # imm = 0x309
1422acf76:     	movl	$0x11, %r8d
1422acf7c:     	movl	$0x309, -0x4(%rbp)      # imm = 0x309
1422acf83:     	movl	$0x11, %ebx
1422acf88:     	movl	$0x309, -0x8(%rbp)      # imm = 0x309
1422acf8f:     	jmp	0x1422ad242 <.text+0x22ac242>
1422acf94:     	testq	%rsi, %rsi
1422acf97:     	je	0x1422ad489 <.text+0x22ac489>
1422acf9d:     	movzbl	0x1387c65(%rip), %eax   # 0x143634c09
1422acfa4:     	movl	$0x18, %ecx
1422acfa9:     	movl	$0x1, %edx
1422acfae:     	callq	0x140613c10 <.text+0x612c10>
1422acfb3:     	testq	%rax, %rax
1422acfb6:     	je	0x1422ad4b3 <.text+0x22ac4b3>
1422acfbc:     	movq	%rax, %rdi
1422acfbf:     	movups	0xd8f402(%rip), %xmm0   # 0x14303c3c8
1422acfc6:     	movups	%xmm0, (%rax)
1422acfc9:     	movabsq	$0x65746968772f7377, %rax # imm = 0x65746968772F7377
1422acfd3:     	movq	%rax, 0x10(%rdi)
1422acfd7:     	movzbl	0x1387c2b(%rip), %eax   # 0x143634c09
1422acfde:     	movl	$0x17, %ecx
1422acfe3:     	movl	$0x1, %edx
1422acfe8:     	callq	0x140613c10 <.text+0x612c10>
1422acfed:     	testq	%rax, %rax
1422acff0:     	movq	%rdi, -0x50(%rbp)
1422acff4:     	je	0x1422ad4e9 <.text+0x22ac4e9>
1422acffa:     	movups	0xd8f3df(%rip), %xmm0   # 0x14303c3e0
1422ad001:     	movups	%xmm0, (%rax)
1422ad004:     	movabsq	$0x646f6f772f73776f, %rcx # imm = 0x646F6F772F73776F
1422ad00e:     	movq	%rax, -0x48(%rbp)
1422ad012:     	movq	%rcx, 0xf(%rax)
1422ad016:     	movzbl	0x1387bec(%rip), %eax   # 0x143634c09
1422ad01d:     	movl	$0x1c, %ecx
1422ad022:     	movl	$0x1, %edx
1422ad027:     	callq	0x140613c10 <.text+0x612c10>
1422ad02c:     	testq	%rax, %rax
1422ad02f:     	je	0x1422ad51f <.text+0x22ac51f>
1422ad035:     	movups	0xd8f3c7(%rip), %xmm0   # 0x14303c403
1422ad03c:     	movups	%xmm0, 0xc(%rax)
1422ad040:     	movups	0xd8f3b0(%rip), %xmm0   # 0x14303c3f7
1422ad047:     	movq	%rax, -0x40(%rbp)
1422ad04b:     	movups	%xmm0, (%rax)
1422ad04e:     	movzbl	0x1387bb4(%rip), %eax   # 0x143634c09
1422ad055:     	movl	$0x14, %ecx
1422ad05a:     	movl	$0x1, %edx
1422ad05f:     	callq	0x140613c10 <.text+0x612c10>
1422ad064:     	testq	%rax, %rax
1422ad067:     	je	0x1422ad555 <.text+0x22ac555>
1422ad06d:     	movups	0xd8f2fb(%rip), %xmm0   # 0x14303c36f
1422ad074:     	movups	%xmm0, (%rax)
1422ad077:     	movq	%rax, -0x38(%rbp)
1422ad07b:     	movl	$0x6b726164, 0x10(%rax) # imm = 0x6B726164
1422ad082:     	movzbl	0x1387b80(%rip), %eax   # 0x143634c09
1422ad089:     	movl	$0x13, %ecx
1422ad08e:     	movl	$0x1, %edx
1422ad093:     	callq	0x140613c10 <.text+0x612c10>
1422ad098:     	testq	%rax, %rax
1422ad09b:     	je	0x1422ad58b <.text+0x22ac58b>
1422ad0a1:     	movups	0xd8f20e(%rip), %xmm0   # 0x14303c2b6
1422ad0a8:     	movups	%xmm0, (%rax)
1422ad0ab:     	movq	%rax, -0x30(%rbp)
1422ad0af:     	movl	$0x6465725f, 0xf(%rax)  # imm = 0x6465725F
1422ad0b6:     	movzbl	0x1387b4c(%rip), %eax   # 0x143634c09
1422ad0bd:     	movl	$0xf, %ecx
1422ad0c2:     	movl	$0x1, %edx
1422ad0c7:     	callq	0x140613c10 <.text+0x612c10>
1422ad0cc:     	testq	%rax, %rax
1422ad0cf:     	je	0x1422ad5c1 <.text+0x22ac5c1>
1422ad0d5:     	movabsq	$0x6b6e69702f6e6f63, %rcx # imm = 0x6B6E69702F6E6F63
1422ad0df:     	movq	%rcx, 0x7(%rax)
1422ad0e3:     	movabsq	$0x63695f726f6c6f63, %rcx # imm = 0x63695F726F6C6F63
1422ad0ed:     	movq	%rax, -0x28(%rbp)
1422ad0f1:     	movq	%rcx, (%rax)
1422ad0f4:     	movzbl	0x1387b0e(%rip), %eax   # 0x143634c09
1422ad0fb:     	movl	$0x19, %ecx
1422ad100:     	movl	$0x1, %edx
1422ad105:     	callq	0x140613c10 <.text+0x612c10>
1422ad10a:     	testq	%rax, %rax
1422ad10d:     	je	0x1422ad5f7 <.text+0x22ac5f7>
1422ad113:     	movups	0xd8f302(%rip), %xmm0   # 0x14303c41c
1422ad11a:     	movups	%xmm0, 0x9(%rax)
1422ad11e:     	movups	0xd8f2ee(%rip), %xmm0   # 0x14303c413
1422ad125:     	movq	%rax, -0x20(%rbp)
1422ad129:     	movups	%xmm0, (%rax)
1422ad12c:     	movzbl	0x1387ad6(%rip), %eax   # 0x143634c09
1422ad133:     	movl	$0x14, %ecx
1422ad138:     	movl	$0x1, %edx
1422ad13d:     	callq	0x140613c10 <.text+0x612c10>
1422ad142:     	testq	%rax, %rax
1422ad145:     	je	0x1422ad62a <.text+0x22ac62a>
1422ad14b:     	movups	0xd8f18d(%rip), %xmm0   # 0x14303c2df
1422ad152:     	movups	%xmm0, (%rax)
1422ad155:     	movq	%rax, -0x18(%rbp)
1422ad159:     	movl	$0x65756c62, 0x10(%rax) # imm = 0x65756C62
1422ad160:     	movzbl	0x1387aa2(%rip), %eax   # 0x143634c09
1422ad167:     	movl	$0x1b, %ecx
1422ad16c:     	movl	$0x1, %edx
1422ad171:     	callq	0x140613c10 <.text+0x612c10>
1422ad176:     	testq	%rax, %rax
1422ad179:     	je	0x1422ad65a <.text+0x22ac65a>
1422ad17f:     	movups	0xd8f1ab(%rip), %xmm0   # 0x14303c331
1422ad186:     	movups	%xmm0, 0xb(%rax)
1422ad18a:     	movups	0xd8f195(%rip), %xmm0   # 0x14303c326
1422ad191:     	movq	%rax, -0x10(%rbp)
1422ad195:     	movups	%xmm0, (%rax)
1422ad198:     	movzbl	0x1387a6a(%rip), %eax   # 0x143634c09
1422ad19f:     	movl	$0x15, %ebx
1422ad1a4:     	movl	$0x15, %ecx
1422ad1a9:     	movl	$0x1, %edx
1422ad1ae:     	callq	0x140613c10 <.text+0x612c10>
1422ad1b3:     	testq	%rax, %rax
1422ad1b6:     	je	0x1422ad68a <.text+0x22ac68a>
1422ad1bc:     	movups	0xd8f17e(%rip), %xmm0   # 0x14303c341
1422ad1c3:     	movups	%xmm0, (%rax)
1422ad1c6:     	movabsq	$0x6e656572675f666f, %rcx # imm = 0x6E656572675F666F
1422ad1d0:     	movq	%rcx, 0xd(%rax)
1422ad1d4:     	movl	$0x5, -0x8(%rbp)
1422ad1db:     	movl	$0x6, -0x4(%rbp)
1422ad1e2:     	movl	$0x1b, %r8d
1422ad1e8:     	movl	$0x2, (%rbp)
1422ad1ef:     	movl	$0x8, 0x8(%rbp)
1422ad1f6:     	movl	$0x19, %r14d
1422ad1fc:     	movl	$0x9, 0xc(%rbp)
1422ad203:     	movl	$0xf, %r13d
1422ad209:     	movl	$0x3, 0x14(%rbp)
1422ad210:     	movl	$0x13, %edx
1422ad215:     	movl	$0x7, 0x4(%rbp)
1422ad21c:     	movl	$0x14, %edi
1422ad221:     	movl	$0x1, 0x10(%rbp)
1422ad228:     	movl	$0x1c, %esi
1422ad22d:     	xorl	%r9d, %r9d
1422ad230:     	movl	$0x17, %r10d
1422ad236:     	movl	$0x4, %r11d
1422ad23c:     	movl	$0x18, %r15d
1422ad242:     	movq	-0x58(%rbp), %r12
1422ad246:     	movq	%r15, (%r12)
1422ad24a:     	movq	-0x50(%rbp), %rcx
1422ad24e:     	movq	%rcx, 0x8(%r12)
1422ad253:     	movq	%r15, 0x10(%r12)
1422ad258:     	movb	$0x3, 0x18(%r12)
1422ad25e:     	movl	%r11d, 0x1c(%r12)
1422ad263:     	movl	$0x0, 0x24(%r12)
1422ad26c:     	movq	%r10, 0x28(%r12)
1422ad271:     	movq	-0x48(%rbp), %rcx
1422ad275:     	movq	%rcx, 0x30(%r12)
1422ad27a:     	movq	%r10, 0x38(%r12)
1422ad27f:     	movb	$0x3, 0x40(%r12)
1422ad285:     	movl	%r9d, 0x44(%r12)
1422ad28a:     	movl	$0x0, 0x4c(%r12)
1422ad293:     	movq	%rsi, 0x50(%r12)
1422ad298:     	movq	-0x40(%rbp), %rcx
1422ad29c:     	movq	%rcx, 0x58(%r12)
1422ad2a1:     	movq	%rsi, 0x60(%r12)
1422ad2a6:     	movb	$0x3, 0x68(%r12)
1422ad2ac:     	movl	0x10(%rbp), %ecx
1422ad2af:     	movl	%ecx, 0x6c(%r12)
1422ad2b4:     	movl	$0x0, 0x74(%r12)
1422ad2bd:     	movq	%rdi, 0x78(%r12)
1422ad2c2:     	movq	-0x38(%rbp), %rcx
1422ad2c6:     	movq	%rcx, 0x80(%r12)
1422ad2ce:     	movq	%rdi, 0x88(%r12)
1422ad2d6:     	movb	$0x3, 0x90(%r12)
1422ad2df:     	movl	0x4(%rbp), %ecx
1422ad2e2:     	movl	%ecx, 0x94(%r12)
1422ad2ea:     	movl	$0x0, 0x9c(%r12)
1422ad2f6:     	movq	%rdx, 0xa0(%r12)
1422ad2fe:     	movq	-0x30(%rbp), %rcx
1422ad302:     	movq	%rcx, 0xa8(%r12)
1422ad30a:     	movq	%rdx, 0xb0(%r12)
1422ad312:     	movb	$0x3, 0xb8(%r12)
1422ad31b:     	movl	0x14(%rbp), %ecx
1422ad31e:     	movl	%ecx, 0xbc(%r12)
1422ad326:     	movl	$0x0, 0xc4(%r12)
1422ad332:     	movq	%r13, 0xc8(%r12)
1422ad33a:     	movq	-0x28(%rbp), %rcx
1422ad33e:     	movq	%rcx, 0xd0(%r12)
1422ad346:     	movq	%r13, 0xd8(%r12)
1422ad34e:     	movb	$0x3, 0xe0(%r12)
1422ad357:     	movl	0xc(%rbp), %ecx
1422ad35a:     	movl	%ecx, 0xe4(%r12)
1422ad362:     	movl	$0x0, 0xec(%r12)
1422ad36e:     	movq	%r14, 0xf0(%r12)
1422ad376:     	movq	-0x20(%rbp), %rcx
1422ad37a:     	movq	%rcx, 0xf8(%r12)
1422ad382:     	movq	%r14, 0x100(%r12)
1422ad38a:     	movb	$0x3, 0x108(%r12)
1422ad393:     	movl	0x8(%rbp), %ecx
1422ad396:     	movl	%ecx, 0x10c(%r12)
1422ad39e:     	movl	$0x0, 0x114(%r12)
1422ad3aa:     	movq	%rdi, 0x118(%r12)
1422ad3b2:     	movq	-0x18(%rbp), %rcx
1422ad3b6:     	movq	%rcx, 0x120(%r12)
1422ad3be:     	movq	%rdi, 0x128(%r12)
1422ad3c6:     	movb	$0x3, 0x130(%r12)
1422ad3cf:     	movl	(%rbp), %ecx
1422ad3d2:     	movl	%ecx, 0x134(%r12)
1422ad3da:     	movl	$0x0, 0x13c(%r12)
1422ad3e6:     	movq	%r8, 0x140(%r12)
1422ad3ee:     	movq	-0x10(%rbp), %rcx
1422ad3f2:     	movq	%rcx, 0x148(%r12)
1422ad3fa:     	movq	%r8, 0x150(%r12)
1422ad402:     	movb	$0x3, 0x158(%r12)
1422ad40b:     	movl	-0x4(%rbp), %ecx
1422ad40e:     	movl	%ecx, 0x15c(%r12)
1422ad416:     	movl	$0x0, 0x164(%r12)
1422ad422:     	movq	%rbx, 0x168(%r12)
1422ad42a:     	movq	%rax, 0x170(%r12)
1422ad432:     	movq	%rbx, 0x178(%r12)
1422ad43a:     	movb	$0x3, 0x180(%r12)
1422ad443:     	movl	-0x8(%rbp), %eax
1422ad446:     	movl	%eax, 0x184(%r12)
1422ad44e:     	movl	$0x0, 0x18c(%r12)
1422ad45a:     	movq	-0x60(%rbp), %rax
1422ad45e:     	movq	$0xa, (%rax)
1422ad465:     	movq	%r12, 0x8(%rax)
1422ad469:     	movq	$0xa, 0x10(%rax)
1422ad471:     	movaps	0x20(%rbp), %xmm6
1422ad475:     	addq	$0xb8, %rsp
1422ad47c:     	popq	%rbx
1422ad47d:     	popq	%rdi
1422ad47e:     	popq	%rsi
1422ad47f:     	popq	%r12
1422ad481:     	popq	%r13
1422ad483:     	popq	%r14
1422ad485:     	popq	%r15
1422ad487:     	popq	%rbp
1422ad488:     	retq
1422ad489:     	movl	$0x8, %ecx
1422ad48e:     	movl	$0x190, %edx            # imm = 0x190
1422ad493:     	callq	0x1428d9000 <.text+0x28d8000>
1422ad498:     	leaq	0xe897c9(%rip), %r8     # 0x143136c68
1422ad49f:     	movl	$0x1, %ecx
1422ad4a4:     	movl	$0x17, %edx
1422ad4a9:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422ad4ae:     	jmp	0x1422ad6a0 <.text+0x22ac6a0>
1422ad4b3:     	leaq	0xe897ae(%rip), %r8     # 0x143136c68
1422ad4ba:     	movl	$0x1, %ecx
1422ad4bf:     	movl	$0x18, %edx
1422ad4c4:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422ad4c9:     	jmp	0x1422ad6a0 <.text+0x22ac6a0>
1422ad4ce:     	leaq	0xe89793(%rip), %r8     # 0x143136c68
1422ad4d5:     	movl	$0x1, %ecx
1422ad4da:     	movl	$0x13, %edx
1422ad4df:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422ad4e4:     	jmp	0x1422ad6a0 <.text+0x22ac6a0>
1422ad4e9:     	leaq	0xe89778(%rip), %r8     # 0x143136c68
1422ad4f0:     	movl	$0x1, %ecx
1422ad4f5:     	movl	$0x17, %edx
1422ad4fa:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422ad4ff:     	jmp	0x1422ad6a0 <.text+0x22ac6a0>
1422ad504:     	leaq	0xe8975d(%rip), %r8     # 0x143136c68
1422ad50b:     	movl	$0x1, %ecx
1422ad510:     	movl	$0x15, %edx
1422ad515:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422ad51a:     	jmp	0x1422ad6a0 <.text+0x22ac6a0>
1422ad51f:     	leaq	0xe89742(%rip), %r8     # 0x143136c68
1422ad526:     	movl	$0x1, %ecx
1422ad52b:     	movl	$0x1c, %edx
1422ad530:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422ad535:     	jmp	0x1422ad6a0 <.text+0x22ac6a0>
1422ad53a:     	leaq	0xe89727(%rip), %r8     # 0x143136c68
1422ad541:     	movl	$0x1, %ecx
1422ad546:     	movl	$0x11, %edx
1422ad54b:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422ad550:     	jmp	0x1422ad6a0 <.text+0x22ac6a0>
1422ad555:     	leaq	0xe8970c(%rip), %r8     # 0x143136c68
1422ad55c:     	movl	$0x1, %ecx
1422ad561:     	movl	$0x14, %edx
1422ad566:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422ad56b:     	jmp	0x1422ad6a0 <.text+0x22ac6a0>
1422ad570:     	leaq	0xe896f1(%rip), %r8     # 0x143136c68
1422ad577:     	movl	$0x1, %ecx
1422ad57c:     	movl	$0x11, %edx
1422ad581:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422ad586:     	jmp	0x1422ad6a0 <.text+0x22ac6a0>
1422ad58b:     	leaq	0xe896d6(%rip), %r8     # 0x143136c68
1422ad592:     	movl	$0x1, %ecx
1422ad597:     	movl	$0x13, %edx
1422ad59c:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422ad5a1:     	jmp	0x1422ad6a0 <.text+0x22ac6a0>
1422ad5a6:     	leaq	0xe896bb(%rip), %r8     # 0x143136c68
1422ad5ad:     	movl	$0x1, %ecx
1422ad5b2:     	movl	$0x11, %edx
1422ad5b7:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422ad5bc:     	jmp	0x1422ad6a0 <.text+0x22ac6a0>
1422ad5c1:     	leaq	0xe896a0(%rip), %r8     # 0x143136c68
1422ad5c8:     	movl	$0x1, %ecx
1422ad5cd:     	movl	$0xf, %edx
1422ad5d2:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422ad5d7:     	jmp	0x1422ad6a0 <.text+0x22ac6a0>
1422ad5dc:     	leaq	0xe89685(%rip), %r8     # 0x143136c68
1422ad5e3:     	movl	$0x1, %ecx
1422ad5e8:     	movl	$0x11, %edx
1422ad5ed:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422ad5f2:     	jmp	0x1422ad6a0 <.text+0x22ac6a0>
1422ad5f7:     	leaq	0xe8966a(%rip), %r8     # 0x143136c68
1422ad5fe:     	movl	$0x1, %ecx
1422ad603:     	movl	$0x19, %edx
1422ad608:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422ad60d:     	jmp	0x1422ad6a0 <.text+0x22ac6a0>
1422ad612:     	leaq	0xe8964f(%rip), %r8     # 0x143136c68
1422ad619:     	movl	$0x1, %ecx
1422ad61e:     	movl	$0x11, %edx
1422ad623:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422ad628:     	jmp	0x1422ad6a0 <.text+0x22ac6a0>
1422ad62a:     	leaq	0xe89637(%rip), %r8     # 0x143136c68
1422ad631:     	movl	$0x1, %ecx
1422ad636:     	movl	$0x14, %edx
1422ad63b:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422ad640:     	jmp	0x1422ad6a0 <.text+0x22ac6a0>
1422ad642:     	leaq	0xe8961f(%rip), %r8     # 0x143136c68
1422ad649:     	movl	$0x1, %ecx
1422ad64e:     	movl	$0x11, %edx
1422ad653:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422ad658:     	jmp	0x1422ad6a0 <.text+0x22ac6a0>
1422ad65a:     	leaq	0xe89607(%rip), %r8     # 0x143136c68
1422ad661:     	movl	$0x1, %ecx
1422ad666:     	movl	$0x1b, %edx
1422ad66b:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422ad670:     	jmp	0x1422ad6a0 <.text+0x22ac6a0>
1422ad672:     	leaq	0xe895ef(%rip), %r8     # 0x143136c68
1422ad679:     	movl	$0x1, %ecx
1422ad67e:     	movl	$0x11, %edx
1422ad683:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422ad688:     	jmp	0x1422ad6a0 <.text+0x22ac6a0>
1422ad68a:     	leaq	0xe895d7(%rip), %r8     # 0x143136c68
1422ad691:     	movl	$0x1, %ecx
1422ad696:     	movl	$0x15, %edx
1422ad69b:     	callq	0x1428d8fe3 <.text+0x28d7fe3>
1422ad6a0:     	ud2
1422ad6a2:     	nopw	%cs:(%rax,%rax)
1422ad6b0:     	movq	%rdx, 0x10(%rsp)
1422ad6b5:     	pushq	%rbp
1422ad6b6:     	pushq	%r15
1422ad6b8:     	pushq	%r14
1422ad6ba:     	pushq	%r13
1422ad6bc:     	pushq	%r12
1422ad6be:     	pushq	%rsi
1422ad6bf:     	pushq	%rdi
1422ad6c0:     	pushq	%rbx
1422ad6c1:     	subq	$0x38, %rsp
1422ad6c5:     	leaq	0x80(%rdx), %rbp
1422ad6cc:     	movaps	%xmm6, 0x20(%rsp)
1422ad6d1:     	movl	$0x190, %edx            # imm = 0x190
1422ad6d6:     	movl	$0x8, %r8d
1422ad6dc:     	movq	-0x58(%rbp), %rcx
1422ad6e0:     	callq	0x140613c20 <.text+0x612c20>
1422ad6e5:     	movaps	0x20(%rsp), %xmm6
1422ad6ea:     	addq	$0x38, %rsp
1422ad6ee:     	popq	%rbx
1422ad6ef:     	popq	%rdi
1422ad6f0:     	popq	%rsi
1422ad6f1:     	popq	%r12
1422ad6f3:     	popq	%r13
1422ad6f5:     	popq	%r14
1422ad6f7:     	popq	%r15
1422ad6f9:     	popq	%rbp
1422ad6fa:     	retq
1422ad6fb:     	nopl	(%rax,%rax)
1422ad700:     	movq	%rdx, 0x10(%rsp)
1422ad705:     	pushq	%rbp
1422ad706:     	pushq	%r15
1422ad708:     	pushq	%r14
1422ad70a:     	pushq	%r13
1422ad70c:     	pushq	%r12
1422ad70e:     	pushq	%rsi
1422ad70f:     	pushq	%rdi
1422ad710:     	pushq	%rbx
1422ad711:     	subq	$0x38, %rsp
1422ad715:     	leaq	0x80(%rdx), %rbp
1422ad71c:     	movaps	%xmm6, 0x20(%rsp)
1422ad721:     	movl	$0x18, %edx
1422ad726:     	movl	$0x1, %r8d
1422ad72c:     	movq	-0x50(%rbp), %rcx
1422ad730:     	callq	0x140613c20 <.text+0x612c20>
1422ad735:     	movaps	0x20(%rsp), %xmm6
1422ad73a:     	addq	$0x38, %rsp
1422ad73e:     	popq	%rbx
1422ad73f:     	popq	%rdi
1422ad740:     	popq	%rsi
1422ad741:     	popq	%r12
1422ad743:     	popq	%r13
1422ad745:     	popq	%r14
1422ad747:     	popq	%r15
1422ad749:     	popq	%rbp
1422ad74a:     	retq
1422ad74b:     	nopl	(%rax,%rax)
1422ad750:     	movq	%rdx, 0x10(%rsp)
1422ad755:     	pushq	%rbp
1422ad756:     	pushq	%r15
1422ad758:     	pushq	%r14
1422ad75a:     	pushq	%r13
1422ad75c:     	pushq	%r12
1422ad75e:     	pushq	%rsi
1422ad75f:     	pushq	%rdi
1422ad760:     	pushq	%rbx
1422ad761:     	subq	$0x38, %rsp
1422ad765:     	leaq	0x80(%rdx), %rbp
1422ad76c:     	movaps	%xmm6, 0x20(%rsp)
1422ad771:     	movl	$0x17, %edx
1422ad776:     	movl	$0x1, %r8d
1422ad77c:     	movq	-0x48(%rbp), %rcx
1422ad780:     	callq	0x140613c20 <.text+0x612c20>
1422ad785:     	movaps	0x20(%rsp), %xmm6
1422ad78a:     	addq	$0x38, %rsp
1422ad78e:     	popq	%rbx
1422ad78f:     	popq	%rdi
1422ad790:     	popq	%rsi
1422ad791:     	popq	%r12
1422ad793:     	popq	%r13
1422ad795:     	popq	%r14
1422ad797:     	popq	%r15
1422ad799:     	popq	%rbp
1422ad79a:     	retq
1422ad79b:     	nopl	(%rax,%rax)
1422ad7a0:     	movq	%rdx, 0x10(%rsp)
1422ad7a5:     	pushq	%rbp
1422ad7a6:     	pushq	%r15
1422ad7a8:     	pushq	%r14
1422ad7aa:     	pushq	%r13
1422ad7ac:     	pushq	%r12
1422ad7ae:     	pushq	%rsi
1422ad7af:     	pushq	%rdi
1422ad7b0:     	pushq	%rbx
1422ad7b1:     	subq	$0x38, %rsp
1422ad7b5:     	leaq	0x80(%rdx), %rbp
1422ad7bc:     	movaps	%xmm6, 0x20(%rsp)
1422ad7c1:     	movl	$0x1c, %edx
1422ad7c6:     	movl	$0x1, %r8d
1422ad7cc:     	movq	-0x40(%rbp), %rcx
1422ad7d0:     	callq	0x140613c20 <.text+0x612c20>
1422ad7d5:     	movaps	0x20(%rsp), %xmm6
1422ad7da:     	addq	$0x38, %rsp
1422ad7de:     	popq	%rbx
1422ad7df:     	popq	%rdi
1422ad7e0:     	popq	%rsi
1422ad7e1:     	popq	%r12
1422ad7e3:     	popq	%r13
1422ad7e5:     	popq	%r14
1422ad7e7:     	popq	%r15
1422ad7e9:     	popq	%rbp
1422ad7ea:     	retq
1422ad7eb:     	nopl	(%rax,%rax)
1422ad7f0:     	movq	%rdx, 0x10(%rsp)
1422ad7f5:     	pushq	%rbp
1422ad7f6:     	pushq	%r15
1422ad7f8:     	pushq	%r14
1422ad7fa:     	pushq	%r13
1422ad7fc:     	pushq	%r12
1422ad7fe:     	pushq	%rsi
1422ad7ff:     	pushq	%rdi
1422ad800:     	pushq	%rbx
1422ad801:     	subq	$0x38, %rsp
1422ad805:     	leaq	0x80(%rdx), %rbp
1422ad80c:     	movaps	%xmm6, 0x20(%rsp)
1422ad811:     	movl	$0x14, %edx
1422ad816:     	movl	$0x1, %r8d
1422ad81c:     	movq	-0x38(%rbp), %rcx
1422ad820:     	callq	0x140613c20 <.text+0x612c20>
1422ad825:     	movaps	0x20(%rsp), %xmm6
1422ad82a:     	addq	$0x38, %rsp
1422ad82e:     	popq	%rbx
1422ad82f:     	popq	%rdi
1422ad830:     	popq	%rsi
1422ad831:     	popq	%r12
1422ad833:     	popq	%r13
1422ad835:     	popq	%r14
1422ad837:     	popq	%r15
1422ad839:     	popq	%rbp
1422ad83a:     	retq
1422ad83b:     	nopl	(%rax,%rax)
1422ad840:     	movq	%rdx, 0x10(%rsp)
1422ad845:     	pushq	%rbp
1422ad846:     	pushq	%r15
1422ad848:     	pushq	%r14
1422ad84a:     	pushq	%r13
1422ad84c:     	pushq	%r12
1422ad84e:     	pushq	%rsi
1422ad84f:     	pushq	%rdi
1422ad850:     	pushq	%rbx
1422ad851:     	subq	$0x38, %rsp
1422ad855:     	leaq	0x80(%rdx), %rbp
1422ad85c:     	movaps	%xmm6, 0x20(%rsp)
1422ad861:     	movl	$0x13, %edx
1422ad866:     	movl	$0x1, %r8d
1422ad86c:     	movq	-0x30(%rbp), %rcx
1422ad870:     	callq	0x140613c20 <.text+0x612c20>
1422ad875:     	movaps	0x20(%rsp), %xmm6
1422ad87a:     	addq	$0x38, %rsp
1422ad87e:     	popq	%rbx
1422ad87f:     	popq	%rdi
1422ad880:     	popq	%rsi
1422ad881:     	popq	%r12
1422ad883:     	popq	%r13
1422ad885:     	popq	%r14
1422ad887:     	popq	%r15
1422ad889:     	popq	%rbp
1422ad88a:     	retq
1422ad88b:     	nopl	(%rax,%rax)
1422ad890:     	movq	%rdx, 0x10(%rsp)
1422ad895:     	pushq	%rbp
1422ad896:     	pushq	%r15
1422ad898:     	pushq	%r14
1422ad89a:     	pushq	%r13
1422ad89c:     	pushq	%r12
1422ad89e:     	pushq	%rsi
1422ad89f:     	pushq	%rdi
1422ad8a0:     	pushq	%rbx
1422ad8a1:     	subq	$0x38, %rsp
1422ad8a5:     	leaq	0x80(%rdx), %rbp
1422ad8ac:     	movaps	%xmm6, 0x20(%rsp)
1422ad8b1:     	movl	$0xf, %edx
1422ad8b6:     	movl	$0x1, %r8d
1422ad8bc:     	movq	-0x28(%rbp), %rcx
1422ad8c0:     	callq	0x140613c20 <.text+0x612c20>
1422ad8c5:     	movaps	0x20(%rsp), %xmm6
1422ad8ca:     	addq	$0x38, %rsp
1422ad8ce:     	popq	%rbx
1422ad8cf:     	popq	%rdi
1422ad8d0:     	popq	%rsi
1422ad8d1:     	popq	%r12
1422ad8d3:     	popq	%r13
1422ad8d5:     	popq	%r14
1422ad8d7:     	popq	%r15
1422ad8d9:     	popq	%rbp
1422ad8da:     	retq
1422ad8db:     	nopl	(%rax,%rax)
1422ad8e0:     	movq	%rdx, 0x10(%rsp)
1422ad8e5:     	pushq	%rbp
1422ad8e6:     	pushq	%r15
1422ad8e8:     	pushq	%r14
1422ad8ea:     	pushq	%r13
1422ad8ec:     	pushq	%r12
1422ad8ee:     	pushq	%rsi
1422ad8ef:     	pushq	%rdi
1422ad8f0:     	pushq	%rbx
1422ad8f1:     	subq	$0x38, %rsp
1422ad8f5:     	leaq	0x80(%rdx), %rbp
1422ad8fc:     	movaps	%xmm6, 0x20(%rsp)
1422ad901:     	movl	$0x19, %edx
1422ad906:     	movl	$0x1, %r8d
1422ad90c:     	movq	-0x20(%rbp), %rcx
1422ad910:     	callq	0x140613c20 <.text+0x612c20>
1422ad915:     	movaps	0x20(%rsp), %xmm6
1422ad91a:     	addq	$0x38, %rsp
1422ad91e:     	popq	%rbx
1422ad91f:     	popq	%rdi
1422ad920:     	popq	%rsi
1422ad921:     	popq	%r12
1422ad923:     	popq	%r13
1422ad925:     	popq	%r14
1422ad927:     	popq	%r15
1422ad929:     	popq	%rbp
1422ad92a:     	retq
1422ad92b:     	nopl	(%rax,%rax)
1422ad930:     	movq	%rdx, 0x10(%rsp)
1422ad935:     	pushq	%rbp
1422ad936:     	pushq	%r15
1422ad938:     	pushq	%r14
1422ad93a:     	pushq	%r13
1422ad93c:     	pushq	%r12
1422ad93e:     	pushq	%rsi
1422ad93f:     	pushq	%rdi
1422ad940:     	pushq	%rbx
1422ad941:     	subq	$0x38, %rsp
1422ad945:     	leaq	0x80(%rdx), %rbp
1422ad94c:     	movaps	%xmm6, 0x20(%rsp)
1422ad951:     	movl	$0x14, %edx
1422ad956:     	movl	$0x1, %r8d
1422ad95c:     	movq	-0x18(%rbp), %rcx
1422ad960:     	callq	0x140613c20 <.text+0x612c20>
1422ad965:     	movaps	0x20(%rsp), %xmm6
1422ad96a:     	addq	$0x38, %rsp
1422ad96e:     	popq	%rbx
1422ad96f:     	popq	%rdi
1422ad970:     	popq	%rsi
1422ad971:     	popq	%r12
1422ad973:     	popq	%r13
1422ad975:     	popq	%r14
1422ad977:     	popq	%r15
1422ad979:     	popq	%rbp
1422ad97a:     	retq
1422ad97b:     	nopl	(%rax,%rax)
1422ad980:     	movq	%rdx, 0x10(%rsp)
1422ad985:     	pushq	%rbp
1422ad986:     	pushq	%r15
1422ad988:     	pushq	%r14
1422ad98a:     	pushq	%r13
1422ad98c:     	pushq	%r12
1422ad98e:     	pushq	%rsi
1422ad98f:     	pushq	%rdi
1422ad990:     	pushq	%rbx
1422ad991:     	subq	$0x38, %rsp
1422ad995:     	leaq	0x80(%rdx), %rbp
1422ad99c:     	movaps	%xmm6, 0x20(%rsp)
1422ad9a1:     	movl	$0x1b, %edx
1422ad9a6:     	movl	$0x1, %r8d
1422ad9ac:     	movq	-0x10(%rbp), %rcx
1422ad9b0:     	callq	0x140613c20 <.text+0x612c20>
1422ad9b5:     	movaps	0x20(%rsp), %xmm6
1422ad9ba:     	addq	$0x38, %rsp
1422ad9be:     	popq	%rbx
1422ad9bf:     	popq	%rdi
1422ad9c0:     	popq	%rsi
1422ad9c1:     	popq	%r12
1422ad9c3:     	popq	%r13
1422ad9c5:     	popq	%r14
1422ad9c7:     	popq	%r15
1422ad9c9:     	popq	%rbp
1422ad9ca:     	retq
1422ad9cb:     	nopl	(%rax,%rax)
1422ad9d0:     	movq	%rdx, 0x10(%rsp)
1422ad9d5:     	pushq	%rbp
1422ad9d6:     	pushq	%r15
1422ad9d8:     	pushq	%r14
1422ad9da:     	pushq	%r13
1422ad9dc:     	pushq	%r12
1422ad9de:     	pushq	%rsi
1422ad9df:     	pushq	%rdi
1422ad9e0:     	pushq	%rbx
1422ad9e1:     	subq	$0x38, %rsp
1422ad9e5:     	leaq	0x80(%rdx), %rbp
1422ad9ec:     	movaps	%xmm6, 0x20(%rsp)
1422ad9f1:     	movl	$0x190, %edx            # imm = 0x190
1422ad9f6:     	movl	$0x8, %r8d
1422ad9fc:     	movq	-0x58(%rbp), %rcx
1422ada00:     	callq	0x140613c20 <.text+0x612c20>
1422ada05:     	movaps	0x20(%rsp), %xmm6
1422ada0a:     	addq	$0x38, %rsp
1422ada0e:     	popq	%rbx
1422ada0f:     	popq	%rdi
1422ada10:     	popq	%rsi
1422ada11:     	popq	%r12
1422ada13:     	popq	%r13
1422ada15:     	popq	%r14
1422ada17:     	popq	%r15
1422ada19:     	popq	%rbp
1422ada1a:     	retq
1422ada1b:     	nopl	(%rax,%rax)
1422ada20:     	movq	%rdx, 0x10(%rsp)
1422ada25:     	pushq	%rbp
1422ada26:     	pushq	%r15
1422ada28:     	pushq	%r14
1422ada2a:     	pushq	%r13
1422ada2c:     	pushq	%r12
1422ada2e:     	pushq	%rsi
1422ada2f:     	pushq	%rdi
1422ada30:     	pushq	%rbx
1422ada31:     	subq	$0x38, %rsp
1422ada35:     	leaq	0x80(%rdx), %rbp
1422ada3c:     	movaps	%xmm6, 0x20(%rsp)
1422ada41:     	movl	$0x17, %edx
1422ada46:     	movl	$0x1, %r8d
1422ada4c:     	movq	-0x50(%rbp), %rcx
1422ada50:     	callq	0x140613c20 <.text+0x612c20>
1422ada55:     	movaps	0x20(%rsp), %xmm6
1422ada5a:     	addq	$0x38, %rsp
1422ada5e:     	popq	%rbx
1422ada5f:     	popq	%rdi
1422ada60:     	popq	%rsi
1422ada61:     	popq	%r12
1422ada63:     	popq	%r13
1422ada65:     	popq	%r14
1422ada67:     	popq	%r15
1422ada69:     	popq	%rbp
1422ada6a:     	retq
1422ada6b:     	nopl	(%rax,%rax)
1422ada70:     	movq	%rdx, 0x10(%rsp)
1422ada75:     	pushq	%rbp
1422ada76:     	pushq	%r15
1422ada78:     	pushq	%r14
1422ada7a:     	pushq	%r13
1422ada7c:     	pushq	%r12
1422ada7e:     	pushq	%rsi
1422ada7f:     	pushq	%rdi
1422ada80:     	pushq	%rbx
1422ada81:     	subq	$0x38, %rsp
1422ada85:     	leaq	0x80(%rdx), %rbp
1422ada8c:     	movaps	%xmm6, 0x20(%rsp)
1422ada91:     	movl	$0x13, %edx
1422ada96:     	movl	$0x1, %r8d
1422ada9c:     	movq	-0x48(%rbp), %rcx
1422adaa0:     	callq	0x140613c20 <.text+0x612c20>
1422adaa5:     	movaps	0x20(%rsp), %xmm6
1422adaaa:     	addq	$0x38, %rsp
1422adaae:     	popq	%rbx
1422adaaf:     	popq	%rdi
1422adab0:     	popq	%rsi
1422adab1:     	popq	%r12
1422adab3:     	popq	%r13
1422adab5:     	popq	%r14
1422adab7:     	popq	%r15
1422adab9:     	popq	%rbp
1422adaba:     	retq
1422adabb:     	nopl	(%rax,%rax)
1422adac0:     	movq	%rdx, 0x10(%rsp)
1422adac5:     	pushq	%rbp
1422adac6:     	pushq	%r15
1422adac8:     	pushq	%r14
1422adaca:     	pushq	%r13
1422adacc:     	pushq	%r12
1422adace:     	pushq	%rsi
1422adacf:     	pushq	%rdi
1422adad0:     	pushq	%rbx
1422adad1:     	subq	$0x38, %rsp
1422adad5:     	leaq	0x80(%rdx), %rbp
1422adadc:     	movaps	%xmm6, 0x20(%rsp)
1422adae1:     	movl	$0x15, %edx
1422adae6:     	movl	$0x1, %r8d
1422adaec:     	movq	-0x40(%rbp), %rcx
1422adaf0:     	callq	0x140613c20 <.text+0x612c20>
1422adaf5:     	movaps	0x20(%rsp), %xmm6
1422adafa:     	addq	$0x38, %rsp
1422adafe:     	popq	%rbx
1422adaff:     	popq	%rdi
1422adb00:     	popq	%rsi
1422adb01:     	popq	%r12
1422adb03:     	popq	%r13
1422adb05:     	popq	%r14
1422adb07:     	popq	%r15
1422adb09:     	popq	%rbp
1422adb0a:     	retq
1422adb0b:     	nopl	(%rax,%rax)
1422adb10:     	movq	%rdx, 0x10(%rsp)
1422adb15:     	pushq	%rbp
1422adb16:     	pushq	%r15
1422adb18:     	pushq	%r14
1422adb1a:     	pushq	%r13
1422adb1c:     	pushq	%r12
1422adb1e:     	pushq	%rsi
1422adb1f:     	pushq	%rdi
1422adb20:     	pushq	%rbx
1422adb21:     	subq	$0x38, %rsp
1422adb25:     	leaq	0x80(%rdx), %rbp
1422adb2c:     	movaps	%xmm6, 0x20(%rsp)
1422adb31:     	movl	$0x11, %edx
1422adb36:     	movl	$0x1, %r8d
1422adb3c:     	movq	-0x38(%rbp), %rcx
1422adb40:     	callq	0x140613c20 <.text+0x612c20>
1422adb45:     	movaps	0x20(%rsp), %xmm6
1422adb4a:     	addq	$0x38, %rsp
1422adb4e:     	popq	%rbx
1422adb4f:     	popq	%rdi
1422adb50:     	popq	%rsi
1422adb51:     	popq	%r12
1422adb53:     	popq	%r13
1422adb55:     	popq	%r14
1422adb57:     	popq	%r15
1422adb59:     	popq	%rbp
1422adb5a:     	retq
1422adb5b:     	nopl	(%rax,%rax)
1422adb60:     	movq	%rdx, 0x10(%rsp)
1422adb65:     	pushq	%rbp
1422adb66:     	pushq	%r15
1422adb68:     	pushq	%r14
1422adb6a:     	pushq	%r13
1422adb6c:     	pushq	%r12
1422adb6e:     	pushq	%rsi
1422adb6f:     	pushq	%rdi
1422adb70:     	pushq	%rbx
1422adb71:     	subq	$0x38, %rsp
1422adb75:     	leaq	0x80(%rdx), %rbp
1422adb7c:     	movaps	%xmm6, 0x20(%rsp)
1422adb81:     	movl	$0x11, %edx
1422adb86:     	movl	$0x1, %r8d
1422adb8c:     	movq	-0x30(%rbp), %rcx
1422adb90:     	callq	0x140613c20 <.text+0x612c20>
1422adb95:     	movaps	0x20(%rsp), %xmm6
1422adb9a:     	addq	$0x38, %rsp
1422adb9e:     	popq	%rbx
1422adb9f:     	popq	%rdi
1422adba0:     	popq	%rsi
1422adba1:     	popq	%r12
1422adba3:     	popq	%r13
1422adba5:     	popq	%r14
1422adba7:     	popq	%r15
1422adba9:     	popq	%rbp
1422adbaa:     	retq
1422adbab:     	nopl	(%rax,%rax)
1422adbb0:     	movq	%rdx, 0x10(%rsp)
1422adbb5:     	pushq	%rbp
1422adbb6:     	pushq	%r15
1422adbb8:     	pushq	%r14
1422adbba:     	pushq	%r13
1422adbbc:     	pushq	%r12
1422adbbe:     	pushq	%rsi
1422adbbf:     	pushq	%rdi
1422adbc0:     	pushq	%rbx
1422adbc1:     	subq	$0x38, %rsp
1422adbc5:     	leaq	0x80(%rdx), %rbp
1422adbcc:     	movaps	%xmm6, 0x20(%rsp)
1422adbd1:     	movl	$0x11, %edx
1422adbd6:     	movl	$0x1, %r8d
1422adbdc:     	movq	-0x28(%rbp), %rcx
1422adbe0:     	callq	0x140613c20 <.text+0x612c20>
1422adbe5:     	movaps	0x20(%rsp), %xmm6
1422adbea:     	addq	$0x38, %rsp
1422adbee:     	popq	%rbx
1422adbef:     	popq	%rdi
1422adbf0:     	popq	%rsi
1422adbf1:     	popq	%r12
1422adbf3:     	popq	%r13
1422adbf5:     	popq	%r14
1422adbf7:     	popq	%r15
1422adbf9:     	popq	%rbp
1422adbfa:     	retq
1422adbfb:     	nopl	(%rax,%rax)
1422adc00:     	movq	%rdx, 0x10(%rsp)
1422adc05:     	pushq	%rbp
1422adc06:     	pushq	%r15
1422adc08:     	pushq	%r14
1422adc0a:     	pushq	%r13
1422adc0c:     	pushq	%r12
1422adc0e:     	pushq	%rsi
1422adc0f:     	pushq	%rdi
1422adc10:     	pushq	%rbx
1422adc11:     	subq	$0x38, %rsp
1422adc15:     	leaq	0x80(%rdx), %rbp
1422adc1c:     	movaps	%xmm6, 0x20(%rsp)
1422adc21:     	movl	$0x11, %edx
1422adc26:     	movl	$0x1, %r8d
1422adc2c:     	movq	-0x20(%rbp), %rcx
1422adc30:     	callq	0x140613c20 <.text+0x612c20>
1422adc35:     	movaps	0x20(%rsp), %xmm6
1422adc3a:     	addq	$0x38, %rsp
1422adc3e:     	popq	%rbx
1422adc3f:     	popq	%rdi
1422adc40:     	popq	%rsi
1422adc41:     	popq	%r12
1422adc43:     	popq	%r13
1422adc45:     	popq	%r14
1422adc47:     	popq	%r15
1422adc49:     	popq	%rbp
1422adc4a:     	retq
1422adc4b:     	nopl	(%rax,%rax)
1422adc50:     	movq	%rdx, 0x10(%rsp)
1422adc55:     	pushq	%rbp
1422adc56:     	pushq	%r15
1422adc58:     	pushq	%r14
1422adc5a:     	pushq	%r13
1422adc5c:     	pushq	%r12
1422adc5e:     	pushq	%rsi
1422adc5f:     	pushq	%rdi
1422adc60:     	pushq	%rbx
1422adc61:     	subq	$0x38, %rsp
1422adc65:     	leaq	0x80(%rdx), %rbp
1422adc6c:     	movaps	%xmm6, 0x20(%rsp)
1422adc71:     	movl	$0x11, %edx
1422adc76:     	movl	$0x1, %r8d
1422adc7c:     	movq	-0x18(%rbp), %rcx
1422adc80:     	callq	0x140613c20 <.text+0x612c20>
1422adc85:     	movaps	0x20(%rsp), %xmm6
1422adc8a:     	addq	$0x38, %rsp
1422adc8e:     	popq	%rbx
1422adc8f:     	popq	%rdi
1422adc90:     	popq	%rsi
1422adc91:     	popq	%r12
1422adc93:     	popq	%r13
1422adc95:     	popq	%r14
1422adc97:     	popq	%r15
1422adc99:     	popq	%rbp
1422adc9a:     	retq
1422adc9b:     	nopl	(%rax,%rax)
1422adca0:     	movq	%rdx, 0x10(%rsp)
1422adca5:     	pushq	%rbp
1422adca6:     	pushq	%r15
1422adca8:     	pushq	%r14
1422adcaa:     	pushq	%r13
1422adcac:     	pushq	%r12
1422adcae:     	pushq	%rsi
1422adcaf:     	pushq	%rdi
1422adcb0:     	pushq	%rbx
1422adcb1:     	subq	$0x38, %rsp
1422adcb5:     	leaq	0x80(%rdx), %rbp
1422adcbc:     	movaps	%xmm6, 0x20(%rsp)
1422adcc1:     	movl	$0x11, %edx
1422adcc6:     	movl	$0x1, %r8d
1422adccc:     	movq	-0x10(%rbp), %rcx
1422adcd0:     	callq	0x140613c20 <.text+0x612c20>
1422adcd5:     	movaps	0x20(%rsp), %xmm6
1422adcda:     	addq	$0x38, %rsp
1422adcde:     	popq	%rbx
1422adcdf:     	popq	%rdi
1422adce0:     	popq	%rsi
1422adce1:     	popq	%r12
1422adce3:     	popq	%r13
1422adce5:     	popq	%r14
1422adce7:     	popq	%r15
1422adce9:     	popq	%rbp
1422adcea:     	retq
