use crate::{
    app_container::AppContainerProfile,
    helper::{get_command_line, get_last_error},
    wide_string::WideString,
};
use std::{
    any::type_name,
    mem,
    os::raw::c_void,
    ptr::{null, null_mut},
};
use windows::{
    core::{PCWSTR, PWSTR},
    Win32::{
        Security::SECURITY_CAPABILITIES,
        System::Threading::{
            CreateProcessW, InitializeProcThreadAttributeList, UpdateProcThreadAttribute,
            EXTENDED_STARTUPINFO_PRESENT, LPPROC_THREAD_ATTRIBUTE_LIST, PROCESS_INFORMATION,
            PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES, STARTUPINFOEXW,
        },
    },
};

#[derive(Debug)]
pub struct IsolatedProcess {
    // TODO
    startup_info: STARTUPINFOEXW,
    process_info: PROCESS_INFORMATION,
    security_capabilities: SECURITY_CAPABILITIES,
    attribute_list_buffer: Vec<u8>,
}

impl IsolatedProcess {
    pub fn run(
        executable_path: &String,
        arguments: &[String],
        app_container_profile: AppContainerProfile,
    ) -> Result<Self, windows::core::Error> {
        let mut attribute_list_size = Self::get_attribute_list_size();
        let mut process = IsolatedProcess {
            startup_info: STARTUPINFOEXW::default(),
            process_info: PROCESS_INFORMATION::default(),
            security_capabilities: SECURITY_CAPABILITIES::default(),
            attribute_list_buffer: vec![0_u8; attribute_list_size],
        };

        process.startup_info.StartupInfo.cb = mem::size_of::<STARTUPINFOEXW>() as u32;
        process.security_capabilities.AppContainerSid = app_container_profile.sid;
        process.initialise_attribute_list(&mut attribute_list_size)?;
        process.add_security_capabilities_to_attributes()?;

        let exe_path = WideString::from(executable_path);
        let mut command_line = get_command_line(executable_path, arguments);
        log::debug!("{}: executable path: `{}`", type_name::<Self>(), exe_path);
        log::debug!("{}: command line: `{}`", type_name::<Self>(), command_line);

        // TODO: Launch the process in a job
        process.launch(PCWSTR::from(&exe_path), PWSTR::from(&mut command_line))?;

        Ok(process)
    }

    fn get_attribute_list_size() -> usize {
        let mut attribute_list_size = 0;
        unsafe {
            InitializeProcThreadAttributeList(
                LPPROC_THREAD_ATTRIBUTE_LIST(null_mut()),
                1,
                0,
                &mut attribute_list_size,
            );
        }
        attribute_list_size
    }

    fn initialise_attribute_list(
        &mut self,
        attribute_list_size: &mut usize,
    ) -> Result<(), windows::core::Error> {
        log::debug!(
            "{}: attribute list size is: {:?}",
            type_name::<Self>(),
            attribute_list_size
        );
        self.startup_info.lpAttributeList =
            LPPROC_THREAD_ATTRIBUTE_LIST(self.attribute_list_buffer.as_mut_ptr() as *mut c_void);
        let success = unsafe {
            InitializeProcThreadAttributeList(
                self.startup_info.lpAttributeList,
                1,
                0,
                attribute_list_size,
            )
        };
        if success.as_bool() {
            log::debug!(
                "{}: attribute list: {:?}",
                type_name::<Self>(),
                self.attribute_list_buffer
            );
            Ok(())
        } else {
            Err(get_last_error())
        }
    }

    fn add_security_capabilities_to_attributes(&mut self) -> Result<(), windows::core::Error> {
        let success = unsafe {
            UpdateProcThreadAttribute(
                self.startup_info.lpAttributeList,
                0,
                PROC_THREAD_ATTRIBUTE_SECURITY_CAPABILITIES
                    .try_into()
                    .unwrap(),
                &self.security_capabilities as *const _ as *const c_void,
                mem::size_of::<SECURITY_CAPABILITIES>(),
                null_mut(),
                null(),
            )
        };
        if success.as_bool() {
            Ok(())
        } else {
            Err(get_last_error())
        }
    }

    fn launch(
        &mut self,
        application_name: PCWSTR,
        command_line: PWSTR,
    ) -> Result<(), windows::core::Error> {
        let success = unsafe {
            CreateProcessW(
                application_name,
                command_line,
                null(),
                null(),
                false,
                EXTENDED_STARTUPINFO_PRESENT,
                null(),
                PCWSTR::null(),
                &self.startup_info.StartupInfo,
                &mut self.process_info,
            )
        };
        if success.as_bool() {
            Ok(())
        } else {
            Err(get_last_error())
        }
    }
}
