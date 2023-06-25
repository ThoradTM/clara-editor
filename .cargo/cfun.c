#include <stdio.h>
#include <string.h>


int main(){
	FILE *fp = fopen("fun.txt","r+");
	char str[1000]; 
	fgets(str,1000,fp);
	for(int i = 0; i < 1000; i++){
		if(str[i] == 34){
			str[i] = 32;
		}
	}
	fprintf(fp,"\n");
	fprintf(fp,str);
	fclose(fp);
	return 0;
}
