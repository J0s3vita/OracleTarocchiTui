# Cyberpunk Tarot / Oracle Deck TUI — Project Spec

> Mazzo di tarocchi/oracoli generativo con estetica dark gotica esoterica, letture giornaliere da terminale, pensato anche come strumento di content creation.

---

## Concept

Un TUI che pesca carte da un mazzo personalizzato (tarocchi classici reinterpretati o un oracolo originale) mostrando il simbolo della carta come ASCII art con animazione di rivelazione, pensato sia per uso personale che come generatore di contenuti social dark surrealist.

## Comando di lancio
 oracledecktui

## Stack consigliato

- **Rust** + `ratatui` + `crossterm` (coerenza con SYNTUI).
- Mazzo definito in `TOML`/`JSON`: nome carta, significato (dritto/rovesciato), simbolo ASCII, tema colore associato.
- `serde`/`serde_json` per parsing mazzo, `rand` per pescaggio carte.
- Export lettura come testo/immagine: possibile generazione di uno screenshot ANSI-to-image per condivisione (fase avanzata).

## Feature — Priorità Alta

- [x] **Pescaggio carta giornaliera**: una carta al giorno, con salvataggio locale per evitare pescaggi multipli nello stesso giorno.
- [x] **Mazzo personalizzato editabile**: file di configurazione con le carte del proprio oracolo (nome, significato, simbolo ASCII, palette), non limitato ai 22/78 arcani classici.
- [x] **Lettura a più carte**: stesura "passato/presente/futuro" o simile, con 3 carte disposte orizzontalmente.
- [x] **Significato dritto/rovesciato**: la carta pescata può uscire invertita, con testo di significato diverso.
- [x] **Cronologia letture**: log locale delle letture passate con data, consultabile in un pannello dedicato.

## Feature — Priorità Media

- [x] **Editor mazzo integrato**: interfaccia per creare/modificare carte senza editare manualmente il file TOML.
- [x] **Temi di mazzo multipli**: possibilità di avere più mazzi (es. "Void Arcana", "Neon Oracle") selezionabili da menu.
- [x] **Note personali per lettura**: possibilità di annotare una riflessione testuale legata alla lettura del giorno.
- [x] **Export testo per social**: generazione di un blocco di testo formattato (carta + significato) pronto da copiare per una caption Instagram.

## Estetica Cyberpunk & Animazioni

- [x] **Shuffle animato**: sequenza di "mescolamento" carte con simboli che scorrono rapidamente prima di fermarsi sulla carta pescata.
- [x] **Reveal con glitch**: il simbolo ASCII della carta si "stabilizza" gradualmente da rumore/glitch a forma definita, invece di apparire istantaneamente.
- [x] **Palette dinamica per carta**: ogni carta può avere un colore dominante associato, che tinge temporaneamente i bordi dell'interfaccia durante la visualizzazione.
- [x] **Effetto "carta rovesciata"**: quando la carta esce invertita, rotazione/flip animato del simbolo ASCII a schermo.
- [x] **Ambientazione**: sfondo con particellare lento (stile rain effect già pensato per SYNTUI/Matrix) per un'atmosfera rituale/contemplativa.

## Roadmap

| Fase | Focus |
|---|---|
| 1 | Mazzo base da file config + pescaggio giornaliero |
| 2 | Lettura a più carte + cronologia |
| 3 | Animazioni shuffle/reveal/glitch |
| 4 | Editor mazzo integrato + export social |

## Note tecniche

- Il mazzo va progettato come dato esterno (TOML) fin dall'inizio, per poter creare facilmente varianti tematiche senza ricompilare.
- L'export "pronto per social" è la feature che collega più direttamente il tool al tuo lavoro di content creation dark surrealist — merita priorità alta nonostante sia elencata come media, se l'obiettivo primario è la produzione di contenuti.
