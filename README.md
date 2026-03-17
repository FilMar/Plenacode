# SpellCode — Sistema Magico

## Idea Centrale

SpellCode è un DSL (Domain Specific Language) per la magia, strutturato come un **linguaggio a tre livelli**: simboli, parole, frasi. Il sistema funziona come un **interprete sequenziale** — ogni parola legge il contesto mondiale corrente, lo modifica, e passa alla parola successiva. Il mondo stesso è lo stato della spell.

```
Frase: ["Fuoco", "Palla", "Lancia"]

"Fuoco" → esegue → modifica WorldContext →
"Palla" → legge WorldContext → modifica WorldContext →
"Lancia" → legge WorldContext → effetto finale
```

Poiché ogni parola è un'**azione separata nel turno**, la spell può essere interrotta, rubata o sabotata da altri attori.

---

## I Tre Livelli

### Livello 1 — Simboli (Lettere)

I simboli sono le unità atomiche del sistema. Sono il **motore** — non cambiano mai tra un gioco e l'altro.

Proprietà fondamentali:
- Tutti hanno lo **stesso peso sintattico** — nessuna categoria predefinita (non esistono "elementi", "forze", "vettori")
- **L'ordine conta**: `[Heat, Energy]` ≠ `[Energy, Heat]`
- Sono **token omogenei** che si trasformano su uno stack
- Il giocatore di solito **non li vede** — esistono sotto il cofano

```
Heat, Cold, Energy, Pressure, Up, Down, Wall, Point,
Void, Life, Death, Mind, Time, Direction, Self, ...
```

---

### Livello 2 — Parole (Morfemi)

Le parole sono **combinazioni fisse di simboli** con un significato stabile. Sono il **vocabolario del gioco** — il game designer le definisce, il giocatore le impara e le usa.

L'ordine dei simboli dentro una parola conta:

```
Heat + Energy + Up    =  "Fuoco"    (calore che sale, espansivo)
Heat + Energy + Down  =  "Brace"    (calore soffocato, lento)
Cold + Energy + Up    =  "Gelo"     (freddo cristallizzato)
Cold + Energy + Down  =  "Nebbia"   (freddo diffuso, basso)
Earth + Pressure + Wall = "Pietra"
Void + Self + Time    =  "Ombra"
```

Le parole sono il confine tra **motore** (simboli) e **gioco** (frasi). Il designer le definisce in un file di configurazione:

```ron
// words.ron
words: [
    Word("Fuoco",   [Heat, Energy, Up]),
    Word("Brace",   [Heat, Energy, Down]),
    Word("Gelo",    [Cold, Energy, Up]),
    Word("Pietra",  [Earth, Pressure, Wall]),
    Word("Ombra",   [Void, Self, Time]),
]
```

---

### Livello 3 — Frasi (Spell)

Le frasi sono **combinazioni di parole** che il giocatore assembla per creare magie. Sono il livello che il giocatore vive direttamente.

L'ordine delle parole nella frase conta:

```
"Fuoco" + "Sfera" + "Direzione"   →  palla di fuoco verso un bersaglio
"Fuoco" + "Muro"  + "Vicino"      →  muro di fuoco intorno al caster
"Gelo"  + "Sfera" + "Direzione"   →  palla di ghiaccio
"Fuoco" + "Gelo"  + "Esplosione"  →  vapore esplosivo (effetto emergente)
"Ombra" + "Sfera" + "Direzione"   →  proiettile che oscura la visione
```

Le frasi **non sono predefinite** dal designer — emergono dalla combinazione delle parole disponibili. Il designer controlla solo quali parole esistono nel gioco.

---

## La Pipeline — Interprete Sequenziale

Non esiste una fase di compilazione separata. La frase viene eseguita **parola per parola**, e il contesto mondiale è lo stato condiviso:

```rust
pub fn cast(words: &[Word], ctx: &mut WorldContext) {
    for word in words {
        word.execute(ctx);  // ctx si aggiorna in place
    }
}
```

Esempio concreto:

```
"Fuoco"  → legge ctx → genera energia termica intorno al caster
            ctx ora contiene: { energia_libera: Fire, posizione: caster }

"Palla"  → legge ctx → raccoglie l'energia libera, la comprime in sfera
            ctx ora contiene: { forma: Sfera(Fire), pronta: true }

"Lancia" → legge ctx → proietta la sfera secondo i simboli di "Lancia"
            ctx ora contiene: { proiettile: in_volo, target: risolto }
```

Ogni parola vede il mondo **già modificato** dalle parole precedenti. Non c'è bytecode intermedio — il mondo stesso è lo stato della spell.

### Contesto come Sorgente di Verità

Nessun valore numerico è hardcodato. Tutto viene estratto dal contesto:

| Cosa | Sorgente |
|------|----------|
| Intensità danno | Stats del caster |
| Bersaglio | NPC/creature vicine, relazioni, fazione |
| Energia disponibile | Effetti delle parole precedenti + ambiente |
| Effetti ambientali | Tile infiammabili, acqua, vento |
| Effetti meteo | Pioggia spegne fuoco, neve rallenta |
| Side effects | Combinazione contestuale di tutto il sopra |

La potenza emerge dall'**attore e dal mondo**, non dalla spell.

---

## Esecuzione Distribuita nel Turno

Poiché ogni parola è un'azione separata, una frase lunga occupa **più turni**:

```
Turno 1: caster esegue "Fuoco"   → energia termica generata nel mondo
Turno 2: caster esegue "Palla"   → comprime l'energia in sfera
Turno 3: caster esegue "Lancia"  → proietta la sfera
```

Questo apre meccaniche di gioco profonde:

**Interruzione** — il caster viene colpito al turno 2. L'energia termica è già nel mondo, libera e pericolosa per tutti vicini.

**Furto** — un altro mago esegue "Palla" al turno 2 prima del caster. "Palla" non appartiene a nessuno — raccoglie l'energia più vicina disponibile nel contesto. Il caster originale perde il materiale.

**Sabotaggio** — lanci "Gelo" mentre il nemico ha appena eseguito "Fuoco". Le energie si neutralizzano, o interagiscono in modo imprevedibile (vapore, esplosione).

**Completamento ostile** — il nemico sta caricando qualcosa di pericoloso, tu esegui "Lancia" prima di lui completando la sua spell nella direzione sbagliata.

**Spell corta vs lunga** — una parola sola è sicura ma debole. Tre parole sono potenti ma vulnerabili per tre turni. Il giocatore sceglie il rischio.

```
Rischio   ████████░░  Potenza   ████████░░
Turni: 3  ██████████  Turni: 1  ██░░░░░░░░
```

---

## Chi Controlla Cosa

```
Motore (magic_core)     → definisce i Simboli e le regole combinatorie
                          non cambia mai tra un gioco e l'altro

Game Designer           → definisce le Parole disponibili nel gioco
                          sceglie il vocabolario, il tono magico del mondo

Giocatore               → compone Frasi dalle Parole che ha imparato
                          scopre combinazioni, impara contestualmente
```

Cambiare solo le parole produce sistemi magici con feeling completamente diverso:

```
Gioco fantasy classico  →  Fuoco, Acqua, Terra, Aria, Luce, Ombra
Gioco horror cosmico    →  Vuoto, Eco, Soglia, Peso, Assenza, Riflesso
Gioco dark fantasy      →  Sangue, Legame, Rottura, Destino, Carne
```

Stessa codebase. Zero codice nuovo.

---

## Progressione del Giocatore

```
Principiante   → impara parole singole come ricette
                 "Fuoco + Sfera = qualcosa brucia"

Intermedio     → capisce come le parole si combinano
                 inizia a sperimentare ordini diversi

Esperto        → trova combinazioni non documentate
                 scopre effetti emergenti contestuali

Maestro        → deduce parole rare per inferenza
                 usa il sistema in modi che nessun NPC conosce
```

Un mago nemico anziano ha accesso a più parole e le combina in modo più sofisticato. Il giocatore può **scoprire combinazioni osservando i nemici**.

---

## Architettura del Codice

```
magic_core              ← interprete puro, zero dipendenze da Bevy
    symbols.rs          ← Symbol, tutti omogenei, il motore immutabile
    words.rs            ← Word, combinazioni di simboli, caricabili da config
    interpreter.rs      ← esegue ogni Word su WorldContext
    context.rs          ← WorldContext, ContextActor, WeatherState

magic_bevy              ← bridge con Bevy ECS
    components.rs       ← SpellBuffer as Component (parole in coda)
    systems.rs          ← consuma una parola per turno, aggiorna WorldContext
    events.rs           ← WordCast, SpellInterrupted, SpellStolen
```

`magic_core` non sa nulla di Entity, Component, o World. Riceve un `WorldContext`, lo modifica, lo restituisce. `magic_bevy` traduce tra Bevy ECS e `WorldContext`.

---

## Testing

La separazione da Bevy permette test veloci e puliti:

**Unit test** — ogni parola modifica il contesto come atteso:
```rust
let mut ctx = WorldContext::empty();
word_fuoco.execute(&mut ctx);
assert!(ctx.has_free_energy(EnergyType::Fire));

word_palla.execute(&mut ctx);
assert!(ctx.has_shape(Shape::Sphere));
assert!(!ctx.has_free_energy(EnergyType::Fire)); // consumata
```

**Test di interazione** — furto e interruzione:
```rust
// mago A esegue "Fuoco", mago B ruba con "Palla"
let mut ctx = WorldContext::with_two_casters(mago_a, mago_b);
word_fuoco.execute_as(&mut ctx, mago_a);
word_palla.execute_as(&mut ctx, mago_b);  // ruba l'energia di A
assert!(ctx.spell_owner() == mago_b);
```

**Property test** (proptest) — invarianti del sistema:
```
- qualsiasi sequenza valida non crasha mai
- stessa parola, contesti diversi → risultati diversi
- parola sconosciuta degradata gracefully a simboli grezzi
- energia libera nel mondo è sempre in stato consistente
```

**Simulation test** — fai girare il sistema per N turni, verifica che il mondo non diverga.
