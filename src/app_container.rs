use crate::wide_string::WideString;
use std::any::type_name;
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

pub struct AppContainerProfile {
    pub container_name: WideString,
    pub sid: PSID,
}

impl AppContainerProfile {
    pub fn new(
        container_name: &WideString,
        display_name: &WideString,
        description: &WideString,
    ) -> Result<Self, windows::core::Error> {
        log::debug!(
            "{}: AppContainer information: name: {}, display name: {}, description: {}",
            type_name::<Self>(),
            container_name,
            display_name,
            description
        );
        match unsafe {
            CreateAppContainerProfile(
                PCWSTR::from(container_name),
                PCWSTR::from(display_name),
                PCWSTR::from(description),
                &[],
            )
        } {
            Ok(sid) => Ok(Self {
                container_name: container_name.to_owned(),
                sid,
            }),
            Err(error) => Err(error),
        }
    }

    pub fn delete(self) -> Result<(), windows::core::Error> {
        unsafe { DeleteAppContainerProfile(PCWSTR::from(&self.container_name)) }
    }

    pub fn derive_from_name(container_name: &WideString) -> Result<Self, windows::core::Error> {
        match unsafe { DeriveAppContainerSidFromAppContainerName(PCWSTR::from(container_name)) } {
            Ok(sid) => Ok(Self {
                container_name: container_name.to_owned(),
                sid,
            }),
            Err(error) => Err(error),
        }
    }
}
