use std::error::Error;

pub(crate) fn self_update() -> Result<(), Box<dyn Error>> {
    let status = self_update::backends::github::Update::configure()
        .repo_owner("1toldyou")
        .repo_name("deploy_this")
        .bin_name("dplyt")
        .show_download_progress(true)
        .current_version(env!("CARGO_PKG_VERSION"))
        .build()?
        .update()?;
    println!("Update status: `{}`!", status.version());
    Ok(())
}