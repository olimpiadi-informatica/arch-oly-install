use color_eyre::eyre::Result;

use crate::{
    args::Args,
    util::{ensure_paru, script},
};

pub fn compilers() -> Result<()> {
    script!(
        "86-compilers",
        "pacman -S --noconfirm base-devel pypy pypy3 python clang"
    );
    Ok(())
}

pub fn debuggers() -> Result<()> {
    script!("86-debuggers", "pacman -S --noconfirm gdb valgrind perf");
    Ok(())
}

pub fn browser(args: &Args) -> Result<()> {
    let Args { homepage, .. } = args;
    let set_homepage = if let Some(home) = homepage {
        format!(r#"lockPref("browser.startup.homepage", "{home}");"#)
    } else {
        String::new()
    };
    script!(
        "86-browser",
        r#"
pacman -S --noconfirm firefox
cat > /usr/lib/firefox/defaults/pref/autoconfig.js << EOF
pref("general.config.filename", "firefox.cfg");
pref("general.config.obscure_value", 0);
EOF
cat > /usr/lib/firefox/firefox.cfg << EOF
//
lockPref("extensions.pocket.enabled", false);
lockPref("browser.newtabpage.activity-stream.feeds.section.topstories", false);
lockPref("trailhead.firstrun.branches", "nofirstrun-empty");
lockPref("browser.aboutwelcome.enabled", false);
lockPref("browser.startup.homepage_override.mstone", "ignore");
lockPref("datareporting.policy.dataSubmissionPolicyBypassNotification", true);
{set_homepage}
EOF
"#
    );

    Ok(())
}

pub fn editors(args: &Args) -> Result<()> {
    let Args {
        contestant_account, ..
    } = args;
    ensure_paru()?;
    script!(
        "86-editors",
        "pacman -S --noconfirm emacs geany gedit gvim neovim kate kdevelop nano pycharm-community-edition"
    );
    script!(
        "86-vscode",
        r#"
pacman -S --noconfirm sqlite3
sudo -u paruuser paru -S --noconfirm visual-studio-code-bin
sudo -u {contestant_account} code --install-extension ms-python.python
sudo -u {contestant_account} code --install-extension ms-vscode.cpptools
sudo -u {contestant_account} code --install-extension vscodevim.vim
sudo -u {contestant_account} mkdir -p ~{contestant_account}/.config/Code/User/globalStorage/
sudo -u {contestant_account} sqlite3 ~{contestant_account}/.config/Code/User/globalStorage/state.vscdb << EOF
CREATE TABLE IF NOT EXISTS ItemTable (key TEXT UNIQUE ON CONFLICT REPLACE, value BLOB);
INSERT INTO ItemTable VALUES('extensionsIdentifiers/disabled', '[{{"id":"vscodevim.vim","uuid":"d96e79c6-8b25-4be3-8545-0e0ecefcae03"}}]');
EOF
"#
    );
    script!(
        "86-eclipse",
        "sudo -u paruuser paru -S --noconfirm eclipse-cpp-bin"
    );
    Ok(())
}
