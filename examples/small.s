.global "main"
"main":
  ".Lmain_entry":
    pushq %rbp
    movq %rsp, %rbp
    subq $0, %rsp
    movq $42, %rdi
    call "x64::exit_with"
    movq %rax, %r10
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
.global "x64::write"
"x64::write":
  ".Lx64::write_entry":
    pushq %rbp
    movq %rsp, %rbp
    subq $24, %rsp
    movq %rdi, -24(%rbp)
    movq %rsi, -8(%rbp)
    movq %rdx, -16(%rbp)
    movq $1, %rax
    syscall
    movq %rbp, %rsp
    popq %rbp
    ret

  .section .rodata
  .text