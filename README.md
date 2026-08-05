# Oracle Deck TUI

Un mazzo di tarocchi/oracoli cyberpunk da terminale,
carta del giorno, stese a più carte, mazzi personalizzabili e simboli ASCII con
animazione di rivelazione a glitch. Pensato anche come generatore di contenuti.

```
        ╔════════════════════════════════════════════════╗
        ║                VIII  The Balance               ║
        ║                  ⟲ rovesciata                  ║
        ║                                                ║
        ║                    ═══╧═══                     ║
        ║                     │                          ║
        ║                   ▢   │   ▢                    ║
        ║                    ╱  │  ╲                     ║
        ║                    ───┼───                     ║
        ║                     ▲                          ║
        ╚════════════════════════════════════════════════╝
```

## Installazione

```bash
cargo install --path .
```

Poi:

```bash
oracledecktui
```

## Cosa fa

- **Carta del giorno** — una pescata al giorno, salvata localmente: se riapri
  l'app lo stesso giorno rivedi la stessa carta, non se ne pesca un'altra.
- **Stesura a tre carte** — Passato · Presente · Futuro, che si rivelano in
  sequenza.
- **Dritto / rovesciato** — ogni carta può uscire invertita, con un significato
  diverso e il simbolo ASCII capovolto.
- **Cronologia** — log di tutte le letture, riapribili ed esportabili.
- **Note personali** — annota una riflessione legata a una lettura.
- **Export per social** — blocco di testo (carta + significato + hashtag dalle
  parole chiave) pronto da incollare come caption.
- **Editor mazzo** — crea/modifica carte (nome, significati, colore, ASCII art)
  senza toccare il TOML a mano; i mazzi si salvano su disco.
- **Mazzi multipli** — due mazzi inclusi ("Void Arcana", "Neon Oracle") più i
  tuoi; si scelgono con `d`.
- **Elimina mazzo** — dal picker (`d`), `d` elimina (con conferma) un mazzo
  personalizzato e il suo file; gli incorporati non si cancellano.
- **Cancella cronologia** — dalla vista Cronologia, `X` svuota tutto (con
  conferma).
- **Navigazione veloce** nelle liste — `↑↓`/`jk`, `PagSu/PagGiù`/`ug`, `Home/End`.`

## Animazioni & estetica

- **Shuffle** — glifi che scorrono rapidi prima di fermarsi sulla carta.
- **Reveal a glitch** — il simbolo si stabilizza da rumore a forma definita,
  dall'alto verso il basso.
- **Palette dinamica** — il bordo della carta si tinge del colore della carta.
- **Carta rovesciata** — il simbolo ASCII viene capovolto e specchiato.
- **Sfondo particellare** — glifi che scendono piano, per un'atmosfera rituale.
- **Palette** — Ritual (viola/ambra), Blood Moon, Abyss; si cambiano con `t`.

## Tasti

**Globali** — `1`-`5` / `Tab` cambia vista · `d` scegli mazzo · `N` crea un
nuovo mazzo · `t` palette · `q` esci.

**Pescaggio** — `Invio` pesca · `Spazio` salta l'animazione · `n` nota · `e`
esporta per social.

**Cronologia** — `↑↓` scrolla · `Invio` riapri · `e` esporta · `X` cancella
tutto. In lista: `PagSu/PagGiù` o `u/g` e `Home/End` per spostarsi in fretta.

**Editor** — `n`/`e`/`x` nuova/modifica/elimina carta · `N` nuovo mazzo · `s`
salva su disco. Nei form multilinea (ASCII art): `Invio` va a capo, `Ctrl+S`
salva.

## Il mazzo come dato esterno

I mazzi sono file TOML in `~/.oracledecktui/decks/`, così puoi creare varianti
tematiche senza ricompilare. Un mazzo personalizzato con lo stesso nome di uno
incorporato lo sostituisce. La cronologia sta in `history.json`. `ORACLE_DIR`
sposta la cartella; `oracledecktui --path` la stampa.

Struttura di una carta nel TOML:

```toml
[[cards]]
name = "The Null"
number = "0"
upright = "Un ciclo si apre nel vuoto. Potenziale puro."
reversed = "Paralisi, paura del salto."
keywords = ["inizio", "vuoto", "potenziale"]
color = [150, 130, 210]
symbol = """
   .-=-.
  /     \\
 |  ( )  |
  \\     /
   '-=-'
"""
```

## Architettura

| File | Contenuto |
|---|---|
| `rng` | PRNG SplitMix64 seedabile (riproducibile nei test) |
| `model` | carte, mazzo, letture, cronologia |
| `deck` | mazzi incorporati + load/save TOML |
| `store` | cartella dati, mazzi, cronologia, pescata del giorno |
| `draw` | logica di estrazione (carte distinte, orientamento) |
| `export` | testo per social |
| `theme` | palette + tinta per carta |
| `ui/card` | rendering carta: shuffle, reveal a glitch, flip rovesciato |
| `ui/particles` | sfondo particellare |
| `ui/views` | giornaliera, stesura, cronologia, editor, aiuto |

## Sviluppo

```bash
cargo test
```

Copre rng, draw, mazzi, storage, export e i flussi via tastiera. Le viste hanno
test di render su `TestBackend`; per guardarle a occhio:

```bash
cargo test dump_screens -- --ignored --nocapture
```
