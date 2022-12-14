use crate::wide_string::WideString;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::borrow::Cow;
use windows::{
    core::{PCWSTR, PWSTR},
    Win32::Foundation::GetLastError,
};

const SUFFIX_LENGTH: usize = 13;

impl From<&WideString> for PCWSTR {
    fn from(wide_string: &WideString) -> Self {
        PCWSTR(wide_string.as_ptr())
    }
}

impl From<&mut WideString> for PWSTR {
    fn from(wide_string: &mut WideString) -> Self {
        PWSTR(wide_string.as_mut_ptr())
    }
}

pub fn get_app_container_suffix() -> WideString {
    WideString::from_iter(
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(SUFFIX_LENGTH)
            .map(char::from),
    )
}

pub fn get_last_error() -> windows::core::Error {
    let error = unsafe { GetLastError() }.to_hresult();
    windows::core::Error::new(error, error.message())
}

pub fn get_command_line(executable_path: &String, arguments: &[String]) -> WideString {
    let mut command_line = String::from(&quote(executable_path) as &str);
    for argument in arguments {
        command_line += " ";
        command_line += &quote(&argument);
    }
    WideString::from(&command_line)
}

pub fn quote(string: &str) -> Cow<str> {
    let mut needs_quoting = false;

    for ch in string.chars() {
        let quote = match ch {
            ' ' | '"' => true,
            _ => false,
        };
        if quote {
            needs_quoting = true;
            break;
        }
    }

    if !needs_quoting {
        return Cow::from(string);
    }

    let mut output = String::with_capacity(string.len() + 2);
    output.push('"');

    for ch in string.chars() {
        match ch {
            '"' => output += "\\\"",
            _ => output.push(ch),
        }
    }

    output.push('"');
    output.into()
}
