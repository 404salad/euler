#include <bits/stdc++.h>
using namespace std;
typedef long long ll;
bool isPrime(int n) {
    if(n<=0) return false;
    if(n==1) return false;
    if(n<4) return true;
    if(n%2==0) return false;
    if(n<9) return true; // as we excluded 4 6 8 as even
    if(n%3==0) return false;
    int f = 5;
    while(f<=floor(sqrt(n))){ // smartly generating 6n+-1 since all primes greater than 3
                              // follow 6n+-1
        if(n%f == 0) return false;
        if(n%(f+2) == 0) return false;
        f = f+6;
    }
    return true;
}
int main() {
    // precompute twice of sum of squares then check if difference is prime?
    vector<ll> primes;
    for(ll i = 0; i<100000; i++) {
        if(isPrime(i)) {
            primes.push_back(i);
        }
        else { // it is a candidate
            bool issokay = false;
            for(auto prime: primes){
                ll need = (i-prime)/2;
                ll root = sqrt(need);
                if(root*root == need){
                    issokay = true;
                    break;
                }
            }
            if(!issokay) cout << i <<" "; 
        }
    }

    return 0;
}
