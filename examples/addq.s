.global "foo"
"foo":
  ".Lfoo_entry":
    pushq %rbp
    movq %rsp, %rbp
    subq $16, %rsp
    movq %rdi, -8(%rbp)
    movq %rsi, -16(%rbp)
    movq -8(%rbp), %r10
    addq -16(%rbp), %r10
    movq %r10, %rax
    movq %rbp, %rsp
    popq %rbp
    ret

  .section .rodata
  .text
.global "main"
"main":
  ".Lmain_entry":
    pushq %rbp
    movq %rsp, %rbp
    subq $0, %rsp
    movq $10, %rdi
    movq $20, %rsi
    call "foo"
    movq %rax, %r10
    movq %r10, %rdi
    call "x64::exit_with"
    movq %rax, %r11
    movq %rbp, %rsp
    popq %rbp
    ret

  .section .rodata
  .text
.global "x64::exit_with"
"x64::exit_with":
  ".Lx64::exit_with_entry":
    pushq %rbp
    movq %rsp, %rbp
    subq $8, %rsp
    movq %rdi, -8(%rbp)
    movq $60, %rax
    syscall
    movq %rbp, %rsp
    popq %rbp
    ret

  .section .rodata
  .text
