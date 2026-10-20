140553030 push rbp
140553031 push rsi
140553032 push rdi
140553033 push rbx
140553034 sub rsp, 0x988
14055303b lea rbp, [rsp + 0x80]
140553043 mov qword ptr [rbp + 0x900], 0xfffffffffffffffe
14055304e mov rsi, rcx
140553051 movzx eax, byte ptr [rip + 0x30e1bb1]
140553058 mov ecx, 0x380
14055305d mov edx, 8
140553062 call 0x140613c10
140553067 test rax, rax
14055306a je 0x1405535b4
140553070 mov qword ptr [rbp + 0x8f8], rax
140553077 xorps xmm0, xmm0
14055307a movaps xmmword ptr [rbp + 0x4c0], xmm0
140553081 movups xmmword ptr [rbp + 0x4d8], xmm0
140553088 movaps xmmword ptr [rbp + 0x4f0], xmm0
14055308f movups xmmword ptr [rbp + 0x508], xmm0
140553096 movaps xmmword ptr [rbp + 0x520], xmm0
14055309d mov dword ptr [rbp + 0x530], 0
1405530a7 movups xmmword ptr [rbp + 0x558], xmm0
1405530ae movaps xmmword ptr [rbp + 0x570], xmm0
1405530b5 movups xmmword ptr [rbp + 0x588], xmm0
1405530bc movaps xmmword ptr [rbp + 0x5a0], xmm0
1405530c3 movups xmmword ptr [rbp + 0x5b8], xmm0
1405530ca mov dword ptr [rbp + 0x5c8], 0
1405530d4 mov qword ptr [rbp + 0x10], 3
1405530dc mov qword ptr [rbp + 0x4a0], 0
1405530e7 mov qword ptr [rbp + 0x4a8], 8
1405530f2 mov qword ptr [rbp + 0x4b0], 0
1405530fd mov qword ptr [rbp + 0x4b8], 0x10
140553108 mov qword ptr [rbp + 0x4d0], 0x10
140553113 mov qword ptr [rbp + 0x4e8], 0x10
14055311e mov qword ptr [rbp + 0x500], 0x10
140553129 mov qword ptr [rbp + 0x518], 0x10
140553134 movabs rbx, 0x8000000000000000
14055313e mov qword ptr [rbp + 0x538], rbx
140553145 lea rax, [rip + 0x24a72a1]
14055314c mov qword ptr [rbp + 0x540], rax
140553153 mov qword ptr [rbp + 0x548], 0x42
14055315e mov qword ptr [rbp + 0x550], 0x10
140553169 mov qword ptr [rbp + 0x568], 0x10
140553174 mov qword ptr [rbp + 0x580], 0x10
14055317f mov qword ptr [rbp + 0x598], 0x10
14055318a mov qword ptr [rbp + 0x5b0], 0x10
140553195 mov dword ptr [rbp + 0x5d0], 0
14055319f mov word ptr [rbp + 0x5d4], 1
1405531a8 mov dword ptr [rbp + 0x5d8], 0
1405531b2 movzx eax, byte ptr [rip + 0x30e1a50]
1405531b9 mov ecx, 0x640
1405531be mov edx, 0x10
1405531c3 call 0x140613c10
1405531c8 test rax, rax
1405531cb je 0x1405535c3
1405531d1 mov rdi, rax
1405531d4 lea rdx, [rbp - 0x60]
1405531d8 mov r8d, 0x640
1405531de mov rcx, rax
1405531e1 call 0x142923c00
1405531e6 lea r8, [rip + 0x24a712b]
1405531ed lea rcx, [rbp + 0x888]
1405531f4 mov rdx, rdi
1405531f7 call 0x140ce4b10
1405531fc lea rcx, [rbp + 0x818]
140553203 call 0x1400342b0
140553208 lea rcx, [rbp + 0x7a8]
14055320f call 0x1402c7b10
140553214 lea rcx, [rbp + 0x738]
14055321b call 0x1404882d0
140553220 lea rcx, [rbp + 0x6c8]
140553227 call 0x14026a360
14055322c lea rcx, [rbp + 0x658]
140553233 call 0x140100dc0
140553238 lea rcx, [rbp + 0x5e8]
14055323f call 0x1404898c0
140553244 lea rcx, [rbp - 0x60]
140553248 call 0x140274560
14055324d movups xmm0, xmmword ptr [rbp + 0x8e8]
140553254 mov rax, qword ptr [rbp + 0x8f8]
14055325b movups xmmword ptr [rax + 0x60], xmm0
14055325f movups xmm0, xmmword ptr [rbp + 0x8d8]
140553266 movups xmmword ptr [rax + 0x50], xmm0
14055326a movups xmm0, xmmword ptr [rbp + 0x8c8]
140553271 movups xmmword ptr [rax + 0x40], xmm0
140553275 movups xmm0, xmmword ptr [rbp + 0x888]
14055327c movups xmm1, xmmword ptr [rbp + 0x898]
140553283 movups xmm2, xmmword ptr [rbp + 0x8a8]
14055328a movups xmm3, xmmword ptr [rbp + 0x8b8]
140553291 movups xmmword ptr [rax + 0x30], xmm3
140553295 movups xmmword ptr [rax + 0x20], xmm2
140553299 movups xmmword ptr [rax + 0x10], xmm1
14055329d movups xmmword ptr [rax], xmm0
1405532a0 movups xmm0, xmmword ptr [rbp + 0x818]
1405532a7 movups xmm1, xmmword ptr [rbp + 0x828]
1405532ae movups xmm2, xmmword ptr [rbp + 0x838]
1405532b5 movups xmm3, xmmword ptr [rbp + 0x848]
1405532bc movups xmmword ptr [rax + 0x70], xmm0
1405532c0 movups xmmword ptr [rax + 0x80], xmm1
1405532c7 movups xmmword ptr [rax + 0x90], xmm2
1405532ce movups xmmword ptr [rax + 0xa0], xmm3
1405532d5 movups xmm0, xmmword ptr [rbp + 0x858]
1405532dc movups xmmword ptr [rax + 0xb0], xmm0
1405532e3 movups xmm0, xmmword ptr [rbp + 0x868]
1405532ea movups xmmword ptr [rax + 0xc0], xmm0
1405532f1 movups xmm0, xmmword ptr [rbp + 0x878]
1405532f8 movups xmmword ptr [rax + 0xd0], xmm0
1405532ff movups xmm0, xmmword ptr [rbp + 0x808]
140553306 movups xmmword ptr [rax + 0x140], xmm0
14055330d movups xmm0, xmmword ptr [rbp + 0x7f8]
140553314 movups xmmword ptr [rax + 0x130], xmm0
14055331b movups xmm0, xmmword ptr [rbp + 0x7e8]
140553322 movups xmmword ptr [rax + 0x120], xmm0
140553329 movups xmm0, xmmword ptr [rbp + 0x7a8]
140553330 movups xmm1, xmmword ptr [rbp + 0x7b8]
140553337 movups xmm2, xmmword ptr [rbp + 0x7c8]
14055333e movups xmm3, xmmword ptr [rbp + 0x7d8]
140553345 movups xmmword ptr [rax + 0x110], xmm3
14055334c movups xmmword ptr [rax + 0x100], xmm2
140553353 movups xmmword ptr [rax + 0xf0], xmm1
14055335a movups xmmword ptr [rax + 0xe0], xmm0
140553361 movups xmm0, xmmword ptr [rbp + 0x738]
140553368 movups xmm1, xmmword ptr [rbp + 0x748]
14055336f movups xmm2, xmmword ptr [rbp + 0x758]
140553376 movups xmm3, xmmword ptr [rbp + 0x768]
14055337d movups xmmword ptr [rax + 0x150], xmm0
140553384 movups xmmword ptr [rax + 0x160], xmm1
14055338b movups xmmword ptr [rax + 0x170], xmm2
140553392 movups xmmword ptr [rax + 0x180], xmm3
140553399 movups xmm0, xmmword ptr [rbp + 0x778]
1405533a0 movups xmmword ptr [rax + 0x190], xmm0
1405533a7 movups xmm0, xmmword ptr [rbp + 0x788]
1405533ae movups xmmword ptr [rax + 0x1a0], xmm0
1405533b5 movups xmm0, xmmword ptr [rbp + 0x798]
1405533bc movups xmmword ptr [rax + 0x1b0], xmm0
1405533c3 movups xmm0, xmmword ptr [rbp + 0x6c8]
1405533ca movups xmm1, xmmword ptr [rbp + 0x6d8]
1405533d1 movups xmm2, xmmword ptr [rbp + 0x6e8]
1405533d8 movups xmm3, xmmword ptr [rbp + 0x6f8]
1405533df movups xmmword ptr [rax + 0x1c0], xmm0
1405533e6 movups xmmword ptr [rax + 0x1d0], xmm1
1405533ed movups xmmword ptr [rax + 0x1e0], xmm2
1405533f4 movups xmmword ptr [rax + 0x1f0], xmm3
1405533fb movups xmm0, xmmword ptr [rbp + 0x708]
140553402 movups xmmword ptr [rax + 0x200], xmm0
140553409 movups xmm0, xmmword ptr [rbp + 0x728]
140553410 movups xmmword ptr [rax + 0x220], xmm0
140553417 movups xmm0, xmmword ptr [rbp + 0x718]
14055341e movups xmmword ptr [rax + 0x210], xmm0
140553425 movups xmm0, xmmword ptr [rbp + 0x6b8]
14055342c movups xmmword ptr [rax + 0x290], xmm0
140553433 movups xmm0, xmmword ptr [rbp + 0x6a8]
14055343a movups xmmword ptr [rax + 0x280], xmm0
140553441 movups xmm0, xmmword ptr [rbp + 0x698]
140553448 movups xmmword ptr [rax + 0x270], xmm0
14055344f movups xmm0, xmmword ptr [rbp + 0x658]
140553456 movups xmm1, xmmword ptr [rbp + 0x668]
14055345d movups xmm2, xmmword ptr [rbp + 0x678]
140553464 movups xmm3, xmmword ptr [rbp + 0x688]
14055346b movups xmmword ptr [rax + 0x260], xmm3
140553472 movups xmmword ptr [rax + 0x250], xmm2
140553479 movups xmmword ptr [rax + 0x240], xmm1
140553480 movups xmmword ptr [rax + 0x230], xmm0
140553487 movups xmm0, xmmword ptr [rbp + 0x5e8]
14055348e movups xmm1, xmmword ptr [rbp + 0x5f8]
140553495 movups xmm2, xmmword ptr [rbp + 0x608]
14055349c movups xmm3, xmmword ptr [rbp + 0x618]
1405534a3 movups xmmword ptr [rax + 0x2a0], xmm0
1405534aa movups xmmword ptr [rax + 0x2b0], xmm1
1405534b1 movups xmmword ptr [rax + 0x2c0], xmm2
1405534b8 movups xmmword ptr [rax + 0x2d0], xmm3
1405534bf movups xmm0, xmmword ptr [rbp + 0x628]
1405534c6 movups xmmword ptr [rax + 0x2e0], xmm0
1405534cd movups xmm0, xmmword ptr [rbp + 0x638]
1405534d4 movups xmmword ptr [rax + 0x2f0], xmm0
1405534db movups xmm0, xmmword ptr [rbp + 0x648]
1405534e2 movups xmmword ptr [rax + 0x300], xmm0
1405534e9 movups xmm0, xmmword ptr [rbp - 0x60]
1405534ed movups xmm1, xmmword ptr [rbp - 0x50]
1405534f1 movups xmm2, xmmword ptr [rbp - 0x40]
1405534f5 movups xmm3, xmmword ptr [rbp - 0x30]
1405534f9 movups xmmword ptr [rax + 0x310], xmm0
140553500 movups xmmword ptr [rax + 0x320], xmm1
140553507 movups xmmword ptr [rax + 0x330], xmm2
14055350e movups xmmword ptr [rax + 0x340], xmm3
140553515 movups xmm0, xmmword ptr [rbp - 0x20]
140553519 movups xmmword ptr [rax + 0x350], xmm0
140553520 movups xmm0, xmmword ptr [rbp]
140553524 movups xmmword ptr [rax + 0x370], xmm0
14055352b movups xmm0, xmmword ptr [rbp - 0x10]
14055352f movups xmmword ptr [rax + 0x360], xmm0
140553536 mov qword ptr [rbp - 0x58], 8
14055353e mov qword ptr [rbp - 0x50], rax
140553542 mov qword ptr [rbp - 0x48], 8
14055354a mov qword ptr [rbp - 0x40], 0
140553552 mov qword ptr [rbp - 0x38], 8
14055355a xorps xmm0, xmm0
14055355d movups xmmword ptr [rbp - 0x30], xmm0
140553561 mov qword ptr [rbp - 0x60], rbx
140553565 lea rcx, [rbp - 0x28]
140553569 call 0x140d2fa20
14055356e movups xmm0, xmmword ptr [rbp]
140553572 movups xmmword ptr [rsi + 0x60], xmm0
140553576 movups xmm0, xmmword ptr [rbp - 0x10]
14055357a movups xmmword ptr [rsi + 0x50], xmm0
14055357e movups xmm0, xmmword ptr [rbp - 0x20]
140553582 movups xmmword ptr [rsi + 0x40], xmm0
140553586 movups xmm0, xmmword ptr [rbp - 0x60]
14055358a movups xmm1, xmmword ptr [rbp - 0x50]
14055358e movups xmm2, xmmword ptr [rbp - 0x40]
140553592 movups xmm3, xmmword ptr [rbp - 0x30]
140553596 movups xmmword ptr [rsi + 0x30], xmm3
14055359a movups xmmword ptr [rsi + 0x20], xmm2
14055359e movups xmmword ptr [rsi + 0x10], xmm1
1405535a2 movups xmmword ptr [rsi], xmm0
1405535a5 mov rax, rsi
1405535a8 add rsp, 0x988
1405535af pop rbx
1405535b0 pop rdi
1405535b1 pop rsi
1405535b2 pop rbp
1405535b3 ret 
1405535b4 mov ecx, 8
1405535b9 mov edx, 0x380
1405535be call 0x1428d9000
1405535c3 mov ecx, 0x10
1405535c8 mov edx, 0x640
1405535cd call 0x1428d9000
1405535d2 ud2 
1405535d4 nop word ptr cs:[rax + rax]
1405535e0 mov qword ptr [rsp + 0x10], rdx
1405535e5 push rbp
1405535e6 push rsi
1405535e7 push rdi
1405535e8 push rbx
1405535e9 sub rsp, 0x28
1405535ed lea rbp, [rdx + 0x80]
1405535f4 lea rcx, [rbp - 0x60]
1405535f8 call 0x1405eafe0
1405535fd nop 
1405535fe add rsp, 0x28
140553602 pop rbx
140553603 pop rdi
140553604 pop rsi
140553605 pop rbp
140553606 ret 
140553607 nop word ptr [rax + rax]
140553610 mov qword ptr [rsp + 0x10], rdx
140553615 push rbp
140553616 push rsi
140553617 push rdi
140553618 push rbx
140553619 sub rsp, 0x28
14055361d lea rbp, [rdx + 0x80]
140553624 mov edx, 0x380
140553629 mov r8d, 8
14055362f mov rcx, qword ptr [rbp + 0x8f8]
140553636 call 0x140613c20
14055363b nop 
14055363c add rsp, 0x28
140553640 pop rbx
140553641 pop rdi
140553642 pop rsi
140553643 pop rbp
140553644 ret 
140553645 nop word ptr cs:[rax + rax]
140553650 mov qword ptr [rsp + 0x10], rdx
140553655 push rbp
140553656 push rsi
140553657 push rdi
140553658 push rbx
140553659 sub rsp, 0x28
14055365d lea rbp, [rdx + 0x80]
140553664 lea rcx, [rbp + 0x888]
14055366b call 0x140029880
140553670 nop 
140553671 add rsp, 0x28
140553675 pop rbx
140553676 pop rdi
140553677 pop rsi
140553678 pop rbp
140553679 ret 
14055367a nop word ptr [rax + rax]
140553680 mov qword ptr [rsp + 0x10], rdx
140553685 push rbp
140553686 push rsi
140553687 push rdi
140553688 push rbx
140553689 sub rsp, 0x28
14055368d lea rbp, [rdx + 0x80]
140553694 lea rcx, [rbp + 0x818]
14055369b call 0x140029880
1405536a0 nop 
1405536a1 add rsp, 0x28
1405536a5 pop rbx
1405536a6 pop rdi
1405536a7 pop rsi
1405536a8 pop rbp
1405536a9 ret 
1405536aa nop word ptr [rax + rax]
1405536b0 mov qword ptr [rsp + 0x10], rdx
1405536b5 push rbp
1405536b6 push rsi
1405536b7 push rdi
1405536b8 push rbx
1405536b9 sub rsp, 0x28
1405536bd lea rbp, [rdx + 0x80]
1405536c4 lea rcx, [rbp + 0x7a8]
1405536cb call 0x140029880
1405536d0 nop 
1405536d1 add rsp, 0x28
1405536d5 pop rbx
1405536d6 pop rdi
1405536d7 pop rsi
1405536d8 pop rbp
1405536d9 ret 
1405536da nop word ptr [rax + rax]
1405536e0 mov qword ptr [rsp + 0x10], rdx
1405536e5 push rbp
1405536e6 push rsi
1405536e7 push rdi
1405536e8 push rbx
1405536e9 sub rsp, 0x28
1405536ed lea rbp, [rdx + 0x80]
1405536f4 lea rcx, [rbp + 0x738]
1405536fb call 0x140029880
140553700 nop 
140553701 add rsp, 0x28
140553705 pop rbx
140553706 pop rdi
140553707 pop rsi
140553708 pop rbp
140553709 ret 
14055370a nop word ptr [rax + rax]
140553710 mov qword ptr [rsp + 0x10], rdx
140553715 push rbp
140553716 push rsi
140553717 push rdi
140553718 push rbx
140553719 sub rsp, 0x28
14055371d lea rbp, [rdx + 0x80]
140553724 lea rcx, [rbp + 0x6c8]
14055372b call 0x140029880
140553730 nop 
140553731 add rsp, 0x28
140553735 pop rbx
140553736 pop rdi
140553737 pop rsi
140553738 pop rbp
140553739 ret 
14055373a nop word ptr [rax + rax]
140553740 mov qword ptr [rsp + 0x10], rdx
140553745 push rbp
140553746 push rsi
140553747 push rdi
140553748 push rbx
140553749 sub rsp, 0x28
14055374d lea rbp, [rdx + 0x80]
140553754 lea rcx, [rbp + 0x658]
14055375b call 0x140029880
140553760 nop 
140553761 add rsp, 0x28
140553765 pop rbx
140553766 pop rdi
140553767 pop rsi
140553768 pop rbp
140553769 ret 
14055376a nop word ptr [rax + rax]
140553770 mov qword ptr [rsp + 0x10], rdx
140553775 push rbp
140553776 push rsi
140553777 push rdi
140553778 push rbx
140553779 sub rsp, 0x28
14055377d lea rbp, [rdx + 0x80]
140553784 lea rcx, [rbp + 0x5e8]
14055378b call 0x140029880
140553790 nop 
140553791 add rsp, 0x28
140553795 pop rbx
140553796 pop rdi
140553797 pop rsi
140553798 pop rbp
140553799 ret 
14055379a nop word ptr [rax + rax]
1405537a0 mov qword ptr [rsp + 0x10], rdx
1405537a5 push rbp
1405537a6 push rsi
1405537a7 push rdi
1405537a8 push rbx
1405537a9 sub rsp, 0x28
1405537ad lea rbp, [rdx + 0x80]
1405537b4 lea rcx, [rbp - 0x60]
1405537b8 call 0x140029880
1405537bd nop 
1405537be add rsp, 0x28
1405537c2 pop rbx
1405537c3 pop rdi
1405537c4 pop rsi
1405537c5 pop rbp
1405537c6 ret 