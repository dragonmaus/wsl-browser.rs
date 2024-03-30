extern crate getopt;

use getopt::Opt;

program::main!("wsl-browser");

fn usage_line(program_name: &str) -> String {
    format!("Usage: {} [URI]", program_name)
}

fn print_usage(program_name: &str) {
    println!("{}", usage_line(program_name));
    println!();
    println!("  -h   display this help");
}

fn program(name: &str) -> program::Result {
    let mut args = program::args();
    let mut opts = getopt::Parser::new(&args, "h");

    #[allow(clippy::never_loop)]
    loop {
        match opts.next().transpose()? {
            None => break,
            Some(opt) => match opt {
                Opt('h', None) => {
                    print_usage(name);
                    return Ok(0);
                },
                _ => unreachable!(),
            },
        }
    }

    let mut args = args.split_off(opts.index());

    if args.is_empty() {
        args.push(String::from("."));
    }

    for arg in args {
        wsl_browser::open(&arg)?;
    }

    Ok(0)
}
