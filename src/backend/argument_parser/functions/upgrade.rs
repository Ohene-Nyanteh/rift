use axoupdater::AxoUpdater;

pub fn upgrade() -> Result<(), Box<dyn std::error::Error>> {
    let mut updater = AxoUpdater::new_for("rift");
    updater.load_receipt()?;
    match updater.run_sync()? {
        Some(result) => {
            println!("Updated to {}", result.new_version);
            Ok(())
        },
        None => {
            println!("Rift is already up to date");
            Ok(())
        },
    }
}
