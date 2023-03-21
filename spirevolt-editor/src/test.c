#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>

//#define SZ 5000


// Global Variables
float A[SZ][SZ];


void* short_test(void){
	time_t start, current;
	time(&start);

	int temp;
 	size_t temp_size = sizeof(temp);

	int i, j, count;
	float wps;
	double elapsed;
	FILE* test_file = fopen(".inferno_test.txt", "w");
	char str[100];

	if (test_file == NULL) {
    		printf("Error: unable to open file test.txt\n");
    		exit(EXIT_FAILURE);
  	}

	for(i = 0; i < SZ/4; i++) {
       		for(j = 0; j < SZ/4; j++) {
          		A[i][j] = i*j;
       		}
	}

	for(i = 0; i < SZ/4; i++) {
       		for(j = 1; j < SZ/4; j++) {
          		A[i][j] = ( A[i][j-1] + A[i][j] ) / 2;
  			time(&current);

    			// Calculate the elapsed time in seconds
   			elapsed = difftime(current, start);
			
			wps = count/elapsed;

    			// Print the elapsed time
    			printf("Elapsed time: %.0f seconds. Write speed: %.02f writes per second\r", elapsed, wps);
			fflush(stdout);

			sprintf(str, "%f", A[i][j]);
			fprintf(test_file, str);
			count += 1;
       		}
    	}
	
	double awps = count/elapsed;
	printf("\nAverage Write Speed: %.02f writes per second\n", awps);
	printf("Short Test Ended Successfully\n");

	return NULL;
}


int extended_test(void){
	time_t start, current;
	time(&start);

	int i, j, count;
       	float wps;
	double elapsed;
	FILE* test_file = fopen(".inferno_test.txt", "w");
	char str[100];

	if (test_file == NULL) {
    		printf("Error: unable to open file test.txt\n");
    		exit(EXIT_FAILURE);
  	}

	for(i = 0; i < SZ; i++) {
       		for(j = 0; j < SZ; j++) {
          		A[i][j] = i*j;
       		}
	}

	for(i = 0; i < SZ; i++) {
       		for(j = 1; j < SZ; j++) {
          		A[i][j] = ( A[i][j-1] + A[i][j] ) / 2;
			time(&current);

    			// Calculate the elapsed time in seconds
   			elapsed = difftime(current, start);

			wps = count/elapsed;

    			// Print the elapsed time
    			printf("Elapsed time: %.0f seconds. Write speed: %.02f writes per second\r", elapsed, wps);
			fflush(stdout);

  			sprintf(str, "%f", A[i][j]);
			fprintf(test_file, str);
			count += 1;
       		}
    	}

	double awps = count/elapsed;
	printf("\nAverage Write Speed: %.02f writes per second\n", awps);	
	printf("\nExtended Test Ended Successfully\n");

	return 0;
}



int normal_test(void){
	time_t start, current;
	time(&start);

	int i, j, count;
       	float wps;
	double elapsed;	
	FILE* test_file = fopen(".inferno_test.txt", "w");
	char str[100];

	if (test_file == NULL) {
    		printf("Error: unable to open file test.txt\n");
    		exit(EXIT_FAILURE);
  	}

	for(i = 0; i < SZ/2; i++) {
       		for(j = 0; j < SZ/2; j++) {
          		A[i][j] = i*j;
       		}
	}

	for(i = 0; i < SZ/2; i++) {
       		for(j = 1; j < SZ/2; j++) {
          		A[i][j] = ( A[i][j-1] + A[i][j] ) / 2;

			time(&current);

    			// Calculate the elapsed time in seconds
   			elapsed = difftime(current, start);

			wps = count/elapsed;

    			// Print the elapsed time
    			printf("Elapsed time: %.0f seconds. Write speed: %.02f writes per second\r", elapsed, wps);
			fflush(stdout);

			sprintf(str, "%f", A[i][j]);
			fprintf(test_file, str);

			count += 1;
       		}
    	}
	
	double awps = count/elapsed;
	printf("\nAverage Write Speed: %.02f writes per second\n", awps);
	printf("\nNormal Test Ended Successfully\n");

	return 0;
}



int cleanup(void){
	if (remove(".inferno_test.txt") != 0) {
    		printf("Error: unable to delete test file!\n");
    		exit(EXIT_FAILURE);
  	}

	return 0;
}


int main(void){
	char userin[100];
	system("clear");
	//system("neofetch");

	printf("Welcome to Inferno\n");
	printf("\nChoose which test to run:\n");
	printf("=========================\n");
	printf("(short) (normal) (extended)\n");

	scanf("%s", userin);
	printf("\n");

	if(strcmp(userin, "short") == 0){
		short_test();
	}else if(strcmp(userin, "normal") == 0){
		normal_test();
	}else if(strcmp(userin, "extended") == 0){
		extended_test();
	}else{
		printf("Error: userinput not matching options.");
		exit(EXIT_FAILURE);
	}

	cleanup();

	return 0;
}
