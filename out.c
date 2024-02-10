#include <stdio.h>
int main(void){
float a;
float b;
float s;
float c;
a = 0;
while(a<1){
printf("Enter number of scores: ");
printf("\n");
if(0 == scanf("%f", &a)) {
a = 0;
scanf("%*s");
}
}
b = 0;
s = 0;
printf("Enter one value at a time: ");
printf("\n");
while(b<a){
if(0 == scanf("%f", &c)) {
c = 0;
scanf("%*s");
}
s = s+c;
b = b+1;
}
printf("Average: ");
printf("\n");
printf("%.2f", (float)(
s/a));
return 0;
}
