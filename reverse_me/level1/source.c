#include <stdio.h>
#include <string.h>

int main() {
  char buffer[20];
  char *mdp = "__stack_check";

  printf("Please enter key: ");
  scanf("%s", buffer);

  if (strcmp(buffer, mdp) == 0) {
    printf("Good job.\n");
  } else {
    printf("Nope.\n");
  }
  return 0;
}