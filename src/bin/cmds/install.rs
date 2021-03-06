use super::{args, logger, match_backends, match_threads};
use clap::{App, Arg, ArgMatches, SubCommand};
use elba::{
    cli::build,
    package::Spec,
    util::{config::Config, errors::Res},
};
use failure::ResultExt;
use std::{env::current_dir, str::FromStr};

pub fn cli() -> App<'static, 'static> {
    SubCommand::with_name("install")
        .about("Installs a package's artifacts")
        .arg(Arg::with_name("spec"))
        .arg(args::build_threads())
        .arg(args::target_bin())
        .arg(
            Arg::with_name("force")
                .long("force")
                .help("Overwrite existing installed binaries"),
        )
}

pub fn exec(c: &mut Config, args: &ArgMatches) -> Res<String> {
    let current = current_dir();

    let proj = if let Some(spec) = args.value_of_lossy("spec") {
        let spec = &*spec;
        let spec = Spec::from_str(spec)
            .with_context(|e| format_err!("the spec `{}` is invalid:\n{}", spec, e))?;
        Ok(spec)
    } else if let Ok(d) = current {
        Err(d)
    } else {
        bail!("no package was specified to be installed and the current directory is inaccessible")
    };

    let logger = logger(c);
    let indices = c.indices.to_vec();
    let global_cache = c.layout();
    let threads = match_threads(c, args);
    let shell = c.shell();

    let ctx = build::BuildCtx {
        indices,
        global_cache,
        logger,
        threads,
        shell,
    };

    let targets = args
        .values_of("bin")
        .map(|x| x.collect())
        .unwrap_or_else(|| vec![]);

    let backend = match_backends(c, args);

    build::install(&ctx, proj, &targets, &backend, args.is_present("force"))
}
