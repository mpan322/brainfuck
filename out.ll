target triple = "x86_64-pc-linux-gnu"
@.format = constant [4 x i8] c"%c\0A\00"
@.buff = global [10 x i8] zeroinitializer
@buff = global ptr @.buff
define i32 @main() {
%1 = load ptr, ptr @buff

; increment the value
%2 = load i8, ptr %1
%3 = add nuw i8 10, %2
store i8 %3, ptr %1

; add to data pointer
%4 = getelementptr [10 x i8], ptr %1, i64 0, i32 1

; increment the value
%5 = load i8, ptr %4
%6 = add nuw i8 7, %5
store i8 %6, ptr %4

; add to data pointer
%7 = getelementptr [10 x i8], ptr %4, i64 0, i32 1

; increment the value
%8 = load i8, ptr %7
%9 = add nuw i8 10, %8
store i8 %9, ptr %7

; add to data pointer
%10 = getelementptr [10 x i8], ptr %7, i64 0, i32 1

; increment the value
%11 = load i8, ptr %10
%12 = add nuw i8 3, %11
store i8 %12, ptr %10

; add to data pointer
%13 = getelementptr [10 x i8], ptr %10, i64 0, i32 1

; increment the value
%14 = load i8, ptr %13
%15 = add nuw i8 1, %14
store i8 %15, ptr %13

; subtract from data pointer
%16 = getelementptr [10 x i8], ptr %13, i64 0, i32 -4

; decrement the value
%17 = load i8, ptr %16
%18 = sub nuw i8 1, %17
store i8 %18, ptr %16

; add to data pointer
%19 = getelementptr [10 x i8], ptr %16, i64 0, i32 1

; increment the value
%20 = load i8, ptr %19
%21 = add nuw i8 2, %20
store i8 %21, ptr %19

; print under data pointer
%22 = load i8, ptr %19
%23 = call i32 (ptr, ...) @printf(ptr @.format, i8 %22)

; add to data pointer
%24 = getelementptr [10 x i8], ptr %19, i64 0, i32 1

; increment the value
%25 = load i8, ptr %24
%26 = add nuw i8 1, %25
store i8 %26, ptr %24

; print under data pointer
%27 = load i8, ptr %24
%28 = call i32 (ptr, ...) @printf(ptr @.format, i8 %27)

; increment the value
%29 = load i8, ptr %24
%30 = add nuw i8 7, %29
store i8 %30, ptr %24

; print under data pointer
%31 = load i8, ptr %24
%32 = call i32 (ptr, ...) @printf(ptr @.format, i8 %31)

; print under data pointer
%33 = load i8, ptr %24
%34 = call i32 (ptr, ...) @printf(ptr @.format, i8 %33)

; increment the value
%35 = load i8, ptr %24
%36 = add nuw i8 3, %35
store i8 %36, ptr %24

; print under data pointer
%37 = load i8, ptr %24
%38 = call i32 (ptr, ...) @printf(ptr @.format, i8 %37)

; add to data pointer
%39 = getelementptr [10 x i8], ptr %24, i64 0, i32 1

; increment the value
%40 = load i8, ptr %39
%41 = add nuw i8 2, %40
store i8 %41, ptr %39

; print under data pointer
%42 = load i8, ptr %39
%43 = call i32 (ptr, ...) @printf(ptr @.format, i8 %42)

; subtract from data pointer
%44 = getelementptr [10 x i8], ptr %39, i64 0, i32 -2

; increment the value
%45 = load i8, ptr %44
%46 = add nuw i8 15, %45
store i8 %46, ptr %44

; print under data pointer
%47 = load i8, ptr %44
%48 = call i32 (ptr, ...) @printf(ptr @.format, i8 %47)

; add to data pointer
%49 = getelementptr [10 x i8], ptr %44, i64 0, i32 1

; print under data pointer
%50 = load i8, ptr %49
%51 = call i32 (ptr, ...) @printf(ptr @.format, i8 %50)

; increment the value
%52 = load i8, ptr %49
%53 = add nuw i8 3, %52
store i8 %53, ptr %49

; print under data pointer
%54 = load i8, ptr %49
%55 = call i32 (ptr, ...) @printf(ptr @.format, i8 %54)

; decrement the value
%56 = load i8, ptr %49
%57 = sub nuw i8 6, %56
store i8 %57, ptr %49

; print under data pointer
%58 = load i8, ptr %49
%59 = call i32 (ptr, ...) @printf(ptr @.format, i8 %58)

; decrement the value
%60 = load i8, ptr %49
%61 = sub nuw i8 8, %60
store i8 %61, ptr %49

; print under data pointer
%62 = load i8, ptr %49
%63 = call i32 (ptr, ...) @printf(ptr @.format, i8 %62)

; add to data pointer
%64 = getelementptr [10 x i8], ptr %49, i64 0, i32 1

; increment the value
%65 = load i8, ptr %64
%66 = add nuw i8 1, %65
store i8 %66, ptr %64

; print under data pointer
%67 = load i8, ptr %64
%68 = call i32 (ptr, ...) @printf(ptr @.format, i8 %67)

; add to data pointer
%69 = getelementptr [10 x i8], ptr %64, i64 0, i32 1

; print under data pointer
%70 = load i8, ptr %69
%71 = call i32 (ptr, ...) @printf(ptr @.format, i8 %70)
ret i32 0
}
declare i32 @printf(ptr noundef, ...)

