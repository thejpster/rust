//! Targets 32-bit Big-Endian PowerPC processors with no FPU

use rustc_abi::Endian;

use crate::spec::{
    Cc, LinkerFlavor, FloatAbi, Lld, PanicStrategy, RelocModel, Target, TargetMetadata, TargetOptions
};

pub(crate) fn target() -> Target {
    Target {
        data_layout: "E-m:e-p:32:32-Fn32-i64:64-n32".into(),
        llvm_target: "powerpc-unknown-none-eabi".into(),
        metadata: TargetMetadata {
            description: Some("Bare 32-bit PowerPC".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(false),
        },
        pointer_width: 32,
        arch: "powerpc".into(),
        options: TargetOptions {
            abi: "eabi".into(),
            emit_debug_gdb_scripts: false,
            endian: Endian::Big,
            features: "-hard-float".into(),
            linker: Some("rust-lld".into()),
            linker_flavor: LinkerFlavor::Gnu(Cc::No, Lld::Yes),
            llvm_floatabi: Some(FloatAbi::Soft),
            max_atomic_width: Some(32),
            panic_strategy: PanicStrategy::Abort,
            relocation_model: RelocModel::Static,
            ..Default::default()
        },
    }
}
