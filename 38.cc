#include <bits/stdc++.h>
using namespace std;

vector<int> digitize(int n) {
    vector<int> digits;
    while(n) {
        digits.push_back(n%10);
        n/=10;
    }
    return digits;
}
int main() {
    /* 
        just smart bruteforce where logical bounds are found
        ie obv the number cannot be bigger than 9876, the number cannot have
        duplicates, the number cannot end on a 5 or a 10 etc... 
    */
    int end = 9999;
    for(int i = 8; i<end; i++) { // i =2 
        if(i%10 == 5 || i%10 == 0) continue;
        int factor = 1;
        set<int> digits_taken;
        bool breakflag = false;
        vector<int> parts;
        while(digits_taken.size()<9) {
            int number = i*factor;
            for(auto digit: digitize(number)){
                if(digit == 0) {
                    breakflag = true; break;
                }
                if(digits_taken.find(digit) != digits_taken.end()) {
                    breakflag = true; break;
                }
                digits_taken.insert(digit);
            };
            if(breakflag == true) break;
            parts.push_back(number);
            factor++;
        }
        if(breakflag == true) continue;
        if(digits_taken.size() == 9){
            cout << i <<": ";
            for(auto e: parts) cout << e <<" "; cout << "\n";
        }

    }
    return 0;
}
