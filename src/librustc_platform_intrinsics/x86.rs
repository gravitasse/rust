// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// DO NOT EDIT: autogenerated by etc/platform-intrinsics/generator.py
// ignore-tidy-linelength

#![allow(unused_imports)]

use {Intrinsic, i, i_, u, u_, f, v, agg, p, void};
use IntrinsicDef::Named;
use rustc::middle::ty;

pub fn find<'tcx>(_tcx: &ty::ctxt<'tcx>, name: &str) -> Option<Intrinsic> {
    if !name.starts_with("x86_mm") { return None }
    Some(match &name["x86_mm".len()..] {
        "_movemask_ps" => Intrinsic {
            inputs: vec![v(f(32), 4)],
            output: i(32),
            definition: Named("llvm.x86.sse.movmsk.ps")
        },
        "_max_ps" => Intrinsic {
            inputs: vec![v(f(32), 4), v(f(32), 4)],
            output: v(f(32), 4),
            definition: Named("llvm.x86.sse.max.ps")
        },
        "_min_ps" => Intrinsic {
            inputs: vec![v(f(32), 4), v(f(32), 4)],
            output: v(f(32), 4),
            definition: Named("llvm.x86.sse.min.ps")
        },
        "_rsqrt_ps" => Intrinsic {
            inputs: vec![v(f(32), 4)],
            output: v(f(32), 4),
            definition: Named("llvm.x86.sse.rsqrt.ps")
        },
        "_rcp_ps" => Intrinsic {
            inputs: vec![v(f(32), 4)],
            output: v(f(32), 4),
            definition: Named("llvm.x86.sse.rcp.ps")
        },
        "_sqrt_ps" => Intrinsic {
            inputs: vec![v(f(32), 4)],
            output: v(f(32), 4),
            definition: Named("llvm.sqrt.v4f32")
        },
        "_adds_epi8" => Intrinsic {
            inputs: vec![v(i(8), 16), v(i(8), 16)],
            output: v(i(8), 16),
            definition: Named("llvm.x86.sse2.padds.b")
        },
        "_adds_epu8" => Intrinsic {
            inputs: vec![v(u(8), 16), v(u(8), 16)],
            output: v(u(8), 16),
            definition: Named("llvm.x86.sse2.paddus.b")
        },
        "_adds_epi16" => Intrinsic {
            inputs: vec![v(i(16), 8), v(i(16), 8)],
            output: v(i(16), 8),
            definition: Named("llvm.x86.sse2.padds.w")
        },
        "_adds_epu16" => Intrinsic {
            inputs: vec![v(u(16), 8), v(u(16), 8)],
            output: v(u(16), 8),
            definition: Named("llvm.x86.sse2.paddus.w")
        },
        "_avg_epu8" => Intrinsic {
            inputs: vec![v(u(8), 16), v(u(8), 16)],
            output: v(u(8), 16),
            definition: Named("llvm.x86.sse2.pavg.b")
        },
        "_avg_epu16" => Intrinsic {
            inputs: vec![v(u(16), 8), v(u(16), 8)],
            output: v(u(16), 8),
            definition: Named("llvm.x86.sse2.pavg.w")
        },
        "_madd_epi16" => Intrinsic {
            inputs: vec![v(i(16), 8), v(i(16), 8)],
            output: v(i(32), 4),
            definition: Named("llvm.x86.sse2.pmadd.wd")
        },
        "_max_epi16" => Intrinsic {
            inputs: vec![v(i(16), 8), v(i(16), 8)],
            output: v(i(16), 8),
            definition: Named("llvm.x86.sse2.pmaxs.w")
        },
        "_max_epu8" => Intrinsic {
            inputs: vec![v(u(8), 16), v(u(8), 16)],
            output: v(u(8), 16),
            definition: Named("llvm.x86.sse2.pmaxu.b")
        },
        "_max_pd" => Intrinsic {
            inputs: vec![v(f(64), 2), v(f(64), 2)],
            output: v(f(64), 2),
            definition: Named("llvm.x86.sse2.max.pd")
        },
        "_min_epi16" => Intrinsic {
            inputs: vec![v(i(16), 8), v(i(16), 8)],
            output: v(i(16), 8),
            definition: Named("llvm.x86.sse2.pmins.w")
        },
        "_min_epu8" => Intrinsic {
            inputs: vec![v(u(8), 16), v(u(8), 16)],
            output: v(u(8), 16),
            definition: Named("llvm.x86.sse2.pminu.b")
        },
        "_min_pd" => Intrinsic {
            inputs: vec![v(f(64), 2), v(f(64), 2)],
            output: v(f(64), 2),
            definition: Named("llvm.x86.sse2.min.pd")
        },
        "_movemask_pd" => Intrinsic {
            inputs: vec![v(f(64), 2)],
            output: i(32),
            definition: Named("llvm.x86.sse2.movmsk.pd")
        },
        "_movemask_epi8" => Intrinsic {
            inputs: vec![v(i(8), 16)],
            output: i(32),
            definition: Named("llvm.x86.sse2.pmovmskb.128")
        },
        "_mul_epu32" => Intrinsic {
            inputs: vec![v(u(32), 4), v(u(32), 4)],
            output: v(u(64), 2),
            definition: Named("llvm.x86.sse2.pmulu.dq")
        },
        "_mulhi_epi16" => Intrinsic {
            inputs: vec![v(i(16), 8), v(i(16), 8)],
            output: v(i(16), 8),
            definition: Named("llvm.x86.sse2.pmulh.w")
        },
        "_mulhi_epu16" => Intrinsic {
            inputs: vec![v(u(16), 8), v(u(16), 8)],
            output: v(u(16), 8),
            definition: Named("llvm.x86.sse2.pmulhu.w")
        },
        "_packs_epi16" => Intrinsic {
            inputs: vec![v(i(16), 8), v(i(16), 8)],
            output: v(i(8), 16),
            definition: Named("llvm.x86.sse2.packsswb.128")
        },
        "_packs_epi32" => Intrinsic {
            inputs: vec![v(i(32), 4), v(i(32), 4)],
            output: v(i(16), 8),
            definition: Named("llvm.x86.sse2.packssdw.128")
        },
        "_packus_epi16" => Intrinsic {
            inputs: vec![v(i(16), 8), v(i(16), 8)],
            output: v(u(8), 16),
            definition: Named("llvm.x86.sse2.packuswb.128")
        },
        "_sad_epu8" => Intrinsic {
            inputs: vec![v(u(8), 16), v(u(8), 16)],
            output: v(u(64), 2),
            definition: Named("llvm.x86.sse2.psad.bw")
        },
        "_sqrt_pd" => Intrinsic {
            inputs: vec![v(f(64), 2)],
            output: v(f(64), 2),
            definition: Named("llvm.sqrt.v2f64")
        },
        "_subs_epi8" => Intrinsic {
            inputs: vec![v(i(8), 16), v(i(8), 16)],
            output: v(i(8), 16),
            definition: Named("llvm.x86.sse2.psubs.b")
        },
        "_subs_epu8" => Intrinsic {
            inputs: vec![v(u(8), 16), v(u(8), 16)],
            output: v(u(8), 16),
            definition: Named("llvm.x86.sse2.psubus.b")
        },
        "_subs_epi16" => Intrinsic {
            inputs: vec![v(i(16), 8), v(i(16), 8)],
            output: v(i(16), 8),
            definition: Named("llvm.x86.sse2.psubs.w")
        },
        "_subs_epu16" => Intrinsic {
            inputs: vec![v(u(16), 8), v(u(16), 8)],
            output: v(u(16), 8),
            definition: Named("llvm.x86.sse2.psubus.w")
        },
        "_addsub_ps" => Intrinsic {
            inputs: vec![v(f(32), 4), v(f(32), 4)],
            output: v(f(32), 4),
            definition: Named("llvm.x86.sse3.addsub.ps")
        },
        "_addsub_pd" => Intrinsic {
            inputs: vec![v(f(64), 2), v(f(64), 2)],
            output: v(f(64), 2),
            definition: Named("llvm.x86.sse3.addsub.pd")
        },
        "_hadd_ps" => Intrinsic {
            inputs: vec![v(f(32), 4), v(f(32), 4)],
            output: v(f(32), 4),
            definition: Named("llvm.x86.sse3.hadd.ps")
        },
        "_hadd_pd" => Intrinsic {
            inputs: vec![v(f(64), 2), v(f(64), 2)],
            output: v(f(64), 2),
            definition: Named("llvm.x86.sse3.hadd.pd")
        },
        "_hsub_ps" => Intrinsic {
            inputs: vec![v(f(32), 4), v(f(32), 4)],
            output: v(f(32), 4),
            definition: Named("llvm.x86.sse3.hsub.ps")
        },
        "_hsub_pd" => Intrinsic {
            inputs: vec![v(f(64), 2), v(f(64), 2)],
            output: v(f(64), 2),
            definition: Named("llvm.x86.sse3.hsub.pd")
        },
        "_abs_epi8" => Intrinsic {
            inputs: vec![v(i(8), 16)],
            output: v(i(8), 16),
            definition: Named("llvm.x86.ssse3.pabs.b.128")
        },
        "_abs_epi16" => Intrinsic {
            inputs: vec![v(i(16), 8)],
            output: v(i(16), 8),
            definition: Named("llvm.x86.ssse3.pabs.w.128")
        },
        "_abs_epi32" => Intrinsic {
            inputs: vec![v(i(32), 4)],
            output: v(i(32), 4),
            definition: Named("llvm.x86.ssse3.pabs.d.128")
        },
        "_hadd_epi16" => Intrinsic {
            inputs: vec![v(i(16), 8), v(i(16), 8)],
            output: v(i(16), 8),
            definition: Named("llvm.x86.ssse3.phadd.w.128")
        },
        "_hadd_epi32" => Intrinsic {
            inputs: vec![v(i(32), 4), v(i(32), 4)],
            output: v(i(32), 4),
            definition: Named("llvm.x86.ssse3.phadd.d.128")
        },
        "_hadds_epi16" => Intrinsic {
            inputs: vec![v(i(16), 8), v(i(16), 8)],
            output: v(i(16), 8),
            definition: Named("llvm.x86.ssse3.phadd.sw.128")
        },
        "_hsub_epi16" => Intrinsic {
            inputs: vec![v(i(16), 8), v(i(16), 8)],
            output: v(i(16), 8),
            definition: Named("llvm.x86.ssse3.phsub.w.128")
        },
        "_hsub_epi32" => Intrinsic {
            inputs: vec![v(i(32), 4), v(i(32), 4)],
            output: v(i(32), 4),
            definition: Named("llvm.x86.ssse3.phsub.d.128")
        },
        "_hsubs_epi16" => Intrinsic {
            inputs: vec![v(i(16), 8), v(i(16), 8)],
            output: v(i(16), 8),
            definition: Named("llvm.x86.ssse3.phsub.sw.128")
        },
        "_maddubs_epi16" => Intrinsic {
            inputs: vec![v(u(8), 16), v(i(8), 16)],
            output: v(i(16), 8),
            definition: Named("llvm.x86.ssse3.pmadd.ub.sw.128")
        },
        "_mulhrs_epi16" => Intrinsic {
            inputs: vec![v(i(16), 8), v(i(16), 8)],
            output: v(i(16), 8),
            definition: Named("llvm.x86.ssse3.pmul.hr.sw.128")
        },
        "_shuffle_epi8" => Intrinsic {
            inputs: vec![v(i(8), 16), v(i(8), 16)],
            output: v(i(8), 16),
            definition: Named("llvm.x86.ssse3.pshuf.b.128")
        },
        "_sign_epi8" => Intrinsic {
            inputs: vec![v(i(8), 16), v(i(8), 16)],
            output: v(i(8), 16),
            definition: Named("llvm.x86.ssse3.psign.b.128")
        },
        "_sign_epi16" => Intrinsic {
            inputs: vec![v(i(16), 8), v(i(16), 8)],
            output: v(i(16), 8),
            definition: Named("llvm.x86.ssse3.psign.w.128")
        },
        "_sign_epi32" => Intrinsic {
            inputs: vec![v(i(32), 4), v(i(32), 4)],
            output: v(i(32), 4),
            definition: Named("llvm.x86.ssse3.psign.d.128")
        },
        "_dp_ps" => Intrinsic {
            inputs: vec![v(f(32), 4), v(f(32), 4), i_(32, 8)],
            output: v(f(32), 4),
            definition: Named("llvm.x86.sse41.dpps")
        },
        "_dp_pd" => Intrinsic {
            inputs: vec![v(f(64), 2), v(f(64), 2), i_(32, 8)],
            output: v(f(64), 2),
            definition: Named("llvm.x86.sse41.dppd")
        },
        "_max_epi8" => Intrinsic {
            inputs: vec![v(i(8), 16), v(i(8), 16)],
            output: v(i(8), 16),
            definition: Named("llvm.x86.sse41.pmaxsb")
        },
        "_max_epu16" => Intrinsic {
            inputs: vec![v(u(16), 8), v(u(16), 8)],
            output: v(u(16), 8),
            definition: Named("llvm.x86.sse41.pmaxuw")
        },
        "_max_epi32" => Intrinsic {
            inputs: vec![v(i(32), 4), v(i(32), 4)],
            output: v(i(32), 4),
            definition: Named("llvm.x86.sse41.pmaxsd")
        },
        "_max_epu32" => Intrinsic {
            inputs: vec![v(u(32), 4), v(u(32), 4)],
            output: v(u(32), 4),
            definition: Named("llvm.x86.sse41.pmaxud")
        },
        "_min_epi8" => Intrinsic {
            inputs: vec![v(i(8), 16), v(i(8), 16)],
            output: v(i(8), 16),
            definition: Named("llvm.x86.sse41.pminsb")
        },
        "_min_epu16" => Intrinsic {
            inputs: vec![v(u(16), 8), v(u(16), 8)],
            output: v(u(16), 8),
            definition: Named("llvm.x86.sse41.pminuw")
        },
        "_min_epi32" => Intrinsic {
            inputs: vec![v(i(32), 4), v(i(32), 4)],
            output: v(i(32), 4),
            definition: Named("llvm.x86.sse41.pminsd")
        },
        "_min_epu32" => Intrinsic {
            inputs: vec![v(u(32), 4), v(u(32), 4)],
            output: v(u(32), 4),
            definition: Named("llvm.x86.sse41.pminud")
        },
        "_minpos_epu16" => Intrinsic {
            inputs: vec![v(u(16), 8)],
            output: v(u(16), 8),
            definition: Named("llvm.x86.sse41.phminposuw")
        },
        "_mpsadbw_epu8" => Intrinsic {
            inputs: vec![v(u(8), 16), v(u(8), 16), i_(32, 8)],
            output: v(u(16), 8),
            definition: Named("llvm.x86.sse41.mpsadbw")
        },
        "_mul_epi32" => Intrinsic {
            inputs: vec![v(i(32), 4), v(i(32), 4)],
            output: v(i(64), 2),
            definition: Named("llvm.x86.sse41.pmuldq")
        },
        "_packus_epi32" => Intrinsic {
            inputs: vec![v(i(32), 4), v(i(32), 4)],
            output: v(u(16), 8),
            definition: Named("llvm.x86.sse41.packusdw")
        },
        "_testc_si128" => Intrinsic {
            inputs: vec![v(u(64), 2), v(u(64), 2)],
            output: i(32),
            definition: Named("llvm.x86.sse41.ptestc")
        },
        "_testnzc_si128" => Intrinsic {
            inputs: vec![v(u(64), 2), v(u(64), 2)],
            output: i(32),
            definition: Named("llvm.x86.sse41.ptestnzc")
        },
        "_testz_si128" => Intrinsic {
            inputs: vec![v(u(64), 2), v(u(64), 2)],
            output: i(32),
            definition: Named("llvm.x86.sse41.ptestz")
        },
        "_cmpestra" => Intrinsic {
            inputs: vec![v(i(8), 16), i(32), v(i(8), 16), i(32), i_(32, 8)],
            output: i(32),
            definition: Named("llvm.x86.sse42.pcmpestria128")
        },
        "_cmpestrc" => Intrinsic {
            inputs: vec![v(i(8), 16), i(32), v(i(8), 16), i(32), i_(32, 8)],
            output: i(32),
            definition: Named("llvm.x86.sse42.pcmpestric128")
        },
        "_cmpestri" => Intrinsic {
            inputs: vec![v(i(8), 16), i(32), v(i(8), 16), i(32), i_(32, 8)],
            output: i(32),
            definition: Named("llvm.x86.sse42.pcmpestri128")
        },
        "_cmpestrm" => Intrinsic {
            inputs: vec![v(i(8), 16), i(32), v(i(8), 16), i(32), i_(32, 8)],
            output: v(i(8), 16),
            definition: Named("llvm.x86.sse42.pcmpestrm128")
        },
        "_cmpestro" => Intrinsic {
            inputs: vec![v(i(8), 16), i(32), v(i(8), 16), i(32), i_(32, 8)],
            output: i(32),
            definition: Named("llvm.x86.sse42.pcmpestrio128")
        },
        "_cmpestrs" => Intrinsic {
            inputs: vec![v(i(8), 16), i(32), v(i(8), 16), i(32), i_(32, 8)],
            output: i(32),
            definition: Named("llvm.x86.sse42.pcmpestris128")
        },
        "_cmpestrz" => Intrinsic {
            inputs: vec![v(i(8), 16), i(32), v(i(8), 16), i(32), i_(32, 8)],
            output: i(32),
            definition: Named("llvm.x86.sse42.pcmpestriz128")
        },
        "_cmpistra" => Intrinsic {
            inputs: vec![v(i(8), 16), v(i(8), 16), i_(32, 8)],
            output: i(32),
            definition: Named("llvm.x86.sse42.pcmpistria128")
        },
        "_cmpistrc" => Intrinsic {
            inputs: vec![v(i(8), 16), v(i(8), 16), i_(32, 8)],
            output: i(32),
            definition: Named("llvm.x86.sse42.pcmpistric128")
        },
        "_cmpistri" => Intrinsic {
            inputs: vec![v(i(8), 16), v(i(8), 16), i_(32, 8)],
            output: i(32),
            definition: Named("llvm.x86.sse42.pcmpistri128")
        },
        "_cmpistrm" => Intrinsic {
            inputs: vec![v(i(8), 16), v(i(8), 16), i_(32, 8)],
            output: v(i(8), 16),
            definition: Named("llvm.x86.sse42.pcmpistrm128")
        },
        "_cmpistro" => Intrinsic {
            inputs: vec![v(i(8), 16), v(i(8), 16), i_(32, 8)],
            output: i(32),
            definition: Named("llvm.x86.sse42.pcmpistrio128")
        },
        "_cmpistrs" => Intrinsic {
            inputs: vec![v(i(8), 16), v(i(8), 16), i_(32, 8)],
            output: i(32),
            definition: Named("llvm.x86.sse42.pcmpistris128")
        },
        "_cmpistrz" => Intrinsic {
            inputs: vec![v(i(8), 16), v(i(8), 16), i_(32, 8)],
            output: i(32),
            definition: Named("llvm.x86.sse42.pcmpistriz128")
        },
        "256_addsub_ps" => Intrinsic {
            inputs: vec![v(f(32), 8), v(f(32), 8)],
            output: v(f(32), 8),
            definition: Named("llvm.x86.avx.addsub.ps.256")
        },
        "256_addsub_pd" => Intrinsic {
            inputs: vec![v(f(64), 4), v(f(64), 4)],
            output: v(f(64), 4),
            definition: Named("llvm.x86.avx.addsub.pd.256")
        },
        "256_dp_ps" => Intrinsic {
            inputs: vec![v(f(32), 8), v(f(32), 8), i_(32, 8)],
            output: v(f(32), 8),
            definition: Named("llvm.x86.avx.dp.ps.256")
        },
        "256_hadd_ps" => Intrinsic {
            inputs: vec![v(f(32), 8), v(f(32), 8)],
            output: v(f(32), 8),
            definition: Named("llvm.x86.avx.hadd.ps.256")
        },
        "256_hadd_pd" => Intrinsic {
            inputs: vec![v(f(64), 4), v(f(64), 4)],
            output: v(f(64), 4),
            definition: Named("llvm.x86.avx.hadd.pd.256")
        },
        "256_hsub_ps" => Intrinsic {
            inputs: vec![v(f(32), 8), v(f(32), 8)],
            output: v(f(32), 8),
            definition: Named("llvm.x86.avx.hsub.ps.256")
        },
        "256_hsub_pd" => Intrinsic {
            inputs: vec![v(f(64), 4), v(f(64), 4)],
            output: v(f(64), 4),
            definition: Named("llvm.x86.avx.hsub.pd.256")
        },
        "256_max_ps" => Intrinsic {
            inputs: vec![v(f(32), 8), v(f(32), 8)],
            output: v(f(32), 8),
            definition: Named("llvm.x86.avx.max.ps.256")
        },
        "256_max_pd" => Intrinsic {
            inputs: vec![v(f(64), 4), v(f(64), 4)],
            output: v(f(64), 4),
            definition: Named("llvm.x86.avx.max.pd.256")
        },
        "256_min_ps" => Intrinsic {
            inputs: vec![v(f(32), 8), v(f(32), 8)],
            output: v(f(32), 8),
            definition: Named("llvm.x86.avx.min.ps.256")
        },
        "256_min_pd" => Intrinsic {
            inputs: vec![v(f(64), 4), v(f(64), 4)],
            output: v(f(64), 4),
            definition: Named("llvm.x86.avx.min.pd.256")
        },
        "256_movemask_ps" => Intrinsic {
            inputs: vec![v(f(32), 8)],
            output: i(32),
            definition: Named("llvm.x86.avx.movmsk.ps.256")
        },
        "256_movemask_pd" => Intrinsic {
            inputs: vec![v(f(64), 4)],
            output: i(32),
            definition: Named("llvm.x86.avx.movmsk.pd.256")
        },
        "_permutevar_ps" => Intrinsic {
            inputs: vec![v(f(32), 4), v(i(32), 4)],
            output: v(f(32), 4),
            definition: Named("llvm.x86.avx.vpermilvar.ps")
        },
        "_permutevar_pd" => Intrinsic {
            inputs: vec![v(f(64), 2), v(i(64), 2)],
            output: v(f(64), 2),
            definition: Named("llvm.x86.avx.vpermilvar.pd")
        },
        "256_permutevar_ps" => Intrinsic {
            inputs: vec![v(f(32), 8), v(i(32), 8)],
            output: v(f(32), 8),
            definition: Named("llvm.x86.avx.vpermilvar.ps.256")
        },
        "256_permutevar_pd" => Intrinsic {
            inputs: vec![v(f(64), 4), v(i(64), 4)],
            output: v(f(64), 4),
            definition: Named("llvm.x86.avx.vpermilvar.pd.256")
        },
        "256_rcp_ps" => Intrinsic {
            inputs: vec![v(f(32), 8)],
            output: v(f(32), 8),
            definition: Named("llvm.x86.avx.rcp.ps.256")
        },
        "256_rsqrt_ps" => Intrinsic {
            inputs: vec![v(f(32), 8)],
            output: v(f(32), 8),
            definition: Named("llvm.x86.avx.rsqrt.ps.256")
        },
        "256_sqrt_ps" => Intrinsic {
            inputs: vec![v(f(32), 8)],
            output: v(f(32), 8),
            definition: Named("llvm.sqrt.v8f32")
        },
        "256_sqrt_pd" => Intrinsic {
            inputs: vec![v(f(64), 4)],
            output: v(f(64), 4),
            definition: Named("llvm.sqrt.v4f64")
        },
        "_testc_ps" => Intrinsic {
            inputs: vec![v(f(32), 4), v(f(32), 4)],
            output: i(32),
            definition: Named("llvm.x86.avx.vtestc.ps")
        },
        "256_testc_ps" => Intrinsic {
            inputs: vec![v(f(32), 8), v(f(32), 8)],
            output: i(32),
            definition: Named("llvm.x86.avx.vtestc.ps.256")
        },
        "_testc_pd" => Intrinsic {
            inputs: vec![v(f(64), 2), v(f(64), 2)],
            output: i(32),
            definition: Named("llvm.x86.avx.vtestc.pd")
        },
        "256_testc_pd" => Intrinsic {
            inputs: vec![v(f(64), 4), v(f(64), 4)],
            output: i(32),
            definition: Named("llvm.x86.avx.vtestc.pd.256")
        },
        "256_testc_si256" => Intrinsic {
            inputs: vec![v(u(64), 4), v(u(64), 4)],
            output: i(32),
            definition: Named("llvm.x86.avx.ptestc.256")
        },
        "_testnzc_ps" => Intrinsic {
            inputs: vec![v(f(32), 4), v(f(32), 4)],
            output: i(32),
            definition: Named("llvm.x86.avx.vtestnzc.ps")
        },
        "256_testnzc_ps" => Intrinsic {
            inputs: vec![v(f(32), 8), v(f(32), 8)],
            output: i(32),
            definition: Named("llvm.x86.avx.vtestnzc.ps.256")
        },
        "_testnzc_pd" => Intrinsic {
            inputs: vec![v(f(64), 2), v(f(64), 2)],
            output: i(32),
            definition: Named("llvm.x86.avx.vtestnzc.pd")
        },
        "256_testnzc_pd" => Intrinsic {
            inputs: vec![v(f(64), 4), v(f(64), 4)],
            output: i(32),
            definition: Named("llvm.x86.avx.vtestnzc.pd.256")
        },
        "256_testnzc_si256" => Intrinsic {
            inputs: vec![v(u(64), 4), v(u(64), 4)],
            output: i(32),
            definition: Named("llvm.x86.avx.ptestnzc.256")
        },
        "_testz_ps" => Intrinsic {
            inputs: vec![v(f(32), 4), v(f(32), 4)],
            output: i(32),
            definition: Named("llvm.x86.avx.vtestz.ps")
        },
        "256_testz_ps" => Intrinsic {
            inputs: vec![v(f(32), 8), v(f(32), 8)],
            output: i(32),
            definition: Named("llvm.x86.avx.vtestz.ps.256")
        },
        "_testz_pd" => Intrinsic {
            inputs: vec![v(f(64), 2), v(f(64), 2)],
            output: i(32),
            definition: Named("llvm.x86.avx.vtestz.pd")
        },
        "256_testz_pd" => Intrinsic {
            inputs: vec![v(f(64), 4), v(f(64), 4)],
            output: i(32),
            definition: Named("llvm.x86.avx.vtestz.pd.256")
        },
        "256_testz_si256" => Intrinsic {
            inputs: vec![v(u(64), 4), v(u(64), 4)],
            output: i(32),
            definition: Named("llvm.x86.avx.ptestz.256")
        },
        "256_abs_epi8" => Intrinsic {
            inputs: vec![v(i(8), 32)],
            output: v(i(8), 32),
            definition: Named("llvm.x86.avx2.avx2.pabs.b")
        },
        "256_abs_epi16" => Intrinsic {
            inputs: vec![v(i(16), 16)],
            output: v(i(16), 16),
            definition: Named("llvm.x86.avx2.avx2.pabs.w")
        },
        "256_abs_epi32" => Intrinsic {
            inputs: vec![v(i(32), 8)],
            output: v(i(32), 8),
            definition: Named("llvm.x86.avx2.avx2.pabs.d")
        },
        "256_adds_epi8" => Intrinsic {
            inputs: vec![v(i(8), 32), v(i(8), 32)],
            output: v(i(8), 32),
            definition: Named("llvm.x86.avx2.avx2.padds.b")
        },
        "256_adds_epu8" => Intrinsic {
            inputs: vec![v(u(8), 32), v(u(8), 32)],
            output: v(u(8), 32),
            definition: Named("llvm.x86.avx2.avx2.paddus.b")
        },
        "256_adds_epi16" => Intrinsic {
            inputs: vec![v(i(16), 16), v(i(16), 16)],
            output: v(i(16), 16),
            definition: Named("llvm.x86.avx2.avx2.padds.w")
        },
        "256_adds_epu16" => Intrinsic {
            inputs: vec![v(u(16), 16), v(u(16), 16)],
            output: v(u(16), 16),
            definition: Named("llvm.x86.avx2.avx2.paddus.w")
        },
        "256_avg_epu8" => Intrinsic {
            inputs: vec![v(u(8), 32), v(u(8), 32)],
            output: v(u(8), 32),
            definition: Named("llvm.x86.avx2.avx2.pavg.b")
        },
        "256_avg_epu16" => Intrinsic {
            inputs: vec![v(u(16), 16), v(u(16), 16)],
            output: v(u(16), 16),
            definition: Named("llvm.x86.avx2.avx2.pavg.w")
        },
        "256_hadd_epi16" => Intrinsic {
            inputs: vec![v(i(16), 16), v(i(16), 16)],
            output: v(i(16), 16),
            definition: Named("llvm.x86.avx2.phadd.w")
        },
        "256_hadd_epi32" => Intrinsic {
            inputs: vec![v(i(32), 8), v(i(32), 8)],
            output: v(i(32), 8),
            definition: Named("llvm.x86.avx2.phadd.d")
        },
        "256_hadds_epi16" => Intrinsic {
            inputs: vec![v(i(16), 16), v(i(16), 16)],
            output: v(i(16), 16),
            definition: Named("llvm.x86.avx2.phadd.sw")
        },
        "256_hsub_epi16" => Intrinsic {
            inputs: vec![v(i(16), 16), v(i(16), 16)],
            output: v(i(16), 16),
            definition: Named("llvm.x86.avx2.phsub.w")
        },
        "256_hsub_epi32" => Intrinsic {
            inputs: vec![v(i(32), 8), v(i(32), 8)],
            output: v(i(32), 8),
            definition: Named("llvm.x86.avx2.phsub.d")
        },
        "256_hsubs_epi16" => Intrinsic {
            inputs: vec![v(i(16), 16), v(i(16), 16)],
            output: v(i(16), 16),
            definition: Named("llvm.x86.avx2.phsub.sw")
        },
        "256_madd_epi16" => Intrinsic {
            inputs: vec![v(i(16), 16), v(i(16), 16)],
            output: v(i(32), 8),
            definition: Named("llvm.x86.avx2.pmadd.wd")
        },
        "256_maddubs_epi16" => Intrinsic {
            inputs: vec![v(i(8), 32), v(i(8), 32)],
            output: v(i(16), 16),
            definition: Named("llvm.x86.avx2.pmadd.ub.sw")
        },
        "256_max_epi8" => Intrinsic {
            inputs: vec![v(i(8), 32), v(i(8), 32)],
            output: v(i(8), 32),
            definition: Named("llvm.x86.avx2.pmaxs.b")
        },
        "256_max_epu8" => Intrinsic {
            inputs: vec![v(u(8), 32), v(u(8), 32)],
            output: v(u(8), 32),
            definition: Named("llvm.x86.avx2.pmaxu.b")
        },
        "256_max_epi16" => Intrinsic {
            inputs: vec![v(i(16), 16), v(i(16), 16)],
            output: v(i(16), 16),
            definition: Named("llvm.x86.avx2.pmaxs.w")
        },
        "256_max_epu16" => Intrinsic {
            inputs: vec![v(u(16), 16), v(u(16), 16)],
            output: v(u(16), 16),
            definition: Named("llvm.x86.avx2.pmaxu.w")
        },
        "256_max_epi32" => Intrinsic {
            inputs: vec![v(i(32), 8), v(i(32), 8)],
            output: v(i(32), 8),
            definition: Named("llvm.x86.avx2.pmaxs.d")
        },
        "256_max_epu32" => Intrinsic {
            inputs: vec![v(u(32), 8), v(u(32), 8)],
            output: v(u(32), 8),
            definition: Named("llvm.x86.avx2.pmaxu.d")
        },
        "256_min_epi8" => Intrinsic {
            inputs: vec![v(i(8), 32), v(i(8), 32)],
            output: v(i(8), 32),
            definition: Named("llvm.x86.avx2.pmins.b")
        },
        "256_min_epu8" => Intrinsic {
            inputs: vec![v(u(8), 32), v(u(8), 32)],
            output: v(u(8), 32),
            definition: Named("llvm.x86.avx2.pminu.b")
        },
        "256_min_epi16" => Intrinsic {
            inputs: vec![v(i(16), 16), v(i(16), 16)],
            output: v(i(16), 16),
            definition: Named("llvm.x86.avx2.pmins.w")
        },
        "256_min_epu16" => Intrinsic {
            inputs: vec![v(u(16), 16), v(u(16), 16)],
            output: v(u(16), 16),
            definition: Named("llvm.x86.avx2.pminu.w")
        },
        "256_min_epi32" => Intrinsic {
            inputs: vec![v(i(32), 8), v(i(32), 8)],
            output: v(i(32), 8),
            definition: Named("llvm.x86.avx2.pmins.d")
        },
        "256_min_epu32" => Intrinsic {
            inputs: vec![v(u(32), 8), v(u(32), 8)],
            output: v(u(32), 8),
            definition: Named("llvm.x86.avx2.pminu.d")
        },
        "256_movemask_epi8" => Intrinsic {
            inputs: vec![v(i(8), 32)],
            output: i(32),
            definition: Named("llvm.x86.avx2.pmovmskb")
        },
        "256_mpsadbw_epu8" => Intrinsic {
            inputs: vec![v(u(8), 32), v(u(8), 32), i_(32, 8)],
            output: v(u(16), 16),
            definition: Named("llvm.x86.avx2.mpsadbw")
        },
        "256_mul_epi64" => Intrinsic {
            inputs: vec![v(i(32), 8), v(i(32), 8)],
            output: v(i(64), 4),
            definition: Named("llvm.x86.avx2.pmulq.dq")
        },
        "256_mul_epu64" => Intrinsic {
            inputs: vec![v(u(32), 8), v(u(32), 8)],
            output: v(u(64), 4),
            definition: Named("llvm.x86.avx2.pmulq.dq")
        },
        "256_mulhi_epi16" => Intrinsic {
            inputs: vec![v(i(16), 16), v(i(16), 16)],
            output: v(i(16), 16),
            definition: Named("llvm.x86.avx2.pmulhw.w")
        },
        "256_mulhi_epu16" => Intrinsic {
            inputs: vec![v(u(16), 16), v(u(16), 16)],
            output: v(u(16), 16),
            definition: Named("llvm.x86.avx2.pmulhw.w")
        },
        "256_mulhrs_epi16" => Intrinsic {
            inputs: vec![v(i(16), 16), v(i(16), 16)],
            output: v(i(16), 16),
            definition: Named("llvm.x86.avx2.pmul.hr.sw")
        },
        "256_packs_epi16" => Intrinsic {
            inputs: vec![v(i(16), 16), v(i(16), 16)],
            output: v(i(8), 32),
            definition: Named("llvm.x86.avx2.packsswb")
        },
        "256_packus_epi16" => Intrinsic {
            inputs: vec![v(i(16), 16), v(i(16), 16)],
            output: v(u(8), 32),
            definition: Named("llvm.x86.avx2.packuswb")
        },
        "256_packs_epi32" => Intrinsic {
            inputs: vec![v(i(32), 8), v(i(32), 8)],
            output: v(i(16), 16),
            definition: Named("llvm.x86.avx2.packssdw")
        },
        "256_packus_epi32" => Intrinsic {
            inputs: vec![v(i(32), 8), v(i(32), 8)],
            output: v(u(16), 16),
            definition: Named("llvm.x86.avx2.packusdw")
        },
        "256_permutevar8x32_epi32" => Intrinsic {
            inputs: vec![v(i(32), 8), v(i(32), 8)],
            output: v(i(32), 8),
            definition: Named("llvm.x86.avx2.permd")
        },
        "256_permutevar8x32_ps" => Intrinsic {
            inputs: vec![v(f(32), 8), v(i(32), 8)],
            output: v(f(32), 8),
            definition: Named("llvm.x86.avx2.permps")
        },
        "256_sad_epu8" => Intrinsic {
            inputs: vec![v(u(8), 32), v(u(8), 32)],
            output: v(u(8), 32),
            definition: Named("llvm.x86.avx2.psad.bw")
        },
        "256_shuffle_epi8" => Intrinsic {
            inputs: vec![v(i(8), 32), v(i(8), 32)],
            output: v(i(8), 32),
            definition: Named("llvm.x86.avx2.pshuf.b")
        },
        "256_sign_epi8" => Intrinsic {
            inputs: vec![v(i(8), 32), v(i(8), 32)],
            output: v(i(8), 32),
            definition: Named("llvm.x86.avx2.psign.b")
        },
        "256_sign_epi16" => Intrinsic {
            inputs: vec![v(i(16), 16), v(i(16), 16)],
            output: v(i(16), 16),
            definition: Named("llvm.x86.avx2.psign.w")
        },
        "256_sign_epi32" => Intrinsic {
            inputs: vec![v(i(32), 8), v(i(32), 8)],
            output: v(i(32), 8),
            definition: Named("llvm.x86.avx2.psign.d")
        },
        "256_subs_epi8" => Intrinsic {
            inputs: vec![v(i(8), 32), v(i(8), 32)],
            output: v(i(8), 32),
            definition: Named("llvm.x86.avx2.psubs.b")
        },
        "256_subs_epu8" => Intrinsic {
            inputs: vec![v(u(8), 32), v(u(8), 32)],
            output: v(u(8), 32),
            definition: Named("llvm.x86.avx2.psubus.b")
        },
        "256_subs_epi16" => Intrinsic {
            inputs: vec![v(i(16), 16), v(i(16), 16)],
            output: v(i(16), 16),
            definition: Named("llvm.x86.avx2.psubs.w")
        },
        "256_subs_epu16" => Intrinsic {
            inputs: vec![v(u(16), 16), v(u(16), 16)],
            output: v(u(16), 16),
            definition: Named("llvm.x86.avx2.psubus.w")
        },
        _ => return None,
    })
}
