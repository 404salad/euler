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

void solve() {
    int n = (int)1e4;
    vector<bool> sev(n+1,true);
    sev[0] = false; sev[1] = false;
    for(int i = 2; i<=n; i++) {
        if(sev[i] == true) {
            for(int k = i+i; k<=n; k+=i) {
                sev[k] = false;
            }
        }
    }
    vector<int> primes;
    for(int i =1e4-1; i>=1e3; i--) {
        if(sev[i]) {
            primes.push_back(i);
        }
    }
    for(auto e: primes) {
        string ps = to_string(e);
        vector<int> all;
        do {
            int p = stoi(ps);
            if(sev[p]) all.push_back(p);
        } while(next_permutation(all(ps)));

        if(all.size()>=3) {
            //sort(all(all));
            for(int I =0;I<all.size(); I++){
                for(int J =I+1;J<all.size(); J++){
                    for(int K =J+1;K<all.size(); K++){
                        //print(vector<int>{I,J,K});
                        if(all[K]-all[J] == all[J]-all[I]) {
                            cout << all[I] <<" "<<all[J]<<" "<<all[K]<<"\n";
                        }
                    }
                }
            }
        }
    }
    //print(primes);
}
