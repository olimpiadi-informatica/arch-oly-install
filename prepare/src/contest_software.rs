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
cd ~{contestant_account}
sudo -u {contestant_account} bash << AS_CONTESTANT
set -xe
mkdir -p .config/JetBrains/PyCharmCE2025.1/options/
cat > .config/JetBrains/PyCharmCE2025.1/options/other.xml << EOF
<application>
  <component name="LangManager">
    <option name="languageName" value="Python" />
  </component>
  <component name="NotRoamableUiSettings">
    <option name="presentationModeIdeScale" value="1.75" />
  </component>
  <component name="PropertyService"><![CDATA[{{
  "keyToString": {{
    "PyCharm.InitialConfiguration": "true",
    "PyCharm.InitialConfiguration.V2": "true",
    "PyCharm.InitialConfiguration.V3": "true",
    "PyCharm.InitialConfiguration.V4": "true",
    "PyCharm.InitialConfiguration.V5": "true",
    "PyCharm.InitialConfiguration.V6": "true",
    "PyCharm.InitialConfiguration.V7": "true",
    "PyCharm.InitialConfiguration.V8": "true",
    "ask.about.ctrl.y.shortcut.v2": "true",
    "experimental.ui.on.first.startup": "true",
    "experimental.ui.used.version": "251.26094.141",
    "experimentalFeature.terminal.shell.command.handling": "false",
    "fileTypeChangedCounter": "2",
    "fontSizeToResetConsole": "13.0",
    "fontSizeToResetEditor": "13.0",
    "ift.hide.welcome.screen.promo": "true",
    "ignore.ide.script.launcher.used": "true",
    "input.method.disabler.muted": "true",
    "previousColorScheme": "_@user_Dark",
    "registry.to.advanced.settings.migration.build": "PC-251.26094.141",
    "terminal.gen.one.option.visible": "false",
    "whats.new.last.shown.version": "9999-1.2-26094-6f065183c835868e278363490753d1b84b4aeb30"
  }},
  "keyToStringList": {{
    "fileTypeDetectors": [
      "com.intellij.ide.scratch.ScratchFileServiceImpl\\\$Detector",
      "org.jetbrains.plugins.textmate.TextMateFileType\\\$TextMateFileDetector"
    ]
  }}
}}]]></component>
</application>
EOF

cat > .config/JetBrains/PyCharmCE2025.1/options/actionSummary.xml << EOF
<application>
  <component name="ActionsLocalSummary">
    <e n="com.intellij.ide.startup.importSettings.chooser.productChooser.SkipImportAction">
      <i c="1" l="1752005559715" />
    </e>
  </component>
</application>
EOF

JAVA_PREF_DIR=.java/.userPrefs/jetbrains/_\!\(\!\!cg\"p\!\(\}}\!\}}@\"j\!\(k\!\|w\"w\!\'8\!b\!\"p\!\'\:\!e@\=\=/
mkdir -p \$JAVA_PREF_DIR
cat > \$JAVA_PREF_DIR/prefs.xml << EOF
<!DOCTYPE map SYSTEM "http://java.sun.com/dtd/preferences.dtd">
<map MAP_XML_VERSION="1.0">
  <entry key="euacommunity_accepted_version" value="1.0"/>
</map>
EOF
mkdir -p .local/share/JetBrains/consentOptions/
echo -n "rsch.send.usage.stat:1.1:0:1752009309526" > .local/share/JetBrains/consentOptions/accepted
AS_CONTESTANT
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
sudo -u {contestant_account} bash << EOF
set -xe
code --install-extension ms-python.python
code --install-extension ms-vscode.cpptools
code --install-extension vscodevim.vim
mkdir -p ~{contestant_account}/.config/Code/User/globalStorage/
sqlite3 ~{contestant_account}/.config/Code/User/globalStorage/state.vscdb << SQLITE
CREATE TABLE IF NOT EXISTS ItemTable (key TEXT UNIQUE ON CONFLICT REPLACE, value BLOB);
INSERT INTO ItemTable VALUES('extensionsIdentifiers/disabled', '[{{"id":"vscodevim.vim","uuid":"d96e79c6-8b25-4be3-8545-0e0ecefcae03"}}]');
SQLITE
echo '{{"workbench.startupEditor": "none"}}' > ~{contestant_account}/.config/Code/User/settings.json
cp /usr/share/applications/code.desktop ~{contestant_account}/Desktop
EOF
"#
    );
    script!(
        "86-eclipse",
        "sudo -u paruuser paru -S --noconfirm eclipse-cpp-bin"
    );
    Ok(())
}
