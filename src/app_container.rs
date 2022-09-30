use crate::wide_string::WideString;
use std::ptr::null_mut;
use windows::{
    core::PCWSTR,
    Win32::{
        Foundation::PSID,
        Security::Isolation::{
            CreateAppContainerProfile, DeleteAppContainerProfile,
            DeriveAppContainerSidFromAppContainerName,
        },
    },
};

#[derive(Debug)]
pub struct AppContainerProfile {
    pub container_name: WideString,
    pub sid: PSID,
}

impl AppContainerProfile {
    pub fn new(container_name: &str) -> Result<Self, windows::core::Error> {
        let mut profile = Self {
            container_name: WideString::from(container_name),
            sid: PSID(null_mut()),
        };
        match unsafe {
            CreateAppContainerProfile(
                PCWSTR::from(&profile.container_name),
                PCWSTR::from(&profile.container_name),
                PCWSTR::from(&profile.container_name),
                None,
            )
        } {
            Ok(sid) => {
                profile.sid = sid;
                Ok(profile)
            }
            Err(error) => Err(error),
        }
    }

    pub fn delete(self) -> Result<(), windows::core::Error> {
        unsafe { DeleteAppContainerProfile(PCWSTR::from(&self.container_name)) }
    }

    pub fn derive_from_name(container_name: &str) -> Result<Self, windows::core::Error> {
        let mut profile = Self {
            container_name: WideString::from(container_name),
            sid: PSID(null_mut()),
        };
        match unsafe {
            DeriveAppContainerSidFromAppContainerName(PCWSTR::from(&profile.container_name))
        } {
            Ok(sid) => {
                profile.sid = sid;
                Ok(profile)
            }
            Err(error) => Err(error),
        }
    }
}
