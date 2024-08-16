target triple = "x86_64-pc-linux-gnu"

@.str = constant [4 x i8] c"%c\0A\00"
@.help = constant [5 x i8] c"%ld\0A\00"
@buff = global [10 x i8] zeroinitializer

define i32 @main() {
  ; get an element at an address & store / load information there

  %1 = getelementptr [10 x i8], ptr @buff, i64 0, i32 0 ; compute indexed address
  store i8 104, ptr %1 ; store to address

  %2 = getelementptr [10 x i8], ptr %1, i64 0, i32 1    ; compute indexed address
  store i8 101, ptr %2 ; store to address

  %3 = getelementptr [10 x i8], ptr %2, i64 0, i32 1    ; compute indexed address
  store i8 108, ptr %3 ; store to address

  %4 = getelementptr [10 x i8], ptr %3, i64 0, i32 1    ; compute indexed address
  store i8 108, ptr %4 ; store to address

  %5 = getelementptr [10 x i8], ptr %4, i64 0, i32 1    ; compute indexed address
  store i8 110, ptr %5 ; store to address

  ; add 1 to what is stored at 5
  %6 = load i8, ptr %5
  %7 = add i8 1, %6
  store i8 %7, ptr %5

  %8 = getelementptr [10 x i8], ptr %5, i64 0, i32 1    ; compute indexed address
  store i8 10, ptr %8 ; store to address

  call i32 @printf(ptr @buff)

  ; %5 = load i8, ptr %1 ; load from address
  ; call i32 @printf(ptr @.str, i8 %5)

  ret i32 0
}

declare i32 @printf(ptr noundef, ...)



; ; make dp & zero it
; %1 = alloca i64
; store i64 0, ptr %1

; ; load dp into 2
; %2 = load i64, ptr %1


; make dp & zero it
; %1 = alloca i64
; store i64 0, ptr %1
