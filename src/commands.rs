use std::process::Command;

pub fn open_editor(editor: &str, folder: &str) -> Result<(), Box<dyn std::error:: Error>> {
    Command::new("sh")
        .arg("-c")
        .arg(format!("{} {}", editor, folder))
        .spawn()?
        .wait()?;

    Ok(())
}

pub fn sync(root: &str) -> Result<(), Box<dyn std::error::Error>> {
    let current_dir = std::env::current_dir()?;
    let formatted_date = chrono::Local::now().format("%Y-%m-%d");
    let commit_message = format!("Sync: {}", formatted_date);

    Command::new("sh")
        .arg("-c")
        .arg(format!("cd {}; git add -A; git commit -m \"{}\"; git pull; git push; ls; cd {}", root, commit_message, current_dir.display()))
        .spawn()?
        .wait()?;

    Ok(())
}
