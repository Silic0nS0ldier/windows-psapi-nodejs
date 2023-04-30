use neon::prelude::*;

fn pid_path(mut cx: FunctionContext) -> JsResult<JsString> {
    let pid = cx.argument::<JsNumber>(0)?.value(&mut cx);

    let max_path = windows::Win32::Foundation::MAX_PATH as usize;
    let mut filename: Vec<u8> = Vec::new();
    filename.resize(max_path, 0);
    let pid_path_len: u32;

    unsafe {
        // Acquire process handle
        let process_handle = windows::Win32::System::Threading::OpenProcess(
            windows::Win32::System::Threading::PROCESS_QUERY_INFORMATION|windows::Win32::System::Threading::PROCESS_VM_READ,
            false,
            pid as u32,
        ).expect("nope");

        pid_path_len = windows::Win32::System::ProcessStatus::GetModuleFileNameExA(process_handle, None, &mut filename);

        windows::Win32::Foundation::CloseHandle(process_handle);
    }

    if pid_path_len == 0 {
        return cx.throw_error("An issue occurred.");
    }

    filename.truncate(pid_path_len as usize);
    
    // convert filename to string
    return match std::str::from_utf8(&filename) {
        Ok(pid_path) => Ok(cx.string(pid_path)),
        Err(_) => cx.throw_error("Can not represent in Unicode."),
    };
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("pidPath", pid_path)?;
    Ok(())
}
