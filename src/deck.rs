//! Mazzi predefiniti (incorporati) e caricamento di mazzi personalizzati TOML.
//!
//! Il mazzo e' un dato esterno fin dall'inizio: i mazzi built-in sono solo il
//! punto di partenza, l'utente puo' aggiungerne in `~/.oracledecktui/decks/`.

use crate::model::{Card, Deck};

fn card(
    name: &str,
    number: &str,
    upright: &str,
    reversed: &str,
    keywords: &[&str],
    color: [u8; 3],
    symbol: &str,
) -> Card {
    Card {
        name: name.to_string(),
        number: number.to_string(),
        upright: upright.to_string(),
        reversed: reversed.to_string(),
        keywords: keywords.iter().map(|s| s.to_string()).collect(),
        symbol: symbol.trim_matches('\n').to_string(),
        color,
    }
}

/// "Void Arcana" — oracolo originale dark surrealist.
pub fn void_arcana() -> Deck {
    Deck {
        name: "Void Arcana".to_string(),
        description: "Oracolo originale dark surrealist. Nessun arcano è ciò che sembra.".to_string(),
        reversed_chance: 0.4,
        cards: vec![
            card(
                "The Null", "0",
                "Un ciclo si apre nel vuoto. Potenziale puro, nessuna forma ancora scelta.",
                "Paralisi, paura del salto. Il vuoto ti trattiene invece di liberarti.",
                &["inizio", "vuoto", "potenziale"],
                [150, 130, 210],
                r#"
   ╭─────────╮
  ╱ · · · · · ╲
  │ ·   ╲ ╱   · │
  │ ·    ∅    · │
  │ ·   ╱ ╲   · │
  ╲ · · · · · ╱
   ╰─────────╯
"#,
            ),
            card(
                "The Signal", "I",
                "Volontà che si irradia. Ciò che pensi comincia a trasmettersi nel reale.",
                "Rumore, dispersione. Il messaggio si perde prima di arrivare.",
                &["manifestazione", "volontà"],
                [90, 220, 200],
                r#"
   (( ● ))
  ·  ╲│╱  ·
     ╲│╱
     ╱│╲
    ╱ │ ╲
   ═══╪═══
      │
  ▂▂▂▂█▂▂▂▂
"#,
            ),
            card(
                "The Oracle", "II",
                "Sapere che non passa dalle parole. Ascolta il silenzio tra i dati.",
                "Segreti che marciscono. Intuizione soffocata dal dubbio.",
                &["intuizione", "mistero"],
                [120, 120, 230],
                r#"
   ╱▔▔▔▔▔╲
  │ ▄   ▄ │
  │ ◑   ◑ │
  │   ▽   │
   ╲ ▁▁▁ ╱
     ╲ ╱
  ▚▞▚ ▞ ▚▞
"#,
            ),
            card(
                "The Current", "III",
                "Forza che scorre invece di spingere. Potere quieto, ininterrotto.",
                "Cortocircuito. La forza si scarica a vuoto, ti brucia.",
                &["energia", "flusso"],
                [230, 160, 80],
                r#"
      ◈
     ╱⚡╲
    │≈≈≈│
    │≈≈≈│
    │≈≈≈│
     ╲⚡╱
      ◈
"#,
            ),
            card(
                "The Sovereign", "IV",
                "Struttura che regge. Confini scelti, non subiti.",
                "Controllo che diventa gabbia. Rigidità che spezza invece di reggere.",
                &["ordine", "struttura"],
                [200, 90, 90],
                r#"
   ♦ ▟█▙ ♦
    ▟███▙
   ┏━━━━━┓
   ┃ ▐█▌ ┃
   ┃ ▐█▌ ┃
   ┗━━━━━┛
"#,
            ),
            card(
                "The Drift", "V",
                "Movimento senza sforzo verso ciò che ti chiama. Lasciati portare.",
                "Deriva cieca. Ti muovi, ma nessuno tiene il timone.",
                &["momento", "viaggio"],
                [80, 180, 230],
                r#"
     ______
   ╱‾      ‾╲__
  ⟨  ◦    ◦    ⟩
   ╲__      __╱
      ‾‾‾‾‾‾
  ≈ ≈ ≈ ≈ ≈ ≈
"#,
            ),
            card(
                "The Hermit Node", "VI",
                "Ti scolleghi per sentirti. La solitudine come antenna.",
                "Isolamento che diventa muro. Ti perdi nel buio che cercavi.",
                &["solitudine", "ricerca"],
                [130, 140, 160],
                r#"
     ◢██◣
    ╱ ◉◉ ╲
   │      │
    ╲    ╱
     ╲  ╱
      ▼▼
   ·  *  ·
"#,
            ),
            card(
                "The Cycle", "VII",
                "La ruota gira: ciò che è sotto salirà. Nulla resta fermo.",
                "Ti aggrappi al giro sbagliato. Ripeti invece di ruotare.",
                &["destino", "cambiamento"],
                [220, 190, 90],
                r#"
    ╭──◍──╮
   ◍  ╲│╱  ◍
   ├──╳╳╳──┤
   ◍  ╱│╲  ◍
    ╰──◍──╯
"#,
            ),
            card(
                "The Balance", "VIII",
                "Verità fredda, senza sconti. Ciò che pesa, pesa davvero.",
                "Bilancia truccata. Ti menti sul peso delle cose.",
                &["verità", "giustizia"],
                [180, 200, 200],
                r#"
       △
    ───┴───
   ╱   │   ╲
  ▢    │    ▢
   ╲   │   ╱
    ═══╧═══
"#,
            ),
            card(
                "The Suspended", "IX",
                "Ti fermi a testa in giù per vedere il vero. Resa che rivela.",
                "Sacrificio inutile. Stai appeso per niente, ostinato.",
                &["resa", "sospensione"],
                [110, 130, 190],
                r#"
   ════╤════
       │
      ◯◯
     ╱   ╲
     ╲   ╱
      ╲ ╱
       ▽
"#,
            ),
            card(
                "The Reboot", "XIII",
                "Morte come riavvio. Qualcosa deve chiudersi perché il resto giri.",
                "Ti aggrappi al vecchio sistema. Il riavvio rimandato marcisce.",
                &["trasformazione", "fine"],
                [200, 70, 110],
                r#"
     ▟███▙
    ▟ ☠ ▙
   │  ⟳   │
    ▜ ▁ ▛
     ▜███▛
    ▂▂▂▂▂▂▂
"#,
            ),
            card(
                "The Crash", "XVI",
                "La torre cade perché era falsa. Distruzione che libera.",
                "Crollo negato. Punti l'edificio invece di lasciarlo andare.",
                &["collasso", "rivelazione"],
                [240, 100, 60],
                r#"
   *  ▟█▙  *
       █▓█
      ╱█▓█╲
     ╱ █▓█ ╲
   ✦ ▔▔▔▔▔▔▔ ✦
    ░░░░░░░░░
"#,
            ),
            card(
                "The Beacon", "XVII",
                "Un faro nel rumore. Speranza sottile ma sufficiente.",
                "Faro spento. Cerchi luce dove hai smesso di guardare.",
                &["speranza", "guida"],
                [120, 210, 230],
                r#"
       ✦
      ╱♢╲
     ╱ ║ ╲
    ▔▔▔║▔▔▔
       ║
       ║
    ~~~~▚~~~~
"#,
            ),
            card(
                "The Static", "XVIII",
                "La luna trasmette solo statica. Ciò che vedi è per metà tuo.",
                "Inganno che si dissolve. La nebbia si apre, brucia un po'.",
                &["illusione", "inconscio"],
                [90, 100, 150],
                r#"
    ▁▂▃▄▅▄▃▂▁
    ▞▚▞▚▞▚▞▚▞
   ) ▁▂▃▂▁ (
    ▞▚▞▚▞▚▞▚▞
    ▁▂▃▄▅▄▃▂▁
"#,
            ),
            card(
                "The Solar", "XIX",
                "Luce piena, niente ombre a nascondersi. Chiarezza calda.",
                "Sole accecante. Tanta luce che non vedi più nulla.",
                &["chiarezza", "gioia"],
                [240, 200, 80],
                r#"
   ╲   │   ╱
    ╲  │  ╱
   ── (☼) ──
    ╱  │  ╲
   ╱   │   ╲
"#,
            ),
            card(
                "The Network", "XXI",
                "Tutto si connette. Il cerchio si chiude e diventa mondo.",
                "Nodi scollegati. Il tutto si sfilaccia ai bordi.",
                &["completezza", "connessione"],
                [110, 220, 160],
                r#"
   ◍───────◍
   │╲     ╱│
   │ ╲   ╱ │
   │  ╲ ╱  │
   │   ◍   │
   │  ╱ ╲  │
   ◍───────◍
"#,
            ),
        ],
    }
}

/// "Neon Oracle" — mazzo piu' piccolo, per dimostrare la scelta multi-mazzo.
pub fn neon_oracle() -> Deck {
    Deck {
        name: "Neon Oracle".to_string(),
        description: "Oracolo breve al neon: sei frammenti per una risposta rapida.".to_string(),
        reversed_chance: 0.35,
        cards: vec![
            card(
                "Glow", "†",
                "Qualcosa in te si accende. Segui il bagliore.",
                "Luce fredda, imitata. Non tutto ciò che brilla è tuo.",
                &["accensione"],
                [255, 90, 200],
                r#"
   ✦  ✦  ✦
  ✦ ▄▄▄▄▄ ✦
  ✦ █▓▓▓█ ✦
  ✦ ▀▀▀▀▀ ✦
   ✦  ✦  ✦
"#,
            ),
            card(
                "Wire", "‡",
                "Un legame regge. Fidati del filo teso.",
                "Filo scoperto. Il contatto brucia invece di unire.",
                &["connessione"],
                [90, 230, 255],
                r#"
  ●╌╌╌╌╌╌╌●
   ╲     ╱
    ╲   ╱
     ╲ ╱
      ╳
     ╱ ╲
  ●╌╌╌╌╌╌╌●
"#,
            ),
            card(
                "Hex", "◇",
                "Uno schema nascosto ti protegge. Non tutto è caos.",
                "Schema rotto. La protezione ha una crepa.",
                &["pattern"],
                [180, 120, 255],
                r#"
   ⬡ ⬡ ⬡ ⬡
  ⬡ ⬡ ⬡ ⬡ ⬡
   ⬡ ⬢ ⬢ ⬡
  ⬡ ⬡ ⬡ ⬡ ⬡
   ⬡ ⬡ ⬡ ⬡
"#,
            ),
            card(
                "Fade", "◈",
                "Lascia sfumare ciò che ha fatto il suo tempo.",
                "Ti dissolvi troppo presto. Resta ancora un momento.",
                &["dissolvenza"],
                [130, 150, 190],
                r#"
  ███████████
  ▓▓▓▓▓▓▓▓▓▓▓
  ▒▒▒▒▒▒▒▒▒▒▒
  ░░░░░░░░░░░
  · · · · · ·
"#,
            ),
            card(
                "Spark", "✧",
                "Un'idea improvvisa. Coglila prima che svanisca.",
                "Scintilla a vuoto. Tanta energia, nessuna direzione.",
                &["intuizione"],
                [255, 210, 90],
                r#"
      ╱✧╲
     ╱ │ ╲
    ╱  │  ╲
   ✧───┼───✧
    ╲  │  ╱
     ╲ │ ╱
      ╲✧╱
"#,
            ),
            card(
                "Echo", "◐",
                "Qualcosa che hai detto torna a te. Ascoltalo.",
                "Eco che non finisce. Ripeti un passato che è passato.",
                &["ritorno"],
                [200, 120, 160],
                r#"
  ◉ ⟩ ⟩ ⟩ ⟩
  ◉ ⟩ ⟩ ⟩
  ◉ ⟩ ⟩
  ◉ ⟩ ⟩ ⟩
  ◉ ⟩ ⟩ ⟩ ⟩
"#,
            ),
        ],
    }
}

/// "Mors Initium" — necromagia, esoterismo e morte. La morte come soglia.
pub fn mors_initium() -> Deck {
    Deck {
        name: "Mors Initium".to_string(),
        description: "Oracolo necromantico: la morte come inizio. Ossa, silenzio e ciò che torna da oltre il velo.".to_string(),
        reversed_chance: 0.45,
        cards: vec![
            card(
                "La Morta", "☠",
                "La fine è una porta, non un muro. Ciò che muore lascia il posto a ciò che era già in viaggio.",
                "Resisti a un cerchio che si chiude. Trattieni un cadavere che merita la terra.",
                &["fine", "soglia", "rinascita"],
                [200, 200, 210],
                r#"
   ╭───────╮
  ╱ ▄   ▄ ╲
 │    ▽    │
 │   ───   │
  ╲ ▔▔▔ ▔ ╱
   ╰───────╯
"#,
            ),
            card(
                "Il Silenzio", "I",
                "Prima della voce c'è il vuoto. Ascoltalo: parla più forte di ogni parola.",
                "Silenzio pieno di ciò che non vuoi dire. Tacere è diverso da ascoltare.",
                &["silenzio", "vuoto", "vigilia"],
                [140, 140, 160],
                r#"
      ( )
     (   )
      ─┼─
      ███
     █████
      ███
"#,
            ),
            card(
                "La Reliquia", "II",
                "Ciò che resta conserva il peso di chi lo possedette. Trattieni la memoria.",
                "Adori la custodia e dimentichi cosa custodisce. La reliquia è vuota.",
                &["memoria", "eredità"],
                [230, 220, 190],
                r#"
   ◈─────◈
  ╱       ╲
  │       │
  ╲       ╱
   ◈─────◈
"#,
            ),
            card(
                "Il Rito", "III",
                "Ripeti i gesti antichi finché cominciano a significare. La forma apre la via.",
                "Gesti vuoti, pronti per nessuno. Ripeti senza credere, il cerchio si spezza.",
                &["rito", "disciplina", "potere"],
                [160, 90, 180],
                r#"
      ▲
     ╱ ╲
    ╱   ╲
   •─────•
    ╲   ╱
     ╲ ╱
      •
"#,
            ),
            card(
                "Lo Spettro", "IV",
                "Qualcuno insiste a volerti avvisare. Il passato pesa ancora un po'.",
                "Un'ombra diventa ossessione. Ti mordi il cuore su chi non è più.",
                &["spirito", "messaggio", "passato"],
                [120, 160, 190],
                r#"
    .─────.
   ( ░ ░ ░ )
   (  ░░░  )
   ( ░ ░ ░ )
   (       )
   (  ▄ ▄  )
   (  ▄ ▄  )
    '─' '─'
"#,
            ),
            card(
                "Il Sepolcro", "V",
                "Un peso che chiudi alle spalle. La terra custodisce ciò che va custodito.",
                "Riapri la fossa per guardare il dolore. Certi morti vogliono restare.",
                &["chiusura", "custodia", "lutto"],
                [120, 120, 130],
                r#"
    ╭──────╮
    │ R.I.P│
    │  ▄▄  │
    │ ▄▄▄▄ │
    │▄▄▄▄▄▄│
    ────────
"#,
            ),
            card(
                "Il Verme", "VI",
                "Lavora nel buio senza fretta. La decomposizione è una forma di cura.",
                "Ti riduci nel marciume che hai evitato. Qualcosa ti mangia da dentro.",
                &["decadimento", "pazienza", "ciclo"],
                [150, 120, 90],
                r#"
   ╭╮  ╭╮
   ││  ││
   ╰╯  ╰╯
   ╭╮  ╭╮
   │╰──╯│
   ╰────╯
"#,
            ),
            card(
                "La Falce", "VII",
                "Taglia ciò che ha fatto il suo tempo, senza rancore. Mietere è giustizia.",
                "Una lama che affini contro chi amavi. Il distacco diventa ferita.",
                &["taglio", "giustizia", "fine"],
                [200, 60, 60],
                r#"
     ╲▔▔▔▔
      ╲
   ▓▓▓▓╲
   ▓▓▓▓ │
   ▓▓▓▓ │
   ▓▓▓▓ │
   ▓▓▓▓ │
       ┘
"#,
            ),
            card(
                "L'Ossario", "VIII",
                "Molte vite accatastate, una sola storia. Il tuo nome è una riga breve ma tua.",
                "Ti misuri con i morti e perdi. Nessuno vince quel numero.",
                &["collettivo", "mortalità", "residuo"],
                [180, 180, 150],
                r#"
   .░░░░.
   ( · · )
   (  ___ )
   .░░░░.
   ( · · )
   (  ___ )
    ░░░░░
"#,
            ),
            card(
                "Il Velo", "IX",
                "Tra te e il resto c'è un tessuto fitto. Impara a guardare attraverso.",
                "Il velo è diventato muro. Tutto arriva attutito, nessuna voce ti raggiunge.",
                &["limite", "visione", "separazione"],
                [110, 90, 150],
                r#"
   ╲██╱
   ╲▓▓╱
   ▒▓·▓▒
   ▓▒·▒▓
   ▒▓·▓▒
   ░·░·░
"#,
            ),
            card(
                "Il Ritorno", "X",
                "Poco muore per sempre: quasi nulla non torna. Chi ti manca può raggiungerti da altrove.",
                "Chi torna può essere ossessione. Distingui ciò che serve da ciò che possiede.",
                &["ritorno", "risveglio", "presenza"],
                [140, 200, 140],
                r#"
       ╱▔╲
      ╱   ╲
       ╲ ╱
        ▲
     ▄▄▄▄▄▄▄
    ░░ ░ ░ ░░
    ░░░░░░░░░
"#,
            ),
            card(
                "La Necropoli", "XII",
                "Una città sotto la città. Sotto i tuoi piedi pulsa un'altra luce, e qualcosa ti riconosce.",
                "Cammini fra le case dei morti senza bussare, e la memoria si perde dietro di te.",
                &["città", "radici", "memoria"],
                [90, 90, 120],
                r#"
   ╱▔▔▔▔▔╲
  │       │
  │   ◍   │
  │  . .  │
  │   │   │
  ╰───┴───╯
   ░░░░░░░░░
"#,
            ),
        ],
    }
}

/// "Athanor" — alchimia. La Grande Opera: Nigredo, Albedo, Rubedo.
pub fn athanor() -> Deck {
    Deck {
        name: "Athanor".to_string(),
        description: "La Grande Opera: Nigredo, Albedo, Rubedo. Trasforma il piombo in oro, e te stesso con esso.".to_string(),
        reversed_chance: 0.35,
        cards: vec![
            card(
                "La Materia Prima", "0",
                "Nel caos iniziale dorme ogni forma. Non è spazzatura: è promessa non ancora cotta.",
                "Ti fossilizzi nel caos. La materia prima resta solo materia.",
                &["caos", "materia", "potenza"],
                [110, 90, 70],
                r#"
    ░▒▓▓▒
    ▒▓◍▓▒
    ▓▓▓▓▓
   ░▓▓▓▓░
    ░▒▓▓▒░
"#,
            ),
            card(
                "Il Mercurio", "I",
                "Tutto può cambiare posto e nome. Mente agile, corpo che si scioglie e si adatta.",
                "Scivoli tra le mani di tutti. Essere ovunque è non essere da nessuna parte.",
                &["trasformazione", "spirito", "scambio"],
                [190, 200, 210],
                r#"
       ☿
      ╱ ╲
     (   )
      ╲ ╱
       ▽
     ≈≈≈≈≈
"#,
            ),
            card(
                "Lo Zolfo", "II",
                "Fuoco interiore, voglia che non si spegne. Brucia per ciò che vuoi.",
                "Ardi e consumi tutto intorno. Un incendio che non trasforma, solo distrugge.",
                &["fuoco", "desiderio", "anima"],
                [230, 180, 60],
                r#"
       ▲
      ▲ ▲
     ▲ ▲ ▲
     ▔▔▔▔▔
     ▓▓▓▓▓
"#,
            ),
            card(
                "Il Sale", "III",
                "La terra del corpo tiene tutto insieme. Staticità che dà forma.",
                "Sei così fermo da diventare terreno. Il sale indurisce il corpo e spegne l'anima.",
                &["corpo", "stabilità", "forma"],
                [225, 225, 225],
                r#"
      ▄▄▄
     ▄████▄
     █▐██▌█
     █▐██▌█
     ▀████▀
      ▀▀▀
"#,
            ),
            card(
                "La Nigredo", "IV",
                "Sciogliersi nel buio è la prima tappa. Il nero non è perdita: è materia che cuoce.",
                "Resti nel nero molto oltre la soglia. Il buio diventa abitudine, non passaggio.",
                &["notte", "dissoluzione", "inizio"],
                [40, 35, 45],
                r#"
      █████
     ██░░░██
     █░░▉░░█
     ██░░░██
      █████
"#,
            ),
            card(
                "L'Albedo", "V",
                "Dopo il nero, il chiaro ricuce i contorni. Ciò che è tuo ricomincia a vedere la luce.",
                "Una bianchezza puntata, un candore che confonde. Troppa luce e perdi ogni ombra.",
                &["chiarezza", "purificazione", "luna"],
                [225, 230, 240],
                r#"
       ░░░
      ░▄▄▄░
      ░████░
      ░▀▀▀░
       ░░░
"#,
            ),
            card(
                "La Rubedo", "VI",
                "Rosso del sangue riscattato. L'ultima tintura: il mondo si tinge di te.",
                "Il compimento si blocca sulla vetta. L'opera è pronta ma tu esiti a firmarla.",
                &["rosso", "culmine", "vita"],
                [220, 60, 70],
                r#"
       ◯
     ◯▄▄▄◯
     ▐███▌
     ◯▀▀▀◯
       ◯
"#,
            ),
            card(
                "Il Crogiolo", "VII",
                "Il recipiente che ospita il tuo fuoco. Le mani che reggono la trasformazione.",
                "Manca il fuoco o manca la cura. Bruci nel crogiolo senza diventare oro.",
                &["crogiuolo", "fusione", "pazienza"],
                [220, 130, 50],
                r#"
       ( )
      ▄▄▄▄▄
     ███████
     ▐█████▌
     ▐█████▌
      ▀▀▀▀▀
      ░░░░░
"#,
            ),
            card(
                "L'Athanor", "VIII",
                "Il forno che non si spegne. Il calore che tiene acceso fino alla soglia.",
                "Ti separi dal fuoco. L'opera si raffredda e la porta resta a metà.",
                &["forno", "costanza", "calore"],
                [160, 120, 80],
                r#"
       ▄▄▄
      ▄████▄
      ▐███▌
      ▐███▌
       ▐█▌
       ████
      ██████
"#,
            ),
            card(
                "La Trasmutazione", "IX",
                "Il piombo che diventa oro: non con un colpo solo, ma con mille ore di pazienza.",
                "L'oro torna piombo. Trasformarsi è un passo indietro e tutto da rifare.",
                &["trasmutazione", "opera", "tempo"],
                [240, 200, 90],
                r#"
      ▀▀▀▀▀
     ███████
     █▄ █ ▄█
     ███████
      ▂▂▂▂▂
       ✦
"#,
            ),
            card(
                "L'Elisir", "X",
                "Una goccia di ciò che cercavi, già nella tua tazza. Sorseggia e bevi con attenzione.",
                "L'elisir è veleno se lo bevi di fretta. Curarsi male è peggio di non curarsi.",
                &["guarigione", "concentrato", "attesa"],
                [140, 220, 180],
                r#"
       .▄.
      ▄███▄
      ▐███▌
       ▐█▌
        ▐▌
        ·
"#,
            ),
            card(
                "La Pietra", "XI",
                "Il centro fermo dentro ogni grido. Un punto che non teme fuoco né martello.",
                "Credi di avere la pietra ma stringi solo il calderone. Tieni un sasso nel palmo chiuso.",
                &["centro", "tesoro", "stabilità"],
                [230, 220, 120],
                r#"
        ▄▄▄
      ▄█████▄
     ▐███████▌
     ▐███████▌
      ▀█████▀
        ▀▀▀
"#,
            ),
            card(
                "Il Sigillo", "XII",
                "Un cerchio che si chiude su ciò che ami. Ciò che è custodito, resta.",
                "Sigillo rotto, la casa non chiude: ciò che tenevi chiuso gira libero.",
                &["protezione", "legame", "custodia"],
                [180, 140, 220],
                r#"
       ▲
      ╱ ╲
     ╱   ╲
    ●  ╳  ●
     ╲   ╱
      ╲ ╱
       ▼
"#,
            ),
            card(
                "La Grande Opera", "XIII",
                "Tutte le tinture si congiungono: ciò che era rotto si ricuce. L'opera è compiuta.",
                "La grande opera resta aperta a metà. Manca una mano che voglia chiuderla.",
                &["completezza", "gloria", "sigillo"],
                [230, 190, 80],
                r#"
      ◍───◍
     ╱     ╲
    ◉  ◐◑  ◉
     ╲     ╱
      ◍───◍
"#,
            ),
        ],
    }
}

/// "La Soglia" — sogni, specchi e luoghi di passaggio. Molto suggestivo.
pub fn la_soglia() -> Deck {
    Deck {
        name: "La Soglia".to_string(),
        description: "Oracolo delle stanze fra le stanze: sogni, specchi e porte che non si chiudono mai.".to_string(),
        reversed_chance: 0.4,
        cards: vec![
            card(
                "Il Sonno", "0",
                "Sprofondi nella coscienza a strati. Di là, il sogno sa già dove andare.",
                "Rifiuti il sonno. La stanchezza resta a galleggiare davanti alla soglia.",
                &["sogno", "abbandono", "notte"],
                [130, 120, 200],
                r#"
       ☾
      ╱▔▔╲
     ( ◐  )
      ╲▁▁╱
      ▂▂▂▂
"#,
            ),
            card(
                "Lo Specchio", "I",
                "Ciò che vedi laggiù non è un estraneo: guarda meglio. È il tuo doppio che aspetta.",
                "Lo specchio ti rimanda, ma tu guardi solo ciò che manca. Scappi da una parte di te.",
                &["riflesso", "identità", "verità"],
                [170, 190, 220],
                r#"
    ╭─────╮
    │ ◐ ◑ │
    │     │
    │ ◑ ◐ │
    ╰─────╯
"#,
            ),
            card(
                "La Porta", "II",
                "Si apre senza rumore e non chiede nulla. È la risposta che stai evitando.",
                "La porta si è chiusa alle tue spalle e non ricordi più da che parte venivi.",
                &["passaggio", "scelta", "ingresso"],
                [150, 110, 190],
                r#"
    ╭─────╮
    │  ◑  │
    │  │  │
    │  │  │
    ╰──┼──╯
    ▄▄▄▄▄▄▄
"#,
            ),
            card(
                "La Pioggia", "III",
                "Pulisce ciò che non sai nominare. Lascia che il vecchio colore scenda a terra.",
                "Pioggia che non arriva. L'aria resta carica di ciò che avresti voluto lavare.",
                &["pulizia", "pianto", "rinnovo"],
                [100, 150, 200],
                r#"
    ░░ ░  ░
     ░ ░░ ░
    ░  ░ ░░
     ░░ ░  ░
    ░  ░░ ░
"#,
            ),
            card(
                "La Cenere", "IV",
                "Quello che resta dopo l'incendio è ancora tuo. La cenere ricorda e feconda.",
                "Soffi sulla cenere e la rimetti in aria. Il passato ti si appiccica agli occhi.",
                &["residuo", "memoria", "terreno"],
                [150, 150, 150],
                r#"
     ▂▂▂▂▂
     ▓▓  ▓▓
     ▓    ▓
     ░    ░
     ░░ ░░
"#,
            ),
            card(
                "L'Eco", "V",
                "Qualcosa che hai detto ti torna, ma diverso. Ascolta bene: è l'altro lato di te.",
                "L'eco si moltiplica e copre tutto. Non distingui più la tua voce dalle altre.",
                &["ritorno", "risonanza", "voce"],
                [120, 200, 180],
                r#"
    ◉ ⟩ ⟨ ◉
      ◉ ⟩ ⟨
        ◉ ⟩
         ◉
      ◉ ⟩ ◉
"#,
            ),
            card(
                "La Nebbia", "VI",
                "I contorni sono sospesi: niente è più evidente. Cammina piano, fidati del passo.",
                "La nebbia ti ingoia. Ogni strada sembra la stessa e non sai più dove sei.",
                &["incertezza", "velo", "silenzio"],
                [110, 120, 150],
                r#"
     ░▒▒░▒
     ▒░▒▒░
     ░▒░▒▒
     ▒▒░▒░
     ░▒▒░▒
"#,
            ),
            card(
                "Il Lampione", "VII",
                "Un solo punto di luce in una strada vuota. Non devi vedere tutto: basta il prossimo passo.",
                "Il lampione sfarfalla e si spegne. Resti al buio con domande che erano già risposte.",
                &["guida", "isolamento", "luce"],
                [240, 200, 120],
                r#"
        ✧
       (◎)
       ─┼─
        │
        │
      ░░░░░
"#,
            ),
            card(
                "Il Corridoio", "VIII",
                "Stai attraversando qualcosa che non è un luogo. Non è deserto: è transito.",
                "Il corridoio si allunga ogni volta che guardi. Cammini ma il fondo si sposta.",
                &["attraversamento", "attesa", "luogo"],
                [90, 100, 130],
                r#"
    █▌·▐█
    █▌·▐█
    █▌·▐█
    █▌·▐█
    ██·██
"#,
            ),
            card(
                "La Scalinata", "IX",
                "Sali per gradini che non sai dove portano. Ogni piano cambia la casa.",
                "Scendi gli stessi gradini e riconosci tutto: forse salivi nel posto sbagliato.",
                &["salita", "livelli", "percorso"],
                [140, 140, 180],
                r#"
    ──▄▄──
    ──▐▐──
    ▄▄▐▐▄▄
    ▐▐▐▐▐▐
    ▐▐▐▐▐▐
"#,
            ),
            card(
                "La Finestra", "X",
                "Una luce filtrata da vetro spesso. Il mondo fuori esiste, ma non è ancora tuo.",
                "Guardi fuori senza aprire. La finestra è uno schermo, e la vita scorre altrove.",
                &["sguardo", "separazione", "luce"],
                [160, 190, 230],
                r#"
    ╭────╮
    │ ◐  │
    │  ◑ │
    ╰────╯
     ·  ·
"#,
            ),
            card(
                "La Lanterna", "XI",
                "Porti la luce con te, piccola ma continua. Basta a non inciampare.",
                "La lanterna è accesa ma la tieni spenta per non consumare. Risparmi la luce e perdi la via.",
                &["portatore", "luce", "viaggio"],
                [230, 170, 90],
                r#"
      ▄▄▄
     █ ● █
     █ █ █
     ▀▀▀▀▀
       ██
"#,
            ),
            card(
                "L'Alba", "XII",
                "La notte del sogno si schiarisce da sola. Torni, e il mondo è quasi lo stesso.",
                "Ti svegli a metà: un piede nel sogno, uno nel giorno. Nessuna luce vince del tutto.",
                &["risveglio", "ritorno", "luce"],
                [240, 180, 130],
                r#"
       ◍
      ◑◉◐
     █████
    ░█████░
    ░░░░░░░
"#,
            ),
        ],
    }
}

/// Tutti i mazzi incorporati.
pub fn builtin_decks() -> Vec<Deck> {
    vec![
        void_arcana(),
        neon_oracle(),
        mors_initium(),
        athanor(),
        la_soglia(),
    ]
}

/// Serializza un mazzo in TOML (per l'export/salvataggio dei mazzi editati).
pub fn to_toml(deck: &Deck) -> Result<String, String> {
    toml::to_string_pretty(deck).map_err(|e| e.to_string())
}

/// Deserializza un mazzo da TOML, validando che non sia vuoto.
pub fn from_toml(text: &str) -> Result<Deck, String> {
    let deck: Deck = toml::from_str(text).map_err(|e| e.message().to_string())?;
    if deck.cards.is_empty() {
        return Err("il mazzo non contiene carte".into());
    }
    Ok(deck)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builtin_decks_are_non_empty_and_well_formed() {
        for deck in builtin_decks() {
            assert!(!deck.cards.is_empty(), "{} è vuoto", deck.name);
            for c in &deck.cards {
                assert!(!c.name.is_empty());
                assert!(!c.upright.is_empty(), "{} senza dritto", c.name);
                assert!(!c.reversed.is_empty(), "{} senza rovescio", c.name);
                assert!(!c.symbol_lines().is_empty(), "{} senza simbolo", c.name);
            }
        }
    }

    #[test]
    fn deck_survives_a_toml_roundtrip() {
        let deck = void_arcana();
        let text = to_toml(&deck).unwrap();
        let back = from_toml(&text).unwrap();
        assert_eq!(back.name, deck.name);
        assert_eq!(back.cards.len(), deck.cards.len());
        assert_eq!(back.cards[0].name, deck.cards[0].name);
    }

    #[test]
    fn all_builtin_decks_roundtrip_individually() {
        for deck in builtin_decks() {
            let back = from_toml(&to_toml(&deck).unwrap()).unwrap();
            assert_eq!(back.name, deck.name);
            assert_eq!(back.cards.len(), deck.cards.len());
        }
    }

    #[test]
    fn builtin_set_is_complete() {
        let names: Vec<String> = builtin_decks().iter().map(|d| d.name.clone()).collect();
        for expected in ["Void Arcana", "Neon Oracle", "Mors Initium", "Athanor", "La Soglia"] {
            assert!(names.iter().any(|n| n == expected), "manca il mazzo {expected}");
        }
    }

    #[test]
    fn empty_deck_is_rejected() {
        let toml = "name = \"Vuoto\"\nreversed_chance = 0.4\ncards = []\n";
        assert!(from_toml(toml).is_err());
    }

    #[test]
    fn symbols_are_reasonably_sized() {
        for deck in builtin_decks() {
            for c in &deck.cards {
                let lines = c.symbol_lines();
                assert!(lines.len() <= 9, "{} troppo alto", c.name);
                assert!(lines.iter().all(|l| l.chars().count() <= 20), "{} troppo largo", c.name);
            }
        }
    }
}
