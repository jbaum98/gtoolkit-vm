use crate::bundlers::Bundler;
use crate::BuildOptions;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

pub struct MacBundler {}

impl MacBundler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create_icns(&self, configuration: &BuildOptions) -> Option<PathBuf> {
        if let Some(icons) = configuration.icons.as_ref() {
            for icon in icons {
                let icon_path = Path::new(icon);
                if icon_path.exists() {
                    if let Some(extension) = icon_path.extension() {
                        if extension == "icns" {
                            return Some(icon_path.to_path_buf());
                        }
                    }
                }
            }
        }
        None
    }
}

impl Bundler for MacBundler {
    fn bundle(&self, configuration: &BuildOptions) {
        let bundle_location = self.bundle_location(configuration);
        let app_name = self.app_name(configuration);

        let app_dir = bundle_location.join(format!("{}.app", &app_name));
        let contents_dir = app_dir.join("Contents");
        let resources_dir = contents_dir.join("Resources");
        let macos_dir = contents_dir.join("MacOS");
        let plugins_dir = macos_dir.join("Plugins");

        if app_dir.exists() {
            fs::remove_dir_all(&app_dir).unwrap();
        }
        fs::create_dir_all(&app_dir).unwrap();
        fs::create_dir(&contents_dir).unwrap();
        fs::create_dir(&resources_dir).unwrap();
        fs::create_dir(&macos_dir).unwrap();
        fs::create_dir(&plugins_dir).unwrap();

        let target_executable_path =
            macos_dir.join(Path::new(&self.executable_name(configuration)));

        fs::copy(
            self.compiled_executable_path(configuration),
            target_executable_path,
        )
        .unwrap();

        fs_extra::copy_items(
            &self.compiled_libraries(configuration),
            plugins_dir,
            &fs_extra::dir::CopyOptions::new(),
        )
        .unwrap();

        let icon = if let Some(icon) = self.create_icns(configuration) {
            let resource_icon_name = resources_dir
                .join(self.app_name(configuration))
                .with_extension("icns");
            fs::copy(icon, resource_icon_name.clone()).unwrap();
            Some(resource_icon_name.clone())
        } else {
            None
        };

        let info_plist_template = mustache::compile_str(INFO_PLIST).unwrap();
        let info = Info {
            bundle_name: self.app_name(configuration),
            bundle_display_name: self.app_name(configuration),
            executable_name: self.executable_name(configuration),
            bundle_identifier: self.bundle_identifier(configuration),
            bundle_version: self.bundle_version(configuration),
            bundle_icon: icon.as_ref().map_or("".to_string(), |icon| {
                icon.file_name().unwrap().to_str().unwrap().to_string()
            }),
        };

        let mut file = File::create(contents_dir.join(Path::new("Info.plist"))).unwrap();
        info_plist_template.render(&mut file, &info).unwrap();
    }
}

#[derive(Serialize)]
struct Info {
    bundle_name: String,
    bundle_display_name: String,
    executable_name: String,
    bundle_identifier: String,
    bundle_version: String,
    bundle_icon: String,
}

const INFO_PLIST: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple Computer//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleDevelopmentRegion</key>
  <string>English</string>
  <key>CFBundleDisplayName</key>
  <string>{{bundle_display_name}}</string>
  <key>CFBundleExecutable</key>
  <string>{{executable_name}}</string>
  <key>CFBundleIdentifier</key>
  <string>{{bundle_identifier}}</string>
  <key>CFBundleIconFile</key>
  <string>{{bundle_icon}}</string>
  <key>CFBundleInfoDictionaryVersion</key>
  <string>6.0</string>
  <key>CFBundleName</key>
  <string>{{bundle_name}}</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleShortVersionString</key>
  <string>{{bundle_version}}</string>
  <key>CFBundleVersion</key>
  <string>{{bundle_version}}</string>
  <key>CSResourcesFileMapped</key>
  <true/>
  <key>LSRequiresCarbon</key>
  <true/>
  <key>NSHighResolutionCapable</key>
  <true/>
  <key>LSEnvironment</key>
	<dict>
	<key>WANTS_INTERACTIVE_SESSION</key>
	<string>true</string>
	</dict>
</dict>
</plist>
"#;