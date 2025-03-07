use std::sync::{Arc, Mutex};
use std::thread;

type Task = Box<dyn FnOnce() + Send + 'static>;
/* FnOnce definit une closure, éxécutée une fois,
    Box<dyn alloue une tâche sur le heap (!= du stack)
    Send envoi les tâches entre thread de façon sécurisée
    'static garanti la fermeture (closure) ne contient pas de ref temporaires */

struct TaskScheduler {
    tasks : Arc<Mutex<Vec<Task>>>,
    threads: Vec<thread::JoinHandle<()>>,
    /*
    Vec<Task> stocke liste de tâches
    Mutex protège la liste de tâches des accès concurrents
    Arc permet partage sécurisé entre les threads
     */
}

impl TaskScheduler {
    //Initialise liste de tâches vide protégé par un Mutex, et une liste de threads vides
    fn new() -> Self {
        TaskScheduler {
            tasks: Arc::new(Mutex::new(Vec::new())),
            threads: Vec::new()
        }
    }

    //Ajoute une tâche à la liste des tâches
    fn add_task(&self, task: Task) {
        //Verrouille l'acces à la liste tâches, et ajoute task à la liste
        self.tasks.lock().unwrap().push(task);
    }

    fn start(&mut self, num_threads: usize) {
        for _ in 0..num_threads {
            let tasks = Arc::clone(&self.tasks); //Clone arc pour acceder aux tasks
            let handle = thread::spawn(move || { //Move permet de capturer tasks
                loop {
                    let task = tasks.lock().unwrap().pop(); //Verouille les tâches, enlève une tâche de la liste (pop)
                    if let Some(task) = task { //Execute la tâche task
                        task();
                    } else { //Une fois que la chaîne est vide, on sort de la boucle
                        break;
                    }
                }
            });
            self.threads.push(handle)
        }
    }

    //Attend que les threads s'éxécutent
    fn wait_all(&mut self) {
        for handle in self.threads.drain(..) { //Vide threads en consommants ses elements
            handle.join().unwrap(); //Attend que chaque thread se termine (unwrap pour s'assurer qu'il n'y a pas d'erreur)
        }
    }
}

fn main() {
    // Crée une instance de TaskScheduler
    let mut scheduler = TaskScheduler::new();

    // Crée 10 tâches qui affichent un message et les ajoutent au planificateur
    for i in 0..100000 {
        let task = Box::new(move || {
            println!("Task {} executed by thread {:?}", i, thread::current().id());
        });
        scheduler.add_task(task);
    }
    //Demarre 4 threads pour éxécuter les tâches
    scheduler.start(4);

    // Attend que tous les threads soient finits pour éxécuter les tâches
    scheduler.wait_all();
}