fn main() -> winrt::Result<()> {
    use bindings::windows::system::diagnostics::*;
 
    for process in ProcessDiagnosticInfo::get_for_processes()? {
        println!(
            "id: {:5} packaged: {:5} name: {}",
            process.process_id()?,
            process.is_packaged()?,
            process.executable_file_name()?
        );
    }
 
    Ok(())
}