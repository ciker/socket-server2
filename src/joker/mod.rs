use crate::messager::Messager;
use crate::utils::normalize_input::normalize;

pub fn joker(mut messager: Messager) {
    match messager.send(String::from("toc toc")) {
        Ok(_) => (),
        Err(error) => {
            eprintln!("Error: {}", error);
            return;
        }
    }

    if let Ok(msg) = messager.read() {
        let normalized = normalize(msg);
        match normalized {
            _ if normalized == "close" => return,
            _ if normalized == "quem e" => (),
            _ => return,
        }
    }

    match messager.send(String::from("Mario")) {
        Ok(_) => (),
        Err(error) => {
            eprintln!("Error: {}", error);
            return;
        }
    }

    if let Ok(msg) = messager.read() {
        let normalized = normalize(msg);
        match normalized {
            _ if normalized == "close" => return,
            _ if normalized == "que mario" => (),
            _ => return,
        }
    }

    let answer = "Mario é um personagem fictício da franquia e série de jogos eletrônicos Mario da Nintendo, criado pelo desenvolvedor e designer de jogos eletrônicos japonês Shigeru Miyamoto. Servindo como mascote da Nintendo e protagonista homônimo da série, Mario já apareceu em mais de 200 jogos desde sua criação.\n
    Mario é retratado como um encanador italiano baixinho rechonchudo e bigodudo vindo do Brooklyn que reside no Reino dos Cogumelos. Ele repetidamente tem a missão de resgatar a Princesa Peach do vilão Bowser, e impedir seus diversos planos de destruir e dominar o reino. Mario também tem outros inimigos ou rivais, incluindo Donkey Kong e Wario. Desde 1995, Mario é dublado por Charles Martinet.";

    match messager.send(String::from(answer)) {
        Ok(_) => (),
        Err(error) => {
            eprintln!("Error: {}", error);
            return;
        }
    }
}
