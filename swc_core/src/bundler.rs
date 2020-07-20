use anyhow::{bail, Error};
use spack::{
    load::Load,
    resolve::{NodeResolver, Resolve},
    BundleKind,
};
use std::{
    panic::{catch_unwind, AssertUnwindSafe},
    sync::Arc,
};
use swc::{config::SourceMapsConfig, Compiler, TransformOutput};
