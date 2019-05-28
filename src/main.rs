extern crate rand;

use std::io;
use std::fs;
use std::net::{TcpListener, TcpStream};
use std::io::{Write};
use rand::Rng;


fn korwin_first_part() -> String {
    match rand::thread_rng().gen_range(0, 6){
        0 => "Ja chcę powiedzieć jedną rzecz:".into(),
        1 => "Trzeba powiedzieć jasno:".into(),
        2 => "Jak powiedział wybitny krakowianin Stanisław Lem,".into(),
        3 => "Proszę mnie dobrze zrozumieć:".into(),
        4 => "Ja chciałem państwu przypomnieć, że".into(),
        5 => "Niech państwo nie mają złudzeń:".into(),
        6 => "Powiedzmy to wyraźnie:".into(),
        _ => "".into()
    }
}

fn korwin_second_part() -> String {
    match rand::thread_rng().gen_range(0, 6){
        0 => "przedstawiciele czerwonej hołoty".into(),
        1 => "ci wszyscy (tfu!) geje".into(),
        2 => "funkcjonariusze reżymowej telewizji".into(),
        3 => "tak zwani ekolodzy".into(),
        4 => "ci wszyscy (tfu!) demokraci".into(),
        5 => "agenci bezpieki".into(),
        6 => "feminazistki".into(),
        _ => "".into()
    }
}

fn korwin_third_part() -> String {
    match rand::thread_rng().gen_range(0, 6){
        0 => "zupełnie bezkarnie".into(),
        1 => "całkowicie bezczelnie".into(),
        2 => "o poglądach na lewo od komunizmu".into(),
        3 => "celowo i świadomie".into(),
        4 => "z premedytacją".into(),
        5 => "od czasów Okrągłego Stołu".into(),
        6 => "w ramach postępu".into(),
        _ => "".into()
    }
}

fn korwin_fourth_part() -> String {
    match rand::thread_rng().gen_range(0, 6){
        0 => "nawołują do podniesienia podatków".into(),
        1 => "próbują wyrzucić kierowców z miast".into(),
        2 => "próbują skłócić Polskę z Rosją".into(),
        3 => "głoszą brednie o globalnym ociepleniu".into(),
        4 => "zakazują posiadania broni".into(),
        5 => "nie dopuszczają prawicy do władzy".into(),
        6 => "uczą dzieci homoseksualizmu".into(),
        _ => "".into()
    }
}

fn korwin_fifth_part() -> String {
    match rand::thread_rng().gen_range(0, 6){
        0 => "bo dzięki temu moga kraść".into(),
        1 => "bo dostają za to pieniądze".into(),
        2 => "bo tak się uczy w państwowej szkole".into(),
        3 => "bo bez tego (tfu!) demokracja nie może istnieć".into(),
        4 => "bo głupich jest więcej niż mądrych".into(),
        5 => "bo chcą stworzyć raj na ziemi".into(),
        6 => "bo chcą niszczyć cywilizacje białego człowieka".into(),
        _ => "".into()
    }
}

fn korwin_sixth_part() -> String {
    match rand::thread_rng().gen_range(0, 6){
        0 => "przez kolejne kadencje".into(),
        1 => "o czym się nie mówi".into(),
        2 => "i właśnie dlatego Europa umiera".into(),
        3 => "ale przyjdą muzułmanie i zrobią porządek".into(),
        4 => "- tak samo zresztą jak za Hitlera".into(),
        5 => "- proszę zobaczyć, co się dzieje na Zachodzie, jeśli mi państwo nie wierzą".into(),
        6 => "co lat temu sto nikomu nie przyszłoby nawet do głowy".into(),
        _ => "".into()
    }
}

fn korwin_entire_sentence() -> String {
    return format!("{} {} {} {} {} {}",
            korwin_first_part(),
            korwin_second_part(),
            korwin_third_part(),
            korwin_fourth_part(),
            korwin_fifth_part(),
            korwin_sixth_part());
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    stream.write_all(korwin_entire_sentence().as_bytes())?;
    Ok(())
}

fn main() -> io::Result<()>{
    let ip = fs::read_to_string("ip.txt").expect("Something went wrong.");
    let listener = TcpListener::bind(ip)?;
    for x in listener.incoming(){
        handle_client(x?);
    }
    Ok(())
}
