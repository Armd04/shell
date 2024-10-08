use deno_task_shell::{ExecuteResult, ShellCommand, ShellCommandContext};
use futures::future::LocalBoxFuture;
use uu_uname::{options, UNameOutput};
pub struct UnameCommand;

fn display(uname: &UNameOutput) -> String {
    let mut output = String::new();
    for name in [
        uname.kernel_name.as_ref(),
        uname.nodename.as_ref(),
        uname.kernel_release.as_ref(),
        uname.kernel_version.as_ref(),
        uname.machine.as_ref(),
        uname.os.as_ref(),
        uname.processor.as_ref(),
        uname.hardware_platform.as_ref(),
    ]
    .into_iter()
    .flatten()
    {
        output.push_str(name);
        output.push(' ');
    }
    output
}

impl ShellCommand for UnameCommand {
    fn execute(&self, mut context: ShellCommandContext) -> LocalBoxFuture<'static, ExecuteResult> {
        Box::pin(async move {
            match execute_uname(&mut context) {
                Ok(_) => ExecuteResult::from_exit_code(0),
                Err(e) => {
                    context.stderr.write_line(&e).ok();
                    ExecuteResult::from_exit_code(1)
                }
            }
        })
    }
}

fn execute_uname(context: &mut ShellCommandContext) -> Result<(), String> {
    let matches = uu_uname::uu_app()
        .override_usage("uname [OPTION]...")
        .no_binary_name(true)
        .try_get_matches_from(&context.args)
        .map_err(|e| e.to_string())?;

    let options = uu_uname::Options {
        all: matches.get_flag(options::ALL),
        kernel_name: matches.get_flag(options::KERNEL_NAME),
        nodename: matches.get_flag(options::NODENAME),
        kernel_release: matches.get_flag(options::KERNEL_RELEASE),
        kernel_version: matches.get_flag(options::KERNEL_VERSION),
        machine: matches.get_flag(options::MACHINE),
        processor: matches.get_flag(options::PROCESSOR),
        hardware_platform: matches.get_flag(options::HARDWARE_PLATFORM),
        os: matches.get_flag(options::OS),
    };

    let uname = UNameOutput::new(&options).unwrap();
    context
        .stdout
        .write_line(display(&uname).trim_end())
        .map_err(|e| e.to_string())?;

    Ok(())
}
