#include <bits/stdc++.h>
using namespace std;
#define fast ios::sync_with_stdio(0);cin.tie(0);
#define rep(i, n)  for (int i = 0; i < (n); i++)
#define rep1(i, n) for (int i = 1; i < (n); i++)
#define repr(i, n) for (int i = (n)-1; i >= 0; i--)
#define all(x) x.begin(), x.end()
#define sum(v) reduce(all(v))
#define pb push_back
#define int long long int
typedef  long double ld;
class bail : public exception {}; // just throw bail()
template<typename T> struct is_pair : false_type {}; template<typename T, typename U> struct is_pair<pair<T, U>> : true_type {}; template<typename T> struct is_string : false_type {}; template<> struct is_string<string> : true_type {}; template<> struct is_string<const char*> : true_type {}; template<> struct is_string<char*> : true_type {}; template<typename T, typename = void> struct is_container : false_type {}; template<typename T> struct is_container<T, void_t<decltype(declval<T>().begin()), decltype(declval<T>().end())>> : bool_constant<!is_string<T>::value> {}; template<typename T> void print(const T& x) { if constexpr (is_pair<T>::value) { cout << "("; print(x.first); cout << ", "; print(x.second); cout << ")"; } else if constexpr (is_string<T>::value) { cout << "\"" << x << "\""; } else if constexpr (is_container<T>::value) { cout << "["; bool first = true; for (const auto& elem : x) { if (!first) cout << ", "; print(elem); first = false; } cout << "]"; } else { cout << x; } }

void solve();
int32_t main() {
    fast;
    solve();
    return 0;
}

bool isprime(int n) {
    for(int i = 2; i*i<n; i++) {
        if(n%i) return false;
    }
    return n>2;
}

void solve() {
    vector<int> primes;
    int n = (int)1e6;
    vector<bool> sev(n+1,true);
    sev[0] = false; sev[1] = false;
    for(int i = 2; i*i<=n; i++) {
        if(sev[i] == true) {
            for(int k = i*i; k<=n; k+=i) {
                sev[k] = false;
            }
        }
    }
    for(int i =0; i<1e6; i++) {
        if(sev[i]) {
            primes.push_back(i);
        }
    }
    auto phi = [&primes](int n) -> int {
        int fi = 0;
        int ans = 1;
        while(n>1){
            int x = 0;
            while(n>1 && n%primes[fi] == 0){
                n/=primes[fi];
                x++;
            }
            if(x!=0){
                ans *= pow((primes[fi]),(x-1))*(primes[fi]-1);
            }
            fi++;
        }
        return ans;
    };

    {
        int mxi = 3;
        int mxp = 3;
        for(int i =1000000; i>=2; i--) {
            int p = phi(i);
            if(mxi*p<i*mxp){
                mxi = i;
                mxp = p;
            }
            cout << mxi<<" "<< mxp <<"\n";
        }
    }
}
