use crate::build_support::builder::Name;
use crate::build_support::Builder;

use std::fmt;
use std::fmt::{Debug, Formatter};
use std::path::PathBuf;

#[derive(Default, Clone)]
pub struct MacBuilder {}

impl MacBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

impl Debug for MacBuilder {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.print_directories(f)
    }
}

impl Builder for MacBuilder {
    fn vm_binary(&self) -> PathBuf {
        self.compiled_libraries_directory()
            .join("libPharoVMCore.dylib")
    }

    fn compiled_libraries_directory(&self) -> PathBuf {
        self.output_directory()
            .join("build")
            .join("build")
            .join("vm")
    }

    fn compile_sources(&self) {
        assert!(
            self.vm_sources_directory().exists(),
            "Source directory must exist: {:?}",
            self.vm_sources_directory().display()
        );
        assert!(
            self.output_directory().exists(),
            "Output directory must exist: {:?}",
            self.output_directory().display()
        );

        let mut config = cmake::Config::new(self.vm_sources_directory());
        config
            .define("COMPILE_EXECUTABLE", "OFF")
            .define("FEATURE_LIB_GIT2", "OFF")
            .define("FEATURE_LIB_SDL2", "OFF");

        if let Some(vm_maker) = self.vm_maker() {
            config.define("GENERATE_PHARO_VM", vm_maker);
        }

        config.build();
    }

    fn platform_include_directory(&self) -> PathBuf {
        self.squeak_include_directory().join("osx")
    }

    fn link_libraries(&self) {
        println!("cargo:rustc-link-lib=PharoVMCore");
        println!("cargo:rustc-link-lib=framework=AppKit");
        println!("cargo:rustc-link-lib=framework=CoreGraphics");

        println!(
            "cargo:rustc-link-search={}",
            self.compiled_libraries_directory().display()
        );
    }

    fn shared_libraries_to_export(&self) -> Vec<(PathBuf, Option<String>)> {
        assert!(
            self.compiled_libraries_directory().exists(),
            "Must exist: {:?}",
            self.compiled_libraries_directory().display()
        );

        let libs: Vec<(Name, Option<&str>)> = vec![
            // core
            (Name::Exact("libPharoVMCore.dylib"), None),
            // plugins
            (Name::Exact("libB2DPlugin.dylib"), None),
            (Name::Exact("libBitBltPlugin.dylib"), None),
            (Name::Exact("libDSAPrims.dylib"), None),
            (Name::Exact("libFileAttributesPlugin.dylib"), None),
            (Name::Exact("libFilePlugin.dylib"), None),
            (Name::Exact("libJPEGReaderPlugin.dylib"), None),
            (Name::Exact("libJPEGReadWriter2Plugin.dylib"), None),
            (Name::Exact("libLargeIntegers.dylib"), None),
            (Name::Exact("libLocalePlugin.dylib"), None),
            (Name::Exact("libMiscPrimitivePlugin.dylib"), None),
            (Name::Exact("libSocketPlugin.dylib"), None),
            (Name::Exact("libSqueakSSL.dylib"), None),
            (Name::Exact("libSurfacePlugin.dylib"), None),
            (Name::Exact("libUnixOSProcessPlugin.dylib"), None),
            (Name::Exact("libUUIDPlugin.dylib"), None),
            // third party
            (Name::Exact("libcairo.2.dylib"), None),
            (
                Name::Optional("libfreetype.6.16.0.dylib"),
                Some("libfreetype.dylib"),
            ),
            (Name::Exact("libpixman-1.dylib"), None),
            (
                Name::Any(vec!["libpng12.dylib", "libpng16.dylib"]),
                Some("libpng.dylib"),
            ),
            // testing
            (Name::Exact("libTestLibrary.dylib"), None),
        ];
        libs.iter()
            .map(|(library, rename)| {
                (
                    library.find_file(&self.compiled_libraries_directory()),
                    rename.map(|name| name.to_string()),
                )
            })
            .filter(|(library, _)| library.is_some())
            .map(|(library, rename)| (library.unwrap(), rename))
            .collect()
    }
}
