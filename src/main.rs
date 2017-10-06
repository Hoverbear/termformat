#[macro_use]
extern crate clap;
extern crate term;
#[macro_use]
extern crate error_chain;

use std::io::{self, Read};

quick_main!(handle);

fn handle() -> Result<()> {
    let args = opts().get_matches();
    let mut terminal = term::stdout().ok_or(ErrorKind::CouldNotOpenTerminal)?;
    configure(&args, &mut *terminal)?;

    let content = if let Some(content) = args.value_of("content") {
        String::from(content)
    } else {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer)?;
        buffer
    };

    write!(terminal, "{}", content)?;
    terminal.reset()?;
    Ok(())
}

fn opts() -> clap::App<'static, 'static> {
    clap::App::new(env!("CARGO_PKG_NAME"))
        .version(crate_version!())
        .author(crate_authors!())
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .global_settings(
            &[
                clap::AppSettings::ColoredHelp,
                clap::AppSettings::GlobalVersion,
            ],
        )
        .arg(
            clap::Arg::with_name("foreground")
                .long("fg")
                .alias("foreground")
                .help("Foreground color.")
                .takes_value(true)
                .possible_values(&Color::variants())
                .required(false),
        )
        .arg(
            clap::Arg::with_name("background")
                .long("bg")
                .alias("background")
                .help("Background color.")
                .takes_value(true)
                .possible_values(&Color::variants())
                .required(false),
        )
        .arg(
            clap::Arg::with_name("bold")
                .long("bold")
                .alias("b")
                .short("b")
                .help("Make text bold.")
                .takes_value(false)
                .required(false),
        )
        .arg(
            clap::Arg::with_name("italics")
                .long("italics")
                .alias("i")
                .short("i")
                .help("Make text italics.")
                .takes_value(false)
                .required(false),
        )
        .arg(
            clap::Arg::with_name("underline")
                .long("underline")
                .alias("u")
                .short("u")
                .help("Make text underline.")
                .takes_value(false)
                .required(false),
        )
        .arg(
            clap::Arg::with_name("content")
                .help("The content to format.")
                .takes_value(true)
                .required(false),
        )
}

fn configure(args: &clap::ArgMatches, terminal: &mut term::StdoutTerminal) -> Result<()> {
    if args.is_present("foreground") {
        let color = value_t!(args.value_of("foreground"), Color)?;
        terminal.attr(term::Attr::ForegroundColor(color as u16))?;
    }

    if args.is_present("background") {
        let color = value_t!(args.value_of("background"), Color)?;
        terminal.attr(term::Attr::BackgroundColor(color as u16))?;
    }

    if args.is_present("bold") {
        terminal.attr(term::Attr::Bold)?;
    }

    if args.is_present("italics") {
        terminal.attr(term::Attr::Italic(true))?;
    }

    if args.is_present("underline") {
        terminal.attr(term::Attr::Underline(true))?;
    }
    Ok(())
}

arg_enum! {
    #[derive(Debug)]
    #[allow(non_camel_case_types)]
    enum Color {
        black = term::color::BLACK as isize,
        blue = term::color::BLUE as isize,
        bright_blue = term::color::BRIGHT_BLUE as isize,
        bright_black = term::color::BRIGHT_BLACK as isize,
        bright_cyan = term::color::BRIGHT_CYAN as isize,
        bright_green = term::color::BRIGHT_GREEN as isize,
        bright_magenta = term::color::BRIGHT_MAGENTA as isize,
        bright_red = term::color::BRIGHT_RED as isize,
        bright_white = term::color::BRIGHT_WHITE as isize,
        bright_yellow = term::color::BRIGHT_YELLOW as isize,
        cyan = term::color::CYAN as isize,
        green = term::color::GREEN as isize,
        magenta = term::color::MAGENTA as isize,
        red = term::color::RED as isize,
        white = term::color::WHITE as isize,
        yellow = term::color::YELLOW as isize
    }
}

error_chain! {
    foreign_links {
        Clap(clap::Error);
        Term(term::Error);
        Io(std::io::Error);
    }

    errors {
        CouldNotOpenTerminal {
            description("Could not open terminal the terminal.")
        }
        NoContent {
            description("Did not find content to format.")
        }
    }
}
