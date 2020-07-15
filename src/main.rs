#[macro_use]
extern crate clap;
extern crate opener;
mod emoji;

fn main() {
    let matches = clap_app!(emo => 
            (version:   crate_version!())
            (author:    crate_authors!())
            (about:     crate_description!())
            (@arg font: -f --font +takes_value "select font")
            (@arg font_list: --font_list "font list")
            (@arg color: -c --color +takes_value "text color(RGBA)")
            (@arg back_color: -b --back_color +takes_value "background color(RGBA)")
            (@arg text: +required "emoji text")
            (@arg out: -o --outpath +takes_value "output path(default is $HOME/emoji)")
            (@arg open: --open "xdg-open")
        ).get_matches();

    if matches.is_present("font_list") {
        emoji::font_list();
    } else {
        let text = matches.value_of("text").unwrap();
        let mut genelater = emoji::Generator::new();
        genelater
            .font(matches.value_of("font").unwrap_or("notosans-mono-bold"))
            .color(matches.value_of("color").unwrap_or("EC71A1FF"))
            .back_color(matches.value_of("back_color").unwrap_or("00000000"))
            .text(text);

        if let Err(e) = genelater.gen(matches.value_of("out")) {
            eprintln!("{:?}", e);
            std::process::exit(1);
        }

        let filename = if let Some(name) = matches.value_of("out") {
            name.to_string()
        } else {
            format!("{}/emoji/{}.png", std::env::var("HOME").unwrap(), text)
        };
        if matches.is_present("open") {
            opener::open(filename).expect("Can't open file");
        }
    }
}
