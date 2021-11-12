use anyhow::{bail, Result};
use cmd_lib_core::run_fun;
use which::which;

/// 使用指定命令 版本与自带补全 生成可更新的zsh补全缓存
///
/// 参考：https://github.com/ohmyzsh/ohmyzsh/blob/master/plugins/cargo/cargo.plugin.zsh
pub fn gen_zsh_cmd_completion(name: &str, ver_opt: &str, comp_opt: &str) -> Result<String> {
    if let Err(e) = which(name) {
        bail!("not found bin {}: {}", name, e)
    }
    // check cmd commands
    if let Err(e) = run_fun(format!("{} {}", name, ver_opt)) {
        bail!("failed to run cmd `{} {}`: {}", name, ver_opt, e)
    }
    if let Err(e) = run_fun(format!("{} {}", name, comp_opt)) {
        bail!("failed to run cmd `{} {}`: {}", name, comp_opt, e)
    }

    Ok(format!(
        r#"if (( $+commands[{name}] )); then
# remove old generated completion file
command rm -f "${{0:A:h}}/_{name}"

ver="$({name} {version_opt} 2>/dev/null)"
ver_file="$ZSH_CACHE_DIR/{name}_version"
comp_file="$ZSH_CACHE_DIR/completions/_{name}"

mkdir -p "${{comp_file:h}}"
(( ${{fpath[(Ie)${{comp_file:h}}]}} )) || fpath=("${{comp_file:h}}" $fpath)

if [[ ! -f "$comp_file" || ! -f "$ver_file" || "$ver" != "$(< "$ver_file")" ]]; then
    {name} {completion_opt} >| "$comp_file"
    echo "$ver" >| "$ver_file"
fi

declare -A _comps
autoload -Uz _{name}
_comps[{name}]=_{name}

unset ver ver_file comp_file
fi"#,
        name = &name,
        version_opt = &ver_opt,
        completion_opt = &comp_opt
    ))
}
