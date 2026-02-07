slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = MainWindow::new()?;
    app.run()?;
    Ok(())
}
