#[macro_use]
extern crate failure;

#[macro_use]
extern crate crate_compile_test;

use crate_compile_test::prelude::*;
use std::env::{current_dir, current_exe};
use std::path::Path;

mod steps;
use steps::{
    assembly::StepFactory as AssemblyTestStepFactory, ir::StepFactory as IRTestStepFactory,
    verification::StepFactory as VerificationTestStepFactory,
};

fn ptx_assembly_tests(tester: &mut TestRunner) {
    tester.add("Debug PTX Assembly", || {
        let mut config = create_config(Mode::BuildSuccess, Profile::Debug);

        config
            .additional_steps
            .push(Box::new(AssemblyTestStepFactory::new()));

        config
    });

    tester.add("Release PTX Assembly", || {
        let mut config = create_config(Mode::BuildSuccess, Profile::Release);

        config
            .additional_steps
            .push(Box::new(AssemblyTestStepFactory::new()));

        config
    });

    tester.add("Debug PTX Assembly Verification", || {
        let mut config = create_config(Mode::BuildSuccess, Profile::Debug);

        config.crates_filter = Box::new(|path| {
            VerificationTestStepFactory::is_runnable()
                && path != Path::new("examples/undefined-ref")
        });

        config
            .additional_steps
            .push(Box::new(VerificationTestStepFactory::new()));

        config
    });

    tester.add("Release PTX Assembly Verification", || {
        let mut config = create_config(Mode::BuildSuccess, Profile::Release);

        config.crates_filter = Box::new(|path| {
            VerificationTestStepFactory::is_runnable()
                && path != Path::new("examples/undefined-ref")
        });

        config
            .additional_steps
            .push(Box::new(VerificationTestStepFactory::new()));

        config
    });
}

fn ir_tests(tester: &mut TestRunner) {
    tester.add("Debug IR", || {
        let mut config = create_config(Mode::BuildSuccess, Profile::Debug);

        config
            .additional_steps
            .push(Box::new(IRTestStepFactory::new()));

        config
    });

    tester.add("Release IR", || {
        let mut config = create_config(Mode::BuildSuccess, Profile::Release);

        config
            .additional_steps
            .push(Box::new(IRTestStepFactory::new()));

        config
    });
}

fn failure_tests(tester: &mut TestRunner) {
    tester.add("Debug linking fail", || {
        create_config(Mode::BuildFail, Profile::Debug)
    });

    tester.add("Release linking fail", || {
        create_config(Mode::BuildFail, Profile::Release)
    });
}

bootstrap_compilation_tests![ptx_assembly_tests, ir_tests, failure_tests];

fn create_config(mode: Mode, profile: Profile) -> Config {
    let mut config = Config::new(mode, "examples");

    config.cargo_command = "xargo".into();
    config.profile = profile;
    config.target = Some("nvptx64-nvidia-cuda".into());

    match config.mode {
        Mode::BuildFail => {
            config.crates_filter = Box::new(|path| path == Path::new("examples/undefined-ref"));
        }

        Mode::BuildSuccess => {
            config.crates_filter = Box::new(|path| path != Path::new("examples/undefined-ref"));
        }

        _ => unimplemented!(),
    };

    config.add_cargo_env(
        "CARGO_TARGET_NVPTX64_NVIDIA_CUDA_LINKER",
        &current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("ptx-linker")
            .to_string_lossy(),
    );

    config.add_cargo_env(
        "RUST_TARGET_PATH",
        &current_dir().unwrap().join("targets").to_string_lossy(),
    );

    config
}
