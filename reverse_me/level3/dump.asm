Dump of assembler code for function main:
   0x0000555555555320 <+0>:	push   %rbp
   0x0000555555555321 <+1>:	mov    %rsp,%rbp
=> 0x0000555555555324 <+4>:	sub    $0x60,%rsp
   0x0000555555555328 <+8>:	movl   $0x0,-0x4(%rbp)
   0x000055555555532f <+15>:	lea    0xd0d(%rip),%rdi        # 0x555555556043
   0x0000555555555336 <+22>:	mov    $0x0,%al
   0x0000555555555338 <+24>:	call   0x555555555050 <printf@plt>
   0x000055555555533d <+29>:	lea    -0x40(%rbp),%rsi
   0x0000555555555341 <+33>:	lea    0xd0e(%rip),%rdi        # 0x555555556056
   0x0000555555555348 <+40>:	mov    $0x0,%al
   0x000055555555534a <+42>:	call   0x5555555550a0 <__isoc99_scanf@plt>
   0x000055555555534f <+47>:	mov    %eax,-0x8(%rbp)
   0x0000555555555352 <+50>:	mov    $0x1,%eax
   0x0000555555555357 <+55>:	cmp    -0x8(%rbp),%eax
   0x000055555555535a <+58>:	je     0x555555555365 <main+69>
   0x0000555555555360 <+64>:	call   0x5555555552e0 <___syscall_malloc>
   0x0000555555555365 <+69>:	movsbl -0x3f(%rbp),%ecx
   0x0000555555555369 <+73>:	mov    $0x32,%eax
   0x000055555555536e <+78>:	cmp    %ecx,%eax
   0x0000555555555370 <+80>:	je     0x55555555537b <main+91>
   0x0000555555555376 <+86>:	call   0x5555555552e0 <___syscall_malloc>
   0x000055555555537b <+91>:	movsbl -0x40(%rbp),%ecx
   0x000055555555537f <+95>:	mov    $0x34,%eax
   0x0000555555555384 <+100>:	cmp    %ecx,%eax
   0x0000555555555386 <+102>:	je     0x555555555391 <main+113>
   0x000055555555538c <+108>:	call   0x5555555552e0 <___syscall_malloc>
   0x0000555555555391 <+113>:	mov    0x2c48(%rip),%rax        # 0x555555557fe0
   0x0000555555555398 <+120>:	mov    (%rax),%rdi
   0x000055555555539b <+123>:	call   0x555555555080 <fflush@plt>
   0x00005555555553a0 <+128>:	lea    -0x21(%rbp),%rdi
   0x00005555555553a4 <+132>:	xor    %esi,%esi
   0x00005555555553a6 <+134>:	mov    $0x9,%edx
   0x00005555555553ab <+139>:	call   0x555555555060 <memset@plt>
   0x00005555555553b0 <+144>:	movb   $0x2a,-0x21(%rbp)
   0x00005555555553b4 <+148>:	movb   $0x0,-0x41(%rbp)
   0x00005555555553b8 <+152>:	movq   $0x2,-0x18(%rbp)
   0x00005555555553c0 <+160>:	movl   $0x1,-0xc(%rbp)
   0x00005555555553c7 <+167>:	lea    -0x21(%rbp),%rdi
   0x00005555555553cb <+171>:	call   0x555555555040 <strlen@plt>
   0x00005555555553d0 <+176>:	mov    %rax,%rcx
   0x00005555555553d3 <+179>:	xor    %eax,%eax
   0x00005555555553d5 <+181>:	cmp    $0x8,%rcx
   0x00005555555553d9 <+185>:	mov    %al,-0x45(%rbp)
   0x00005555555553dc <+188>:	jae    0x555555555403 <main+227>
   0x00005555555553e2 <+194>:	mov    -0x18(%rbp),%rax
   0x00005555555553e6 <+198>:	mov    %rax,-0x50(%rbp)
   0x00005555555553ea <+202>:	lea    -0x40(%rbp),%rdi
   0x00005555555553ee <+206>:	call   0x555555555040 <strlen@plt>
   0x00005555555553f3 <+211>:	mov    %rax,%rcx
   0x00005555555553f6 <+214>:	mov    -0x50(%rbp),%rax
   0x00005555555553fa <+218>:	cmp    %rcx,%rax
   0x00005555555553fd <+221>:	setb   %al
   0x0000555555555400 <+224>:	mov    %al,-0x45(%rbp)
   0x0000555555555403 <+227>:	mov    -0x45(%rbp),%al
   0x0000555555555406 <+230>:	test   $0x1,%al
   0x0000555555555408 <+232>:	jne    0x555555555413 <main+243>
   0x000055555555540e <+238>:	jmp    0x555555555461 <main+321>
   0x0000555555555413 <+243>:	mov    -0x18(%rbp),%rax
   0x0000555555555417 <+247>:	mov    -0x40(%rbp,%rax,1),%al
   0x000055555555541b <+251>:	mov    %al,-0x44(%rbp)
   0x000055555555541e <+254>:	mov    -0x18(%rbp),%rax
   0x0000555555555422 <+258>:	mov    -0x3f(%rbp,%rax,1),%al
   0x0000555555555426 <+262>:	mov    %al,-0x43(%rbp)
   0x0000555555555429 <+265>:	mov    -0x18(%rbp),%rax
   0x000055555555542d <+269>:	mov    -0x3e(%rbp,%rax,1),%al
   0x0000555555555431 <+273>:	mov    %al,-0x42(%rbp)
   0x0000555555555434 <+276>:	lea    -0x44(%rbp),%rdi
   0x0000555555555438 <+280>:	call   0x555555555090 <atoi@plt>
   0x000055555555543d <+285>:	mov    %al,%cl
   0x000055555555543f <+287>:	movslq -0xc(%rbp),%rax
   0x0000555555555443 <+291>:	mov    %cl,-0x21(%rbp,%rax,1)
   0x0000555555555447 <+295>:	mov    -0x18(%rbp),%rax
   0x000055555555544b <+299>:	add    $0x3,%rax
   0x000055555555544f <+303>:	mov    %rax,-0x18(%rbp)
   0x0000555555555453 <+307>:	mov    -0xc(%rbp),%eax
   0x0000555555555456 <+310>:	add    $0x1,%eax
   0x0000555555555459 <+313>:	mov    %eax,-0xc(%rbp)
   0x000055555555545c <+316>:	jmp    0x5555555553c7 <main+167>
   0x0000555555555461 <+321>:	movslq -0xc(%rbp),%rax
   0x0000555555555465 <+325>:	movb   $0x0,-0x21(%rbp,%rax,1)
   0x000055555555546a <+330>:	lea    0xb93(%rip),%rsi        # 0x555555556004
   0x0000555555555471 <+337>:	lea    -0x21(%rbp),%rdi
   0x0000555555555475 <+341>:	call   0x555555555070 <strcmp@plt>
   0x000055555555547a <+346>:	mov    %eax,-0x10(%rbp)
   0x000055555555547d <+349>:	mov    -0x10(%rbp),%eax
   0x0000555555555480 <+352>:	mov    %eax,-0x54(%rbp)
   0x0000555555555483 <+355>:	sub    $0xfffffffe,%eax
   0x0000555555555486 <+358>:	je     0x555555555536 <main+534>
   0x000055555555548c <+364>:	jmp    0x555555555491 <main+369>
   0x0000555555555491 <+369>:	mov    -0x54(%rbp),%eax
   0x0000555555555494 <+372>:	sub    $0xffffffff,%eax
   0x0000555555555497 <+375>:	je     0x55555555552c <main+524>
   0x000055555555549d <+381>:	jmp    0x5555555554a2 <main+386>
   0x00005555555554a2 <+386>:	mov    -0x54(%rbp),%eax
   0x00005555555554a5 <+389>:	test   %eax,%eax
   0x00005555555554a7 <+391>:	je     0x55555555555e <main+574>
   0x00005555555554ad <+397>:	jmp    0x5555555554b2 <main+402>
   0x00005555555554b2 <+402>:	mov    -0x54(%rbp),%eax
   0x00005555555554b5 <+405>:	sub    $0x1,%eax
   0x00005555555554b8 <+408>:	je     0x555555555518 <main+504>
   0x00005555555554be <+414>:	jmp    0x5555555554c3 <main+419>
   0x00005555555554c3 <+419>:	mov    -0x54(%rbp),%eax
   0x00005555555554c6 <+422>:	sub    $0x2,%eax
   0x00005555555554c9 <+425>:	je     0x555555555522 <main+514>
   0x00005555555554cf <+431>:	jmp    0x5555555554d4 <main+436>
   0x00005555555554d4 <+436>:	mov    -0x54(%rbp),%eax
   0x00005555555554d7 <+439>:	sub    $0x3,%eax
   0x00005555555554da <+442>:	je     0x555555555540 <main+544>
   0x00005555555554e0 <+448>:	jmp    0x5555555554e5 <main+453>
   0x00005555555554e5 <+453>:	mov    -0x54(%rbp),%eax
   0x00005555555554e8 <+456>:	sub    $0x4,%eax
   0x00005555555554eb <+459>:	je     0x55555555554a <main+554>
   0x00005555555554f1 <+465>:	jmp    0x5555555554f6 <main+470>
   0x00005555555554f6 <+470>:	mov    -0x54(%rbp),%eax
   0x00005555555554f9 <+473>:	sub    $0x5,%eax
   0x00005555555554fc <+476>:	je     0x555555555554 <main+564>
   0x0000555555555502 <+482>:	jmp    0x555555555507 <main+487>
   0x0000555555555507 <+487>:	mov    -0x54(%rbp),%eax
   0x000055555555550a <+490>:	sub    $0x73,%eax
   0x000055555555550d <+493>:	je     0x555555555568 <main+584>
   0x0000555555555513 <+499>:	jmp    0x555555555572 <main+594>
   0x0000555555555518 <+504>:	call   0x5555555552e0 <___syscall_malloc>
   0x000055555555551d <+509>:	jmp    0x555555555577 <main+599>
   0x0000555555555522 <+514>:	call   0x5555555552e0 <___syscall_malloc>
   0x0000555555555527 <+519>:	jmp    0x555555555577 <main+599>
   0x000055555555552c <+524>:	call   0x5555555552e0 <___syscall_malloc>
   0x0000555555555531 <+529>:	jmp    0x555555555577 <main+599>
   0x0000555555555536 <+534>:	call   0x5555555552e0 <___syscall_malloc>
   0x000055555555553b <+539>:	jmp    0x555555555577 <main+599>
   0x0000555555555540 <+544>:	call   0x5555555552e0 <___syscall_malloc>
   0x0000555555555545 <+549>:	jmp    0x555555555577 <main+599>
   0x000055555555554a <+554>:	call   0x5555555552e0 <___syscall_malloc>
   0x000055555555554f <+559>:	jmp    0x555555555577 <main+599>
   0x0000555555555554 <+564>:	call   0x5555555552e0 <___syscall_malloc>
   0x0000555555555559 <+569>:	jmp    0x555555555577 <main+599>
   0x000055555555555e <+574>:	call   0x555555555300 <____syscall_malloc>
   0x0000555555555563 <+579>:	jmp    0x555555555577 <main+599>
   0x0000555555555568 <+584>:	call   0x5555555552e0 <___syscall_malloc>
   0x000055555555556d <+589>:	jmp    0x555555555577 <main+599>
   0x0000555555555572 <+594>:	call   0x5555555552e0 <___syscall_malloc>
   0x0000555555555577 <+599>:	xor    %eax,%eax
   0x0000555555555579 <+601>:	add    $0x60,%rsp
   0x000055555555557d <+605>:	pop    %rbp
   0x000055555555557e <+606>:	ret    
End of assembler dump.
