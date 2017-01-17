%{

#include "global.h"
#include "test.h"

#include <stdio.h>
#include <stdlib.h>

%}

digit   [0-9]
plus    +

%%

{digit}     {
                yylval = atof(yytext);
                return(DIGIT);
            }
"+"         return(PLUS);
"\n"        return(END);
