use crate::messager::Messager;

pub fn joker(mut messager: Messager) {
    match messager.send(String::from("toc toc")) {
        Ok(_) => (),
        Err(error) => eprintln!("Error: {}", error),
    }
    while let Ok(msg) = messager.read() {
        match msg {
            _ if msg == "close" => {
                drop(messager);
                break;
            }
            _ => match messager.send(String::from("sucesso")) {
                Ok(_) => (),
                Err(error) => eprintln!("Error: {}", error),
            },
        };
    }
}
