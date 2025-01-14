# Descrizione datasets
Per testare gli algoritmi, sono stati selezioni diversi datasets, sia grafi reali che sintetici.

| Name            | Nodes    | Edges       | Avg Degree
| ----            | ----:    | ----:       | ---- 
| syn_50k_50k     | 50_000   | 50_000      | 2.31
| syn_50k_100k    | 50_000   | 100_000     | 4.01
| syn_50k_200k    | 50_000   | 200_000     | 8.03
| syn_50k_500k    | 50_000   | 500_000     | 20
| syn_50k_1M      | 50_000   | 1_000_000   | 40
| syn_50k_1,5M    | 50_000   | 1_500_000   | 60
| syn_50k_2M      | 50_000   | 2_000_000   | 80
| syn_50k_1M      | 50_000   | 1_000_000   | 40
| syn_100k_1M     | 100_000  | 1_000_000   | 20
| syn_250k_1M     | 250_000  | 1_000_000   | 8
| syn_500k_1M     | 500_000  | 1_000_000   | 4
| soc-wiki-vote   | 889      | 2_914       |
| bio-CE-GN       | 2_219    | 53_683      | 48
| bio-HS-CX       | 4_412    | 108_818     |
| bio-grid-yeast  | 6_008    | 313_890     |
| facebook_artist | 50_515   | 819_306     |
| notredame       | 325_729  | 1_497_134   |
| amazon          | 334_863  | 925_872     | 5.53
| rec-eachmovie   | 74_424   | 2_811_717   |



|file        | #CC
| syn_50k_100k | 32(V)
| syn_50k_500k | 1(x)[3] 



# Librerie
Perciò le librerie usate sono state *rayon*, per gestire il parallelismo e *dashmap* per ottenere una hashmap thread-safe.

# Schema di parallelismo
Esistono diversi modi per gestire il multithreading in Rust: thread (nativi), Rayon e Tokio.

Tokio è stata esclusa perchè il focus è più verso operazione asincrone (networking) che data processing.

I thread nativi, oltre ad essere meno facili da usare, hanno presentato il problema di come accedere alle strutture su cui operare. 
*Rayon* risolve questo offrendo un iteratore parallelo.


Inoltre *Rayon* offre la creazione di un ThreadPool globale di dimensione fissata. Quando è richiesta la parallelizazione di una attività, vengono usati tali thread.


# Strutture dati
Sebbene esista già la libreria *petgraph* adatta ad operazione su grafi, le strutture offerte da questa non sono pensate per un contesto parallelo. Pertanto si è creata una struttura adatta a questo caso. Le due strutture sono *ConcurrentDiGraph* e *ConcurrentUnGraph*, rispettivamente usate per rappresentare grafi orientati e non.

I grafi sono rappresentati come liste di adiacenza, usando una dashmap alla base invece di una hasmap standard. Una hashmap standard infatti presenta un unico lock, mentre la dashmap presenta una struttura divisa in *shards*: a valori hash diversi, corrisponodono lock diversi. Ciò permette scritture parallele più efficienti.



# Algoritmi
La versione implementata è quella ottimazata con *Edge Pruning* e *Oblivious Seed*.

Dato che OS restituisice un grafo orientato, i vicini di un nodo sono definiti come $NN_{G}(u) = \{v \vert (u -> v) \in G\}$


# Ottimizzazioni (linguaggio)
Negli accessi alle strutture indicizzate (Vec), Rust effettua dei [bound check](https://nnethercote.github.io/perf-book/bounds-checks.html). Sono stati evitati accessi diretti usando gli iteratori.

Inoltre per evitare/minimizzare riallocazioni, dove possibile, tutte le strutture sono state inizializzate con una capacità (stimata).


*You cannot make software run faster. Ever. That’s not a thing. You can only make it do less work.*


# Benchmark
I benchmark consistono in *N* esecuzioni degli algoritmi, e il risultato è il tempo medio delle esecuzioni


# Confronto con NetworkX
Una differenza tra le due versioni è il tempo di lettura del file mtx. Bottleneck in python, molto più veloce in Rust.


Altra differenza tra l'algoritmo e la funzione CC di NetworkX è l'approccio adottato nella generazione delle CC.

Cracker ha un'approccio eager: le componenti connesse sono generate subito, 
mentre NetworkX ha un approccio lazy: le componenti sono restituite con un generatore.

Per un benchmark equo, il generatore risultante viene iterato completamente.