# Librerie
Il progetto prevede l'implementazione multithreaded id un algoritmo per grafi. 
Perciò le librerie usate sono state *rayon*, per gestire il parallelismo e *petgraph* per ottenere strutture e funzione per grafi.

# Schema di parallelismo
*Rayon* offre la creazione di un ThreadPool globale di dimensione fissata. Quando è richiesta la parallelizazione di una attività, vengono usati tali thread.

# Strutture dati
La libreria *petgraph* offre diversi modi per rappresentare un grafo. La principale è *Graph*. 

Questa rappresenta i nodi tramite il loro indice ed un weight (corrisponde a dati aggiuntivi). L'inserimento di archi richiede di specificare gli index, mentre la rimozione di un nodo causa lo spostamento degli indici successivi. Ciò causa problemi nelle fasi dell'algoritmo in cui vengono rimossi dei nodi.

La scelta finale è ricaduta su *GraphMap*. A differenza della precedente si perde la possibilità di aggiungere dati aggiuntivi ai nodi, ma viene meno la necessità di tenere traccia degli indici: si può aggiungere un arco semplicemente specificando le label dei nodi. 




## Altre strutture
Una operazione frequente è l'inserimento/lettura da una *HashMap*. Invece di gestire esplicitamente la sincronizzazione, ho usato una *DashMap*. Tale struttura è zucchero sintattico per una *HashMap* con un *RwLock*, lock che permette lettori multipli ma un singolo scrittore.

Inoltre data l'impossibilità di aggiungere dati ai nodi della *GraphMap*, il risultato della fase di Seed Propagation è salvato in una HashMap ({nodo: seed}).

