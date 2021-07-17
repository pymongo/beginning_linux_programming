#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <sys/sem.h>
#include <stdbool.h>

#if defined(__GNU_LIBRARY__) && !defined(_SEM_SEMUN_UNDEFINED)
/* union semun is defined by including <sys/sem.h> */
#else
/* according to X/OPEN we have to define it ourselves */
union semun {
    int val;                    /* value for SETVAL */
    struct semid_ds *buf;       /* buffer for IPC_STAT, IPC_SET */
    unsigned short int *array;  /* array for GETALL, SETALL */
    struct seminfo *__buf;      /* buffer for IPC_INFO */
};
#endif

// semaphore_p changes the semaphore by -1 (waiting)
int semaphore_p(const int sem_id)
{
    struct sembuf sem_b;
    sem_b.sem_num = 0;
    sem_b.sem_op = -1; /* P() */
    sem_b.sem_flg = SEM_UNDO;

    if (semop(sem_id, &sem_b, 1) == -1) {
        fprintf(stderr, "semaphore_p failed\n");
        return 0;
    }
    return 1;
}

// semaphore_v is similar except for setting the sem_op part of the sembuf structure to 1,
// so that the semaphore becomes available
int semaphore_v(const int sem_id)
{
    struct sembuf sem_b;
    sem_b.sem_num = 0;
    sem_b.sem_op = 1; /* V() */
    sem_b.sem_flg = SEM_UNDO;

    if (semop(sem_id, &sem_b, 1) == -1) {
        fprintf(stderr, "semaphore_v failed\n");
        return 0;
    }
    return 1;
}

// process_1: init_del_sem=1 ./a.out
// process_2: ./a.out
// 进程1或进程2启动顺序无所谓，同一时间只会有一个进程在打印
int main(const int argc, const char *argv[])
{
    //srand((unsigned int)time(NULL)); // Bad, system time might be controlled by user https://reviews.llvm.org/D44143
    srand((unsigned int)getpid());

    // key=IPC_PRIVATE only share sem in current process
    const int sem_id = semget((key_t)1234, 1, 0666 | IPC_CREAT);

    char op_char = 'O';
    const bool init_del_sem = getenv("init_del_sem") == NULL;
    if (init_del_sem) {
        union semun sem_union;
        sem_union.val = 1; // because first call is semaphore_p(sem_val -= 1)
        // initializes the semaphore using the SETVAL command in a semctl call.
        // We need to do this before we can use the semaphore
        const int res = semctl(sem_id, 0, SETVAL, sem_union);
        if (res == -1) {
            fprintf(stderr, "Failed to initialize semaphore\n");
            exit(EXIT_FAILURE);
        }
        op_char = 'X';
        sleep(2);
    }

/* Then we have a loop which enters and leaves the critical section ten times.
 There, we first make a call to semaphore_p which sets the semaphore to wait, as
 this program is about to enter the critical section. */
    for(int i = 0; i < 10; i++) {
        if (!semaphore_p(sem_id)) exit(EXIT_FAILURE);
        printf("%c", op_char); fflush(stdout);
        usleep(rand() % 1000000);
        printf("%c", op_char); fflush(stdout);
/* After the critical section, we call semaphore_v, setting the semaphore available,
 before going through the for loop again after a random wait. After the loop, the call
 to del_semvalue is made to clean up the code. */
        if (!semaphore_v(sem_id)) exit(EXIT_FAILURE);
        usleep(rand() % 1000000);
    }

    if (init_del_sem) {
        sleep(3);
        const union semun sem_union;
        // IPC_RMID to remove the semaphore's ID
        const int res = semctl(sem_id, 0, IPC_RMID, sem_union);
        if (res == -1) {
            fprintf(stderr, "Failed to delete semaphore\n");
        }
    }

    printf("\nPID=%d - finished\n", getpid());
    exit(EXIT_SUCCESS);
}
