%{

#include "global.h"
#include "y.tab.h"

#include <stdio.h>
#include <stdlib.h>

int yywrap (void)
{
    return 1;
}

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
