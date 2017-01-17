%{
#include "global.h"
#include <stdio.h>
%}

%token  DIGIT PLUS END

%start Start
%%

Start:
    | Start Line
    ;

Line:
      END
    | Expression END    { printf("Results: %f\n", $1); }
    ;

Expression:
      DIGIT             { $$=$1; }
    | Expression PLUS Expression    { $$=$1+$3; }
    ;

%%

int yyerror(char* message) {
    printf("%s\n", message);
}

int main(void) {
    yyparse();
}
