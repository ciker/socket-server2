use crate::messager::Messager;
use crate::utils::normalize_input::normalize;

pub fn joker(mut messager: Messager) {
    match talk(&mut messager, "Toc Toc", vec!["quem e"]) {
        Ok(_) => (),
        Err(error) => {
            eprintln!("{}", error);
            return;
        }
    }

    match talk(
        &mut messager,
        "Mario",
        vec!["que mario", "mario que mario", "mario"],
    ) {
        Ok(_) => (),
        Err(error) => {
            eprintln!("{}", error);
            return;
        }
    }

    let answer = "Mario é um personagem fictício da franquia e série de jogos eletrônicos Mario da Nintendo, criado pelo desenvolvedor e designer de jogos eletrônicos japonês Shigeru Miyamoto. Servindo como mascote da Nintendo e protagonista homônimo da série, Mario já apareceu em mais de 200 jogos desde sua criação.\n
    Mario é retratado como um encanador italiano baixinho rechonchudo e bigodudo vindo do Brooklyn que reside no Reino dos Cogumelos. Ele repetidamente tem a missão de resgatar a Princesa Peach do vilão Bowser, e impedir seus diversos planos de destruir e dominar o reino. Mario também tem outros inimigos ou rivais, incluindo Donkey Kong e Wario. Desde 1995, Mario é dublado por Charles Martinet.\n
    Leia mais em https://pt.wikipedia.org/wiki/Mario_(personagem)";

    match messager.send(String::from(answer)) {
        Ok(_) => (),
        Err(error) => {
            eprintln!("Error: {}", error);
            return;
        }
    }
}

fn talk(messager: &mut Messager, output: &str, accepted_answers: Vec<&str>) -> Result<(), String> {
    match messager.send(String::from(output)) {
        Ok(_) => (),
        Err(_) => return Err("Could not write".to_owned()),
    };

    if let Ok(msg) = messager.read() {
        let normalized = normalize(msg);
        match normalized {
            _ if normalized == "close" => return Ok(()),
            _ if is_valid_answer(normalized, &accepted_answers) => Ok(()),
            _ => {
                let mut aux_messager = messager;
                return talk(&mut aux_messager, output, accepted_answers);
            }
        }
    } else {
        Err("Could not read".to_owned())
    }
}

fn is_valid_answer(answer: String, accepted_answers: &Vec<&str>) -> bool {
    for quote in accepted_answers {
        if answer.eq(quote) {
            return true;
        }
    }
    false
}
