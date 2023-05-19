#define _GNU_SOURCE
#include <sched.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/wait.h>
#include <unistd.h>
#include <unistd.h>

static char child_stack[1048576];

static int child_fn(void* arg) {
  char c;
  printf("PID: %ld\n", (long)getpid());
  chdir("/var/tmp/test-container");
  int res = system("ls -alh");
  res = chroot("/var/tmp/test-container");
  if(res != 0) {
    printf("Error\n");
  }
  FILE *ptr = fopen("hi", "r");
  c = fgetc(ptr);
  while (c != EOF)
  {
      printf ("%c", c);
      c = fgetc(ptr);
  }
  fclose(ptr);
  return 0;
}

int start_namespace() {
  pid_t child_pid = clone(
    child_fn, 
    child_stack+1048576, 
    CLONE_NEWPID | SIGCHLD , NULL
  );
  printf("clone() = %ld\n", (long)child_pid);
  waitpid(child_pid, NULL, 0);
}

int main(int argc, char ** argv) {
  start_namespace();
  return 0;
}