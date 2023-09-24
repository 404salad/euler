#include <iostream>
#include <math.h>
using namespace std;

int main() {
    long long i = 1;
    long long sum = 0;
    
    for(i;i<=1000;i++){
       sum+= pow(i,i); 
    }
    cout << sum;
}
