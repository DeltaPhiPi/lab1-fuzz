use std::{fmt::{format, Display, Write}, str::FromStr, sync::LazyLock};

use clap::Parser;
use rand::{distributions::Standard, prelude::Distribution, Rng, seq::SliceRandom};
use rand_derive2::RandGen;


#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
struct Fecha {
    año: i32,
    mes: i32,
    día: i32,
}
fn is_leap_year(year: i32) -> bool {
    return (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}
fn dias_mes(mes: i32, año: i32) -> i32 {
    match mes {
        2 => if is_leap_year(año) {29} else {28},
        4 | 6 | 9 | 11 => 30,
        _ => 31
    }
}
impl rand::distributions::Distribution<Fecha> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Fecha {
        let año = rng.gen_range(1999..2010);
        let mes = rng.gen_range(1..=12);
        let día = rng.gen_range(1..=dias_mes(mes, año));
        return Fecha { año, mes, día }
    }
}
impl Display for Fecha {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.día.fmt(f)?;
        f.write_char('/')?;
        self.mes.fmt(f)?;
        f.write_char('/')?;
        self.año.fmt(f)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Copy, Default)]
struct Cédula(i32);
impl Distribution<Cédula> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cédula {
        Cédula(rng.gen_range(100..150))
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Default)]
struct Nombre(String);
#[derive(Debug, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Default)]
struct Apellido(String);

static NOMBRES: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    include_str!("../nombres").lines().collect()
});
static APELLIDOS: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    include_str!("../apellidos").lines().collect()
});

impl Distribution<Nombre> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Nombre {
        Nombre(NOMBRES[rng.gen_range(0..2000)].to_owned())
    }
}
impl Distribution<Apellido> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Apellido {
        Apellido(APELLIDOS[rng.gen_range(0..1000)].to_owned())
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
#[derive(RandGen)]
enum Token {
    CrearFecha(Fecha),
    ImprimirFecha,
    LiberarFecha,
    AumentarDias(i32),
    CompararFechas(Fecha, Fecha),
    CrearPersona(Cédula, Nombre, Apellido),
    ImprimirCIPersona,
    ImprimirFechaPersona,
    ImprimirNombreYApellidoPersona,
    ImprimirPersona,
    LiberarPersona,
    EsMasJovenPersona(Fecha, Cédula, Nombre, Apellido),
    CrearGrupo,
    AgregarAGrupo,
    ImprimirGrupo,
    LiberarGrupo,
    EstaEnGrupo(Cédula),
    HayPersonasFecha(Fecha),
    RemoverDeGrupo(Cédula),
    #[rand_derive(skip)]
    Fin
}

impl Token {
    fn to_string(&self) -> String {
        use Token::*;
        let mut s = match self {
            CrearFecha(f) => format!("crearFecha {}", f),
            ImprimirFecha => "imprimirFecha".to_owned(),
            LiberarFecha => "liberarFecha".to_owned(),
            AumentarDias(x) => format!("aumentarDias {}", x).to_owned(),
            CompararFechas(f1,f2) => format!("compararFechas {} {}", f1, f2).to_owned(),
            CrearPersona(x, n, a) => format!("crearPersona {} {} {}", x.0, n.0, a.0),
            ImprimirCIPersona => "imprimirCiPersona".to_owned(),
            ImprimirFechaPersona => "imprimirFechaPersona".to_owned(),
            ImprimirNombreYApellidoPersona => "imprimirNombreYApellidoPersona".to_owned(),
            ImprimirPersona => "imprimirPersona".to_owned(),
            LiberarPersona => "liberarPersona".to_owned(),
            EsMasJovenPersona(f1, x, n, a) => format!("esMasJovenPersona {} {} {} {}", f1, x.0, n.0, a.0),
            CrearGrupo => "crearGrupo".to_owned(),
            AgregarAGrupo => "agregarAGrupo".to_owned(),
            ImprimirGrupo => "imprimirGrupo".to_owned(),
            LiberarGrupo => "liberarGrupo".to_owned(),
            EstaEnGrupo(x) => format!("estaEnGrupo {}", x.0),
            HayPersonasFecha(f) => format!("hayPersonasFecha {}", f),
            RemoverDeGrupo(x) => format!("removerDeGrupo {}", x.0),
            Fin => "Fin".to_owned()
        };
        s.push('\n');
        s
    }
}



#[derive(clap::ValueEnum, Debug, PartialEq, Clone, Copy, Eq)]
#[clap(rename_all = "kebab_case")]
enum TestCase {
    Fecha,
    Persona,
    Grupo,
}
impl FromStr for TestCase {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "fecha" => Self::Fecha,
            "persona" => Self::Persona,
            "grupo" => Self::Grupo,
            _ => return Err(())
        })
    }
}
impl TestCase {
    fn to_tokens(&self, size: usize) -> Vec<Token> {
        match self {
            TestCase::Fecha => generate_date_test_cases(size),
            TestCase::Grupo => generate_group_test_cases(size),
            TestCase::Persona => generate_person_test_cases(size)
        }
    }
}
fn generate_person_test_cases(size: usize) -> Vec<Token> {
    let mut ra = rand::thread_rng();
    let mut v = vec![Token::CrearFecha(ra.gen()), Token::CrearPersona(ra.gen(), ra.gen(), ra.gen())];
    let mut flag = false;
    for _ in 0..size {
    match ra.gen_range((if flag {1} else {flag = false; 0})..3) {
        0 => {
            v.extend([Token::ImprimirPersona]);
            flag = true;
        },
        1 => {
            v.extend([Token::EsMasJovenPersona(ra.gen(), ra.gen(), ra.gen(), ra.gen())]);
        },
        2 => {
            v.extend([Token::LiberarPersona, Token::CrearFecha(ra.gen()),Token::CrearPersona(ra.gen(), ra.gen(), ra.gen())])
        },
        _ => unreachable!()
    }}
    v.extend([Token::LiberarFecha, Token::LiberarPersona, Token::LiberarGrupo, Token::Fin]);
    return v;
}
fn generate_date_test_cases(size: usize) -> Vec<Token> {
    let mut ra = rand::thread_rng();
    let mut v = vec![Token::CrearFecha(ra.gen())];
    for _ in 0..size {
        match ra.gen_range(0..3) {
            0 => {
                v.extend([Token::AumentarDias(ra.gen_range(0..300)), Token::ImprimirFecha]);
            },
            1 => {
                v.extend([Token::CompararFechas(ra.gen(), ra.gen())]);
            },
            2 => {
                v.extend([Token::LiberarFecha, Token::CrearFecha(ra.gen())])
            }
            _ => unreachable!()
        };
    }
    v.extend([Token::LiberarFecha, Token::LiberarPersona, Token::LiberarGrupo, Token::Fin]);
    return v;
}
fn generate_group_test_cases(size: usize) -> Vec<Token> {
    let mut ra = rand::thread_rng();
    let mut v = vec![Token::CrearGrupo];
    let mut cédulas = vec![ra.gen()];
    let mut fechas = vec![ra.gen()];
    for _ in 0..size {
    match ra.gen_range(0..10) {
        0..4 => {
            cédulas.push(ra.gen());
            fechas.push(ra.gen());
            v.extend([
                Token::CrearFecha(*fechas.last().unwrap()), 
                Token::CrearPersona(*cédulas.last().unwrap(), ra.gen(), ra.gen()),
                Token::AgregarAGrupo,
            ])
        },
        4 => {
            v.push(Token::ImprimirGrupo);
        },
        5..7 => {
            v.push(Token::EstaEnGrupo(*cédulas.choose(&mut ra).unwrap()));
        },
        7..9 => {
            v.push(Token::RemoverDeGrupo(*cédulas.choose(&mut ra).unwrap()));
        },
        9 => {
            v.push(Token::HayPersonasFecha(*fechas.choose(&mut ra).unwrap()));
        }
        _ => unreachable!()
    }}
    v.extend([Token::LiberarFecha, Token::LiberarPersona, Token::LiberarGrupo, Token::Fin]);
    return v;
}
#[derive(clap::Parser)]
struct CLIOptions {
    r#type: TestCase,
    len: usize
}

fn main() {
    let options = CLIOptions::parse();
    println!("{}", options.r#type.to_tokens(options.len).into_iter().map(|x| x.to_string()).collect::<Vec<_>>().join(""));    
}
