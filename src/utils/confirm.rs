use console::Term;
use dialoguer::{theme::ColorfulTheme, Confirm};

pub fn confirm_default_yes(prompt: &str) -> anyhow::Result<bool> {
    let confirmation = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(true)
        .show_default(true)
        .wait_for_newline(true)
        .interact_on(&Term::stdout())?;
    Ok(confirmation)
}

pub fn confirm_default_no(prompt: &str) -> anyhow::Result<bool> {
    let confirmation = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(prompt)
        .default(false)
        .show_default(true)
        .wait_for_newline(true)
        .interact_on(&Term::stdout())?;
    Ok(confirmation)
}
