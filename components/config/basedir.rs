/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

//! Contains routines for retrieving default config directories.
//! For linux based platforms, it uses the XDG base directory spec but provides
//! similar abstractions for non-linux platforms.

#[cfg(all(target_os = "android", not(feature = "gonk")))]
use android_injected_glue;
#[cfg(any(target_os = "macos", target_os = "windows"))]
use std::env;
#[cfg(all(target_os = "android", not(feature = "gonk")))]
use std::ffi::CStr;
use std::path::PathBuf;
#[cfg(all(unix, not(target_os = "macos"), not(target_os = "ios"), not(target_os = "android")))]
use xdg;

#[cfg(all(unix, not(target_os = "macos"), not(target_os = "ios"), not(target_os = "android")))]
pub fn default_config_dir() -> Option<PathBuf> {
    let xdg_dirs = xdg::BaseDirectories::with_profile("servo", "default").unwrap();
    let config_dir = xdg_dirs.get_config_home();
    Some(config_dir)
}

#[cfg(all(target_os = "android", not(feature = "gonk")))]
#[allow(unsafe_code)]
pub fn default_config_dir() -> Option<PathBuf> {
    let dir =
        unsafe { CStr::from_ptr((*android_injected_glue::get_app().activity).externalDataPath) };
    Some(PathBuf::from(dir.to_str().unwrap()))
}

#[cfg(all(unix, not(target_os = "macos"), not(target_os = "ios"), not(target_os = "android")))]
pub fn default_data_dir() -> Option<PathBuf> {
    let xdg_dirs = xdg::BaseDirectories::with_profile("servo", "default").unwrap();
    let data_dir = xdg_dirs.get_data_home();
    Some(data_dir)
}

#[cfg(all(target_os = "android", not(feature = "gonk")))]
#[allow(unsafe_code)]
pub fn default_data_dir() -> Option<PathBuf> {
    let dir =
        unsafe { CStr::from_ptr((*android_injected_glue::get_app().activity).internalDataPath) };
    Some(PathBuf::from(dir.to_str().unwrap()))
}

#[cfg(all(unix, not(target_os = "macos"), not(target_os = "ios"), not(target_os = "android")))]
pub fn default_cache_dir() -> Option<PathBuf> {
    let xdg_dirs = xdg::BaseDirectories::with_profile("servo", "default").unwrap();
    let cache_dir = xdg_dirs.get_cache_home();
    Some(cache_dir)
}

#[cfg(all(target_os = "android", not(feature = "gonk")))]
#[allow(unsafe_code)]
pub fn default_cache_dir() -> Option<PathBuf> {
    // TODO: Use JNI to call context.getCacheDir().
    // There is no equivalent function in NDK/NativeActivity.
    let dir =
        unsafe { CStr::from_ptr((*android_injected_glue::get_app().activity).externalDataPath) };
    Some(PathBuf::from(dir.to_str().unwrap()))
}

#[cfg(target_os = "macos")]
pub fn default_config_dir() -> Option<PathBuf> {
    let mut config_dir = env::home_dir().unwrap();
    config_dir.push("Library");
    config_dir.push("Application Support");
    config_dir.push("Servo");
    Some(config_dir)
}

#[cfg(target_os = "windows")]
pub fn default_config_dir() -> Option<PathBuf> {
    let mut config_dir = match env::var_os("APPDATA") {
        Some(appdata_path) => PathBuf::from(appdata_path),
        None => {
            let mut dir = env::home_dir().unwrap();
            dir.push("Appdata");
            dir.push("Roaming");
            dir
        }
    };
    config_dir.push("Servo");
    Some(config_dir)
}

#[cfg(feature = "gonk")]
pub fn default_config_dir() -> Option<PathBuf> {
    Some("/data/local/servo".into())
}

#[cfg(feature = "gonk")]
pub fn default_data_dir() -> Option<PathBuf> {
    Some("/data/local/servo".into())
}

#[cfg(feature = "gonk")]
pub fn default_cache_dir() -> Option<PathBuf> {
    Some("/data/local/servo".into())
}
