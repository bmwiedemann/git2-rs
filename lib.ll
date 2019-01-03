; ModuleID = 'lib.3a1fbbbh-cgu.0'
source_filename = "lib.3a1fbbbh-cgu.0"
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%"core::fmt::Formatter" = type { [0 x i64], { i64, i64 }, [0 x i64], { i64, i64 }, [0 x i64], { {}*, [3 x i64]* }, [0 x i64], { i64*, i64* }, [0 x i64], { [0 x { i8*, i8* }]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i8], i8, [7 x i8] }
%"core::cell::BorrowMutError" = type { [0 x i8], {}, [0 x i8] }
%"core::fmt::Arguments" = type { [0 x i64], { [0 x { [0 x i8]*, i64 }]*, i64 }, [0 x i64], { i64*, i64 }, [0 x i64], { [0 x { i8*, i8* }]*, i64 }, [0 x i64] }
%"core::str::Utf8Error" = type { [0 x i64], i64, [0 x i8], { i8, i8 }, [6 x i8] }
%"std::thread::local::AccessError" = type { [0 x i8], {}, [0 x i8] }
%ObjectType = type { [0 x i8] }
%"std::ffi::c_str::CStr" = type { [0 x i8], [0 x i8] }
%"std::path::Path" = type { [0 x i8], %"std::ffi::os_str::OsStr" }
%"std::ffi::os_str::OsStr" = type { [0 x i8], %"std::sys::unix::os_str::Slice" }
%"std::sys::unix::os_str::Slice" = type { [0 x i8], [0 x i8] }
%"index::IndexEntry" = type { [0 x i64], %"alloc::vec::Vec<u8>", [0 x i32], i32, [0 x i32], i32, [0 x i32], i32, [0 x i32], i32, [0 x i32], i32, [0 x i32], i32, [0 x i32], i32, [0 x i32], i32, [0 x i32], i32, [0 x i16], i16, [0 x i16], i16, [0 x i16] }
%"alloc::vec::Vec<u8>" = type { [0 x i64], { i8*, i64 }, [0 x i64], i64, [0 x i64] }
%"libgit2_sys::git_index_entry" = type { [0 x i32], { i32, i32 }, [0 x i32], { i32, i32 }, [0 x i32], i32, [0 x i32], i32, [0 x i32], i32, [0 x i32], i32, [0 x i32], i32, [0 x i32], i32, [0 x i8], %"libgit2_sys::git_oid", [0 x i8], i16, [0 x i16], i16, [0 x i16], i8*, [0 x i64] }
%"libgit2_sys::git_oid" = type { [0 x i8], [20 x i8], [0 x i8] }
%"core::result::Result<&str, core::str::Utf8Error>" = type { [0 x i64], i64, [2 x i64] }
%"unwind::libunwind::_Unwind_Exception" = type { [0 x i64], i64, [0 x i64], void (i32, %"unwind::libunwind::_Unwind_Exception"*)*, [0 x i64], [6 x i64], [0 x i64] }
%"unwind::libunwind::_Unwind_Context" = type { [0 x i8] }

@0 = private unnamed_addr constant <{ [16 x i8] }> <{ [16 x i8] c"already borrowed" }>, align 1
@1 = private unnamed_addr constant <{ [43 x i8] }> <{ [43 x i8] c"called `Option::unwrap()` on a `None` value" }>, align 1
@2 = private unnamed_addr constant <{ [17 x i8] }> <{ [17 x i8] c"libcore/option.rs" }>, align 1
@3 = private unnamed_addr constant <{ i8*, [8 x i8], i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [43 x i8] }>, <{ [43 x i8] }>* @1, i32 0, i32 0, i32 0), [8 x i8] c"+\00\00\00\00\00\00\00", i8* getelementptr inbounds (<{ [17 x i8] }>, <{ [17 x i8] }>* @2, i32 0, i32 0, i32 0), [16 x i8] c"\11\00\00\00\00\00\00\00c\01\00\00\15\00\00\00" }>, align 8
@vtable.0 = private unnamed_addr constant { void ({ i8*, i64 }*)*, i64, i64, { {}*, [3 x i64]* } ({ i8*, i64 }*)*, { {}*, [3 x i64]* } ({ i8*, i64 }*)* } { void ({ i8*, i64 }*)* @_ZN4core3ptr13drop_in_place17h394991667200e1a7E, i64 16, i64 8, { {}*, [3 x i64]* } ({ i8*, i64 }*)* @"_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$9box_me_up17hea965a0f076813e3E", { {}*, [3 x i64]* } ({ i8*, i64 }*)* @"_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$3get17h5f87b2fa8bba6715E" }, align 8
@4 = private unnamed_addr constant <{ [57 x i8] }> <{ [57 x i8] c"cannot access a TLS value during or after it is destroyed" }>, align 1
@5 = private unnamed_addr constant <{ [43 x i8] }> <{ [43 x i8] c"called `Result::unwrap()` on an `Err` value" }>, align 1
@6 = private unnamed_addr constant <{ [0 x i8] }> zeroinitializer, align 8
@7 = private unnamed_addr constant <{ [2 x i8] }> <{ [2 x i8] c": " }>, align 1
@8 = private unnamed_addr constant <{ i8*, [8 x i8], i8*, [8 x i8] }> <{ i8* getelementptr inbounds (<{ [0 x i8] }>, <{ [0 x i8] }>* @6, i32 0, i32 0, i32 0), [8 x i8] zeroinitializer, i8* getelementptr inbounds (<{ [2 x i8] }>, <{ [2 x i8] }>* @7, i32 0, i32 0, i32 0), [8 x i8] c"\02\00\00\00\00\00\00\00" }>, align 8
@9 = private unnamed_addr constant <{ [128 x i8] }> <{ [128 x i8] c"\01\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\03\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\03\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00 \00\00\00\00\00\00\00\03\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00\03\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\03\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00 \00\00\00\00\00\00\00\03\00\00\00\00\00\00\00" }>, align 8
@10 = private unnamed_addr constant <{ [17 x i8] }> <{ [17 x i8] c"libcore/result.rs" }>, align 1
@11 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [17 x i8] }>, <{ [17 x i8] }>* @10, i32 0, i32 0, i32 0), [16 x i8] c"\11\00\00\00\00\00\00\00\F1\03\00\00\05\00\00\00" }>, align 8
@vtable.4 = private unnamed_addr constant { void ({ [0 x i8]*, i64 }*)*, i64, i64, i64 ({ [0 x i8]*, i64 }*)* } { void ({ [0 x i8]*, i64 }*)* @_ZN4core3ptr13drop_in_place17h9bd9235bcfc5fc40E, i64 16, i64 8, i64 ({ [0 x i8]*, i64 }*)* @"_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$11get_type_id17h82fc7cfd158950e2E" }, align 8
@vtable.5 = private unnamed_addr constant { void ({}*)*, i64, i64, i64 ({}*)* } { void ({}*)* @_ZN4core3ptr13drop_in_place17h6e528a715c899f9fE, i64 0, i64 1, i64 ({}*)* @"_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$11get_type_id17h96af0782982a05e3E" }, align 8
@12 = private unnamed_addr constant <{ [12 x i8] }> <{ [12 x i8] c"src/index.rs" }>, align 1
@13 = private unnamed_addr constant <{ i8*, [16 x i8] }> <{ i8* getelementptr inbounds (<{ [12 x i8] }>, <{ [12 x i8] }>* @12, i32 0, i32 0, i32 0), [16 x i8] c"\0C\00\00\00\00\00\00\00Y\00\00\00\09\00\00\00" }>, align 8
@14 = private unnamed_addr constant <{ [14 x i8] }> <{ [14 x i8] c"explicit panic" }>, align 1
@_ZN3lib5panic10LAST_ERROR7__getit5__KEY17h3776bf008a82988eE = internal thread_local global <{ [40 x i8] }> zeroinitializer, align 32

; <T as core::any::Any>::get_type_id
; Function Attrs: norecurse nounwind nonlazybind readnone uwtable
define internal i64 @"_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$11get_type_id17h82fc7cfd158950e2E"({ [0 x i8]*, i64 }* noalias nocapture readonly dereferenceable(16) %self) unnamed_addr #0 {
start:
  ret i64 1229646359891580772
}

; <T as core::any::Any>::get_type_id
; Function Attrs: norecurse nounwind nonlazybind readnone uwtable
define internal i64 @"_ZN36_$LT$T$u20$as$u20$core..any..Any$GT$11get_type_id17h96af0782982a05e3E"({}* noalias nocapture nonnull readonly %self) unnamed_addr #0 {
start:
  ret i64 7549865886324542212
}

; std::thread::local::fast::destroy_value
; Function Attrs: nonlazybind uwtable
define internal void @_ZN3std6thread5local4fast13destroy_value17h626edf2a7806eeeaE(i8* nocapture %ptr) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %0 = getelementptr inbounds i8, i8* %ptr, i64 33
  store i8 1, i8* %0, align 1
; call std::sys::unix::fast_thread_local::requires_move_before_drop
  %1 = tail call zeroext i1 @_ZN3std3sys4unix17fast_thread_local25requires_move_before_drop17h550ffcf33fedbc57E()
  %_7.sroa.0.0..sroa_idx = bitcast i8* %ptr to i64*
  %_7.sroa.0.0.copyload = load i64, i64* %_7.sroa.0.0..sroa_idx, align 8
  br i1 %1, label %bb3, label %bb4

bb3:                                              ; preds = %start
  %_7.sroa.28.0..sroa_idx9 = getelementptr inbounds i8, i8* %ptr, i64 16
  %2 = bitcast i8* %_7.sroa.28.0..sroa_idx9 to i64*
  %_7.sroa.28.0.copyload = load i64, i64* %2, align 8
  %_7.sroa.5.0..sroa_idx12 = getelementptr inbounds i8, i8* %ptr, i64 24
  %3 = bitcast i8* %_7.sroa.5.0..sroa_idx12 to i64*
  %_7.sroa.5.0.copyload = load i64, i64* %3, align 8
  %4 = icmp eq i64 %_7.sroa.0.0.copyload, 0
  %5 = icmp eq i64 %_7.sroa.28.0.copyload, 0
  %or.cond = or i1 %4, %5
  br i1 %or.cond, label %bb10, label %bb2.i.i.i.i2

bb2.i.i.i.i2:                                     ; preds = %bb3
  %6 = inttoptr i64 %_7.sroa.28.0.copyload to {}*
  %7 = inttoptr i64 %_7.sroa.5.0.copyload to void ({}*)**
  %8 = load void ({}*)*, void ({}*)** %7, align 8, !invariant.load !1, !nonnull !1
  invoke void %8({}* nonnull %6)
          to label %bb3.i.i.i.i.i3 unwind label %cleanup.i.i.i.i.i5

bb3.i.i.i.i.i3:                                   ; preds = %bb2.i.i.i.i2
  %9 = inttoptr i64 %_7.sroa.5.0.copyload to i64*
  %10 = getelementptr inbounds i64, i64* %9, i64 1
  %11 = load i64, i64* %10, align 8, !invariant.load !1, !alias.scope !2
  %12 = icmp eq i64 %11, 0
  br i1 %12, label %bb10, label %bb4.i.i.i.i.i.i4

bb4.i.i.i.i.i.i4:                                 ; preds = %bb3.i.i.i.i.i3
  %13 = inttoptr i64 %_7.sroa.28.0.copyload to i8*
  %14 = getelementptr inbounds i64, i64* %9, i64 2
  %15 = load i64, i64* %14, align 8, !invariant.load !1, !alias.scope !2
  tail call void @__rust_dealloc(i8* nonnull %13, i64 %11, i64 %15) #9, !noalias !2
  br label %bb10

cleanup.i.i.i.i.i5:                               ; preds = %bb2.i.i.i.i2
  %16 = landingpad { i8*, i32 }
          cleanup
  %17 = inttoptr i64 %_7.sroa.28.0.copyload to i8*
  %18 = inttoptr i64 %_7.sroa.5.0.copyload to i64*
; call alloc::alloc::box_free
  tail call fastcc void @_ZN5alloc5alloc8box_free17h215c3c281cc7c6f3E(i8* nonnull %17, i64* noalias nonnull readonly dereferenceable(24) %18) #11
  resume { i8*, i32 } %16

bb4:                                              ; preds = %start
  %19 = icmp eq i64 %_7.sroa.0.0.copyload, 0
  br i1 %19, label %bb10, label %bb2.i

bb2.i:                                            ; preds = %bb4
  %20 = getelementptr inbounds i8, i8* %ptr, i64 16
  %21 = bitcast i8* %20 to {}**
  %22 = load {}*, {}** %21, align 8
  %23 = icmp eq {}* %22, null
  br i1 %23, label %bb10, label %bb2.i.i.i.i

bb2.i.i.i.i:                                      ; preds = %bb2.i
  %24 = getelementptr inbounds i8, i8* %ptr, i64 24
  %25 = bitcast i8* %24 to void ({}*)***
  %26 = load void ({}*)**, void ({}*)*** %25, align 8, !nonnull !1
  %27 = load void ({}*)*, void ({}*)** %26, align 8, !invariant.load !1, !nonnull !1
  invoke void %27({}* nonnull %22)
          to label %bb3.i.i.i.i.i unwind label %cleanup.i.i.i.i.i

bb3.i.i.i.i.i:                                    ; preds = %bb2.i.i.i.i
  %28 = bitcast i8* %24 to i64**
  %29 = load i64*, i64** %28, align 8, !nonnull !1
  %30 = getelementptr inbounds i64, i64* %29, i64 1
  %31 = load i64, i64* %30, align 8, !invariant.load !1, !alias.scope !5
  %32 = icmp eq i64 %31, 0
  br i1 %32, label %bb10, label %bb4.i.i.i.i.i.i

bb4.i.i.i.i.i.i:                                  ; preds = %bb3.i.i.i.i.i
  %33 = bitcast i8* %20 to i8**
  %34 = load i8*, i8** %33, align 8, !nonnull !1
  %35 = getelementptr inbounds i64, i64* %29, i64 2
  %36 = load i64, i64* %35, align 8, !invariant.load !1, !alias.scope !5
  tail call void @__rust_dealloc(i8* nonnull %34, i64 %31, i64 %36) #9, !noalias !5
  br label %bb10

cleanup.i.i.i.i.i:                                ; preds = %bb2.i.i.i.i
  %37 = landingpad { i8*, i32 }
          cleanup
  %38 = bitcast i8* %20 to i8**
  %39 = load i8*, i8** %38, align 8, !nonnull !1
  %40 = bitcast i8* %24 to i64**
  %41 = load i64*, i64** %40, align 8, !nonnull !1
; call alloc::alloc::box_free
  tail call fastcc void @_ZN5alloc5alloc8box_free17h215c3c281cc7c6f3E(i8* nonnull %39, i64* noalias nonnull readonly dereferenceable(24) %41) #11
  resume { i8*, i32 } %37

bb10:                                             ; preds = %bb4.i.i.i.i.i.i, %bb3.i.i.i.i.i, %bb2.i, %bb4, %bb4.i.i.i.i.i.i4, %bb3.i.i.i.i.i3, %bb3
  ret void
}

; std::panicking::begin_panic
; Function Attrs: cold noinline noreturn nonlazybind uwtable
define internal fastcc void @_ZN3std9panicking11begin_panic17h472fddf7b7ec90f3E() unnamed_addr #2 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %_6 = alloca { i8*, i64 }, align 8
  %0 = bitcast { i8*, i64 }* %_6 to i8*
  call void @llvm.lifetime.start.p0i8(i64 16, i8* nonnull %0)
  %.fca.0.gep = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_6, i64 0, i32 0
  store i8* getelementptr inbounds (<{ [14 x i8] }>, <{ [14 x i8] }>* @14, i64 0, i32 0, i64 0), i8** %.fca.0.gep, align 8
  %.fca.1.gep = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %_6, i64 0, i32 1
  store i64 14, i64* %.fca.1.gep, align 8
  %1 = bitcast { i8*, i64 }* %_6 to {}*
; call std::panicking::rust_panic_with_hook
  call void @_ZN3std9panicking20rust_panic_with_hook17h2abd0ff3555ca508E({}* nonnull %1, [3 x i64]* noalias readonly dereferenceable(24) bitcast ({ void ({ i8*, i64 }*)*, i64, i64, { {}*, [3 x i64]* } ({ i8*, i64 }*)*, { {}*, [3 x i64]* } ({ i8*, i64 }*)* }* @vtable.0 to [3 x i64]*), i64* noalias readonly dereferenceable_or_null(48) null, { [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @13 to { [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*))
  unreachable
}

; <&T as core::fmt::Display>::fmt
; Function Attrs: nonlazybind uwtable
define internal zeroext i1 @"_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h3a1c279ce18fc823E"({ [0 x i8]*, i64 }* noalias nocapture readonly dereferenceable(16) %self, %"core::fmt::Formatter"* dereferenceable(96) %f) unnamed_addr #1 {
start:
  %0 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %self, i64 0, i32 0
  %1 = load [0 x i8]*, [0 x i8]** %0, align 8, !nonnull !1
  %2 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %self, i64 0, i32 1
  %3 = load i64, i64* %2, align 8
; call <str as core::fmt::Display>::fmt
  %4 = tail call zeroext i1 @"_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17h4d06f72123a3bf61E"([0 x i8]* noalias nonnull readonly %1, i64 %3, %"core::fmt::Formatter"* nonnull dereferenceable(96) %f)
  ret i1 %4
}

; core::ptr::drop_in_place
; Function Attrs: norecurse nounwind nonlazybind readnone uwtable
define internal void @_ZN4core3ptr13drop_in_place17h394991667200e1a7E({ i8*, i64 }* nocapture %arg0) unnamed_addr #0 {
start:
  ret void
}

; core::ptr::drop_in_place
; Function Attrs: norecurse nounwind nonlazybind readnone uwtable
define internal void @_ZN4core3ptr13drop_in_place17h6e528a715c899f9fE({}* nocapture %arg0) unnamed_addr #0 {
start:
  ret void
}

; core::ptr::drop_in_place
; Function Attrs: norecurse nounwind nonlazybind readnone uwtable
define internal void @_ZN4core3ptr13drop_in_place17h9bd9235bcfc5fc40E({ [0 x i8]*, i64 }* nocapture %arg0) unnamed_addr #0 {
start:
  ret void
}

; core::result::unwrap_failed
; Function Attrs: cold noinline noreturn nonlazybind uwtable
define internal fastcc void @_ZN4core6result13unwrap_failed17h1e4b46b65af320c3E() unnamed_addr #2 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %error = alloca %"core::cell::BorrowMutError", align 1
  %_10 = alloca [2 x { i8*, i8* }], align 8
  %_3 = alloca %"core::fmt::Arguments", align 8
  %msg = alloca { [0 x i8]*, i64 }, align 8
  %0 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %msg, i64 0, i32 0
  store [0 x i8]* bitcast (<{ [16 x i8] }>* @0 to [0 x i8]*), [0 x i8]** %0, align 8
  %1 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %msg, i64 0, i32 1
  store i64 16, i64* %1, align 8
  %2 = bitcast %"core::fmt::Arguments"* %_3 to i8*
  call void @llvm.lifetime.start.p0i8(i64 48, i8* nonnull %2)
  %3 = bitcast [2 x { i8*, i8* }]* %_10 to i8*
  call void @llvm.lifetime.start.p0i8(i64 32, i8* nonnull %3)
  %4 = getelementptr inbounds %"core::cell::BorrowMutError", %"core::cell::BorrowMutError"* %error, i64 0, i32 0, i64 0
  %5 = bitcast [2 x { i8*, i8* }]* %_10 to { [0 x i8]*, i64 }**
  store { [0 x i8]*, i64 }* %msg, { [0 x i8]*, i64 }** %5, align 8
  %6 = getelementptr inbounds [2 x { i8*, i8* }], [2 x { i8*, i8* }]* %_10, i64 0, i64 0, i32 1
  store i8* bitcast (i1 ({ [0 x i8]*, i64 }*, %"core::fmt::Formatter"*)* @"_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h3a1c279ce18fc823E" to i8*), i8** %6, align 8
  %7 = getelementptr inbounds [2 x { i8*, i8* }], [2 x { i8*, i8* }]* %_10, i64 0, i64 1, i32 0
  store i8* %4, i8** %7, align 8
  %8 = getelementptr inbounds [2 x { i8*, i8* }], [2 x { i8*, i8* }]* %_10, i64 0, i64 1, i32 1
  store i8* bitcast (i1 (%"core::cell::BorrowMutError"*, %"core::fmt::Formatter"*)* @"_ZN63_$LT$core..cell..BorrowMutError$u20$as$u20$core..fmt..Debug$GT$3fmt17hca7d2710462e2ce1E" to i8*), i8** %8, align 8
  %9 = bitcast %"core::fmt::Arguments"* %_3 to [0 x { [0 x i8]*, i64 }]**
  store [0 x { [0 x i8]*, i64 }]* bitcast (<{ i8*, [8 x i8], i8*, [8 x i8] }>* @8 to [0 x { [0 x i8]*, i64 }]*), [0 x { [0 x i8]*, i64 }]** %9, align 8, !alias.scope !8, !noalias !11
  %10 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %_3, i64 0, i32 1, i32 1
  store i64 2, i64* %10, align 8, !alias.scope !8, !noalias !11
  %11 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %_3, i64 0, i32 3, i32 0
  store i64* bitcast (<{ [128 x i8] }>* @9 to i64*), i64** %11, align 8, !alias.scope !8, !noalias !11
  %12 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %_3, i64 0, i32 3, i32 1
  store i64 2, i64* %12, align 8, !alias.scope !8, !noalias !11
  %13 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %_3, i64 0, i32 5, i32 0
  %14 = bitcast [0 x { i8*, i8* }]** %13 to [2 x { i8*, i8* }]**
  store [2 x { i8*, i8* }]* %_10, [2 x { i8*, i8* }]** %14, align 8, !alias.scope !8, !noalias !11
  %15 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %_3, i64 0, i32 5, i32 1
  store i64 2, i64* %15, align 8, !alias.scope !8, !noalias !11
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17hf3272c92bd6c0e0fE(%"core::fmt::Arguments"* noalias nocapture nonnull dereferenceable(48) %_3, { [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @11 to { [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*))
  unreachable
}

; core::result::unwrap_failed
; Function Attrs: cold noinline noreturn nonlazybind uwtable
define internal fastcc void @_ZN4core6result13unwrap_failed17h2423a2d9b55cb964E(%"core::str::Utf8Error"* noalias nocapture dereferenceable(16) %error) unnamed_addr #2 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %_10 = alloca [2 x { i8*, i8* }], align 8
  %_3 = alloca %"core::fmt::Arguments", align 8
  %msg = alloca { [0 x i8]*, i64 }, align 8
  %0 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %msg, i64 0, i32 0
  store [0 x i8]* bitcast (<{ [43 x i8] }>* @5 to [0 x i8]*), [0 x i8]** %0, align 8
  %1 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %msg, i64 0, i32 1
  store i64 43, i64* %1, align 8
  %2 = bitcast %"core::fmt::Arguments"* %_3 to i8*
  call void @llvm.lifetime.start.p0i8(i64 48, i8* nonnull %2)
  %3 = bitcast [2 x { i8*, i8* }]* %_10 to i8*
  call void @llvm.lifetime.start.p0i8(i64 32, i8* nonnull %3)
  %4 = bitcast [2 x { i8*, i8* }]* %_10 to { [0 x i8]*, i64 }**
  store { [0 x i8]*, i64 }* %msg, { [0 x i8]*, i64 }** %4, align 8
  %5 = getelementptr inbounds [2 x { i8*, i8* }], [2 x { i8*, i8* }]* %_10, i64 0, i64 0, i32 1
  store i8* bitcast (i1 ({ [0 x i8]*, i64 }*, %"core::fmt::Formatter"*)* @"_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h3a1c279ce18fc823E" to i8*), i8** %5, align 8
  %6 = getelementptr inbounds [2 x { i8*, i8* }], [2 x { i8*, i8* }]* %_10, i64 0, i64 1, i32 0
  %7 = bitcast i8** %6 to %"core::str::Utf8Error"**
  store %"core::str::Utf8Error"* %error, %"core::str::Utf8Error"** %7, align 8
  %8 = getelementptr inbounds [2 x { i8*, i8* }], [2 x { i8*, i8* }]* %_10, i64 0, i64 1, i32 1
  store i8* bitcast (i1 (%"core::str::Utf8Error"*, %"core::fmt::Formatter"*)* @"_ZN57_$LT$core..str..Utf8Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h038d69fafa9b888eE" to i8*), i8** %8, align 8
  %9 = bitcast %"core::fmt::Arguments"* %_3 to [0 x { [0 x i8]*, i64 }]**
  store [0 x { [0 x i8]*, i64 }]* bitcast (<{ i8*, [8 x i8], i8*, [8 x i8] }>* @8 to [0 x { [0 x i8]*, i64 }]*), [0 x { [0 x i8]*, i64 }]** %9, align 8, !alias.scope !15, !noalias !18
  %10 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %_3, i64 0, i32 1, i32 1
  store i64 2, i64* %10, align 8, !alias.scope !15, !noalias !18
  %11 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %_3, i64 0, i32 3, i32 0
  store i64* bitcast (<{ [128 x i8] }>* @9 to i64*), i64** %11, align 8, !alias.scope !15, !noalias !18
  %12 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %_3, i64 0, i32 3, i32 1
  store i64 2, i64* %12, align 8, !alias.scope !15, !noalias !18
  %13 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %_3, i64 0, i32 5, i32 0
  %14 = bitcast [0 x { i8*, i8* }]** %13 to [2 x { i8*, i8* }]**
  store [2 x { i8*, i8* }]* %_10, [2 x { i8*, i8* }]** %14, align 8, !alias.scope !15, !noalias !18
  %15 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %_3, i64 0, i32 5, i32 1
  store i64 2, i64* %15, align 8, !alias.scope !15, !noalias !18
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17hf3272c92bd6c0e0fE(%"core::fmt::Arguments"* noalias nocapture nonnull dereferenceable(48) %_3, { [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @11 to { [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*))
  unreachable
}

; core::result::unwrap_failed
; Function Attrs: cold noinline noreturn nonlazybind uwtable
define internal fastcc void @_ZN4core6result13unwrap_failed17he3b0fbc31f1f03ebE() unnamed_addr #2 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %error = alloca %"std::thread::local::AccessError", align 1
  %_10 = alloca [2 x { i8*, i8* }], align 8
  %_3 = alloca %"core::fmt::Arguments", align 8
  %msg = alloca { [0 x i8]*, i64 }, align 8
  %0 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %msg, i64 0, i32 0
  store [0 x i8]* bitcast (<{ [57 x i8] }>* @4 to [0 x i8]*), [0 x i8]** %0, align 8
  %1 = getelementptr inbounds { [0 x i8]*, i64 }, { [0 x i8]*, i64 }* %msg, i64 0, i32 1
  store i64 57, i64* %1, align 8
  %2 = bitcast %"core::fmt::Arguments"* %_3 to i8*
  call void @llvm.lifetime.start.p0i8(i64 48, i8* nonnull %2)
  %3 = bitcast [2 x { i8*, i8* }]* %_10 to i8*
  call void @llvm.lifetime.start.p0i8(i64 32, i8* nonnull %3)
  %4 = getelementptr inbounds %"std::thread::local::AccessError", %"std::thread::local::AccessError"* %error, i64 0, i32 0, i64 0
  %5 = bitcast [2 x { i8*, i8* }]* %_10 to { [0 x i8]*, i64 }**
  store { [0 x i8]*, i64 }* %msg, { [0 x i8]*, i64 }** %5, align 8
  %6 = getelementptr inbounds [2 x { i8*, i8* }], [2 x { i8*, i8* }]* %_10, i64 0, i64 0, i32 1
  store i8* bitcast (i1 ({ [0 x i8]*, i64 }*, %"core::fmt::Formatter"*)* @"_ZN44_$LT$$RF$T$u20$as$u20$core..fmt..Display$GT$3fmt17h3a1c279ce18fc823E" to i8*), i8** %6, align 8
  %7 = getelementptr inbounds [2 x { i8*, i8* }], [2 x { i8*, i8* }]* %_10, i64 0, i64 1, i32 0
  store i8* %4, i8** %7, align 8
  %8 = getelementptr inbounds [2 x { i8*, i8* }], [2 x { i8*, i8* }]* %_10, i64 0, i64 1, i32 1
  store i8* bitcast (i1 (%"std::thread::local::AccessError"*, %"core::fmt::Formatter"*)* @"_ZN68_$LT$std..thread..local..AccessError$u20$as$u20$core..fmt..Debug$GT$3fmt17h82e5a9d1278eec63E" to i8*), i8** %8, align 8
  %9 = bitcast %"core::fmt::Arguments"* %_3 to [0 x { [0 x i8]*, i64 }]**
  store [0 x { [0 x i8]*, i64 }]* bitcast (<{ i8*, [8 x i8], i8*, [8 x i8] }>* @8 to [0 x { [0 x i8]*, i64 }]*), [0 x { [0 x i8]*, i64 }]** %9, align 8, !alias.scope !22, !noalias !25
  %10 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %_3, i64 0, i32 1, i32 1
  store i64 2, i64* %10, align 8, !alias.scope !22, !noalias !25
  %11 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %_3, i64 0, i32 3, i32 0
  store i64* bitcast (<{ [128 x i8] }>* @9 to i64*), i64** %11, align 8, !alias.scope !22, !noalias !25
  %12 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %_3, i64 0, i32 3, i32 1
  store i64 2, i64* %12, align 8, !alias.scope !22, !noalias !25
  %13 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %_3, i64 0, i32 5, i32 0
  %14 = bitcast [0 x { i8*, i8* }]** %13 to [2 x { i8*, i8* }]**
  store [2 x { i8*, i8* }]* %_10, [2 x { i8*, i8* }]** %14, align 8, !alias.scope !22, !noalias !25
  %15 = getelementptr inbounds %"core::fmt::Arguments", %"core::fmt::Arguments"* %_3, i64 0, i32 5, i32 1
  store i64 2, i64* %15, align 8, !alias.scope !22, !noalias !25
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17hf3272c92bd6c0e0fE(%"core::fmt::Arguments"* noalias nocapture nonnull dereferenceable(48) %_3, { [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly dereferenceable(24) bitcast (<{ i8*, [16 x i8] }>* @11 to { [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*))
  unreachable
}

; alloc::alloc::box_free
; Function Attrs: inlinehint nounwind nonlazybind uwtable
define internal fastcc void @_ZN5alloc5alloc8box_free17h215c3c281cc7c6f3E(i8* nonnull %ptr.0, i64* noalias readonly dereferenceable(24) %ptr.1) unnamed_addr #3 {
start:
  %0 = getelementptr inbounds i64, i64* %ptr.1, i64 1
  %1 = load i64, i64* %0, align 8, !invariant.load !1
  %2 = icmp eq i64 %1, 0
  br i1 %2, label %bb7, label %bb4

bb4:                                              ; preds = %start
  %3 = getelementptr inbounds i64, i64* %ptr.1, i64 2
  %4 = load i64, i64* %3, align 8, !invariant.load !1
  tail call void @__rust_dealloc(i8* nonnull %ptr.0, i64 %1, i64 %4) #9
  br label %bb7

bb7:                                              ; preds = %start, %bb4
  ret void
}

; <std::panicking::begin_panic::PanicPayload<A> as core::panic::BoxMeUp>::get
; Function Attrs: norecurse nounwind nonlazybind readonly uwtable
define internal { {}*, [3 x i64]* } @"_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$3get17h5f87b2fa8bba6715E"({ i8*, i64 }* dereferenceable(16) %self) unnamed_addr #4 {
start:
  %0 = bitcast { i8*, i64 }* %self to {}**
  %1 = load {}*, {}** %0, align 8
  %2 = icmp eq {}* %1, null
  %3 = bitcast { i8*, i64 }* %self to {}*
  %.sink2 = select i1 %2, {}* bitcast (<{ [0 x i8] }>* @6 to {}*), {}* %3
  %.sink = select i1 %2, [3 x i64]* bitcast ({ void ({}*)*, i64, i64, i64 ({}*)* }* @vtable.5 to [3 x i64]*), [3 x i64]* bitcast ({ void ({ [0 x i8]*, i64 }*)*, i64, i64, i64 ({ [0 x i8]*, i64 }*)* }* @vtable.4 to [3 x i64]*)
  %4 = insertvalue { {}*, [3 x i64]* } undef, {}* %.sink2, 0
  %5 = insertvalue { {}*, [3 x i64]* } %4, [3 x i64]* %.sink, 1
  ret { {}*, [3 x i64]* } %5
}

; <std::panicking::begin_panic::PanicPayload<A> as core::panic::BoxMeUp>::box_me_up
; Function Attrs: nonlazybind uwtable
define internal { {}*, [3 x i64]* } @"_ZN91_$LT$std..panicking..begin_panic..PanicPayload$LT$A$GT$$u20$as$u20$core..panic..BoxMeUp$GT$9box_me_up17hea965a0f076813e3E"({ i8*, i64 }* nocapture dereferenceable(16) %self) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %0 = bitcast { i8*, i64 }* %self to i64*
  %1 = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %self, i64 0, i32 0
  %tmp.sroa.0.0.copyload.i4.i.i2.i.i = load i8*, i8** %1, align 8
  %tmp.sroa.5.0..sroa_idx2.i.i.i.i.i = getelementptr inbounds { i8*, i64 }, { i8*, i64 }* %self, i64 0, i32 1
  %tmp.sroa.5.0.copyload.i.i.i.i.i = load i64, i64* %tmp.sroa.5.0..sroa_idx2.i.i.i.i.i, align 8
  store i64 0, i64* %0, align 8
  %2 = icmp eq i8* %tmp.sroa.0.0.copyload.i4.i.i2.i.i, null
  br i1 %2, label %bb10, label %bb5

bb5:                                              ; preds = %start
  %3 = tail call i8* @__rust_alloc(i64 16, i64 8) #9, !noalias !29
  %4 = icmp eq i8* %3, null
  br i1 %4, label %bb7.i.i, label %bb7

bb7.i.i:                                          ; preds = %bb5
; call alloc::alloc::handle_alloc_error
  tail call void @_ZN5alloc5alloc18handle_alloc_error17haef099f38630551aE(i64 16, i64 8) #9
  unreachable

bb7:                                              ; preds = %bb5
  %5 = bitcast i8* %3 to i8**
  store i8* %tmp.sroa.0.0.copyload.i4.i.i2.i.i, i8** %5, align 8, !noalias !29
  %6 = getelementptr inbounds i8, i8* %3, i64 8
  %7 = bitcast i8* %6 to i64*
  store i64 %tmp.sroa.5.0.copyload.i.i.i.i.i, i64* %7, align 8, !noalias !29
  %8 = bitcast i8* %3 to {}*
  br label %bb10

bb10:                                             ; preds = %bb7, %start
  %.sink3 = phi {}* [ %8, %bb7 ], [ inttoptr (i64 1 to {}*), %start ]
  %.sink = phi [3 x i64]* [ bitcast ({ void ({ [0 x i8]*, i64 }*)*, i64, i64, i64 ({ [0 x i8]*, i64 }*)* }* @vtable.4 to [3 x i64]*), %bb7 ], [ bitcast ({ void ({}*)*, i64, i64, i64 ({}*)* }* @vtable.5 to [3 x i64]*), %start ]
  %9 = icmp ne {}* %.sink3, null
  tail call void @llvm.assume(i1 %9)
  %10 = insertvalue { {}*, [3 x i64]* } undef, {}* %.sink3, 0
  %11 = insertvalue { {}*, [3 x i64]* } %10, [3 x i64]* %.sink, 1
  ret { {}*, [3 x i64]* } %11
}

; lib::panic::check
; Function Attrs: nonlazybind uwtable
define void @_ZN3lib5panic5check17h281f861c6ca5df27E() unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %0 = load i8, i8* getelementptr inbounds (<{ [40 x i8] }>, <{ [40 x i8] }>* @_ZN3lib5panic10LAST_ERROR7__getit5__KEY17h3776bf008a82988eE, i64 0, i32 0, i64 33), align 1, !range !32, !noalias !33
  %1 = icmp eq i8 %0, 0
  br i1 %1, label %bb8.i.i.i.i, label %bb4.i.i

bb8.i.i.i.i:                                      ; preds = %start
  %2 = load i8, i8* getelementptr inbounds (<{ [40 x i8] }>, <{ [40 x i8] }>* @_ZN3lib5panic10LAST_ERROR7__getit5__KEY17h3776bf008a82988eE, i64 0, i32 0, i64 32), align 32, !range !32, !noalias !33
  %3 = icmp eq i8 %2, 0
  br i1 %3, label %bb8.i.i.i.i.i, label %bb12.i.i

bb8.i.i.i.i.i:                                    ; preds = %bb8.i.i.i.i
; call std::sys::unix::fast_thread_local::register_dtor
  tail call void @_ZN3std3sys4unix17fast_thread_local13register_dtor17h412fad7fe7b77161E(i8* getelementptr inbounds (<{ [40 x i8] }>, <{ [40 x i8] }>* @_ZN3lib5panic10LAST_ERROR7__getit5__KEY17h3776bf008a82988eE, i64 0, i32 0, i64 0), void (i8*)* nonnull @_ZN3std6thread5local4fast13destroy_value17h626edf2a7806eeeaE), !noalias !33
  store i8 1, i8* getelementptr inbounds (<{ [40 x i8] }>, <{ [40 x i8] }>* @_ZN3lib5panic10LAST_ERROR7__getit5__KEY17h3776bf008a82988eE, i64 0, i32 0, i64 32), align 32, !noalias !33
  br label %bb12.i.i

bb12.i.i:                                         ; preds = %bb8.i.i.i.i.i, %bb8.i.i.i.i
  %4 = load i64, i64* bitcast (<{ [40 x i8] }>* @_ZN3lib5panic10LAST_ERROR7__getit5__KEY17h3776bf008a82988eE to i64*), align 32, !range !36, !noalias !33
  %switch.i.i = icmp eq i64 %4, 1
  br i1 %switch.i.i, label %bb16.i.i, label %bb14.i.i

bb14.i.i:                                         ; preds = %bb12.i.i
  %t.0.copyload2.i.i.i.i.i.i.i.i = load <4 x i64>, <4 x i64>* bitcast (<{ [40 x i8] }>* @_ZN3lib5panic10LAST_ERROR7__getit5__KEY17h3776bf008a82988eE to <4 x i64>*), align 32, !noalias !37
  store <4 x i64> <i64 1, i64 0, i64 0, i64 undef>, <4 x i64>* bitcast (<{ [40 x i8] }>* @_ZN3lib5panic10LAST_ERROR7__getit5__KEY17h3776bf008a82988eE to <4 x i64>*), align 32, !noalias !33
  %_10.sroa.0.0.vec.extract.i.i.i = extractelement <4 x i64> %t.0.copyload2.i.i.i.i.i.i.i.i, i32 0
  %_10.sroa.0.16.vec.extract.i.i.i = extractelement <4 x i64> %t.0.copyload2.i.i.i.i.i.i.i.i, i32 2
  %_10.sroa.0.24.vec.extract.i.i.i = extractelement <4 x i64> %t.0.copyload2.i.i.i.i.i.i.i.i, i32 3
  %5 = icmp eq i64 %_10.sroa.0.0.vec.extract.i.i.i, 0
  %6 = icmp eq i64 %_10.sroa.0.16.vec.extract.i.i.i, 0
  %or.cond.i.i.i = or i1 %5, %6
  br i1 %or.cond.i.i.i, label %bb7.i.i.i, label %bb2.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i:                                ; preds = %bb14.i.i
  %7 = inttoptr i64 %_10.sroa.0.16.vec.extract.i.i.i to {}*
  %8 = inttoptr i64 %_10.sroa.0.24.vec.extract.i.i.i to void ({}*)**
  %9 = load void ({}*)*, void ({}*)** %8, align 8, !invariant.load !1, !noalias !33, !nonnull !1
  invoke void %9({}* nonnull %7)
          to label %bb3.i.i.i.i.i.i.i.i unwind label %cleanup.i.i.i.i.i.i.i.i, !noalias !33

bb3.i.i.i.i.i.i.i.i:                              ; preds = %bb2.i.i.i.i.i.i.i
  %10 = inttoptr i64 %_10.sroa.0.24.vec.extract.i.i.i to i64*
  %11 = getelementptr inbounds i64, i64* %10, i64 1
  %12 = load i64, i64* %11, align 8, !invariant.load !1, !alias.scope !41, !noalias !33
  %13 = icmp eq i64 %12, 0
  br i1 %13, label %bb7.i.i.i, label %bb4.i.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i.i:                            ; preds = %bb3.i.i.i.i.i.i.i.i
  %14 = inttoptr i64 %_10.sroa.0.16.vec.extract.i.i.i to i8*
  %15 = getelementptr inbounds i64, i64* %10, i64 2
  %16 = load i64, i64* %15, align 8, !invariant.load !1, !alias.scope !41, !noalias !33
  tail call void @__rust_dealloc(i8* nonnull %14, i64 %12, i64 %16) #9, !noalias !44
  br label %bb7.i.i.i

cleanup.i.i.i.i.i.i.i.i:                          ; preds = %bb2.i.i.i.i.i.i.i
  %17 = landingpad { i8*, i32 }
          cleanup
  %18 = inttoptr i64 %_10.sroa.0.16.vec.extract.i.i.i to i8*
  %19 = inttoptr i64 %_10.sroa.0.24.vec.extract.i.i.i to i64*
; call alloc::alloc::box_free
  tail call fastcc void @_ZN5alloc5alloc8box_free17h215c3c281cc7c6f3E(i8* nonnull %18, i64* noalias nonnull readonly dereferenceable(24) %19) #11, !noalias !33
  resume { i8*, i32 } %17

bb7.i.i.i:                                        ; preds = %bb4.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i, %bb14.i.i
  %20 = load i64, i64* bitcast (<{ [40 x i8] }>* @_ZN3lib5panic10LAST_ERROR7__getit5__KEY17h3776bf008a82988eE to i64*), align 32, !range !36, !noalias !33
  %switch.i.i.i.i = icmp eq i64 %20, 1
  br i1 %switch.i.i.i.i, label %bb16.i.i, label %bb2.i.i.i.i

bb2.i.i.i.i:                                      ; preds = %bb7.i.i.i
; call core::panicking::panic
  tail call void @_ZN4core9panicking5panic17h59ee191e7f64099dE({ [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly dereferenceable(40) bitcast (<{ i8*, [8 x i8], i8*, [16 x i8] }>* @3 to { [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }*)), !noalias !33
  unreachable

bb16.i.i:                                         ; preds = %bb7.i.i.i, %bb12.i.i
  %21 = load i64, i64* bitcast (i8* getelementptr inbounds (<{ [40 x i8] }>, <{ [40 x i8] }>* @_ZN3lib5panic10LAST_ERROR7__getit5__KEY17h3776bf008a82988eE, i64 0, i32 0, i64 8) to i64*), align 8, !noalias !33
  %22 = icmp eq i64 %21, 0
  br i1 %22, label %"_ZN46_$LT$std..thread..local..LocalKey$LT$T$GT$$GT$4with17h333feba022888ae6E.exit", label %bb4.i.i.i.i.i

bb4.i.i.i.i.i:                                    ; preds = %bb16.i.i
; call core::result::unwrap_failed
  tail call fastcc void @_ZN4core6result13unwrap_failed17h1e4b46b65af320c3E(), !noalias !33
  unreachable

bb4.i.i:                                          ; preds = %start
; call core::result::unwrap_failed
  tail call fastcc void @_ZN4core6result13unwrap_failed17he3b0fbc31f1f03ebE(), !noalias !45
  unreachable

"_ZN46_$LT$std..thread..local..LocalKey$LT$T$GT$$GT$4with17h333feba022888ae6E.exit": ; preds = %bb16.i.i
  store i64 -1, i64* bitcast (i8* getelementptr inbounds (<{ [40 x i8] }>, <{ [40 x i8] }>* @_ZN3lib5panic10LAST_ERROR7__getit5__KEY17h3776bf008a82988eE, i64 0, i32 0, i64 8) to i64*), align 8, !noalias !33
  %tmp.sroa.0.0.copyload.i7.i.i2.i.i.i8.i6.i = load i8*, i8** bitcast (i8* getelementptr inbounds (<{ [40 x i8] }>, <{ [40 x i8] }>* @_ZN3lib5panic10LAST_ERROR7__getit5__KEY17h3776bf008a82988eE, i64 0, i32 0, i64 16) to i8**), align 16, !noalias !33
  %tmp.sroa.5.0.copyload.i6.i.i3.i.i.i7.i7.i = load i8*, i8** bitcast (i8* getelementptr inbounds (<{ [40 x i8] }>, <{ [40 x i8] }>* @_ZN3lib5panic10LAST_ERROR7__getit5__KEY17h3776bf008a82988eE, i64 0, i32 0, i64 24) to i8**), align 8, !noalias !33
  tail call void @llvm.memset.p0i8.i64(i8* nonnull getelementptr inbounds (<{ [40 x i8] }>, <{ [40 x i8] }>* @_ZN3lib5panic10LAST_ERROR7__getit5__KEY17h3776bf008a82988eE, i64 0, i32 0, i64 8), i8 0, i64 16, i32 8, i1 false), !noalias !33
  %23 = icmp eq i8* %tmp.sroa.0.0.copyload.i7.i.i2.i.i.i8.i6.i, null
  br i1 %23, label %_ZN4core3ptr13drop_in_place17h3f30137973e10800E.exit, label %bb2.i

bb2.i:                                            ; preds = %"_ZN46_$LT$std..thread..local..LocalKey$LT$T$GT$$GT$4with17h333feba022888ae6E.exit"
  %24 = bitcast i8* %tmp.sroa.0.0.copyload.i7.i.i2.i.i.i8.i6.i to {}*
  %25 = icmp ne i8* %tmp.sroa.5.0.copyload.i6.i.i3.i.i.i7.i7.i, null
  tail call void @llvm.assume(i1 %25)
  %26 = bitcast i8* %tmp.sroa.5.0.copyload.i6.i.i3.i.i.i7.i7.i to void ({}*)**
  %27 = load void ({}*)*, void ({}*)** %26, align 8, !invariant.load !1, !nonnull !1
  invoke void %27({}* nonnull %24)
          to label %bb3.i.i unwind label %cleanup.i.i

bb3.i.i:                                          ; preds = %bb2.i
  %28 = getelementptr inbounds i8, i8* %tmp.sroa.5.0.copyload.i6.i.i3.i.i.i7.i7.i, i64 8
  %29 = bitcast i8* %28 to i64*
  %30 = load i64, i64* %29, align 8, !invariant.load !1, !alias.scope !48
  %31 = icmp eq i64 %30, 0
  br i1 %31, label %_ZN4core3ptr13drop_in_place17h3f30137973e10800E.exit, label %bb4.i.i.i

bb4.i.i.i:                                        ; preds = %bb3.i.i
  %32 = getelementptr inbounds i8, i8* %tmp.sroa.5.0.copyload.i6.i.i3.i.i.i7.i7.i, i64 16
  %33 = bitcast i8* %32 to i64*
  %34 = load i64, i64* %33, align 8, !invariant.load !1, !alias.scope !48
  tail call void @__rust_dealloc(i8* nonnull %tmp.sroa.0.0.copyload.i7.i.i2.i.i.i8.i6.i, i64 %30, i64 %34) #9, !noalias !48
  br label %_ZN4core3ptr13drop_in_place17h3f30137973e10800E.exit

cleanup.i.i:                                      ; preds = %bb2.i
  %35 = landingpad { i8*, i32 }
          cleanup
  %36 = bitcast i8* %tmp.sroa.5.0.copyload.i6.i.i3.i.i.i7.i7.i to i64*
; call alloc::alloc::box_free
  tail call fastcc void @_ZN5alloc5alloc8box_free17h215c3c281cc7c6f3E(i8* nonnull %tmp.sroa.0.0.copyload.i7.i.i2.i.i.i8.i6.i, i64* noalias nonnull readonly dereferenceable(24) %36) #11
  resume { i8*, i32 } %35

_ZN4core3ptr13drop_in_place17h3f30137973e10800E.exit: ; preds = %"_ZN46_$LT$std..thread..local..LocalKey$LT$T$GT$$GT$4with17h333feba022888ae6E.exit", %bb3.i.i, %bb4.i.i.i
  ret void
}

; lib::call::try
; Function Attrs: norecurse nounwind nonlazybind readnone uwtable
define { i32, i32 } @_ZN3lib4call3try17hfe15144358394a08E(i32 %ret) unnamed_addr #0 {
start:
  %0 = insertvalue { i32, i32 } { i32 0, i32 undef }, i32 %ret, 1
  ret { i32, i32 } %0
}

; lib::call::impls::<impl lib::call::Convert<i32> for lib::ObjectType>::convert
; Function Attrs: norecurse nounwind nonlazybind readnone uwtable
define i32 @"_ZN3lib4call5impls75_$LT$impl$u20$lib..call..Convert$LT$i32$GT$$u20$for$u20$lib..ObjectType$GT$7convert17hc909eb0130d0bbc0E"(%ObjectType* noalias nocapture nonnull readonly %self) unnamed_addr #0 {
start:
  ret i32 4
}

; lib::index::index_matched_path_cb
; Function Attrs: nonlazybind uwtable
define i32 @_ZN3lib5index21index_matched_path_cb17h362bc79cf1573866E(i8* %path, i8* %matched_pathspec, i8*) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
; call std::ffi::c_str::CStr::from_ptr
  %1 = tail call { %"std::ffi::c_str::CStr"*, i64 } @_ZN3std3ffi5c_str4CStr8from_ptr17hb129c3df1c28e4bbE(i8* %path)
  %2 = extractvalue { %"std::ffi::c_str::CStr"*, i64 } %1, 1
  %3 = add i64 %2, -1
  %4 = icmp eq i64 %2, 0
  br i1 %4, label %bb4.i.i.i.i, label %_ZN3std3ffi5c_str4CStr8to_bytes17h267747ec95802125E.exit

bb4.i.i.i.i:                                      ; preds = %start
; call core::slice::slice_index_len_fail
  tail call void @_ZN4core5slice20slice_index_len_fail17h16ba3d3f76e9f048E(i64 %3, i64 0)
  unreachable

_ZN3std3ffi5c_str4CStr8to_bytes17h267747ec95802125E.exit: ; preds = %start
; call std::ffi::c_str::CStr::from_ptr
  %5 = tail call { %"std::ffi::c_str::CStr"*, i64 } @_ZN3std3ffi5c_str4CStr8from_ptr17hb129c3df1c28e4bbE(i8* %matched_pathspec)
  %6 = extractvalue { %"std::ffi::c_str::CStr"*, i64 } %5, 1
  %7 = add i64 %6, -1
  %8 = icmp eq i64 %6, 0
  br i1 %8, label %bb4.i.i.i.i7, label %_ZN3std3ffi5c_str4CStr8to_bytes17h267747ec95802125E.exit8

bb4.i.i.i.i7:                                     ; preds = %_ZN3std3ffi5c_str4CStr8to_bytes17h267747ec95802125E.exit
; call core::slice::slice_index_len_fail
  tail call void @_ZN4core5slice20slice_index_len_fail17h16ba3d3f76e9f048E(i64 %7, i64 0)
  unreachable

_ZN3std3ffi5c_str4CStr8to_bytes17h267747ec95802125E.exit8: ; preds = %_ZN3std3ffi5c_str4CStr8to_bytes17h267747ec95802125E.exit
  %9 = extractvalue { %"std::ffi::c_str::CStr"*, i64 } %1, 0
  %10 = getelementptr inbounds %"std::ffi::c_str::CStr", %"std::ffi::c_str::CStr"* %9, i64 0, i32 0
  %11 = extractvalue { %"std::ffi::c_str::CStr"*, i64 } %5, 0
  %12 = getelementptr inbounds %"std::ffi::c_str::CStr", %"std::ffi::c_str::CStr"* %11, i64 0, i32 0
  %13 = bitcast i8* %0 to {}**
  %14 = load {}*, {}** %13, align 8, !noalias !51, !nonnull !1
  %15 = getelementptr inbounds i8, i8* %0, i64 8
  %16 = bitcast i8* %15 to i32 ({}*, %"std::path::Path"*, i64, [0 x i8]*, i64)***
  %17 = load i32 ({}*, %"std::path::Path"*, i64, [0 x i8]*, i64)**, i32 ({}*, %"std::path::Path"*, i64, [0 x i8]*, i64)*** %16, align 8, !noalias !51, !nonnull !1
; call <std::ffi::os_str::OsStr as std::sys::unix::ext::ffi::OsStrExt>::from_bytes
  %18 = tail call { %"std::ffi::os_str::OsStr"*, i64 } @"_ZN78_$LT$std..ffi..os_str..OsStr$u20$as$u20$std..sys..unix..ext..ffi..OsStrExt$GT$10from_bytes17h6d92ecd139692d5bE"([0 x i8]* noalias nonnull readonly %10, i64 %3), !noalias !51
  %19 = extractvalue { %"std::ffi::os_str::OsStr"*, i64 } %18, 0
  %20 = extractvalue { %"std::ffi::os_str::OsStr"*, i64 } %18, 1
; call <std::ffi::os_str::OsStr as core::convert::AsRef<std::ffi::os_str::OsStr>>::as_ref
  %21 = tail call { %"std::ffi::os_str::OsStr"*, i64 } @"_ZN95_$LT$std..ffi..os_str..OsStr$u20$as$u20$core..convert..AsRef$LT$std..ffi..os_str..OsStr$GT$$GT$6as_ref17h93bfc661ae56cda7E"(%"std::ffi::os_str::OsStr"* noalias nonnull readonly %19, i64 %20), !noalias !51
  %22 = extractvalue { %"std::ffi::os_str::OsStr"*, i64 } %21, 0
  %23 = extractvalue { %"std::ffi::os_str::OsStr"*, i64 } %21, 1
  %24 = bitcast %"std::ffi::os_str::OsStr"* %22 to %"std::path::Path"*
  %25 = getelementptr inbounds i32 ({}*, %"std::path::Path"*, i64, [0 x i8]*, i64)*, i32 ({}*, %"std::path::Path"*, i64, [0 x i8]*, i64)** %17, i64 3
  %26 = load i32 ({}*, %"std::path::Path"*, i64, [0 x i8]*, i64)*, i32 ({}*, %"std::path::Path"*, i64, [0 x i8]*, i64)** %25, align 8, !invariant.load !1, !noalias !51, !nonnull !1
  %27 = icmp ne %"std::ffi::os_str::OsStr"* %22, null
  tail call void @llvm.assume(i1 %27), !noalias !51
  %28 = tail call i32 %26({}* nonnull %14, %"std::path::Path"* noalias nonnull readonly %24, i64 %23, [0 x i8]* noalias nonnull readonly %12, i64 %7), !noalias !51
  ret i32 %28
}

; <lib::index::IndexEntry as lib::util::Binding>::from_raw
; Function Attrs: nonlazybind uwtable
define void @"_ZN61_$LT$lib..index..IndexEntry$u20$as$u20$lib..util..Binding$GT$8from_raw17hd34233c2e87789a8E"(%"index::IndexEntry"* noalias nocapture sret dereferenceable(64), %"libgit2_sys::git_index_entry"* noalias nocapture readonly dereferenceable(72) %raw) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %1 = getelementptr inbounds %"libgit2_sys::git_index_entry", %"libgit2_sys::git_index_entry"* %raw, i64 0, i32 5
  %2 = load i32, i32* %1, align 8
  %3 = getelementptr inbounds %"libgit2_sys::git_index_entry", %"libgit2_sys::git_index_entry"* %raw, i64 0, i32 7
  %4 = load i32, i32* %3, align 4
  %5 = getelementptr inbounds %"libgit2_sys::git_index_entry", %"libgit2_sys::git_index_entry"* %raw, i64 0, i32 9
  %6 = load i32, i32* %5, align 8
  %7 = getelementptr inbounds %"libgit2_sys::git_index_entry", %"libgit2_sys::git_index_entry"* %raw, i64 0, i32 11
  %8 = load i32, i32* %7, align 4
  %9 = getelementptr inbounds %"libgit2_sys::git_index_entry", %"libgit2_sys::git_index_entry"* %raw, i64 0, i32 13
  %10 = load i32, i32* %9, align 8
  %11 = getelementptr inbounds %"libgit2_sys::git_index_entry", %"libgit2_sys::git_index_entry"* %raw, i64 0, i32 15
  %12 = load i32, i32* %11, align 4
  %13 = getelementptr inbounds %"libgit2_sys::git_index_entry", %"libgit2_sys::git_index_entry"* %raw, i64 0, i32 19
  %14 = load i16, i16* %13, align 4
  %15 = getelementptr inbounds %"libgit2_sys::git_index_entry", %"libgit2_sys::git_index_entry"* %raw, i64 0, i32 21
  %16 = load i16, i16* %15, align 2
  %17 = getelementptr inbounds %"libgit2_sys::git_index_entry", %"libgit2_sys::git_index_entry"* %raw, i64 0, i32 23
  %18 = bitcast i8** %17 to [0 x i8]**
  %19 = load [0 x i8]*, [0 x i8]** %18, align 8
  %20 = and i16 %14, 4095
  %21 = zext i16 %20 to i64
  %22 = icmp eq i16 %20, 0
  br i1 %22, label %"_ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$6to_vec17hc638978acbd10799E.exit", label %"_ZN47_$LT$core..result..Result$LT$T$C$$u20$E$GT$$GT$6unwrap17hf06ada516e9d7915E.exit.i.i.i.i.i"

"_ZN47_$LT$core..result..Result$LT$T$C$$u20$E$GT$$GT$6unwrap17hf06ada516e9d7915E.exit.i.i.i.i.i": ; preds = %start
  %23 = tail call i8* @__rust_alloc(i64 %21, i64 1) #9, !noalias !56
  %24 = icmp eq i8* %23, null
  br i1 %24, label %bb18.i.i.i.i.i, label %"_ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$6to_vec17hc638978acbd10799E.exit"

bb18.i.i.i.i.i:                                   ; preds = %"_ZN47_$LT$core..result..Result$LT$T$C$$u20$E$GT$$GT$6unwrap17hf06ada516e9d7915E.exit.i.i.i.i.i"
; call alloc::alloc::handle_alloc_error
  tail call void @_ZN5alloc5alloc18handle_alloc_error17haef099f38630551aE(i64 %21, i64 1), !noalias !56
  unreachable

"_ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$6to_vec17hc638978acbd10799E.exit": ; preds = %start, %"_ZN47_$LT$core..result..Result$LT$T$C$$u20$E$GT$$GT$6unwrap17hf06ada516e9d7915E.exit.i.i.i.i.i"
  %ptr.0.i.i.i.i.i = phi i8* [ inttoptr (i64 1 to i8*), %start ], [ %23, %"_ZN47_$LT$core..result..Result$LT$T$C$$u20$E$GT$$GT$6unwrap17hf06ada516e9d7915E.exit.i.i.i.i.i" ]
  %25 = getelementptr inbounds [0 x i8], [0 x i8]* %19, i64 0, i64 0
  tail call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull %ptr.0.i.i.i.i.i, i8* nonnull %25, i64 %21, i32 1, i1 false), !noalias !65
  %26 = getelementptr inbounds %"index::IndexEntry", %"index::IndexEntry"* %0, i64 0, i32 3
  store i32 0, i32* %26, align 8
  %27 = getelementptr inbounds %"index::IndexEntry", %"index::IndexEntry"* %0, i64 0, i32 5
  store i32 0, i32* %27, align 4
  %28 = getelementptr inbounds %"index::IndexEntry", %"index::IndexEntry"* %0, i64 0, i32 7
  store i32 %2, i32* %28, align 8
  %29 = getelementptr inbounds %"index::IndexEntry", %"index::IndexEntry"* %0, i64 0, i32 9
  store i32 %4, i32* %29, align 4
  %30 = getelementptr inbounds %"index::IndexEntry", %"index::IndexEntry"* %0, i64 0, i32 11
  store i32 %6, i32* %30, align 8
  %31 = getelementptr inbounds %"index::IndexEntry", %"index::IndexEntry"* %0, i64 0, i32 13
  store i32 %8, i32* %31, align 4
  %32 = getelementptr inbounds %"index::IndexEntry", %"index::IndexEntry"* %0, i64 0, i32 15
  store i32 %10, i32* %32, align 8
  %33 = getelementptr inbounds %"index::IndexEntry", %"index::IndexEntry"* %0, i64 0, i32 17
  store i32 %12, i32* %33, align 4
  %34 = getelementptr inbounds %"index::IndexEntry", %"index::IndexEntry"* %0, i64 0, i32 19
  store i32 0, i32* %34, align 8
  %35 = getelementptr inbounds %"index::IndexEntry", %"index::IndexEntry"* %0, i64 0, i32 21
  store i16 %14, i16* %35, align 4
  %36 = getelementptr inbounds %"index::IndexEntry", %"index::IndexEntry"* %0, i64 0, i32 23
  store i16 %16, i16* %36, align 2
  %_29.sroa.0.0..sroa_cast = bitcast %"index::IndexEntry"* %0 to i8**
  store i8* %ptr.0.i.i.i.i.i, i8** %_29.sroa.0.0..sroa_cast, align 8
  %_29.sroa.4.0..sroa_idx6 = getelementptr inbounds %"index::IndexEntry", %"index::IndexEntry"* %0, i64 0, i32 1, i32 1, i32 1
  store i64 %21, i64* %_29.sroa.4.0..sroa_idx6, align 8
  %_29.sroa.5.0..sroa_idx8 = getelementptr inbounds %"index::IndexEntry", %"index::IndexEntry"* %0, i64 0, i32 1, i32 3
  store i64 %21, i64* %_29.sroa.5.0..sroa_idx8, align 8
  ret void
}

; <lib::index::IndexEntry as lib::util::Binding>::raw
; Function Attrs: noreturn nonlazybind uwtable
define void @"_ZN61_$LT$lib..index..IndexEntry$u20$as$u20$lib..util..Binding$GT$3raw17h775010522187f971E"(%"libgit2_sys::git_index_entry"* noalias nocapture readnone sret dereferenceable(72), %"index::IndexEntry"* noalias nocapture readonly dereferenceable(64) %self) unnamed_addr #5 {
start:
; call std::panicking::begin_panic
  tail call fastcc void @_ZN3std9panicking11begin_panic17h472fddf7b7ec90f3E()
  unreachable
}

; lib::ObjectType::str
; Function Attrs: nonlazybind uwtable
define { [0 x i8]*, i64 } @_ZN3lib10ObjectType3str17he37b419e8f5fc8cbE(%ObjectType* noalias nocapture nonnull readonly %self) unnamed_addr #1 personality i32 (i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*)* @rust_eh_personality {
start:
  %_7.i = alloca %"core::str::Utf8Error", align 8
  %_12 = alloca %"core::result::Result<&str, core::str::Utf8Error>", align 8
  %0 = tail call i8* @git_object_type2string(i32 4)
; call std::ffi::c_str::CStr::from_ptr
  %1 = tail call { %"std::ffi::c_str::CStr"*, i64 } @_ZN3std3ffi5c_str4CStr8from_ptr17hb129c3df1c28e4bbE(i8* %0)
  %2 = extractvalue { %"std::ffi::c_str::CStr"*, i64 } %1, 1
  %3 = add i64 %2, -1
  %4 = icmp eq i64 %2, 0
  br i1 %4, label %bb4.i.i.i.i, label %_ZN3std3ffi5c_str4CStr8to_bytes17h267747ec95802125E.exit

bb4.i.i.i.i:                                      ; preds = %start
; call core::slice::slice_index_len_fail
  tail call void @_ZN4core5slice20slice_index_len_fail17h16ba3d3f76e9f048E(i64 %3, i64 0)
  unreachable

_ZN3std3ffi5c_str4CStr8to_bytes17h267747ec95802125E.exit: ; preds = %start
  %5 = extractvalue { %"std::ffi::c_str::CStr"*, i64 } %1, 0
  %6 = getelementptr inbounds %"std::ffi::c_str::CStr", %"std::ffi::c_str::CStr"* %5, i64 0, i32 0
  %7 = bitcast %"core::result::Result<&str, core::str::Utf8Error>"* %_12 to i8*
  call void @llvm.lifetime.start.p0i8(i64 24, i8* nonnull %7)
; call core::str::from_utf8
  call void @_ZN4core3str9from_utf817heaf0f1aad34b02e6E(%"core::result::Result<&str, core::str::Utf8Error>"* noalias nocapture nonnull sret dereferenceable(24) %_12, [0 x i8]* noalias nonnull readonly %6, i64 %3)
  %8 = getelementptr inbounds %"core::result::Result<&str, core::str::Utf8Error>", %"core::result::Result<&str, core::str::Utf8Error>"* %_12, i64 0, i32 0, i64 0
  %9 = load i64, i64* %8, align 8, !range !36, !alias.scope !66
  %switch.i = icmp eq i64 %9, 1
  %10 = getelementptr inbounds %"core::result::Result<&str, core::str::Utf8Error>", %"core::result::Result<&str, core::str::Utf8Error>"* %_12, i64 0, i32 2
  br i1 %switch.i, label %bb4.i, label %"_ZN47_$LT$core..result..Result$LT$T$C$$u20$E$GT$$GT$6unwrap17he53e6ce679d95354E.exit"

bb4.i:                                            ; preds = %_ZN3std3ffi5c_str4CStr8to_bytes17h267747ec95802125E.exit
  %11 = bitcast [2 x i64]* %10 to i8*
  %12 = bitcast %"core::str::Utf8Error"* %_7.i to i8*
  call void @llvm.lifetime.start.p0i8(i64 16, i8* nonnull %12), !noalias !66
  call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull %12, i8* %11, i64 16, i32 8, i1 false)
; call core::result::unwrap_failed
  call fastcc void @_ZN4core6result13unwrap_failed17h2423a2d9b55cb964E(%"core::str::Utf8Error"* noalias nocapture nonnull dereferenceable(16) %_7.i), !noalias !66
  unreachable

"_ZN47_$LT$core..result..Result$LT$T$C$$u20$E$GT$$GT$6unwrap17he53e6ce679d95354E.exit": ; preds = %_ZN3std3ffi5c_str4CStr8to_bytes17h267747ec95802125E.exit
  %13 = bitcast [2 x i64]* %10 to [0 x i8]**
  %14 = load [0 x i8]*, [0 x i8]** %13, align 8, !alias.scope !66, !nonnull !1
  %15 = getelementptr inbounds %"core::result::Result<&str, core::str::Utf8Error>", %"core::result::Result<&str, core::str::Utf8Error>"* %_12, i64 0, i32 2, i64 1
  %16 = load i64, i64* %15, align 8, !alias.scope !66
  %17 = insertvalue { [0 x i8]*, i64 } undef, [0 x i8]* %14, 0
  %18 = insertvalue { [0 x i8]*, i64 } %17, i64 %16, 1
  call void @llvm.lifetime.end.p0i8(i64 24, i8* nonnull %7)
  ret { [0 x i8]*, i64 } %18
}

; Function Attrs: nonlazybind uwtable
declare i32 @rust_eh_personality(i32, i32, i64, %"unwind::libunwind::_Unwind_Exception"*, %"unwind::libunwind::_Unwind_Context"*) unnamed_addr #6

; core::panicking::panic
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking5panic17h59ee191e7f64099dE({ [0 x i64], { [0 x i8]*, i64 }, [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly dereferenceable(40)) unnamed_addr #2

; <std::ffi::os_str::OsStr as core::convert::AsRef<std::ffi::os_str::OsStr>>::as_ref
; Function Attrs: nonlazybind uwtable
declare { %"std::ffi::os_str::OsStr"*, i64 } @"_ZN95_$LT$std..ffi..os_str..OsStr$u20$as$u20$core..convert..AsRef$LT$std..ffi..os_str..OsStr$GT$$GT$6as_ref17h93bfc661ae56cda7E"(%"std::ffi::os_str::OsStr"* noalias nonnull readonly, i64) unnamed_addr #1

; std::sys::unix::fast_thread_local::requires_move_before_drop
; Function Attrs: nonlazybind uwtable
declare zeroext i1 @_ZN3std3sys4unix17fast_thread_local25requires_move_before_drop17h550ffcf33fedbc57E() unnamed_addr #1

; std::panicking::rust_panic_with_hook
; Function Attrs: noreturn nonlazybind uwtable
declare void @_ZN3std9panicking20rust_panic_with_hook17h2abd0ff3555ca508E({}* nonnull, [3 x i64]* noalias readonly dereferenceable(24), i64* noalias readonly dereferenceable_or_null(48), { [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly dereferenceable(24)) unnamed_addr #5

; <str as core::fmt::Display>::fmt
; Function Attrs: nonlazybind uwtable
declare zeroext i1 @"_ZN42_$LT$str$u20$as$u20$core..fmt..Display$GT$3fmt17h4d06f72123a3bf61E"([0 x i8]* noalias nonnull readonly, i64, %"core::fmt::Formatter"* dereferenceable(96)) unnamed_addr #1

; Function Attrs: argmemonly nounwind
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* nocapture writeonly, i8* nocapture readonly, i64, i32, i1) #7

; std::sys::unix::fast_thread_local::register_dtor
; Function Attrs: nonlazybind uwtable
declare void @_ZN3std3sys4unix17fast_thread_local13register_dtor17h412fad7fe7b77161E(i8*, void (i8*)* nonnull) unnamed_addr #1

; alloc::alloc::handle_alloc_error
; Function Attrs: noreturn nounwind nonlazybind uwtable
declare void @_ZN5alloc5alloc18handle_alloc_error17haef099f38630551aE(i64, i64) unnamed_addr #8

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core9panicking9panic_fmt17hf3272c92bd6c0e0fE(%"core::fmt::Arguments"* noalias nocapture dereferenceable(48), { [0 x i64], { [0 x i8]*, i64 }, [0 x i32], i32, [0 x i32], i32, [0 x i32] }* noalias readonly dereferenceable(24)) unnamed_addr #2

; Function Attrs: nounwind
declare void @llvm.assume(i1) #9

; <core::cell::BorrowMutError as core::fmt::Debug>::fmt
; Function Attrs: nonlazybind uwtable
declare zeroext i1 @"_ZN63_$LT$core..cell..BorrowMutError$u20$as$u20$core..fmt..Debug$GT$3fmt17hca7d2710462e2ce1E"(%"core::cell::BorrowMutError"* noalias nonnull readonly, %"core::fmt::Formatter"* dereferenceable(96)) unnamed_addr #1

; <core::str::Utf8Error as core::fmt::Debug>::fmt
; Function Attrs: nonlazybind uwtable
declare zeroext i1 @"_ZN57_$LT$core..str..Utf8Error$u20$as$u20$core..fmt..Debug$GT$3fmt17h038d69fafa9b888eE"(%"core::str::Utf8Error"* noalias readonly dereferenceable(16), %"core::fmt::Formatter"* dereferenceable(96)) unnamed_addr #1

; <std::thread::local::AccessError as core::fmt::Debug>::fmt
; Function Attrs: nonlazybind uwtable
declare zeroext i1 @"_ZN68_$LT$std..thread..local..AccessError$u20$as$u20$core..fmt..Debug$GT$3fmt17h82e5a9d1278eec63E"(%"std::thread::local::AccessError"* noalias nonnull readonly, %"core::fmt::Formatter"* dereferenceable(96)) unnamed_addr #1

; Function Attrs: nounwind nonlazybind uwtable
declare noalias i8* @__rust_alloc(i64, i64) unnamed_addr #10

; Function Attrs: nounwind nonlazybind uwtable
declare void @__rust_dealloc(i8*, i64, i64) unnamed_addr #10

; core::slice::slice_index_len_fail
; Function Attrs: cold noinline noreturn nonlazybind uwtable
declare void @_ZN4core5slice20slice_index_len_fail17h16ba3d3f76e9f048E(i64, i64) unnamed_addr #2

; std::ffi::c_str::CStr::from_ptr
; Function Attrs: nonlazybind uwtable
declare { %"std::ffi::c_str::CStr"*, i64 } @_ZN3std3ffi5c_str4CStr8from_ptr17hb129c3df1c28e4bbE(i8*) unnamed_addr #1

; <std::ffi::os_str::OsStr as std::sys::unix::ext::ffi::OsStrExt>::from_bytes
; Function Attrs: nonlazybind uwtable
declare { %"std::ffi::os_str::OsStr"*, i64 } @"_ZN78_$LT$std..ffi..os_str..OsStr$u20$as$u20$std..sys..unix..ext..ffi..OsStrExt$GT$10from_bytes17h6d92ecd139692d5bE"([0 x i8]* noalias nonnull readonly, i64) unnamed_addr #1

; Function Attrs: nounwind nonlazybind uwtable
declare i8* @git_object_type2string(i32) unnamed_addr #10

; core::str::from_utf8
; Function Attrs: nonlazybind uwtable
declare void @_ZN4core3str9from_utf817heaf0f1aad34b02e6E(%"core::result::Result<&str, core::str::Utf8Error>"* noalias nocapture sret dereferenceable(24), [0 x i8]* noalias nonnull readonly, i64) unnamed_addr #1

; Function Attrs: argmemonly nounwind
declare void @llvm.lifetime.start.p0i8(i64, i8* nocapture) #7

; Function Attrs: argmemonly nounwind
declare void @llvm.lifetime.end.p0i8(i64, i8* nocapture) #7

; Function Attrs: argmemonly nounwind
declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i32, i1) #7

attributes #0 = { norecurse nounwind nonlazybind readnone uwtable "probe-stack"="__rust_probestack" }
attributes #1 = { nonlazybind uwtable "probe-stack"="__rust_probestack" }
attributes #2 = { cold noinline noreturn nonlazybind uwtable "probe-stack"="__rust_probestack" }
attributes #3 = { inlinehint nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" }
attributes #4 = { norecurse nounwind nonlazybind readonly uwtable "probe-stack"="__rust_probestack" }
attributes #5 = { noreturn nonlazybind uwtable "probe-stack"="__rust_probestack" }
attributes #6 = { nonlazybind uwtable "probe-stack"="__rust_probestack" "target-cpu"="x86-64" }
attributes #7 = { argmemonly nounwind }
attributes #8 = { noreturn nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" }
attributes #9 = { nounwind }
attributes #10 = { nounwind nonlazybind uwtable "probe-stack"="__rust_probestack" }
attributes #11 = { noinline }

!llvm.module.flags = !{!0}

!0 = !{i32 2, !"RtLibUseGOT", i32 1}
!1 = !{}
!2 = !{!3}
!3 = distinct !{!3, !4, !"_ZN5alloc5alloc8box_free17h215c3c281cc7c6f3E: %ptr.1"}
!4 = distinct !{!4, !"_ZN5alloc5alloc8box_free17h215c3c281cc7c6f3E"}
!5 = !{!6}
!6 = distinct !{!6, !7, !"_ZN5alloc5alloc8box_free17h215c3c281cc7c6f3E: %ptr.1"}
!7 = distinct !{!7, !"_ZN5alloc5alloc8box_free17h215c3c281cc7c6f3E"}
!8 = !{!9}
!9 = distinct !{!9, !10, !"_ZN4core3fmt9Arguments16new_v1_formatted17hf11ec085fdfcd9c5E: argument 0"}
!10 = distinct !{!10, !"_ZN4core3fmt9Arguments16new_v1_formatted17hf11ec085fdfcd9c5E"}
!11 = !{!12, !13, !14}
!12 = distinct !{!12, !10, !"_ZN4core3fmt9Arguments16new_v1_formatted17hf11ec085fdfcd9c5E: %pieces.0"}
!13 = distinct !{!13, !10, !"_ZN4core3fmt9Arguments16new_v1_formatted17hf11ec085fdfcd9c5E: %args.0"}
!14 = distinct !{!14, !10, !"_ZN4core3fmt9Arguments16new_v1_formatted17hf11ec085fdfcd9c5E: %fmt.0"}
!15 = !{!16}
!16 = distinct !{!16, !17, !"_ZN4core3fmt9Arguments16new_v1_formatted17hf11ec085fdfcd9c5E: argument 0"}
!17 = distinct !{!17, !"_ZN4core3fmt9Arguments16new_v1_formatted17hf11ec085fdfcd9c5E"}
!18 = !{!19, !20, !21}
!19 = distinct !{!19, !17, !"_ZN4core3fmt9Arguments16new_v1_formatted17hf11ec085fdfcd9c5E: %pieces.0"}
!20 = distinct !{!20, !17, !"_ZN4core3fmt9Arguments16new_v1_formatted17hf11ec085fdfcd9c5E: %args.0"}
!21 = distinct !{!21, !17, !"_ZN4core3fmt9Arguments16new_v1_formatted17hf11ec085fdfcd9c5E: %fmt.0"}
!22 = !{!23}
!23 = distinct !{!23, !24, !"_ZN4core3fmt9Arguments16new_v1_formatted17hf11ec085fdfcd9c5E: argument 0"}
!24 = distinct !{!24, !"_ZN4core3fmt9Arguments16new_v1_formatted17hf11ec085fdfcd9c5E"}
!25 = !{!26, !27, !28}
!26 = distinct !{!26, !24, !"_ZN4core3fmt9Arguments16new_v1_formatted17hf11ec085fdfcd9c5E: %pieces.0"}
!27 = distinct !{!27, !24, !"_ZN4core3fmt9Arguments16new_v1_formatted17hf11ec085fdfcd9c5E: %args.0"}
!28 = distinct !{!28, !24, !"_ZN4core3fmt9Arguments16new_v1_formatted17hf11ec085fdfcd9c5E: %fmt.0"}
!29 = !{!30}
!30 = distinct !{!30, !31, !"_ZN35_$LT$alloc..boxed..Box$LT$T$GT$$GT$3new17h4fdb857929c782c4E: %x.0"}
!31 = distinct !{!31, !"_ZN35_$LT$alloc..boxed..Box$LT$T$GT$$GT$3new17h4fdb857929c782c4E"}
!32 = !{i8 0, i8 2}
!33 = !{!34}
!34 = distinct !{!34, !35, !"_ZN46_$LT$std..thread..local..LocalKey$LT$T$GT$$GT$8try_with17hecf89db7c9eb249cE: argument 0"}
!35 = distinct !{!35, !"_ZN46_$LT$std..thread..local..LocalKey$LT$T$GT$$GT$8try_with17hecf89db7c9eb249cE"}
!36 = !{i64 0, i64 2}
!37 = !{!38, !40, !34}
!38 = distinct !{!38, !39, !"_ZN4core3mem7replace17h05b6ddc6b02376b9E: argument 0"}
!39 = distinct !{!39, !"_ZN4core3mem7replace17h05b6ddc6b02376b9E"}
!40 = distinct !{!40, !39, !"_ZN4core3mem7replace17h05b6ddc6b02376b9E: %src"}
!41 = !{!42}
!42 = distinct !{!42, !43, !"_ZN5alloc5alloc8box_free17h215c3c281cc7c6f3E: %ptr.1"}
!43 = distinct !{!43, !"_ZN5alloc5alloc8box_free17h215c3c281cc7c6f3E"}
!44 = !{!42, !34}
!45 = !{!46}
!46 = distinct !{!46, !47, !"_ZN47_$LT$core..result..Result$LT$T$C$$u20$E$GT$$GT$6expect17h5f60cc568f401c20E: %self"}
!47 = distinct !{!47, !"_ZN47_$LT$core..result..Result$LT$T$C$$u20$E$GT$$GT$6expect17h5f60cc568f401c20E"}
!48 = !{!49}
!49 = distinct !{!49, !50, !"_ZN5alloc5alloc8box_free17h215c3c281cc7c6f3E: %ptr.1"}
!50 = distinct !{!50, !"_ZN5alloc5alloc8box_free17h215c3c281cc7c6f3E"}
!51 = !{!52, !54}
!52 = distinct !{!52, !53, !"_ZN3lib5index21index_matched_path_cb28_$u7b$$u7b$closure$u7d$$u7d$17h8244bc8ed4ef70d2E: %arg0"}
!53 = distinct !{!53, !"_ZN3lib5index21index_matched_path_cb28_$u7b$$u7b$closure$u7d$$u7d$17h8244bc8ed4ef70d2E"}
!54 = distinct !{!54, !55, !"_ZN3lib5panic4wrap17h3070a2a1169f8203E: %f"}
!55 = distinct !{!55, !"_ZN3lib5panic4wrap17h3070a2a1169f8203E"}
!56 = !{!57, !59, !61, !62, !64}
!57 = distinct !{!57, !58, !"_ZN33_$LT$alloc..vec..Vec$LT$T$GT$$GT$13with_capacity17h5680cec8769623e1E: argument 0"}
!58 = distinct !{!58, !"_ZN33_$LT$alloc..vec..Vec$LT$T$GT$$GT$13with_capacity17h5680cec8769623e1E"}
!59 = distinct !{!59, !60, !"_ZN5alloc5slice4hack6to_vec17hd797bd46500e9626E: argument 0"}
!60 = distinct !{!60, !"_ZN5alloc5slice4hack6to_vec17hd797bd46500e9626E"}
!61 = distinct !{!61, !60, !"_ZN5alloc5slice4hack6to_vec17hd797bd46500e9626E: %s.0"}
!62 = distinct !{!62, !63, !"_ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$6to_vec17hc638978acbd10799E: argument 0"}
!63 = distinct !{!63, !"_ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$6to_vec17hc638978acbd10799E"}
!64 = distinct !{!64, !63, !"_ZN5alloc5slice29_$LT$impl$u20$$u5b$T$u5d$$GT$6to_vec17hc638978acbd10799E: %self.0"}
!65 = !{!59, !62}
!66 = !{!67}
!67 = distinct !{!67, !68, !"_ZN47_$LT$core..result..Result$LT$T$C$$u20$E$GT$$GT$6unwrap17he53e6ce679d95354E: %self"}
!68 = distinct !{!68, !"_ZN47_$LT$core..result..Result$LT$T$C$$u20$E$GT$$GT$6unwrap17he53e6ce679d95354E"}
