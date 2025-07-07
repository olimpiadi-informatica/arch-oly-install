use std::fs::create_dir_all;

use color_eyre::eyre::Result;

use crate::{
    args::Args,
    util::{ensure_paru, iso_root, script},
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
    let Args {
        homepage,
        contestant_account,
        ca_certificate,
        ..
    } = args;
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
sudo -u {contestant_account} cp /usr/share/applications/firefox.desktop ~{contestant_account}/Desktop
"#
    );

    if !ca_certificate.is_empty() {
        let file_dir = iso_root().join("install/ca-certificates");
        create_dir_all(&file_dir)?;

        for f in ca_certificate.iter() {
            std::fs::copy(f, file_dir.join(f.file_name().unwrap()))?;
        }

        script!(
            "87-ca-certificates",
            r#"
for cert in /install/ca-certificates/*
do
    trust anchor $cert
done
update-ca-trust
"#
        );
    }

    Ok(())
}

pub fn editors(args: &Args) -> Result<()> {
    let Args {
        pycharm,
        codeblocks,
        contestant_account,
        ..
    } = args;
    ensure_paru()?;
    script!(
        "86-editors",
        "pacman -S --noconfirm emacs geany gedit gvim neovim kate kdevelop nano"
    );

    if *pycharm {
        script!(
            "86-pycharm",
            r#"
pacman -S --noconfirm pycharm-community-edition
"#
        );
    }

    if *codeblocks {
        script!("86-codeblocks", "pacman -S --noconfirm codeblocks xterm");
    }

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
sudo -u {contestant_account} cp /usr/share/applications/code.desktop ~{contestant_account}/Desktop
"#
    );
    script!(
        "86-eclipse",
        "sudo -u paruuser paru -S --noconfirm eclipse-cpp-bin"
    );
    Ok(())
}
