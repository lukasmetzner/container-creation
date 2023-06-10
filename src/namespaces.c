#define _GNU_SOURCE
#include <sched.h>
#include <stdio.h>
#include <stdlib.h>
#include <sys/wait.h>
#include <unistd.h>
#include <unistd.h>
#include <string.h>

typedef struct
{
  char *init_args;
  char *container_path;
} ContainerArgs;

static char child_stack[1048576];

static int child_fn(ContainerArgs *containerArgs)
{
  printf("PID: %ld\n", (long)getpid());
  chroot(containerArgs->container_path);
  chdir("/");
  system((const char*) containerArgs->init_args);
  return 0;
}

int start_namespace(void *container_path, void *init_args)
{
  ContainerArgs *containerArgs;
  containerArgs = malloc(sizeof(strlen(container_path)) + 36);
  containerArgs->container_path = container_path;
  containerArgs->init_args = init_args;
  pid_t child_pid = clone(
      child_fn,
      child_stack + 1048576,
      // CLONE_NEWCGROUP | CLONE_NEWPID | SIGCHLD,
      SIGCHLD,
      containerArgs);
  printf("clone() = %ld\n", (long)child_pid);
  waitpid(child_pid, NULL, 0);
}

// int main(int argc, char** args) {
//   char* c = "/tmp/asdf/3f0fbc22-d733-4fd3-9b1e-98811041a752/ce1d66bff996cbccd4d52b704e94469aef49119d98c2e7dea6b279166f6789ce";
//   char* i = "ls -alh";
//   start_namespace(c, i);
//   return 0;
// }