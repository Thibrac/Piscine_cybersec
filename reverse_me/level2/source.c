#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>

void no() {
  puts("Nope.");
  exit(1);
}

void ok() { puts("Good job."); }

int main() {
  char input[30];
  char mdp_crypt[30];
  int i;
  int j;

  printf("Please enter key: ");
  if (scanf("%23s", input) != 1) {
    no();
  }
  if (input[0] != '0' || input[1] != '0') {
    no();
  }
  memset(mdp_crypt, 0, 9);
  mdp_crypt[0] = 'd';
  i = 2;
  j = 1;
  while (strlen(mdp_crypt) < 8) {
    if (strlen(input) <= i) {
      break;
    }
    char tmp[4] = {input[i], input[i + 1], input[i + 2], '\0'};
    mdp_crypt[j] = atoi(tmp);
    j++;
    i += 3;
  }
  mdp_crypt[j] = '\0';

  if (strcmp(mdp_crypt, "delabere") != 0) {
    no();
  }
  ok();
  return 0;
}