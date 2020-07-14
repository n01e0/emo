#[macro_use]
extern crate clap;
mod emoji;

fn main() {
    let matches = clap_app!(emo => 
            (version:   crate_version!())
            (author:    crate_authors!())
            (about:     crate_description!())
            (@arg font: -f --font +takes_value "select font")
            (@arg font_list: --font_list "font list")
            (@arg color: -c --color +takes_value "text color(RGBA)")
            (@arg back_color: -b --bg +takes_value "background color(RGBA)")
            (@arg text: +required "emoji text")
            (@arg out: -o --outpath +takes_value "output path(default is $HOME/emoji)")
        ).get_matches();

    if matches.is_present("font_list") {
        emoji::font_list();
    } else {

        let mut genelater = emoji::Generator::new();
        genelater
            .font(matches.value_of("font").unwrap_or("notosans-mono-bold"))
            .color(matches.value_of("color").unwrap_or("EC71A1FF"))
            .back_color(matches.value_of("back_color").unwrap_or("00000000"))
            .text(matches.value_of("text").unwrap());

        if let Err(e) = genelater.gen(matches.value_of("out")) {
            eprintln!("{:?}", e);
        }
    }
}
